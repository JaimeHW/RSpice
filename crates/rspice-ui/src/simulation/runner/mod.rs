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

use super::config::AnalysisConfig;
use super::multi_run::AnalysisSpec;
use super::results::SimulationResult;
use super::status::{SimulationProgress, SimulationStatus};

mod spec;

/// Optional execution overrides for spec-driven analyses.
#[derive(Debug, Clone, Default)]
pub struct SpecExecutionOptions {
    pub temp: Option<crate::services::simulation_runner::TempRunConfig>,
    pub corner: Option<crate::services::simulation_runner::CornerRunConfig>,
    pub pac: Option<crate::services::simulation_runner::PacRunConfig>,
    pub pxf: Option<crate::services::simulation_runner::PxfRunConfig>,
    pub tf: Option<crate::services::simulation_runner::TfRunConfig>,
    pub pnoise: Option<crate::services::simulation_runner::PnoiseRunConfig>,
    pub pstb: Option<crate::services::simulation_runner::PstbRunConfig>,
}

//=============================================================================
// Simulation Runner
//=============================================================================

#[derive(Debug, Clone)]
enum SimulationRequest {
    Config(Box<AnalysisConfig>),
    Spec {
        spec: Box<AnalysisSpec>,
        options: Box<SpecExecutionOptions>,
    },
}

#[derive(Debug, Clone)]
struct NetlistInput {
    netlist: String,
    source_path: Option<PathBuf>,
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

    /// Current simulation thread handle
    thread_handle: Option<JoinHandle<Result<SimulationResult, SimulationError>>>,

    /// Completed inline result waiting for the controller to poll it.
    pending_result: Option<Result<SimulationResult, SimulationError>>,
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
            thread_handle: None,
            pending_result: None,
        }
    }

    /// Check if a simulation is currently running
    pub fn is_running(&self) -> bool {
        if let Some(ref handle) = self.thread_handle {
            !handle.is_finished()
        } else {
            false
        }
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

    /// Get full progress information
    pub fn progress(&self) -> SimulationProgress {
        lock_progress(&self.progress, "SimulationRunner::progress").clone()
    }

    /// Abort current simulation
    pub fn abort(&self) {
        self.abort_flag.store(true, Ordering::SeqCst);
    }

    /// Check if aborted
    pub fn is_aborted(&self) -> bool {
        self.abort_flag.load(Ordering::SeqCst)
    }

    /// Poll for completion and get result
    ///
    /// Returns `Some(result)` if simulation completed, `None` if still running or no simulation.
    pub fn poll_result(&mut self) -> Option<Result<SimulationResult, SimulationError>> {
        if let Some(result) = self.pending_result.take() {
            return Some(result);
        }

        // Check if thread is finished
        let is_finished = self.thread_handle.as_ref().is_some_and(|h| h.is_finished());

        if is_finished {
            // Take the handle and join
            if let Some(handle) = self.thread_handle.take() {
                match handle.join() {
                    Ok(result) => {
                        return Some(result);
                    }
                    Err(_) => {
                        return Some(Err(SimulationError::ThreadPanic));
                    }
                }
            }
        }

        None
    }

    fn has_unpolled_result(&self) -> bool {
        self.pending_result.is_some()
            || self
                .thread_handle
                .as_ref()
                .is_some_and(|handle| handle.is_finished())
    }

    #[cfg(any(target_arch = "wasm32", test))]
    fn store_pending_result(
        &mut self,
        result: Result<SimulationResult, SimulationError>,
    ) -> Result<(), SimulationError> {
        if self.pending_result.is_some() || self.thread_handle.is_some() {
            return Err(SimulationError::AlreadyRunning);
        }

        self.pending_result = Some(result);
        Ok(())
    }

    /// Start a simulation with the given configuration
    ///
    /// Returns error if a simulation is already running.
    pub fn start(
        &mut self,
        config: AnalysisConfig,
        netlist: String,
    ) -> Result<(), SimulationError> {
        self.start_with_source_path(config, netlist, None)
    }

    /// Start a simulation with the given configuration and source path.
    pub fn start_with_source_path(
        &mut self,
        config: AnalysisConfig,
        netlist: String,
        source_path: Option<PathBuf>,
    ) -> Result<(), SimulationError> {
        self.start_request(
            SimulationRequest::Config(Box::new(config)),
            NetlistInput {
                netlist,
                source_path,
            },
        )
    }

    /// Start a simulation from strongly-typed analysis spec.
    pub fn start_spec(
        &mut self,
        spec: AnalysisSpec,
        netlist: String,
    ) -> Result<(), SimulationError> {
        self.start_spec_with_options_with_source_path(
            spec,
            netlist,
            SpecExecutionOptions::default(),
            None,
        )
    }

    /// Start a simulation from strongly-typed analysis spec with explicit execution options.
    pub fn start_spec_with_options(
        &mut self,
        spec: AnalysisSpec,
        netlist: String,
        options: SpecExecutionOptions,
    ) -> Result<(), SimulationError> {
        self.start_spec_with_options_with_source_path(spec, netlist, options, None)
    }

    /// Start a simulation from strongly-typed analysis spec with explicit
    /// execution options and a source path for relative include resolution.
    pub fn start_spec_with_options_with_source_path(
        &mut self,
        spec: AnalysisSpec,
        netlist: String,
        options: SpecExecutionOptions,
        source_path: Option<PathBuf>,
    ) -> Result<(), SimulationError> {
        self.start_request(
            SimulationRequest::Spec {
                spec: Box::new(spec),
                options: Box::new(options),
            },
            NetlistInput {
                netlist,
                source_path,
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
        self.abort_flag.store(false, Ordering::SeqCst);
        {
            let mut progress = lock_progress(&self.progress, "SimulationRunner::start_request");
            *progress = SimulationProgress::new();
        }

        // Clone Arcs for the thread
        let progress = Arc::clone(&self.progress);
        let abort_flag = Arc::clone(&self.abort_flag);

        // Spawn simulation thread with real engine. Browser builds have no
        // threads: solve inline on the UI thread (demo-scale decks finish in
        // milliseconds; a Web Worker is the planned home) — progress state is
        // final by the time this returns, and the absent JoinHandle is fine
        // because completion is observed through `progress`, not the handle.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let handle = std::thread::spawn(move || {
                run_simulation_thread(request, input, progress, abort_flag)
            });
            self.thread_handle = Some(handle);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result = run_simulation_thread(request, input, progress, abort_flag);
            self.store_pending_result(result)?;
        }
        Ok(())
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&mut self, netlist: String) -> Result<(), SimulationError> {
        self.start(AnalysisConfig::DcOp, netlist)
    }

    /// Run DC operating point analysis with a source path for relative include
    /// resolution.
    pub fn run_dc_op_with_source_path(
        &mut self,
        netlist: String,
        source_path: Option<PathBuf>,
    ) -> Result<(), SimulationError> {
        self.start_with_source_path(AnalysisConfig::DcOp, netlist, source_path)
    }
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

/// Bridges the engine's abort/progress hook onto the runner's shared state:
/// polls the UI abort flag and folds the reported completed fraction back
/// into the status line at the engine's abort-poll cadence.
struct RunnerSignal {
    abort_flag: Arc<AtomicBool>,
    progress: Arc<Mutex<SimulationProgress>>,
}

impl rspice_core::abort_signal::AbortSignal for RunnerSignal {
    fn is_aborted(&self) -> bool {
        self.abort_flag.load(Ordering::SeqCst)
    }

    fn observe_progress(&self, fraction: f64) {
        let mut p = lock_progress(&self.progress, "observe_progress");
        p.observe_engine_fraction(fraction);
    }
}

/// Simulation execution in background thread
///
/// Runs the actual rspice-core simulation engine.
fn run_simulation_thread(
    request: SimulationRequest,
    input: NetlistInput,
    progress: Arc<Mutex<SimulationProgress>>,
    abort_flag: Arc<AtomicBool>,
) -> Result<SimulationResult, SimulationError> {
    use super::engine_bridge::EngineBridge;

    // Update status: parsing
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(parse)");
        p.update_status(SimulationStatus::Parsing);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = lock_progress(&progress, "run_simulation_thread(abort-after-parse)");
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Create engine bridge
    let bridge = EngineBridge::new();

    // Update status: building
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(build)");
        p.update_status(SimulationStatus::Building);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = lock_progress(&progress, "run_simulation_thread(abort-after-build)");
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Update status based on analysis type
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(status-by-analysis)");
        match &request {
            SimulationRequest::Config(config) => match config.as_ref() {
                AnalysisConfig::DcOp => p.update_status(SimulationStatus::DcOperatingPoint),
                AnalysisConfig::DcSweep(dc) => p.update_status(SimulationStatus::DcSweep {
                    source: dc.source.clone(),
                    progress: 0.0,
                }),
                AnalysisConfig::Transient(tran) => p.update_status(SimulationStatus::Transient {
                    time: 0.0,
                    stop_time: tran.stop_time,
                }),
                AnalysisConfig::Ac(ac) => p.update_status(SimulationStatus::AcAnalysis {
                    freq: ac.start_freq,
                    stop_freq: ac.stop_freq,
                }),
                _ => p.update_status(SimulationStatus::DcOperatingPoint),
            },
            SimulationRequest::Spec { spec, options } => match spec.as_ref() {
                AnalysisSpec::DcOp => p.update_status(SimulationStatus::DcOperatingPoint),
                AnalysisSpec::DcSweep { source_name, .. } => {
                    p.update_status(SimulationStatus::DcSweep {
                        source: source_name.clone(),
                        progress: 0.0,
                    })
                }
                AnalysisSpec::Transient { stop_time, .. } => {
                    p.update_status(SimulationStatus::Transient {
                        time: 0.0,
                        stop_time: *stop_time,
                    })
                }
                AnalysisSpec::Ac {
                    start_freq,
                    stop_freq,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *start_freq,
                    stop_freq: *stop_freq,
                }),
                AnalysisSpec::Disto {
                    start_freq,
                    stop_freq,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *start_freq,
                    stop_freq: *stop_freq,
                }),
                AnalysisSpec::Noise {
                    start_freq,
                    stop_freq,
                    ..
                } => p.update_status(SimulationStatus::NoiseAnalysis {
                    freq: *start_freq,
                    stop_freq: *stop_freq,
                }),
                AnalysisSpec::Pss {
                    fundamental_freq, ..
                } => {
                    let period = if *fundamental_freq > 0.0 {
                        1.0 / *fundamental_freq
                    } else {
                        0.0
                    };
                    p.update_status(SimulationStatus::Transient {
                        time: 0.0,
                        stop_time: period,
                    })
                }
                AnalysisSpec::HarmonicBalance { tones, .. } => {
                    p.update_status(SimulationStatus::AcAnalysis {
                        freq: tones.first().map(|tone| tone.frequency).unwrap_or(1.0),
                        stop_freq: tones
                            .iter()
                            .map(|tone| tone.frequency * tone.harmonics.max(1) as f64)
                            .fold(1.0, f64::max),
                    })
                }
                AnalysisSpec::Pac => {
                    if let Some(pac) = &options.as_ref().pac {
                        p.update_status(SimulationStatus::AcAnalysis {
                            freq: pac.start_freq,
                            stop_freq: pac.stop_freq,
                        });
                    } else {
                        p.update_status(SimulationStatus::AcAnalysis {
                            freq: 1.0,
                            stop_freq: 1.0,
                        });
                    }
                }
                AnalysisSpec::Pxf => {
                    if let Some(pxf) = &options.as_ref().pxf {
                        p.update_status(SimulationStatus::AcAnalysis {
                            freq: pxf.start_freq,
                            stop_freq: pxf.stop_freq,
                        });
                    } else {
                        p.update_status(SimulationStatus::AcAnalysis {
                            freq: 1.0,
                            stop_freq: 1.0,
                        });
                    }
                }
                AnalysisSpec::Stb {
                    start_freq,
                    stop_freq,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *start_freq,
                    stop_freq: *stop_freq,
                }),
                AnalysisSpec::Pstb => {
                    if let Some(pstb) = &options.as_ref().pstb {
                        p.update_status(SimulationStatus::AcAnalysis {
                            freq: pstb.pss_fundamental_freq,
                            stop_freq: pstb.pss_fundamental_freq,
                        });
                    } else {
                        p.update_status(SimulationStatus::AcAnalysis {
                            freq: 1.0,
                            stop_freq: 1.0,
                        });
                    }
                }
                AnalysisSpec::SParameter {
                    start_freq,
                    stop_freq,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *start_freq,
                    stop_freq: *stop_freq,
                }),
                AnalysisSpec::Envelope { stop_time, .. } => {
                    p.update_status(SimulationStatus::Transient {
                        time: 0.0,
                        stop_time: *stop_time,
                    })
                }
                AnalysisSpec::Fourier {
                    fundamental_freq,
                    num_harmonics,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *fundamental_freq,
                    stop_freq: *fundamental_freq * (*num_harmonics).max(1) as f64,
                }),
                AnalysisSpec::MonteCarlo => p.update_status(SimulationStatus::PostProcessing),
                AnalysisSpec::Parametric => p.update_status(SimulationStatus::DcSweep {
                    source: "STEP".to_string(),
                    progress: 0.0,
                }),
                AnalysisSpec::Corner => p.update_status(SimulationStatus::DcSweep {
                    source: "TEMP".to_string(),
                    progress: 0.0,
                }),
                AnalysisSpec::Reliability { .. } => {
                    p.update_status(SimulationStatus::PostProcessing)
                }
                AnalysisSpec::Optimization { .. } => {
                    p.update_status(SimulationStatus::PostProcessing)
                }
                AnalysisSpec::Soa { stop_time, .. } => {
                    p.update_status(SimulationStatus::Transient {
                        time: 0.0,
                        stop_time: *stop_time,
                    })
                }
                AnalysisSpec::PoleZero { .. } => p.update_status(SimulationStatus::PoleZero),
                AnalysisSpec::Sensitivity { .. } => p.update_status(SimulationStatus::Sensitivity),
                _ => p.update_status(SimulationStatus::PostProcessing),
            },
        }
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = lock_progress(&progress, "run_simulation_thread(abort-before-execute)");
        p.abort();
        return Err(SimulationError::Aborted);
    }

    let signal = RunnerSignal {
        abort_flag: abort_flag.clone(),
        progress: progress.clone(),
    };

    let result = match request {
        SimulationRequest::Config(config) => {
            // Run simulation via engine bridge with abort support
            log::info!("Running simulation via engine bridge: {:?}", config);
            match bridge.run_with_abort_and_source_path(
                &config,
                &input.netlist,
                input.source_path.as_deref(),
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
            spec::run_spec_request(
                &bridge,
                *spec,
                *options,
                &input.netlist,
                input.source_path.as_deref(),
                &signal,
            )?
        }
    };

    // Mark complete
    {
        let mut p = lock_progress(&progress, "run_simulation_thread(complete)");
        p.complete();
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

    /// Circuit building error
    CircuitError(String),

    /// Solver error
    SolverError(String),

    /// Convergence failure
    ConvergenceFailed { iterations: usize, message: String },

    /// Simulation was aborted
    Aborted,

    /// A simulation is already running
    AlreadyRunning,

    /// Thread panicked
    ThreadPanic,

    /// Invalid configuration
    InvalidConfig(String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SimulationError::CircuitError(msg) => write!(f, "Circuit error: {}", msg),
            SimulationError::SolverError(msg) => write!(f, "Solver error: {}", msg),
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
            SimulationError::Aborted => write!(f, "Simulation aborted"),
            SimulationError::AlreadyRunning => write!(f, "A simulation is already running"),
            SimulationError::ThreadPanic => write!(f, "Simulation thread panicked"),
            SimulationError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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

        let start_result = runner.start(AnalysisConfig::DcOp, String::new());
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

        let start_result = runner.start(AnalysisConfig::DcOp, String::new());
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
}
