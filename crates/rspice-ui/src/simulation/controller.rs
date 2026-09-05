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
use crate::simulation::execution::{
    ExecutionArtifactEnvelope, TouchstoneExportPolicy, canonical_analysis_kind,
};
use crate::simulation::multi_run::{
    AnalysisSpec, FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal,
    OptimizationVariable, PssMethod, SpPort,
};
use crate::simulation::output_contract::{
    PreparedSavedOutput, materialize_live_saved_outputs,
    materialize_saved_outputs_preserving_engine, retain_plan_saved_outputs,
};
use crate::simulation::plan::AnalysisNumericOverride;
use crate::simulation::runner::SpecExecutionOptions;
use crate::simulation::runner::{
    SimulationError, TransientDigitalEventSample, TransientRealEventSample, TransientSampleDelta,
};
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, ComplexResultValue, DcOpResult,
    DigitalEventPointEvidence, DigitalEventTraceEvidence, MonteCarloVariableMetadata,
    OperatingPointValue, PeriodicNoiseOutputQuantity, RealEventPointEvidence,
    RealEventTraceEvidence, ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence,
    ReliabilityShiftEvidence, ReliabilityStressEvidence, SensitivityResultMode,
    SensitivityResultRow, SimulationRunIntent, SimulationRunLifecycle, SoaEvaluationEvidence,
    SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence, WaveformData,
};
use crate::workbench::app_state::{ActiveViewer, AppState, SpecializedViewerCacheProvenance};
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;

mod analysis_commands;
mod analysis_helpers;
mod analysis_plan;
mod analysis_run_config;
mod analysis_spec_build;
#[cfg(test)]
mod directive_parse_ratchet;
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
pub(crate) use transient_post::{DerivedViewerLoadState, build_eye_from_waveform};

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

#[derive(Clone)]
struct PreparedCampaignMember {
    plan_name: String,
    snapshot: crate::simulation::execution::PreparedRunSnapshot,
}

struct ActiveSimulationCampaign {
    id: crate::product::SimulationCampaignId,
    name: String,
    member_count: u32,
    dispatched_count: u32,
    completed_count: u32,
    failed_count: u32,
    cancelled: bool,
    pending: VecDeque<PreparedCampaignMember>,
}

/// Reviewed forecast returned after a campaign has been frozen and its first
/// member has been submitted to the local execution queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SimulationCampaignDispatchReceipt {
    pub campaign_id: crate::product::SimulationCampaignId,
    pub member_count: usize,
    pub task_count: usize,
}

#[derive(Debug, Default)]
struct LiveTransientAccumulator {
    waveforms: Vec<LiveTransientWaveform>,
    digital_events: Vec<DigitalEventTraceEvidence>,
    real_events: Vec<RealEventTraceEvidence>,
    retained_event_points: usize,
}

#[derive(Debug)]
struct LiveTransientWaveform {
    name: String,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl LiveTransientAccumulator {
    const MAX_SOURCE_SAMPLES: usize = 8_192;
    const COMPACTED_SOURCE_SAMPLES: usize = crate::state::DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES;
    /// Live event points retained across all nodes while a run is in flight.
    ///
    /// An event history is exact — every point is a committed transition, so
    /// there is nothing in it a decimation could drop without lying about
    /// when a net changed. The provisional history therefore stops growing at
    /// this ceiling rather than being thinned, and the terminal result, which
    /// carries the whole schedule, replaces it at completion.
    const MAX_LIVE_EVENT_POINTS: usize = 8_192;

    fn clear(&mut self) {
        self.waveforms.clear();
        self.digital_events.clear();
        self.real_events.clear();
        self.retained_event_points = 0;
    }

    fn is_empty(&self) -> bool {
        self.waveforms.is_empty() && !self.has_events()
    }

    fn has_events(&self) -> bool {
        !self.digital_events.is_empty() || !self.real_events.is_empty()
    }

    fn ingest(&mut self, deltas: Vec<TransientSampleDelta>) {
        for delta in deltas {
            if !delta.time.is_finite() {
                continue;
            }
            let mut samples = HashMap::with_capacity(delta.waveforms.len());
            let mut malformed = false;
            for sample in delta.waveforms {
                if !sample.value.is_finite() {
                    malformed = true;
                    break;
                }
                if samples.insert(sample.name, sample.value).is_some() {
                    malformed = true;
                    break;
                }
            }
            if malformed {
                continue;
            }
            // Events are per-node timelines, not columns of the shared analog
            // grid, so they are kept whenever the message itself is sound.
            // The alignment rule below governs only the grid.
            self.ingest_events(delta.time, delta.events, delta.real_events);
            if samples.is_empty() {
                continue;
            }

            if self.waveforms.is_empty() {
                let mut samples = samples.into_iter().collect::<Vec<_>>();
                samples.sort_by(|left, right| left.0.cmp(&right.0));
                self.waveforms = samples
                    .into_iter()
                    .map(|(name, value)| LiveTransientWaveform {
                        name,
                        x: vec![delta.time],
                        y: vec![value],
                    })
                    .collect();
                continue;
            }
            if samples.len() != self.waveforms.len()
                || self.waveforms.iter().any(|waveform| {
                    !samples.contains_key(&waveform.name)
                        || waveform
                            .x
                            .last()
                            .is_some_and(|previous| *previous >= delta.time)
                })
            {
                // A live point is one aligned solver sample. Publishing a
                // partial or schema-changing delta would make differential
                // expressions combine different times, so reject the whole
                // provisional point. The terminal result remains authoritative.
                continue;
            }
            for waveform in &mut self.waveforms {
                waveform.x.push(delta.time);
                waveform.y.push(samples[&waveform.name]);
            }
        }
        self.compact_if_needed();
    }

    /// Fold one accepted point's changed event values into the provisional
    /// per-node histories.
    ///
    /// The history is change-compressed and strictly increasing in time, the
    /// same shape the engine records into a terminal result: a repeated value
    /// or a time that does not advance is dropped for that node alone, so one
    /// stale message cannot corrupt the nodes beside it.
    fn ingest_events(
        &mut self,
        time: f64,
        digital: Vec<TransientDigitalEventSample>,
        real: Vec<TransientRealEventSample>,
    ) {
        for event in digital {
            if self.retained_event_points >= Self::MAX_LIVE_EVENT_POINTS {
                return;
            }
            if event.name.trim().is_empty()
                || event.value_code > crate::state::MAX_DIGITAL_EVENT_CODE
            {
                continue;
            }
            let index = match self
                .digital_events
                .iter()
                .position(|trace| trace.node_name == event.name)
            {
                Some(index) => index,
                None => {
                    self.digital_events.push(DigitalEventTraceEvidence {
                        node_name: event.name,
                        points: Vec::new(),
                    });
                    self.digital_events.len() - 1
                }
            };
            let points = &mut self.digital_events[index].points;
            if points
                .last()
                .is_some_and(|last| last.time_s >= time || last.value_code == event.value_code)
            {
                continue;
            }
            points.push(DigitalEventPointEvidence {
                time_s: time,
                value_code: event.value_code,
            });
            self.retained_event_points += 1;
        }
        for event in real {
            if self.retained_event_points >= Self::MAX_LIVE_EVENT_POINTS {
                return;
            }
            if event.name.trim().is_empty() || !event.value.is_finite() {
                continue;
            }
            let index = match self
                .real_events
                .iter()
                .position(|trace| trace.node_name == event.name)
            {
                Some(index) => index,
                None => {
                    self.real_events.push(RealEventTraceEvidence {
                        node_name: event.name,
                        points: Vec::new(),
                    });
                    self.real_events.len() - 1
                }
            };
            let points = &mut self.real_events[index].points;
            if points
                .last()
                .is_some_and(|last| last.time_s >= time || last.value == event.value)
            {
                continue;
            }
            points.push(RealEventPointEvidence {
                time_s: time,
                value: event.value,
            });
            self.retained_event_points += 1;
        }
    }

    /// The provisional event schedule as retained evidence, ordered by node
    /// name so the result digest is a function of the history alone.
    ///
    /// A history the validator would reject is offered as nothing at all,
    /// exactly as the terminal conversion does: the Events sheet must never
    /// have to decide whether its own evidence is usable.
    fn event_payload(&self, analysis_type: AnalysisType) -> Option<AnalysisResultPayload> {
        if !self.has_events() {
            return None;
        }
        let mut digital_traces = self.digital_events.clone();
        digital_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
        let mut real_traces = self.real_events.clone();
        real_traces.sort_by(|left, right| left.node_name.cmp(&right.node_name));
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
        };
        payload
            .validate_for(analysis_type)
            .is_ok()
            .then_some(payload)
    }

    /// Bound the source arrays used to build the provisional live document.
    ///
    /// All traces retain one common time selection, so differential and
    /// derived expressions remain aligned. Each displayed trace contributes
    /// bucket extrema to that shared selection; deterministic evenly spaced
    /// points fill any spare capacity. The terminal solver result is untouched
    /// and replaces this provisional cache at completion.
    fn compact_if_needed(&mut self) {
        let Some(point_count) = self.waveforms.iter().map(|waveform| waveform.x.len()).min() else {
            return;
        };
        if point_count <= Self::MAX_SOURCE_SAMPLES || point_count <= 2 {
            return;
        }

        let target = Self::COMPACTED_SOURCE_SAMPLES.min(point_count).max(2);
        let maximum_extrema_series = ((target - 2) / 2).max(1);
        let extrema_series = self.waveforms.len().min(maximum_extrema_series).max(1);
        let bucket_count = ((target - 2) / (2 * extrema_series)).max(1);
        let interior = point_count - 2;
        let mut selected = std::collections::BTreeSet::new();
        selected.insert(0usize);
        selected.insert(point_count - 1);

        for bucket in 0..bucket_count {
            let start = 1 + bucket * interior / bucket_count;
            let end = 1 + (bucket + 1) * interior / bucket_count;
            if start >= end {
                continue;
            }
            for waveform in self.waveforms.iter().take(extrema_series) {
                let mut minimum = start;
                let mut maximum = start;
                for index in start + 1..end {
                    if waveform.y[index].total_cmp(&waveform.y[minimum]).is_lt() {
                        minimum = index;
                    }
                    if waveform.y[index].total_cmp(&waveform.y[maximum]).is_gt() {
                        maximum = index;
                    }
                }
                selected.insert(minimum);
                selected.insert(maximum);
            }
        }
        for slot in 1..target - 1 {
            if selected.len() >= target {
                break;
            }
            selected.insert(slot * (point_count - 1) / (target - 1));
        }
        let selected = selected.into_iter().take(target).collect::<Vec<_>>();
        for waveform in &mut self.waveforms {
            waveform.x = selected.iter().map(|index| waveform.x[*index]).collect();
            waveform.y = selected.iter().map(|index| waveform.y[*index]).collect();
        }
    }

    fn source_analysis(&self, analysis_type: AnalysisType, label: &str) -> AnalysisResult {
        let waveforms = self
            .waveforms
            .iter()
            .enumerate()
            .map(|(index, waveform)| {
                WaveformData::new(
                    waveform.name.clone(),
                    waveform.x.clone(),
                    waveform.y.clone(),
                    SimulationController::color_for_index(index),
                )
            })
            .collect();
        let analysis = AnalysisResult::live_transient_partial(1, analysis_type, label)
            .with_waveforms(waveforms);
        match self.event_payload(analysis_type) {
            Some(payload) => analysis.with_result_payload(payload),
            None => analysis,
        }
    }
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
    /// Immutable prepared-task label, including its exact Run Set point.
    current_analysis_label: Option<String>,
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
    /// Save/streaming policy authenticated by the active prepared snapshot.
    current_save_policy: crate::simulation::execution::SavePolicy,
    /// Source domain authenticated by the active run dispatch. Manual-deck
    /// task IDs are deterministic source projections, not plan-owned IDs.
    current_source_domain: AnalysisResultSourceDomain,
    /// Stable run ID that owns the in-flight batch.
    current_run_id: Option<u64>,
    /// Exact accepted samples observed for the current transient task. These
    /// back both adaptive live presentation and a truthful partial result if
    /// the task terminates before returning a full engine result.
    live_transient: LiveTransientAccumulator,

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
    /// Frozen, declared-order multi-plan campaign. Each member is an
    /// independent prepared run; this record only schedules and rolls them up.
    active_campaign: Option<ActiveSimulationCampaign>,
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
            current_analysis_label: None,
            current_spec_options: None,
            current_provenance: None,
            current_config_digest: None,
            current_effective_source_content_digest: None,
            current_op_effective_source_content_digest: None,
            current_saved_output_contracts: Vec::new(),
            current_save_policy:
                crate::simulation::execution::SavePolicy::RetainEngineProducedResults,
            current_source_domain: AnalysisResultSourceDomain::SimulationPlan,
            current_run_id: None,
            live_transient: LiveTransientAccumulator::default(),
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
            active_campaign: None,
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
                    if let Some(campaign) = self.active_campaign.as_mut() {
                        campaign.pending.clear();
                        campaign.cancelled = true;
                    }
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
        self.publish_live_transient_samples(state);
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

        let dispatch = match self.consume_snapshot_for_dispatch(state) {
            Ok(dispatch) => dispatch,
            Err(error) => {
                if state.simulation.run_intent == SimulationRunIntent::SimulateRunSet {
                    state.workbench.preflight.invalidate();
                }
                state.push_sim_message(ConsoleMessage::warning(error.to_string()));
                state.simulation.status = "Run blocked".to_owned();
                return;
            }
        };
        if dispatch.intent() == SimulationRunIntent::SimulateRunSet {
            // The retained report describes a one-shot authorization. Once its
            // permit has been consumed, leaving that report reusable would
            // advertise authority the controller no longer owns.
            state.workbench.preflight.invalidate();
        }
        if let Err(error) = self.start_authorized_dispatch(state, dispatch, None) {
            state.push_sim_message(ConsoleMessage::error(error.to_string()));
            state.simulation.status = "Run blocked".to_owned();
        }
    }

    fn start_authorized_dispatch(
        &mut self,
        state: &mut AppState,
        mut dispatch: crate::simulation::execution::AuthorizedRunDispatch,
        campaign_membership: Option<crate::state::SimulationCampaignMembership>,
    ) -> Result<(), crate::simulation::execution::PreparationError> {
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

        self.pending_analyses.clear();
        self.successful_analysis_instances.clear();
        self.execution_artifacts.clear();
        self.current_source_domain = source_domain;
        self.current_save_policy = dispatched_save_policy;
        self.total_analyses = dispatch.task_count();
        self.current_analysis_idx = 0;
        self.cached_netlist = Some(dispatch.executable_netlist().to_owned());

        if let Some(cross_probe) = dispatch.take_cross_probe()
            && campaign_membership.is_none()
        {
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
        if let Some(membership) = campaign_membership {
            run.set_campaign_membership(membership)
                .expect("campaign membership was validated before run creation");
        }
        let run_id = run.id;
        let execution_identity = run
            .execution_identity()
            .expect("current simulation runs always allocate job identity");
        if let (Some(plan_id), Some(limit)) = (
            dispatched_plan_id,
            dispatched_save_policy.retained_dataset_limit(),
        ) {
            state.simulation.prune_plan_runs(plan_id, limit);
        }
        self.current_run_id = Some(run_id);
        state.simulation.active_execution = Some(execution_identity);
        state.simulation.abort_request = None;
        // Every run's decks, whatever asked for it. The source one point
        // solved is the only artifact that settles what that point solved, and
        // it exists exactly once — here, between authorization and the queue
        // that consumes it. The manual baseline below is a different artifact
        // for a different question: it is the deck somebody *typed*, which the
        // working copy is diffed against.
        state
            .simulation
            .executed_decks
            .retain(crate::state::ExecutedDeck {
                run_id,
                points: dispatch
                    .tasks()
                    .map(|task| {
                        let deck = std::sync::Arc::clone(task.executable_netlist());
                        crate::state::ExecutedDeckPoint {
                            model_sources: crate::state::sealed_model_sources(&deck),
                            label: task.label().to_owned(),
                            deck,
                        }
                    })
                    .collect(),
            });
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

        let interrupted = self.current_run_id.filter(|run_sequence| {
            state
                .simulation
                .run_by_sequence(*run_sequence)
                .is_some_and(|run| !run.lifecycle.is_terminal())
        });
        if let Some(run_sequence) = interrupted {
            // An interruption is a run failure like any other: the verdict
            // flips, a terminal lifecycle is sealed, and the generation moves
            // once, after both. The reader is told only when the seal took —
            // a lifecycle this run could not enter is an internal fault, not
            // news about their simulation.
            let errors = self.seal_failed_run(
                state,
                Some(run_sequence),
                None,
                Some(SimulationRunLifecycle::Interrupted),
            );
            if errors.is_empty() {
                state.push_sim_message(ConsoleMessage::warning(
                    "Simulation execution was interrupted because its design context changed"
                        .to_owned(),
                ));
            } else {
                for error in errors {
                    log::error!("Failed to seal interrupted simulation run lifecycle: {error}");
                }
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
        self.current_analysis_label = None;
        self.current_spec_options = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_effective_source_content_digest = None;
        self.current_op_effective_source_content_digest = None;
        self.current_saved_output_contracts.clear();
        self.current_save_policy =
            crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
        self.live_transient.clear();
        self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
        self.current_run_id = None;
        self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
        self.current_analysis_idx = 0;
        self.total_analyses = 0;
        self.transient_post = transient_post::TransientPostCoordinator::default();
        self.active_campaign = None;
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
        self.current_analysis_label = None;
        self.current_spec_options = None;
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_effective_source_content_digest = None;
        self.current_op_effective_source_content_digest = None;
        self.current_saved_output_contracts.clear();
        self.live_transient.clear();
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
                    // Nothing to retain — the provenance the result would have
                    // carried is what failed — but the verdict still flips,
                    // and that is a new generation of the run all the same.
                    let target_run_id = self.target_run_id(state);
                    let errors = self.seal_failed_run(state, target_run_id, None, None);
                    Self::report_seal_errors(state, errors);
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
            let target_run_id = self.target_run_id(state);
            let errors = self.seal_failed_run(state, target_run_id, Some(failed), None);
            Self::report_seal_errors(state, errors);
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
                // Nothing to retain — the provenance the result would have
                // carried is what failed — but the verdict still flips, and
                // that is a new generation of the run all the same.
                let target_run_id = self.target_run_id(state);
                let errors = self.seal_failed_run(state, target_run_id, None, None);
                Self::report_seal_errors(state, errors);
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
            let errors = self.seal_failed_run(state, target_run_id, failed_analysis, None);
            Self::report_seal_errors(state, errors);
            self.pending_analyses.clear();
            self.finish_simulation_batch(state);
            state.simulation.status = "Error".to_string();
            return;
        }

        // Start the simulation
        let start_result = next_analysis
            .resolve_dependency_artifacts(&self.execution_artifacts)
            .map_err(|error| SimulationError::InvalidConfig(error.to_string()))
            .and_then(|dispatch| {
                let stream_transient_samples = self.current_save_policy.live_streaming_enabled()
                    && self.current_saved_output_contracts.iter().any(|contract| {
                        contract.streaming()
                            == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
                            || contract.policy()
                                == crate::state::SavedOutputPolicy::FailureDiagnosticsOnly
                    });
                self.runner
                    .start_prepared(dispatch, stream_transient_samples)
            });
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
                let errors = self.seal_failed_run(state, target_run_id, failed_analysis, None);
                Self::report_seal_errors(state, errors);
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
            let errors = self.seal_failed_run(state, target_run_id, None, None);
            Self::report_seal_errors(state, errors);
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

    fn target_run_id(&self, _state: &AppState) -> Option<u64> {
        // A completion belongs only to the run allocated for this dispatch.
        // Falling back to whichever result document happens to be newest can
        // corrupt history after selection changes or recovery-state edits.
        self.current_run_id
    }

    fn materialize_current_saved_outputs(&mut self, analysis: &mut AnalysisResult) {
        let contracts = std::mem::take(&mut self.current_saved_output_contracts);
        if matches!(
            self.current_save_policy,
            crate::simulation::execution::SavePolicy::PlanOwned { .. }
        ) {
            if self.current_save_policy.output_selection_mode()
                == crate::state::OutputSelectionMode::SaveAll
            {
                // Save All is a retention override, not a command to open
                // every engine quantity in the plot. Authored outputs/probes
                // still own initial display intent; everything else starts in
                // the data browser and remains available on demand.
                for waveform in &mut analysis.waveforms {
                    waveform.visible = false;
                }
                materialize_saved_outputs_preserving_engine(analysis, &contracts);
            } else {
                retain_plan_saved_outputs(analysis, &contracts);
            }
        }
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
        let analysis = analysis.with_provenance(provenance);
        let run = state
            .simulation
            .run_by_sequence_mut(run_id)
            .ok_or_else(|| format!("completed analysis target run {run_id} does not exist"))?;
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
    fn seal_aborted_run(&mut self, state: &mut AppState, partial: Option<AnalysisResult>) {
        let errors = self.seal_failed_run(
            state,
            self.current_run_id,
            partial,
            Some(SimulationRunLifecycle::Aborted),
        );
        Self::report_seal_errors(state, errors);
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
        state: &mut AppState,
        target_run_id: Option<u64>,
        failed: Option<AnalysisResult>,
        terminal: Option<SimulationRunLifecycle>,
    ) -> Vec<String> {
        let mut errors = Vec::new();
        let Some(run_id) = target_run_id else {
            return errors;
        };
        let Some(run) = state.simulation.run_by_sequence_mut(run_id) else {
            return errors;
        };
        if let Some(failed) = failed
            && let Err(error) = self.retain_analysis_under_current_policy(run, failed)
        {
            errors.push(error);
        }
        run.success = false;
        if let Some(terminal) = terminal
            && let Err(error) = run.finish_lifecycle(terminal)
        {
            errors.push(error);
        }
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);
        errors
    }

    /// Report every error [`Self::seal_failed_run`] returned, the way the
    /// console reports any other failure.
    fn report_seal_errors(state: &mut AppState, errors: Vec<String>) {
        for error in errors {
            log::error!("{error}");
            state.push_sim_message(ConsoleMessage::error(error));
        }
    }

    /// Adopt one terminal result only when the complete retained run remains
    /// inside the ceiling authenticated by its prepared snapshot.
    fn retain_analysis_under_current_policy(
        &self,
        run: &mut crate::state::SimulationRun,
        analysis: AnalysisResult,
    ) -> Result<(), String> {
        self.validate_analysis_retention(run, &analysis)?;
        run.replace_live_or_add_analysis(analysis);
        Ok(())
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
                    total.saturating_add(existing.retained_storage_bytes())
                });
            let requested = already_retained.saturating_add(analysis.retained_storage_bytes());
            if requested > maximum {
                return Err(format!(
                    "Run {} retention requires {}, exceeding the authenticated {} storage ceiling",
                    run.id,
                    crate::simulation::run_set::format_bytes(requested),
                    crate::simulation::run_set::format_bytes(maximum),
                ));
            }
        }
        Ok(())
    }

    fn publish_live_transient_samples(&mut self, state: &mut AppState) {
        let deltas = self.runner.drain_transient_samples();
        if deltas.is_empty() {
            return;
        }
        self.live_transient.ingest(deltas);
        if !self.current_saved_output_contracts.iter().any(|contract| {
            contract.streaming()
                == crate::state::SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
        }) {
            return;
        }
        if !self.current_save_policy.live_streaming_enabled() {
            return;
        }
        let Some(provenance) = self.current_provenance.clone() else {
            log::error!("Accepted transient samples have no prepared-task provenance");
            return;
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
            .or_else(|| {
                self.current_spec
                    .as_ref()
                    .map(|spec| self.analysis_name_for_spec(spec))
            })
            .or_else(|| {
                self.current_config
                    .as_ref()
                    .map(|config| self.analysis_name(config))
            })
            .unwrap_or("Transient");
        let source = self.live_transient.source_analysis(analysis_type, label);
        let waveforms =
            materialize_live_saved_outputs(&source, &self.current_saved_output_contracts);
        let events = self.live_transient.event_payload(analysis_type);
        if waveforms.is_empty() && events.is_none() {
            return;
        }
        let partial = AnalysisResult::live_transient_partial(1, analysis_type, label)
            .with_waveforms(waveforms)
            .with_provenance(provenance);
        let partial = match events {
            Some(payload) => partial.with_result_payload(payload),
            None => partial,
        };
        let Some(run_id) = self.target_run_id(state) else {
            log::error!("Accepted transient samples have no target simulation run");
            return;
        };
        let retained = state
            .simulation
            .run_by_sequence_mut(run_id)
            .ok_or_else(|| format!("live transient target run {run_id} does not exist"))
            .and_then(|run| {
                self.validate_analysis_retention(run, &partial)?;
                run.upsert_live_analysis(partial)
            });
        if let Err(error) = retained {
            log::error!("Could not publish live transient samples: {error}");
            return;
        }
        state
            .simulation
            .select_latest_analysis_in_run_sequence(run_id);
    }

    /// Write the failure to the console, anchored to the objects the engine
    /// named for it.
    ///
    /// The message is unchanged whether or not the engine could attribute
    /// the failure — the anchor is an addition, not a rewrite. A failure
    /// that named nothing (a parse error names no conductor) writes exactly
    /// the row it always did.
    fn report_failed_analysis(
        state: &mut AppState,
        message: &str,
        attribution: &Option<crate::state::ConvergenceAttribution>,
    ) {
        let anchor = attribution.as_ref().and_then(|attribution| {
            let nets: Vec<String> = attribution.nets().map(str::to_owned).collect();
            let devices: Vec<String> = attribution.devices().map(str::to_owned).collect();
            (!nets.is_empty() || !devices.is_empty())
                .then_some(crate::diagnostics::LogAnchor::Simulation { nets, devices })
        });
        state.log_buffer.log_anchored(
            crate::diagnostics::LogSeverity::Error,
            crate::diagnostics::LogSource::Simulation,
            message,
            None,
            anchor,
        );
        if let Some(attribution) = attribution {
            // A second row, not a longer first one: the failure's own prose
            // is the engine's and stays whole, and what the engine named is
            // a separate statement an author can act on.
            state.log_buffer.log_anchored(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Engine,
                attribution.summary(),
                None,
                None,
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
        let mut analysis = if self.live_transient.is_empty()
            || !self.current_save_policy.retain_failure_diagnostics()
        {
            AnalysisResult::failed(1, analysis_type, label, error.into())
        } else {
            let mut analysis = self.live_transient.source_analysis(analysis_type, label);
            analysis.error_message = Some(error.into());
            analysis
        };
        analysis.provenance = Some(provenance);
        analysis
    }

    /// Finish the simulation batch and clean up state
    fn finish_simulation_batch(&mut self, state: &mut AppState) {
        let completed_analysis_count = self.total_analyses;
        let completed_run_id = self.target_run_id(state);
        self.point_families.clear();
        let run_success = completed_run_id
            .and_then(|run_id| state.simulation.run_by_sequence(run_id))
            .map(|run| run.success)
            .unwrap_or(false);
        self.live_transient.clear();

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
            if state.simulation.select_run_by_sequence(run_id) {
                state.simulation.complete_run();
            } else {
                let message = format!(
                    "Simulation batch target run {run_id} disappeared; no existing result dataset was modified"
                );
                log::error!("{message}");
                state.push_sim_message(ConsoleMessage::error(message));
            }
        } else {
            let message = "Simulation batch lost its exact target run; no existing result dataset was modified";
            log::error!("{message}");
            state.push_sim_message(ConsoleMessage::error(message.to_owned()));
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
        self.current_provenance = None;
        self.current_config_digest = None;
        self.current_effective_source_content_digest = None;
        self.current_op_effective_source_content_digest = None;
        self.current_saved_output_contracts.clear();
        self.current_save_policy =
            crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
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
        // Anchored to the run rather than left as prose. This line and the
        // session notice the shell raises describe the same completion, and
        // the anchor is what lets the two recognise each other instead of
        // both being kept as separate activity.
        state.push_sim_message_anchored(
            summary,
            completed_run_id
                .map(|run_sequence| crate::diagnostics::LogAnchor::ResultRun { run_sequence }),
        );

        self.complete_campaign_member(state, run_success);

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
            // The deck and the run it belongs to are sealed together; nothing
            // downstream may pair a buffer with a run it did not execute.
            state.ui.netlist.last_run_id = completed_run_id;
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

    /// The result family a finished task of this specification belongs to.
    ///
    /// Derived from the canonical tag protocol rather than restated here: the
    /// receipt authenticates a retained result by comparing its family against
    /// `PreparedRunTaskReceipt::result_analysis_type`, so a second opinion
    /// about the same specification would reject correct runs.
    fn spec_to_analysis_type(&self, spec: &AnalysisSpec) -> AnalysisType {
        canonical_analysis_kind(spec).result_analysis_type()
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
                        .current_analysis_label
                        .clone()
                        .or_else(|| {
                            self.current_spec
                                .as_ref()
                                .map(|spec| self.analysis_name_for_spec(spec).to_owned())
                        })
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|config| self.analysis_name(config).to_owned())
                        })
                        .unwrap_or_else(|| "Analysis".to_owned());

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
                                            | AnalysisSpec::Psp { .. }
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
                    let hb_artifact_required = self
                        .current_provenance
                        .as_ref()
                        .map(|provenance| provenance.source_instance_id())
                        .is_some_and(|producer| {
                            self.pending_analyses.iter().any(|task| {
                                task.dependencies().contains(&producer)
                                    && matches!(
                                        task.spec(),
                                        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. }
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
                            Some(hb_spec @ AnalysisSpec::HarmonicBalance { .. }),
                            Some(provenance),
                            Some(config_digest),
                        ) if hb_artifact_required => ExecutionArtifactEnvelope::from_hb_result(
                            provenance.prepared_snapshot_digest(),
                            provenance.source_instance_id(),
                            provenance.source_revision(),
                            config_digest,
                            hb_spec,
                            &sim_result,
                        )
                        .map_err(|error| {
                            format!(
                                "HB result could not produce its authenticated spectral-state artifact: {error}"
                            )
                        }),
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

                    // Prepare external and derived evidence while the raw
                    // solver result is available, but publish neither until
                    // the immutable analysis has crossed the exact run's
                    // retention boundary successfully.
                    let mut prepared_touchstone_export =
                        target_run_id.and_then(|run_id| {
                            match self.prepare_touchstone_export(&sim_result, run_id) {
                                Ok(prepared) => prepared,
                                Err(error) => {
                                    state.push_sim_message(ConsoleMessage::warning(format!(
                                        "Touchstone export skipped: {error}"
                                    )));
                                    None
                                }
                            }
                        });
                    let mut prepared_yield_evidence = self
                        .yield_manager
                        .analyze_monte_carlo(&sim_result)
                        .map(|yield_results| {
                            let provenance = target_run_id
                                .and_then(|run_sequence| {
                                    state.simulation.run_by_sequence(run_sequence)
                                })
                                .and_then(|run| {
                                    YieldAnalysisProvenance::from_monte_carlo_result(
                                        run.run_id,
                                        run.dataset_id,
                                        &sim_result,
                                    )
                                });
                            (yield_results.values().cloned().collect(), provenance)
                        });

                    self.apply_result_side_effects(state, &sim_result);
                    if let crate::simulation::SimulationResult::Transient { convergence, .. } =
                        &sim_result
                    {
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

                    let mut analysis_result = if let Some(config) = &self.current_config {
                        self.convert_to_analysis_result_owned(sim_result, config)
                    } else {
                        self.convert_to_analysis_result_with_metadata_owned(
                            sim_result,
                            analysis_type,
                            &current_label,
                        )
                    };
                    self.retain_periodic_noise_result_metadata(&mut analysis_result);
                    self.retain_sparameter_result_metadata(&mut analysis_result);
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
                    if analysis_result.analysis_type == AnalysisType::Transient {
                        self.populate_transient_post_views(state, &analysis_result);
                    }
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
                                if let Some(prepared) = prepared_touchstone_export.take() {
                                    Self::commit_touchstone_export(state, export_io, prepared);
                                }
                                if let Some((yield_results, provenance)) =
                                    prepared_yield_evidence.take()
                                {
                                    state
                                        .simulation
                                        .replace_yield_evidence(yield_results, provenance);
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
                                let errors = self.seal_failed_run(state, target_run_id, None, None);
                                Self::report_seal_errors(state, errors);
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
                        let errors = self.seal_failed_run(state, target_run_id, None, None);
                        Self::report_seal_errors(state, errors);
                    }

                    // Display the just-completed analysis without rebuilding waveform buffers.
                    if let Some(run_id) = target_run_id {
                        state
                            .simulation
                            .select_latest_analysis_in_run_sequence(run_id);
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
                    log::info!("Analysis aborted; retaining any accepted transient prefix");
                    let partial = if self.live_transient.is_empty()
                        || !self.current_save_policy.retain_failure_diagnostics()
                    {
                        None
                    } else {
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
                            .clone()
                            .or_else(|| {
                                self.current_spec
                                    .as_ref()
                                    .map(|spec| self.analysis_name_for_spec(spec).to_owned())
                            })
                            .or_else(|| {
                                self.current_config
                                    .as_ref()
                                    .map(|config| self.analysis_name(config).to_owned())
                            })
                            .unwrap_or_else(|| "Transient".to_owned());
                        self.current_provenance.take().map(|provenance| {
                            self.partial_failure_analysis(
                                analysis_type,
                                &label,
                                "Simulation aborted by user",
                                provenance,
                            )
                        })
                    };
                    let mut partial = partial.map(|mut analysis| {
                        self.materialize_current_saved_outputs(&mut analysis);
                        analysis
                    });
                    self.seal_aborted_run(state, partial.take());
                    self.pending_analyses.clear();
                    self.successful_analysis_instances.clear();
                    self.execution_artifacts.clear();
                    self.point_families.clear();
                    self.cached_netlist = None;
                    self.current_config = None;
                    self.current_spec = None;
                    self.current_analysis_label = None;
                    self.current_spec_options = None;
                    self.current_provenance = None;
                    self.current_config_digest = None;
                    self.current_effective_source_content_digest = None;
                    self.current_op_effective_source_content_digest = None;
                    self.current_saved_output_contracts.clear();
                    self.current_save_policy =
                        crate::simulation::execution::SavePolicy::RetainEngineProducedResults;
                    self.live_transient.clear();
                    self.current_source_domain = AnalysisResultSourceDomain::SimulationPlan;
                    self.current_run_id = None;
                    self.touchstone_export_policy = TouchstoneExportPolicy::disabled();
                    self.current_analysis_idx = 0;
                    self.total_analyses = 0;
                    state.simulation.active_execution = None;
                    state.simulation.abort_request = None;
                    state.ui.netlist.pending_manual_run_id = None;
                    state.ui.netlist.pending_run_buffer = None;
                    state.simulation.status = "Aborted".to_string();
                    state.push_sim_message(ConsoleMessage::warning(
                        "Simulation aborted by user".to_owned(),
                    ));
                    self.complete_campaign_member(state, false);
                }
                Err(e) => {
                    let attribution = e.attribution().cloned();
                    Self::report_failed_analysis(
                        state,
                        &format!("Analysis failed: {e}"),
                        &attribution,
                    );

                    // Mark run as partially failed and add failed analysis entry
                    let failed_label = self
                        .current_analysis_label
                        .clone()
                        .or_else(|| {
                            self.current_spec
                                .as_ref()
                                .map(|spec| self.analysis_name_for_spec(spec).to_owned())
                        })
                        .or_else(|| {
                            self.current_config
                                .as_ref()
                                .map(|config| self.analysis_name(config).to_owned())
                        })
                        .unwrap_or_else(|| "Analysis".to_owned());
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
                        let mut analysis = self.partial_failure_analysis(
                            failed_type,
                            &failed_label,
                            e.to_string(),
                            provenance,
                        );
                        analysis.failure_attribution = attribution;
                        self.materialize_current_saved_outputs(&mut analysis);
                        Some(analysis)
                    } else {
                        let message =
                            "Internal error: failed analysis has no prepared-task provenance";
                        log::error!("{message}");
                        state.push_sim_message(ConsoleMessage::error(message.to_owned()));
                        None
                    };
                    let errors = self.seal_failed_run(state, target_run_id, failed_analysis, None);
                    Self::report_seal_errors(state, errors);

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
