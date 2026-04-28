//! HB result construction from converged spectral state.

use super::*;
impl HbSolver {
    /// Build HbResult from solver state
    pub fn build_result(&self, state: &HbSolverState) -> HbResult {
        let mut result = HbResult::new(
            self.config.fundamental_freq,
            self.num_nodes,
            self.num_harmonics,
        );

        result.converged = state.converged;
        result.iterations = state.iteration;
        result.residual_norm = state.residual_norm;
        result.node_names = self.node_names.clone();

        // Copy spectral voltages
        for (node, spectrum) in state.x.iter().enumerate() {
            let mut sv = SpectralVoltage::new(
                self.node_names.get(node).cloned().unwrap_or_default(),
                self.num_harmonics,
            );
            sv.coefficients = spectrum.clone();
            sv.frequencies = self.config.harmonic_frequencies();
            result.spectral_voltages.push(sv);
        }

        result
    }
}
