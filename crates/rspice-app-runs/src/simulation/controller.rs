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

use crate::diagnostics::LogSeverity;
use crate::simulation::RunHost;
#[cfg(test)]
use crate::simulation::config::NoiseSweepType;
use crate::simulation::execution::TouchstoneExportPolicy;
use crate::simulation::multi_run::AnalysisSpec;
#[cfg(test)]
use crate::simulation::multi_run::FrequencySweep;
#[cfg(test)]
use crate::simulation::multi_run::PssMethod;
#[cfg(test)]
use crate::simulation::multi_run::SpPort;
use crate::simulation::output_contract::materialize_live_saved_outputs;
use crate::simulation::output_contract::retain_plan_saved_outputs;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
#[cfg(test)]
use crate::state::AnalysisResultFamilyMetadata;
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisResultProvenance, AnalysisResultSourceDomain,
    AnalysisType, DigitalEventPointEvidence, DigitalEventTraceEvidence, RealEventPointEvidence,
    RealEventTraceEvidence, RetainedTrace, RunEvent, ShownWaveforms, SimulationRunIntent,
    SimulationRunLifecycle,
};
use rspice_app_services::workbench::platform::spawn_or_inline;
use rspice_app_services::workbench::workflows::export_workflow::ExportWorkflowIo;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_simulation::error::SimulationError;
use rspice_simulation::execution_artifact::ExecutionArtifactEnvelope;
use rspice_simulation::execution_identity::canonical_analysis_kind;
use rspice_simulation::execution_options::SpecExecutionOptions;
use rspice_simulation::live_transient::{
    TransientDigitalBusSample, TransientDigitalEventSample, TransientRealEventSample,
    TransientSampleDelta,
};
#[cfg(test)]
use rspice_simulation::manual_deck;
use rspice_simulation::output_contract::PreparedSavedOutput;
#[cfg(test)]
use rspice_simulation::preparation::QueuedAnalysis;
use rspice_simulation_contract::setup_state::SimulationSetup;

mod analysis_commands;
mod analysis_plan;
#[cfg(test)]
mod analysis_run_config;
mod analysis_spec_build;
mod completion;
use analysis_spec_build::analysis_inputs;
mod deck_preparation;
#[cfg(any(test, feature = "test-support"))]
pub use deck_preparation::{DeckPreparationHold, prepare_deck_runs_away_here};
pub use deck_preparation::{DeckRunVerdict, prepare_deck_runs_away_from_frame};
mod engine_check;
pub use engine_check::{PreflightOutcome, PreflightStage, PreflightVerdict};
#[cfg(test)]
pub(crate) mod dc_history_tests;
#[cfg(test)]
mod directive_note_tests;
#[cfg(test)]
mod directive_parse_ratchet;
#[cfg(not(target_arch = "wasm32"))]
mod live_segment;
mod live_transient;
mod machine_memory;
#[cfg(test)]
mod manual_deck_tests;
mod monte_carlo_checkpoint;
pub mod prepared_run;
#[cfg(test)]
mod projection_ratchet;
mod results_convert;
pub mod results_post;
mod results_update;
mod run_inputs;
mod run_lines;
use run_lines::write_run_line;
mod stored_results;
#[cfg(any(test, feature = "test-support"))]
pub mod test_execution;
#[cfg(any(test, feature = "test-support"))]
pub mod test_host;
mod touchstone;
mod transient_checkpoint;
pub mod transient_post;
pub use transient_post::{DerivedViewerLoadState, build_eye_from_waveform};

use self::live_transient::LiveTransientAccumulator;

#[derive(Clone)]
pub struct PreparedCampaignMember {
    plan_name: String,
    snapshot: crate::simulation::execution::PreparedRunSnapshot,
}

pub struct ActiveSimulationCampaign {
    id: crate::product::SimulationCampaignId,
    name: String,
    member_count: u32,
    dispatched_count: u32,
    completed_count: u32,
    failed_count: u32,
    cancelled: bool,
    pub pending: VecDeque<PreparedCampaignMember>,
    /// When the campaign was queued, for the time its summary reports.
    started: crate::time_compat::Instant,
}

/// Reviewed forecast returned after a campaign has been frozen and its first
/// member has been submitted to the local execution queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationCampaignDispatchReceipt {
    pub campaign_id: crate::product::SimulationCampaignId,
    pub member_count: usize,
    pub task_count: usize,
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
    pub runner: SimulationRunner,
    /// Current analysis config (stored for result processing when polling completes)
    current_config: Option<AnalysisConfig>,
    /// Current strongly-typed analysis spec (always set while running)
    pub current_spec: Option<AnalysisSpec>,
    /// Immutable prepared-task label, including its exact Run Set point.
    current_analysis_label: Option<String>,
    /// Exact typed execution options for the active spec-driven task. These
    /// carry result semantics, such as whether PNOISE produced output PSD or
    /// dBc/Hz phase noise, across the asynchronous runner boundary.
    current_spec_options: Option<SpecExecutionOptions>,
    /// Receiving resource limits authenticated by the active prepared task.
    current_execution_limits: Option<rspice_core::ResourceLimits>,
    /// A failed requested snapshot must also fail a concurrently completed task.
    current_transient_checkpoint_error: Option<String>,
    /// The fundamental the periodic carrier of the active task actually
    /// converged at, in hertz, captured from the resolved dependency at
    /// dispatch.
    ///
    /// This is not the fundamental the card authored. An autonomous carrier
    /// holds its period as the shooting solver's unknown and the solver moves
    /// it, so the frequency a conversion or noise basis is built on is a
    /// property of the *carrier*, not of the configuration that asked for it.
    /// The authored guess reaches the runner as
    /// `PnoiseRunConfig::pss_fundamental_freq` and is what
    /// `PeriodicStateArtifact::validate_consumer_basis` matches on; it is the
    /// wrong number to publish as the carrier a result was measured against.
    current_periodic_carrier_hz: Option<f64>,
    /// Authenticated producer context captured from the resolved dispatch.
    current_artifact_producer:
        Option<rspice_simulation::execution_artifact::PreparedArtifactProducer>,
    /// Frozen identity of the prepared task currently owned by the runner.
    /// Captured before the authorized dispatch token is moved into the runner.
    pub current_provenance: Option<AnalysisResultProvenance>,
    /// Digest of the exact prepared payload currently executing.
    current_config_digest: Option<crate::product::ContentDigest>,
    /// Source identity extended with the exact OP voltage-corner mutation.
    current_op_effective_source_content_digest: Option<crate::product::ContentDigest>,
    /// Immutable output contracts authenticated with the current task before
    /// its dispatch token is moved into the runner.
    current_saved_output_contracts: Vec<PreparedSavedOutput>,
    /// Save/streaming policy authenticated by the active prepared snapshot.
    current_save_policy: crate::simulation::execution::SavePolicy,
    /// Source domain authenticated by the active run dispatch. Manual-deck
    /// task IDs are deterministic source projections, not plan-owned IDs.
    current_source_domain: AnalysisResultSourceDomain,
    /// The outputs the running manual deck names: what its run plots.
    current_deck_outputs: crate::simulation::execution::DeckOutputs,
    /// Stable run ID that owns the in-flight batch.
    pub current_run_id: Option<u64>,
    /// When the runner took the active task, for its line in the run's log.
    current_task_started: Option<crate::time_compat::Instant>,
    /// What the run advised when it started, which its tasks' engine logs
    /// repeat and its log states once.
    current_advisories: Vec<String>,
    /// Where the active task's numbered engine lines were kept in its run's
    /// log, by the ordinal its sink gave each: what a notice the task raised
    /// names the line that stated it by.
    current_engine_lines: HashMap<u64, usize>,
    /// Exact accepted samples observed for the current transient task. These
    /// back both adaptive live presentation and a truthful partial result if
    /// the task terminates before returning a full engine result.
    live_transient: LiveTransientAccumulator,
    /// The rows the running transient's segment holds, which the desktop's live plot draws.
    #[cfg(not(target_arch = "wasm32"))]
    live_segment: live_segment::LiveSegmentFollower,
    /// When the live plot was last drawn again, and whether newer samples wait for it.
    last_live_publish: Option<crate::time_compat::Instant>,
    live_publish_due: bool,
    /// The results store each kept transient wrote, by its segment's path: the run and the
    /// analysis a stored column that cannot be read is said in.
    stored_segments: HashMap<String, (u64, String)>,

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
    pub current_analysis_idx: usize,
    /// Total number of analyses in current batch
    pub total_analyses: usize,
    /// Cached netlist for multi-analysis runs (avoids regeneration)
    cached_netlist: Option<std::sync::Arc<str>>,
    /// Runtime coordinator for transient-derived viewer data (eye/FFT).
    pub transient_post: transient_post::TransientPostCoordinator,
    /// App-state design epoch this controller's runner/queue belong to.
    design_execution_epoch: u64,
    /// Retained preflight and generation-safe dispatch authority.
    pub run_authorization: crate::simulation::execution::PreparedRunAuthorization,
    /// What the retained run set was prepared from, which its dispatch
    /// checks. Set and dropped with the retained run.
    retained_run_inputs: Option<run_inputs::RetainedRunInputs>,
    /// Moves whenever the retained run is dropped or replaced, so a preflight
    /// pipeline can tell whether the run it prepared is still wanted.
    retained_run_generation: u64,
    /// Preflight's preparation and engine pass, while they run.
    preflight: Option<engine_check::PendingPreflight>,
    #[cfg(any(test, feature = "test-support"))]
    preflight_test_hooks: engine_check::PreflightTestHooks,
    /// A deck's run being prepared away from the frame, from the deck a Run
    /// press took.
    deck_preparation: Option<deck_preparation::PendingDeckPreparation>,
    /// A test's hold on the next deck preparation's worker.
    #[cfg(any(test, feature = "test-support"))]
    deck_preparation_hold: Option<deck_preparation::WorkerHold>,
    /// Run-bound automatic export policy captured by immutable preflight.
    touchstone_export_policy: TouchstoneExportPolicy,
    /// Frozen, declared-order multi-plan campaign. Each member is an
    /// independent prepared run; this record only schedules and rolls them up.
    pub active_campaign: Option<ActiveSimulationCampaign>,
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
            current_config: None,
            current_spec: None,
            current_analysis_label: None,
            current_spec_options: None,
            current_execution_limits: None,
            current_transient_checkpoint_error: None,
            current_periodic_carrier_hz: None,
            current_artifact_producer: None,
            current_provenance: None,
            current_config_digest: None,
            current_op_effective_source_content_digest: None,
            current_saved_output_contracts: Vec::new(),
            current_save_policy:
                crate::simulation::execution::SavePolicy::RetainEngineProducedResults,
            current_source_domain: AnalysisResultSourceDomain::SimulationPlan,
            current_deck_outputs: Default::default(),
            current_run_id: None,
            current_task_started: None,
            current_advisories: Vec::new(),
            current_engine_lines: HashMap::new(),
            live_transient: LiveTransientAccumulator::default(),
            #[cfg(not(target_arch = "wasm32"))]
            live_segment: live_segment::LiveSegmentFollower::default(),
            last_live_publish: None,
            live_publish_due: false,
            stored_segments: HashMap::new(),
            pending_analyses: VecDeque::new(),
            successful_analysis_instances: HashSet::new(),
            execution_artifacts: HashMap::new(),
            point_families: crate::simulation::point_family::PointFamilyRegistry::default(),
            current_analysis_idx: 0,
            total_analyses: 0,
            cached_netlist: None,
            transient_post: transient_post::TransientPostCoordinator::default(),
            design_execution_epoch: 0,
            run_authorization: crate::simulation::execution::PreparedRunAuthorization::default(),
            retained_run_inputs: None,
            retained_run_generation: 0,
            preflight: None,
            #[cfg(any(test, feature = "test-support"))]
            preflight_test_hooks: engine_check::PreflightTestHooks::default(),
            deck_preparation: None,
            #[cfg(any(test, feature = "test-support"))]
            deck_preparation_hold: None,
            touchstone_export_policy: TouchstoneExportPolicy::disabled(),
            active_campaign: None,
        }
    }

    /// Process simulation state updates
    ///
    /// Call this once per frame in the app update loop.
    pub fn update(
        &mut self,
        state: &mut impl RunHost,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) {
        self.reset_if_design_replaced(state);
        self.report_unreadable_columns(state);

        // Handle simulation trigger
        if state.model().simulation.execution.trigger_simulation {
            log::info!(
                "Simulation triggered ({} analyses enabled)",
                state.model().sim_setup.enabled_analysis_instance_count()
            );
            state.model_mut().simulation.execution.trigger_simulation = false;
            self.start_simulation(state);
        }

        // Handle abort trigger
        if state.model().simulation.execution.trigger_abort {
            log::info!("Simulation abort triggered!");
            let model = state.model_mut();
            model.simulation.execution.trigger_abort = false;
            let requested_execution = model.simulation.execution.abort_request.take();
            let controller_execution = self.current_run_id.and_then(|run_sequence| {
                model
                    .simulation
                    .retained
                    .run_by_sequence(run_sequence)
                    .and_then(|run| run.execution_identity())
            });
            let bound_request = requested_execution.filter(|requested| {
                Some(*requested) == controller_execution
                    && Some(*requested) == model.simulation.execution.active_execution
            });
            if let Some(requested_execution) = bound_request {
                let cancellation_result = model.simulation
                    .retained.run_by_stable_id_mut(requested_execution.run_id)
                    .ok_or_else(|| {
                        "The active simulation run disappeared before cancellation could be acknowledged"
                            .to_owned()
                    })
                    .and_then(|run| run.mark_cancelling());
                if let Err(error) = cancellation_result {
                    self.run_line(
                        model,
                        LogSeverity::Error,
                        format!("Cancellation was refused: {error}"),
                    );
                } else {
                    self.runner.abort();
                    // Stop the batch from dispatching any additional task, but
                    // retain the active task metadata until the runner returns
                    // its abort acknowledgement. That acknowledgement is the
                    // authority for the terminal lifecycle and duration.
                    self.pending_analyses.clear();
                    if let Some(campaign) = self.active_campaign.as_mut() {
                        campaign.pending.clear();
                        campaign.cancelled = true;
                    }
                    model.simulation.execution.status = "Cancelling".to_owned();
                    self.run_line(model, LogSeverity::Info, "Cancellation requested");
                }
            } else {
                state.prompt("Nothing to cancel: the run this request named is no longer running.");
            }
        }

        // Poll for completion
        let model = state.model_mut();
        self.publish_engine_log(model);
        self.publish_live_transient_samples(model);
        self.publish_monte_carlo_checkpoint(model);
        self.publish_transient_checkpoint(model);
        self.poll_completion(state, export_io);

        // Apply/cancel background transient post-processing work after any
        // selection changes that happened during the previous frame.
        state.apply_run_event(RunEvent::CompletionsCollected);
        self.sync_transient_post_views(state);

        // Update running state
        let is_running = self.runner.is_running();
        let live_window = self
            .live_window()
            .filter(|_| is_running && self.streams_live());
        // A run asked to stop is stopping, whatever stage the engine last named.
        let stopping = state.model().simulation.cancellation_is_pending();
        let execution = &mut state.model_mut().simulation.execution;
        execution.progress =
            Self::ui_progress_fraction(self.runner.progress_fraction(), is_running);
        execution.reported_progress = self
            .runner
            .progress_fraction()
            .filter(|_| is_running)
            .map(|fraction| f64::from(fraction).clamp(0.0, 1.0));
        execution.phase = match (is_running, stopping) {
            (false, _) => None,
            (true, true) => Some("stopping".to_owned()),
            (true, false) => run_lines::phase_words(&self.runner.status()),
        };
        execution.live_window = live_window;
        if !is_running {
            execution.live_preview_note = None;
        }
        execution.is_running = is_running;
    }

    pub fn engine_availability(&self) -> super::status::EngineAvailability {
        self.runner.engine_availability()
    }

    /// The deck the next dispatch consumes, as preflight retained it, and the
    /// digest of the snapshot it seals: what a run started now executes.
    #[cfg(any(test, feature = "test-support"))]
    pub fn retained_run_deck(&self) -> Option<(&str, crate::product::ContentDigest)> {
        self.run_authorization
            .retained_snapshot()
            .map(|snapshot| (snapshot.executable_netlist(), snapshot.digest()))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_engine_wakeup(&mut self, wakeup: std::sync::Arc<dyn Fn() + Send + Sync>) {
        self.runner
            .set_engine_capability_observer(std::sync::Arc::new(|observation| {
                let Some(root) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.document_element())
                else {
                    return;
                };
                if let Some(observation) = observation {
                    let _ = root.set_attribute(
                        "data-rspice-wasm-jit-status",
                        if observation.available {
                            "qualified"
                        } else {
                            "rejected"
                        },
                    );
                    if let Some(result) = observation.solver_result {
                        let _ = root.set_attribute(
                            "data-rspice-wasm-jit-solver-result",
                            &result.to_string(),
                        );
                    } else {
                        let _ = root.remove_attribute("data-rspice-wasm-jit-solver-result");
                    }
                } else {
                    let _ = root.remove_attribute("data-rspice-wasm-jit-status");
                    let _ = root.remove_attribute("data-rspice-wasm-jit-solver-result");
                }
            }));
        self.runner.set_engine_wakeup(wakeup);
    }

    pub fn retry_engine_startup(&mut self) -> Result<(), Box<SimulationError>> {
        self.runner.retry_engine_startup().map_err(Box::new)
    }

    /// Start a new simulation batch
    ///
    /// Builds all enabled analyses into a queue and starts the first one.
    /// Subsequent analyses are started automatically upon completion.
    pub fn start_simulation(&mut self, state: &mut impl RunHost) {
        log::info!("start_simulation called");
        self.design_execution_epoch = state.model().design_execution_epoch;

        match state.model().simulation.execution.run_intent {
            SimulationRunIntent::SimulateRunSet => self.start_simulate_run_set(state),
            SimulationRunIntent::ManualDeck => self.start_manual_deck_simulation(state),
        }
    }

    fn start_authorized_snapshot(&mut self, state: &mut impl RunHost) {
        if self.has_active_batch() {
            state.prompt("A run is already in progress. Wait for it to finish, or cancel it.");
            return;
        }

        let (dispatch, prepared_deck) = match self.consume_snapshot_and_deck(state) {
            Ok(consumed) => consumed,
            Err(error) => {
                if state.model().simulation.execution.run_intent
                    == SimulationRunIntent::SimulateRunSet
                {
                    state.runs_mut().preflight.invalidate();
                }
                run_inputs::state_refused_dispatch(state, &error);
                return;
            }
        };
        if dispatch.intent() == SimulationRunIntent::SimulateRunSet {
            // The retained report describes a one-shot authorization. Once its
            // permit has been consumed, leaving that report reusable would
            // advertise authority the controller no longer owns.
            state.runs_mut().preflight.invalidate();
        }
        if let Err(error) = self.start_authorized_dispatch(state, dispatch, None, prepared_deck) {
            let model = state.model_mut();
            model.record(
                LogSeverity::Error,
                crate::diagnostics::LogSource::Run,
                format!("The run could not start: {error}"),
            );
            model.simulation.execution.status = "Run blocked".to_owned();
        }
    }

    /// Open a run for an authorized dispatch and start its first task.
    ///
    /// A campaign member names its membership and the plan it runs. A manual
    /// deck's run keeps the text of `prepared_deck`, the deck its snapshot
    /// was prepared from, as the text it ran, and is named by it when a Run
    /// press took it.
    fn start_authorized_dispatch(
        &mut self,
        state: &mut impl RunHost,
        mut dispatch: crate::simulation::execution::AuthorizedRunDispatch,
        campaign_member: Option<(crate::state::SimulationCampaignMembership, &str)>,
        prepared_deck: Option<run_inputs::PreparedDeck>,
    ) -> Result<(), crate::simulation::execution::PreparationError> {
        let (campaign_membership, member_plan) = campaign_member.unzip();
        if let Some(membership) = campaign_membership.as_ref() {
            membership.validate().map_err(|error| {
                crate::simulation::execution::PreparationError::new(
                    crate::simulation::execution::PreparationStage::Authorization,
                    error,
                )
            })?;
        }
        let source_domain = crate::simulation::execution::result_source_domain(dispatch.intent());
        let dispatched_plan_id = dispatch.simulation_plan_id();
        let dispatched_save_policy = dispatch.save_policy();
        let run_receipt = dispatch.prepared_run_receipt(source_domain)?;

        // A typed deck is named by its file, and its findings point at lines
        // of that file: the deck the press took, else the deck in front.
        let (manual_text, pressed_deck) =
            prepared_deck.map(|deck| (deck.text, deck.pressed)).unzip();
        let manual_deck = (dispatch.intent() == SimulationRunIntent::ManualDeck)
            .then(|| {
                pressed_deck.flatten().or_else(|| {
                    state
                        .manual_deck()
                        .map(crate::simulation::ManualDeckCopy::of)
                })
            })
            .flatten();
        // A run outside every plan is kept under the project's own limit, or
        // under the one Preferences sets when the project names none.
        let model = state.model_mut();
        model.simulation.retained.preferred_dataset_limit = Some(model.preferences.runs_kept());
        model.hold_document_datasets();
        // Allocate before changing controller queues, cross-probe mappings, or
        // manual-deck publication state: exhaustion must leave no partial batch.
        let run = model
            .simulation
            .start_prepared_run(run_receipt)
            .map_err(|error| {
                crate::simulation::execution::PreparationError::new(
                    crate::simulation::execution::PreparationStage::Authorization,
                    error,
                )
            })?;
        if let Some(membership) = campaign_membership.as_ref() {
            run.set_campaign_membership(membership.clone())
                .expect("campaign membership was validated before run creation");
        }
        let run_id = run.id;
        let execution_identity = run
            .execution_identity()
            .expect("current simulation runs always allocate job identity");

        self.pending_analyses.clear();
        self.successful_analysis_instances.clear();
        self.execution_artifacts.clear();
        self.current_source_domain = source_domain;
        self.current_deck_outputs = dispatch.deck_outputs().clone();
        self.current_save_policy = dispatched_save_policy;
        // One count of what the run's tasks write to the results store, held to its plan's
        // storage budget as they write it.
        self.runner
            .begin_storage_budget(dispatched_save_policy.maximum_storage_bytes());
        // One record of the files the run's analyses write, shared by all its
        // tasks, so a later analysis appends to what an earlier one wrote and
        // nothing appends to an earlier run's.
        self.runner.begin_hdl_run();
        self.total_analyses = dispatch.task_count();
        self.current_analysis_idx = 0;
        self.cached_netlist = Some(std::sync::Arc::clone(dispatch.shared_executable_netlist()));

        // The design a schematic run solves is the cell its cross-probe map
        // was read off; a typed deck is its file.
        let cross_probe = dispatch.take_cross_probe();
        let design = match &cross_probe {
            Some(cross_probe) => Some(cross_probe.source_reference.cell.clone()),
            None => manual_deck.as_ref().map(|deck| deck.name.clone()),
        };
        if dispatch.intent() == SimulationRunIntent::ManualDeck {
            model.simulation.source.manual_deck_path =
                manual_deck.map(crate::simulation::ManualDeckCopy::findings_path);
        }
        if let Some(cross_probe) = cross_probe
            && campaign_membership.is_none()
        {
            model.simulation.source.cross_probe.install(
                cross_probe.source_reference,
                cross_probe.sheets,
                cross_probe.topology_version,
                cross_probe.masters,
            );
        }
        let mut analyses: Vec<&str> = Vec::new();
        for task in dispatch.tasks() {
            let analysis = Self::analysis_name_for_spec(task.spec());
            if !analyses.contains(&analysis) {
                analyses.push(analysis);
            }
        }
        run_lines::announce_run_start(state, run_id, design.as_deref(), &analyses);
        if let (Some(membership), Some(plan)) = (campaign_membership.as_ref(), member_plan) {
            write_run_line(
                state.model_mut(),
                Some(run_id),
                LogSeverity::Info,
                format!(
                    "Campaign '{}', member {} of {}: plan '{plan}'",
                    membership.name(),
                    membership.member_index(),
                    membership.member_count()
                ),
            );
        }
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        for advisory in dispatch.advisories() {
            write_run_line(
                state.model_mut(),
                Some(run_id),
                LogSeverity::Warning,
                advisory.clone(),
            );
        }
        self.current_advisories = dispatch.advisories().to_vec();

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

        if let (Some(plan_id), Some(limit)) = (
            dispatched_plan_id,
            dispatched_save_policy.retained_dataset_limit(),
        ) {
            let model = state.model_mut();
            model.hold_document_datasets();
            model.simulation.prune_plan_runs(plan_id, limit);
        }
        // The new run supersedes the outcomes earlier runs reported, and the
        // pruning above may have retired a run whose failure still stood.
        state.model_mut().supersede_run_diagnostics(run_id);
        let facts = state.slot_facts();
        let model = state.model_mut();
        model.open_run_outcome(run_id, facts);
        self.current_run_id = Some(run_id);
        model.simulation.execution.active_execution = Some(execution_identity);
        model.simulation.execution.abort_request = None;
        // Every run's decks, whatever asked for it. The source one point
        // solved is the only artifact that settles what that point solved, and
        // it exists exactly once — here, between authorization and the queue
        // that consumes it. The manual baseline below is a different artifact
        // for a different question: it is the deck somebody *typed*, which the
        // working copy is diffed against.
        model
            .simulation
            .retained
            .executed_decks
            .retain(crate::state::ExecutedDeck {
                run_id,
                points: dispatch
                    .tasks()
                    .map(|task| {
                        // Read out of this very deck where the run was prepared.
                        crate::state::ExecutedDeckPoint::digested(
                            task.label(),
                            std::sync::Arc::clone(task.executable_netlist()),
                            task.sealed_model_sources().to_vec(),
                            task.executable_netlist_digest(),
                        )
                    })
                    .collect(),
            });
        if dispatch.intent() == SimulationRunIntent::ManualDeck {
            // The text the run read, shared with the deck it was prepared
            // from; copied from the snapshot only when none was kept.
            let manual_source = manual_text.unwrap_or_else(|| {
                crate::state::RevisedText::from(dispatch.manual_source().unwrap_or_default())
            });
            let manual_run = &mut state.runs_mut().manual_run;
            manual_run.pending_manual_run_id = Some(run_id);
            manual_run.pending_run_buffer = Some(manual_source);
        }

        self.pending_analyses = dispatch.into_tasks();
        self.start_next_analysis(state);
        Ok(())
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

    fn reset_if_design_replaced(&mut self, state: &mut impl RunHost) {
        let model = state.model_mut();
        if self.design_execution_epoch == model.design_execution_epoch {
            return;
        }

        let interrupted = self.current_run_id.filter(|run_sequence| {
            model
                .simulation
                .retained
                .run_by_sequence(*run_sequence)
                .is_some_and(|run| !run.lifecycle.is_terminal())
        });
        if let Some(run_sequence) = interrupted {
            // An interruption is a run failure like any other: the verdict
            // flips, a terminal lifecycle is sealed, and the generation moves
            // once, after both. The run's log says which happened: the
            // interruption, or why the run could not be closed.
            let errors = self.seal_failed_run(
                model,
                Some(run_sequence),
                None,
                Some(SimulationRunLifecycle::Interrupted),
            );
            if errors.is_empty() {
                write_run_line(
                    model,
                    Some(run_sequence),
                    LogSeverity::Error,
                    "Interrupted: the design it was running was closed or replaced",
                );
            } else {
                for error in errors {
                    write_run_line(
                        model,
                        Some(run_sequence),
                        LogSeverity::Error,
                        format!("The interrupted run could not be closed: {error}"),
                    );
                }
            }
            run_lines::announce_run_end(model, run_sequence);
        }
        self.reset_for_design_replacement();
        let execution = &mut state.model_mut().simulation.execution;
        execution.active_execution = None;
        execution.abort_request = None;
        execution.trigger_abort = false;
        let manual_run = &mut state.runs_mut().manual_run;
        manual_run.pending_manual_run_id = None;
        manual_run.pending_run_buffer = None;
        self.design_execution_epoch = state.model().design_execution_epoch;
    }

    fn reset_for_design_replacement(&mut self) {
        self.runner.reset_for_design_replacement();
        self.pending_analyses.clear();
        self.successful_analysis_instances.clear();
        self.execution_artifacts.clear();
        self.point_families.clear();
        self.cached_netlist = None;
        self.clear_prepared_run();
        self.cancel_preflight();
        self.cancel_deck_run_preparation();
        self.current_config = None;
        self.current_spec = None;
        self.current_analysis_label = None;
        self.current_spec_options = None;
        self.current_execution_limits = None;
        self.current_transient_checkpoint_error = None;
        self.current_periodic_carrier_hz = None;
        self.current_artifact_producer = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_op_effective_source_content_digest = None;
        self.current_saved_output_contracts.clear();
        self.current_save_policy =
            crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
        self.clear_live_preview();
        self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
        self.current_deck_outputs = Default::default();
        self.current_run_id = None;
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        self.current_analysis_idx = 0;
        self.total_analyses = 0;
        self.transient_post = transient_post::TransientPostCoordinator::default();
        self.active_campaign = None;
    }

    fn start_manual_deck_simulation(&mut self, state: &mut impl RunHost) {
        self.start_authorized_snapshot(state);
    }
    fn start_simulate_run_set(&mut self, state: &mut impl RunHost) {
        self.start_authorized_snapshot(state);
    }
    /// Start the next analysis in the queue
    ///
    /// Called after start_simulation() initializes the queue, and again
    /// after each analysis completes until the queue is empty.
    fn start_next_analysis(&mut self, state: &mut impl RunHost) {
        if !self.runner.can_accept_prepared_task() {
            log::error!(
                "Refusing to dequeue a prepared analysis while the runner still owns active or unpolled work"
            );
            return;
        }
        self.current_config = None;
        self.current_spec = None;
        self.current_analysis_label = None;
        self.current_spec_options = None;
        self.current_execution_limits = None;
        self.current_transient_checkpoint_error = None;
        self.current_periodic_carrier_hz = None;
        self.current_artifact_producer = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_op_effective_source_content_digest = None;
        self.current_saved_output_contracts.clear();
        self.clear_live_preview();
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
            // Named by the results the run retained for them, which a failed
            // or skipped prerequisite leaves too.
            let run = self.current_run_id.and_then(|run_sequence| {
                state
                    .model()
                    .simulation
                    .retained
                    .run_by_sequence(run_sequence)
            });
            let unavailable = unavailable_dependencies
                .iter()
                .map(|dependency| {
                    run.and_then(|run| run.find_analysis_by_source_instance(*dependency))
                        .map_or_else(
                            || "an earlier analysis".to_owned(),
                            |analysis| analysis.label.clone(),
                        )
                })
                .collect::<Vec<_>>()
                .join(", ");
            let message = format!(
                "Skipped {analysis_name}: it depends on {unavailable}, which did not complete"
            );
            let provenance = match AnalysisResultProvenance::new_with_authored_source_domain(
                self.current_source_domain,
                candidate.instance_id(),
                candidate.authored_instance_id(),
                candidate.source_revision(),
                candidate.snapshot_digest(),
                candidate.dependencies().to_vec(),
            )
            .and_then(|provenance| {
                provenance
                    .with_pvt_point(candidate.pvt_point().cloned())
                    .with_family_member(candidate.family_member().cloned())
            }) {
                Ok(provenance) => provenance,
                Err(error) => {
                    self.run_line(
                        state.model_mut(),
                        LogSeverity::Error,
                        format!("{analysis_name} could not be identified: {error}"),
                    );
                    // Nothing to retain — the provenance the result would have
                    // carried is what failed — but the verdict still flips,
                    // and that is a new generation of the run all the same.
                    let target_run_id = self.target_run_id();
                    let errors = self.seal_failed_run(state.model_mut(), target_run_id, None, None);
                    self.report_seal_errors(state.model_mut(), errors);
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
            retain_plan_saved_outputs(&mut failed, candidate.saved_output_contracts());
            let target_run_id = self.target_run_id();
            let errors = self.seal_failed_run(state.model_mut(), target_run_id, Some(failed), None);
            self.report_seal_errors(state.model_mut(), errors);
            self.run_line(state.model_mut(), LogSeverity::Warning, message);
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
        .and_then(|provenance| {
            provenance
                .with_pvt_point(next_analysis.pvt_point().cloned())
                .with_family_member(next_analysis.family_member().cloned())
        }) {
            Ok(provenance) => provenance,
            Err(error) => {
                self.run_line(
                    state.model_mut(),
                    LogSeverity::Error,
                    format!("{analysis_name} could not be identified: {error}"),
                );
                // Nothing to retain — the provenance the result would have
                // carried is what failed — but the verdict still flips, and
                // that is a new generation of the run all the same.
                let target_run_id = self.target_run_id();
                let errors = self.seal_failed_run(state.model_mut(), target_run_id, None, None);
                self.report_seal_errors(state.model_mut(), errors);
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
        self.current_analysis_label = Some(analysis_name.clone());
        self.current_spec_options = Some(next_analysis.spec_options().clone());
        self.current_execution_limits = Some(next_analysis.execution_limits());
        self.current_provenance = Some(provenance);
        self.current_config_digest = Some(next_analysis.config_digest());
        self.current_op_effective_source_content_digest = config.as_ref().and_then(|config| {
            let AnalysisConfig::DcOp(config) = config else {
                return None;
            };
            Some(
                rspice_simulation::execution_identity::operating_point_effective_source_digest_of(
                    next_analysis.executable_netlist_digest(),
                    config.run_point.clone(),
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
        state.model_mut().simulation.execution.status = status_msg.clone();

        self.run_line(
            state.model_mut(),
            LogSeverity::Info,
            if self.total_analyses > 1 {
                format!(
                    "Starting {analysis_name} (analysis {} of {})",
                    self.current_analysis_idx, self.total_analyses
                )
            } else {
                format!("Starting {analysis_name}")
            },
        );

        // Use cached netlist. If this is unexpectedly missing, fail gracefully
        // instead of panicking so the UI can recover.
        if self.cached_netlist.is_none() {
            let message = format!("{analysis_name} could not be started: the run has no netlist");
            self.run_line(state.model_mut(), LogSeverity::Error, message.clone());
            let target_run_id = self.target_run_id();
            let failed_analysis = self.current_provenance.take().map(|provenance| {
                AnalysisResult::failed(1, self.spec_to_analysis_type(&spec), analysis_name, message)
                    .with_provenance(provenance)
            });
            let failed_analysis = failed_analysis.map(|mut analysis| {
                self.materialize_current_saved_outputs(&mut analysis);
                analysis
            });
            let errors =
                self.seal_failed_run(state.model_mut(), target_run_id, failed_analysis, None);
            self.report_seal_errors(state.model_mut(), errors);
            self.pending_analyses.clear();
            self.finish_simulation_batch(state);
            state.model_mut().simulation.execution.status = "Error".to_string();
            return;
        }

        // Start the simulation
        let start_result = next_analysis
            .resolve_dependency_artifacts(&self.execution_artifacts)
            .map_err(|error| Box::new(SimulationError::InvalidConfig(error.to_string())))
            .and_then(|dispatch| {
                self.current_artifact_producer = Some(
                    dispatch
                        .artifact_producer()
                        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?,
                );
                // The carrier this task is about to be solved against, taken
                // from the resolved artifact rather than from the request: the
                // shooting solver moves an autonomous period off the authored
                // guess, so only the retained result says what the basis is.
                self.current_periodic_carrier_hz = dispatch
                    .dependencies()
                    .periodic_state()
                    .ok()
                    .map(|state| state.operating_point().analysis().result.frequency);
                let stream_transient_samples = self.streams_live()
                    || (self.current_save_policy.live_streaming_enabled()
                        && self.current_saved_output_contracts.iter().any(|contract| {
                            contract.policy()
                                == crate::state::SavedOutputPolicy::FailureDiagnosticsOnly
                        }));
                self.runner
                    .start_prepared(dispatch, stream_transient_samples)
                    .map_err(Box::new)
            });
        match start_result {
            Ok(()) => {
                self.current_task_started = Some(crate::time_compat::Instant::now());
                // The runner numbers each task's lines from zero again.
                self.current_engine_lines.clear();
                if let Some(run_id) = self.target_run_id()
                    && let Some(run) = state
                        .model_mut()
                        .simulation
                        .retained
                        .run_by_sequence_mut(run_id)
                    && let Err(error) = run.mark_running()
                {
                    self.run_line(
                        state.model_mut(),
                        LogSeverity::Error,
                        format!("The run could not be marked running: {error}"),
                    );
                }
                log::info!(
                    "Analysis {}/{} started successfully",
                    self.current_analysis_idx,
                    self.total_analyses
                );
            }
            Err(e) => {
                let error_message = format!("{analysis_name} could not be started: {e}");
                // Mark run as failed but continue with remaining analyses
                let target_run_id = self.target_run_id();
                // An analysis the engine would not start is a failure of the
                // run like any other, and stands with them; it reached only
                // the Console before.
                Self::report_failed_analysis(
                    state,
                    &analysis_name,
                    &error_message,
                    &None,
                    target_run_id,
                );
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
                let errors =
                    self.seal_failed_run(state.model_mut(), target_run_id, failed_analysis, None);
                self.report_seal_errors(state.model_mut(), errors);
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
        state: &mut impl RunHost,
        analysis_name: &str,
        analysis_type: AnalysisType,
        provenance: AnalysisResultProvenance,
    ) {
        let declaration = provenance.source_instance_id();
        let target_run_id = self.target_run_id();
        let model = state.model_mut();
        let assembled = target_run_id
            .and_then(|run_id| model.simulation.retained.run_by_sequence(run_id))
            .ok_or_else(|| "the analysis belongs to no run".to_owned())
            .and_then(|run| self.point_families.family_for(declaration, run));
        let mut analysis = match assembled {
            Ok(result) => self.convert_to_analysis_result_with_metadata_owned(
                result,
                analysis_type,
                analysis_name,
            ),
            Err(error) => {
                self.run_line(
                    model,
                    LogSeverity::Error,
                    format!("{analysis_name} failed: {error}"),
                );
                AnalysisResult::failed(1, analysis_type, analysis_name, error)
            }
        };
        self.materialize_current_saved_outputs(&mut analysis);

        // Retention already fails the run when the family did, so only a
        // retention error of its own needs answering here.
        if let Err(error) =
            self.retain_completed_analysis(model, target_run_id, analysis, provenance)
        {
            self.run_line(
                model,
                LogSeverity::Error,
                format!("{analysis_name}: the result was not kept: {error}"),
            );
            let errors = self.seal_failed_run(model, target_run_id, None, None);
            self.report_seal_errors(model, errors);
        }
        if let Some(run_id) = target_run_id {
            model.simulation.refresh_run_samples(run_id);
        }

        if self.pending_analyses.is_empty() {
            self.finish_simulation_batch(state);
        } else {
            self.start_next_analysis(state);
        }
    }

    fn target_run_id(&self) -> Option<u64> {
        // A completion belongs only to the run allocated for this dispatch.
        // Falling back to whichever result document happens to be newest can
        // corrupt history after selection changes or recovery-state edits.
        self.current_run_id
    }

    fn materialize_current_saved_outputs(&mut self, analysis: &mut AnalysisResult) {
        if let Some(provenance) = &self.current_provenance {
            self.point_families
                .capture_result(provenance.source_instance_id(), analysis);
        }
        let contracts = std::mem::take(&mut self.current_saved_output_contracts);
        crate::simulation::output_contract::apply_saved_output_policy(
            analysis,
            self.current_save_policy,
            &contracts,
        );
    }

    fn retain_completed_analysis(
        &mut self,
        model: &mut Model,
        target_run_id: Option<u64>,
        analysis: AnalysisResult,
        provenance: AnalysisResultProvenance,
    ) -> Result<bool, String> {
        let run_id = target_run_id.ok_or_else(|| "the analysis belongs to no run".to_owned())?;
        let completed_instance = provenance.source_instance_id();
        let succeeded = analysis.success;
        let analysis = analysis.with_provenance(provenance);
        let run = model
            .simulation
            .retained
            .run_by_sequence_mut(run_id)
            .ok_or_else(|| format!("run {run_id} is no longer in the result history"))?;
        self.retain_analysis_under_current_policy(run, analysis)?;
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

    /// Seal the aborted run, keeping whatever prefix the save policy admits.
    ///
    /// The run the reader may already be looking at is mutated three ways
    /// here: a partial analysis is appended to it, its verdict flips to
    /// failed, and its lifecycle is sealed as `Aborted`. That is one new
    /// generation of the retained evidence, which is exactly what
    /// [`Self::seal_failed_run`] is for.
    fn seal_aborted_run(&mut self, model: &mut Model, partial: Option<AnalysisResult>) {
        let errors = self.seal_failed_run(
            model,
            self.current_run_id,
            partial,
            Some(SimulationRunLifecycle::Aborted),
        );
        self.report_seal_errors(model, errors);
    }

    /// Seal a run's failure, as one event.
    ///
    /// Whatever prefix the save policy admits is retained, the run's verdict
    /// flips to failed, a terminal lifecycle is sealed when the caller names
    /// one, and only then does the dataset's generation move. Every Results
    /// memo over the run is keyed on that generation — the dataset digest the
    /// inspector's tamper check reads, the operating-point row plan, the
    /// retained-evidence verdict — so it has to be declared after the last
    /// mutation rather than between them, and ten paths that each performed
    /// some of these mutations declared it between none of them.
    ///
    /// The order inside is not arbitrary. Retention precedes the lifecycle
    /// seal because `finish_lifecycle` evaluates the run's specification
    /// verdicts against its retained analyses; a partial sealed the other way
    /// round would be judged against a run it is not part of.
    ///
    /// This is the only place a shipped path flips `success`, which
    /// [`tests::no_shipped_path_fails_a_run_outside_the_sealing_helper`]
    /// holds it to. Errors come back in the order they happened, for the
    /// caller to report the way it reports every other console error; a
    /// target run that no longer exists is not one of them, because a batch
    /// that is already unwinding has nothing to seal.
    fn seal_failed_run(
        &self,
        model: &mut Model,
        target_run_id: Option<u64>,
        failed: Option<AnalysisResult>,
        terminal: Option<SimulationRunLifecycle>,
    ) -> Vec<String> {
        let mut errors = Vec::new();
        let Some(run_id) = target_run_id else {
            return errors;
        };
        let Some(run) = model.simulation.retained.run_by_sequence_mut(run_id) else {
            return errors;
        };
        if let Some(failed) = failed
            && let Err(error) = self.retain_analysis_under_current_policy(run, failed)
        {
            errors.push(error);
        }
        // A checkpoint can be the only retained evidence when no transient
        // prefix exists, or when the full terminal result exceeded the budget.
        // Seal it as interrupted instead of leaving a stale running marker.
        for analysis in &mut run.analyses {
            if analysis.is_live_partial()
                && (analysis.monte_carlo_checkpoint.is_some()
                    || analysis.transient_checkpoint.is_some())
            {
                analysis.error_message = Some(
                    if terminal == Some(SimulationRunLifecycle::Aborted) {
                        "Simulation aborted by user"
                    } else if analysis.transient_checkpoint.is_some() {
                        "Transient ended before its result could be retained"
                    } else {
                        "Monte Carlo ended before its result could be retained"
                    }
                    .into(),
                );
            }
        }
        run.success = false;
        if let Some(terminal) = terminal
            && let Err(error) = run.finish_lifecycle(terminal)
        {
            errors.push(error);
        }
        model.simulation.view.data_version = model.simulation.view.data_version.wrapping_add(1);
        errors
    }

    /// Report every error [`Self::seal_failed_run`] returned in the run's log,
    /// the way it reports any other failure.
    fn report_seal_errors(&self, model: &mut Model, errors: Vec<String>) {
        for error in errors {
            self.run_line(model, LogSeverity::Error, error);
        }
    }

    /// Adopt one terminal result only when the complete retained run remains
    /// inside the ceiling authenticated by its prepared snapshot.
    fn retain_analysis_under_current_policy(
        &self,
        run: &mut crate::state::SimulationRun,
        mut analysis: AnalysisResult,
    ) -> Result<(), String> {
        // Terminal conversion must keep the last admitted journal, including
        // on abort/error. Match the complete prepared identity, not a label.
        self.preserve_live_checkpoints(run, &mut analysis);
        if analysis.success
            && analysis.analysis_type == AnalysisType::Transient
            && self.transient_checkpoint_capture_requested()
            && analysis.transient_checkpoint.is_none()
        {
            return Err("Transient completed without its requested retained checkpoint".into());
        }
        if let Some(checkpoint) = &analysis.transient_checkpoint {
            checkpoint.validate_for(&analysis.data)?;
        }
        if analysis.success
            && analysis.analysis_type == AnalysisType::MonteCarlo
            && self
                .current_spec_options
                .as_ref()
                .is_some_and(|options| options.mc_checkpoint.is_some())
            && analysis.monte_carlo_checkpoint.is_none()
        {
            return Err("Monte Carlo completed without its requested retained checkpoint".into());
        }
        if let Some(checkpoint) = &analysis.monte_carlo_checkpoint {
            checkpoint.validate_for((&analysis).into())?;
        }
        self.validate_analysis_retention(run, &analysis)?;
        run.replace_live_or_add_analysis(analysis);
        Ok(())
    }

    fn preserve_live_checkpoints(
        &self,
        run: &crate::state::SimulationRun,
        analysis: &mut AnalysisResult,
    ) {
        if let Some(provenance) = analysis.provenance()
            && let Some(live) =
                run.find_analysis_by_source_instance(provenance.source_instance_id())
            && live.is_live_partial()
            && live.provenance() == Some(provenance)
        {
            if analysis.monte_carlo_checkpoint.is_none() {
                analysis.monte_carlo_checkpoint = live.monte_carlo_checkpoint.clone();
            }
            if analysis.transient_checkpoint.is_none() {
                analysis.transient_checkpoint = live.transient_checkpoint.clone();
            }
        }
    }

    fn validate_analysis_retention(
        &self,
        run: &crate::state::SimulationRun,
        analysis: &AnalysisResult,
    ) -> Result<(), String> {
        if let Some(maximum) = self.current_save_policy.maximum_storage_bytes() {
            let replacing_instance = analysis
                .provenance()
                .map(AnalysisResultProvenance::source_instance_id);
            let already_retained = run
                .analyses
                .iter()
                .filter(|existing| {
                    !(existing.is_live_partial()
                        && replacing_instance.is_some_and(|replacing_instance| {
                            existing.provenance().is_some_and(|existing_provenance| {
                                existing_provenance.source_instance_id() == replacing_instance
                            })
                        }))
                })
                .fold(0_u64, |total, existing| {
                    total.saturating_add(existing.retained_memory_bytes())
                });
            // What the results store holds is held to the budget as it is written, by the
            // bytes the store wrote; only what the run keeps in memory is counted here.
            let requested = already_retained.saturating_add(analysis.retained_memory_bytes());
            if requested > maximum {
                return Err(format!(
                    "the run's results would take {}, more than its {} storage limit",
                    rspice_simulation_contract::run_set::format_bytes(requested),
                    rspice_simulation_contract::run_set::format_bytes(maximum),
                ));
            }
        }
        Ok(())
    }

    /// Write what the engine logged since the previous frame into the run's
    /// own log.
    ///
    /// Drained beside the live transient samples because it is the same kind of
    /// thing: evidence the running analysis published that the UI thread owns
    /// presenting. The run's log rather than the session's: a verbose solve
    /// must not push the session's history out of the Console, and the run
    /// keeps its account of itself after the session ends.
    fn publish_engine_log(&mut self, model: &mut Model) {
        let lines = self.runner.drain_engine_log();
        if lines.is_empty() {
            return;
        }
        if let Some(run) = self
            .current_run_id
            .and_then(|run_sequence| model.simulation.retained.run_by_sequence_mut(run_sequence))
        {
            let kept = crate::diagnostics::engine_log::publish(
                lines,
                &mut run.log,
                crate::time_compat::checked_unix_time_ms().unwrap_or_default(),
                &self.current_advisories,
            );
            self.current_engine_lines.extend(kept);
        }
    }

    /// Carry a finished task's withheld and suppressed counts into the run's
    /// log, once per task, before the next dispatch clears them.
    fn absorb_engine_log_tally(&mut self, model: &mut Model) {
        let tally = self.runner.engine_log_tally();
        if let Some(run) = self
            .current_run_id
            .and_then(|run_sequence| model.simulation.retained.run_by_sequence_mut(run_sequence))
        {
            run.log
                .absorb_engine_seal(tally.withheld, tally.suppressed_warnings);
        }
    }

    /// The live plot is drawn again at most this often: each drawing builds the preview's
    /// waveforms again, and more often than this the eye cannot follow.
    const LIVE_PUBLISH_EVERY: std::time::Duration = std::time::Duration::from_millis(100);

    fn publish_live_transient_samples(&mut self, model: &mut Model) {
        let deltas = self.runner.drain_transient_samples();
        #[cfg(not(target_arch = "wasm32"))]
        let grew = self.live_segment.poll(
            self.runner.live_segment(),
            self.previews_every_column()
                .then_some(&self.current_deck_outputs),
        );
        #[cfg(target_arch = "wasm32")]
        let grew = false;
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(error) = self.live_segment.take_failure() {
            self.stop_live_preview(model, &error);
        }
        if !deltas.is_empty() || grew {
            if let Some(window) = self.live_window() {
                self.live_transient.set_span(window);
            }
            self.live_transient.ingest(deltas);
            self.live_publish_due = true;
        }
        if !self.live_publish_due {
            return;
        }
        if !self.streams_live() {
            return;
        }
        if self
            .last_live_publish
            .is_some_and(|at| at.elapsed() < Self::LIVE_PUBLISH_EVERY)
        {
            return;
        }
        self.last_live_publish = Some(crate::time_compat::Instant::now());
        self.live_publish_due = false;
        let held_back = self.publish_live_preview(model);
        let mut note = self.live_transient.gaps().note();
        if held_back {
            let words = "Differences and expressions are drawn when the run ends.";
            note = Some(note.map_or_else(|| words.to_owned(), |note| format!("{note} {words}")));
        }
        model.simulation.execution.live_preview_note = note;
    }

    /// Whether the running task's samples are drawn while it runs: the outputs its plan
    /// streams, or every column of a manual deck, unless live delivery is turned off.
    fn streams_live(&self) -> bool {
        self.current_save_policy.live_streaming_enabled()
            && (self.previews_every_column()
                || self.current_saved_output_contracts.iter().any(|contract| {
                    contract.streaming()
                        == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
                }))
    }

    /// A manual deck's task, which no plan names outputs for: its result is every column the
    /// engine writes, and so is its live preview. Either plots only what the deck names.
    fn previews_every_column(&self) -> bool {
        self.current_source_domain == AnalysisResultSourceDomain::ManualDeck
            && self.current_saved_output_contracts.is_empty()
    }

    /// Plot, of a manual deck's waveforms, those the deck names and no other.
    pub(super) fn plot_deck_outputs(&self, waveforms: &mut [RetainedTrace]) {
        for waveform in waveforms {
            waveform.visible = self.current_deck_outputs.plots(&waveform.name);
        }
    }

    /// Forget the running task's live samples and the rows followed for it.
    fn clear_live_preview(&mut self) {
        self.live_transient.clear();
        #[cfg(not(target_arch = "wasm32"))]
        self.live_segment.clear();
        self.last_live_publish = None;
        self.live_publish_due = false;
    }

    /// The span of time the running transient fills, from where its result starts to its stop
    /// time; `None` for any other task.
    fn live_window(&self) -> Option<(f64, f64)> {
        match (&self.current_spec, &self.current_config) {
            (
                Some(AnalysisSpec::Transient {
                    start_time,
                    stop_time,
                    ..
                }),
                _,
            ) => Some((*start_time, *stop_time)),
            (_, Some(AnalysisConfig::Transient(config))) => {
                Some((config.start_time, config.stop_time))
            }
            _ => None,
        }
        .filter(|(start, stop)| start.is_finite() && stop > start)
    }

    /// The samples the live plot draws from. On the desktop, a run that writes its segment as
    /// it goes is drawn from the rows the segment holds, read through the store as a finished
    /// run's are; any other run from the history taken from the live queue.
    fn live_source(&self, analysis_type: AnalysisType, label: &str) -> AnalysisResult {
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut source = self.live_transient.source_analysis(analysis_type, label);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(rows) = self.live_segment.rows() {
            // A trace stores no colour, as a finished run's do: the palette colours it by its
            // slot, so the live plot's colours are the ones the finished result keeps.
            source.waveforms = rows
                .waveforms
                .iter()
                .cloned()
                .map(|data| RetainedTrace {
                    data,
                    color: String::new(),
                    visible: true,
                })
                .collect();
        }
        source
    }

    /// Show the samples accepted so far as the running analysis's live
    /// result.
    ///
    /// A preview that cannot be shown stops for the rest of the task, and its
    /// reason is written once: this runs every frame, and the solve goes on
    /// regardless, its result replacing the preview when it completes.
    /// Whether outputs the plan streams were held back to the run's end: on the desktop, a
    /// difference or an expression reads its columns whole, so it is not drawn from a segment
    /// that is still being written.
    fn publish_live_preview(&mut self, model: &mut Model) -> bool {
        if self.live_transient.preview_stopped() {
            return false;
        }
        if self.live_transient.take_event_loss_notice() {
            self.run_line(
                model,
                LogSeverity::Warning,
                "Live event preview stopped because some event data could not be retained or delivered. The preview ends at its last complete point. Final results use the solver's retained event history.",
            );
        }
        let Some(provenance) = self.current_provenance.clone() else {
            self.stop_live_preview(model, "the running analysis could not be identified");
            return false;
        };
        let analysis_type = self
            .current_spec
            .as_ref()
            .map(|spec| self.spec_to_analysis_type(spec))
            .or_else(|| {
                self.current_config
                    .as_ref()
                    .map(|config| self.config_to_analysis_type(config))
            })
            .unwrap_or(AnalysisType::Transient);
        let label = self
            .current_analysis_label
            .as_deref()
            .or_else(|| self.current_spec.as_ref().map(Self::analysis_name_for_spec))
            .or_else(|| {
                self.current_config
                    .as_ref()
                    .map(|config| self.analysis_name(config))
            })
            .unwrap_or("Transient")
            .to_owned();
        #[cfg(not(target_arch = "wasm32"))]
        let from_segment = self.live_segment.rows().is_some();
        #[cfg(target_arch = "wasm32")]
        let from_segment = false;
        let (contracts, held_back) = if from_segment {
            let drawn = self
                .current_saved_output_contracts
                .iter()
                .filter(|contract| reads_one_column(contract))
                .cloned()
                .collect::<Vec<_>>();
            let held_back = self.current_saved_output_contracts.iter().any(|contract| {
                contract.streaming()
                    == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
                    && !reads_one_column(contract)
            });
            (std::borrow::Cow::Owned(drawn), held_back)
        } else {
            (
                std::borrow::Cow::Borrowed(self.current_saved_output_contracts.as_slice()),
                false,
            )
        };
        let mut shown = None;
        let waveforms = if self.previews_every_column() && from_segment {
            // Every column of a segment is built with each opening, off the frame, and the
            // frame only swaps it in: between openings there is nothing new to draw.
            let Some((traces, built)) = self.take_built_preview() else {
                return held_back;
            };
            shown = Some(built);
            traces
        } else if self.previews_every_column() {
            // In the order the finished result lists them, by name, so its panes and colours
            // stay where they are when the run ends.
            let mut every = std::mem::take(&mut self.live_source(analysis_type, &label).waveforms);
            every.sort_by(|left, right| left.name.cmp(&right.name));
            self.plot_deck_outputs(&mut every);
            every
        } else {
            materialize_live_saved_outputs(&self.live_source(analysis_type, &label), &contracts)
        };
        let events = self.live_transient.event_payload(analysis_type);
        if waveforms.is_empty() && events.is_none() {
            self.release(shown);
            return held_back;
        }
        let partial = AnalysisResult::live_transient_partial(1, analysis_type, &label)
            .with_waveforms(waveforms)
            .with_provenance(provenance);
        let partial = match events {
            Some(payload) => partial.with_result_payload(payload),
            None => partial,
        };
        let Some(run_id) = self.target_run_id() else {
            self.stop_live_preview(model, "the running analysis belongs to no run");
            self.release((partial, shown));
            return held_back;
        };
        if let Err(error) = self.retain_live_transient_analysis(model, run_id, partial, shown) {
            self.stop_live_preview(model, &error);
        }
        held_back
    }

    /// A manual deck's preview as the newest opening of its segment built it, once: its
    /// traces, and the view's copy of them.
    #[cfg(not(target_arch = "wasm32"))]
    fn take_built_preview(&mut self) -> Option<(Vec<RetainedTrace>, ShownWaveforms)> {
        self.live_segment
            .take_preview()
            .map(|preview| (preview.traces, preview.shown))
    }

    /// A browser run writes no segment, so nothing is built for it.
    #[cfg(target_arch = "wasm32")]
    fn take_built_preview(&mut self) -> Option<(Vec<RetainedTrace>, ShownWaveforms)> {
        None
    }

    /// Let go of `value` where its size costs no frame: on the desktop, on a thread of its own.
    fn release(&mut self, value: impl std::any::Any + Send) {
        #[cfg(not(target_arch = "wasm32"))]
        self.live_segment.release(value);
        #[cfg(target_arch = "wasm32")]
        drop(value);
    }

    /// Stop the live preview for the rest of the task, saying why in the
    /// run's log, or in the session's when there is no run to say it in.
    fn stop_live_preview(&mut self, model: &mut Model, reason: &str) {
        self.live_transient.stop_preview();
        self.run_line(
            model,
            LogSeverity::Warning,
            format!("Live preview stopped: {reason}"),
        );
    }

    /// Retain `partial` as the run's live result and redraw the run if it is shown: from
    /// `shown`, the view's copy of its waveforms, when one was built off the frame. What it
    /// replaces is let go of off the frame.
    fn retain_live_transient_analysis(
        &mut self,
        model: &mut Model,
        run_id: u64,
        mut partial: AnalysisResult,
        shown: Option<ShownWaveforms>,
    ) -> Result<(), String> {
        let (index, replaced) = model
            .simulation
            .retained
            .run_by_sequence_mut(run_id)
            .ok_or_else(|| format!("run {run_id} is no longer in the result history"))
            .and_then(|run| {
                self.preserve_live_checkpoints(run, &mut partial);
                self.validate_analysis_retention(run, &partial)?;
                run.upsert_live_analysis(partial)
            })?;
        self.release(replaced);
        match shown {
            Some(shown) => {
                let previous = model
                    .simulation
                    .refresh_run_samples_with(run_id, index, shown);
                self.release(previous);
            }
            None => {
                model.simulation.refresh_run_samples(run_id);
            }
        }
        Ok(())
    }

    /// Record the failure twice, for the two readers it has.
    ///
    /// The run's log gets what happened, in the engine's words. The
    /// diagnostic store gets the condition: this design does not simulate,
    /// which stands until the inputs it was measured against move, tied to
    /// the objects the engine named for it so the Problems page can go to
    /// them. A log line is a record of the event, not a place to jump from.
    pub fn report_failed_analysis(
        state: &mut impl RunHost,
        analysis: &str,
        message: &str,
        attribution: &Option<crate::state::ConvergenceAttribution>,
        run_sequence: Option<u64>,
    ) {
        write_run_line(state.model_mut(), run_sequence, LogSeverity::Error, message);
        if let Some(run_sequence) = run_sequence {
            let facts = state.slot_facts();
            state.model_mut().publish_run_failure(
                analysis,
                message,
                attribution.as_ref(),
                run_sequence,
                facts,
            );
        }
    }

    fn partial_failure_analysis(
        &self,
        analysis_type: AnalysisType,
        label: &str,
        error: impl Into<String>,
        provenance: AnalysisResultProvenance,
    ) -> AnalysisResult {
        #[cfg(not(target_arch = "wasm32"))]
        let rows = self.live_segment.rows();
        #[cfg(target_arch = "wasm32")]
        let rows = None::<&rspice_results::waveform::RetainedWaveform>;
        let mut analysis = if (self.live_transient.is_empty() && rows.is_none())
            || !self.current_save_policy.retain_failure_diagnostics()
        {
            AnalysisResult::failed(1, analysis_type, label, error.into())
        } else {
            #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
            let mut analysis = self.live_transient.source_analysis(analysis_type, label);
            // The rows a run that wrote its segment had written when it stopped, thinned as
            // the live history of any other run is: the segment of a run that failed is not kept.
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(rows) = rows
                && analysis.waveforms.is_empty()
            {
                let mut history = LiveTransientAccumulator::default();
                if let Some(window) = self.live_window() {
                    history.set_span(window);
                }
                if history.ingest_rows(&rows.waveforms).is_ok() {
                    analysis.waveforms = std::mem::take(
                        &mut history.source_analysis(analysis_type, label).waveforms,
                    );
                }
            }
            analysis.error_message = Some(error.into());
            analysis
        };
        analysis.provenance = Some(provenance);
        analysis
    }

    /// Finish the simulation batch and clean up state
    fn finish_simulation_batch(&mut self, state: &mut impl RunHost) {
        let completed_run_id = self.target_run_id();
        self.point_families.clear();
        let model = state.model_mut();
        let completed_run =
            completed_run_id.and_then(|run_id| model.simulation.retained.run_by_sequence(run_id));
        let run_success = completed_run.is_some_and(|run| run.success);
        // Whether the run's results storage stopped it, and whether its plan's budget did.
        let storage_stop = completed_run
            .and_then(|run| run.results_storage_stop())
            .map(|stop| stop.budget.is_some());
        self.clear_live_preview();

        if let Some(run_id) = completed_run_id
            && let Some(run) = model.simulation.retained.run_by_sequence_mut(run_id)
        {
            let terminal = if run_success {
                SimulationRunLifecycle::Completed
            } else {
                SimulationRunLifecycle::Failed
            };
            if let Err(error) = run.finish_lifecycle(terminal) {
                write_run_line(
                    model,
                    Some(run_id),
                    LogSeverity::Error,
                    format!("The run could not be closed: {error}"),
                );
            }
        }

        // Complete the run (syncs waveforms and selects first analysis)
        if let Some(run_id) = completed_run_id {
            if model.simulation.select_run_by_sequence(run_id) {
                model.simulation.complete_run();
                run_lines::announce_run_end(model, run_id);
            } else {
                model.record(
                    LogSeverity::Error,
                    crate::diagnostics::LogSource::Run,
                    format!(
                        "Run {run_id} left the result history before it finished; no other run's results were changed"
                    ),
                );
            }
        } else {
            model.record(
                LogSeverity::Error,
                crate::diagnostics::LogSource::Run,
                "A run finished without the run it belonged to; no run's results were changed",
            );
        }
        Self::promote_manual_deck_baseline(state, run_success, completed_run_id);

        // Clear cached netlist
        self.cached_netlist = None;
        self.successful_analysis_instances.clear();
        self.execution_artifacts.clear();
        self.current_config = None;
        self.current_spec = None;
        self.current_analysis_label = None;
        self.current_spec_options = None;
        self.current_execution_limits = None;
        self.current_transient_checkpoint_error = None;
        self.current_periodic_carrier_hz = None;
        self.current_artifact_producer = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_op_effective_source_content_digest = None;
        self.current_saved_output_contracts.clear();
        self.current_save_policy =
            crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
        self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
        self.current_deck_outputs = Default::default();
        self.current_run_id = None;
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        self.current_analysis_idx = 0;
        self.total_analyses = 0;
        let execution = &mut state.model_mut().simulation.execution;
        execution.active_execution = None;
        execution.abort_request = None;

        execution.status = match storage_stop {
            Some(true) => "Stopped: storage budget reached".to_string(),
            Some(false) => "Stopped: results storage full".to_string(),
            None if run_success => "Complete".to_string(),
            None => "Completed with errors".to_string(),
        };

        self.complete_campaign_member(state, run_success);

        log::info!("Simulation batch completed");
    }

    fn promote_manual_deck_baseline(
        state: &mut impl RunHost,
        run_success: bool,
        completed_run_id: Option<u64>,
    ) {
        let manual_run = &mut state.runs_mut().manual_run;
        let pending_matches = manual_run.pending_manual_run_id.is_some()
            && manual_run.pending_manual_run_id == completed_run_id;
        if !pending_matches {
            return;
        }

        let pending_buffer = manual_run.pending_run_buffer.take();
        manual_run.pending_manual_run_id = None;

        if run_success && let Some(buffer) = pending_buffer {
            let current_buffer = state.model().simulation.source.netlist_content.clone();
            let param_values = Self::manual_deck_param_values(&buffer);
            let manual_run = &mut state.runs_mut().manual_run;
            manual_run.last_run_buffer = Some(buffer);
            // The deck and the run it belongs to are sealed together; nothing
            // downstream may pair a buffer with a run it did not execute.
            manual_run.last_run_id = completed_run_id;
            manual_run.last_run_params = param_values;
            if let Some(baseline) = manual_run.last_run_buffer.as_deref() {
                manual_run.edited_lines =
                    Self::changed_lines_against_baseline(&current_buffer, baseline);
            }
        }
    }

    /// The lines of `current` that differ from `baseline`, the deck a run
    /// executed, by a line diff: inserting a line marks that line, not every
    /// line after it. Where lines were deleted, the line now standing in
    /// their place is marked, so a deletion still shows.
    ///
    /// An edit reaches a stretch of the deck. The whole lines before it and
    /// after it, the same in both, are passed over as bytes, and only the
    /// lines between are diffed, numbered from where they start. Among
    /// repeated lines that leaves the mark on the line that was edited,
    /// where a diff of every line may mark one of its twins.
    pub fn changed_lines_against_baseline(current: &str, baseline: &str) -> HashSet<usize> {
        let (head, tail) = same_lines_around(current.as_bytes(), baseline.as_bytes());
        let head_lines = line_count(&current[..head]);
        let current_lines = current[head..current.len() - tail]
            .lines()
            .collect::<Vec<_>>();
        let baseline_lines = baseline[head..baseline.len() - tail]
            .lines()
            .collect::<Vec<_>>();
        let last =
            (head_lines + current_lines.len() + line_count(&current[current.len() - tail..]))
                .checked_sub(1);
        let mut changed = HashSet::new();
        for op in
            similar::capture_diff_slices(similar::Algorithm::Myers, &baseline_lines, &current_lines)
        {
            match op {
                similar::DiffOp::Equal { .. } => {}
                similar::DiffOp::Delete { new_index, .. } => {
                    changed.extend(last.map(|last| (head_lines + new_index).min(last)));
                }
                similar::DiffOp::Insert {
                    new_index, new_len, ..
                }
                | similar::DiffOp::Replace {
                    new_index, new_len, ..
                } => changed.extend(head_lines + new_index..head_lines + new_index + new_len),
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

    fn analysis_name_for_spec(spec: &AnalysisSpec) -> &'static str {
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

    /// The result family a finished task of this specification belongs to.
    ///
    /// Derived from the canonical tag protocol rather than restated here: the
    /// receipt authenticates a retained result by comparing its family against
    /// `PreparedRunTaskReceipt::result_analysis_type`, so a second opinion
    /// about the same specification would reject correct runs.
    fn spec_to_analysis_type(&self, spec: &AnalysisSpec) -> AnalysisType {
        canonical_analysis_kind(spec).result_analysis_type()
    }

    /// Observe runner status and drain final checkpoint evidence before applying completion.
    fn poll_completion(
        &mut self,
        state: &mut impl RunHost,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) {
        // Update status display with multi-analysis progress
        let status = self.runner.status();
        let model = state.model_mut();
        let cancellation_pending = model
            .simulation
            .execution
            .active_execution
            .and_then(|identity| model.simulation.retained.run_by_stable_id(identity.run_id))
            .is_some_and(|run| run.lifecycle == SimulationRunLifecycle::Cancelling);
        if cancellation_pending {
            model.simulation.execution.status = "Cancelling".to_owned();
        } else if !matches!(status, SimulationStatus::Idle)
            && !matches!(status, SimulationStatus::Completed { .. })
        {
            // Show progress-aware status
            let phase =
                run_lines::phase_words(&status).unwrap_or_else(|| status.display_name().to_owned());
            if self.total_analyses > 1 {
                model.simulation.execution.status = format!(
                    "Analysis {}/{}: {phase}",
                    self.current_analysis_idx, self.total_analyses,
                );
            } else {
                model.simulation.execution.status = phase;
            }
        }

        // Check for completion
        if let Some(result) = self.runner.poll_result() {
            // Polling may observe completion after the frame's first stream
            // drain. Adopt its final journal and its last engine lines before
            // taking provenance or dispatching the next task (which clears the
            // runner queue): the task's sink has dropped by now, so the row
            // closing a bounded log is in the queue too.
            self.publish_monte_carlo_checkpoint(model);
            self.publish_transient_checkpoint(model);
            self.publish_engine_log(model);
            self.absorb_engine_log_tally(model);
            let result = match self.current_transient_checkpoint_error.take() {
                Some(error) => Err(SimulationError::InvalidConfig(error)),
                None => result,
            };
            self.accept_completion(state, export_io, result);
        }
    }

    /// Check if a simulation is currently running
    #[cfg(any(test, feature = "test-support"))]
    pub fn is_running(&self) -> bool {
        self.runner.is_running()
    }

    fn ui_progress_fraction(progress: Option<f32>, is_running: bool) -> f64 {
        progress
            .map(|value| f64::from(value).clamp(0.0, 1.0))
            .unwrap_or_else(|| if is_running { 0.08 } else { 0.0 })
    }

    /// Abort current simulation
    #[cfg(any(test, feature = "test-support"))]
    pub fn abort(&self) {
        self.runner.abort();
    }
}

// Visible to the crate, because the shell's feature tests build from its
// fixtures.
#[cfg(test)]
pub(crate) mod tests;

/// Whether a saved output draws one column of the run as it is: a node's voltage to ground, or
/// one branch's or device's current. Only such an output is drawn from a segment the run is still
/// writing; a difference or an expression reads its columns whole.
fn reads_one_column(contract: &PreparedSavedOutput) -> bool {
    if contract.kind() != crate::state::SavedOutputKind::RawVoltageOrCurrent {
        return false;
    }
    let expression = contract.source_expression().trim();
    if rspice_results::saved_output::device_current_probe(expression).is_some() {
        return true;
    }
    let Some((function, rest)) = expression.split_once('(') else {
        return false;
    };
    let Some(argument) = rest.trim_end().strip_suffix(')') else {
        return false;
    };
    let argument = argument.trim();
    (function.trim().eq_ignore_ascii_case("V") || function.trim().eq_ignore_ascii_case("I"))
        && !argument.is_empty()
        && !argument.contains(',')
        && argument != "0"
}

/// How many bytes of whole lines `current` and `baseline` share at their
/// start and at their end: each a run of whole lines in both, the two never
/// overlapping. Compared a block at a time, at memory speed.
fn same_lines_around(current: &[u8], baseline: &[u8]) -> (usize, usize) {
    const BLOCK: usize = 4096;
    let shorter = current.len().min(baseline.len());
    let mut same = 0;
    while same + BLOCK <= shorter && current[same..same + BLOCK] == baseline[same..same + BLOCK] {
        same += BLOCK;
    }
    same += current[same..shorter]
        .iter()
        .zip(&baseline[same..shorter])
        .take_while(|(current, baseline)| current == baseline)
        .count();
    // Back to the start of the line the bytes part in, unless both end there.
    let head = if same == current.len() && same == baseline.len() {
        same
    } else {
        current[..same]
            .iter()
            .rposition(|&byte| byte == b'\n')
            .map_or(0, |at| at + 1)
    };
    let room = shorter - head;
    let mut same = 0;
    while same + BLOCK <= room
        && current[current.len() - same - BLOCK..current.len() - same]
            == baseline[baseline.len() - same - BLOCK..baseline.len() - same]
    {
        same += BLOCK;
    }
    same += current[current.len() - room..current.len() - same]
        .iter()
        .rev()
        .zip(
            baseline[baseline.len() - room..baseline.len() - same]
                .iter()
                .rev(),
        )
        .take_while(|(current, baseline)| current == baseline)
        .count();
    // On to the start of a line in both: just after a line break the two
    // share, or where the part after the head starts.
    let starts_line = |text: &[u8], tail: usize| {
        let at = text.len() - tail;
        at == head || text[at - 1] == b'\n'
    };
    let mut tail = same;
    while tail > 0 && !(starts_line(current, tail) && starts_line(baseline, tail)) {
        let shared = &current[current.len() - tail..];
        tail = shared
            .iter()
            .position(|&byte| byte == b'\n')
            .map_or(0, |at| tail - at - 1);
    }
    (head, tail)
}

/// How many lines `text` holds, as `str::lines` counts them.
fn line_count(text: &str) -> usize {
    let breaks = text.bytes().filter(|&byte| byte == b'\n').count();
    breaks + usize::from(!text.is_empty() && !text.ends_with('\n'))
}
