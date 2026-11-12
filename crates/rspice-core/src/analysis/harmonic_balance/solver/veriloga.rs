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

use super::native_bjt::NativeStamp;
use crate::Value;
use crate::analysis::harmonic_balance::HbError;
use crate::device::MatrixStamper;
use crate::device::veriloga::{VerilogADevice, VerilogAEvaluationMode};
use rspice_veriloga::vm::IntegrationCoefficients;

/// The transient analysis code: `analysis("tran")` reads the same in both
/// probes, so the charge difference never sees an analysis predicate change.
const TRANSIENT_ANALYSIS: u8 = 2;

/// One stamp's residual `F` over the augmented unknowns, and its Jacobian
/// entries `(row, column, dF/dx)`.
type Probe = (Vec<Value>, Vec<(usize, usize, Value)>);

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
}

impl HbVerilogADevice {
    /// Take an instance into the sampler: begin a transient analysis on two
    /// private copies and accept one point of the charge probe at the zero
    /// state, which becomes the reference charge.
    pub(super) fn try_new(
        device: VerilogADevice,
        fundamental_hz: Value,
        unknowns: usize,
    ) -> Result<Self, HbError> {
        let interval = (std::f64::consts::TAU * fundamental_hz).recip();
        let name = device.name.to_string();
        let failure = |phase: &str, error: &dyn std::fmt::Display| {
            HbError::InvalidCircuit(format!(
                "Verilog-A device '{name}' HB {phase} failed: {error}"
            ))
        };
        let companion = IntegrationCoefficients::backward_euler(interval)
            .map_err(|error| failure("charge-probe setup", &error))?;
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
        if let Some(&row) = rows.last()
            && row >= unknowns
        {
            return Err(HbError::InvalidCircuit(format!(
                "Verilog-A device '{name}' stamps MNA row {row}, outside the {unknowns}-unknown periodic state"
            )));
        }
        Ok(Self {
            static_probe,
            charge_probe,
            interval,
            rows,
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

    /// Stamp `F` and `Q - Q_ref` at one collocation sample, with their
    /// Jacobians, in the sampler's `-F` residual convention.
    pub(super) fn stamp_periodic_fq(
        &mut self,
        solution: &[Value],
        f: &mut NativeStamp,
        q: &mut NativeStamp,
    ) -> Result<(), HbError> {
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
        Ok(())
    }
}
