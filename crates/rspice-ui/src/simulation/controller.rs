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

use std::collections::{HashMap, HashSet, VecDeque};
#[cfg(test)]
use std::path::PathBuf;

use crate::diagnostics::ConsoleMessage;
use crate::io::{SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter};
use crate::services::yield_manager::{YieldAnalysisManager, YieldAnalysisProvenance};
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, DcSweepConfig, NoiseAnalysisConfig, NoiseSweepType,
    PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::execution::{ExecutionArtifactEnvelope, TouchstoneExportPolicy};
use crate::simulation::multi_run::{
    AnalysisRunType, AnalysisSpec, FrequencySweep, HbToneSpec, OptimizationAlgorithm,
    OptimizationGoal, OptimizationVariable, PssMethod, SpPort,
};
use crate::simulation::output_contract::{PreparedSavedOutput, materialize_saved_outputs};
use crate::simulation::plan::AnalysisNumericOverride;
use crate::simulation::runner::SimulationError;
use crate::simulation::runner::SpecExecutionOptions;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, ComplexResultValue, DcOpResult,
    MonteCarloVariableMetadata, OperatingPointValue, PeriodicNoiseOutputQuantity,
    ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence, ReliabilityShiftEvidence,
    ReliabilityStressEvidence, SensitivityResultMode, SensitivityResultRow, SimulationRunIntent,
    SimulationRunLifecycle, SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence,
    SoaViolationEvidence, SoaViolationSeverityEvidence,
};
use crate::workbench::app_state::{ActiveViewer, AppState, SpecializedViewerCacheProvenance};
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;

mod analysis_commands;
mod analysis_helpers;
mod analysis_plan;
mod analysis_run_config;
mod analysis_spec_build;
mod manual_deck;
pub(crate) mod prepared_run;
#[cfg(test)]
mod projection_ratchet;
mod results_convert;
mod results_post;
mod results_update;
pub(crate) mod spice_value;
mod touchstone;
mod transient_post;
pub(super) use analysis_commands::splice_before_terminal_end_card;
pub(crate) use transient_post::DerivedViewerLoadState;

use self::spice_value::parse_spice_value_checked;

#[derive(Debug, Clone)]
pub(super) struct QueuedAnalysis {
    pub(super) spec: AnalysisSpec,
    pub(super) config: Option<AnalysisConfig>,
    pub(super) spec_options: SpecExecutionOptions,
    pub(super) analysis_line: String,
    /// Numerical departures authored against this analysis. Snapshot
    /// preparation turns them into a second `.OPTIONS` block in this task's own
    /// deck; a manual deck states its options in the deck itself and therefore
    /// never carries one.
    pub(super) numeric_override: Option<AnalysisNumericOverride>,
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
    /// Exact typed execution options for the active spec-driven task. These
    /// carry result semantics, such as whether PNOISE produced output PSD or
    /// dBc/Hz phase noise, across the asynchronous runner boundary.
    current_spec_options: Option<SpecExecutionOptions>,
    /// Frozen identity of the prepared task currently owned by the runner.
    /// Captured before the authorized dispatch token is moved into the runner.
    current_provenance: Option<AnalysisResultProvenance>,
    /// Digest of the exact prepared payload currently executing.
    current_config_digest: Option<crate::product::ContentDigest>,
    /// Identity of the exact per-task executable source. Process-corner
    /// overrides are already materialized in these bytes; voltage-corner
    /// parameters remain authenticated in the OP seed payload.
    current_effective_source_content_digest: Option<crate::product::ContentDigest>,
    /// Source identity extended with the exact OP voltage-corner mutation.
    current_op_effective_source_content_digest: Option<crate::product::ContentDigest>,
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
    /// Batch-local numerical artifacts keyed by their exact producer task.
    execution_artifacts: HashMap<crate::product::AnalysisInstanceId, ExecutionArtifactEnvelope>,
    /// The PVT declarations this batch expanded into points. A declaration is
    /// not dispatched, so its plotting family is assembled from the point
    /// results when its own turn in the queue comes.
    point_families: crate::simulation::point_family::PointFamilyRegistry,
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
            current_spec_options: None,
            current_provenance: None,
            current_config_digest: None,
            current_effective_source_content_digest: None,
            current_op_effective_source_content_digest: None,
            current_saved_output_contracts: Vec::new(),
            current_source_domain: AnalysisResultSourceDomain::SimulationPlan,
            current_run_id: None,
            pending_analyses: VecDeque::new(),
            successful_analysis_instances: HashSet::new(),
            execution_artifacts: HashMap::new(),
            point_families: crate::simulation::point_family::PointFamilyRegistry::default(),
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
        state.synchronize_specialized_viewer_cache_authority();
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
        self.execution_artifacts.clear();
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

        self.point_families.clear();
        for task in dispatch.tasks() {
            self.point_families.register(task);
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
        self.execution_artifacts.clear();
        self.point_families.clear();
        self.cached_netlist = None;
        self.clear_prepared_run();
        self.current_config = None;
        self.current_spec = None;
        self.current_spec_options = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_effective_source_content_digest = None;
        self.current_op_effective_source_content_digest = None;
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
        self.current_spec_options = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_effective_source_content_digest = None;
        self.current_op_effective_source_content_digest = None;
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
            let provenance = match AnalysisResultProvenance::new_with_authored_source_domain(
                self.current_source_domain,
                candidate.instance_id(),
                candidate.authored_instance_id(),
                candidate.source_revision(),
                candidate.snapshot_digest(),
                candidate.dependencies().to_vec(),
            )
            .map(|provenance| provenance.with_pvt_point(candidate.pvt_point().cloned()))
            {
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
        let provenance = match AnalysisResultProvenance::new_with_authored_source_domain(
            self.current_source_domain,
            next_analysis.instance_id(),
            next_analysis.authored_instance_id(),
            next_analysis.source_revision(),
            next_analysis.snapshot_digest(),
            next_analysis.dependencies().to_vec(),
        )
        .map(|provenance| provenance.with_pvt_point(next_analysis.pvt_point().cloned()))
        {
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

        // A PVT declaration's turn assembles the family from the point results
        // the queue has already retained. It never reaches the runner: the
        // points are the solve, and handing the declaration to an executor is
        // what used to make an N-point sweep cost 2N. Its saved outputs are
        // taken here all the same, because a declaration that skips the runner
        // still produces a result an output contract was written against.
        //
        // The registry decides, not the spec: it holds exactly the declarations
        // this dispatch expanded, so a parametric run that steps a design
        // parameter rather than a temperature is not in it and still reaches
        // the engine that sweeps it.
        if self.point_families.declares(next_analysis.instance_id()) {
            let analysis_type = self.spec_to_analysis_type(&spec);
            self.current_saved_output_contracts = next_analysis.saved_output_contracts().to_vec();
            self.assemble_point_family(state, &analysis_name, analysis_type, provenance);
            return;
        }

        self.current_config = config.clone();
        self.current_spec = Some(spec.clone());
        self.current_spec_options = Some(next_analysis.spec_options().clone());
        self.current_provenance = Some(provenance);
        self.current_config_digest = Some(next_analysis.config_digest());
        self.current_effective_source_content_digest = Some(
            crate::workbench::documents::netlist_document::source_content_digest(
                next_analysis.executable_netlist(),
            ),
        );
        self.current_op_effective_source_content_digest = config.as_ref().and_then(|config| {
            let AnalysisConfig::DcOp(config) = config else {
                return None;
            };
            Some(
                crate::simulation::execution::operating_point_effective_source_digest(
                    next_analysis.executable_netlist(),
                    config.run_point,
                ),
            )
        });
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
        let start_result = next_analysis
            .resolve_dependency_artifacts(&self.execution_artifacts)
            .map_err(|error| SimulationError::InvalidConfig(error.to_string()))
            .and_then(|dispatch| self.runner.start_prepared(dispatch));
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

    /// Complete a PVT declaration's task without an engine call.
    ///
    /// The declaration keeps its place in the queue because the run's
    /// authenticated receipt has a task for it and retained results must stay
    /// an exact ordered prefix of that list. Its points are already retained by
    /// the time its turn comes, so the family is a reduction of them.
    fn assemble_point_family(
        &mut self,
        state: &mut AppState,
        analysis_name: &str,
        analysis_type: AnalysisType,
        provenance: AnalysisResultProvenance,
    ) {
        let declaration = provenance.source_instance_id();
        let target_run_id = self.target_run_id(state);
        let assembled = target_run_id
            .and_then(|run_id| state.simulation.run_by_sequence(run_id))
            .ok_or_else(|| "PVT family has no target simulation run".to_owned())
            .and_then(|run| self.point_families.family_for(declaration, run));
        let mut analysis = match assembled {
            Ok(result) => self.convert_to_analysis_result_with_metadata_owned(
                result,
                analysis_type,
                analysis_name,
            ),
            Err(error) => {
                state.push_sim_message(ConsoleMessage::error(format!("{analysis_name}: {error}")));
                AnalysisResult::failed(1, analysis_type, analysis_name, error)
            }
        };
        self.materialize_current_saved_outputs(&mut analysis);

        // Retention already fails the run when the family did, so only a
        // retention error of its own needs answering here.
        if let Err(error) =
            self.retain_completed_analysis(state, target_run_id, analysis, provenance)
        {
            log::error!("{error}");
            state.push_sim_message(ConsoleMessage::error(error));
            if let Some(run_id) = target_run_id
                && let Some(run) = state.simulation.run_by_sequence_mut(run_id)
            {
                run.success = false;
            }
        }
        if let Some(run_id) = target_run_id {
            state
                .simulation
                .select_latest_analysis_in_run_sequence(run_id);
        }

        if self.pending_analyses.is_empty() {
            self.finish_simulation_batch(state);
        } else {
            self.start_next_analysis(state);
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
        self.point_families.clear();
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
        self.execution_artifacts.clear();
        self.current_config = None;
        self.current_spec = None;
        self.current_spec_options = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_effective_source_content_digest = None;
        self.current_op_effective_source_content_digest = None;
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
                if let Ok(value) = crate::quantity::engineering::parse_engineering_value(raw) {
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
            AnalysisConfig::DcOp(_) => "DC Operating Point",
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
            AnalysisConfig::DcOp(_) => AnalysisType::DcOp,
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
            // A retained coefficient spectrum is exactly what the harmonic
            // balance viewer already draws, so the spectrum reuses that owner
            // rather than introducing a second one that renders the same fact.
            AnalysisRunType::PssSpectrum => AnalysisType::HarmonicBalance,
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

                    let required_artifact_waveforms = self
                        .current_provenance
                        .as_ref()
                        .map(|provenance| provenance.source_instance_id())
                        .into_iter()
                        .flat_map(|producer| {
                            self.pending_analyses.iter().filter_map(move |task| {
                                let AnalysisSpec::Fourier {
                                    output_node,
                                    output_ref,
                                    ..
                                } = task.spec()
                                else {
                                    return None;
                                };
                                task.dependencies().contains(&producer).then(|| {
                                    let mut required = vec![output_node.clone()];
                                    if !output_ref.trim().is_empty()
                                        && !output_ref.trim().eq_ignore_ascii_case("0")
                                    {
                                        required.push(output_ref.clone());
                                    }
                                    required
                                })
                            })
                        })
                        .flatten()
                        .collect::<Vec<_>>();
                    let periodic_artifact_required = self
                        .current_provenance
                        .as_ref()
                        .map(|provenance| provenance.source_instance_id())
                        .is_some_and(|producer| {
                            self.pending_analyses.iter().any(|task| {
                                task.dependencies().contains(&producer)
                                    && matches!(
                                        task.spec(),
                                        AnalysisSpec::Pac
                                            | AnalysisSpec::Pxf
                                            | AnalysisSpec::Pnoise
                                            | AnalysisSpec::Pstb
                                    )
                            })
                        });
                    let dc_seed_artifact_required = self
                        .current_provenance
                        .as_ref()
                        .map(|provenance| provenance.source_instance_id())
                        .is_some_and(|producer| {
                            self.pending_analyses.iter().any(|task| {
                                task.dependencies().contains(&producer)
                                    && matches!(
                                        task.spec(),
                                        AnalysisSpec::Pss {
                                            method: PssMethod::Shooting,
                                            ..
                                        }
                                    )
                            })
                        });
                    let produced_artifact = match (
                        self.current_spec.as_ref(),
                        self.current_provenance.as_ref(),
                        self.current_config_digest,
                    ) {
                        (
                            Some(AnalysisSpec::Transient { .. }),
                            Some(provenance),
                            Some(config_digest),
                        ) if !required_artifact_waveforms.is_empty() => {
                            ExecutionArtifactEnvelope::from_transient_result(
                                provenance.prepared_snapshot_digest(),
                                provenance.source_instance_id(),
                                provenance.source_revision(),
                                config_digest,
                                &sim_result,
                                &required_artifact_waveforms,
                            )
                            .map_err(|error| {
                                format!(
                                    "Transient result could not produce its authenticated dependency artifact: {error}"
                                )
                            })
                        }
                        (
                            Some(pss_spec @ AnalysisSpec::Pss { .. }),
                            Some(provenance),
                            Some(config_digest),
                        ) if periodic_artifact_required => {
                            ExecutionArtifactEnvelope::from_periodic_result(
                                provenance.prepared_snapshot_digest(),
                                provenance.source_instance_id(),
                                provenance.source_revision(),
                                config_digest,
                                pss_spec,
                                &sim_result,
                            )
                            .map_err(|error| {
                                format!(
                                    "PSS result could not produce its authenticated periodic-state artifact: {error}"
                                )
                            })
                        }
                        (
                            Some(AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }),
                            Some(provenance),
                            Some(config_digest),
                        ) if dc_seed_artifact_required => {
                            match (
                                self.current_effective_source_content_digest,
                                self.current_config.as_ref(),
                            ) {
                                (
                                    Some(effective_source_content_digest),
                                    Some(AnalysisConfig::DcOp(prepared_config)),
                                ) => {
                                    ExecutionArtifactEnvelope::from_dc_operating_point_result(
                                        provenance.prepared_snapshot_digest(),
                                        provenance.source_instance_id(),
                                        provenance.source_revision(),
                                        config_digest,
                                        effective_source_content_digest,
                                        prepared_config,
                                        &sim_result,
                                    )
                                    .map_err(|error| {
                                        format!(
                                            "Operating-point result could not produce its authenticated shooting-PSS seed: {error}"
                                        )
                                    })
                                }
                                _ => Err(
                                    "operating-point result has no authenticated effective source and prepared configuration"
                                        .to_owned(),
                                ),
                            }
                        }
                        _ => Ok(None),
                    };
                    let (produced_artifact, artifact_failure) = match produced_artifact {
                        Ok(artifact) => (artifact, None),
                        Err(message) => {
                            log::error!("{message}");
                            state.push_sim_message(ConsoleMessage::error(message.clone()));
                            (None, Some(message))
                        }
                    };

                    self.apply_result_side_effects(state, &sim_result);
                    if let crate::simulation::SimulationResult::Transient {
                        time,
                        waveforms,
                        convergence,
                        ..
                    } = &sim_result
                    {
                        self.populate_transient_post_views(state, time, waveforms);
                        // Only when the solver needed help. A clean run says
                        // nothing, so the console stays a signal rather than a
                        // per-run receipt.
                        if convergence.has_issues() {
                            state.push_sim_message(ConsoleMessage::warning(format!(
                                "{current_label}: {}",
                                convergence.summary()
                            )));
                        }
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
                                YieldAnalysisProvenance::from_monte_carlo_result(
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
                    // Point results are one analysis solved at several
                    // conditions, so the analysis name alone repeats across
                    // every sibling. The point is what tells them apart.
                    if let Some(point) = self
                        .current_provenance
                        .as_ref()
                        .and_then(AnalysisResultProvenance::pvt_point)
                    {
                        analysis_result.label =
                            format!("{} \u{00b7} {}", analysis_result.label, point.label());
                    }
                    self.retain_periodic_noise_result_metadata(&mut analysis_result);
                    if let Some(AnalysisResultPayload::OperatingPoint {
                        effective_source_content_digest,
                        ..
                    }) = analysis_result.result_payload.as_mut()
                    {
                        *effective_source_content_digest =
                            self.current_op_effective_source_content_digest;
                    }
                    if let Some(message) = artifact_failure {
                        analysis_result.success = false;
                        analysis_result.error_message = Some(message);
                    }
                    self.materialize_current_saved_outputs(&mut analysis_result);
                    let retention_error = analysis_result.error_message.clone();
                    if let Some(provenance) = self.current_provenance.take() {
                        let completed_instance = provenance.source_instance_id();
                        match self.retain_completed_analysis(
                            state,
                            target_run_id,
                            analysis_result,
                            provenance,
                        ) {
                            Ok(true) => {
                                if let Some(artifact) = produced_artifact {
                                    self.execution_artifacts
                                        .insert(completed_instance, artifact);
                                }
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
                    self.execution_artifacts.clear();
                    self.point_families.clear();
                    self.cached_netlist = None;
                    self.current_config = None;
                    self.current_spec = None;
                    self.current_spec_options = None;
                    self.current_provenance = None;
                    self.current_config_digest = None;
                    self.current_effective_source_content_digest = None;
                    self.current_op_effective_source_content_digest = None;
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
    #[cfg(test)]
    pub fn is_running(&self) -> bool {
        self.runner.is_running()
    }

    fn ui_progress_fraction(progress: Option<f32>, is_running: bool) -> f64 {
        progress
            .map(|value| f64::from(value).clamp(0.0, 1.0))
            .unwrap_or_else(|| if is_running { 0.08 } else { 0.0 })
    }

    /// Abort current simulation
    #[cfg(test)]
    pub fn abort(&self) {
        self.runner.abort();
    }
}

#[cfg(test)]
mod tests;
