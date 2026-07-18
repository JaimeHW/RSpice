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

    /// Stamp coupled inductor mutual-coupling overlays for transient analysis.
    ///
    /// The standalone inductors stamp their own self-inductance rows; each
    /// pair adds only the -r12 cross terms and mutual history sources.
    pub fn stamp_coupled_inductor_pairs_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            let br1 = self.num_nodes + binding.branch1_ordinal;
            let br2 = self.num_nodes + binding.branch2_ordinal;
            binding
                .device
                .stamp_transient_mutual(br1, br2, dt, coeff, &mut stamper);
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

    /// Replace inductive branch entries in `b - A*x` with DAE residuals
    /// evaluated from current differences. This is the correction-form
    /// counterpart to the absolute companion stamps above.
    pub fn stabilize_inductor_transient_correction_rhs(
        &self,
        correction_rhs: &mut [Value],
        iterate: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        self.inductors.overwrite_transient_correction_rhs(
            correction_rhs,
            iterate,
            dt,
            coeff,
            self.num_nodes,
        );
        for binding in &self.coupled_inductor_pairs {
            let branch1 = self.num_nodes + binding.branch1_ordinal;
            let branch2 = self.num_nodes + binding.branch2_ordinal;
            binding.device.add_transient_mutual_correction_rhs(
                branch1,
                branch2,
                correction_rhs,
                iterate,
                dt,
                coeff,
            );
        }
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .overwrite_transient_correction_rhs(correction_rhs, iterate, dt, coeff);
        }
    }

    /// Update coupled inductor transient history from an accepted solution.
    pub fn update_coupled_inductor_pair_state(&mut self, solution: &[Value]) {
        let num_nodes = self.num_nodes;
        for binding in &mut self.coupled_inductor_pairs {
            let br1 = num_nodes + binding.branch1_ordinal;
            let br2 = num_nodes + binding.branch2_ordinal;
            binding
                .device
                .update_state_with_branches(solution, br1, br2);
        }
    }

    /// Update multi-winding transformer transient history from an accepted solution.
    pub fn update_multi_winding_transformer_state(&mut self, solution: &[Value]) {
        for binding in &mut self.multi_winding_transformers {
            binding.device.update_state_from_solution(solution);
        }
    }
}
