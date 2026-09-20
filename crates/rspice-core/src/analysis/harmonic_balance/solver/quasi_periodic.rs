//! Reuse registered physical devices and exact MNA for a driven QPSS solve.
use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicError as Error, QuasiPeriodicGrid, QuasiPeriodicSolution,
    QuasiPeriodicSolveConfig,
    solve::{self, Circuit, LinearEntry, Sample},
};
use std::sync::Arc;

fn device_error(error: HbError) -> Error {
    match error {
        HbError::Aborted => Error::Aborted,
        other => Error::InvalidCircuit(other.to_string()),
    }
}

impl HbSolver {
    /// Solve the registered circuit on independent tone phases. `sources`
    /// supplies the entire MNA right hand side in full signed Fourier-series
    /// coefficients (a cosine of peak A has coefficients A/2). Existing HB
    /// harmonic source tables and the HB fundamental are not used.
    ///
    /// Branch devices require the canonical exact-MNA registry. The current
    /// analytic dense backend permits at most 512 real spectral unknowns;
    /// caller resource limits can further restrict it. The result certifies
    /// the retained Galerkin equations, not truncation or aliasing error.
    pub fn solve_quasi_periodic_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicSolveConfig,
        sources: &[Vec<Complex64>],
        seed: Option<&[Vec<Complex64>]>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicSolution, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        self.validate_quasi_periodic_circuit()?;
        solve::solve_with_abort(self, grid, config, sources, seed, limits, abort)
    }

    fn validate_quasi_periodic_circuit(&self) -> Result<(), Error> {
        self.validate_nonlinear_device_parameters()
            .map_err(device_error)?;
        if !self.l_matrix.is_empty() {
            return Err(Error::InvalidCircuit(
                "QPSS inductors require exact branch equations".into(),
            ));
        }
        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            return Err(Error::InvalidCircuit(
                "QPSS Verilog-A F/Q sampling is not connected".into(),
            ));
        }
        if self.periodic_mna_branches.len() != self.periodic_mna_branch_names.len() {
            return Err(Error::InvalidCircuit(
                "QPSS branch descriptors and names are misaligned".into(),
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for (index, branch) in self.periodic_mna_branches.iter().enumerate() {
            let (ordinal, pos, neg) = branch.ordinal_and_terminals();
            if ordinal != index + 1 || pos > self.num_nodes || neg > self.num_nodes || pos == neg {
                return Err(Error::InvalidCircuit(
                    "QPSS branch has invalid canonical MNA coordinates".into(),
                ));
            }
            match branch {
                ExactMnaBranch::Inductor { inductance, .. }
                    if !inductance.is_finite() || *inductance == 0.0 =>
                {
                    return Err(Error::InvalidCircuit(
                        "QPSS branch inductance must be finite and nonzero".into(),
                    ));
                }
                ExactMnaBranch::VoltageSource { source_index, .. } => {
                    if !seen.insert(*source_index) {
                        return Err(Error::InvalidCircuit(
                            "QPSS source has duplicate branch equations".into(),
                        ));
                    }
                    if let Some(source) = self.voltage_source_branches.get(*source_index)
                        && (source.node_pos != pos || source.node_neg != neg)
                    {
                        return Err(Error::InvalidCircuit(
                            "QPSS voltage source terminals disagree with its branch".into(),
                        ));
                    }
                }
                _ => {}
            }
        }
        if (0..self.voltage_source_branches.len()).any(|index| !seen.contains(&index)) {
            return Err(Error::InvalidCircuit(
                "QPSS voltage source is missing its exact branch equation".into(),
            ));
        }
        Ok(())
    }
}

impl Circuit for HbSolver {
    fn unknowns(&self) -> usize {
        self.num_nodes
            .saturating_add(self.periodic_mna_branches.len())
    }

    fn voltage_equation(&self, row: usize) -> bool {
        row >= self.num_nodes
    }

    fn linear_entries(&self, frequency_hz: Value) -> Result<Vec<LinearEntry>, Error> {
        let omega = std::f64::consts::TAU * frequency_hz;
        let mut entries = Vec::new();
        for (matrix, reactive) in [(&self.g_matrix, false), (&self.c_matrix, true)] {
            for &(row, col, value) in matrix {
                if row >= self.num_nodes || col >= self.num_nodes {
                    return Err(Error::InvalidCircuit(
                        "QPSS nodal stamp is outside the node table".into(),
                    ));
                }
                let coefficient = if reactive {
                    Complex64::new(0.0, omega * value)
                } else {
                    Complex64::new(value, 0.0)
                };
                entries.push((row, col, coefficient));
            }
        }
        for (index, branch) in self.periodic_mna_branches.iter().enumerate() {
            let row = self.num_nodes + index;
            let (_, pos, neg) = branch.ordinal_and_terminals();
            for (node, sign) in [(pos, 1.0), (neg, -1.0)] {
                if node > 0 {
                    entries.push((node - 1, row, Complex64::new(sign, 0.0)));
                    if !matches!(branch, ExactMnaBranch::ConstitutivePort { .. }) {
                        entries.push((row, node - 1, Complex64::new(sign, 0.0)));
                    }
                }
            }
            match branch {
                ExactMnaBranch::Inductor { inductance, .. } => {
                    entries.push((row, row, Complex64::new(0.0, -omega * inductance)));
                }
                ExactMnaBranch::Resistor { resistance, .. } => {
                    // Large-signal equations use the physical DC resistance;
                    // an authored AC= override belongs to linearized analyses.
                    entries.push((row, row, Complex64::new(-resistance, 0.0)));
                }
                _ => {}
            }
        }
        for &(row, col, value) in &self.exact_mna_static_entries {
            entries.push((row, col, Complex64::new(value, 0.0)));
        }
        for &(row, col, value) in &self.exact_mna_inductance_entries {
            entries.push((row, col, Complex64::new(0.0, -omega * value)));
        }
        for network in &self.exact_periodic_networks {
            network
                .try_visit_direct_entries(omega, self.unknowns(), |row, col, value| {
                    entries.push((row, col, value))
                })
                .map_err(device_error)?;
        }
        Ok(entries)
    }

    fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error> {
        let mut sample = self
            .quasi_periodic_native_sample(state, jacobian)
            .map_err(device_error)?;
        // Legacy compact devices use num_nodes as the ground sentinel.
        // Including branch-current coordinates would turn ground into the
        // first branch current when evaluating their terminal voltages.
        let voltages = &state[..self.num_nodes];
        for device in &self.nonlinear_devices {
            sample.current.extend(
                device
                    .evaluate(voltages)
                    .into_iter()
                    .filter(|(row, _)| *row < self.num_nodes),
            );
            sample.charge.extend(
                device
                    .charge(voltages)
                    .into_iter()
                    .filter(|(row, _)| *row < self.num_nodes),
            );
            if jacobian {
                for (derivatives, target) in [
                    (device.jacobian(voltages), &mut sample.conductance),
                    (device.charge_jacobian(voltages), &mut sample.capacitance),
                ] {
                    target.extend(derivatives.into_iter().filter_map(|((row, col), value)| {
                        (row < self.num_nodes && col < self.num_nodes).then_some((row, col, value))
                    }));
                }
            }
        }
        Ok(sample)
    }
}

#[cfg(test)]
mod tests;
