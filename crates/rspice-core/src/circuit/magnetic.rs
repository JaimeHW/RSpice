use super::*;

impl CircuitData {
    /// Refresh effective inductance values for all Jiles-Atherton inductors.
    ///
    /// Call this with the latest solution vector before transient companion
    /// stamping so nonlinear core state updates feed into the MNA coefficients.
    pub fn refresh_jiles_atherton_inductances(&mut self, solution: &[Value]) {
        use crate::device::NonlinearDevice;

        let num_nodes = self.num_nodes;
        for idx in 0..self.jiles_atherton_inductors.len() {
            let (inductor_index, l_eff) = {
                let binding = &mut self.jiles_atherton_inductors[idx];
                let branch_matrix_index = num_nodes + binding.branch_ordinal;
                binding.device.set_branch_index(branch_matrix_index);
                binding.device.update(solution);
                (
                    binding.inductor_index,
                    binding.device.effective_inductance(),
                )
            };

            if let Some(slot) = self.inductors.inductances.get_mut(inductor_index)
                && l_eff.is_finite()
                && l_eff > 0.0
            {
                *slot = l_eff.max(1e-18);
            }
        }
    }

    /// Stamp coupled inductor companion models for transient analysis.
    pub fn stamp_coupled_inductor_pairs_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            binding
                .device
                .stamp_transient_companion(dt, coeff, &mut stamper, &mut []);
        }
    }

    /// Stamp multi-winding transformer companion models for transient analysis.
    pub fn stamp_multi_winding_transformers_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .stamp_transient_companion(dt, coeff, &mut stamper, &mut []);
        }
    }

    /// Update coupled inductor transient history from an accepted solution.
    pub fn update_coupled_inductor_pair_state(&mut self, solution: &[Value]) {
        for binding in &mut self.coupled_inductor_pairs {
            binding.device.update_state_from_solution(solution);
        }
    }

    /// Update multi-winding transformer transient history from an accepted solution.
    pub fn update_multi_winding_transformer_state(&mut self, solution: &[Value]) {
        for binding in &mut self.multi_winding_transformers {
            binding.device.update_state_from_solution(solution);
        }
    }
}
