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
        // ngspice transient reference decks default to trapezoidal integration,
        // and pinning it for free-running comparisons avoids TrapGear switching
        // artifacts while preserving production defaults elsewhere.
        //
        // Locked replay is the exception, because ngspice's `trap` is *variable
        // order* -- it carries order 1 and 2 and drops to 1 under its own error
        // control -- while `Trapezoidal` here is fixed order 2. That difference
        // is invisible while RSpice picks its own steps and decisive when it
        // replays someone else's: a reference axis is coarse wherever the
        // producing run judged the waveform slow, and fixed-order trapezoidal
        // carries its full truncation error across those steps. On
        // `general/mosamp.cir` the reference leaves a 65 ns step through an
        // amplifier slew, where the trapezoidal rule's own error is 4.5% -- the
        // method reproducing the analytic RC step response exactly as
        // Pade(1,1), not a defect in it. The hybrid damps where ngspice's order
        // control damps, and tracks the same reference to 0.4%.
        let integration_method = if locked_time_grid.is_some() {
            rspice_core::analysis::IntegrationMethod::TrapGear
        } else {
            rspice_core::analysis::IntegrationMethod::Trapezoidal
        };
        let config = SimulationConfig {
            locked_time_grid,
            integration_method,
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
