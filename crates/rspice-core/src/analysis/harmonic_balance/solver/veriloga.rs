//! Verilog-A instances in the harmonic-balance F/Q sampler.
//!
//! An admitted module's residual is `F(x) + dQ(x)/dt`: the compiler has
//! proved that every contributed `ddt` is the derivative of a charge the
//! module stores (the weighted F/Q split), and the operators with state
//! outside that form are refused before the solver is built. Harmonic
//! balance needs `F`, `Q` and their partials at each collocation sample, and
//! the module's own evaluation entry points supply all four:
//!
//! * a static stamp, on a copy whose integration rule is inactive so every
//!   `ddt` evaluates to zero, is `F(x)` and `dF/dx`;
//! * a Backward Euler stamp over an interval `h`, on a second copy whose
//!   accepted history was primed once at a fixed reference state `x_ref`, is
//!   `F(x) + (Q(x) - Q(x_ref))/h` and `dF/dx + (dQ/dx)/h`.
//!
//! The two are separate copies because a copy whose rule turns active
//! promotes its current candidate into its history, as a transient does when
//! it starts from its operating point.
//!
//! The difference of the two, times `h`, is the charge relative to the
//! reference and its exact Jacobian. The companion is algebraic, not a
//! discretization: no truncation error enters, and the reference charge is a
//! constant that only the zeroth harmonic sees, where `jkω` multiplies it by
//! zero. Evaluation only writes candidate state, so the primed history is
//! never disturbed and every sample is independent of the order in which the
//! solver asks. `h = 1/ω₀` keeps `(Q - Q_ref)/h` on the scale of the
//! fundamental's displacement current, so the subtraction loses nothing the
//! residual tolerance can see.
//!
//! # Operators with state
//!
//! An `idt`, `laplace_*` or `absdelay` site keeps state outside `F + dQ/dt`,
//! and so does a `ddt` the compiler could not prove the derivative of a
//! charge the module contributes (one scaled by a quantity that varies with
//! bias, a nested one, one that steers a branch, one a branch-current read
//! sees, or one whose charge it cannot recover). Each becomes two periodic
//! unknowns of its own, its input `u` and its output `y`. The module's
//! residual is sampled with every output held at the solver's `y` (see
//! [`rspice_veriloga::vm::PeriodicOperatorRecord`]), the input row holds
//! `u - x(v, y) = 0` for the input `x` the module computes,
//! and a [`super::ExactPeriodicNetwork::VerilogAOperator`] holds
//! `Y - T(jω) U = 0` at every harmonic: `1/(jω)` for `idt`, whose DC output
//! its initial condition fixes instead, `H(jω)` for a Laplace filter,
//! `e^{-jωτ}` for a delay and `jω` for a `ddt`. The other `ddt` sites keep
//! the charge path above.
//!
//! A module that reads `$abstime` is evaluated at each sample's own instant.
//! Before solving, both probes' stamps at four instants of the period must
//! equal their stamps one period later, or the module is refused by name.
//!
//! The Jacobian stays exact. One evaluation with every output tangent zero
//! gives the partials with the outputs held, and the difference of an
//! evaluation seeding one output's tangent from it gives the partials with
//! respect to that output, on the residual and on every site's input; the
//! derivative programs are linear in their tangents, so the difference is
//! algebraically exact.

use super::HbSolverState;
use super::native_bjt::NativeStamp;
use crate::Value;
use crate::analysis::harmonic_balance::HbError;
use crate::device::MatrixStamper;
use crate::device::veriloga::{VerilogADevice, VerilogAEvaluationMode};
use rspice_veriloga::vm::{
    IntegrationCoefficients, PeriodicOperatorKind, PeriodicOperatorRecord, PeriodicOperatorSite,
};
use std::collections::BTreeMap;

/// The transient analysis code: `analysis("tran")` reads the same in both
/// probes, so the charge difference never sees an analysis predicate change.
const TRANSIENT_ANALYSIS: u8 = 2;

/// One stamp's residual `F` over the augmented unknowns, and its Jacobian
/// entries `(row, column, dF/dx)`.
type Probe = (Vec<Value>, Vec<(usize, usize, Value)>);

use super::HbOperatorTransfer;

/// One operator site's unknowns and what fixes them.
#[derive(Debug, Clone)]
struct HbOperatorPorts {
    site: PeriodicOperatorSite,
    /// Zero-based augmented row of the input `u`.
    input: usize,
    /// Zero-based augmented row of the output `y`.
    output: usize,
    /// An `idt`'s initial condition, or an `absdelay`'s delay.
    parameter: Option<Value>,
    /// The operator as a message names it.
    label: String,
}

/// One operator site the solver registers: its unknowns' rows, as a network
/// names them, and its transfer.
pub(super) struct HbOperatorNetwork {
    pub(super) name: String,
    pub(super) input: usize,
    pub(super) output: usize,
    pub(super) transfer: HbOperatorTransfer,
}

#[derive(Debug, Clone)]
pub(super) struct HbVerilogADevice {
    /// HB-private copy with an inactive integration rule: `F` and `dF/dx`.
    static_probe: VerilogADevice,
    /// HB-private copy integrating by Backward Euler over `interval` from
    /// accepted history that holds `Q(x_ref)`.
    charge_probe: VerilogADevice,
    /// The charge probe's companion interval, `1/ω₀`.
    interval: Value,
    /// Zero-based augmented MNA rows the instance stamps.
    rows: Vec<usize>,
    /// The rows of the module's own equations, without its operator rows.
    equation_rows: Vec<usize>,
    /// Operator sites carried as unknowns of their own, in frame order.
    operators: Vec<HbOperatorPorts>,
    /// Whether the module reads `$abstime`, which each sample then sets to
    /// its own instant.
    reads_time: bool,
}

/// A probe's Jacobian entries: zero-based row, column and value.
type ProbeJacobian = [(usize, usize, Value)];

/// Sum a probe's Jacobian entries by location.
fn summed(entries: &ProbeJacobian) -> BTreeMap<(usize, usize), Value> {
    let mut sums = BTreeMap::new();
    for &(row, col, value) in entries {
        *sums.entry((row, col)).or_insert(0.0) += value;
    }
    sums
}

impl HbVerilogADevice {
    /// Take an instance into the sampler: begin a transient analysis on two
    /// private copies and accept one point of the charge probe at the zero
    /// state, which becomes the reference charge.
    /// `ports` gives each of `device.periodic_operator_sites()`, in order, the
    /// zero-based rows of its input and output unknowns. Returns the device
    /// and the network each site needs.
    pub(super) fn try_new(
        device: VerilogADevice,
        fundamental_hz: Value,
        unknowns: usize,
        ports: &[(usize, usize)],
    ) -> Result<(Self, Vec<HbOperatorNetwork>), HbError> {
        let interval = (std::f64::consts::TAU * fundamental_hz).recip();
        let name = device.name.to_string();
        let failure = |phase: &str, error: &dyn std::fmt::Display| {
            HbError::InvalidCircuit(format!(
                "Verilog-A device '{name}' HB {phase} failed: {error}"
            ))
        };
        let companion = IntegrationCoefficients::backward_euler(interval)
            .map_err(|error| failure("charge-probe setup", &error))?;
        let sites = device.periodic_operator_sites();
        let reads_time = device.analog_operator_inventory().contains(&"$abstime");
        if sites.len() != ports.len() {
            return Err(HbError::InvalidCircuit(format!(
                "Verilog-A device '{name}' has {} operator sites and {} periodic port pairs",
                sites.len(),
                ports.len()
            )));
        }
        let mut device = device;
        if !sites.is_empty() {
            device
                .try_install_periodic_operator_frame(&sites)
                .map_err(|error| failure("operator-state setup", &error))?;
            device.set_periodic_operator_outputs(&vec![0.0; sites.len()], None);
        }
        let mut static_probe = device.clone();
        static_probe
            .try_begin_analysis(TRANSIENT_ANALYSIS)
            .and_then(|()| {
                static_probe.try_set_integration_coefficients(IntegrationCoefficients::inactive())
            })
            .map_err(|error| failure("static-probe setup", &error))?;
        let mut charge_probe = device;
        charge_probe
            .try_begin_analysis(TRANSIENT_ANALYSIS)
            .and_then(|()| charge_probe.try_set_integration_coefficients(companion))
            .map_err(|error| failure("charge-probe setup", &error))?;
        let mut rows = Vec::new();
        let mut rhs_rows = Vec::new();
        let reference = vec![0.0; unknowns];
        charge_probe
            .try_stamp_with_mode(
                &reference,
                |row, _, _| rows.push(row),
                |row, _| rhs_rows.push(row),
                VerilogAEvaluationMode::StaticProbe,
            )
            .and_then(|()| charge_probe.try_advance_state())
            .map_err(|error| failure("reference charge", &error))?;
        rows.extend(rhs_rows);
        rows.extend(
            charge_probe
                .mapped_jacobian_locations()
                .into_iter()
                .flatten()
                .chain(charge_probe.mapped_branch_incidence())
                .filter_map(|(row, _)| row),
        );
        rows.extend(
            charge_probe
                .mapped_rhs_rows()
                .into_iter()
                .flatten()
                .map(|(row, _)| row),
        );
        rows.sort_unstable();
        rows.dedup();
        let equation_rows = rows.clone();
        rows.extend(ports.iter().flat_map(|&(input, output)| [input, output]));
        rows.sort_unstable();
        rows.dedup();
        if let Some(&row) = rows.last()
            && row >= unknowns
        {
            return Err(HbError::InvalidCircuit(format!(
                "Verilog-A device '{name}' stamps MNA row {row}, outside the {unknowns}-unknown periodic state"
            )));
        }
        let mut device = Self {
            static_probe,
            charge_probe,
            interval,
            rows,
            equation_rows,
            operators: Vec::new(),
            reads_time,
        };
        let networks = device.try_bind_operators(&sites, ports, unknowns)?;
        if reads_time {
            device.check_periodic_time(unknowns, fundamental_hz.recip())?;
        }
        Ok((device, networks))
    }

    /// Read each operator site's parameters off one evaluation at the zero
    /// state, and build its ports and network.
    fn try_bind_operators(
        &mut self,
        sites: &[PeriodicOperatorSite],
        ports: &[(usize, usize)],
        unknowns: usize,
    ) -> Result<Vec<HbOperatorNetwork>, HbError> {
        if sites.is_empty() {
            return Ok(Vec::new());
        }
        let name = self.name().to_owned();
        Self::probe(&mut self.static_probe, &vec![0.0; unknowns])?;
        let record = self.record()?;
        let mut networks = Vec::with_capacity(sites.len());
        for (index, (&site, &(input, output))) in sites.iter().zip(ports).enumerate() {
            let (family, written) = match site.kind {
                PeriodicOperatorKind::Integral => ("idt", "idt"),
                PeriodicOperatorKind::Laplace => ("laplace", "laplace_*"),
                PeriodicOperatorKind::Delay => ("absdelay", "absdelay"),
                PeriodicOperatorKind::Derivative => ("ddt", "ddt"),
            };
            let origin = self.static_probe.analog_operator_origin(family, site.slot);
            let label = match origin {
                Some(origin) => match origin.location() {
                    Some(location) => format!("`{}` at {location}", origin.operator),
                    None => format!("`{}`", origin.operator),
                },
                None => format!("`{written}`"),
            };
            let refuse = |reason: &str| {
                HbError::InvalidCircuit(format!(
                    "harmonic balance cannot carry {label} of Verilog-A device '{name}': {reason}"
                ))
            };
            if record.dependent_parameters[index] {
                return Err(refuse(match site.kind {
                    PeriodicOperatorKind::Delay => {
                        "its delay depends on the circuit's unknowns, so no fixed e^(-jwt) carries it"
                    }
                    _ => {
                        "its initial condition depends on the circuit's unknowns, so it fixes no DC value"
                    }
                }));
            }
            let (parameter, transfer) = match site.kind {
                PeriodicOperatorKind::Integral => {
                    let ic = record.initial_conditions[index]
                        .filter(|ic| ic.is_finite())
                        .ok_or_else(|| refuse("its initial condition was not evaluated"))?;
                    (Some(ic), HbOperatorTransfer::Integral)
                }
                PeriodicOperatorKind::Laplace => {
                    let filter = self
                        .static_probe
                        .laplace_filter(site.slot)
                        .cloned()
                        .ok_or_else(|| refuse("its filter is missing"))?;
                    filter.transfer_at(0.0).map_err(|_| {
                        refuse(
                            "its transfer has a pole at the origin, so its DC output is undefined",
                        )
                    })?;
                    (None, HbOperatorTransfer::Laplace(Box::new(filter)))
                }
                PeriodicOperatorKind::Delay => {
                    let delay = record.delays[index]
                        .filter(|delay| delay.is_finite() && *delay >= 0.0)
                        .ok_or_else(|| refuse("its delay is not a finite nonnegative time"))?;
                    (Some(delay), HbOperatorTransfer::Delay(delay))
                }
                PeriodicOperatorKind::Derivative => (None, HbOperatorTransfer::Derivative),
            };
            networks.push(HbOperatorNetwork {
                name: format!("{name}:{family}{}", site.slot),
                input,
                output,
                transfer,
            });
            self.operators.push(HbOperatorPorts {
                site,
                input,
                output,
                parameter,
                label,
            });
        }
        Ok(networks)
    }

    /// Refuse an orbit in which an `idt` input has a mean over the period.
    ///
    /// Its initial condition fixes the output's DC term, so the input's own
    /// DC term is free in the harmonic-balance system; a nonzero one is a
    /// ramp no periodic output carries, which transient analysis would show
    /// as the integral running away.
    pub(super) fn check_periodic_integrals(
        &self,
        state: &HbSolverState,
        num_nodes: usize,
        reltol: Value,
        abstol: Value,
    ) -> Result<(), HbError> {
        for ports in &self.operators {
            if ports.site.kind != PeriodicOperatorKind::Integral {
                continue;
            }
            let Some(spectrum) = ports
                .input
                .checked_sub(num_nodes)
                .and_then(|branch| state.mna_branch_currents.get(branch))
            else {
                continue;
            };
            let Some((mean, harmonics)) = spectrum.split_first() else {
                continue;
            };
            let scale = harmonics
                .iter()
                .map(|value| value.norm())
                .fold(0.0, Value::max);
            if mean.norm() > reltol * scale + abstol {
                return Err(HbError::InvalidCircuit(format!(
                    "harmonic balance cannot carry {} of Verilog-A device '{}': its input has \
                     mean {:e} over the period, so its output ramps and has no periodic \
                     steady state",
                    ports.label,
                    self.name(),
                    mean.re
                )));
            }
        }
        Ok(())
    }

    /// What the static probe's last evaluation saw.
    fn record(&self) -> Result<PeriodicOperatorRecord, HbError> {
        self.static_probe
            .periodic_operator_record()
            .cloned()
            .ok_or_else(|| {
                HbError::InvalidCircuit(format!(
                    "Verilog-A device '{}' lost its operator-state frame",
                    self.name()
                ))
            })
    }

    pub(super) fn name(&self) -> &str {
        &self.static_probe.name
    }

    /// Zero-based augmented MNA rows the instance stamps.
    pub(super) fn rows(&self) -> &[usize] {
        &self.rows
    }

    /// One stamp of `device` at `solution`: `A·x - z`, which is the residual
    /// `F` of the companion `A·x_new = z` the stamp linearizes, and its
    /// Jacobian `A`.
    fn probe(device: &mut VerilogADevice, solution: &[Value]) -> Result<Probe, HbError> {
        let mut matrix = Vec::new();
        let mut rhs = Vec::new();
        device
            .try_stamp_with_mode(
                solution,
                |row, col, value| matrix.push((row, col, value)),
                |row, value| rhs.push((row, value)),
                VerilogAEvaluationMode::StaticProbe,
            )
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "Verilog-A device '{}' HB sample failed: {error}",
                    device.name
                ))
            })?;
        let mut residual = vec![0.0; solution.len()];
        for &(row, col, value) in &matrix {
            residual[row] += value * solution[col];
        }
        for &(row, value) in &rhs {
            residual[row] -= value;
        }
        Ok((residual, matrix))
    }

    /// Refuse a module whose equations read `$abstime` other than
    /// periodically over `period`: at each quarter of the period, both
    /// probes' stamps at the zero state must be the same one period later.
    /// A ramp in time differs; a tone of the analysis's frequency does not.
    fn check_periodic_time(&mut self, unknowns: usize, period: Value) -> Result<(), HbError> {
        let zero = vec![0.0; unknowns];
        let mut stamps = |time: Value| -> Result<Vec<Value>, HbError> {
            let mut values = Vec::new();
            for probe in [&mut self.static_probe, &mut self.charge_probe] {
                probe.try_set_time(time).map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "Verilog-A device '{}' cannot be evaluated at t = {time:e} s: {error}",
                        probe.name
                    ))
                })?;
                let (residual, jacobian) = Self::probe(probe, &zero)?;
                values.extend(residual);
                values.extend(summed(&jacobian).into_values());
            }
            Ok(values)
        };
        // One scale over every instant: at an instant where every entry
        // vanishes, what is left is rounding, not the module's equations.
        let mut pairs = Vec::with_capacity(4);
        for quarter in 0..4 {
            let time = Value::from(quarter) * period / 4.0;
            pairs.push((time, stamps(time)?, stamps(time + period)?));
        }
        let scale = pairs
            .iter()
            .flat_map(|(_, now, later)| now.iter().chain(later))
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        for (time, now, later) in &pairs {
            let differs = now.len() != later.len()
                || now
                    .iter()
                    .zip(later)
                    .any(|(a, b)| (a - b).abs() > 1.0e-9 * scale + Value::MIN_POSITIVE);
            if differs {
                return Err(HbError::InvalidCircuit(format!(
                    "harmonic balance cannot sample Verilog-A device '{}' (module '{}'): it \
                     reads `$abstime`, and its equations at t = {time:e} s and one period of \
                     the analysis later differ, so they are not periodic over the analysis \
                     period",
                    self.name(),
                    self.static_probe.model_name()
                )));
            }
        }
        Ok(())
    }

    /// Stamp `F` and `Q - Q_ref` at one collocation sample at `time`, with
    /// their Jacobians, in the sampler's `-F` residual convention.
    pub(super) fn stamp_periodic_fq(
        &mut self,
        solution: &[Value],
        time: Value,
        f: &mut NativeStamp,
        q: &mut NativeStamp,
    ) -> Result<(), HbError> {
        if self.reads_time {
            for probe in [&mut self.static_probe, &mut self.charge_probe] {
                probe.try_set_time(time).map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "Verilog-A device '{}' cannot be evaluated at t = {time:e} s: {error}",
                        probe.name
                    ))
                })?;
            }
        }
        let outputs: Vec<Value> = self
            .operators
            .iter()
            .map(|ports| solution[ports.output])
            .collect();
        if !outputs.is_empty() {
            self.static_probe
                .set_periodic_operator_outputs(&outputs, None);
            self.charge_probe
                .set_periodic_operator_outputs(&outputs, None);
        }
        let (static_residual, static_jacobian) = Self::probe(&mut self.static_probe, solution)?;
        let (companion_residual, companion_jacobian) =
            Self::probe(&mut self.charge_probe, solution)?;
        let h = self.interval;
        for &row in &self.rows {
            let charge = h * (companion_residual[row] - static_residual[row]);
            f.stamp_rhs(row + 1, -static_residual[row]);
            q.stamp_rhs(row + 1, -charge);
        }
        for &(row, col, value) in &static_jacobian {
            f.stamp(row + 1, col + 1, value);
            q.stamp(row + 1, col + 1, -h * value);
        }
        for &(row, col, value) in &companion_jacobian {
            q.stamp(row + 1, col + 1, h * value);
        }
        if self.operators.is_empty() {
            return Ok(());
        }
        let record = self.record()?;
        self.stamp_operator_rows(
            solution,
            &outputs,
            &record,
            (&static_jacobian, &companion_jacobian),
            f,
            q,
        )
    }

    /// Stamp each operator's input row `u - x(v, y)`, an `idt`'s output row's
    /// initial condition, and the partials of the module's equations and of
    /// every input with respect to each output.
    fn stamp_operator_rows(
        &mut self,
        solution: &[Value],
        outputs: &[Value],
        record: &PeriodicOperatorRecord,
        (static_jacobian, companion_jacobian): (&ProbeJacobian, &ProbeJacobian),
        f: &mut NativeStamp,
        q: &mut NativeStamp,
    ) -> Result<(), HbError> {
        let name = self.name().to_owned();
        let h = self.interval;
        let fail = |label: &str, reason: &str| {
            HbError::InvalidCircuit(format!(
                "harmonic balance cannot carry {label} of Verilog-A device '{name}': {reason}"
            ))
        };
        // Each site's input partials along circuit columns, and the first
        // column a seeded tangent can be read back at.
        let mut columns: Vec<Vec<(rspice_veriloga::vm::PeriodicOperatorAxis, usize, Value)>> =
            vec![Vec::new(); self.operators.len()];
        for (&(site, axis), &value) in &record.input_partials {
            if let Some(column) = self.static_probe.periodic_operator_axis_index(axis) {
                columns[site].push((axis, column, value));
            }
        }
        for (index, ports) in self.operators.iter().enumerate() {
            if record.dependent_parameters[index] {
                return Err(fail(
                    &ports.label,
                    "its initial condition or delay changes over the period",
                ));
            }
            let input = record.inputs[index]
                .ok_or_else(|| fail(&ports.label, "its input was not evaluated"))?;
            match ports.site.kind {
                PeriodicOperatorKind::Integral => {
                    let ic = record.initial_conditions[index];
                    if ic != ports.parameter {
                        return Err(fail(
                            &ports.label,
                            "its initial condition changes over the period",
                        ));
                    }
                    f.stamp_rhs(ports.output + 1, ports.parameter.unwrap_or(0.0));
                }
                PeriodicOperatorKind::Delay => {
                    if record.delays[index] != ports.parameter {
                        return Err(fail(&ports.label, "its delay changes over the period"));
                    }
                }
                PeriodicOperatorKind::Laplace | PeriodicOperatorKind::Derivative => {}
            }
            f.stamp_rhs(ports.input + 1, input - solution[ports.input]);
            f.stamp(ports.input + 1, ports.input + 1, 1.0);
            for &(_, column, value) in &columns[index] {
                f.stamp(ports.input + 1, column + 1, -value);
            }
        }
        let static_held = summed(static_jacobian);
        let companion_held = summed(companion_jacobian);
        for seed in 0..self.operators.len() {
            let Some(&(_, column, _)) = columns[seed].first() else {
                return Err(fail(
                    &self.operators[seed].label,
                    "its input does not depend on any of the circuit's unknowns",
                ));
            };
            // Every model column the seeded site's derivative ran along adds
            // its tangent to the solver column it maps to.
            let multiplicity = columns[seed]
                .iter()
                .filter(|&&(_, other, _)| other == column)
                .count() as Value;
            self.static_probe
                .set_periodic_operator_outputs(outputs, Some(seed));
            self.charge_probe
                .set_periodic_operator_outputs(outputs, Some(seed));
            let (_, static_seeded) = Self::probe(&mut self.static_probe, solution)?;
            let (_, companion_seeded) = Self::probe(&mut self.charge_probe, solution)?;
            let seeded_record = self.record()?;
            let static_seeded = summed(&static_seeded);
            let companion_seeded = summed(&companion_seeded);
            let output = self.operators[seed].output;
            let difference = |seeded: &BTreeMap<(usize, usize), Value>,
                              held: &BTreeMap<(usize, usize), Value>,
                              row: usize| {
                (seeded.get(&(row, column)).copied().unwrap_or(0.0)
                    - held.get(&(row, column)).copied().unwrap_or(0.0))
                    / multiplicity
            };
            for &row in &self.equation_rows {
                let conduction = difference(&static_seeded, &static_held, row);
                let companion = difference(&companion_seeded, &companion_held, row);
                f.stamp(row + 1, output + 1, conduction);
                q.stamp(row + 1, output + 1, h * (companion - conduction));
            }
            // A site's input reads this output along a model column both
            // inputs' derivatives ran along.
            for (index, ports) in self.operators.iter().enumerate() {
                let along = columns[index]
                    .iter()
                    .find(|(axis, _, _)| columns[seed].iter().any(|(other, _, _)| other == axis))
                    .map(|&(axis, _, held)| (axis, held));
                if let Some((axis, held)) = along
                    && let Some(&seeded) = seeded_record.input_partials.get(&(index, axis))
                {
                    f.stamp(ports.input + 1, output + 1, -(seeded - held));
                }
            }
        }
        self.static_probe
            .set_periodic_operator_outputs(outputs, None);
        self.charge_probe
            .set_periodic_operator_outputs(outputs, None);
        Ok(())
    }
}
