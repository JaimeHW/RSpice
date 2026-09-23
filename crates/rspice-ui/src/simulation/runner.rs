//! Simulation Runner - Async Simulation Execution
//!
//! Provides the bridge between UI and rspice-core simulation engine with:
//! - Async simulation execution on background thread
//! - Thread-safe progress updates
//! - Abort capability
//! - Result caching

use std::path::PathBuf;
use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicBool, Ordering},
};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

use crate::diagnostics::engine_log::{EngineLogLine, EngineLogQueue, RunLogSink};

use super::config::AnalysisConfig;
use super::execution::{ResolvedExecutionDependencies, ResolvedTaskDispatch};
use super::multi_run::AnalysisSpec;
use super::results::SimulationResult;
use super::status::{SimulationProgress, SimulationStatus};

/// Maximum UI-only transient deltas waiting for an application frame.
///
/// The solver owns the authoritative full result. This queue exists only for
/// live presentation, so a background tab or suspended browser must not let it
/// grow to the full multi-million-point analysis ceiling. When it fills, the
/// oldest undisplayed point is replaced by newer evidence; terminal retention
/// remains lossless and atomically replaces the live document.
const MAX_PENDING_LIVE_TRANSIENT_SAMPLES: usize = 8_192;

pub(crate) mod monte_carlo_checkpoint;
use monte_carlo_checkpoint::{CheckpointObserver, CheckpointQueue, replace_checkpoint};
mod live_impulses;
pub(in crate::simulation) use live_impulses::{CurrentImpulseBuffer, CurrentImpulseDelta};
use live_impulses::{LiveTransientQueue, PublishedCurrentImpulses};

#[cfg(test)]
mod device_e2e_tests;
/// Reachable from anywhere in the crate under test because the surfaces that
/// judge a specification live outside `crate::simulation` and must be able to
/// ask the executor for real per-point evidence rather than invent it.
#[cfg(test)]
pub(crate) mod pvt_point_evidence;
mod spec;
#[cfg(any(target_arch = "wasm32", test))]
mod wasm_worker;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod worker_contract;

pub(crate) mod study;

/// Optional execution overrides for spec-driven analyses.
#[derive(Debug, Clone, Default)]
pub struct SpecExecutionOptions {
    pub study_base: Option<study::StudyRunConfig>,
    pub(crate) mc_checkpoint: Option<monte_carlo_checkpoint::MonteCarloCheckpointRequest>,
    /// Histogram bins for the default all-node OP study. Configured bases carry their own.
    pub mc_histogram_bins: Option<usize>,
    pub mc_statistics: Option<crate::simulation::dialog::mc::statistics::McStatisticsConfig>,
    pub temp: Option<crate::services::simulation_runner::TempRunConfig>,
    /// Base analysis paired with a design-parameter `.STEP`. `None` retains
    /// the classic operating-point behavior for older prepared requests.
    pub parametric_base: Option<crate::services::simulation_runner::CornerBaseMode>,
    pub corner: Option<crate::services::simulation_runner::CornerRunConfig>,
    pub pac: Option<crate::services::simulation_runner::PacRunConfig>,
    pub pxf: Option<crate::services::simulation_runner::PxfRunConfig>,
    pub pnoise: Option<crate::services::simulation_runner::PnoiseRunConfig>,
    pub pstb: Option<crate::services::simulation_runner::PstbRunConfig>,
}

/// Per-task operating environment selected by the Studio Run Set.
/// Process-model selection is already materialized into the prepared source;
/// these values cover the two inputs that must be applied to the parsed deck
/// immediately before any analysis is dispatched.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AnalysisExecutionEnvironment {
    pub temperature_celsius: f64,
    pub supply_voltage: Option<f64>,
    pub nominal_supply_voltage: Option<f64>,
    #[serde(default)]
    pub supply_source_names: Vec<String>,
}

/// One fully accepted transient point published by the engine while the
/// producing analysis is still running. Only retained analog traces are
/// included, so the message is compact and has the same names as the final
/// result conversion.
///
/// The event fields carry only the nodes whose committed value differs from
/// the one this run last published. The engine reports its whole committed
/// event state at every accepted point, which for a settled net is the same
/// value thousands of times over; a live message that repeated it would cost
/// a name and a number per node per step to say nothing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TransientSampleDelta {
    pub time: f64,
    pub waveforms: Vec<TransientWaveformSample>,
    /// Digital event nodes that changed at this accepted time.
    #[serde(default)]
    pub events: Vec<TransientDigitalEventSample>,
    /// Real-valued event nodes that changed at this accepted time.
    #[serde(default)]
    pub real_events: Vec<TransientRealEventSample>,
    /// The run's digital bus declarations, published once.
    ///
    /// The table is run-constant — the engine hands the same borrowed slice
    /// at every accepted point — so it rides the first message that can name
    /// its members and is empty on every one after. Repeating it would cost a
    /// name per member per step to say what the first message already said.
    #[serde(default)]
    pub buses: Vec<TransientDigitalBusSample>,
    /// Newly accepted charge observations, independently timed and sequenced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_impulses: Option<CurrentImpulseDelta>,
}

/// One digital bus a run declared, as it crosses to the live viewer.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TransientDigitalBusSample {
    pub name: String,
    pub msb: i64,
    pub lsb: i64,
    /// Member node names, declared MSB first — the engine's node ids resolved
    /// through the run's node table, because a live viewer holds names.
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TransientWaveformSample {
    pub name: String,
    pub value: f64,
    pub y_unit: String,
}

/// One digital event node's newly committed value at an accepted point.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TransientDigitalEventSample {
    pub name: String,
    /// XSPICE 12-state event code, the same encoding the terminal result's
    /// event evidence carries.
    pub value_code: u8,
}

/// One real-valued event node's newly committed value at an accepted point.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TransientRealEventSample {
    pub name: String,
    pub value: f64,
}

//=============================================================================
// Simulation Runner
//=============================================================================

#[derive(Debug, Clone)]
pub(crate) enum SimulationRequest {
    Config(Box<AnalysisConfig>),
    Spec {
        spec: Box<AnalysisSpec>,
        options: Box<SpecExecutionOptions>,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct NetlistInput {
    netlist: String,
    source_path: Option<PathBuf>,
    project_veriloga_runtimes: crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    measurement_references:
        crate::simulation::measurement_references::PreparedMeasurementReferences,
    dependencies: ResolvedExecutionDependencies,
    environment: Option<AnalysisExecutionEnvironment>,
    stream_transient_samples: bool,
}

/// Thread-safe simulation runner
///
/// Manages simulation execution on a background thread with progress tracking
/// and abort capability.
pub struct SimulationRunner {
    /// Current progress (thread-safe)
    progress: Arc<Mutex<SimulationProgress>>,

    /// Abort flag
    abort_flag: Arc<AtomicBool>,

    /// Accepted transient points waiting for the UI controller. This is
    /// deliberately separate from progress: progress may be coalesced, while
    /// waveform samples must remain lossless and ordered.
    transient_samples: Arc<Mutex<LiveTransientQueue>>,

    /// What the engine logged during this run, waiting for the UI controller.
    ///
    /// Beside `transient_samples` and for the same reasons: the solver thread
    /// writes it, the controller drains it once a frame, and it is cleared with
    /// the rest of the per-run state when a request starts. It is not folded
    /// into progress because a log line is a statement the run made, not a
    /// value that may be coalesced.
    engine_log: Arc<Mutex<EngineLogQueue>>,

    monte_carlo_checkpoint: CheckpointQueue,

    /// Current simulation thread handle
    thread_handle: Option<JoinHandle<Result<SimulationResult, SimulationError>>>,

    /// Completed inline result waiting for the controller to poll it.
    pending_result: Option<Result<SimulationResult, SimulationError>>,

    /// Browser worker state. Native builds use `thread_handle`.
    #[cfg(target_arch = "wasm32")]
    worker_handle: wasm_worker::WorkerHandle,
}

impl Default for SimulationRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationRunner {
    /// Create a new simulation runner
    pub fn new() -> Self {
        Self {
            progress: Arc::new(Mutex::new(SimulationProgress::default())),
            abort_flag: Arc::new(AtomicBool::new(false)),
            transient_samples: Arc::new(Mutex::new(LiveTransientQueue::default())),
            engine_log: Arc::new(Mutex::new(EngineLogQueue::default())),
            monte_carlo_checkpoint: Arc::new(Mutex::new(None)),
            thread_handle: None,
            pending_result: None,
            #[cfg(target_arch = "wasm32")]
            worker_handle: wasm_worker::WorkerHandle::new(),
        }
    }

    /// Check if a simulation is currently running
    pub fn is_running(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        if self.worker_handle.is_running() {
            return true;
        }

        if let Some(ref handle) = self.thread_handle {
            !handle.is_finished()
        } else {
            false
        }
    }

    pub(crate) fn engine_availability(&self) -> super::status::EngineAvailability {
        #[cfg(target_arch = "wasm32")]
        {
            self.worker_handle.availability()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            super::status::EngineAvailability::Ready
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn set_engine_wakeup(&mut self, wakeup: Arc<dyn Fn() + Send + Sync>) {
        self.worker_handle.set_wakeup(wakeup);
    }

    /// Retry backend startup without queuing a simulation or disturbing a
    /// completion that the controller still owns.
    pub(crate) fn retry_engine_startup(&mut self) -> Result<(), SimulationError> {
        if !self.can_accept_prepared_task() {
            return Err(SimulationError::AlreadyRunning);
        }
        #[cfg(target_arch = "wasm32")]
        self.worker_handle.retry_startup()?;
        Ok(())
    }

    /// Get current status
    pub fn status(&self) -> SimulationStatus {
        lock_progress(&self.progress, "SimulationRunner::status")
            .status
            .clone()
    }

    /// Get current progress percentage (0.0 to 1.0)
    pub fn progress_fraction(&self) -> Option<f32> {
        lock_progress(&self.progress, "SimulationRunner::progress_fraction")
            .status
            .progress()
    }

    /// Abort current simulation
    pub fn abort(&self) {
        self.abort_flag.store(true, Ordering::SeqCst);
        #[cfg(target_arch = "wasm32")]
        self.worker_handle.abort();
    }

    /// Drain every accepted transient point published since the previous UI
    /// update. The engine has already retained the authoritative full result;
    /// this queue exists only for responsive live presentation.
    pub(in crate::simulation) fn drain_transient_samples(&self) -> Vec<TransientSampleDelta> {
        let mut samples = match self.transient_samples.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                log::warn!("Recovered poisoned live-transient queue in runner");
                poisoned.into_inner()
            }
        };
        samples.drain()
    }

    /// Drain everything the engine logged since the previous UI update.
    ///
    /// The controller writes these into the Console as `ENG` rows. The run's
    /// own bound is inside the queue, so a solver that never stops talking
    /// cannot grow this without limit.
    pub(in crate::simulation) fn drain_engine_log(&self) -> Vec<EngineLogLine> {
        crate::diagnostics::engine_log::lock_queue(&self.engine_log).drain()
    }

    /// Latest complete journal, available independently of terminal success.
    pub(in crate::simulation) fn take_monte_carlo_checkpoint(&self) -> Option<Arc<[u8]>> {
        self.monte_carlo_checkpoint
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
    }

    #[cfg(test)]
    pub(in crate::simulation) fn store_checkpoint_for_test(&self, bytes: Arc<[u8]>) {
        monte_carlo_checkpoint::replace_checkpoint(&self.monte_carlo_checkpoint, bytes);
    }

    /// Abort and discard all runner-local completion/progress state.
    ///
    /// Native worker threads cannot be force-killed, but setting the shared
    /// abort flag and dropping the join handle detaches stale work so its
    /// eventual result cannot be polled into a replacement document.
    pub(crate) fn reset_for_design_replacement(&mut self) {
        self.abort();
        self.thread_handle = None;
        self.pending_result = None;
        self.abort_flag = Arc::new(AtomicBool::new(false));
        self.progress = Arc::new(Mutex::new(SimulationProgress::default()));
        self.transient_samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        self.engine_log = Arc::new(Mutex::new(EngineLogQueue::default()));
        self.monte_carlo_checkpoint = Arc::new(Mutex::new(None));

        #[cfg(target_arch = "wasm32")]
        {
            let _ = self.worker_handle.poll_result();
        }
    }

    /// Check if aborted
    #[cfg(test)]
    pub fn is_aborted(&self) -> bool {
        self.abort_flag.load(Ordering::SeqCst)
    }

    /// Poll for completion and get result
    ///
    /// Returns `Some(result)` if simulation completed, `None` if still running or no simulation.
    pub fn poll_result(&mut self) -> Option<Result<SimulationResult, SimulationError>> {
        if let Some(result) = self.pending_result.take() {
            return Some(self.result_after_abort(result));
        }

        #[cfg(target_arch = "wasm32")]
        if let Some(result) = self.worker_handle.poll_result() {
            return Some(self.result_after_abort(result));
        }

        // Check if thread is finished
        let is_finished = self.thread_handle.as_ref().is_some_and(|h| h.is_finished());

        if is_finished {
            // Take the handle and join
            if let Some(handle) = self.thread_handle.take() {
                let result = match handle.join() {
                    Ok(result) => result,
                    Err(_) => Err(SimulationError::ThreadPanic),
                };
                return Some(self.result_after_abort(result));
            }
        }

        None
    }

    fn result_after_abort(
        &self,
        result: Result<SimulationResult, SimulationError>,
    ) -> Result<SimulationResult, SimulationError> {
        if self.abort_flag.load(Ordering::SeqCst) {
            Err(SimulationError::Aborted)
        } else {
            result
        }
    }

    fn has_unpolled_result(&self) -> bool {
        let has_native_result = self.pending_result.is_some()
            || self
                .thread_handle
                .as_ref()
                .is_some_and(|handle| handle.is_finished());

        #[cfg(target_arch = "wasm32")]
        {
            has_native_result || self.worker_handle.has_unpolled_result()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            has_native_result
        }
    }

    /// Whether a new prepared task can be accepted without displacing either
    /// active work or a completion that the controller has not consumed yet.
    ///
    /// A finished native thread is intentionally still busy from the
    /// controller's perspective until `poll_result` joins it. This closes the
    /// otherwise small window where a second Run action could overwrite the
    /// batch metadata needed to classify that completion.
    pub(in crate::simulation) fn can_accept_prepared_task(&self) -> bool {
        !self.is_running() && !self.has_unpolled_result()
    }

    #[cfg(test)]
    pub(crate) fn store_pending_result(
        &mut self,
        result: Result<SimulationResult, SimulationError>,
    ) -> Result<(), SimulationError> {
        if self.pending_result.is_some() || self.thread_handle.is_some() {
            return Err(SimulationError::AlreadyRunning);
        }

        self.pending_result = Some(result);
        Ok(())
    }

    /// Dispatch one task taken from an authorized immutable run snapshot.
    ///
    /// Prepared netlists are self-contained, so a source path is deliberately
    /// unavailable here: the worker cannot reopen editor-era dependencies.
    pub(in crate::simulation) fn start_prepared(
        &mut self,
        dispatch: ResolvedTaskDispatch,
        stream_transient_samples: bool,
    ) -> Result<(), SimulationError> {
        let (
            task,
            executable_netlist,
            project_veriloga_runtimes,
            measurement_references,
            dependencies,
            environment,
        ) = dispatch.into_runner_parts();
        let request = match task.config {
            Some(config) => SimulationRequest::Config(Box::new(config)),
            None => SimulationRequest::Spec {
                spec: Box::new(task.spec),
                options: Box::new(task.spec_options),
            },
        };
        self.start_request(
            request,
            NetlistInput {
                netlist: executable_netlist.to_string(),
                source_path: None,
                project_veriloga_runtimes,
                measurement_references,
                dependencies,
                environment,
                stream_transient_samples,
            },
        )
    }

    /// Start a simulation with the given configuration
    ///
    /// Returns error if a simulation is already running.
    #[cfg(test)]
    fn start(&mut self, config: AnalysisConfig, netlist: String) -> Result<(), SimulationError> {
        self.start_with_source_path(config, netlist, None)
    }

    /// Start a simulation with the given configuration and source path.
    #[cfg(test)]
    fn start_with_source_path(
        &mut self,
        config: AnalysisConfig,
        netlist: String,
        source_path: Option<PathBuf>,
    ) -> Result<(), SimulationError> {
        self.start_request(
            SimulationRequest::Config(Box::new(config)),
            NetlistInput {
                measurement_references: Default::default(),
                netlist,
                source_path,
                project_veriloga_runtimes: Default::default(),
                dependencies: Default::default(),
                environment: None,
                stream_transient_samples: false,
            },
        )
    }

    fn start_request(
        &mut self,
        request: SimulationRequest,
        input: NetlistInput,
    ) -> Result<(), SimulationError> {
        if self.is_running() || self.has_unpolled_result() {
            return Err(SimulationError::AlreadyRunning);
        }

        // Reset state
        self.take_monte_carlo_checkpoint();
        self.abort_flag.store(false, Ordering::SeqCst);
        match self.transient_samples.lock() {
            Ok(mut samples) => samples.clear(),
            Err(poisoned) => poisoned.into_inner().clear(),
        }
        crate::diagnostics::engine_log::lock_queue(&self.engine_log).clear();
        {
            let mut progress = lock_progress(&self.progress, "SimulationRunner::start_request");
            *progress = SimulationProgress::new();
        }

        // Clone Arcs for the thread
        let progress = Arc::clone(&self.progress);
        let abort_flag = Arc::clone(&self.abort_flag);
        let transient_samples = input
            .stream_transient_samples
            .then(|| Arc::clone(&self.transient_samples));

        // Spawn simulation thread with real engine. Browser builds route
        // through the module worker so the egui UI thread stays responsive.
        // Former inline browser execution is deliberately not retained.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let streams = RunStreams {
                monte_carlo_checkpoint: Some(Arc::clone(&self.monte_carlo_checkpoint)),
                transient_samples,
                engine_log: Some(RunLogSink::queued(
                    Arc::clone(&self.engine_log),
                    request_asked_for_verbose(&request),
                )),
                ..RunStreams::default()
            };
            let handle = std::thread::spawn(move || {
                run_simulation_thread(request, input, progress, abort_flag, streams)
            });
            self.thread_handle = Some(handle);
        }
        // The worker executes in its own wasm instance, so a queue this side
        // of the boundary is not something its run can write: the worker-side
        // run posts each line across the contract and the UI handler pushes it
        // onto this runner's queue.
        #[cfg(target_arch = "wasm32")]
        {
            wasm_worker::start_worker_request(
                &mut self.worker_handle,
                request,
                input,
                progress,
                abort_flag,
                transient_samples,
                Arc::clone(&self.engine_log),
                Arc::clone(&self.monte_carlo_checkpoint),
            )?;
        }
        Ok(())
    }
}

/// Whether this request asked the engine to trace its solve.
///
/// Read off the request rather than threaded separately, because the request
/// *is* where the switch lives: the HB and PSS forms author `verbose` into
/// their specification, and a manual deck authors it into the same field
/// through `VERBOSE=`. A study inherits this switch from its frozen periodic
/// base. Other requests publish receipts without enabling the solver trace.
pub(in crate::simulation::runner) fn request_asked_for_verbose(
    request: &SimulationRequest,
) -> bool {
    match request {
        SimulationRequest::Config(_) => false,
        SimulationRequest::Spec { spec, options } => match spec.as_ref() {
            AnalysisSpec::Pss { verbose, .. } | AnalysisSpec::HarmonicBalance { verbose, .. } => {
                *verbose
            }
            AnalysisSpec::MonteCarlo { .. } | AnalysisSpec::Optimization { .. } => options
                .study_base
                .as_ref()
                .is_some_and(|base| match &base.analysis {
                    study::StudyAnalysis::Native(AnalysisSpec::HarmonicBalance {
                        verbose, ..
                    }) => *verbose,
                    study::StudyAnalysis::Pss(pss) => {
                        matches!(pss.request, AnalysisSpec::Pss { verbose: true, .. })
                    }
                    _ => false,
                }),
            _ => false,
        },
    }
}

/// Everything a run publishes while it is still running, other than progress.
///
/// One parameter rather than four: the two observers exist for the browser
/// worker, which has no shared memory to write a queue in, and the two queues
/// exist for the native run, which does. Every future stream belongs here for
/// the same reason.
#[derive(Default)]
pub(in crate::simulation::runner) struct RunStreams {
    pub(in crate::simulation::runner) progress_observer: Option<ProgressObserver>,
    pub(in crate::simulation::runner) transient_samples: Option<Arc<Mutex<LiveTransientQueue>>>,
    pub(in crate::simulation::runner) transient_sample_observer: Option<TransientSampleObserver>,
    pub(in crate::simulation::runner) engine_log: Option<RunLogSink>,
    pub(in crate::simulation::runner) monte_carlo_checkpoint: Option<CheckpointQueue>,
    pub(in crate::simulation::runner) checkpoint_observer: Option<CheckpointObserver>,
}

fn lock_progress<'a>(
    progress: &'a Arc<Mutex<SimulationProgress>>,
    context: &str,
) -> MutexGuard<'a, SimulationProgress> {
    match progress.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            log::warn!(
                "Recovered poisoned simulation-progress lock in runner ({})",
                context
            );
            poisoned.into_inner()
        }
    }
}

pub(in crate::simulation::runner) type ProgressObserver = fn(&SimulationProgress);
pub(in crate::simulation::runner) type TransientSampleObserver = fn(&TransientSampleDelta);

fn notify_progress(progress: &SimulationProgress, observer: Option<ProgressObserver>) {
    if let Some(observer) = observer {
        observer(progress);
    }
}

/// Bridges the engine's abort/progress hook onto the runner's shared state:
/// polls the UI abort flag and folds the reported completed fraction back
/// into the status line at the engine's abort-poll cadence.
struct RunnerSignal {
    abort_flag: Arc<AtomicBool>,
    progress: Arc<Mutex<SimulationProgress>>,
    progress_observer: Option<ProgressObserver>,
    transient_samples: Option<Arc<Mutex<LiveTransientQueue>>>,
    transient_sample_observer: Option<TransientSampleObserver>,
    published_events: Mutex<PublishedEventValues>,
}

/// The last event value this run published for each node, so an accepted
/// point can report changes instead of the whole committed state.
///
/// One signal serves one analysis — `run_simulation_thread` builds it after
/// the deck is prepared and drops it when the engine returns — so the map is
/// empty at the first accepted point of every run without being cleared.
/// [`AbortSignal`](rspice_core::abort_signal::AbortSignal) observes through a
/// shared reference, hence the lock; it is uncontended, because only the
/// solver thread reports samples.
#[derive(Debug, Default)]
struct PublishedEventValues {
    currents: PublishedCurrentImpulses,
    digital: std::collections::HashMap<rspice_core::NodeId, u8>,
    real: std::collections::HashMap<rspice_core::NodeId, u64>,
    /// Whether this run has already published its bus declarations.
    buses: bool,
}

/// The netlist name of an event node, or `None` when the node table does not
/// cover it. Event values are keyed by node id, where ground is zero and never
/// appears, so `node_names[node_id - 1]` is the node's name.
fn event_node_name(node_names: &[String], node: rspice_core::NodeId) -> Option<&str> {
    node_names
        .get(node.checked_sub(1)?)
        .map(String::as_str)
        .filter(|name| !name.trim().is_empty())
}

fn push_live_transient_sample(
    samples: &Arc<Mutex<LiveTransientQueue>>,
    delta: TransientSampleDelta,
) {
    let mut samples = match samples.lock() {
        Ok(samples) => samples,
        Err(poisoned) => poisoned.into_inner(),
    };
    samples.push(delta);
}

impl RunnerSignal {
    /// The event nodes whose committed value differs from the one this run
    /// last published, and the record of what was published updated to match.
    ///
    /// A node the node table cannot name is skipped without being recorded:
    /// it stays a candidate, so the first accepted point at which a name does
    /// resolve reports it rather than treating it as already seen.
    fn changed_event_values(
        &self,
        sample: &rspice_core::abort_signal::TransientSample<'_>,
    ) -> (
        Vec<TransientDigitalEventSample>,
        Vec<TransientRealEventSample>,
    ) {
        if sample.digital_values.is_empty() && sample.real_values.is_empty() {
            return (Vec::new(), Vec::new());
        }
        let mut published = match self.published_events.lock() {
            Ok(published) => published,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut events = Vec::new();
        for &(node, code) in sample.digital_values {
            let Some(name) = event_node_name(sample.node_names, node) else {
                continue;
            };
            if published.digital.insert(node, code.0) == Some(code.0) {
                continue;
            }
            events.push(TransientDigitalEventSample {
                name: name.to_owned(),
                value_code: code.0,
            });
        }
        let mut real_events = Vec::new();
        for &(node, value) in sample.real_values {
            let Some(name) = event_node_name(sample.node_names, node) else {
                continue;
            };
            // Compared as bits so the record is an exact account of what was
            // published: two values that are not equal as floats are never
            // treated as the same reported value.
            let bits = value.to_bits();
            if published.real.insert(node, bits) == Some(bits) {
                continue;
            }
            real_events.push(TransientRealEventSample {
                name: name.to_owned(),
                value,
            });
        }
        (events, real_events)
    }

    /// The run's bus declarations, the first time they can be stated.
    ///
    /// The engine's table names members by node id; a live message names them
    /// the way the terminal evidence does, so a bus whose members the node
    /// table cannot name yet is not published *and not recorded* — the next
    /// accepted point tries again rather than the run losing its table to one
    /// early sample.
    fn declared_buses(
        &self,
        sample: &rspice_core::abort_signal::TransientSample<'_>,
    ) -> Vec<TransientDigitalBusSample> {
        if sample.digital_buses.is_empty() {
            return Vec::new();
        }
        let mut published = match self.published_events.lock() {
            Ok(published) => published,
            Err(poisoned) => poisoned.into_inner(),
        };
        if published.buses {
            return Vec::new();
        }
        let mut buses = Vec::with_capacity(sample.digital_buses.len());
        for bus in sample.digital_buses {
            let mut members = Vec::with_capacity(bus.members.len());
            for &node in &bus.members {
                let Some(name) = event_node_name(sample.node_names, node) else {
                    return Vec::new();
                };
                members.push(name.to_owned());
            }
            buses.push(TransientDigitalBusSample {
                name: bus.name.clone(),
                msb: bus.msb,
                lsb: bus.lsb,
                members,
            });
        }
        published.buses = true;
        buses
    }
}

impl rspice_core::abort_signal::AbortSignal for RunnerSignal {
    fn is_aborted(&self) -> bool {
        self.abort_flag.load(Ordering::SeqCst)
    }

    fn observe_progress(&self, fraction: f64) {
        let mut p = lock_progress(&self.progress, "observe_progress");
        p.observe_engine_fraction(fraction);
        notify_progress(&p, self.progress_observer);
    }

    fn observe_transient_sample(&self, sample: rspice_core::abort_signal::TransientSample<'_>) {
        if self.transient_samples.is_none() && self.transient_sample_observer.is_none() {
            return;
        }
        let Some(&time) = sample.time.last() else {
            return;
        };
        let mut waveforms = Vec::with_capacity(
            sample
                .node_voltages
                .len()
                .saturating_add(sample.branch_currents.len()),
        );
        for (index, values) in sample.node_voltages.iter().enumerate() {
            let Some(&value) = values.last() else {
                continue;
            };
            let name = sample
                .node_names
                .get(index)
                .cloned()
                .unwrap_or_else(|| (index + 1).to_string());
            waveforms.push(TransientWaveformSample {
                name,
                value,
                y_unit: "V".to_owned(),
            });
        }
        for (index, values) in sample.branch_currents.iter().enumerate() {
            let Some(&value) = values.last() else {
                continue;
            };
            let branch = sample
                .branch_names
                .get(index)
                .cloned()
                .unwrap_or_else(|| format!("branch{}", index + 1));
            let name = if branch.len() >= 3
                && (branch.starts_with("I(") || branch.starts_with("i("))
                && branch.ends_with(')')
            {
                branch
            } else {
                format!("I({branch})")
            };
            waveforms.push(TransientWaveformSample {
                name,
                value,
                y_unit: "A".to_owned(),
            });
        }
        let (events, real_events) = self.changed_event_values(&sample);
        let buses = self.declared_buses(&sample);
        let current_impulses = match self.published_events.lock() {
            Ok(mut published) => published.currents.publish(&sample),
            Err(poisoned) => poisoned.into_inner().currents.publish(&sample),
        };
        let delta = TransientSampleDelta {
            time,
            waveforms,
            events,
            real_events,
            buses,
            current_impulses,
        };
        if let Some(samples) = &self.transient_samples {
            push_live_transient_sample(samples, delta.clone());
        }
        if let Some(observer) = self.transient_sample_observer {
            observer(&delta);
        }
    }
}

fn initial_status_for_request(request: &SimulationRequest) -> SimulationStatus {
    match request {
        SimulationRequest::Config(config) => match config.as_ref() {
            AnalysisConfig::DcOp(_) => SimulationStatus::DcOperatingPoint,
            AnalysisConfig::DcSweep(dc) => SimulationStatus::DcSweep {
                source: dc.source.clone(),
                progress: 0.0,
            },
            AnalysisConfig::Transient(tran) => SimulationStatus::Transient {
                time: 0.0,
                stop_time: tran.stop_time,
            },
            AnalysisConfig::Ac(ac) => SimulationStatus::AcAnalysis {
                freq: ac.start_freq,
                stop_freq: ac.stop_freq,
            },
            AnalysisConfig::Noise(noise) => SimulationStatus::NoiseAnalysis {
                freq: noise.start_freq,
                stop_freq: noise.stop_freq,
            },
            AnalysisConfig::PoleZero(_) => SimulationStatus::PoleZero,
            AnalysisConfig::Sensitivity(_) => SimulationStatus::Sensitivity,
        },
        SimulationRequest::Spec { spec, options } => initial_status_for_spec(spec, options),
    }
}

fn initial_status_for_spec(
    spec: &AnalysisSpec,
    options: &SpecExecutionOptions,
) -> SimulationStatus {
    match spec {
        AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. } => SimulationStatus::DcOperatingPoint,
        AnalysisSpec::DcSweep { source_name, .. } => SimulationStatus::DcSweep {
            source: source_name.clone(),
            progress: 0.0,
        },
        AnalysisSpec::Transient { stop_time, .. } => SimulationStatus::Transient {
            time: 0.0,
            stop_time: *stop_time,
        },
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            ..
        } => SimulationStatus::AcAnalysis {
            freq: *start_freq,
            stop_freq: *stop_freq,
        },
        AnalysisSpec::AcData { frequencies, .. } => SimulationStatus::AcAnalysis {
            freq: frequencies.first().copied().unwrap_or(0.0),
            stop_freq: frequencies.last().copied().unwrap_or(0.0),
        },
        AnalysisSpec::Noise {
            start_freq,
            stop_freq,
            ..
        } => SimulationStatus::NoiseAnalysis {
            freq: *start_freq,
            stop_freq: *stop_freq,
        },
        AnalysisSpec::Pss {
            fundamental_freq,
            method,
            num_harmonics,
            ..
        } => match method {
            crate::simulation::multi_run::PssMethod::Shooting => SimulationStatus::Transient {
                time: 0.0,
                stop_time: positive_period(*fundamental_freq),
            },
            crate::simulation::multi_run::PssMethod::HarmonicBalance => {
                SimulationStatus::AcAnalysis {
                    freq: *fundamental_freq,
                    stop_freq: *fundamental_freq * (*num_harmonics).max(1) as f64,
                }
            }
        },
        // The spectrum reads a state that already converged, so its progress
        // is a frequency sweep over the retained harmonics, not a solve. The
        // fundamental is not known until the artifact is opened; report the
        // harmonic index range instead of inventing a frequency.
        AnalysisSpec::PssSpectrum { num_harmonics } => SimulationStatus::AcAnalysis {
            freq: 1.0,
            stop_freq: (*num_harmonics).max(1) as f64,
        },
        AnalysisSpec::HarmonicBalance { tones, .. } => SimulationStatus::AcAnalysis {
            freq: tones.first().map(|tone| tone.frequency).unwrap_or(1.0),
            stop_freq: tones
                .iter()
                .map(|tone| tone.frequency * tone.harmonics.max(1) as f64)
                .fold(1.0, f64::max),
        },
        AnalysisSpec::Tf { .. } => SimulationStatus::DcOperatingPoint,
        AnalysisSpec::Sensitivity { .. } => SimulationStatus::Sensitivity,
        AnalysisSpec::PoleZero { .. } => SimulationStatus::PoleZero,
        AnalysisSpec::Pac => {
            let pac = options.pac.as_ref().cloned().unwrap_or_default();
            SimulationStatus::AcAnalysis {
                freq: pac.start_freq,
                stop_freq: pac.stop_freq,
            }
        }
        AnalysisSpec::Pnoise => {
            let pnoise = options.pnoise.as_ref().cloned().unwrap_or_default();
            SimulationStatus::NoiseAnalysis {
                freq: pnoise.start_freq,
                stop_freq: pnoise.stop_freq,
            }
        }
        AnalysisSpec::Pxf => {
            let pxf = options.pxf.as_ref().cloned().unwrap_or_default();
            SimulationStatus::AcAnalysis {
                freq: pxf.start_freq,
                stop_freq: pxf.stop_freq,
            }
        }
        AnalysisSpec::Pstb => {
            let pstb = options.pstb.as_ref().cloned().unwrap_or_default();
            SimulationStatus::AcAnalysis {
                freq: pstb.pss_fundamental_freq,
                stop_freq: pstb.pss_fundamental_freq,
            }
        }
        AnalysisSpec::Stb {
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            ..
        } => SimulationStatus::AcAnalysis {
            freq: *start_freq,
            stop_freq: *stop_freq,
        },
        AnalysisSpec::MonteCarlo { .. } => SimulationStatus::PostProcessing,
        AnalysisSpec::Parametric => SimulationStatus::DcSweep {
            source: if options.temp.is_some() {
                "TEMP".to_string()
            } else {
                "STEP".to_string()
            },
            progress: 0.0,
        },
        AnalysisSpec::Corner => SimulationStatus::DcSweep {
            source: "CORNER".to_string(),
            progress: 0.0,
        },
        AnalysisSpec::Reliability { .. } | AnalysisSpec::Optimization { .. } => {
            SimulationStatus::PostProcessing
        }
        AnalysisSpec::Soa { stop_time, .. } | AnalysisSpec::Envelope { stop_time, .. } => {
            SimulationStatus::Transient {
                time: 0.0,
                stop_time: *stop_time,
            }
        }
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            ..
        } => SimulationStatus::AcAnalysis {
            freq: *fundamental_freq,
            stop_freq: *fundamental_freq * (*num_harmonics).max(1) as f64,
        },
        // The spectrum the transient already computed is selected, not solved.
        // Its band is stated in bins, because the bin width depends on the
        // record the engine actually retained and is only known once the
        // artifact is open.
        AnalysisSpec::Fft { request } => SimulationStatus::AcAnalysis {
            freq: 0.0,
            stop_freq: (request.points / 2) as f64,
        },
        AnalysisSpec::Qpss { tones, .. } => SimulationStatus::AcAnalysis {
            freq: tones.first().map_or(0.0, |tone| tone.frequency),
            stop_freq: tones.iter().map(|tone| tone.frequency).fold(0.0, f64::max),
        },
        AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            ..
        }
        | AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            ..
        } => SimulationStatus::AcAnalysis {
            freq: *start_freq,
            stop_freq: *stop_freq,
        },
        AnalysisSpec::Qpac {
            start_freq,
            stop_freq,
            controls,
            ..
        } => SimulationStatus::AcAnalysis {
            freq: controls
                .explicit_offsets
                .as_ref()
                .and_then(|v| v.first().copied())
                .unwrap_or(*start_freq),
            stop_freq: controls
                .explicit_offsets
                .as_ref()
                .and_then(|v| v.last().copied())
                .unwrap_or(*stop_freq),
        },
        AnalysisSpec::Qpxf {
            start_freq,
            stop_freq,
            controls,
            ..
        } => SimulationStatus::AcAnalysis {
            freq: controls
                .explicit_frequencies
                .as_ref()
                .and_then(|v| v.first().copied())
                .unwrap_or(*start_freq),
            stop_freq: controls
                .explicit_frequencies
                .as_ref()
                .and_then(|v| v.last().copied())
                .unwrap_or(*stop_freq),
        },
        AnalysisSpec::Hbnoise {
            start_freq,
            stop_freq,
            ..
        } => SimulationStatus::NoiseAnalysis {
            freq: *start_freq,
            stop_freq: *stop_freq,
        },
        AnalysisSpec::Qpnoise {
            start_freq,
            stop_freq,
            controls,
            ..
        } => SimulationStatus::NoiseAnalysis {
            freq: controls
                .explicit_frequencies
                .as_ref()
                .and_then(|v| v.first().copied())
                .unwrap_or(*start_freq),
            stop_freq: controls
                .explicit_frequencies
                .as_ref()
                .and_then(|v| v.last().copied())
                .unwrap_or(*stop_freq),
        },
        AnalysisSpec::TransientNoise { stop_time, .. } => SimulationStatus::Transient {
            time: 0.0,
            stop_time: *stop_time,
        },
        // What it solves: one nominal operating point, then two more per
        // statistical variable. `PostProcessing` described a kind that never
        // reached a solver.
        AnalysisSpec::DcMismatch { .. } => SimulationStatus::DcOperatingPoint,
    }
}

fn positive_period(frequency: f64) -> f64 {
    if frequency > 0.0 {
        1.0 / frequency
    } else {
        0.0
    }
}

/// Simulation execution in background thread
///
/// Runs the actual rspice-core simulation engine.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn run_simulation_thread(
    request: SimulationRequest,
    input: NetlistInput,
    progress: Arc<Mutex<SimulationProgress>>,
    abort_flag: Arc<AtomicBool>,
    streams: RunStreams,
) -> Result<SimulationResult, SimulationError> {
    run_simulation_thread_with_progress_observer(request, input, progress, abort_flag, streams)
}

pub(in crate::simulation::runner) fn run_simulation_thread_with_progress_observer(
    request: SimulationRequest,
    input: NetlistInput,
    progress: Arc<Mutex<SimulationProgress>>,
    abort_flag: Arc<AtomicBool>,
    streams: RunStreams,
) -> Result<SimulationResult, SimulationError> {
    use super::engine_bridge::EngineBridge;

    let RunStreams {
        progress_observer,
        transient_samples,
        transient_sample_observer,
        engine_log,
        monte_carlo_checkpoint,
        checkpoint_observer,
    } = streams;

    // Whatever the engine logs from here on belongs to this run, and only to
    // it: the guard is removed on every exit path below, including a panic
    // unwinding out of the solver.
    let _engine_log = engine_log.map(crate::diagnostics::engine_log::install);

    // Update status: parsing
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(parse)");
        p.update_status(SimulationStatus::Parsing);
        notify_progress(&p, progress_observer);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = lock_progress(&progress, "run_simulation_thread(abort-after-parse)");
        p.abort();
        notify_progress(&p, progress_observer);
        return Err(SimulationError::Aborted);
    }

    if !input.project_veriloga_runtimes.is_empty() {
        input.project_veriloga_runtimes.install().map_err(|error| {
            SimulationError::CircuitError(format!(
                "Could not install prepared project Verilog-A runtimes: {error}"
            ))
        })?;
    }

    // Create engine bridge
    let bridge =
        EngineBridge::new().with_measurement_references(input.measurement_references.clone());

    // Update status: building
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(build)");
        p.update_status(SimulationStatus::Building);
        notify_progress(&p, progress_observer);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = lock_progress(&progress, "run_simulation_thread(abort-after-build)");
        p.abort();
        notify_progress(&p, progress_observer);
        return Err(SimulationError::Aborted);
    }

    // Update status based on analysis type.
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(status-by-analysis)");
        p.update_status(initial_status_for_request(&request));
        notify_progress(&p, progress_observer);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = lock_progress(&progress, "run_simulation_thread(abort-before-execute)");
        p.abort();
        notify_progress(&p, progress_observer);
        return Err(SimulationError::Aborted);
    }

    let signal = RunnerSignal {
        abort_flag: abort_flag.clone(),
        progress: progress.clone(),
        progress_observer,
        transient_samples,
        transient_sample_observer,
        published_events: Mutex::default(),
    };

    let publish_checkpoint = |bytes: &[u8]| {
        if let Some(queue) = &monte_carlo_checkpoint {
            replace_checkpoint(queue, Arc::from(bytes));
        }
        if let Some(observer) = &checkpoint_observer {
            observer(bytes)?;
        }
        Ok(())
    };
    let checkpoint_observer = (monte_carlo_checkpoint.is_some() || checkpoint_observer.is_some())
        .then_some(&publish_checkpoint as &(dyn Fn(&[u8]) -> Result<(), SimulationError> + Sync));
    let result = match request {
        SimulationRequest::Config(config) => {
            input
                .dependencies
                .validate_for_config()
                .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
            // Run simulation via engine bridge with abort support
            log::info!("Running simulation via engine bridge: {:?}", config);
            match bridge.run_with_abort_and_source_path_and_environment(
                &config,
                &input.netlist,
                input.source_path.as_deref(),
                input.environment,
                &signal,
            ) {
                Ok(r) => {
                    log::info!("Engine bridge returned successfully");
                    r
                }
                Err(e) => {
                    log::error!("Engine bridge error: {:?}", e);
                    return Err(e);
                }
            }
        }
        SimulationRequest::Spec { spec, options } => {
            log::info!("Running simulation via spec path: {:?}", spec.run_type());
            spec::run_spec_request_with_environment_and_checkpoint_observer(
                &bridge,
                *spec,
                *options,
                &input.netlist,
                input.source_path.as_deref(),
                &input.dependencies,
                input.environment,
                &signal,
                checkpoint_observer,
            )?
        }
    };

    // Mark complete
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(complete)");
        p.complete();
        notify_progress(&p, progress_observer);
    }

    log::info!("Simulation thread completed successfully");
    Ok(result)
}

//=============================================================================
// Simulation Error
//=============================================================================

/// Errors that can occur during simulation
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    /// Netlist parsing error
    ParseError(String),

    /// A node or device referenced by a behavioral expression could not be bound.
    BehavioralReference {
        owner_name: String,
        canonical_owner_name: String,
        dependency_name: String,
        canonical_dependency_name: String,
        reason: String,
    },

    /// Circuit building error
    CircuitError(String),

    /// Binding an instance to a Verilog-A or mixed Verilog-AMS master failed.
    ///
    /// Kept apart from [`Self::ParseError`] and [`Self::CircuitError`] — which
    /// is where the whole `.VERILOGA` seam used to land, split between them by
    /// whichever untyped engine variant each site happened to reach for —
    /// because the workbench acts on this one: `instance` names the symbol to
    /// mark on the schematic, `module` the master to open, `location` the
    /// source file to show, and `kind` decides between "the file is missing",
    /// "the name is wrong", "this value is wrong", "this port is wrong", and
    /// "this is ours to fix" without reading a word of the message.
    Elaboration {
        /// Deck name of the instance, when one instance owns the failure.
        instance: Option<String>,
        /// The master or source the failure is about.
        module: Option<String>,
        /// Stable token from `rspice_core::ElaborationErrorKind::as_str`.
        kind: String,
        /// File and line the engine could point at, rendered as it renders it.
        location: Option<String>,
        /// The engine's full sentence, which is what a person reads.
        message: String,
    },

    /// Solver error
    SolverError(String),

    /// A well-formed output request the finished analysis does not carry.
    ///
    /// The signal is the spelling that was authored, not the registry's
    /// canonical form: what a person has to fix is the request they wrote,
    /// and naming it back to them in another spelling would not identify it.
    RequestedSignalUnavailable {
        signal: String,
        analysis: String,
        coordinate: Option<String>,
    },

    /// A result whose signal registry and numeric payload disagree with the
    /// schema its own result type promises.
    ///
    /// Nothing in the design is at fault and no edit fixes it, so the detail
    /// exists to be reported rather than acted on. Boxed because the payload
    /// is wider than every other variant and this enum is the error half of
    /// `Result` throughout the crate.
    ResultSchemaMismatch(Box<ResultSchemaMismatch>),

    /// Convergence failure
    ConvergenceFailed { iterations: usize, message: String },

    /// A failure the engine could attribute to named design objects.
    ///
    /// `message` is the exact text the unattributed variant would have
    /// displayed, so nothing a person reads changes when the engine gains
    /// the ability to name the objects behind it. The attribution is the
    /// addition, and it is what lets a schematic mark them.
    Attributed {
        message: String,
        attribution: crate::state::ConvergenceAttribution,
    },

    /// Simulation was aborted
    Aborted,

    /// A simulation is already running
    AlreadyRunning,

    /// Thread panicked
    ThreadPanic,

    /// Invalid configuration
    InvalidConfig(String),

    /// An engine outcome the result-only runner cannot represent.
    UnsupportedOutcome(String),

    /// A configurable production resource budget was exceeded.
    ResourceLimit {
        resource: String,
        requested: usize,
        limit: usize,
    },
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SimulationError::BehavioralReference {
                canonical_owner_name,
                canonical_dependency_name,
                reason,
                ..
            } => write!(
                f,
                "Device instance {canonical_owner_name}: Problem with value for \
                 {canonical_dependency_name} in {canonical_owner_name} ({reason})"
            ),
            SimulationError::CircuitError(msg) => write!(f, "Circuit error: {}", msg),
            // The engine's own rendering, verbatim: it already leads with the
            // span, the instance and the master, so re-labelling it here would
            // only push that identification further from the reader.
            SimulationError::Elaboration { message, .. } => write!(f, "{message}"),
            SimulationError::SolverError(msg) => write!(f, "Solver error: {}", msg),
            SimulationError::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            } => {
                write!(
                    f,
                    "Requested signal '{signal}' is unavailable for {analysis} analysis"
                )?;
                match coordinate {
                    Some(coordinate) => write!(f, " at {coordinate}"),
                    None => Ok(()),
                }
            }
            SimulationError::ResultSchemaMismatch(mismatch) => write!(f, "{mismatch}"),
            SimulationError::ConvergenceFailed {
                iterations,
                message,
            } => {
                write!(
                    f,
                    "Convergence failed after {} iterations: {}",
                    iterations, message
                )
            }
            SimulationError::Attributed { message, .. } => write!(f, "{message}"),
            SimulationError::Aborted => write!(f, "Simulation aborted"),
            SimulationError::AlreadyRunning => write!(f, "A simulation is already running"),
            SimulationError::ThreadPanic => write!(f, "Simulation thread panicked"),
            SimulationError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            SimulationError::UnsupportedOutcome(msg) => {
                write!(f, "Unsupported simulation outcome: {msg}")
            }
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => write!(
                f,
                "Resource limit exceeded for {resource}: requested {requested}, limit {limit}"
            ),
        }
    }
}

impl SimulationError {
    /// The design objects the engine named for this failure, if it named any.
    ///
    /// One reader of the [`Self::Attributed`] payload, so the console anchor
    /// and any later marker are looking at the same answer rather than each
    /// re-matching the variant.
    #[must_use]
    pub fn attribution(&self) -> Option<&crate::state::ConvergenceAttribution> {
        match self {
            Self::Attributed { attribution, .. } => Some(attribution),
            _ => None,
        }
    }
}

impl std::error::Error for SimulationError {}

/// The detail behind [`SimulationError::ResultSchemaMismatch`].
///
/// Serialized rather than mirrored into a separate worker-side type: a result
/// that fails its own schema is the same report on both sides of the browser
/// worker boundary, and the two registries are what a report consists of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultSchemaMismatch {
    /// The analysis whose result failed its schema.
    pub analysis: String,
    /// The sweep, frequency, or time point, where the failure has one.
    pub coordinate: Option<String>,
    /// The family of signals whose registry and payload disagree.
    pub signal_family: String,
    /// Both registries in their original order, because the order is part of
    /// the contract that was broken.
    pub expected_names: Vec<String>,
    pub actual_names: Vec<String>,
    pub expected_value_count: usize,
    pub actual_value_count: usize,
}

impl std::fmt::Display for ResultSchemaMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Result schema mismatch for {} analysis", self.analysis)?;
        if let Some(coordinate) = &self.coordinate {
            write!(f, " at {coordinate}")?;
        }
        write!(
            f,
            " in {}: expected names {:?} with {} value(s), got names {:?} with {} value(s)",
            self.signal_family,
            self.expected_names,
            self.expected_value_count,
            self.actual_names,
            self.actual_value_count
        )
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Run one specification through the real runner and return what the
    /// engine logged while it ran.
    ///
    /// Deliberately `run_simulation_thread_with_progress_observer` and not a
    /// hand-installed sink: what is under test is that *the runner* installs
    /// one for the duration of a run, which is the whole of the bridge on this
    /// side.
    #[cfg(not(target_arch = "wasm32"))]
    fn engine_log_of_run(spec: AnalysisSpec, netlist: &str) -> Vec<EngineLogLine> {
        crate::diagnostics::engine_log::studio_logger_for_tests();
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        let verbose = request_asked_for_verbose(&SimulationRequest::Spec {
            spec: Box::new(spec.clone()),
            options: Box::new(SpecExecutionOptions::default()),
        });
        let result = run_simulation_thread_with_progress_observer(
            SimulationRequest::Spec {
                spec: Box::new(spec),
                options: Box::new(SpecExecutionOptions::default()),
            },
            NetlistInput {
                measurement_references: Default::default(),
                netlist: netlist.to_owned(),
                source_path: None,
                project_veriloga_runtimes: Default::default(),
                dependencies: Default::default(),
                environment: None,
                stream_transient_samples: false,
            },
            Arc::new(Mutex::new(SimulationProgress::default())),
            Arc::new(AtomicBool::new(false)),
            RunStreams {
                engine_log: Some(RunLogSink::queued(Arc::clone(&queue), verbose)),
                ..RunStreams::default()
            },
        );
        result.expect("the specification reaches the engine and converges");
        crate::diagnostics::engine_log::lock_queue(&queue).drain()
    }

    /// The receipt names numbers that exist nowhere else.
    ///
    /// How many device noise sources were installed, and the seed they were
    /// drawn from, are decided inside the engine while the run is building —
    /// the form states a requested seed, the engine states the one it used —
    /// and until a run's log reached the Console the only reader of that
    /// sentence was a stderr stream a windowed application never shows.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_transient_noise_run_states_its_receipt_in_the_console() {
        const DECK: &str = "transient noise divider\n\
                            V1 in 0 DC 1\n\
                            R1 in out 10k\n\
                            R2 out 0 10k\n\
                            .end\n";
        let spec = AnalysisSpec::TransientNoise {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: 1.0e-9,
            seed: Some(4_242),
            noise_fmax: 1.0e9,
            noise_fmin: None,
            scale: 1.0,
            uic: false,
        };
        // The deck the run executes is the card the Studio writes, spliced in
        // by the same builder the Analyses page displays.
        let card =
            crate::simulation::controller::SimulationController::build_transient_noise_command(
                &spec,
            )
            .expect("the specification writes its card");
        let deck = DECK.replace(".end\n", &format!("{card}\n.end\n"));

        let lines = engine_log_of_run(spec, &deck);
        let receipt = lines
            .iter()
            .find(|line| line.message.starts_with("Transient noise:"))
            .unwrap_or_else(|| {
                panic!(
                    "the run stated no transient-noise receipt: {:?}",
                    lines.iter().map(|line| &line.message).collect::<Vec<_>>()
                )
            });
        assert_eq!(receipt.severity, crate::diagnostics::LogSeverity::Info);
        assert!(
            receipt.message.contains("from seed 4242"),
            "the receipt names the seed the form showed: {}",
            receipt.message
        );
        assert!(
            receipt.message.contains("device noise source(s)"),
            "the receipt counts what it injected: {}",
            receipt.message
        );
    }

    /// A one-tone nonlinear HB solve traces its Newton iterations when the
    /// form asks for them, and traces nothing when it does not.
    ///
    /// What this fixture's solve actually writes is one `log::debug!` per
    /// Newton iteration of the nonlinear solve it runs, plus the presolve's
    /// account of the initial guess — measured, not assumed. The engine has
    /// other traces behind `HbConfig::verbose` itself (the certified exact
    /// matrix-free step, the Krylov residual), and they are admitted here by
    /// the same rule; this deck simply converges before it needs them.
    ///
    /// The quiet half is the half that matters. `Debug` is what the engine
    /// writes its whole solver trace at, and the run that did not ask for it
    /// captures none of it — so the switch on the form is what decided, not
    /// the bridge deciding for everyone.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn an_hb_verbose_run_traces_its_newton_iterations_in_the_console() {
        const DECK: &str = "hb verbose diode\n\
                            VDRIVE in 0 DC 1\n\
                            R1 in out 100\n\
                            D1 out 0 DMOD\n\
                            .model DMOD D (IS=1u N=1.48)\n\
                            .end\n";
        let spec = |verbose| AnalysisSpec::HarmonicBalance {
            tones: vec![crate::simulation::multi_run::HbToneSpec::new(1.0e6, 1)],
            reltol: 1.0e-6,
            abstol: 1.0e-12,
            max_iterations: 100,
            damping: 1.0,
            min_damping: 0.01,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose,
        };

        let traced = engine_log_of_run(spec(true), DECK);
        let iterations = traced
            .iter()
            .filter(|line| line.severity == crate::diagnostics::LogSeverity::Debug)
            .filter(|line| {
                line.message.starts_with("Newton iter") || line.message.starts_with("HB ")
            })
            .count();
        assert!(
            iterations >= 2,
            "a verbose HB solve traced {iterations} solver iterations: {:?}",
            traced.iter().map(|line| &line.message).collect::<Vec<_>>()
        );

        let quiet = engine_log_of_run(spec(false), DECK);
        assert!(
            quiet
                .iter()
                .all(|line| line.severity != crate::diagnostics::LogSeverity::Debug),
            "an ordinary HB solve traced its solver anyway: {:?}",
            quiet.iter().map(|line| &line.message).collect::<Vec<_>>()
        );
    }

    /// Only the two analyses whose card carries `VERBOSE=` can ask for a trace.
    #[test]
    fn only_a_periodic_request_can_ask_the_engine_to_trace_itself() {
        let options = Box::new(SpecExecutionOptions::default());
        let spec = |spec| SimulationRequest::Spec {
            spec: Box::new(spec),
            options: options.clone(),
        };
        assert!(!request_asked_for_verbose(&SimulationRequest::Config(
            Box::new(AnalysisConfig::dc_op())
        )));
        assert!(!request_asked_for_verbose(&spec(AnalysisSpec::LegacyDcOp)));

        let pss = |verbose| AnalysisSpec::Pss {
            method: crate::simulation::multi_run::PssMethod::Shooting,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["V1".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-7,
            oscillator_mode: false,
            oscillator_node: None,
            num_harmonics: 8,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose,
        };
        assert!(!request_asked_for_verbose(&spec(pss(false))));
        assert!(request_asked_for_verbose(&spec(pss(true))));
    }

    #[test]
    fn poll_result_returns_pending_result_once() {
        let mut runner = SimulationRunner::new();

        runner
            .store_pending_result(Ok(SimulationResult::default()))
            .expect("stores pending result");

        let first = runner
            .poll_result()
            .expect("pending result should be delivered");
        assert!(matches!(
            first,
            Ok(SimulationResult::MeasurementsOnly { .. })
        ));
        assert!(
            runner.poll_result().is_none(),
            "pending result should be consumed"
        );
    }

    #[test]
    fn start_rejects_unpolled_pending_result() {
        let mut runner = SimulationRunner::new();

        runner
            .store_pending_result(Err(SimulationError::InvalidConfig(
                "inline failure".to_string(),
            )))
            .expect("stores pending error");

        let start_result = runner.start(AnalysisConfig::dc_op(), String::new());
        assert_eq!(start_result, Err(SimulationError::AlreadyRunning));

        let pending = runner
            .poll_result()
            .expect("pending error should remain available");
        assert!(matches!(
            pending,
            Err(SimulationError::InvalidConfig(message)) if message == "inline failure"
        ));
        assert!(
            runner.poll_result().is_none(),
            "pending error should be consumed"
        );
    }

    #[test]
    fn start_rejects_unpolled_finished_thread_result() {
        let mut runner = SimulationRunner::new();
        runner.thread_handle = Some(std::thread::spawn(|| Ok(SimulationResult::default())));
        while !runner
            .thread_handle
            .as_ref()
            .expect("thread handle is set")
            .is_finished()
        {
            std::thread::yield_now();
        }

        let start_result = runner.start(AnalysisConfig::dc_op(), String::new());
        assert_eq!(start_result, Err(SimulationError::AlreadyRunning));

        let pending = runner
            .poll_result()
            .expect("finished thread result should remain available")
            .expect("thread result should be ok");
        assert!(matches!(pending, SimulationResult::MeasurementsOnly { .. }));
        assert!(
            runner.poll_result().is_none(),
            "thread result should be consumed"
        );
    }

    #[test]
    fn design_replacement_reset_isolates_old_native_handles() {
        let mut runner = SimulationRunner::new();
        let old_abort = Arc::clone(&runner.abort_flag);
        let old_progress = Arc::clone(&runner.progress);

        runner.reset_for_design_replacement();

        assert!(
            old_abort.load(Ordering::SeqCst),
            "old worker handle should remain aborted"
        );
        assert!(
            !runner.is_aborted(),
            "future runs should start from a fresh abort flag"
        );
        assert!(!Arc::ptr_eq(&old_abort, &runner.abort_flag));
        assert!(!Arc::ptr_eq(&old_progress, &runner.progress));
    }

    #[test]
    fn runner_signal_streams_live_current_impulses_as_sparse_accepted_suffixes() {
        use rspice_core::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};
        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        let signal = RunnerSignal {
            abort_flag: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(Mutex::new(SimulationProgress::default())),
            progress_observer: None,
            transient_samples: Some(Arc::clone(&samples)),
            transient_sample_observer: None,
            published_events: Mutex::default(),
        };
        let mut traces = vec![CurrentImpulseTrace {
            owner: CurrentImpulseOwner::DeviceLead {
                device_name: "Q1".into(),
                parameter: "ic".into(),
            },
            complete: true,
            points: vec![],
        }];
        let mut received = CurrentImpulseBuffer::default();
        for index in 0..4 {
            if index == 1 || index == 3 {
                traces[0].points.push(CurrentImpulsePoint {
                    time: index as f64 - 0.2,
                    charge_coulombs: if index == 1 { -0.002 } else { 0.004 },
                });
            }
            rspice_core::abort_signal::AbortSignal::observe_transient_sample(
                &signal,
                rspice_core::abort_signal::TransientSample {
                    time: &[0.0, index as f64],
                    node_names: &[],
                    node_voltages: &[],
                    branch_names: &[],
                    branch_currents: &[],
                    current_impulses: Some(&traces),
                    digital_values: &[],
                    digital_buses: &[],
                    real_values: &[],
                },
            );
            let mut drained = samples.lock().unwrap().drain();
            let delta = drained.pop().unwrap().current_impulses.unwrap();
            if index == 2 {
                assert!(
                    delta.traces.is_empty(),
                    "unchanged cumulative history is not retransmitted"
                );
            }
            if index == 3 {
                assert_eq!(
                    delta.traces[0].points.len(),
                    1,
                    "only the new suffix crosses the boundary"
                );
            }
            received.ingest(delta);
        }
        let history = received.history().unwrap();
        history.validate().unwrap();
        assert!(history.delivery_complete);
        assert_eq!(history.traces, traces);
    }

    #[test]
    fn runner_signal_publishes_only_the_latest_committed_transient_point() {
        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        let signal = RunnerSignal {
            abort_flag: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(Mutex::new(SimulationProgress::default())),
            progress_observer: None,
            transient_samples: Some(Arc::clone(&samples)),
            transient_sample_observer: None,
            published_events: Mutex::default(),
        };
        let result = rspice_core::engine::TransientResult {
            current_impulses: None,
            time: vec![0.0, 2.5e-9],
            step_sizes: vec![0.0, 2.5e-9],
            voltages: vec![vec![0.0, 1.25], Vec::new()],
            branch_currents: vec![vec![0.0, -2.0e-3]],
            num_nodes: 2,
            node_names: vec!["out".to_owned(), "discarded".to_owned()],
            branch_names: vec!["V1".to_owned()],
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        rspice_core::abort_signal::AbortSignal::observe_transient_sample(
            &signal,
            rspice_core::abort_signal::TransientSample {
                time: &result.time,
                node_names: &result.node_names,
                node_voltages: &result.voltages,
                branch_names: &result.branch_names,
                branch_currents: &result.branch_currents,
                current_impulses: result.current_impulses.as_deref(),
                digital_values: &[],
                digital_buses: &[],
                real_values: &[],
            },
        );

        let sample = samples.lock().unwrap().drain().remove(0);
        assert_eq!(sample.time, 2.5e-9);
        assert_eq!(sample.waveforms.len(), 2);
        assert_eq!(sample.waveforms[0].name, "out");
        assert_eq!(sample.waveforms[0].value, 1.25);
        assert_eq!(sample.waveforms[0].y_unit, "V");
        assert_eq!(sample.waveforms[1].name, "I(V1)");
        assert_eq!(sample.waveforms[1].value, -2.0e-3);
        assert_eq!(sample.waveforms[1].y_unit, "A");
        assert!(sample.events.is_empty());
        assert!(sample.real_events.is_empty());
    }

    #[test]
    fn runner_signal_publishes_event_nodes_only_when_their_value_changes() {
        use rspice_core::abort_signal::{AbortSignal as _, DigitalEventCode, TransientSample};

        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        let signal = RunnerSignal {
            abort_flag: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(Mutex::new(SimulationProgress::default())),
            progress_observer: None,
            transient_samples: Some(Arc::clone(&samples)),
            transient_sample_observer: None,
            published_events: Mutex::default(),
        };
        let node_names = vec!["clk".to_owned(), "d".to_owned(), "vsense".to_owned()];
        let voltages = vec![vec![0.0, 0.0, 0.0], Vec::new(), Vec::new()];
        // Node 4 has no entry in the node table, so it cannot be named and is
        // never reported; nodes 1 and 2 are digital, node 3 is real-valued.
        let observe = |time: &[f64],
                       digital: &[(rspice_core::NodeId, DigitalEventCode)],
                       real: &[(rspice_core::NodeId, f64)]| {
            signal.observe_transient_sample(TransientSample {
                time,
                node_names: &node_names,
                node_voltages: &voltages,
                branch_names: &[],
                branch_currents: &[],
                current_impulses: None,
                digital_values: digital,
                digital_buses: &[],
                real_values: real,
            });
        };

        observe(
            &[0.0],
            &[(1, DigitalEventCode(0)), (2, DigitalEventCode(12))],
            &[(3, 1.5), (4, 2.5)],
        );
        observe(
            &[0.0, 1.0e-9],
            &[(1, DigitalEventCode(0)), (2, DigitalEventCode(12))],
            &[(3, 1.5), (4, 2.5)],
        );
        observe(
            &[0.0, 1.0e-9, 2.0e-9],
            &[(1, DigitalEventCode(1)), (2, DigitalEventCode(12))],
            &[(3, 1.5), (4, 9.0)],
        );

        let queued = samples.lock().expect("live queue").drain();
        assert_eq!(queued.len(), 3);

        assert_eq!(
            queued[0].events,
            vec![
                TransientDigitalEventSample {
                    name: "clk".to_owned(),
                    value_code: 0,
                },
                TransientDigitalEventSample {
                    name: "d".to_owned(),
                    value_code: 12,
                },
            ]
        );
        assert_eq!(
            queued[0].real_events,
            vec![TransientRealEventSample {
                name: "vsense".to_owned(),
                value: 1.5,
            }]
        );

        assert!(
            queued[1].events.is_empty() && queued[1].real_events.is_empty(),
            "an unchanged committed state must publish nothing"
        );

        assert_eq!(
            queued[2].events,
            vec![TransientDigitalEventSample {
                name: "clk".to_owned(),
                value_code: 1,
            }]
        );
        assert!(
            queued[2].real_events.is_empty(),
            "the only real node that changed has no name in the node table"
        );
    }

    #[test]
    fn live_transient_queue_is_bounded_for_suspended_ui_consumers() {
        let samples = Arc::new(Mutex::new(LiveTransientQueue::default()));
        for index in 0..MAX_PENDING_LIVE_TRANSIENT_SAMPLES + 17 {
            push_live_transient_sample(
                &samples,
                TransientSampleDelta {
                    current_impulses: None,
                    time: index as f64,
                    waveforms: Vec::new(),
                    events: Vec::new(),
                    real_events: Vec::new(),
                    buses: Vec::new(),
                },
            );
        }

        let samples = samples.lock().expect("live queue");
        assert_eq!(samples.samples.len(), MAX_PENDING_LIVE_TRANSIENT_SAMPLES);
        assert_eq!(samples.samples.front().expect("oldest retained").time, 17.0);
        assert_eq!(
            samples.samples.back().expect("latest retained").time,
            (MAX_PENDING_LIVE_TRANSIENT_SAMPLES + 16) as f64
        );
    }

    #[test]
    fn initial_status_uses_specific_config_statuses() {
        let noise = SimulationRequest::Config(Box::new(AnalysisConfig::Noise(
            crate::simulation::config::NoiseAnalysisConfig {
                output_node: "out".to_string(),
                reference_node: "0".to_string(),
                input_source: "V1".to_string(),
                sweep_type: crate::simulation::config::AcSweepType::Decade,
                num_points: 10,
                start_freq: 12.0,
                stop_freq: 34.0,
                ..crate::simulation::config::NoiseAnalysisConfig::default()
            },
        )));
        assert_eq!(
            initial_status_for_request(&noise),
            SimulationStatus::NoiseAnalysis {
                freq: 12.0,
                stop_freq: 34.0
            }
        );

        let pole_zero = SimulationRequest::Config(Box::new(AnalysisConfig::PoleZero(
            crate::simulation::config::PoleZeroConfig {
                input_node: "in".to_string(),
                input_ref: "0".to_string(),
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                transfer_type: "VOL".to_string(),
                analysis_type: crate::simulation::config::PzAnalysisType::PoleZero,
            },
        )));
        assert_eq!(
            initial_status_for_request(&pole_zero),
            SimulationStatus::PoleZero
        );

        let sensitivity = SimulationRequest::Config(Box::new(AnalysisConfig::Sensitivity(
            crate::simulation::config::SensitivityConfig {
                output_var: "V(out)".to_string(),
                ..Default::default()
            },
        )));
        assert_eq!(
            initial_status_for_request(&sensitivity),
            SimulationStatus::Sensitivity
        );
    }

    #[test]
    fn initial_status_uses_spec_execution_options_for_rf_analyses() {
        let tf = SimulationRequest::Spec {
            spec: Box::new(AnalysisSpec::Tf {
                input_source: "V1".to_owned(),
                output_expression: "V(out)".to_owned(),
                transfer_gain: true,
                input_resistance: true,
                output_resistance: true,
                normalization: crate::simulation::multi_run::TfNormalization::None,
                accuracy: crate::simulation::multi_run::TfAccuracy::Balanced,
            }),
            options: Box::new(SpecExecutionOptions::default()),
        };
        assert_eq!(
            initial_status_for_request(&tf),
            SimulationStatus::DcOperatingPoint
        );

        let pnoise = SimulationRequest::Spec {
            spec: Box::new(AnalysisSpec::Pnoise),
            options: Box::new(SpecExecutionOptions {
                pnoise: Some(crate::services::simulation_runner::PnoiseRunConfig {
                    start_freq: 3.0,
                    stop_freq: 30.0,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };
        assert_eq!(
            initial_status_for_request(&pnoise),
            SimulationStatus::NoiseAnalysis {
                freq: 3.0,
                stop_freq: 30.0
            }
        );
    }

    #[test]
    fn production_runner_surface_exposes_only_the_opaque_prepared_start() {
        let source = include_str!("runner.rs");
        let controller_source = include_str!("controller.rs");
        // Split boundary-sensitive search strings so this test's own source
        // cannot satisfy or invalidate the assertions.
        let prepared_signature = ["pub(in crate::simulation) fn ", "start_prepared"].concat();
        assert_eq!(source.match_indices(&prepared_signature).count(), 1);
        for forbidden in [
            ["pub(crate) fn ", "start_request"].concat(),
            ["pub(in crate::simulation) fn ", "start_request"].concat(),
        ] {
            assert!(
                !source.contains(&forbidden),
                "raw start request must remain private: {forbidden}"
            );
        }

        for signature in [
            "\n    fn start(&mut self",
            "\n    fn start_with_source_path(",
        ] {
            let index = source
                .find(signature)
                .unwrap_or_else(|| panic!("missing raw test helper {signature}"));
            let prefix = &source[..index];
            assert_eq!(
                prefix.lines().next_back().map(str::trim),
                Some("#[cfg(test)]"),
                "raw start helper must remain test-only: {signature}",
            );
        }

        assert_eq!(
            controller_source.match_indices(".start_prepared(").count(),
            1,
            "the controller must have one opaque task dispatch point"
        );
        for forbidden in ["self.runner.start(", "self.runner.start_with_source_path("] {
            assert!(
                !controller_source.contains(forbidden),
                "controller bypasses the authorized dispatch boundary: {forbidden}"
            );
        }
    }

    #[test]
    fn prepared_task_readiness_rejects_active_and_finished_unpolled_threads() {
        let mut runner = SimulationRunner::new();
        let (release_sender, release_receiver) = std::sync::mpsc::channel::<()>();
        runner.thread_handle = Some(std::thread::spawn(move || {
            release_receiver.recv().expect("release active worker");
            Err(SimulationError::Aborted)
        }));

        assert!(runner.is_running());
        assert!(!runner.can_accept_prepared_task());
        release_sender.send(()).expect("release worker");

        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while runner
            .thread_handle
            .as_ref()
            .is_some_and(|handle| !handle.is_finished())
        {
            assert!(
                std::time::Instant::now() < deadline,
                "test worker did not finish"
            );
            std::thread::yield_now();
        }

        assert!(!runner.is_running());
        assert!(runner.has_unpolled_result());
        assert!(!runner.can_accept_prepared_task());
        assert!(matches!(
            runner.poll_result(),
            Some(Err(SimulationError::Aborted))
        ));
        assert!(runner.can_accept_prepared_task());
    }
}
