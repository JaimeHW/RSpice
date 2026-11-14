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

use super::native_bjt::NativeStamp;
use crate::Value;
use crate::analysis::harmonic_balance::HbError;
use crate::device::MatrixStamper;
use crate::device::veriloga_builtins::BuiltinVerilogAInstance;
use rspice_matrix::StaticMatrix;
use rspice_veriloga_runtime::{
    GeneratedAnalysisKind, GeneratedDdtCoefficients, GeneratedSimulationParameters,
};

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
    pub(super) fn try_new(
        device: BuiltinVerilogAInstance,
        fundamental_hz: Value,
        num_nodes: usize,
        unknowns: usize,
        simparams: GeneratedSimulationParameters,
    ) -> Result<Self, HbError> {
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
            rows,
        };
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
        Ok(sampler)
    }

    pub(super) fn name(&self) -> &str {
        &self.static_probe.instance_name
    }

    /// Zero-based augmented MNA rows the instance stamps.
    pub(super) fn rows(&self) -> &[usize] {
        &self.rows
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
            if value != 0.0 {
                f.stamp(row + 1, col + 1, value);
                q.stamp(row + 1, col + 1, -h * value);
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
            if value != 0.0 {
                q.stamp(row + 1, col + 1, h * value);
            }
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
        Ok(())
    }
}

/// The scratch one probe stamp works in, owned by the sampler and sized once.
struct ProbeBuffers<'a> {
    rhs: &'a mut [Value],
    residual: &'a mut [Value],
    rows: &'a [usize],
}
