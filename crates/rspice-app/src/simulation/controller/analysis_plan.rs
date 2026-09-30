//! Preflight over a saved analysis plan.
//!
//! Checks that a stored plan still describes something the current design
//! can run — that its saved outputs still exist, and that its analyses are
//! still available — before a run is allowed to start on it.

use super::*;

use crate::simulation::execution::PreparedTask;
use crate::simulation::plan::FrozenSimulationPlan;

impl SimulationController {
    /// Compile a candidate saved output through the same frozen-plan and
    /// prepared-task path used by run preflight. This intentionally does not
    /// consult mutable draft rows after the task specs have been prepared.
    pub fn saved_output_preflight(
        &self,
        state: &AppState,
        output: &crate::state::SavedOutput,
    ) -> rspice_simulation::output_contract::SavedOutputPreflightReport {
        self.saved_outputs_preflight(state, std::slice::from_ref(output))
            .pop()
            .expect("single-output preflight always returns one report")
    }

    /// Compile an output table against one frozen plan/task projection. Model
    /// source sealing and spec construction happen once, independent of the
    /// number of rows rendered by Simulation Studio.
    pub fn saved_outputs_preflight(
        &self,
        state: &AppState,
        outputs: &[crate::state::SavedOutput],
    ) -> Vec<rspice_simulation::output_contract::SavedOutputPreflightReport> {
        if outputs.is_empty() {
            return Vec::new();
        }
        let plan = match Self::build_analysis_plan(&state.sim_setup) {
            Ok(plan) => plan,
            Err(errors) => {
                return invalid_saved_output_reports(outputs.len(), errors.join("; "));
            }
        };
        // Rendering an output table is not authorization to run, so a project
        // without an attached technology compiles it from the model library.
        let sealed_sources = if state.project_technology_in_effect() {
            state.seal_project_execution_model_sources()
        } else {
            state
                .model_library_manager
                .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
        };
        let sealed_models = match sealed_sources {
            Ok(sealed) => sealed,
            Err(error) => {
                return invalid_saved_output_reports(outputs.len(), error);
            }
        };
        let tasks = match Self::build_queue_from_plan(state, &plan, &sealed_models) {
            Ok(tasks) => tasks,
            Err(errors) => {
                return invalid_saved_output_reports(outputs.len(), errors.join("; "));
            }
        };
        outputs
            .iter()
            .map(|output| {
                rspice_simulation::output_contract::preflight_saved_output(
                    output,
                    tasks
                        .iter()
                        .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
                    crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES,
                )
            })
            .collect()
    }

    pub(super) fn build_analysis_plan(
        state: &SimulationSetup,
    ) -> Result<FrozenSimulationPlan, Vec<String>> {
        let plan = state.analysis_plan.as_ref().ok_or_else(|| {
            vec![
                "The simulation plan has not been migrated to stable analysis instances".to_owned(),
            ]
        })?;
        plan.freeze().map_err(|error| vec![error.to_string()])
    }

    pub(super) fn build_queue_from_plan(
        state: &AppState,
        plan: &FrozenSimulationPlan,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<Vec<PreparedTask>, Vec<String>> {
        crate::simulation::execution::prepare_plan_tasks(
            analysis_inputs(state),
            plan,
            sealed_model_sources,
            &state.simulation.imported_monte_carlo_checkpoints,
            state.schematic.session.current_file.as_deref(),
        )
    }
}

fn invalid_saved_output_reports(
    count: usize,
    reason: impl Into<String>,
) -> Vec<rspice_simulation::output_contract::SavedOutputPreflightReport> {
    let report = rspice_simulation::output_contract::SavedOutputPreflightReport::invalid(reason);
    vec![report; count]
}

#[cfg(test)]
mod tests;
