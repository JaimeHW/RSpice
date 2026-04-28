//! Engine configuration helpers used by the ngspice regression harness.

use super::*;

impl TestRunner {
    pub(super) fn create_dc_engine(&self) -> Engine {
        // Regression harness prioritizes convergence robustness over speed so
        // difficult ngspice decks exercise model behavior instead of solver limits.
        let mut config = SimulationConfig::default();
        config.max_iterations = config.max_iterations.max(1200);
        config.convergence_config = ConvergenceConfig::robust();
        // ngspice regression references run at 27C -> 300.15 K by default.
        config.temperature = 300.15;
        Engine::new(config)
    }

    pub(super) fn create_dynamic_engine(&self) -> Engine {
        // Dynamic regression runs should track production transient behavior,
        // while keeping default ambient aligned with ngspice references.
        let mut config = SimulationConfig::default();
        // ngspice transient reference decks default to trapezoidal integration.
        // Fixing method here avoids TrapGear switching artifacts in waveform
        // comparisons while preserving production defaults elsewhere.
        config.integration_method = crate::analysis::IntegrationMethod::Trapezoidal;
        // ngspice regression references run at 27C -> 300.15 K by default.
        config.temperature = 300.15;
        // Sub-ps floor improves waveform alignment around steep HFET/MESA edges.
        config.min_timestep = 1e-12;
        Engine::new(config)
    }

    #[inline]
    pub(super) fn simulation_result_contains_non_finite(result: &crate::SimulationResult) -> bool {
        result.node_voltages.iter().any(|value| !value.is_finite())
            || result
                .branch_currents
                .iter()
                .any(|value| !value.is_finite())
    }

    #[inline]
    pub(super) fn dc_sweep_results_contain_non_finite(
        results: &[(Value, crate::SimulationResult)],
    ) -> bool {
        results.iter().any(|(x, result)| {
            !x.is_finite() || Self::simulation_result_contains_non_finite(result)
        })
    }
}
