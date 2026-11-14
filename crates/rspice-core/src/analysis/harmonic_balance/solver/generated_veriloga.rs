//! Generated compact models in the harmonic-balance F/Q sampler.
//!
//! The generator decides, once per model, whether the model's residual is
//! `F(x) + dQ(x)/dt` with nothing carried between evaluations (its
//! `harmonic_balance_refusal` is `None`), and the capability gate refuses
//! every other model before the solver is built. The model's own probe stamp
//! then supplies both halves exactly, as it does for a runtime module (see
//! `super::veriloga`):
//!
//! * a stamp whose `ddt` coefficients are inactive, so every `ddt` evaluates
//!   to zero, is `F(x)` and `dF/dx`;
//! * a Backward Euler stamp over `h` on a second copy, whose accepted history
//!   holds the charges at a fixed reference state `x_ref`, is
//!   `F(x) + (Q(x) - Q(x_ref))/h` and `dF/dx + (dQ/dx)/h`.
//!
//! Both copies evaluate in probe mode, so no `$limit` enters and neither
//! publishes accepted state: every sample is independent of the order in
//! which the solver asks. `h = 1/ω₀` keeps `(Q - Q_ref)/h` on the scale of
//! the fundamental's displacement current.
//!
//! # Operators carried as unknowns of their own
//!
//! A `ddt` the generator could not prove the derivative of a contributed
//! charge is carried as two periodic unknowns, its input `u` and its output
//! `y`, as the runtime route carries one (see `super::veriloga`): both probes
//! evaluate the site at the solver's `y` and record its input there (see
//! [`rspice_veriloga_runtime::GeneratedPeriodicFrame`]), an input row holds
//! `u - x(v, y) = 0`, and a network holds `Y = jkω U` at each harmonic. The
//! partials with respect to `y` are exact: a probe seeding the site's output
//! tangent along one lane its input was differentiated along differs from
//! the held probe, in that lane's column, by exactly `∂F/∂y`, `∂Q/∂y` and
//! every other site's `∂u/∂y`.
//!
//! Every `idt` is carried the same way with `Y = U/(jkω)` at each harmonic,
//! as the runtime route carries one: its initial condition fixes its DC
//! output, so it must not depend on the circuit's unknowns, and an orbit in
//! which its input has a mean over the period is refused after the solve.

use super::HbOperatorTransfer;
use super::HbSolverState;
use super::native_bjt::NativeStamp;
use crate::Value;
use crate::analysis::harmonic_balance::HbError;
use crate::device::MatrixStamper;
use crate::device::veriloga_builtins::BuiltinVerilogAInstance;
use rspice_matrix::StaticMatrix;
use rspice_veriloga_runtime::{
    GeneratedAnalysisKind, GeneratedDdtCoefficients, GeneratedPeriodicKind,
    GeneratedSimulationParameters,
};

/// Each carried site's input and output rows and its transfer, in site
/// order, for the solver to register as networks.
pub(super) type OperatorNetworks = Vec<(usize, usize, HbOperatorTransfer)>;

/// One operator site of a generated model carried as unknowns of its own.
#[derive(Debug, Clone, Copy)]
struct HbGeneratedOperator {
    kind: GeneratedPeriodicKind,
    /// Zero-based augmented row of the input `u`.
    input: usize,
    /// Zero-based augmented row of the output `y`.
    output: usize,
    /// An `idt`'s initial condition, read at setup, which fixes its DC
    /// output; `None` for a `ddt` or an `idt` the model does not evaluate.
    initial_condition: Option<Value>,
}

/// Scratch the carried sites' rows are built in, sized once.
#[derive(Debug, Clone, Default)]
struct HbOperatorScratch {
    outputs: Vec<Value>,
    seeds: Vec<Option<u32>>,
    lanes: Vec<u32>,
    /// Per site: the lane its output is seeded along and its column.
    seed_columns: Vec<Option<(u32, usize)>>,
    /// Per site and model row: the held probes' Jacobian in its column.
    held_static: Vec<Value>,
    held_charge: Vec<Value>,
    /// Per site `t` and seeded site `s`: `∂u_t` along `s`'s lane, held.
    held_partials: Vec<Option<Value>>,
}

pub(super) struct HbGeneratedDevice {
    /// HB-private copy with inactive `ddt` coefficients: `F` and `dF/dx`.
    static_probe: BuiltinVerilogAInstance,
    /// HB-private copy integrating by Backward Euler over `interval` from
    /// accepted history that holds `Q(x_ref)`.
    charge_probe: BuiltinVerilogAInstance,
    /// The sink both probes stamp into: every entry among the instance's own
    /// unknowns, over the whole periodic state.
    matrix: StaticMatrix,
    /// The charge probe's companion interval, `1/ω₀`.
    interval: Value,
    num_nodes: usize,
    simparams: GeneratedSimulationParameters,
    /// Zero-based augmented MNA rows the instance stamps, sorted.
    rows: Vec<usize>,
    /// The right-hand side a probe stamps into, over the whole periodic
    /// state; only `rows` are ever written or read.
    rhs: Vec<Value>,
    /// Per row of `rows`: the static probe's residual `F`.
    static_residual: Vec<Value>,
    /// Per row of `rows`: the charge probe's residual `F + (Q - Q_ref)/h`.
    charge_residual: Vec<Value>,
    /// Carried operator sites, in the model's site order.
    operators: Vec<HbGeneratedOperator>,
    /// `rows` and every carried site's input and output rows, sorted.
    stamped_rows: Vec<usize>,
    scratch: HbOperatorScratch,
}

impl Clone for HbGeneratedDevice {
    fn clone(&self) -> Self {
        Self {
            static_probe: self.static_probe.clone(),
            charge_probe: self.charge_probe.clone(),
            matrix: self.matrix.clone_structure(),
            interval: self.interval,
            num_nodes: self.num_nodes,
            simparams: self.simparams,
            rows: self.rows.clone(),
            rhs: self.rhs.clone(),
            static_residual: self.static_residual.clone(),
            charge_residual: self.charge_residual.clone(),
            operators: self.operators.clone(),
            stamped_rows: self.stamped_rows.clone(),
            scratch: self.scratch.clone(),
        }
    }
}

impl std::fmt::Debug for HbGeneratedDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HbGeneratedDevice")
            .field("static_probe", &self.static_probe)
            .field("charge_probe", &self.charge_probe)
            .field("interval", &self.interval)
            .field("num_nodes", &self.num_nodes)
            .field("rows", &self.rows)
            .finish_non_exhaustive()
    }
}

impl HbGeneratedDevice {
    /// Take an instance into the sampler: begin a transient analysis on two
    /// private copies, evaluate both once at the zero state and accept that
    /// point, which becomes the charge probe's reference charge.
    /// `ports` gives each of `device.harmonic_balance_periodic_sites()`, in
    /// order, the zero-based rows of its input and output unknowns. Returns
    /// the sampler and, per site, its rows and transfer.
    pub(super) fn try_new(
        device: BuiltinVerilogAInstance,
        fundamental_hz: Value,
        num_nodes: usize,
        unknowns: usize,
        simparams: GeneratedSimulationParameters,
        ports: &[(usize, usize)],
    ) -> Result<(Self, OperatorNetworks), HbError> {
        let name = device.instance_name.clone();
        let failure = |phase: &str, error: &dyn std::fmt::Display| {
            HbError::InvalidCircuit(format!(
                "generated Verilog-A device '{name}' HB {phase} failed: {error}"
            ))
        };
        if let Some(reason) = device.harmonic_balance_refusal() {
            return Err(HbError::InvalidCircuit(format!(
                "generated Verilog-A device '{name}' cannot be sampled by harmonic balance: {reason}"
            )));
        }
        let interval = (std::f64::consts::TAU * fundamental_hz).recip();
        let companion =
            GeneratedDdtCoefficients::from_companion_values(1.0, -1.0, 0.0, false, false, interval)
                .map_err(|error| failure("charge-probe setup", &error))?;
        let mut rows: Vec<usize> = device
            .nodes
            .iter()
            .filter(|&&node| node > 0)
            .map(|&node| node - 1)
            .chain(
                device
                    .branches
                    .iter()
                    .filter(|&&branch| branch > 0)
                    .map(|&branch| num_nodes + branch - 1),
            )
            .collect();
        rows.sort_unstable();
        rows.dedup();
        if let Some(&row) = rows.last()
            && row >= unknowns
        {
            return Err(HbError::InvalidCircuit(format!(
                "generated Verilog-A device '{name}' stamps MNA row {row}, outside the \
                 {unknowns}-unknown periodic state"
            )));
        }
        let sites = device.harmonic_balance_periodic_sites();
        if sites.len() != ports.len() {
            return Err(HbError::InvalidCircuit(format!(
                "generated Verilog-A device '{name}' has {} carried operator sites and {} \
                 periodic port pairs",
                sites.len(),
                ports.len()
            )));
        }
        let operators: Vec<HbGeneratedOperator> = sites
            .iter()
            .zip(ports)
            .map(|(&kind, &(input, output))| HbGeneratedOperator {
                kind,
                input,
                output,
                initial_condition: None,
            })
            .collect();
        let mut stamped_rows = rows.clone();
        stamped_rows.extend(ports.iter().flat_map(|&(input, output)| [input, output]));
        stamped_rows.sort_unstable();
        stamped_rows.dedup();
        if let Some(&row) = stamped_rows.last()
            && row >= unknowns
        {
            return Err(HbError::InvalidCircuit(format!(
                "generated Verilog-A device '{name}' carries an operator row {row}, outside the \
                 {unknowns}-unknown periodic state"
            )));
        }
        let triplets: Vec<(usize, usize, Value)> = rows
            .iter()
            .flat_map(|&row| rows.iter().map(move |&col| (row, col, 0.0)))
            .collect();
        let matrix = StaticMatrix::from_triplets(unknowns, unknowns, &triplets)
            .map_err(|error| failure("stamp-pattern setup", &error))?;
        let mut sampler = Self {
            static_probe: device.clone(),
            charge_probe: device,
            matrix,
            interval,
            num_nodes,
            simparams,
            rhs: vec![0.0; unknowns],
            static_residual: vec![0.0; rows.len()],
            charge_residual: vec![0.0; rows.len()],
            scratch: HbOperatorScratch {
                outputs: vec![0.0; operators.len()],
                seeds: vec![None; operators.len()],
                lanes: Vec::new(),
                seed_columns: vec![None; operators.len()],
                held_static: vec![0.0; operators.len() * rows.len()],
                held_charge: vec![0.0; operators.len() * rows.len()],
                held_partials: vec![None; operators.len() * operators.len()],
            },
            rows,
            operators,
            stamped_rows,
        };
        if !sampler.operators.is_empty() {
            sampler.static_probe.install_periodic_frame();
            sampler.charge_probe.install_periodic_frame();
        }
        for (probe, coefficients) in [
            (
                &mut sampler.static_probe,
                GeneratedDdtCoefficients::inactive(),
            ),
            (&mut sampler.charge_probe, companion),
        ] {
            probe.set_initially_off(false);
            probe.link_static_stamps(&sampler.matrix, num_nodes);
            probe
                .begin_analysis(GeneratedAnalysisKind::Tran, simparams, num_nodes)
                .map_err(|error| failure("analysis setup", &error))?;
            probe.set_analysis_step(false, false);
            probe.set_timepoint(0.0, interval, coefficients);
        }
        let reference = vec![0.0; unknowns];
        for probe in [&mut sampler.static_probe, &mut sampler.charge_probe] {
            Self::probe(
                probe,
                &mut sampler.matrix,
                ProbeBuffers {
                    rhs: &mut sampler.rhs,
                    residual: &mut sampler.static_residual,
                    rows: &sampler.rows,
                },
                &reference,
                num_nodes,
                simparams,
            )
            .and_then(|()| {
                probe
                    .advance_state()
                    .map_err(|error| failure("reference charge", &error))
            })?;
        }
        sampler.read_initial_conditions();
        let transfers = sampler
            .operators
            .iter()
            .map(|operator| {
                let transfer = match operator.kind {
                    GeneratedPeriodicKind::Derivative => HbOperatorTransfer::Derivative,
                    GeneratedPeriodicKind::Integral => HbOperatorTransfer::Integral,
                };
                (operator.input, operator.output, transfer)
            })
            .collect();
        Ok((sampler, transfers))
    }

    pub(super) fn name(&self) -> &str {
        &self.static_probe.instance_name
    }

    /// Zero-based augmented MNA rows the instance stamps, its carried
    /// operator sites' rows included.
    pub(super) fn stamped_rows(&self) -> &[usize] {
        &self.stamped_rows
    }

    /// One probe stamp of `device` at `solution`, left in `matrix` as its
    /// Jacobian `A`, with `A·x - z` at each of the instance's rows in
    /// `buffers.residual`: the residual `F` of the companion `A·x_new = z` the
    /// stamp linearizes. Nothing is allocated: a sample runs once per
    /// collocation point per Newton evaluation.
    fn probe(
        device: &mut BuiltinVerilogAInstance,
        matrix: &mut StaticMatrix,
        buffers: ProbeBuffers<'_>,
        solution: &[Value],
        num_nodes: usize,
        simparams: GeneratedSimulationParameters,
    ) -> Result<(), HbError> {
        let ProbeBuffers {
            rhs,
            residual,
            rows,
        } = buffers;
        matrix.clear_values();
        for &row in rows {
            rhs[row] = 0.0;
        }
        device
            .stamp_probe(
                matrix,
                rhs,
                solution,
                num_nodes,
                GeneratedAnalysisKind::Tran,
                simparams,
            )
            .map_err(|error| {
                let message = format!(
                    "generated Verilog-A device '{}' HB sample failed: {error}",
                    device.instance_name
                );
                if crate::circuit::generated_evaluation_is_nonfinite(&error) {
                    HbError::NonFiniteSample(message)
                } else {
                    HbError::InvalidCircuit(message)
                }
            })?;
        for (slot, &row) in residual.iter_mut().zip(rows) {
            *slot = -rhs[row];
        }
        for (row, col, value) in matrix.stored_entries() {
            if value != 0.0
                && let Ok(slot) = rows.binary_search(&row)
            {
                residual[slot] += value * solution[col];
            }
        }
        Ok(())
    }

    /// Stamp `F` and `Q - Q_ref` at one collocation sample, with their
    /// Jacobians, in the sampler's `-F` residual convention.
    pub(super) fn stamp_periodic_fq(
        &mut self,
        solution: &[Value],
        f: &mut NativeStamp,
        q: &mut NativeStamp,
    ) -> Result<(), HbError> {
        let h = self.interval;
        let carried = !self.operators.is_empty();
        if carried {
            self.prepare_frames(solution, None);
        }
        Self::probe(
            &mut self.static_probe,
            &mut self.matrix,
            ProbeBuffers {
                rhs: &mut self.rhs,
                residual: &mut self.static_residual,
                rows: &self.rows,
            },
            solution,
            self.num_nodes,
            self.simparams,
        )?;
        if carried {
            self.choose_seed_columns();
        }
        for (row, col, value) in self.matrix.stored_entries() {
            if value != 0.0 {
                f.stamp(row + 1, col + 1, value);
                q.stamp(row + 1, col + 1, -h * value);
            }
            capture_column(
                &self.scratch.seed_columns,
                &self.rows,
                &mut self.scratch.held_static,
                (row, col, value),
            );
        }
        Self::probe(
            &mut self.charge_probe,
            &mut self.matrix,
            ProbeBuffers {
                rhs: &mut self.rhs,
                residual: &mut self.charge_residual,
                rows: &self.rows,
            },
            solution,
            self.num_nodes,
            self.simparams,
        )?;
        for (row, col, value) in self.matrix.stored_entries() {
            if value != 0.0 {
                q.stamp(row + 1, col + 1, h * value);
            }
            capture_column(
                &self.scratch.seed_columns,
                &self.rows,
                &mut self.scratch.held_charge,
                (row, col, value),
            );
        }
        for ((&row, &static_residual), &charge_residual) in self
            .rows
            .iter()
            .zip(&self.static_residual)
            .zip(&self.charge_residual)
        {
            f.stamp_rhs(row + 1, -static_residual);
            q.stamp_rhs(row + 1, -h * (charge_residual - static_residual));
        }
        if carried {
            self.stamp_operator_rows(solution, f, q)?;
        }
        Ok(())
    }

    /// Keep each `idt`'s initial condition from the reference evaluation.
    fn read_initial_conditions(&mut self) {
        let Some(frame) = self.static_probe.periodic_frame() else {
            return;
        };
        for (site, operator) in self.operators.iter_mut().enumerate() {
            if matches!(operator.kind, GeneratedPeriodicKind::Integral) {
                operator.initial_condition =
                    Some(frame.initial_condition(site)).filter(|ic| !ic.is_nan());
            }
        }
    }

    /// Stamp an evaluated `idt`'s DC output, which its initial condition
    /// fixes, refusing one whose initial condition the orbit moves.
    fn stamp_integral_dc(
        frame: &rspice_veriloga_runtime::GeneratedPeriodicFrame,
        site: usize,
        operator: &HbGeneratedOperator,
        f: &mut NativeStamp,
    ) -> Result<(), &'static str> {
        let ic = frame.initial_condition(site);
        if ic.is_nan() {
            return Ok(());
        }
        if frame.initial_condition_is_dependent(site) {
            return Err(
                "its initial condition depends on the circuit's unknowns, so it fixes no DC value",
            );
        }
        if operator.initial_condition.map(Value::to_bits) != Some(ic.to_bits()) {
            return Err("its initial condition changes over the period");
        }
        f.stamp_rhs(operator.output + 1, ic);
        Ok(())
    }

    /// Refuse an orbit in which an `idt` input has a mean over the period:
    /// its initial condition fixes the output's DC term, so a nonzero mean is
    /// a ramp no periodic output carries.
    pub(super) fn check_periodic_integrals(
        &self,
        state: &HbSolverState,
        reltol: Value,
        abstol: Value,
    ) -> Result<(), HbError> {
        for (site, operator) in self.operators.iter().enumerate() {
            if !matches!(operator.kind, GeneratedPeriodicKind::Integral) {
                continue;
            }
            let Some((mean, harmonics)) = operator
                .input
                .checked_sub(self.num_nodes)
                .and_then(|branch| state.mna_branch_currents.get(branch))
                .and_then(|spectrum| spectrum.split_first())
            else {
                continue;
            };
            let scale = harmonics
                .iter()
                .map(|value| value.norm())
                .fold(0.0, Value::max);
            if mean.norm() > reltol * scale + abstol {
                return Err(HbError::InvalidCircuit(format!(
                    "harmonic balance cannot carry `idt` site {site} of generated Verilog-A \
                     device '{}': its input has mean {:e} over the period, so its output ramps \
                     and has no periodic steady state",
                    self.name(),
                    mean.re
                )));
            }
        }
        Ok(())
    }

    /// Supply every carried site's output from `solution` to both probes,
    /// seeding `seeded`'s output tangent along its chosen lane.
    fn prepare_frames(&mut self, solution: &[Value], seeded: Option<usize>) {
        let scratch = &mut self.scratch;
        for (output, operator) in scratch.outputs.iter_mut().zip(&self.operators) {
            *output = solution[operator.output];
        }
        for index in 0..scratch.seeds.len() {
            scratch.seeds[index] = if seeded == Some(index) {
                scratch.seed_columns[index].map(|(lane, _)| lane)
            } else {
                None
            };
        }
        for probe in [&mut self.static_probe, &mut self.charge_probe] {
            if let Some(frame) = probe.periodic_frame_mut() {
                frame.prepare(&scratch.outputs, &scratch.seeds);
            }
        }
    }

    /// From the held static probe's record: per site, the first lane its
    /// input was differentiated along that stamps into one of the model's
    /// rows, and every site's input partial along it.
    fn choose_seed_columns(&mut self) {
        let Some(frame) = self.static_probe.periodic_frame() else {
            return;
        };
        let scratch = &mut self.scratch;
        let count = self.operators.len();
        for site in 0..count {
            frame.input_lanes(site, &mut scratch.lanes);
            scratch.seed_columns[site] = scratch.lanes.iter().find_map(|&lane| {
                self.static_probe
                    .lane_unknown(lane, self.num_nodes)
                    .filter(|column| self.rows.binary_search(column).is_ok())
                    .map(|column| (lane, column))
            });
        }
        for seeded in 0..count {
            let lane = scratch.seed_columns[seeded].map(|(lane, _)| lane);
            for site in 0..count {
                scratch.held_partials[site * count + seeded] =
                    lane.and_then(|lane| frame.input_partial(site, lane));
            }
        }
    }

    /// Stamp each carried site's input row `u - x(v, y)` and the partials of
    /// the model's rows and of every site's input with respect to each
    /// site's output, read off one seeded pair of probes per site.
    fn stamp_operator_rows(
        &mut self,
        solution: &[Value],
        f: &mut NativeStamp,
        q: &mut NativeStamp,
    ) -> Result<(), HbError> {
        let h = self.interval;
        let count = self.operators.len();
        let width = self.rows.len();
        let name = self.static_probe.instance_name.as_str();
        let refuse = |site: usize, kind: GeneratedPeriodicKind, reason: &str| {
            let operator = match kind {
                GeneratedPeriodicKind::Derivative => "ddt",
                GeneratedPeriodicKind::Integral => "idt",
            };
            HbError::InvalidCircuit(format!(
                "harmonic balance cannot carry `{operator}` site {site} of generated Verilog-A \
                 device '{name}': {reason}"
            ))
        };
        {
            let frame = self
                .static_probe
                .periodic_frame()
                .ok_or_else(|| refuse(0, self.operators[0].kind, "its frame was lost"))?;
            for (site, operator) in self.operators.iter().enumerate() {
                // The generator refuses a carried site that a bias-dependent
                // condition switches, so one the model did not evaluate here
                // is evaluated on no sample and its output is never read: its
                // input row holds `u = 0`.
                let input = match frame.input(site) {
                    input if input.is_nan() => 0.0,
                    input if input.is_finite() => input,
                    _ => return Err(refuse(site, operator.kind, "its input is not finite")),
                };
                f.stamp_rhs(operator.input + 1, input - solution[operator.input]);
                f.stamp(operator.input + 1, operator.input + 1, 1.0);
                if matches!(operator.kind, GeneratedPeriodicKind::Integral) {
                    Self::stamp_integral_dc(frame, site, operator, f)
                        .map_err(|reason| refuse(site, operator.kind, reason))?;
                }
                frame.input_lanes(site, &mut self.scratch.lanes);
                for &lane in &self.scratch.lanes {
                    if let Some(column) = self.static_probe.lane_unknown(lane, self.num_nodes)
                        && let Some(partial) = frame.input_partial(site, lane)
                    {
                        f.stamp(operator.input + 1, column + 1, -partial);
                    }
                }
            }
        }
        for seeded in 0..count {
            let output = self.operators[seeded].output;
            // A site with no lane to seed is one the model did not evaluate
            // or whose input no unknown moves. Its output then follows from
            // its own network alone, so leaving out the model rows' partials
            // with respect to it can slow Newton but never moves the orbit.
            let Some((lane, column)) = self.scratch.seed_columns[seeded] else {
                continue;
            };
            self.prepare_frames(solution, Some(seeded));
            Self::probe(
                &mut self.static_probe,
                &mut self.matrix,
                ProbeBuffers {
                    rhs: &mut self.rhs,
                    residual: &mut self.static_residual,
                    rows: &self.rows,
                },
                solution,
                self.num_nodes,
                self.simparams,
            )?;
            for (row, col, value) in self.matrix.stored_entries() {
                if col == column
                    && let Ok(slot) = self.rows.binary_search(&row)
                {
                    let difference = value - self.scratch.held_static[seeded * width + slot];
                    if difference != 0.0 {
                        f.stamp(row + 1, output + 1, difference);
                        q.stamp(row + 1, output + 1, -h * difference);
                    }
                }
            }
            if let Some(frame) = self.static_probe.periodic_frame() {
                for (site, operator) in self.operators.iter().enumerate() {
                    let held = self.scratch.held_partials[site * count + seeded].unwrap_or(0.0);
                    let difference = frame.input_partial(site, lane).unwrap_or(held) - held;
                    if difference != 0.0 {
                        f.stamp(operator.input + 1, output + 1, -difference);
                    }
                }
            }
            Self::probe(
                &mut self.charge_probe,
                &mut self.matrix,
                ProbeBuffers {
                    rhs: &mut self.rhs,
                    residual: &mut self.charge_residual,
                    rows: &self.rows,
                },
                solution,
                self.num_nodes,
                self.simparams,
            )?;
            for (row, col, value) in self.matrix.stored_entries() {
                if col == column
                    && let Ok(slot) = self.rows.binary_search(&row)
                {
                    let difference = value - self.scratch.held_charge[seeded * width + slot];
                    if difference != 0.0 {
                        q.stamp(row + 1, output + 1, h * difference);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Keep a held probe's Jacobian entry when it lies in a carried site's
/// seed column: `held[site * rows + slot]`.
fn capture_column(
    seed_columns: &[Option<(u32, usize)>],
    rows: &[usize],
    held: &mut [Value],
    (row, col, value): (usize, usize, Value),
) {
    for (site, seed) in seed_columns.iter().enumerate() {
        if seed.is_some_and(|(_, column)| column == col)
            && let Ok(slot) = rows.binary_search(&row)
        {
            held[site * rows.len() + slot] = value;
        }
    }
}

/// The scratch one probe stamp works in, owned by the sampler and sized once.
struct ProbeBuffers<'a> {
    rhs: &'a mut [Value],
    residual: &'a mut [Value],
    rows: &'a [usize],
}
