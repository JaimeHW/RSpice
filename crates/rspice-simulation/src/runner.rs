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

use crate::error::SimulationError;
use crate::execution_options::SpecExecutionOptions;
pub(crate) use rspice_simulation_contract::worker_protocol::AnalysisExecutionEnvironment;

use crate::engine_log::{EngineLogLine, EngineLogQueue, RunLogSink};

use super::execution::ResolvedTaskDispatch;
use super::status::{SimulationProgress, SimulationStatus};
use crate::execution_artifact::ResolvedExecutionDependencies;
use crate::results::SimulationResult;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::AnalysisConfig;

#[cfg(test)]
pub(crate) mod monte_carlo_checkpoint_tests;
use crate::live_transient::{
    LiveTransientPublisher, LiveTransientQueue, TransientSampleDelta, TransientSampleObserver,
};
use crate::monte_carlo_checkpoint::{CheckpointObserver, CheckpointQueue, replace_checkpoint};

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
    project_veriloga_runtimes: crate::veriloga::PreparedVerilogARuntimeSet,
    measurement_references: crate::measurement_references::PreparedMeasurementReferences,
    dependencies: ResolvedExecutionDependencies,
    environment: Option<AnalysisExecutionEnvironment>,
    stream_transient_samples: bool,
    execution_limits: rspice_core::ResourceLimits,
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

    pub fn engine_availability(&self) -> super::status::EngineAvailability {
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
    pub fn set_engine_wakeup(&mut self, wakeup: Arc<dyn Fn() + Send + Sync>) {
        self.worker_handle.set_wakeup(wakeup);
    }

    /// Observe backend qualification without granting access to worker state.
    #[cfg(target_arch = "wasm32")]
    pub fn set_engine_capability_observer(
        &mut self,
        observer: Arc<dyn Fn(Option<crate::status::EngineJitObservation>) + Send + Sync>,
    ) {
        self.worker_handle.set_capability_observer(observer);
    }

    /// Retry backend startup without queuing a simulation or disturbing a
    /// completion that the controller still owns.
    pub fn retry_engine_startup(&mut self) -> Result<(), SimulationError> {
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
    pub fn drain_transient_samples(&self) -> Vec<TransientSampleDelta> {
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
    pub fn drain_engine_log(&self) -> Vec<EngineLogLine> {
        crate::engine_log::lock_queue(&self.engine_log).drain()
    }

    /// Latest complete journal, available independently of terminal success.
    pub fn take_monte_carlo_checkpoint(&self) -> Option<Arc<[u8]>> {
        self.monte_carlo_checkpoint
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
    }

    /// Abort and discard all runner-local completion/progress state.
    ///
    /// Native worker threads cannot be force-killed, but setting the shared
    /// abort flag and dropping the join handle detaches stale work so its
    /// eventual result cannot be polled into a replacement document.
    pub fn reset_for_design_replacement(&mut self) {
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
    pub fn can_accept_prepared_task(&self) -> bool {
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
    pub fn start_prepared(
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
            execution_limits,
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
                execution_limits,
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
                execution_limits: rspice_core::ResourceLimits::default(),
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
        crate::engine_log::lock_queue(&self.engine_log).clear();
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
#[cfg(any(not(target_arch = "wasm32"), feature = "browser-worker", test))]
pub(in crate::runner) fn request_asked_for_verbose(request: &SimulationRequest) -> bool {
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
                    crate::study::StudyAnalysis::Native(AnalysisSpec::HarmonicBalance {
                        verbose,
                        ..
                    }) => *verbose,
                    crate::study::StudyAnalysis::Pss(pss) => {
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
pub(in crate::runner) struct RunStreams {
    pub(in crate::runner) progress_observer: Option<ProgressObserver>,
    pub(in crate::runner) transient_samples: Option<Arc<Mutex<LiveTransientQueue>>>,
    pub(in crate::runner) transient_sample_observer: Option<TransientSampleObserver>,
    pub(in crate::runner) engine_log: Option<RunLogSink>,
    pub(in crate::runner) monte_carlo_checkpoint: Option<CheckpointQueue>,
    pub(in crate::runner) checkpoint_observer: Option<CheckpointObserver>,
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

pub(in crate::runner) type ProgressObserver = fn(&SimulationProgress);

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
    live_transient: LiveTransientPublisher,
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
        self.live_transient.observe(sample);
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
            rspice_simulation_contract::analysis_spec::PssMethod::Shooting => {
                SimulationStatus::Transient {
                    time: 0.0,
                    stop_time: positive_period(*fundamental_freq),
                }
            }
            rspice_simulation_contract::analysis_spec::PssMethod::HarmonicBalance => {
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
        AnalysisSpec::Optimization { .. } => SimulationStatus::PostProcessing,
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

pub(in crate::runner) fn run_simulation_thread_with_progress_observer(
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
    let _engine_log = engine_log.map(crate::engine_log::install);

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

    if let SimulationRequest::Spec { spec, .. } = &request
        && let Some(reason) =
            crate::execution::execution_resource_policy_blocker(spec, input.execution_limits)
    {
        return Err(SimulationError::InvalidConfig(reason.into()));
    }

    // Create engine bridge
    // The source is already expanded; root admission was checked in preflight.
    let mut limits = input.execution_limits;
    limits.max_netlist_bytes = limits.max_expanded_source_bytes;
    let bridge = EngineBridge::new()
        .with_resource_limits(limits)
        .with_measurement_references(input.measurement_references.clone());

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
        live_transient: LiveTransientPublisher::new(transient_samples, transient_sample_observer),
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
            spec::run_spec_request_in_context(
                &bridge,
                *spec,
                *options,
                spec::SpecExecutionContext {
                    netlist: &input.netlist,
                    source_path: input.source_path.as_deref(),
                    dependencies: &input.dependencies,
                    environment: input.environment,
                    abort_flag: &signal,
                    checkpoint_observer,
                },
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
// Tests
//=============================================================================

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub use worker_contract::{
    cancel_prepared_wasm_jit_request_value, prepare_wasm_jit_request_value,
    run_prepared_wasm_jit_request_value, run_worker_request_value,
};

pub use crate::engine_services::{TfRunConfig, infer_tf_run_config};

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
        struct CaptureLogger;
        impl log::Log for CaptureLogger {
            fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
                crate::engine_log::admits(metadata)
            }
            fn log(&self, record: &log::Record<'_>) {
                crate::engine_log::offer(record);
            }
            fn flush(&self) {}
        }
        static INSTALL: std::sync::Once = std::sync::Once::new();
        INSTALL.call_once(|| {
            log::set_logger(&CaptureLogger).expect("install runtime test logger");
            crate::engine_log::note_stderr_level(log::LevelFilter::Off);
        });
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        let verbose = request_asked_for_verbose(&SimulationRequest::Spec {
            spec: Box::new(spec.clone()),
            options: Box::new(SpecExecutionOptions::default()),
        });
        let dependencies = if matches!(spec, AnalysisSpec::HarmonicBalance { .. }) {
            super::pvt_point_evidence::op_dependencies(
                netlist,
                netlist,
                netlist,
                Default::default(),
            )
        } else {
            ResolvedExecutionDependencies::default()
        };
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
                dependencies,
                environment: None,
                stream_transient_samples: false,
                execution_limits: rspice_core::ResourceLimits::default(),
            },
            Arc::new(Mutex::new(SimulationProgress::default())),
            Arc::new(AtomicBool::new(false)),
            RunStreams {
                engine_log: Some(RunLogSink::queued(Arc::clone(&queue), verbose)),
                ..RunStreams::default()
            },
        );
        result.expect("the specification reaches the engine and converges");
        crate::engine_log::lock_queue(&queue).drain()
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
        let card = crate::analysis_preparation::build_transient_noise_command(&spec)
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
        assert_eq!(
            receipt.severity,
            rspice_app_types::diagnostics::LogSeverity::Info
        );
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
    /// The fixture requests Krylov corrections, whose iteration and residual
    /// traces are gated by `HbConfig::verbose`. Its sinusoidal drive requires
    /// nonlinear corrections after the explicit operating-point startup.
    ///
    /// The quiet half is the half that matters. `Debug` is what the engine
    /// writes its whole solver trace at, and the run that did not ask for it
    /// captures none of it — so the switch on the form is what decided, not
    /// the bridge deciding for everyone.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn an_hb_verbose_run_traces_its_newton_iterations_in_the_console() {
        const DECK: &str = "hb verbose diode\n\
                            VDRIVE in 0 DC 1 SIN(1 0.1 1MEG)\n\
                            R1 in out 100\n\
                            D1 out 0 DMOD\n\
                            .model DMOD D (IS=1u N=1.48)\n\
                            .end\n";
        let spec = |verbose| AnalysisSpec::HarmonicBalance {
            tones: vec![rspice_simulation_contract::analysis_spec::HbToneSpec::new(
                1.0e6, 1,
            )],
            reltol: 1.0e-6,
            abstol: 1.0e-12,
            max_iterations: 100,
            damping: 1.0,
            min_damping: 0.01,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: true,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose,
        };

        let traced = engine_log_of_run(spec(true), DECK);
        let iterations = traced
            .iter()
            .filter(|line| line.severity == rspice_app_types::diagnostics::LogSeverity::Debug)
            .filter(|line| {
                line.message.starts_with("HB exact matrix-free solve:")
                    || line.message.starts_with("HB Krylov solve:")
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
                .all(|line| line.severity != rspice_app_types::diagnostics::LogSeverity::Debug),
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
            method: rspice_simulation_contract::analysis_spec::PssMethod::Shooting,
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
    fn initial_status_uses_specific_config_statuses() {
        let noise = SimulationRequest::Config(Box::new(AnalysisConfig::Noise(
            rspice_simulation_contract::config::NoiseAnalysisConfig {
                output_node: "out".to_string(),
                reference_node: "0".to_string(),
                input_source: "V1".to_string(),
                sweep_type: rspice_simulation_contract::config::AcSweepType::Decade,
                num_points: 10,
                start_freq: 12.0,
                stop_freq: 34.0,
                ..rspice_simulation_contract::config::NoiseAnalysisConfig::default()
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
            rspice_simulation_contract::config::PoleZeroConfig {
                input_node: "in".to_string(),
                input_ref: "0".to_string(),
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                transfer_type: "VOL".to_string(),
                analysis_type: rspice_simulation_contract::config::PzAnalysisType::PoleZero,
            },
        )));
        assert_eq!(
            initial_status_for_request(&pole_zero),
            SimulationStatus::PoleZero
        );

        let sensitivity = SimulationRequest::Config(Box::new(AnalysisConfig::Sensitivity(
            rspice_simulation_contract::config::SensitivityConfig {
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
                normalization: rspice_simulation_contract::analysis_spec::TfNormalization::None,
                accuracy: rspice_simulation_contract::analysis_spec::TfAccuracy::Balanced,
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
                pnoise: Some(crate::periodic::PnoiseRunConfig {
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
