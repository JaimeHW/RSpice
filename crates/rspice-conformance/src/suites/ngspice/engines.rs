//! Engine configuration helpers used by the ngspice regression harness.

use super::*;

impl TestRunner {
    pub(super) fn create_dc_engine(&self) -> Engine {
        // Regression harness prioritizes convergence robustness over speed so
        // difficult ngspice decks exercise model behavior instead of solver limits.
        let defaults = SimulationConfig::default();
        let config = SimulationConfig {
            max_iterations: defaults.max_iterations.max(1200),
            convergence_config: ConvergenceConfig::robust(),
            // ngspice regression references run at 27C -> 300.15 K by default.
            temperature: 300.15,
            spice_dialect: rspice_core::engine::SpiceDialect::Ngspice,
            ..defaults
        };
        Engine::new(config)
    }

    pub(super) fn create_dynamic_engine(&self) -> Engine {
        self.create_dynamic_engine_with_locked_grid(None)
    }

    /// Same engine, optionally replaying a fixed accepted-step sequence.
    ///
    /// The grid is passed in at construction rather than assigned afterwards
    /// because `Engine`'s config is not publicly mutable, and should not be:
    /// changing integration settings on a live engine is not a supported
    /// operation for any caller. See [`super::time_analyses`] for why the
    /// locked grid exists.
    pub(super) fn create_dynamic_engine_with_locked_grid(
        &self,
        locked_time_grid: Option<std::sync::Arc<Vec<Value>>>,
    ) -> Engine {
        // Dynamic regression runs should track production transient behavior,
        // while keeping default ambient aligned with ngspice references.
        let config = SimulationConfig {
            locked_time_grid,
            // ngspice transient reference decks default to trapezoidal integration.
            // Fixing method here avoids TrapGear switching artifacts in waveform
            // comparisons while preserving production defaults elsewhere.
            integration_method: rspice_core::analysis::IntegrationMethod::Trapezoidal,
            // ngspice regression references run at 27C -> 300.15 K by default.
            temperature: 300.15,
            // Sub-ps floor improves waveform alignment around steep HFET/MESA edges.
            min_timestep: 1e-12,
            spice_dialect: rspice_core::engine::SpiceDialect::Ngspice,
            ..Default::default()
        };
        Engine::new(config)
    }

    #[inline]
    pub(super) fn simulation_result_contains_non_finite(result: &rspice_core::SimulationResult) -> bool {
        result.node_voltages.iter().any(|value| !value.is_finite())
            || result
                .branch_currents
                .iter()
                .any(|value| !value.is_finite())
    }

    #[inline]
    pub(super) fn dc_sweep_results_contain_non_finite(
        results: &[(Value, rspice_core::SimulationResult)],
    ) -> bool {
        results.iter().any(|(x, result)| {
            !x.is_finite() || Self::simulation_result_contains_non_finite(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ngspice_regression_engines_use_ngspice_dialect() {
        let runner = TestRunner::new(std::env::temp_dir(), TestRunnerConfig::default());

        assert_eq!(
            runner.create_dc_engine().config().spice_dialect,
            rspice_core::engine::SpiceDialect::Ngspice
        );
        assert_eq!(
            runner.create_dynamic_engine().config().spice_dialect,
            rspice_core::engine::SpiceDialect::Ngspice
        );
    }
}
