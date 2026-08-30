//! HB result construction from converged spectral state.

use super::*;
impl HbSolver {
    /// Build an HB result only after proving that the retained node and MNA
    /// branch rows match the solver's canonical basis.
    pub(crate) fn build_result(&self, state: &HbSolverState) -> Result<HbResult, HbError> {
        let harmonic_count = self.num_harmonics.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit("HB result harmonic count exceeds this platform".to_string())
        })?;
        if self.node_names.len() != self.num_nodes || state.x.len() != self.num_nodes {
            return Err(HbError::InvalidCircuit(format!(
                "HB result construction has {} names and {} spectra for {} nodes",
                self.node_names.len(),
                state.x.len(),
                self.num_nodes
            )));
        }
        for (node, (name, spectrum)) in self.node_names.iter().zip(&state.x).enumerate() {
            if name.is_empty() || name.trim() != name || spectrum.len() != harmonic_count {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result node row {node} has a non-canonical name or {} coefficients; expected {harmonic_count}",
                    spectrum.len()
                )));
            }
            if spectrum
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result node '{name}' contains a non-finite coefficient"
                )));
            }
            if spectrum
                .first()
                .is_some_and(|coefficient| coefficient.im != 0.0)
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result node '{name}' has a nonzero imaginary DC coefficient"
                )));
            }
        }
        if !state.residual_norm.is_finite() || state.residual_norm < 0.0 {
            return Err(HbError::InvalidCircuit(
                "HB result residual norm is invalid".to_string(),
            ));
        }

        let branch_names = if self.periodic_mna_branches.is_empty() {
            if state.mna_branch_currents.len() != self.voltage_source_branch_names.len() {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result has {} MNA-current rows for {} voltage-source names",
                    state.mna_branch_currents.len(),
                    self.voltage_source_branch_names.len()
                )));
            }
            let mut names = Vec::new();
            names
                .try_reserve_exact(self.voltage_source_branch_names.len())
                .map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "HB result branch-name allocation failed: {error}"
                    ))
                })?;
            for name in &self.voltage_source_branch_names {
                if name.is_empty() || name.trim() != name {
                    return Err(HbError::InvalidCircuit(
                        "HB result contains a non-canonical voltage-source name".to_string(),
                    ));
                }
                names.push(name.clone());
            }
            names
        } else {
            let names = self.try_periodic_mna_branch_names()?;
            if state.mna_branch_currents.len() != names.len() {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result has {} MNA-current rows for {} canonical branch names",
                    state.mna_branch_currents.len(),
                    names.len()
                )));
            }
            names
        };
        for (branch, (name, spectrum)) in branch_names
            .iter()
            .zip(&state.mna_branch_currents)
            .enumerate()
        {
            if spectrum.len() != harmonic_count {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result MNA branch '{name}' at ordinal {} contains {} coefficients; expected {harmonic_count}",
                    branch + 1,
                    spectrum.len()
                )));
            }
            if spectrum
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result MNA branch '{name}' contains a non-finite coefficient"
                )));
            }
            if spectrum
                .first()
                .is_some_and(|coefficient| coefficient.im != 0.0)
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB result MNA branch '{name}' has a nonzero imaginary DC coefficient"
                )));
            }
        }

        let mut result = HbResult::new(
            self.config.fundamental_freq,
            self.num_nodes,
            self.num_harmonics,
        );

        result.converged = state.converged;
        result.iterations = state.total_iterations.max(state.iteration);
        result.residual_norm = state.residual_norm;
        result.node_names = self.node_names.clone();

        // Copy spectral voltages, converting the solver's internal Fourier
        // coefficients to amplitude phasors at the boundary: harmonic k
        // contributes 2*Re(c_k e^{jk w t}) to the waveform, so the reported
        // amplitude of harmonic k >= 1 is 2*c_k while DC passes through.
        for (node, spectrum) in state.x.iter().enumerate() {
            let mut sv = SpectralVoltage::new(
                self.node_names.get(node).cloned().unwrap_or_default(),
                self.num_harmonics,
            );
            sv.coefficients = spectrum
                .iter()
                .enumerate()
                .map(|(k, &c)| if k == 0 { c } else { c * 2.0 })
                .collect();
            sv.frequencies = self.config.harmonic_frequencies();
            result.spectral_voltages.push(sv);
        }

        for (branch_idx, spectrum) in state.mna_branch_currents.iter().enumerate() {
            result.mna_branch_currents.push(SpectralBranchCurrent {
                device_name: branch_names[branch_idx].clone(),
                coefficients: spectrum
                    .iter()
                    .enumerate()
                    .map(|(harmonic, &coefficient)| {
                        if harmonic == 0 {
                            coefficient
                        } else {
                            coefficient * 2.0
                        }
                    })
                    .collect(),
                frequencies: self.config.harmonic_frequencies(),
            });
        }

        Ok(result)
    }
}
