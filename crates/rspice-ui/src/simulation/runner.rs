//! Simulation Runner - Async Simulation Execution
//!
//! Provides the bridge between UI and rspice-core simulation engine with:
//! - Async simulation execution on background thread
//! - Thread-safe progress updates
//! - Abort capability
//! - Result caching

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};

use super::config::AnalysisConfig;
use super::multi_run::AnalysisSpec;
use super::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use super::status::{SimulationProgress, SimulationStatus};

/// Optional execution overrides for spec-driven analyses.
#[derive(Debug, Clone, Default)]
pub struct SpecExecutionOptions {
    pub temp: Option<crate::services::simulation_runner::TempRunConfig>,
    pub corner: Option<crate::services::simulation_runner::CornerRunConfig>,
    pub pac: Option<crate::services::simulation_runner::PacRunConfig>,
    pub tf: Option<crate::services::simulation_runner::TfRunConfig>,
    pub pnoise: Option<crate::services::simulation_runner::PnoiseRunConfig>,
}

//=============================================================================
// Simulation Runner
//=============================================================================

#[derive(Debug, Clone)]
enum SimulationRequest {
    Config(AnalysisConfig),
    Spec {
        spec: AnalysisSpec,
        options: SpecExecutionOptions,
    },
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

    /// Cached results from last successful simulation
    last_result: Option<SimulationResult>,
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
            last_result: None,
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
        self.progress.lock().unwrap().status.clone()
    }

    /// Get current progress percentage (0.0 to 1.0)
    pub fn progress_fraction(&self) -> Option<f32> {
        self.progress.lock().unwrap().status.progress()
    }

    /// Get full progress information
    pub fn progress(&self) -> SimulationProgress {
        self.progress.lock().unwrap().clone()
    }

    /// Get last successful result
    pub fn last_result(&self) -> Option<&SimulationResult> {
        self.last_result.as_ref()
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
        // Check if thread is finished
        let is_finished = self.thread_handle.as_ref().is_some_and(|h| h.is_finished());

        if is_finished {
            // Take the handle and join
            if let Some(handle) = self.thread_handle.take() {
                match handle.join() {
                    Ok(result) => {
                        // Cache successful result
                        if let Ok(ref sim_result) = result {
                            self.last_result = Some(sim_result.clone());
                        }
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

    /// Start a simulation with the given configuration
    ///
    /// Returns error if a simulation is already running.
    pub fn start(
        &mut self,
        config: AnalysisConfig,
        netlist: String,
    ) -> Result<(), SimulationError> {
        self.start_request(SimulationRequest::Config(config), netlist)
    }

    /// Start a simulation from strongly-typed analysis spec.
    pub fn start_spec(
        &mut self,
        spec: AnalysisSpec,
        netlist: String,
    ) -> Result<(), SimulationError> {
        self.start_spec_with_options(spec, netlist, SpecExecutionOptions::default())
    }

    /// Start a simulation from strongly-typed analysis spec with explicit execution options.
    pub fn start_spec_with_options(
        &mut self,
        spec: AnalysisSpec,
        netlist: String,
        options: SpecExecutionOptions,
    ) -> Result<(), SimulationError> {
        self.start_request(SimulationRequest::Spec { spec, options }, netlist)
    }

    fn start_request(
        &mut self,
        request: SimulationRequest,
        netlist: String,
    ) -> Result<(), SimulationError> {
        if self.is_running() {
            return Err(SimulationError::AlreadyRunning);
        }

        // Reset state
        self.abort_flag.store(false, Ordering::SeqCst);
        {
            let mut progress = self.progress.lock().unwrap();
            *progress = SimulationProgress::new();
        }

        // Clone Arcs for the thread
        let progress = Arc::clone(&self.progress);
        let abort_flag = Arc::clone(&self.abort_flag);

        // Spawn simulation thread with real engine
        let handle =
            thread::spawn(move || run_simulation_thread(request, netlist, progress, abort_flag));

        self.thread_handle = Some(handle);
        Ok(())
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&mut self, netlist: String) -> Result<(), SimulationError> {
        self.start(AnalysisConfig::DcOp, netlist)
    }

    /// Clear cached results
    pub fn clear_results(&mut self) {
        self.last_result = None;
    }
}

/// Simulation execution in background thread
///
/// Runs the actual rspice-core simulation engine.
fn run_simulation_thread(
    request: SimulationRequest,
    netlist: String,
    progress: Arc<Mutex<SimulationProgress>>,
    abort_flag: Arc<AtomicBool>,
) -> Result<SimulationResult, SimulationError> {
    use super::engine_bridge::EngineBridge;

    // Update status: parsing
    {
        let mut p = progress.lock().unwrap();
        p.update_status(SimulationStatus::Parsing);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = progress.lock().unwrap();
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Create engine bridge
    let bridge = EngineBridge::new();

    // Update status: building
    {
        let mut p = progress.lock().unwrap();
        p.update_status(SimulationStatus::Building);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = progress.lock().unwrap();
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Update status based on analysis type
    {
        let mut p = progress.lock().unwrap();
        match &request {
            SimulationRequest::Config(config) => match config {
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
            SimulationRequest::Spec { spec, options } => match spec {
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
                AnalysisSpec::HarmonicBalance {
                    tone1_freq,
                    tone1_harmonics,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *tone1_freq,
                    stop_freq: *tone1_freq * (*tone1_harmonics).max(1) as f64,
                }),
                AnalysisSpec::Pac => {
                    if let Some(pac) = &options.pac {
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
                AnalysisSpec::Stb {
                    start_freq,
                    stop_freq,
                    ..
                } => p.update_status(SimulationStatus::AcAnalysis {
                    freq: *start_freq,
                    stop_freq: *stop_freq,
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
                AnalysisSpec::PoleZero { .. } => p.update_status(SimulationStatus::PoleZero),
                AnalysisSpec::Sensitivity { .. } => p.update_status(SimulationStatus::Sensitivity),
                _ => p.update_status(SimulationStatus::PostProcessing),
            },
        }
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = progress.lock().unwrap();
        p.abort();
        return Err(SimulationError::Aborted);
    }

    let result = match request {
        SimulationRequest::Config(config) => {
            // Run simulation via engine bridge with abort support
            log::info!("Running simulation via engine bridge: {:?}", config);
            match bridge.run_with_abort(&config, &netlist, &abort_flag) {
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
            run_spec_request(spec, options, &netlist, &abort_flag)?
        }
    };

    // Mark complete
    {
        let mut p = progress.lock().unwrap();
        p.complete();
    }

    log::info!("Simulation thread completed successfully");
    Ok(result)
}

fn run_spec_request(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    abort_flag: &Arc<AtomicBool>,
) -> Result<SimulationResult, SimulationError> {
    use crate::services::simulation_runner as svc_runner;

    if abort_flag.load(Ordering::SeqCst) {
        return Err(SimulationError::Aborted);
    }

    match spec {
        AnalysisSpec::MonteCarlo => {
            let data = svc_runner::run_monte_carlo_analysis(netlist)
                .map_err(SimulationError::InvalidConfig)?;
            let variables = data
                .variables
                .into_iter()
                .map(|var| MonteCarloVariableResult {
                    name: var.name,
                    mean: var.mean,
                    std_dev: var.std_dev,
                    min: var.min,
                    max: var.max,
                    histogram: var.histogram,
                    bin_edges: var.bin_edges,
                })
                .collect();
            Ok(SimulationResult::MonteCarlo {
                runs_requested: data.runs_requested,
                runs_completed: data.runs_completed,
                num_failures: data.num_failures,
                all_converged: data.all_converged,
                variables,
            })
        }
        AnalysisSpec::Parametric => {
            let data = if let Some(temp_cfg) = options.temp {
                svc_runner::run_parametric_analysis_with_config(netlist, &temp_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_parametric_analysis(netlist)
                    .map_err(SimulationError::InvalidConfig)?
            };
            let sweep_values = data.sweep_values;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .voltages
                .into_iter()
                .map(|(name, values)| {
                    (
                        name.clone(),
                        WaveformData::new_time_domain(name, sweep_values.clone(), values),
                    )
                })
                .collect();
            Ok(SimulationResult::Parametric {
                target: data.target,
                sweep_values,
                waveforms,
                num_failures: data.num_failures,
            })
        }
        AnalysisSpec::Corner => {
            let data = if let Some(corner_cfg) = options.corner {
                svc_runner::run_corner_analysis_with_config(netlist, &corner_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_corner_analysis(netlist).map_err(SimulationError::InvalidConfig)?
            };
            let x_values = data.x_values;
            let x_label = data.x_label;
            let x_unit = data.x_unit;
            let temperatures_c = data.temperatures_c;
            let corner_labels = data.corner_labels;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .voltages
                .into_iter()
                .map(|(name, values)| {
                    let waveform = WaveformData {
                        name: name.clone(),
                        x_values: x_values.clone(),
                        y_values: values,
                        y_unit: "V".to_string(),
                        x_unit: x_unit.clone(),
                        is_complex: false,
                        y_imag: None,
                    };
                    (name.clone(), waveform)
                })
                .collect();
            Ok(SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures: data.num_failures,
            })
        }
        AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
        } => {
            let data =
                svc_runner::run_pss_analysis(netlist, fundamental_freq, num_harmonics, tolerance)
                    .map_err(SimulationError::InvalidConfig)?;

            let time = data.time;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .waveforms
                .into_iter()
                .map(|(name, values)| {
                    (
                        name.clone(),
                        WaveformData::new_time_domain(name, time.clone(), values),
                    )
                })
                .collect();

            Ok(SimulationResult::Transient { time, waveforms })
        }
        AnalysisSpec::HarmonicBalance {
            tone1_freq,
            tone1_harmonics,
            tone2_freq,
            tone2_harmonics,
        } => {
            let data = svc_runner::run_hb_analysis(
                netlist,
                tone1_freq,
                tone1_harmonics,
                tone2_freq,
                tone2_harmonics,
            )
            .map_err(SimulationError::InvalidConfig)?;

            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .spectra
                .into_iter()
                .map(|(name, spectrum)| {
                    let freqs: Vec<f64> = spectrum.iter().map(|(freq, _, _)| *freq).collect();
                    let real: Vec<f64> = spectrum
                        .iter()
                        .map(|(_, mag, phase_deg)| *mag * phase_deg.to_radians().cos())
                        .collect();
                    let imag: Vec<f64> = spectrum
                        .iter()
                        .map(|(_, mag, phase_deg)| *mag * phase_deg.to_radians().sin())
                        .collect();
                    (
                        name.clone(),
                        WaveformData::new_complex(name, freqs, real, imag),
                    )
                })
                .collect();
            let frequencies = waveforms
                .values()
                .next()
                .map(|wf| wf.x_values.clone())
                .unwrap_or_default();

            Ok(SimulationResult::Ac {
                frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Tf => {
            let data = if let Some(tf_cfg) = options.tf {
                svc_runner::run_tf_analysis_with_config(netlist, &tf_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_tf_analysis(netlist).map_err(SimulationError::InvalidConfig)?
            };

            let mut waveforms: std::collections::HashMap<String, WaveformData> =
                std::collections::HashMap::new();
            let transfer_name = format!("H({}/{})", data.output_label, data.input_source);
            waveforms.insert(
                transfer_name.clone(),
                WaveformData::new_complex(
                    transfer_name,
                    data.frequencies.clone(),
                    data.transfer.iter().map(|value| value.re).collect(),
                    data.transfer.iter().map(|value| value.im).collect(),
                ),
            );

            if let Some(gd) = data.group_delay {
                let (freqs, delays): (Vec<f64>, Vec<f64>) = gd.into_iter().unzip();
                waveforms.insert(
                    "group_delay".to_string(),
                    WaveformData {
                        name: "group_delay".to_string(),
                        x_values: freqs,
                        y_values: delays,
                        y_unit: "s".to_string(),
                        x_unit: "Hz".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

            if let Some(zin) = data.input_impedance {
                let zin_name = format!("Zin({})", data.input_source);
                waveforms.insert(
                    zin_name.clone(),
                    WaveformData::new_complex(
                        zin_name,
                        data.frequencies.clone(),
                        zin.iter().map(|value| value.re).collect(),
                        zin.iter().map(|value| value.im).collect(),
                    ),
                );
            }

            if let Some(zout) = data.output_impedance {
                let zout_name = format!("Zout({})", data.output_label);
                waveforms.insert(
                    zout_name.clone(),
                    WaveformData::new_complex(
                        zout_name,
                        data.frequencies.clone(),
                        zout.iter().map(|value| value.re).collect(),
                        zout.iter().map(|value| value.im).collect(),
                    ),
                );
            }

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Pac => {
            let pac_cfg = options.pac.ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "PAC analysis requires explicit PAC execution options".to_string(),
                )
            })?;
            let data = svc_runner::run_pac_analysis(netlist, &pac_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .spectra
                .into_iter()
                .map(|(name, spectrum)| {
                    let freqs: Vec<f64> = spectrum.iter().map(|(freq, _, _)| *freq).collect();
                    let real: Vec<f64> = spectrum
                        .iter()
                        .map(|(_, mag, phase_deg)| *mag * phase_deg.to_radians().cos())
                        .collect();
                    let imag: Vec<f64> = spectrum
                        .iter()
                        .map(|(_, mag, phase_deg)| *mag * phase_deg.to_radians().sin())
                        .collect();
                    (
                        name.clone(),
                        WaveformData::new_complex(name, freqs, real, imag),
                    )
                })
                .collect();

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Pnoise => {
            let data = if let Some(pnoise_cfg) = options.pnoise {
                svc_runner::run_pnoise_analysis_with_config(netlist, &pnoise_cfg)
                    .map_err(SimulationError::InvalidConfig)?
            } else {
                svc_runner::run_pnoise_analysis(netlist).map_err(SimulationError::InvalidConfig)?
            };

            let freq_len = data.frequencies.len().max(1);
            let contributors = data
                .contributors
                .into_iter()
                .map(|(name, percentage)| (name, vec![percentage; freq_len]))
                .collect();

            Ok(SimulationResult::Noise {
                frequencies: data.frequencies,
                output_noise: data.output_noise,
                input_noise: data.input_noise,
                contributors,
            })
        }
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            points_per_decade,
        } => {
            let data = svc_runner::run_stb_analysis(
                netlist,
                &probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
            )
            .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "Loop Gain (dB)".to_string(),
                WaveformData {
                    name: "Loop Gain (dB)".to_string(),
                    x_values: data.frequencies.clone(),
                    y_values: data.loop_gain_db,
                    y_unit: "dB".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Loop Phase (deg)".to_string(),
                WaveformData {
                    name: "Loop Phase (deg)".to_string(),
                    x_values: data.frequencies.clone(),
                    y_values: data.loop_phase_deg,
                    y_unit: "deg".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        unsupported => Err(SimulationError::InvalidConfig(format!(
            "{:?} is not supported by SimulationRunner::start_spec",
            unsupported.run_type()
        ))),
    }
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

    /// Minimal valid netlist for testing DC operating point
    fn test_netlist() -> String {
        "* Test Circuit\nV1 vdd 0 5\nR1 vdd out 1k\nR2 out 0 1k\n.end\n".to_string()
    }

    #[test]
    fn test_runner_new() {
        let runner = SimulationRunner::new();
        assert!(!runner.is_running());
        assert!(matches!(runner.status(), SimulationStatus::Idle));
    }

    #[test]
    fn test_runner_not_running_initially() {
        let runner = SimulationRunner::new();
        assert!(!runner.is_running());
        assert!(runner.progress_fraction().is_none());
    }

    #[test]
    fn test_runner_start_and_poll() {
        let mut runner = SimulationRunner::new();

        // Start simulation with valid netlist
        let result = runner.start(AnalysisConfig::DcOp, test_netlist());
        assert!(result.is_ok());

        // Should be running now (or already finished if very fast)
        // Don't assert running since the simulation is very fast

        // Wait for completion
        thread::sleep(std::time::Duration::from_millis(200));

        // Poll for result
        let result = runner.poll_result();
        assert!(result.is_some(), "Expected simulation result, got None");
        let sim_result = result.unwrap();
        assert!(
            sim_result.is_ok(),
            "Expected Ok result, got: {:?}",
            sim_result
        );

        // No longer running
        assert!(!runner.is_running());
    }

    #[test]
    fn test_runner_abort() {
        let mut runner = SimulationRunner::new();

        // Start simulation with valid netlist
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

        // Abort immediately
        runner.abort();

        // Wait for thread to notice abort
        thread::sleep(std::time::Duration::from_millis(100));

        // Poll for result - might be aborted or might have completed before abort took effect
        let result = runner.poll_result();
        // Result should exist after polling (either aborted or completed)
        if let Some(res) = result {
            // Either aborted or completed is acceptable
            assert!(matches!(res, Err(SimulationError::Aborted)) || res.is_ok());
        }
    }

    #[test]
    fn test_runner_already_running() {
        let mut runner = SimulationRunner::new();

        // Start first simulation with valid netlist
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

        // Try to start another while running (might already be done if fast)
        if runner.is_running() {
            let result = runner.start(AnalysisConfig::DcOp, test_netlist());
            assert!(matches!(result, Err(SimulationError::AlreadyRunning)));
        }

        // Cleanup
        runner.abort();
        thread::sleep(std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_runner_clear_results() {
        let mut runner = SimulationRunner::new();
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();
        thread::sleep(std::time::Duration::from_millis(200));
        let result = runner.poll_result();

        // Verify simulation completed successfully before testing clear
        assert!(result.is_some(), "Expected simulation result");
        let sim_result = result.unwrap();
        assert!(sim_result.is_ok(), "Simulation failed: {:?}", sim_result);

        assert!(runner.last_result().is_some(), "Expected cached result");
        runner.clear_results();
        assert!(runner.last_result().is_none(), "Expected cleared result");
    }

    #[test]
    fn test_simulation_error_display() {
        let err = SimulationError::ParseError("test error".to_string());
        assert!(err.to_string().contains("Parse error"));

        let err = SimulationError::ConvergenceFailed {
            iterations: 50,
            message: "did not converge".to_string(),
        };
        assert!(err.to_string().contains("50"));
    }

    #[test]
    fn test_runner_progress_update() {
        let mut runner = SimulationRunner::new();
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

        // Give thread time to start
        thread::sleep(std::time::Duration::from_millis(10));

        // Check status is not idle (or completed if very fast)
        let _status = runner.status();
        // Progress status might be Idle if completed very quickly, so just check it ran

        // Cleanup
        thread::sleep(std::time::Duration::from_millis(200));
        let _ = runner.poll_result();
    }

    #[test]
    fn test_runner_default() {
        let runner = SimulationRunner::default();
        assert!(!runner.is_running());
    }

    #[test]
    fn test_run_dc_op_convenience() {
        let mut runner = SimulationRunner::new();
        let result = runner.run_dc_op(test_netlist());
        assert!(result.is_ok());

        // Cleanup - wait and poll
        thread::sleep(std::time::Duration::from_millis(200));
        let result = runner.poll_result();
        assert!(result.is_some(), "Expected result from dc_op");
    }

    #[test]
    fn test_runner_start_spec_monte_carlo() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Monte Carlo smoke test
.param RV=1k
V1 in 0 1
R1 in 0 {RV}
.mc 8 gauss 0.05
.end
"#
        .to_string();

        runner
            .start_spec(AnalysisSpec::MonteCarlo, netlist)
            .expect("spec run should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected Monte Carlo result");
        let result = result.unwrap().expect("Monte Carlo should succeed");
        match result {
            SimulationResult::MonteCarlo {
                runs_requested,
                runs_completed,
                ..
            } => {
                assert_eq!(runs_requested, 8);
                assert!(runs_completed <= runs_requested);
            }
            other => panic!("Expected MonteCarlo result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_parametric_temp() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Parametric TEMP sweep smoke test
V1 in 0 1
R1 in 0 1k
.step temp list -40 25 85
.end
"#
        .to_string();

        runner
            .start_spec(AnalysisSpec::Parametric, netlist)
            .expect("parametric spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected parametric result");
        let result = result.unwrap().expect("Parametric should succeed");
        match result {
            SimulationResult::Parametric {
                target,
                sweep_values,
                ..
            } => {
                assert_eq!(target, "TEMP");
                assert!(!sweep_values.is_empty());
            }
            other => panic!("Expected Parametric result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_parametric_with_temp_ac_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Parametric TEMP sweep with AC base-mode override
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.step temp list -40 25 125
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: Some(crate::services::simulation_runner::TempRunConfig {
                temperatures_c: vec![-40.0, 25.0, 125.0],
                base_mode: crate::services::simulation_runner::CornerBaseMode::Ac {
                    start_freq: 1e3,
                    stop_freq: 1e6,
                    points_per_unit: 8,
                    sweep: crate::services::simulation_runner::CornerFrequencySweep::Decade,
                },
            }),
            corner: None,
            pac: None,
            tf: None,
            pnoise: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Parametric, netlist, options)
            .expect("parametric AC options should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected parametric AC result");
        let result = result.unwrap().expect("Parametric AC should succeed");
        match result {
            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                ..
            } => {
                assert_eq!(target, "TEMP");
                assert_eq!(sweep_values.len(), 3);
                assert!(
                    waveforms
                        .keys()
                        .any(|name| name.eq_ignore_ascii_case("|V(out)|")),
                    "expected |V(out)| trace, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
            }
            other => panic!("Expected Parametric result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_corner_temp() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Corner TEMP sweep smoke test
V1 in 0 1
R1 in 0 1k
.temp -40 25 85
.end
"#
        .to_string();

        runner
            .start_spec(AnalysisSpec::Corner, netlist)
            .expect("corner spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected corner result");
        let result = result.unwrap().expect("Corner should succeed");
        match result {
            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                ..
            } => {
                assert_eq!(temperatures_c.len(), 3);
                assert_eq!(x_values, vec![-40.0, 25.0, 85.0]);
                assert_eq!(x_label, "Temperature");
                assert_eq!(x_unit, "C");
            }
            other => panic!("Expected Corner result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_corner_with_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Corner PVT sweep smoke test
VDD vdd 0 1.0
R1 vdd out 1k
R2 out 0 1k
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: None,
            corner: Some(crate::services::simulation_runner::CornerRunConfig {
                process_corners: vec![
                    crate::services::simulation_runner::CornerProcess::TT,
                    crate::services::simulation_runner::CornerProcess::FF,
                ],
                voltages: vec![0.9, 1.1],
                temperatures_c: vec![25.0],
                full_matrix: true,
                nominal_voltage: Some(1.0),
                base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
            }),
            pac: None,
            tf: None,
            pnoise: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Corner, netlist, options)
            .expect("corner spec with options should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected corner result");
        let result = result.unwrap().expect("Corner should succeed");
        match result {
            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                ..
            } => {
                assert_eq!(temperatures_c.len(), 4);
                assert_eq!(x_values, vec![0.0, 1.0, 2.0, 3.0]);
                assert_eq!(x_label, "Corner Index");
                assert_eq!(x_unit, "");
                assert_eq!(corner_labels.len(), 4);
                assert!(corner_labels
                    .iter()
                    .any(|label| label.contains("FF_1.100000V")));
            }
            other => panic!("Expected Corner result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_corner_with_ac_base_mode_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Corner AC sweep smoke test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: None,
            corner: Some(crate::services::simulation_runner::CornerRunConfig {
                process_corners: vec![crate::services::simulation_runner::CornerProcess::TT],
                voltages: vec![1.0],
                temperatures_c: vec![-40.0, 25.0, 125.0],
                full_matrix: true,
                nominal_voltage: Some(1.0),
                base_mode: crate::services::simulation_runner::CornerBaseMode::Ac {
                    start_freq: 1e3,
                    stop_freq: 1e6,
                    points_per_unit: 8,
                    sweep: crate::services::simulation_runner::CornerFrequencySweep::Decade,
                },
            }),
            pac: None,
            tf: None,
            pnoise: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Corner, netlist, options)
            .expect("corner AC options should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected corner AC result");
        let result = result.unwrap().expect("Corner AC should succeed");
        match result {
            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                waveforms,
                ..
            } => {
                assert_eq!(temperatures_c.len(), 3);
                assert_eq!(x_values, vec![-40.0, 25.0, 125.0]);
                assert_eq!(x_label, "Temperature");
                assert_eq!(x_unit, "C");
                assert!(
                    waveforms
                        .keys()
                        .any(|name| name.eq_ignore_ascii_case("|V(out)|")),
                    "expected |V(out)| trace, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
            }
            other => panic!("Expected Corner result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pss() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PSS smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Pss {
                    fundamental_freq: 1e6,
                    num_harmonics: 8,
                    tolerance: 1e-4,
                },
                netlist,
            )
            .expect("PSS spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected PSS result");
        let result = result.unwrap().expect("PSS should succeed");
        match result {
            SimulationResult::Transient { time, waveforms } => {
                assert!(!time.is_empty());
                assert!(!waveforms.is_empty());
            }
            other => panic!("Expected Transient result for PSS, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_harmonic_balance() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* HB smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::HarmonicBalance {
                    tone1_freq: 1e6,
                    tone1_harmonics: 5,
                    tone2_freq: None,
                    tone2_harmonics: 0,
                },
                netlist,
            )
            .expect("HB spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected HB result");
        let result = result.unwrap().expect("HB should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(!waveforms.is_empty());
                assert!(
                    waveforms
                        .values()
                        .any(|wf| wf.is_complex && wf.y_imag.is_some()),
                    "expected at least one complex HB waveform"
                );
            }
            other => panic!("Expected AC result for HB, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pac() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PAC smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: None,
            corner: None,
            pac: Some(crate::services::simulation_runner::PacRunConfig {
                pss_fundamental_freq: 1e6,
                pss_num_harmonics: 8,
                pss_tolerance: 1e-4,
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: crate::services::simulation_runner::PacFrequencySweep::Decade,
                max_sideband: 2,
                input_source: "V1".to_string(),
                output_node: "out".to_string(),
                output_ref: None,
                pac_magnitude: 1.0,
                include_dc: true,
                reltol: 1e-3,
                abstol: 1e-12,
            }),
            tf: None,
            pnoise: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Pac, netlist, options)
            .expect("PAC spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected PAC result");
        let result = result.unwrap().expect("PAC should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(!waveforms.is_empty());
                assert!(
                    waveforms
                        .keys()
                        .any(|name| name.contains("[sb=") && name.starts_with("V(")),
                    "expected PAC sideband trace names, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
                assert!(waveforms.values().all(|wf| wf.is_complex
                    && wf
                        .y_imag
                        .as_ref()
                        .is_some_and(|imag| imag.len() == wf.y_values.len())));
            }
            other => panic!("Expected AC result for PAC, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_tf_with_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* TF smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: None,
            corner: None,
            pac: None,
            tf: Some(crate::services::simulation_runner::TfRunConfig {
                start_freq: 10.0,
                stop_freq: 1e6,
                points_per_unit: 6,
                sweep: crate::services::simulation_runner::TfFrequencySweep::Decade,
                input_source: "V1".to_string(),
                output_node: "out".to_string(),
                output_ref: None,
                group_delay: true,
                input_impedance: true,
                output_impedance: true,
            }),
            pnoise: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Tf, netlist, options)
            .expect("TF spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected TF result");
        let result = result.unwrap().expect("TF should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(
                    waveforms.keys().any(|name| name.starts_with("H(")),
                    "expected transfer waveform, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
                assert!(
                    waveforms.keys().any(|name| name.starts_with("Zin(")),
                    "expected Zin waveform, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
                assert!(
                    waveforms.keys().any(|name| name.starts_with("Zout(")),
                    "expected Zout waveform, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
                assert!(waveforms
                    .values()
                    .any(|wf| wf.is_complex && wf.y_imag.as_ref().is_some()));
            }
            other => panic!("Expected AC result for TF, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pnoise_with_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PNOISE smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: None,
            corner: None,
            pac: None,
            tf: None,
            pnoise: Some(crate::services::simulation_runner::PnoiseRunConfig {
                pss_fundamental_freq: 1e6,
                pss_num_harmonics: 8,
                pss_tolerance: 1e-4,
                start_freq: 10.0,
                stop_freq: 1e6,
                points_per_unit: 6,
                sweep: crate::services::simulation_runner::PnoiseFrequencySweep::Decade,
                max_sideband: 3,
                output_node: "out".to_string(),
                output_ref: None,
                noise_ref: crate::services::simulation_runner::PnoiseReference::Output,
                integrated_noise: true,
                noise_summary: true,
                reltol: 1e-3,
                abstol: 1e-18,
            }),
        };

        runner
            .start_spec_with_options(AnalysisSpec::Pnoise, netlist, options)
            .expect("PNOISE spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected PNOISE result");
        let result = result.unwrap().expect("PNOISE should succeed");
        match result {
            SimulationResult::Noise {
                frequencies,
                output_noise,
                ..
            } => {
                assert!(!frequencies.is_empty());
                assert_eq!(frequencies.len(), output_noise.len());
                assert!(output_noise.iter().all(|value| value.is_finite()));
            }
            other => panic!("Expected Noise result for PNOISE, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_stb() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* STB smoke test
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Stb {
                    probe_node: "1".to_string(),
                    start_freq: 1.0,
                    stop_freq: 1e6,
                    points_per_decade: 8,
                },
                netlist,
            )
            .expect("STB spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected STB result");
        let result = result.unwrap().expect("STB should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(waveforms.contains_key("Loop Gain (dB)"));
                assert!(waveforms.contains_key("Loop Phase (deg)"));
                assert_eq!(
                    waveforms
                        .get("Loop Gain (dB)")
                        .expect("loop-gain waveform should exist")
                        .x_values
                        .len(),
                    frequencies.len()
                );
                assert_eq!(
                    waveforms
                        .get("Loop Phase (deg)")
                        .expect("loop-phase waveform should exist")
                        .x_values
                        .len(),
                    frequencies.len()
                );
            }
            other => panic!("Expected AC result for STB, got {:?}", other),
        }
    }
}
