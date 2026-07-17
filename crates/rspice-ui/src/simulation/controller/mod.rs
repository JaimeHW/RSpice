//! Simulation Controller - Orchestrates Simulation Execution
//!
//! This module provides the orchestration layer between the UI state and the
//! simulation runner. It handles:
//!
//! - Processing `trigger_simulation` flag from UI
//! - Generating netlist from schematic
//! - Starting simulation with appropriate config
//! - Polling for completion and updating results
//! - **Multi-analysis execution**: Running all enabled analyses sequentially
//!
//! # Usage
//!
//! Call `SimulationController::update()` once per frame in the app update loop.

use std::collections::{HashSet, VecDeque};
#[cfg(test)]
use std::path::PathBuf;

use crate::common::app::{AppState, ConsoleMessage};
use crate::common::export_workflow::ExportWorkflowIo;
use crate::io::{SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter};
use crate::services::yield_manager::YieldAnalysisManager;
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, DcSweepConfig, NoiseAnalysisConfig, PoleZeroConfig,
    PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::execution::TouchstoneExportPolicy;
use crate::simulation::multi_run::{
    AnalysisRunType, AnalysisSpec, FrequencySweep, HbToneSpec, OptimizationAlgorithm,
    OptimizationGoal, OptimizationVariable, SpPort,
};
use crate::simulation::output_contract::{PreparedSavedOutput, materialize_saved_outputs};
use crate::simulation::runner::SimulationError;
use crate::simulation::runner::SpecExecutionOptions;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, ComplexResultValue, DcOpResult,
    MonteCarloVariableMetadata, OperatingPointValue, ReliabilityCheckpointEvidence,
    ReliabilityDeviceEvidence, ReliabilityShiftEvidence, ReliabilityStressEvidence,
    SensitivityResultMode, SensitivityResultRow, SimulationRunIntent, SimulationRunLifecycle,
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence,
};

mod analysis_commands;
mod analysis_helpers;
mod analysis_plan;
mod analysis_run_config;
mod analysis_spec_build;
mod manual_deck;
mod prepared_run;
mod results_convert;
mod results_post;
mod results_update;
pub(crate) mod spice_value;
mod touchstone;
mod transient_post;
pub(crate) use transient_post::DerivedViewerLoadState;

use self::spice_value::parse_spice_value_checked;

#[derive(Debug, Clone)]
pub(super) struct QueuedAnalysis {
    pub(super) spec: AnalysisSpec,
    pub(super) config: Option<AnalysisConfig>,
    pub(super) spec_options: SpecExecutionOptions,
    pub(super) analysis_line: String,
}

//=============================================================================
// Simulation Controller
//=============================================================================

/// Orchestrates simulation execution from UI trigger to result display
///
/// Supports commercial-grade multi-analysis execution where multiple enabled
/// analyses (DC OP, Transient, AC, DC Sweep) run sequentially with all results
/// stored under a single simulation run.
pub struct SimulationController {
    /// The background simulation runner
    runner: SimulationRunner,
    /// Manager for yield analysis (Monte Carlo)
    yield_manager: YieldAnalysisManager,
    /// Current analysis config (stored for result processing when polling completes)
    current_config: Option<AnalysisConfig>,
    /// Current strongly-typed analysis spec (always set while running)
    current_spec: Option<AnalysisSpec>,
    /// Frozen identity of the prepared task currently owned by the runner.
    /// Captured before the authorized dispatch token is moved into the runner.
    current_provenance: Option<AnalysisResultProvenance>,
    /// Immutable output contracts authenticated with the current task before
    /// its dispatch token is moved into the runner.
    current_saved_output_contracts: Vec<PreparedSavedOutput>,
    /// Source domain authenticated by the active run dispatch. Manual-deck
    /// task IDs are deterministic source projections, not plan-owned IDs.
    current_source_domain: AnalysisResultSourceDomain,
    /// Stable run ID that owns the in-flight batch.
    current_run_id: Option<u64>,

    // =========================================================================
    // Multi-Analysis Queue
    // =========================================================================
    /// Queue of pending analyses to run in current simulation batch
    pending_analyses: VecDeque<crate::simulation::execution::AuthorizedTaskDispatch>,
    /// Exact prepared instances that completed successfully in this batch.
    /// A dependent task is dispatchable only when every frozen prerequisite
    /// appears here; failed and skipped tasks never grant dependency authority.
    successful_analysis_instances: HashSet<crate::product::AnalysisInstanceId>,
    /// Current analysis index (1-based for display: "Analysis 2/4")
    current_analysis_idx: usize,
    /// Total number of analyses in current batch
    total_analyses: usize,
    /// Cached netlist for multi-analysis runs (avoids regeneration)
    cached_netlist: Option<String>,
    /// Runtime coordinator for transient-derived viewer data (eye/FFT).
    transient_post: transient_post::TransientPostCoordinator,
    /// App-state design epoch this controller's runner/queue belong to.
    design_execution_epoch: u64,
    /// Single outstanding immutable preflight result, if any.
    pending_prepared_run: Option<prepared_run::PendingPreparedRun>,
    /// Run-bound automatic export policy captured by immutable preflight.
    touchstone_export_policy: TouchstoneExportPolicy,
    /// Generation-safe authority shared by every prepared dispatch token.
    execution_permits: crate::simulation::execution::ExecutionPermitIssuer,
}

impl Default for SimulationController {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationController {
    /// Create a new simulation controller
    pub fn new() -> Self {
        Self {
            runner: SimulationRunner::new(),
            yield_manager: YieldAnalysisManager::new(),
            current_config: None,
            current_spec: None,
            current_provenance: None,
            current_saved_output_contracts: Vec::new(),
            current_source_domain: AnalysisResultSourceDomain::SimulationPlan,
            current_run_id: None,
            pending_analyses: VecDeque::new(),
            successful_analysis_instances: HashSet::new(),
            current_analysis_idx: 0,
            total_analyses: 0,
            cached_netlist: None,
            transient_post: transient_post::TransientPostCoordinator::default(),
            design_execution_epoch: 0,
            pending_prepared_run: None,
            touchstone_export_policy: TouchstoneExportPolicy::disabled(),
            execution_permits: crate::simulation::execution::ExecutionPermitIssuer::default(),
        }
    }

    /// Process simulation state updates
    ///
    /// Call this once per frame in the app update loop.
    pub(crate) fn update(
        &mut self,
        state: &mut AppState,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) {
        self.reset_if_design_replaced(state);

        // Handle simulation trigger
        if state.simulation.trigger_simulation {
            log::info!(
                "Simulation triggered ({} analyses enabled)",
                state.sim_setup.enabled_analysis_instance_count()
            );
            state.simulation.trigger_simulation = false;
            self.start_simulation(state);
        }

        // Handle abort trigger
        if state.simulation.trigger_abort {
            log::info!("Simulation abort triggered!");
            state.simulation.trigger_abort = false;
            let requested_execution = state.simulation.abort_request.take();
            let controller_execution = self.current_run_id.and_then(|run_sequence| {
                state
                    .simulation
                    .run_by_sequence(run_sequence)
                    .and_then(|run| run.execution_identity())
            });
            let bound_request = requested_execution.filter(|requested| {
                Some(*requested) == controller_execution
                    && Some(*requested) == state.simulation.active_execution
            });
            if let Some(requested_execution) = bound_request {
                let cancellation_result = state
                    .simulation
                    .run_by_stable_id_mut(requested_execution.run_id)
                    .ok_or_else(|| {
                        "The active simulation run disappeared before cancellation could be acknowledged"
                            .to_owned()
                    })
                    .and_then(|run| run.mark_cancelling());
                if let Err(error) = cancellation_result {
                    state.push_sim_message(ConsoleMessage::error(format!(
                        "Simulation cancellation was rejected: {error}"
                    )));
                } else {
                    self.runner.abort();
                    // Stop the batch from dispatching any additional task, but
                    // retain the active task metadata until the runner returns
                    // its abort acknowledgement. That acknowledgement is the
                    // authority for the terminal lifecycle and duration.
                    self.pending_analyses.clear();
                    state.simulation.status = "Cancelling".to_owned();
                    state.push_sim_message(ConsoleMessage::warning(
                        "Simulation cancellation requested".to_owned(),
                    ));
                }
            } else {
                state.push_sim_message(ConsoleMessage::warning(
                    "Ignored a stale or unbound simulation cancellation request; the active execution was left intact"
                        .to_owned(),
                ));
            }
        }

        // Poll for completion
        self.poll_completion(state, export_io);

        // Apply/cancel background transient post-processing work after any
        // selection changes that happened during the previous frame.
        self.sync_transient_post_views(state);

        // Update running state
        let is_running = self.runner.is_running();
        state.simulation.progress =
            Self::ui_progress_fraction(self.runner.progress_fraction(), is_running);
        state.simulation.is_running = is_running;
    }

    /// Start a new simulation batch
    ///
    /// Builds all enabled analyses into a queue and starts the first one.
    /// Subsequent analyses are started automatically upon completion.
    fn start_simulation(&mut self, state: &mut AppState) {
        log::info!("start_simulation called");
        self.design_execution_epoch = state.design_execution_epoch;

        match state.simulation.run_intent {
            SimulationRunIntent::SimulateRunSet => self.start_simulate_run_set(state),
            SimulationRunIntent::ManualDeck => self.start_manual_deck_simulation(state),
        }
    }

    fn start_authorized_snapshot(&mut self, state: &mut AppState) {
        if self.has_active_batch() {
            state.push_sim_message(ConsoleMessage::warning(
                "A simulation batch is already active; wait for it to finish or abort it before starting another run"
                    .to_owned(),
            ));
            return;
        }

        let mut dispatch = match self.consume_snapshot_for_dispatch(state) {
            Ok(dispatch) => dispatch,
            Err(error) => {
                state.push_sim_message(ConsoleMessage::warning(error.to_string()));
                state.simulation.status = "Run blocked".to_owned();
                return;
            }
        };
        let source_domain = match dispatch.intent() {
            SimulationRunIntent::SimulateRunSet => AnalysisResultSourceDomain::SimulationPlan,
            SimulationRunIntent::ManualDeck => AnalysisResultSourceDomain::ManualDeck,
        };
        let run_receipt = match dispatch.prepared_run_receipt(source_domain) {
            Ok(receipt) => receipt,
            Err(error) => {
                state.push_sim_message(ConsoleMessage::error(error.to_string()));
                state.simulation.status = "Run blocked".to_owned();
                return;
            }
        };

        self.pending_analyses.clear();
        self.successful_analysis_instances.clear();
        self.current_source_domain = source_domain;
        self.total_analyses = dispatch.task_count();
        self.current_analysis_idx = 0;
        self.cached_netlist = Some(dispatch.executable_netlist().to_owned());

        if let Some(cross_probe) = dispatch.take_cross_probe() {
            cross_probe.apply(state);
        }
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        for advisory in dispatch.advisories() {
            state.push_sim_message(ConsoleMessage::warning(advisory.clone()));
        }

        let queued_names = dispatch
            .tasks()
            .map(|entry| entry.label())
            .collect::<Vec<_>>();
        log::info!(
            "Dispatching prepared snapshot {} with {} ordered task(s): {:?}",
            dispatch.digest(),
            self.total_analyses,
            queued_names
        );

        let run = state.simulation.start_prepared_run(run_receipt);
        let run_id = run.id;
        let execution_identity = run
            .execution_identity()
            .expect("current simulation runs always allocate job identity");
        self.current_run_id = Some(run_id);
        state.simulation.active_execution = Some(execution_identity);
        state.simulation.abort_request = None;
        if dispatch.intent() == SimulationRunIntent::ManualDeck {
            let manual_source = dispatch.manual_source().unwrap_or_default().to_owned();
            state.ui.netlist.pending_manual_run_id = Some(run_id);
            state.ui.netlist.pending_run_buffer = Some(manual_source);
            state.push_sim_message(ConsoleMessage::info(
                "Running sealed manually edited netlist source".to_owned(),
            ));
        }

        if self.total_analyses > 1 {
            state.push_sim_message(ConsoleMessage::info(format!(
                "Starting simulation batch: {} analyses",
                self.total_analyses
            )));
        }
        self.pending_analyses = dispatch.into_tasks();
        self.start_next_analysis(state);
    }

    /// Treat both controller-owned batch metadata and runner-local work as
    /// authoritative. In particular, `is_running()` alone is insufficient: a
    /// native worker can have finished while its result is still unpolled.
    fn has_active_batch(&self) -> bool {
        self.current_run_id.is_some()
            || self.current_spec.is_some()
            || self.current_config.is_some()
            || self.current_provenance.is_some()
            || !self.current_saved_output_contracts.is_empty()
            || self.total_analyses != 0
            || !self.pending_analyses.is_empty()
            || !self.runner.can_accept_prepared_task()
    }

    #[cfg(test)]
    fn analysis_source_path(state: &AppState) -> Option<PathBuf> {
        match state.simulation.run_intent {
            SimulationRunIntent::ManualDeck => state.workspace.netlist_source_path.clone(),
            SimulationRunIntent::SimulateRunSet => state.schematic.current_file.clone(),
        }
    }

    fn reset_if_design_replaced(&mut self, state: &mut AppState) {
        if self.design_execution_epoch == state.design_execution_epoch {
            return;
        }

        let interrupted = self.current_run_id.and_then(|run_sequence| {
            state
                .simulation
                .run_by_sequence_mut(run_sequence)
                .filter(|run| !run.lifecycle.is_terminal())
        });
        if let Some(run) = interrupted {
            run.success = false;
            if let Err(error) = run.finish_lifecycle(SimulationRunLifecycle::Interrupted) {
                log::error!("Failed to seal interrupted simulation run lifecycle: {error}");
            } else {
                state.push_sim_message(ConsoleMessage::warning(
                    "Simulation execution was interrupted because its design context changed"
                        .to_owned(),
                ));
            }
        }
        self.reset_for_design_replacement();
        state.simulation.active_execution = None;
        state.simulation.abort_request = None;
        state.simulation.trigger_abort = false;
        state.ui.netlist.pending_manual_run_id = None;
        state.ui.netlist.pending_run_buffer = None;
        self.design_execution_epoch = state.design_execution_epoch;
    }

    fn reset_for_design_replacement(&mut self) {
        self.runner.reset_for_design_replacement();
        self.pending_analyses.clear();
        self.successful_analysis_instances.clear();
        self.cached_netlist = None;
        self.clear_prepared_run();
        self.current_config = None;
        self.current_spec = None;
        self.current_provenance = None;
        self.current_saved_output_contracts.clear();
        self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
        self.current_run_id = None;
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        self.current_analysis_idx = 0;
        self.total_analyses = 0;
        self.transient_post = transient_post::TransientPostCoordinator::default();
    }

    fn start_manual_deck_simulation(&mut self, state: &mut AppState) {
        self.start_authorized_snapshot(state);
    }
    fn start_simulate_run_set(&mut self, state: &mut AppState) {
        self.start_authorized_snapshot(state);
    }
    /// Start the next analysis in the queue
    ///
    /// Called after start_simulation() initializes the queue, and again
    /// after each analysis completes until the queue is empty.
    fn start_next_analysis(&mut self, state: &mut AppState) {
        if !self.runner.can_accept_prepared_task() {
            log::error!(
                "Refusing to dequeue a prepared analysis while the runner still owns active or unpolled work"
            );
            return;
        }
        self.current_config = None;
        self.current_spec = None;
        self.current_provenance = None;
        self.current_saved_output_contracts.clear();
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();

        let mut skipped_blocked_task = false;
        let next_analysis = loop {
            let Some(candidate) = self.pending_analyses.pop_front() else {
                if skipped_blocked_task {
                    self.finish_simulation_batch(state);
                } else {
                    log::warn!("start_next_analysis called with empty queue");
                }
                return;
            };
            let unavailable_dependencies = candidate
                .dependencies()
                .iter()
                .copied()
                .filter(|dependency| !self.successful_analysis_instances.contains(dependency))
                .collect::<Vec<_>>();
            if unavailable_dependencies.is_empty() {
                break candidate;
            }

            skipped_blocked_task = true;
            self.current_analysis_idx += 1;
            let analysis_name = candidate.label().to_owned();
            let unavailable = unavailable_dependencies
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            let message = format!(
                "Skipped {analysis_name}: prerequisite analysis result(s) {unavailable} did not complete successfully"
            );
            let provenance = match AnalysisResultProvenance::new_with_source_domain(
                self.current_source_domain,
                candidate.instance_id(),
                candidate.source_revision(),
                candidate.snapshot_digest(),
                candidate.dependencies().to_vec(),
            ) {
                Ok(provenance) => provenance,
                Err(error) => {
                    let internal = format!(
                        "Prepared dependent analysis '{analysis_name}' has invalid provenance: {error}"
                    );
                    log::error!("{internal}");
                    state.push_sim_message(ConsoleMessage::error(internal));
                    if let Some(run_id) = self.target_run_id(state)
                        && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
                    {
                        run.success = false;
                    }
                    self.pending_analyses.clear();
                    self.finish_simulation_batch(state);
                    return;
                }
            };
            let mut failed = AnalysisResult::failed(
                1,
                self.spec_to_analysis_type(candidate.spec()),
                analysis_name,
                message.clone(),
            )
            .with_provenance(provenance);
            materialize_saved_outputs(&mut failed, candidate.saved_output_contracts());
            if let Some(run_id) = self.target_run_id(state)
                && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
            {
                run.add_analysis(failed);
                run.success = false;
            }
            state.push_sim_message(ConsoleMessage::warning(message));
        };
        log::info!(
            "Starting queued analysis {} ({:?}, instance {}, source revision {}, {} dependencies, config {}, snapshot {})",
            next_analysis.label(),
            next_analysis.spec().run_type(),
            next_analysis.instance_id(),
            next_analysis.source_revision().get(),
            next_analysis.dependencies().len(),
            next_analysis.config_digest(),
            next_analysis.snapshot_digest(),
        );
        let spec = next_analysis.spec().clone();
        let config = next_analysis.config().cloned();
        let analysis_name = next_analysis.label().to_owned();
        self.touchstone_export_policy = next_analysis.touchstone_export_policy().clone();
        let provenance = match AnalysisResultProvenance::new_with_source_domain(
            self.current_source_domain,
            next_analysis.instance_id(),
            next_analysis.source_revision(),
            next_analysis.snapshot_digest(),
            next_analysis.dependencies().to_vec(),
        ) {
            Ok(provenance) => provenance,
            Err(error) => {
                let message = format!(
                    "Prepared analysis '{}' has invalid result provenance: {error}",
                    analysis_name
                );
                log::error!("{message}");
                state.push_sim_message(ConsoleMessage::error(message));
                if let Some(run_id) = self.target_run_id(state)
                    && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
                {
                    run.success = false;
                }
                self.pending_analyses.clear();
                self.finish_simulation_batch(state);
                return;
            }
        };

        self.current_analysis_idx += 1;
        self.current_config = config.clone();
        self.current_spec = Some(spec.clone());
        self.current_provenance = Some(provenance);
        self.current_saved_output_contracts = next_analysis.saved_output_contracts().to_vec();

        // Update status with multi-analysis progress
        let status_msg = if self.total_analyses > 1 {
            format!(
                "Analysis {}/{}: {}",
                self.current_analysis_idx, self.total_analyses, analysis_name
            )
        } else {
            analysis_name.clone()
        };
        state.simulation.status = status_msg.clone();

        // Log to console
        state.push_sim_message(ConsoleMessage::info(format!(
            "Starting {}...",
            if self.total_analyses > 1 {
                format!(
                    "{} ({}/{})",
                    analysis_name, self.current_analysis_idx, self.total_analyses
                )
            } else {
                analysis_name.clone()
            }
        )));

        // Use cached netlist. If this is unexpectedly missing, fail gracefully
        // instead of panicking so the UI can recover.
        if self.cached_netlist.is_none() {
            let message = format!(
                "Internal error: missing cached netlist while starting {}",
                analysis_name
            );
            log::error!("{}", message);
            state.push_sim_message(ConsoleMessage::error(message.clone()));
            let target_run_id = self.target_run_id(state);
            let failed_analysis = self.current_provenance.take().map(|provenance| {
                AnalysisResult::failed(1, self.spec_to_analysis_type(&spec), analysis_name, message)
                    .with_provenance(provenance)
            });
            let failed_analysis = failed_analysis.map(|mut analysis| {
                self.materialize_current_saved_outputs(&mut analysis);
                analysis
            });
            if let Some(run_id) = target_run_id
                && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
            {
                if let Some(failed_analysis) = failed_analysis {
                    run.add_analysis(failed_analysis);
                }
                run.success = false;
            }
            self.pending_analyses.clear();
            self.finish_simulation_batch(state);
            state.simulation.status = "Error".to_string();
            return;
        }

        // Start the simulation
        let start_result = self.runner.start_prepared(next_analysis);
        match start_result {
            Ok(()) => {
                if let Some(run_id) = self.target_run_id(state)
                    && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
                    && let Err(error) = run.mark_running()
                {
                    log::error!("Failed to advance simulation run lifecycle: {error}");
                    state.push_sim_message(ConsoleMessage::error(error));
                }
                log::info!(
                    "Analysis {}/{} started successfully",
                    self.current_analysis_idx,
                    self.total_analyses
                );
            }
            Err(e) => {
                log::error!("Failed to start simulation: {}", e);
                let error_message = format!("Failed to start simulation: {e}");
                state.push_sim_message(ConsoleMessage::error(format!(
                    "Failed to start simulation: {}",
                    e
                )));
                // Mark run as failed but continue with remaining analyses
                let target_run_id = self.target_run_id(state);
                let failed_analysis = self.current_provenance.take().map(|provenance| {
                    AnalysisResult::failed(
                        1,
                        self.spec_to_analysis_type(&spec),
                        analysis_name,
                        error_message,
                    )
                    .with_provenance(provenance)
                });
                let failed_analysis = failed_analysis.map(|mut analysis| {
                    self.materialize_current_saved_outputs(&mut analysis);
                    analysis
                });
                if let Some(run_id) = target_run_id
                    && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
                {
                    if let Some(failed_analysis) = failed_analysis {
                        run.add_analysis(failed_analysis);
                    }
                    run.success = false;
                }
                // Try to start next analysis if any remain
                if !self.pending_analyses.is_empty() {
                    self.start_next_analysis(state);
                } else {
                    self.finish_simulation_batch(state);
                }
            }
        }
    }

    fn target_run_id(&self, state: &AppState) -> Option<u64> {
        self.current_run_id
            .or_else(|| state.simulation.runs.first().map(|run| run.id))
    }

    fn materialize_current_saved_outputs(&mut self, analysis: &mut AnalysisResult) {
        let contracts = std::mem::take(&mut self.current_saved_output_contracts);
        materialize_saved_outputs(analysis, &contracts);
    }

    fn retain_completed_analysis(
        &mut self,
        state: &mut AppState,
        target_run_id: Option<u64>,
        analysis: AnalysisResult,
        provenance: AnalysisResultProvenance,
    ) -> Result<bool, String> {
        let run_id = target_run_id
            .ok_or_else(|| "completed analysis has no target simulation run".to_owned())?;
        let completed_instance = provenance.source_instance_id();
        let succeeded = analysis.success;
        let run = state
            .simulation
            .run_by_sequence_mut(run_id)
            .ok_or_else(|| format!("completed analysis target run {run_id} does not exist"))?;
        run.add_analysis(analysis.with_provenance(provenance));
        if succeeded {
            self.successful_analysis_instances
                .insert(completed_instance);
        }
        log::info!(
            "Added analysis to run {} (now has {} analyses)",
            run.id,
            run.analyses.len()
        );
        Ok(succeeded)
    }

    /// Finish the simulation batch and clean up state
    fn finish_simulation_batch(&mut self, state: &mut AppState) {
        let completed_analysis_count = self.total_analyses;
        let completed_run_id = self.target_run_id(state);
        let run_success = completed_run_id
            .and_then(|run_id| state.simulation.run_by_sequence(run_id))
            .map(|run| run.success)
            .or_else(|| state.simulation.active_run().map(|run| run.success))
            .unwrap_or(true);

        if let Some(run_id) = completed_run_id
            && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
        {
            let terminal = if run_success {
                SimulationRunLifecycle::Completed
            } else {
                SimulationRunLifecycle::Failed
            };
            if let Err(error) = run.finish_lifecycle(terminal) {
                log::error!("Failed to seal completed run lifecycle: {error}");
                state.push_sim_message(ConsoleMessage::error(error));
            }
        }

        // Complete the run (syncs waveforms and selects first analysis)
        if let Some(run_id) = completed_run_id {
            state.simulation.select_run_by_sequence(run_id);
            state.simulation.complete_run();
        } else {
            state.simulation.complete_run();
        }
        Self::promote_manual_deck_baseline(state, run_success, completed_run_id);

        // Clear cached netlist
        self.cached_netlist = None;
        self.successful_analysis_instances.clear();
        self.current_config = None;
        self.current_spec = None;
        self.current_provenance = None;
        self.current_saved_output_contracts.clear();
        self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
        self.current_run_id = None;
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        self.current_analysis_idx = 0;
        self.total_analyses = 0;
        state.simulation.active_execution = None;
        state.simulation.abort_request = None;

        state.simulation.status = if run_success {
            "Complete".to_string()
        } else {
            "Completed with errors".to_string()
        };
        let summary = if run_success {
            if completed_analysis_count > 1 {
                ConsoleMessage::info(format!(
                    "All {completed_analysis_count} analyses completed successfully"
                ))
            } else {
                ConsoleMessage::info("Simulation completed successfully".to_owned())
            }
        } else if completed_analysis_count > 1 {
            ConsoleMessage::error(format!(
                "Analysis batch completed with errors ({completed_analysis_count} planned)"
            ))
        } else {
            ConsoleMessage::error("Simulation completed with errors".to_owned())
        };
        state.push_sim_message(summary);

        log::info!("Simulation batch completed");
    }

    fn promote_manual_deck_baseline(
        state: &mut AppState,
        run_success: bool,
        completed_run_id: Option<u64>,
    ) {
        let pending_matches = state.ui.netlist.pending_manual_run_id.is_some()
            && state.ui.netlist.pending_manual_run_id == completed_run_id;
        if !pending_matches {
            return;
        }

        let pending_buffer = state.ui.netlist.pending_run_buffer.take();
        state.ui.netlist.pending_manual_run_id = None;

        if run_success && let Some(buffer) = pending_buffer {
            let current_buffer = state.simulation.netlist_content.clone();
            let param_values = Self::manual_deck_param_values(&buffer);
            state.ui.netlist.last_run_buffer = Some(buffer);
            state.ui.netlist.last_run_params = param_values;
            if let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() {
                state.ui.netlist.edited_lines =
                    Self::changed_lines_against_baseline(&current_buffer, baseline);
            }
        }
    }

    fn changed_lines_against_baseline(
        current: &str,
        baseline: &str,
    ) -> std::collections::HashSet<usize> {
        let current_lines: Vec<&str> = current.lines().collect();
        let baseline_lines: Vec<&str> = baseline.lines().collect();
        let max_len = current_lines.len().max(baseline_lines.len());
        let mut changed = std::collections::HashSet::new();

        for idx in 0..max_len {
            if current_lines.get(idx) != baseline_lines.get(idx) {
                changed.insert(idx);
            }
        }

        changed
    }

    fn manual_deck_param_values(buffer: &str) -> std::collections::HashMap<String, f64> {
        let mut out = std::collections::HashMap::new();
        for line in buffer.lines() {
            let trimmed = line.trim_start();
            let lower = trimmed.to_ascii_lowercase();
            if !(lower.starts_with(".param") || lower.starts_with(".parameter")) {
                continue;
            }
            let Some(after_cmd) = trimmed.find(char::is_whitespace) else {
                continue;
            };
            for (name, raw) in Self::manual_deck_param_assignments(&trimmed[after_cmd..]) {
                if raw.starts_with('{') {
                    continue;
                }
                if let Ok(value) = crate::properties::engineering::parse_engineering_value(raw) {
                    out.insert(name.to_ascii_lowercase(), value);
                }
            }
        }
        out
    }

    fn manual_deck_param_assignments(line: &str) -> Vec<(String, &str)> {
        let bytes = line.as_bytes();
        let mut assignments = Vec::new();
        let mut i = 0usize;

        while i < bytes.len() {
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] == b';' || bytes[i] == b'$' {
                break;
            }

            let name_start = i;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch.is_whitespace() || ch == '=' {
                    break;
                }
                i += 1;
            }
            if name_start == i {
                break;
            }
            let name = line[name_start..i].to_string();

            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] != b'=' {
                break;
            }
            i += 1;
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() {
                break;
            }

            let value_start = i;
            if bytes[i] == b'{' {
                let mut depth = 0i32;
                while i < bytes.len() {
                    match bytes[i] {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                i += 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            } else {
                while i < bytes.len() && !(bytes[i] as char).is_whitespace() {
                    i += 1;
                }
            }
            assignments.push((name, line[value_start..i].trim()));
        }

        assignments
    }

    fn analysis_name(&self, config: &AnalysisConfig) -> &'static str {
        match config {
            AnalysisConfig::DcOp => "DC Operating Point",
            AnalysisConfig::DcSweep(_) => "DC Sweep",
            AnalysisConfig::Transient(_) => "Transient",
            AnalysisConfig::Ac(_) => "AC",
            AnalysisConfig::Noise(_) => "Noise",
            AnalysisConfig::PoleZero(_) => "Pole-Zero",
            AnalysisConfig::Sensitivity(_) => "Sensitivity",
        }
    }

    fn analysis_name_for_spec(&self, spec: &AnalysisSpec) -> &'static str {
        spec.run_type().display_name()
    }

    /// Convert AnalysisConfig to corresponding AnalysisType enum
    ///
    /// Maps the engine's analysis configuration to the UI state's analysis type
    /// for proper categorization in the Results Browser.
    fn config_to_analysis_type(&self, config: &AnalysisConfig) -> AnalysisType {
        match config {
            AnalysisConfig::DcOp => AnalysisType::DcOp,
            AnalysisConfig::DcSweep(_) => AnalysisType::DcSweep,
            AnalysisConfig::Transient(_) => AnalysisType::Transient,
            AnalysisConfig::Ac(_) => AnalysisType::Ac,
            AnalysisConfig::Noise(_) => AnalysisType::Noise,
            AnalysisConfig::PoleZero(_) => AnalysisType::PoleZero,
            AnalysisConfig::Sensitivity(_) => AnalysisType::Sensitivity,
        }
    }

    fn spec_to_analysis_type(&self, spec: &AnalysisSpec) -> AnalysisType {
        match spec.run_type() {
            AnalysisRunType::DcOp => AnalysisType::DcOp,
            AnalysisRunType::DcSweep => AnalysisType::DcSweep,
            AnalysisRunType::Ac => AnalysisType::Ac,
            AnalysisRunType::Disto => AnalysisType::Disto,
            AnalysisRunType::Transient => AnalysisType::Transient,
            AnalysisRunType::Noise => AnalysisType::Noise,
            AnalysisRunType::Tf => AnalysisType::Tf,
            AnalysisRunType::Sensitivity => AnalysisType::Sensitivity,
            AnalysisRunType::PoleZero => AnalysisType::PoleZero,
            AnalysisRunType::HarmonicBalance => AnalysisType::HarmonicBalance,
            AnalysisRunType::Pss => AnalysisType::Pss,
            AnalysisRunType::Pac => AnalysisType::Pac,
            AnalysisRunType::Pnoise => AnalysisType::Pnoise,
            AnalysisRunType::Pxf => AnalysisType::Pxf,
            AnalysisRunType::Pstb => AnalysisType::Pstb,
            AnalysisRunType::Stb => AnalysisType::Stb,
            AnalysisRunType::MonteCarlo => AnalysisType::MonteCarlo,
            AnalysisRunType::Parametric => AnalysisType::Parametric,
            AnalysisRunType::Corner => AnalysisType::Corner,
            AnalysisRunType::Reliability => AnalysisType::Reliability,
            AnalysisRunType::Optimization => AnalysisType::Optimization,
            AnalysisRunType::Soa => AnalysisType::Soa,
            AnalysisRunType::SParameter => AnalysisType::SParameter,
            AnalysisRunType::Envelope => AnalysisType::Envelope,
            AnalysisRunType::Fourier => AnalysisType::Fourier,
            AnalysisRunType::Qpss => AnalysisType::Qpss,
            AnalysisRunType::Hbsp => AnalysisType::Hbsp,
            AnalysisRunType::Hbnoise => AnalysisType::Hbnoise,
            AnalysisRunType::Psp => AnalysisType::Psp,
            AnalysisRunType::Qpac => AnalysisType::Qpac,
            AnalysisRunType::Qpnoise => AnalysisType::Qpnoise,
            AnalysisRunType::Qpxf => AnalysisType::Qpxf,
            AnalysisRunType::TransientNoise => AnalysisType::TransientNoise,
            AnalysisRunType::DcMismatch => AnalysisType::DcMismatch,
        }
    }

    /// Convert SimulationResult to AnalysisResult for storage in Results Browser
    ///
    /// Extracts data from the engine's SimulationResult and creates an AnalysisResult
    /// with the appropriate type and data for display.
    /// done, finalizes the simulation batch.
    fn poll_completion(
        &mut self,
        state: &mut AppState,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) {
        // Update status display with multi-analysis progress
        let status = self.runner.status();
        let cancellation_pending = state
            .simulation
            .active_execution
            .and_then(|identity| state.simulation.run_by_stable_id(identity.run_id))
            .is_some_and(|run| run.lifecycle == SimulationRunLifecycle::Cancelling);
        if cancellation_pending {
            state.simulation.status = "Cancelling".to_owned();
        } else if !matches!(status, SimulationStatus::Idle)
            && !matches!(status, SimulationStatus::Completed { .. })
        {
            // Show progress-aware status
            if self.total_analyses > 1 {
                state.simulation.status = format!(
                    "Analysis {}/{}: {}",
                    self.current_analysis_idx,
                    self.total_analyses,
                    status.display_name()
                );
            } else {
                state.simulation.status = status.display_name().to_string();
            }
        }

        // Check for completion
        if let Some(result) = self.runner.poll_result() {
            match result {
                Ok(sim_result) => {
                    log::info!(
                        "Analysis {}/{} completed! Result type: {:?}",
                        self.current_analysis_idx,
                        self.total_analyses,
                        std::mem::discriminant(&sim_result)
                    );

                    // Log completion to console
                    let current_label = self
                        .current_spec
                        .as_ref()
                        .map(|spec| self.analysis_name_for_spec(spec))
                        .or_else(|| self.current_config.as_ref().map(|c| self.analysis_name(c)))
                        .unwrap_or("Analysis")
                        .to_owned();

                    // Convert SimulationResult to AnalysisResult and add to run
                    let analysis_type = self
                        .current_spec
                        .as_ref()
                        .map(|spec| self.spec_to_analysis_type(spec))
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|cfg| self.config_to_analysis_type(cfg))
                        })
                        .unwrap_or(AnalysisType::DcOp);
                    let target_run_id = self.target_run_id(state);

                    self.apply_result_side_effects(state, &sim_result);
                    if let crate::simulation::SimulationResult::Transient {
                        time, waveforms, ..
                    } = &sim_result
                    {
                        self.populate_transient_post_views(state, time, waveforms);
                    }

                    // Optional Touchstone export for S-parameter analyses.
                    let export_run_id = target_run_id
                        .or_else(|| state.simulation.active_run().map(|run| run.id))
                        .unwrap_or(0);
                    self.maybe_export_touchstone_for_run(
                        state,
                        &sim_result,
                        export_io,
                        export_run_id,
                    );

                    // --- Phase 10-11-12 Integration Glue (run once per analysis) ---

                    // Run Yield Analysis (if MC results are present)
                    if let Some(yield_results) = self.yield_manager.analyze_monte_carlo(&sim_result)
                    {
                        let yield_provenance = target_run_id
                            .and_then(|run_sequence| state.simulation.run_by_sequence(run_sequence))
                            .and_then(|run| {
                                crate::services::YieldAnalysisProvenance::from_monte_carlo_result(
                                    run.run_id,
                                    run.dataset_id,
                                    &sim_result,
                                )
                            });
                        state.simulation.replace_yield_evidence(
                            yield_results.values().cloned().collect(),
                            yield_provenance,
                        );
                    }

                    let mut analysis_result = if let Some(config) = &self.current_config {
                        self.convert_to_analysis_result_owned(sim_result, config)
                    } else {
                        self.convert_to_analysis_result_with_metadata_owned(
                            sim_result,
                            analysis_type,
                            &current_label,
                        )
                    };
                    self.materialize_current_saved_outputs(&mut analysis_result);
                    let retention_error = analysis_result.error_message.clone();
                    if let Some(provenance) = self.current_provenance.take() {
                        match self.retain_completed_analysis(
                            state,
                            target_run_id,
                            analysis_result,
                            provenance,
                        ) {
                            Ok(true) => {
                                if self.total_analyses > 1 {
                                    state.push_sim_message(ConsoleMessage::info(format!(
                                        "{} completed ({}/{})",
                                        current_label,
                                        self.current_analysis_idx,
                                        self.total_analyses
                                    )));
                                }
                            }
                            Ok(false) => {
                                state.push_sim_message(ConsoleMessage::error(format!(
                                    "{} result retention failed: {}",
                                    current_label,
                                    retention_error
                                        .as_deref()
                                        .unwrap_or("unknown retention error")
                                )));
                            }
                            Err(error) => {
                                log::error!("{error}");
                                state.push_sim_message(ConsoleMessage::error(error));
                                if let Some(run) = state.simulation.active_run_mut() {
                                    run.success = false;
                                }
                                self.pending_analyses.clear();
                            }
                        }
                    } else {
                        let message = format!(
                            "Internal error: completed {} has no prepared-task provenance",
                            current_label
                        );
                        log::error!("{message}");
                        state.push_sim_message(ConsoleMessage::error(message));
                        if let Some(run_id) = target_run_id
                            && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
                        {
                            run.success = false;
                        }
                    }

                    // Display the just-completed analysis without rebuilding waveform buffers.
                    if let Some(run_id) = target_run_id {
                        state
                            .simulation
                            .select_latest_analysis_in_run_sequence(run_id);
                    } else {
                        state.simulation.select_latest_analysis();
                    }

                    // =========================================================
                    // Multi-analysis chaining: start next or finish batch
                    // =========================================================
                    if !self.pending_analyses.is_empty() {
                        log::info!(
                            "Starting next analysis ({} remaining)",
                            self.pending_analyses.len()
                        );
                        self.start_next_analysis(state);
                    } else {
                        // All analyses complete - finalize the batch.
                        self.finish_simulation_batch(state);
                    }
                }
                Err(SimulationError::Aborted) => {
                    log::info!("Analysis aborted; discarding cancellation completion result");
                    if let Some(run_sequence) = self.current_run_id
                        && let Some(run) = state.simulation.run_by_sequence_mut(run_sequence)
                    {
                        run.success = false;
                        if let Err(error) = run.finish_lifecycle(SimulationRunLifecycle::Aborted) {
                            log::error!("Failed to seal aborted run lifecycle: {error}");
                            state.push_sim_message(ConsoleMessage::error(error));
                        }
                    }
                    self.pending_analyses.clear();
                    self.successful_analysis_instances.clear();
                    self.cached_netlist = None;
                    self.current_config = None;
                    self.current_spec = None;
                    self.current_provenance = None;
                    self.current_saved_output_contracts.clear();
                    self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
                    self.current_run_id = None;
                    self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
                    state.simulation.active_execution = None;
                    state.simulation.abort_request = None;
                    state.ui.netlist.pending_manual_run_id = None;
                    state.ui.netlist.pending_run_buffer = None;
                    state.simulation.status = "Aborted".to_string();
                    state.push_sim_message(ConsoleMessage::warning(
                        "Simulation aborted by user".to_owned(),
                    ));
                }
                Err(e) => {
                    state
                        .push_sim_message(ConsoleMessage::error(format!("Analysis failed: {}", e)));

                    // Mark run as partially failed and add failed analysis entry
                    let failed_label = self
                        .current_spec
                        .as_ref()
                        .map(|spec| self.analysis_name_for_spec(spec))
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|cfg| self.analysis_name(cfg))
                        })
                        .unwrap_or("Analysis")
                        .to_string();
                    let failed_type = self
                        .current_spec
                        .as_ref()
                        .map(|spec| self.spec_to_analysis_type(spec))
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|cfg| self.config_to_analysis_type(cfg))
                        })
                        .unwrap_or(AnalysisType::DcOp);
                    let target_run_id = self.target_run_id(state);
                    let failed_analysis = if let Some(provenance) = self.current_provenance.take() {
                        let mut analysis =
                            AnalysisResult::failed(1, failed_type, failed_label, e.to_string())
                                .with_provenance(provenance);
                        self.materialize_current_saved_outputs(&mut analysis);
                        Some(analysis)
                    } else {
                        let message =
                            "Internal error: failed analysis has no prepared-task provenance";
                        log::error!("{message}");
                        state.push_sim_message(ConsoleMessage::error(message.to_owned()));
                        None
                    };
                    if let Some(run_id) = target_run_id
                        && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
                    {
                        if let Some(failed_analysis) = failed_analysis {
                            run.add_analysis(failed_analysis);
                        }
                        run.success = false;
                    }

                    // Continue with remaining analyses (commercial behavior: don't abort batch)
                    if !self.pending_analyses.is_empty() {
                        log::info!(
                            "Analysis failed, continuing with {} remaining",
                            self.pending_analyses.len()
                        );
                        self.start_next_analysis(state);
                    } else {
                        state.simulation.status = "Completed with errors".to_string();
                        self.finish_simulation_batch(state);
                    }
                }
            }
        }
    }

    /// Check if a simulation is currently running
    pub fn is_running(&self) -> bool {
        self.runner.is_running()
    }

    fn ui_progress_fraction(progress: Option<f32>, is_running: bool) -> f64 {
        progress
            .map(|value| f64::from(value).clamp(0.0, 1.0))
            .unwrap_or_else(|| if is_running { 0.08 } else { 0.0 })
    }

    /// Abort current simulation
    pub fn abort(&self) {
        self.runner.abort();
    }

    /// Get current status
    pub fn status(&self) -> SimulationStatus {
        self.runner.status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
    use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
    use crate::simulation::plan::AnalysisKind;
    use crate::state::{ComponentType, Point, PreparedSourceCheckReceipt, SimulationRunProvenance};
    use std::cell::RefCell;
    use std::path::Path;

    #[derive(Debug, Default)]
    struct MockExportWorkflowIo {
        writes: RefCell<Vec<(PathBuf, String)>>,
        create_only_writes: RefCell<Vec<(PathBuf, String)>>,
    }

    impl ExportWorkflowIo for MockExportWorkflowIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(None)
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.writes
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_string()));
            Ok(())
        }

        fn write_new_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.create_only_writes
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_string()));
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    fn state_with_current_drc_error() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));

        let mut result = DrcResult::new();
        result.add_violation(DrcViolation::new(
            1,
            DrcViolationType::MissingGround,
            "missing ground",
            DrcLocation::Global,
        ));
        result.completed = true;
        state.dialogs.drc_results = Some(result);
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        state
    }

    fn state_with_current_clean_drc() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        let mut result = DrcResult::new();
        result.completed = true;
        state.dialogs.drc_results = Some(result);
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        state
    }

    fn bind_test_run_running(
        state: &mut AppState,
        controller: &mut SimulationController,
        run_sequence: u64,
    ) {
        let run = state
            .simulation
            .run_by_sequence_mut(run_sequence)
            .expect("test execution has a retained run");
        run.mark_running().expect("test run enters running state");
        let identity = run.execution_identity();
        controller.current_run_id = Some(run_sequence);
        state.simulation.active_execution = identity;
    }

    fn bind_and_request_test_abort(state: &mut AppState, controller: &mut SimulationController) {
        let run_sequence = state
            .simulation
            .active_run()
            .expect("test abort has an active run")
            .id;
        bind_test_run_running(state, controller, run_sequence);
        state
            .simulation
            .request_abort_active_run()
            .expect("test abort binds to the active execution");
    }

    fn synthetic_sparameter_result() -> crate::simulation::SimulationResult {
        let frequencies = vec![1.0e6, 2.0e6];
        let mut waveforms = std::collections::HashMap::new();
        for (name, real, imag) in [
            ("S11", vec![0.1, 0.2], vec![0.0, 0.0]),
            ("S21", vec![0.3, 0.4], vec![0.01, 0.02]),
            ("S12", vec![0.5, 0.6], vec![0.03, 0.04]),
            ("S22", vec![0.7, 0.8], vec![0.05, 0.06]),
        ] {
            waveforms.insert(
                name.to_string(),
                crate::simulation::results::WaveformData::new_complex(
                    name,
                    frequencies.clone(),
                    real,
                    imag,
                ),
            );
        }

        crate::simulation::SimulationResult::Ac {
            frequencies,
            waveforms,
            measurements: Vec::new(),
        }
    }

    fn synthetic_dc_op_result() -> crate::simulation::SimulationResult {
        let mut result = crate::simulation::results::DcOpResult::default();
        result.node_voltages.insert("out".to_string(), 1.25);
        crate::simulation::SimulationResult::DcOp(result)
    }

    fn synthetic_result_provenance() -> AnalysisResultProvenance {
        AnalysisResultProvenance::new(
            crate::product::AnalysisInstanceId::new(),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([0x39; 32]),
            Vec::new(),
        )
        .expect("synthetic prepared-task provenance is valid")
    }

    #[test]
    fn failed_result_retention_never_satisfies_prepared_dependencies() {
        let mut state = AppState::default();
        let run_sequence = state.simulation.start_run().id;
        let provenance = synthetic_result_provenance();
        let instance = provenance.source_instance_id();
        let failed = AnalysisResult::failed(
            1,
            AnalysisType::Reliability,
            "Reliability",
            "invalid retained evidence",
        );
        let mut controller = SimulationController::new();

        assert!(
            !controller
                .retain_completed_analysis(&mut state, Some(run_sequence), failed, provenance)
                .expect("failed result is retained as failure evidence")
        );
        assert!(!controller.successful_analysis_instances.contains(&instance));
        let run = state
            .simulation
            .run_by_sequence(run_sequence)
            .expect("target run remains");
        assert!(!run.success);
        assert_eq!(run.analyses.len(), 1);
        assert!(!run.analyses[0].success);
    }

    fn exact_vec(values: &[f64]) -> Vec<f64> {
        let mut result = Vec::with_capacity(values.len());
        result.extend_from_slice(values);
        assert_eq!(
            result.capacity(),
            result.len(),
            "test vectors must be tightly allocated so pointer reuse is meaningful"
        );
        result
    }

    #[test]
    fn direct_trigger_is_blocked_by_current_drc_errors() {
        let mut state = state_with_current_drc_error();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();

        state.simulation.request_simulate_run_set();
        controller.update(&mut state, &export_io);

        assert!(!state.simulation.trigger_simulation);
        assert!(!state.simulation.is_running);
        assert_eq!(state.simulation.status, "Run blocked");
    }

    #[test]
    fn controller_plan_run_is_sealed_with_exact_prepared_receipt_before_results() {
        let mut state = state_with_current_clean_drc();
        let plan = state.sim_setup.analysis_plan.as_ref().expect("stable plan");
        let plan_id = plan.id();
        let plan_revision = plan.revision();
        let task_id = plan
            .instances()
            .iter()
            .find(|instance| instance.enabled())
            .expect("enabled plan task")
            .id();
        let project_revision = state.workspace.project.revision();
        let mut controller = SimulationController::new();
        let metadata = controller
            .prepare_run_set_for_preflight(&state)
            .expect("clean plan preflight");

        state.simulation.request_simulate_run_set();
        controller.start_simulation(&mut state);

        let run = state.simulation.active_run().expect("prepared run starts");
        let receipt = run
            .prepared_receipt()
            .expect("run is sealed before results");
        assert_eq!(
            receipt.source_domain(),
            AnalysisResultSourceDomain::SimulationPlan
        );
        assert_eq!(receipt.simulation_plan_id(), Some(plan_id));
        assert_eq!(receipt.project_revision(), project_revision);
        assert_eq!(receipt.prepared_snapshot_digest(), metadata.snapshot_digest);
        assert_eq!(receipt.source_content_digest(), metadata.source_digest);
        assert_eq!(
            receipt.source_check_receipt(),
            PreparedSourceCheckReceipt::SchematicDrc(metadata.receipt_digest)
        );
        assert_eq!(receipt.tasks().len(), 1);
        let task = &receipt.tasks()[0];
        assert_eq!(task.instance_id(), task_id);
        assert_eq!(task.source_revision(), plan_revision);
        assert_eq!(task.analysis_kind_tag(), 5);
        assert!(task.dependencies().is_empty());
        assert_ne!(
            task.config_digest(),
            crate::product::ContentDigest::from_bytes([0; 32])
        );
        assert!(run.analyses.is_empty());
        assert!(run.validate_provenance().is_ok());

        controller.abort();
    }

    #[test]
    fn controller_plan_run_receipt_survives_production_project_round_trip() {
        let mut state = state_with_current_clean_drc();
        let mut controller = SimulationController::new();
        controller
            .prepare_run_set_for_preflight(&state)
            .expect("clean plan preflight");
        state.simulation.request_simulate_run_set();
        controller.start_simulation(&mut state);
        let task_provenance = controller
            .current_provenance
            .clone()
            .expect("controller owns the first prepared plan task");
        let expected_source_id = task_provenance.source_instance_id();
        state
            .simulation
            .active_run_mut()
            .expect("prepared plan run")
            .add_analysis(
                AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                    .with_provenance(task_provenance),
            );
        controller.abort();

        let project = crate::common::project_lifecycle::snapshot(&state)
            .expect("production snapshot accepts controller run");
        let json = crate::io::project_io::serialize_project_file(&project)
            .expect("controller plan run serializes");
        let loaded = crate::io::project_io::load_project_text(&json, None)
            .expect("controller plan run reloads");
        let loaded_plan_id = loaded
            .execution_context
            .as_ref()
            .expect("execution context retained")
            .simulation_plan
            .stable_analysis_plan()
            .expect("stable plan retained")
            .id();
        let restored = loaded
            .simulation_results
            .into_simulation_state()
            .expect("controller plan history restores");
        let run = &restored.runs[0];
        let receipt = run.prepared_receipt().expect("prepared receipt retained");
        let result_provenance = run.analyses[0]
            .provenance
            .as_ref()
            .expect("result provenance retained");

        assert_eq!(
            receipt.source_domain(),
            AnalysisResultSourceDomain::SimulationPlan
        );
        assert_eq!(receipt.simulation_plan_id(), Some(loaded_plan_id));
        assert_eq!(receipt.tasks()[0].instance_id(), expected_source_id);
        assert_eq!(result_provenance.source_instance_id(), expected_source_id);
        assert_eq!(
            result_provenance.prepared_snapshot_digest(),
            receipt.prepared_snapshot_digest()
        );
    }

    #[test]
    fn design_context_reset_discards_pending_controller_result() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Ok(synthetic_dc_op_result()))
            .expect("seed old pending result");

        state.clear_design_execution_context();
        controller.update(&mut state, &export_io);

        assert!(
            !state.simulation.has_results(),
            "stale result from previous design should be discarded"
        );
        assert_eq!(state.log_buffer.len(), 0);
        assert_eq!(state.simulation.status, "");
        assert!(!controller.is_running());
    }

    #[test]
    fn design_epoch_reset_terminalizes_executor_owned_history() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_sequence);

        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        controller.update(&mut state, &export_io);

        let run = state
            .simulation
            .run_by_sequence(run_sequence)
            .expect("interrupted history remains retained");
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Interrupted);
        assert!(!run.success);
        assert!(state.simulation.active_execution.is_none());
        assert!(state.simulation.abort_request.is_none());
    }

    #[test]
    fn stale_cancellation_request_never_mutates_run_lifecycle() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_sequence);
        state
            .simulation
            .request_abort_active_run()
            .expect("request is initially bound");
        state.simulation.active_execution = None;

        controller.update(&mut state, &export_io);

        assert_eq!(
            state
                .simulation
                .run_by_sequence(run_sequence)
                .unwrap()
                .lifecycle,
            SimulationRunLifecycle::Running
        );
        assert!(state.simulation.abort_request.is_none());
        assert!(
            state
                .log_buffer
                .entries()
                .any(|entry| entry.message.contains("stale or unbound"))
        );
    }

    #[test]
    fn abort_trigger_discards_worker_aborted_result_without_failed_analysis() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        state.simulation.status = "Running".to_string();
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Err(crate::simulation::runner::SimulationError::Aborted))
            .expect("seed worker abort result");
        bind_and_request_test_abort(&mut state, &mut controller);

        controller.update(&mut state, &export_io);

        assert_eq!(state.simulation.status, "Aborted");
        let run = state.simulation.active_run().expect("active run remains");
        assert!(
            run.analyses.is_empty(),
            "aborted worker result must not be recorded as a failed analysis: {:?}",
            run.analyses
        );
        assert!(!run.success);
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Aborted);
    }

    #[test]
    fn abort_trigger_discards_unpolled_success_result() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        state.simulation.status = "Running".to_string();
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Ok(synthetic_dc_op_result()))
            .expect("seed unpolled success result");
        bind_and_request_test_abort(&mut state, &mut controller);

        controller.update(&mut state, &export_io);

        assert_eq!(state.simulation.status, "Aborted");
        let run = state.simulation.active_run().expect("active run remains");
        assert!(
            run.analyses.is_empty(),
            "success result that arrived before abort poll must not be recorded: {:?}",
            run.analyses
        );
        assert!(!run.success);
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Aborted);
    }

    #[test]
    fn completed_result_attaches_to_started_run_when_active_selection_changes() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let older_run_id = state.simulation.start_run().id;
        let started_run_id = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, started_run_id);
        assert!(
            state.simulation.select_run(1),
            "user can inspect an older run while a newer run is in flight"
        );
        controller.current_spec = Some(AnalysisSpec::DcOp);
        let provenance = synthetic_result_provenance();
        let expected_source_id = provenance.source_instance_id();
        controller.current_provenance = Some(provenance);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Ok(synthetic_dc_op_result()))
            .expect("seed completed run result");

        controller.update(&mut state, &export_io);

        let older_run = state
            .simulation
            .run_by_sequence(older_run_id)
            .expect("older run remains");
        let started_run = state
            .simulation
            .run_by_sequence(started_run_id)
            .expect("started run remains");
        assert!(
            older_run.analyses.is_empty(),
            "completed analysis must not contaminate the selected historical run"
        );
        assert_eq!(started_run.analyses.len(), 1);
        assert_eq!(
            started_run.analyses[0]
                .provenance
                .as_ref()
                .expect("completed result has prepared provenance")
                .source_instance_id(),
            expected_source_id
        );
        assert_eq!(
            state.simulation.active_run().map(|run| run.id),
            Some(started_run_id)
        );
        assert_eq!(started_run.lifecycle, SimulationRunLifecycle::Completed);
    }

    #[test]
    fn completed_transient_result_reuses_owned_waveform_buffers_in_run_history() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_sequence);
        controller.current_spec = Some(AnalysisSpec::Transient {
            stop_time: 2.0e-9,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        });
        controller.current_provenance = Some(synthetic_result_provenance());
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;

        let time = exact_vec(&[0.0, 1.0e-9, 2.0e-9]);
        let time_ptr = time.as_ptr();
        let values = exact_vec(&[0.0, 0.5, 1.0]);
        let values_ptr = values.as_ptr();
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            crate::simulation::results::WaveformData {
                name: "V(out)".to_string(),
                x_values: Vec::new(),
                y_values: values,
                y_unit: "V".to_string(),
                x_unit: "s".to_string(),
                is_complex: false,
                y_imag: None,
            },
        );

        controller
            .runner
            .store_pending_result(Ok(crate::simulation::SimulationResult::Transient {
                time,
                waveforms,
                measurements: Vec::new(),
            }))
            .expect("seed completed transient result");

        controller.update(&mut state, &export_io);

        let analysis = state
            .simulation
            .active_analysis()
            .expect("completed transient analysis is selected");
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "V(out)")
            .expect("transient waveform is stored in run history");

        assert_eq!(
            waveform.x.iter().as_slice().as_ptr(),
            time_ptr,
            "time vector should move into run history instead of being copied"
        );
        assert_eq!(
            waveform.y.iter().as_slice().as_ptr(),
            values_ptr,
            "sample vector should move into run history instead of being copied"
        );
    }

    #[test]
    fn completed_dc_sweep_result_reuses_owned_shared_axis_buffers_in_run_history() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_sequence);
        controller.current_spec = Some(AnalysisSpec::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 2.0,
            step: 1.0,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        });
        controller.current_provenance = Some(synthetic_result_provenance());
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;

        let sweep_values = exact_vec(&[0.0, 1.0, 2.0]);
        let sweep_ptr = sweep_values.as_ptr();
        let values = exact_vec(&[0.1, 0.2, 0.3]);
        let values_ptr = values.as_ptr();
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            crate::simulation::results::WaveformData {
                name: "V(out)".to_string(),
                x_values: Vec::new(),
                y_values: values,
                y_unit: "V".to_string(),
                x_unit: "V".to_string(),
                is_complex: false,
                y_imag: None,
            },
        );

        controller
            .runner
            .store_pending_result(Ok(crate::simulation::SimulationResult::DcSweep {
                sweep_var: "V1".to_string(),
                sweep_values,
                waveforms,
                measurements: Vec::new(),
            }))
            .expect("seed completed DC sweep result");

        controller.update(&mut state, &export_io);

        let analysis = state
            .simulation
            .active_analysis()
            .expect("completed DC sweep analysis is selected");
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "V(out)")
            .expect("DC sweep waveform is stored in run history");

        assert_eq!(
            waveform.x.iter().as_slice().as_ptr(),
            sweep_ptr,
            "shared sweep axis should move into run history instead of being copied"
        );
        assert_eq!(
            waveform.y.iter().as_slice().as_ptr(),
            values_ptr,
            "sweep samples should move into run history instead of being copied"
        );
    }

    #[test]
    fn failed_completion_retains_exact_prepared_task_provenance() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let run_sequence = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_sequence);
        controller.current_spec = Some(AnalysisSpec::DcOp);
        let provenance = synthetic_result_provenance();
        let expected_source_id = provenance.source_instance_id();
        let expected_snapshot = provenance.prepared_snapshot_digest();
        controller.current_provenance = Some(provenance);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Err(SimulationError::SolverError(
                "singular matrix".to_owned(),
            )))
            .expect("seed failed result");

        controller.update(&mut state, &export_io);

        let analysis = &state.simulation.active_run().expect("run remains").analyses[0];
        let restored = analysis
            .provenance
            .as_ref()
            .expect("failed result has prepared provenance");
        assert!(!analysis.success);
        assert_eq!(restored.source_instance_id(), expected_source_id);
        assert_eq!(restored.prepared_snapshot_digest(), expected_snapshot);
        assert_eq!(
            state.simulation.active_run().expect("failed run").lifecycle,
            SimulationRunLifecycle::Failed
        );
    }

    #[test]
    fn failed_prerequisite_skips_dependent_prepared_task_with_exact_provenance() {
        use crate::product::{ContentDigest, ObjectRevision};
        use crate::simulation::dialog::corner::ProcessCorner;
        use crate::simulation::execution::{
            ExecutionPermitIssuer, ExecutionTargetCapabilities, PreparedRunSnapshot, PreparedTask,
            RunSourceReceipt, SavePolicy, SnapshotParts,
        };

        let prerequisite_id = crate::product::AnalysisInstanceId::new();
        let dependent_id = crate::product::AnalysisInstanceId::new();
        let task = |spec, line: &str| QueuedAnalysis {
            spec,
            config: None,
            spec_options: SpecExecutionOptions::default(),
            analysis_line: line.to_owned(),
        };
        let snapshot = PreparedRunSnapshot::new(SnapshotParts {
            intent: SimulationRunIntent::SimulateRunSet,
            simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
            project_revision: 3,
            topology_revision: 4,
            source_digest: ContentDigest::from_bytes([0x71; 32]),
            reference_process: ProcessCorner::TT,
            reference_temperature_celsius: 27.0,
            tasks: vec![
                PreparedTask::new(
                    prerequisite_id,
                    ObjectRevision::INITIAL,
                    Vec::new(),
                    "Prerequisite",
                    task(AnalysisSpec::DcOp, ".op"),
                ),
                PreparedTask::new(
                    dependent_id,
                    ObjectRevision::INITIAL,
                    vec![prerequisite_id],
                    "Dependent",
                    task(AnalysisSpec::Pac, ".pac"),
                ),
            ],
            executable_netlist: "deck\n.op\n.end\n".to_owned(),
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities: Vec::new(),
            project_veriloga_runtime: None,
            target: ExecutionTargetCapabilities::current(),
            receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([0x72; 32])),
            advisories: Vec::new(),
            manual_source: None,
            cross_probe: None,
            touchstone_export: TouchstoneExportPolicy::disabled(),
            sealed_source_dependencies: Vec::new(),
        })
        .expect("dependency-ordered snapshot validates");
        let digest = snapshot.digest();
        let issuer = ExecutionPermitIssuer::default();
        let proof = issuer
            .issue(digest)
            .expect("permit issues")
            .consume(digest, digest)
            .expect("permit consumes");
        let mut tasks = snapshot
            .authorize_dispatch(proof)
            .expect("snapshot authorizes")
            .into_tasks();
        let failed_task = tasks.pop_front().expect("prerequisite task");

        let mut state = AppState::default();
        let run_sequence = state.simulation.start_run().id;
        let failed_provenance = AnalysisResultProvenance::new(
            failed_task.instance_id(),
            failed_task.source_revision(),
            failed_task.snapshot_digest(),
            failed_task.dependencies().to_vec(),
        )
        .expect("failed prerequisite provenance");
        state
            .simulation
            .run_by_sequence_mut(run_sequence)
            .expect("active run")
            .add_analysis(
                AnalysisResult::failed(1, AnalysisType::DcOp, "Prerequisite", "solver failed")
                    .with_provenance(failed_provenance),
            );

        let mut controller = SimulationController::new();
        controller.current_run_id = Some(run_sequence);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 2;
        controller.pending_analyses = tasks;
        controller.start_next_analysis(&mut state);

        let run = state
            .simulation
            .run_by_sequence(run_sequence)
            .expect("completed run remains");
        assert_eq!(run.analyses.len(), 2);
        let skipped = run
            .find_analysis_by_source_instance(dependent_id)
            .expect("dependent receives a retained skipped result");
        assert!(!skipped.success);
        assert!(
            skipped
                .error_message
                .as_deref()
                .is_some_and(|message| message.contains("prerequisite analysis result"))
        );
        assert_eq!(
            skipped
                .provenance
                .as_ref()
                .expect("skipped provenance")
                .dependency_ids(),
            &[prerequisite_id]
        );
        assert_eq!(state.simulation.status, "Completed with errors");
        assert!(!controller.is_running());
    }

    #[test]
    fn touchstone_auto_export_uses_export_workflow_io() {
        let mut state = AppState::default();
        state.simulation.start_run();

        let mut controller = SimulationController::new();
        controller.touchstone_export_policy = TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("designs"),
            std::ffi::OsString::from("amp"),
        )
        .expect("valid prepared Touchstone policy");
        controller.current_spec = Some(AnalysisSpec::SParameter {
            start_freq: 1.0e6,
            stop_freq: 2.0e6,
            points_per_unit: 2,
            sweep: FrequencySweep::Linear,
            z0: 50.0,
            ports: vec![
                SpPort {
                    node_pos: "IN".to_string(),
                    node_neg: "0".to_string(),
                    z0: None,
                },
                SpPort {
                    node_pos: "OUT".to_string(),
                    node_neg: "0".to_string(),
                    z0: Some(75.0),
                },
            ],
        });
        controller.current_analysis_idx = 1;
        // Live editor mutations after dispatch must not redirect or reformat
        // the prepared automatic export.
        state.schematic.current_file = Some(PathBuf::from("changed").join("redirect.sch"));
        let mut changed = crate::simulation::dialog::SpConfig::default();
        changed.touchstone_export = false;
        changed.touchstone_version = 1;
        state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(&changed);

        let export_io = MockExportWorkflowIo::default();
        controller.maybe_export_touchstone_for_run(
            &mut state,
            &synthetic_sparameter_result(),
            &export_io,
            1,
        );

        assert!(export_io.writes.borrow().is_empty());
        let writes = export_io.create_only_writes.borrow();
        assert_eq!(writes.len(), 1);
        assert_eq!(
            writes[0].0,
            PathBuf::from("designs").join("amp_run0001_sp01.s2p")
        );
        assert!(writes[0].1.contains("[Version] 2.0"));
        assert!(
            writes[0]
                .1
                .contains("[Reference] 5.000000000000e1 7.500000000000e1")
        );
    }

    #[test]
    fn touchstone_native_completion_message_confirms_file_export() {
        let message =
            SimulationController::touchstone_export_completed_message(Path::new("amp.s2p"));
        assert!(message.contains("Exported Touchstone"));
        assert!(message.contains("amp.s2p"));
    }

    #[test]
    fn ac_result_conversion_retains_complex_components_for_export() {
        let controller = SimulationController::new();
        let result = synthetic_sparameter_result();

        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            result,
            AnalysisType::SParameter,
            "SP",
        );
        let magnitude = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "|S11|")
            .expect("magnitude trace exists");
        let complex = magnitude
            .complex
            .as_ref()
            .expect("magnitude trace retains complex source data");

        assert_eq!(complex.source_name, "S11");
        assert_eq!(&*complex.real, &[0.1, 0.2]);
        assert_eq!(&*complex.imag, &[0.0, 0.0]);
    }

    #[test]
    fn ac_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
        let controller = SimulationController::new();
        let frequencies = vec![1.0, 10.0, 100.0];
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(bad_real)".to_string(),
            crate::simulation::results::WaveformData::new_complex(
                "V(bad_real)",
                frequencies.clone(),
                vec![1.0, 2.0],
                vec![0.0, 0.0, 0.0],
            ),
        );
        waveforms.insert(
            "V(bad_imag)".to_string(),
            crate::simulation::results::WaveformData::new_complex(
                "V(bad_imag)",
                frequencies.clone(),
                vec![1.0, 2.0, 3.0],
                vec![0.0, 0.0],
            ),
        );
        waveforms.insert(
            "V(good)".to_string(),
            crate::simulation::results::WaveformData::new_complex(
                "V(good)",
                frequencies.clone(),
                vec![3.0, 4.0, 5.0],
                vec![4.0, 3.0, 0.0],
            ),
        );

        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements: Vec::new(),
            },
            AnalysisType::Ac,
            "AC",
        );

        let names: Vec<_> = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect();
        assert_eq!(names, vec!["|V(good)|", "phase(V(good))"]);
        assert!(
            analysis
                .waveforms
                .iter()
                .all(|waveform| waveform.x.len() == waveform.y.len()),
            "converted AC traces must never pair mismatched x/y arrays"
        );
    }

    #[test]
    fn noise_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
        let controller = SimulationController::new();
        let mut contributors = std::collections::HashMap::new();
        contributors.insert("good".to_string(), vec![1.0e-18, 2.0e-18, 3.0e-18]);
        contributors.insert("bad".to_string(), vec![1.0e-18, 2.0e-18]);

        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Noise {
                frequencies: vec![1.0, 10.0, 100.0],
                output_noise: vec![2.0e-18, 3.0e-18],
                input_noise: Some(vec![1.0e-18, 1.5e-18, 2.0e-18]),
                contributors,
                summary: None,
            },
            AnalysisType::Noise,
            "Noise",
        );

        let names: Vec<_> = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect();
        assert_eq!(names, vec!["inoise", "noise(good)"]);
        assert!(
            analysis
                .waveforms
                .iter()
                .all(|waveform| waveform.x.len() == waveform.y.len()),
            "converted noise traces must never pair mismatched x/y arrays"
        );
    }

    #[test]
    fn advanced_result_conversion_retains_exact_family_metadata() {
        use crate::state::{AnalysisResultFamilyMetadata, MonteCarloVariableMetadata};

        let controller = SimulationController::new();
        let empty_waveforms = || std::collections::HashMap::new();

        let monte_carlo = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::MonteCarlo {
                seed: 0x5eed,
                runs_requested: 4,
                runs_completed: 3,
                num_failures: 1,
                all_converged: false,
                variables: vec![crate::simulation::results::MonteCarloVariableResult {
                    name: "V(out)".to_owned(),
                    samples: vec![0.9, 1.0, 1.1],
                    mean: 1.0,
                    std_dev: 0.1,
                    min: 0.9,
                    max: 1.1,
                    histogram: vec![1, 2],
                    bin_edges: vec![0.85, 1.0, 1.15],
                }],
            },
            AnalysisType::MonteCarlo,
            "MC",
        );
        assert_eq!(
            monte_carlo.family_metadata,
            Some(AnalysisResultFamilyMetadata::MonteCarlo {
                seed: 0x5eed,
                runs_requested: 4,
                runs_completed: 3,
                failures: 1,
                all_converged: false,
                variables: vec![MonteCarloVariableMetadata {
                    name: "V(out)".to_owned(),
                    samples: vec![0.9, 1.0, 1.1],
                    mean: 1.0,
                    std_dev: 0.1,
                    min: 0.9,
                    max: 1.1,
                }],
            })
        );
        assert_eq!(monte_carlo.waveforms[0].name, "hist(V(out))");

        let parametric = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Parametric {
                target: "PARAM rload".to_owned(),
                sweep_values: vec![1_000.0, 2_000.0],
                waveforms: empty_waveforms(),
                num_failures: 1,
            },
            AnalysisType::Parametric,
            "STEP",
        );
        assert_eq!(
            parametric.family_metadata,
            Some(AnalysisResultFamilyMetadata::Parametric {
                target: "PARAM rload".to_owned(),
                sweep_values: vec![1_000.0, 2_000.0],
                failed_points: 1,
            })
        );

        let corner = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Corner {
                x_values: vec![0.0, 1.0],
                x_label: "Corner Index".to_owned(),
                x_unit: String::new(),
                temperatures_c: vec![-40.0, 125.0],
                corner_labels: vec!["SS_0.9V_-40C".to_owned(), "FF_1.1V_125C".to_owned()],
                waveforms: empty_waveforms(),
                num_failures: 0,
            },
            AnalysisType::Corner,
            "Corner",
        );
        assert_eq!(
            corner.family_metadata,
            Some(AnalysisResultFamilyMetadata::Corner {
                x_values: vec![0.0, 1.0],
                x_label: "Corner Index".to_owned(),
                x_unit: String::new(),
                temperatures_c: vec![-40.0, 125.0],
                corner_labels: vec!["SS_0.9V_-40C".to_owned(), "FF_1.1V_125C".to_owned()],
                failed_corners: 0,
            })
        );

        let reliability = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Reliability {
                years: vec![1.0, 5.0, 10.0],
                waveforms: empty_waveforms(),
                device_results: vec![crate::simulation::ReliabilityResult {
                    device_id: "M1".to_owned(),
                    stress: crate::simulation::StressMetrics {
                        avg_vgs_stress: 1.2,
                        avg_vds_stress: 1.8,
                        avg_temp: 358.15,
                        duration: 3_600.0,
                    },
                    shifts: std::collections::HashMap::from([
                        ("1y".to_owned(), crate::simulation::ParamShift::default()),
                        ("5y".to_owned(), crate::simulation::ParamShift::default()),
                        ("10y".to_owned(), crate::simulation::ParamShift::default()),
                    ]),
                }],
            },
            AnalysisType::Reliability,
            "Reliability",
        );
        assert_eq!(
            reliability.family_metadata,
            Some(AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0, 10.0],
            })
        );
        assert!(matches!(
            reliability.result_payload,
            Some(AnalysisResultPayload::Reliability { ref devices }) if devices.len() == 1
        ));

        let optimization = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Optimization {
                iterations: vec![0.0, 1.0, 2.0],
                waveforms: empty_waveforms(),
                best_cost: 0.125,
                best_variables: std::collections::HashMap::from([
                    ("w".to_owned(), 2.0e-6),
                    ("l".to_owned(), 180.0e-9),
                ]),
                converged: true,
            },
            AnalysisType::Optimization,
            "Optimization",
        );
        assert_eq!(
            optimization.family_metadata,
            Some(AnalysisResultFamilyMetadata::Optimization {
                iterations: vec![0.0, 1.0, 2.0],
                best_cost: 0.125,
                best_variables: std::collections::BTreeMap::from([
                    ("l".to_owned(), 180.0e-9),
                    ("w".to_owned(), 2.0e-6),
                ]),
                converged: true,
            })
        );

        let soa = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Soa {
                time: vec![0.0, 1.0e-9],
                waveforms: empty_waveforms(),
                violations: Vec::new(),
                evaluations: vec![crate::services::safety::SoAEvaluation {
                    device_id: "M1".to_owned(),
                    parameter: crate::services::safety::SoAParameter::Vgs,
                    limit_value: 1.8,
                    worst_actual_value: 1.0,
                    worst_time: 1.0e-9,
                    sample_count: 2,
                    unit: "V".to_owned(),
                    description: "Maximum gate-source voltage".to_owned(),
                    verdict: crate::services::safety::SoARuleVerdict::Pass,
                }],
            },
            AnalysisType::Soa,
            "SOA",
        );
        assert_eq!(
            soa.family_metadata,
            Some(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0e-9],
            })
        );
        assert!(matches!(
            soa.result_payload,
            Some(AnalysisResultPayload::Soa {
                ref evaluations,
                ref violations,
            }) if evaluations.len() == 1 && violations.is_empty()
        ));
    }

    #[test]
    fn scalar_and_complex_analysis_conversion_retains_exact_typed_payloads() {
        use crate::state::{
            AnalysisResultPayload, ComplexResultValue, SensitivityResultMode, SensitivityResultRow,
        };

        let controller = SimulationController::new();
        let pole_zero = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::PoleZero {
                poles: vec![(-1.0, 2.0), (-1.0, -2.0)],
                zeros: vec![(-3.0, 0.0)],
                gain: 4.0,
            },
            AnalysisType::PoleZero,
            "PZ",
        );
        assert_eq!(
            pole_zero.result_payload,
            Some(AnalysisResultPayload::PoleZero {
                poles: vec![
                    ComplexResultValue {
                        real: -1.0,
                        imaginary: 2.0,
                    },
                    ComplexResultValue {
                        real: -1.0,
                        imaginary: -2.0,
                    },
                ],
                zeros: vec![ComplexResultValue {
                    real: -3.0,
                    imaginary: 0.0,
                }],
                gain: 4.0,
            })
        );
        assert!(pole_zero.has_data());

        let sensitivity = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Sensitivity {
                output: "V(out)".to_owned(),
                ac_mode: true,
                frequency_hz: Some(10_000.0),
                sensitivities: std::collections::HashMap::from([
                    ("width".to_owned(), 2.0),
                    ("length".to_owned(), -1.0),
                ]),
                normalized: std::collections::HashMap::from([
                    ("width".to_owned(), 0.5),
                    ("length".to_owned(), -0.25),
                ]),
            },
            AnalysisType::Sensitivity,
            "SENS",
        );
        assert_eq!(
            sensitivity.result_payload,
            Some(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Ac {
                    frequency_hz: 10_000.0,
                },
                rows: vec![
                    SensitivityResultRow {
                        parameter: "length".to_owned(),
                        raw: -1.0,
                        normalized: -0.25,
                    },
                    SensitivityResultRow {
                        parameter: "width".to_owned(),
                        raw: 2.0,
                        normalized: 0.5,
                    },
                ],
            })
        );

        let scalar = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::MeasurementsOnly {
                measurements: std::collections::HashMap::from([
                    ("zeta".to_owned(), 0.7),
                    ("gain".to_owned(), 10.0),
                ]),
            },
            AnalysisType::Tf,
            "TF",
        );
        assert_eq!(
            scalar.result_payload,
            Some(AnalysisResultPayload::ScalarMeasurements {
                values: std::collections::BTreeMap::from([
                    ("gain".to_owned(), 10.0),
                    ("zeta".to_owned(), 0.7),
                ]),
            })
        );
    }

    #[test]
    fn incomplete_reliability_and_soa_results_fail_closed_without_retained_payloads() {
        let controller = SimulationController::new();
        let reliability = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Reliability {
                years: vec![1.0, 10.0],
                waveforms: std::collections::HashMap::new(),
                device_results: Vec::new(),
            },
            AnalysisType::Reliability,
            "Reliability",
        );
        assert!(!reliability.success);
        assert!(reliability.result_payload.is_none());
        assert!(
            reliability
                .error_message
                .as_deref()
                .is_some_and(|message| message.contains("no device evidence"))
        );

        let soa = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Soa {
                time: vec![0.0, 1.0],
                waveforms: std::collections::HashMap::new(),
                violations: Vec::new(),
                evaluations: Vec::new(),
            },
            AnalysisType::Soa,
            "SOA",
        );
        assert!(!soa.success);
        assert!(soa.result_payload.is_none());
        assert!(
            soa.error_message
                .as_deref()
                .is_some_and(|message| message.contains("no evaluated-rule evidence"))
        );
    }

    #[test]
    fn invalid_sensitivity_result_contract_fails_closed() {
        let controller = SimulationController::new();
        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Sensitivity {
                output: "V(out)".to_owned(),
                ac_mode: false,
                frequency_hz: None,
                sensitivities: std::collections::HashMap::from([("width".to_owned(), 2.0)]),
                normalized: std::collections::HashMap::new(),
            },
            AnalysisType::Sensitivity,
            "SENS",
        );

        assert!(!analysis.success);
        assert!(analysis.result_payload.is_none());
        assert!(
            analysis
                .error_message
                .as_deref()
                .is_some_and(|message| message.contains("misaligned"))
        );
    }

    #[test]
    fn manual_deck_trigger_runs_deck_analysis_without_enabled_run_set() {
        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.set_enabled(transient_id, false)
            .expect("the sole run-set analysis disables");
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
        state.simulation.request_manual_deck_run();
        let mut controller = SimulationController::new();
        controller
            .validate_manual_deck_document(&state)
            .expect("explicit validation authorizes the exact manual deck");

        controller.start_simulation(&mut state);
        let total_analyses = controller.total_analyses;
        let current_spec = controller.current_spec.clone();
        let source_domain = controller
            .current_provenance
            .as_ref()
            .map(AnalysisResultProvenance::source_domain);
        let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
        let run_count = state.simulation.runs.len();
        let status = state.simulation.status.clone();
        controller.abort();

        assert_eq!(total_analyses, 1);
        assert!(matches!(current_spec, Some(AnalysisSpec::DcOp)));
        assert_eq!(source_domain, Some(AnalysisResultSourceDomain::ManualDeck));
        assert!(cached_netlist.contains(".op\n.end"));
        assert_eq!(run_count, 1);
        assert_eq!(status, "DC Operating Point");
    }

    #[test]
    fn controller_manual_run_receipt_remains_authoritative_if_result_provenance_is_stripped() {
        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.set_enabled(transient_id, false)
            .expect("run-set analysis disables");
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
        let project_revision = state.workspace.project.revision();
        state.simulation.request_manual_deck_run();
        let mut controller = SimulationController::new();
        controller
            .validate_manual_deck_document(&state)
            .expect("explicit validation authorizes the exact manual deck");

        controller.start_simulation(&mut state);

        let task_provenance = controller
            .current_provenance
            .clone()
            .expect("manual task owns exact provenance");
        let expanded = controller
            .cached_netlist
            .as_deref()
            .expect("sealed manual deck");
        let expected_source_digest = crate::simulation::execution::content_digest(
            "rspice.manual-executable-source/v1",
            expanded.as_bytes(),
        );
        {
            let run = state.simulation.active_run().expect("manual run starts");
            let receipt = run.prepared_receipt().expect("manual run is sealed");
            assert_eq!(
                receipt.source_domain(),
                AnalysisResultSourceDomain::ManualDeck
            );
            assert_eq!(receipt.simulation_plan_id(), None);
            assert_eq!(receipt.project_revision(), project_revision);
            assert_eq!(
                receipt.prepared_snapshot_digest(),
                task_provenance.prepared_snapshot_digest()
            );
            assert_eq!(receipt.source_content_digest(), expected_source_digest);
            assert!(receipt.source_check_receipt().is_manual_source_check());
            assert_eq!(receipt.tasks().len(), 1);
            let task = &receipt.tasks()[0];
            assert_eq!(task.instance_id(), task_provenance.source_instance_id());
            assert_eq!(task.source_revision(), project_revision);
            assert_eq!(task.analysis_kind_tag(), 0);
            assert!(task.dependencies().is_empty());
        }

        let run = state
            .simulation
            .active_run_mut()
            .expect("manual run remains active");
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_provenance(task_provenance),
        );
        assert!(run.validate_provenance().is_ok());
        run.analyses[0].provenance = None;
        assert!(matches!(
            run.provenance(),
            Some(SimulationRunProvenance::Prepared(_))
        ));
        assert!(run.validate_provenance().is_err());

        controller.abort();
    }

    #[test]
    fn controller_manual_run_receipt_survives_production_project_round_trip() {
        let mut state = AppState::default();
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
        state.simulation.request_manual_deck_run();
        let mut controller = SimulationController::new();
        controller
            .validate_manual_deck_document(&state)
            .expect("explicit validation authorizes the exact manual deck");
        controller.start_simulation(&mut state);
        let task_provenance = controller
            .current_provenance
            .clone()
            .expect("controller owns the prepared manual task");
        let expected_source_id = task_provenance.source_instance_id();
        state
            .simulation
            .active_run_mut()
            .expect("prepared manual run")
            .add_analysis(
                AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_provenance(task_provenance),
            );
        controller.abort();

        let project = crate::common::project_lifecycle::snapshot(&state)
            .expect("production snapshot accepts controller manual run");
        let json = crate::io::project_io::serialize_project_file(&project)
            .expect("controller manual run serializes");
        let loaded = crate::io::project_io::load_project_text(&json, None)
            .expect("controller manual run reloads");
        assert!(
            loaded.execution_context.is_some(),
            "manual receipt must remain independent of the unrelated retained plan"
        );
        let restored = loaded
            .simulation_results
            .into_simulation_state()
            .expect("controller manual history restores");
        let run = &restored.runs[0];
        let receipt = run.prepared_receipt().expect("manual receipt retained");
        let result_provenance = run.analyses[0]
            .provenance
            .as_ref()
            .expect("manual result provenance retained");

        assert_eq!(
            receipt.source_domain(),
            AnalysisResultSourceDomain::ManualDeck
        );
        assert_eq!(receipt.simulation_plan_id(), None);
        assert!(receipt.source_check_receipt().is_manual_source_check());
        assert_eq!(receipt.tasks()[0].instance_id(), expected_source_id);
        assert_eq!(result_provenance.source_instance_id(), expected_source_id);
        assert_eq!(
            result_provenance.prepared_snapshot_digest(),
            receipt.prepared_snapshot_digest()
        );
    }

    #[test]
    fn manual_deck_run_preserves_editor_source_without_ui_option_injection() {
        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.set_enabled(transient_id, false)
            .expect("the sole run-set analysis disables");
        state.sim_setup.options.reltol = 1.0e-4;
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
        state.simulation.request_manual_deck_run();
        let mut controller = SimulationController::new();
        controller
            .validate_manual_deck_document(&state)
            .expect("explicit validation authorizes the exact manual deck");

        controller.start_simulation(&mut state);
        let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
        controller.abort();

        assert_eq!(cached_netlist, "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
        assert!(!cached_netlist.contains(".OPTIONS"));
        assert!(!cached_netlist.contains("RELTOL"));
    }

    #[test]
    fn manual_deck_runs_use_imported_netlist_origin_for_relative_includes() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
        state.workspace.netlist_source =
            Some("deck\n.include models.lib\nV1 out 0 1\n.op\n.end\n".to_string());
        state.workspace.netlist_source_path = Some(PathBuf::from("decks").join("bias.cir"));

        assert_eq!(
            SimulationController::analysis_source_path(&state).as_deref(),
            Some(std::path::Path::new("decks").join("bias.cir").as_path())
        );
    }

    #[test]
    fn manual_deck_runs_do_not_fall_back_to_schematic_path() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
        state.workspace.netlist_source = Some("deck\nV1 out 0 1\n.op\n.end\n".to_string());

        assert!(
            SimulationController::analysis_source_path(&state).is_none(),
            "manual netlist text without an origin must not resolve includes from the schematic file"
        );
    }

    #[test]
    fn simulate_run_set_does_not_run_manual_deck_source() {
        let mut state = AppState::default();
        let plan = state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan");
        let transient_id = plan.instances()[0].id();
        plan.set_enabled(transient_id, false)
            .expect("default transient disables");
        let (op_id, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts as the sole enabled analysis");
        assert_eq!(plan.instances()[0].id(), op_id);
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
        state.simulation.request_simulate_run_set();
        let mut controller = SimulationController::new();

        controller.start_simulation(&mut state);
        let status = state.simulation.status.clone();
        let run_count = state.simulation.runs.len();
        let total_analyses = controller.total_analyses;
        controller.abort();

        assert_eq!(status, "Run blocked");
        assert_eq!(run_count, 0);
        assert_eq!(total_analyses, 0);
    }

    #[test]
    fn successful_manual_deck_run_promotes_pending_baseline() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        state.simulation.netlist_content =
            "deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string();
        let run_id = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_id);
        state.ui.netlist.pending_manual_run_id = Some(run_id);
        state.ui.netlist.pending_run_buffer =
            Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string());
        state.ui.netlist.edited_lines.insert(0);

        controller.finish_simulation_batch(&mut state);

        assert_eq!(
            state.ui.netlist.last_run_buffer.as_deref(),
            Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n")
        );
        assert!((state.ui.netlist.last_run_params["r"] - 1e3).abs() < 1e-9);
        assert!((state.ui.netlist.last_run_params["cl"] - 2e-12).abs() < 1e-21);
        assert!(!state.ui.netlist.last_run_params.contains_key("expr"));
        assert!(state.ui.netlist.pending_manual_run_id.is_none());
        assert!(state.ui.netlist.pending_run_buffer.is_none());
        assert!(state.ui.netlist.edited_lines.is_empty());
    }

    #[test]
    fn failed_manual_deck_run_keeps_previous_baseline() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        state.ui.netlist.last_run_buffer = Some("old\n.op\n.end\n".to_string());
        let run = state.simulation.start_run();
        let run_id = run.id;
        run.success = false;
        bind_test_run_running(&mut state, &mut controller, run_id);
        state.ui.netlist.pending_manual_run_id = Some(run_id);
        state.ui.netlist.pending_run_buffer = Some("new\n.op\n.end\n".to_string());

        controller.finish_simulation_batch(&mut state);

        assert_eq!(
            state.ui.netlist.last_run_buffer.as_deref(),
            Some("old\n.op\n.end\n")
        );
        assert!(state.ui.netlist.pending_manual_run_id.is_none());
        assert!(state.ui.netlist.pending_run_buffer.is_none());
    }

    #[test]
    fn successful_manual_deck_run_preserves_post_launch_diff_pips() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        state.simulation.netlist_content = "deck\n.op\nR1 out 0 2k\n.end\n".to_string();
        let run_id = state.simulation.start_run().id;
        bind_test_run_running(&mut state, &mut controller, run_id);
        state.ui.netlist.pending_manual_run_id = Some(run_id);
        state.ui.netlist.pending_run_buffer = Some("deck\n.op\nR1 out 0 1k\n.end\n".to_string());

        controller.finish_simulation_batch(&mut state);

        assert_eq!(
            state.ui.netlist.last_run_buffer.as_deref(),
            Some("deck\n.op\nR1 out 0 1k\n.end\n")
        );
        assert!(state.ui.netlist.edited_lines.contains(&2));
        assert_eq!(state.ui.netlist.edited_lines.len(), 1);
    }

    #[test]
    fn ui_progress_fraction_uses_runner_fraction_or_running_floor() {
        assert!((SimulationController::ui_progress_fraction(Some(0.42), true) - 0.42).abs() < 1e-6);
        assert_eq!(SimulationController::ui_progress_fraction(None, true), 0.08);
        assert_eq!(SimulationController::ui_progress_fraction(None, false), 0.0);
        assert_eq!(
            SimulationController::ui_progress_fraction(Some(1.2), true),
            1.0
        );
    }
}
