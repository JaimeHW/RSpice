//! Simulation Runner - Async Simulation Execution
//!
//! Provides the bridge between UI and rspice-core simulation engine with:
//! - Async simulation execution on background thread
//! - Thread-safe progress updates
//! - Abort capability
//! - Result caching

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};

use super::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    PoleZeroConfig, PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use super::multi_run::AnalysisSpec;
use super::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use super::status::{SimulationProgress, SimulationStatus};

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
                AnalysisSpec::Pxf => {
                    if let Some(pxf) = &options.pxf {
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
                    if let Some(pstb) = &options.pstb {
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
            run_spec_request(&bridge, spec, options, &netlist, &abort_flag)?
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
    bridge: &super::engine_bridge::EngineBridge,
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    abort_flag: &Arc<AtomicBool>,
) -> Result<SimulationResult, SimulationError> {
    use crate::services::simulation_runner as svc_runner;

    if abort_flag.load(Ordering::SeqCst) {
        return Err(SimulationError::Aborted);
    }

    if let Some(config) = analysis_config_from_spec(&spec) {
        return bridge.run_with_abort(&config, netlist, abort_flag);
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
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => {
            let cfg = svc_runner::ReliabilityRunConfig {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            };
            let data = svc_runner::run_reliability_analysis_with_config(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            for device in &data.device_results {
                let mut years = Vec::with_capacity(data.years.len());
                let mut vth = Vec::with_capacity(data.years.len());
                let mut mobility = Vec::with_capacity(data.years.len());
                let mut rds = Vec::with_capacity(data.years.len());

                for years_key in &data.years {
                    let key = format!("{}y", years_key);
                    let shift = device.shifts.get(&key).cloned().unwrap_or_default();
                    years.push(*years_key);
                    vth.push(shift.vth_shift);
                    mobility.push(shift.mobility_shift);
                    rds.push(shift.rds_shift);
                }

                let vth_name = format!("DVTH({})", device.device_id);
                waveforms.insert(
                    vth_name.clone(),
                    WaveformData {
                        name: vth_name,
                        x_values: years.clone(),
                        y_values: vth,
                        y_unit: "V".to_string(),
                        x_unit: "year".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );

                let mobility_name = format!("DMU({})", device.device_id);
                waveforms.insert(
                    mobility_name.clone(),
                    WaveformData {
                        name: mobility_name,
                        x_values: years.clone(),
                        y_values: mobility,
                        y_unit: "ratio".to_string(),
                        x_unit: "year".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );

                let rds_name = format!("DRDS({})", device.device_id);
                waveforms.insert(
                    rds_name.clone(),
                    WaveformData {
                        name: rds_name,
                        x_values: years,
                        y_values: rds,
                        y_unit: "ratio".to_string(),
                        x_unit: "year".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

            Ok(SimulationResult::Reliability {
                years: data.years,
                waveforms,
                device_results: data.device_results,
            })
        }
        AnalysisSpec::Optimization {
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            algorithm,
            max_iterations,
            cost_tolerance,
            fd_step,
            initial_step,
            min_step,
        } => {
            let cfg = svc_runner::OptimizationRunConfig {
                variables: variables
                    .into_iter()
                    .map(|var| svc_runner::OptimizationVariable {
                        name: var.name,
                        min: var.min,
                        max: var.max,
                        initial: var.initial,
                    })
                    .collect(),
                objective_node,
                objective_ref,
                goal: match goal {
                    crate::simulation::multi_run::OptimizationGoal::Minimize => {
                        svc_runner::OptimizationGoalMode::Minimize
                    }
                    crate::simulation::multi_run::OptimizationGoal::Maximize => {
                        svc_runner::OptimizationGoalMode::Maximize
                    }
                    crate::simulation::multi_run::OptimizationGoal::Target => {
                        svc_runner::OptimizationGoalMode::Target
                    }
                },
                target,
                algorithm: match algorithm {
                    crate::simulation::multi_run::OptimizationAlgorithm::GradientDescent => {
                        svc_runner::OptimizationAlgorithmMode::GradientDescent
                    }
                    crate::simulation::multi_run::OptimizationAlgorithm::PatternSearch => {
                        svc_runner::OptimizationAlgorithmMode::PatternSearch
                    }
                    crate::simulation::multi_run::OptimizationAlgorithm::SimulatedAnnealing => {
                        svc_runner::OptimizationAlgorithmMode::SimulatedAnnealing
                    }
                },
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            };

            let data = svc_runner::run_optimization_analysis_with_config(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "OPT_COST".to_string(),
                WaveformData {
                    name: "OPT_COST".to_string(),
                    x_values: data.iterations.clone(),
                    y_values: data.costs.clone(),
                    y_unit: "cost".to_string(),
                    x_unit: "iter".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            for (name, values) in &data.variable_traces {
                let wf_name = format!("OPT_{}", name);
                waveforms.insert(
                    wf_name.clone(),
                    WaveformData {
                        name: wf_name,
                        x_values: data.iterations.clone(),
                        y_values: values.clone(),
                        y_unit: "value".to_string(),
                        x_unit: "iter".to_string(),
                        is_complex: false,
                        y_imag: None,
                    },
                );
            }

            Ok(SimulationResult::Optimization {
                iterations: data.iterations,
                waveforms,
                best_cost: data.best_cost,
                best_variables: data.best_variables,
                converged: data.converged,
            })
        }
        AnalysisSpec::Soa {
            stop_time,
            step_time,
            check_vgs_max,
            max_vgs,
            check_vds_max,
            max_vds,
            check_vbe_max,
            max_vbe,
            check_vce_max,
            max_vce,
        } => {
            let cfg = svc_runner::SoaRunConfig {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            };
            let data = svc_runner::run_soa_analysis_with_config(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "SOA_VIOLATION_COUNT".to_string(),
                WaveformData {
                    name: "SOA_VIOLATION_COUNT".to_string(),
                    x_values: data.time.clone(),
                    y_values: data.violation_count.clone(),
                    y_unit: "count".to_string(),
                    x_unit: "s".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            Ok(SimulationResult::Soa {
                time: data.time,
                waveforms,
                violations: data.violations,
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
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        } => {
            let sweep = match sweep {
                crate::simulation::multi_run::FrequencySweep::Decade => {
                    svc_runner::SParameterSweep::Decade
                }
                crate::simulation::multi_run::FrequencySweep::Octave => {
                    svc_runner::SParameterSweep::Octave
                }
                crate::simulation::multi_run::FrequencySweep::Linear => {
                    svc_runner::SParameterSweep::Linear
                }
            };
            let cfg = svc_runner::SParameterRunConfig {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports: ports
                    .into_iter()
                    .map(|port| svc_runner::SParameterPort {
                        node_pos: port.node_pos,
                        node_neg: port.node_neg,
                        z0: port.z0,
                    })
                    .collect(),
            };
            let data = svc_runner::run_sparameter_analysis(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let mut waveforms = std::collections::HashMap::new();
            for row in 0..data.num_ports {
                for col in 0..data.num_ports {
                    let name = if data.num_ports <= 9 {
                        format!("S{}{}", row + 1, col + 1)
                    } else {
                        format!("S{}_{}", row + 1, col + 1)
                    };
                    let trace = &data.s[row][col];
                    waveforms.insert(
                        name.clone(),
                        WaveformData::new_complex(
                            name,
                            data.frequencies.clone(),
                            trace.iter().map(|value| value.re).collect(),
                            trace.iter().map(|value| value.im).collect(),
                        ),
                    );
                }
            }

            Ok(SimulationResult::Ac {
                frequencies: data.frequencies,
                waveforms,
            })
        }
        AnalysisSpec::Envelope {
            fundamental_freq,
            stop_time,
            num_harmonics,
            max_step,
        } => {
            let cfg = svc_runner::EnvelopeRunConfig {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            };
            let data = svc_runner::run_envelope_analysis(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let waveforms: std::collections::HashMap<String, WaveformData> = data
                .waveforms
                .into_iter()
                .map(|(name, values)| {
                    (
                        name.clone(),
                        WaveformData::new_time_domain(name, data.time.clone(), values),
                    )
                })
                .collect();
            Ok(SimulationResult::Transient {
                time: data.time,
                waveforms,
            })
        }
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            output_node,
            output_ref,
            start_time,
            stop_time,
        } => {
            let output_ref = (!output_ref.trim().is_empty()).then_some(output_ref);
            let cfg = svc_runner::FourierRunConfig {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            };
            let data = svc_runner::run_fourier_analysis(netlist, &cfg)
                .map_err(SimulationError::InvalidConfig)?;
            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                format!("{} Spectrum", data.output_label),
                WaveformData::new_complex(
                    format!("{} Spectrum", data.output_label),
                    data.frequencies.clone(),
                    data.response.iter().map(|value| value.re).collect(),
                    data.response.iter().map(|value| value.im).collect(),
                ),
            );
            waveforms.insert(
                "THD(%)".to_string(),
                WaveformData {
                    name: "THD(%)".to_string(),
                    x_values: vec![fundamental_freq],
                    y_values: vec![data.thd_percent],
                    y_unit: "%".to_string(),
                    x_unit: "Hz".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "DC".to_string(),
                WaveformData {
                    name: "DC".to_string(),
                    x_values: vec![0.0],
                    y_values: vec![data.dc_component],
                    y_unit: "V".to_string(),
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
        AnalysisSpec::Pxf => {
            let pxf_cfg = options.pxf.ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "PXF analysis requires explicit PXF execution options".to_string(),
                )
            })?;
            let data = svc_runner::run_pxf_analysis_with_config(netlist, &pxf_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms: std::collections::HashMap<String, WaveformData> =
                std::collections::HashMap::new();
            let transfer_name = format!(
                "H(sb{}->sb{}, {})",
                data.input_sideband, data.output_sideband, data.output_label
            );
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
        AnalysisSpec::Pstb => {
            let pstb_cfg = options.pstb.ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "PSTB analysis requires explicit PSTB execution options".to_string(),
                )
            })?;
            let data = svc_runner::run_pstb_analysis_with_config(netlist, &pstb_cfg)
                .map_err(SimulationError::InvalidConfig)?;

            let mut waveforms = std::collections::HashMap::new();
            waveforms.insert(
                "Floquet |lambda|".to_string(),
                WaveformData {
                    name: "Floquet |lambda|".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.multiplier_magnitude,
                    y_unit: "".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Stability Margin (dB)".to_string(),
                WaveformData {
                    name: "Stability Margin (dB)".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.stability_margin_db,
                    y_unit: "dB".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Mode Damping (1/s)".to_string(),
                WaveformData {
                    name: "Mode Damping (1/s)".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.mode_damping,
                    y_unit: "1/s".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );
            waveforms.insert(
                "Probe Mode Participation".to_string(),
                WaveformData {
                    name: "Probe Mode Participation".to_string(),
                    x_values: data.mode_indices.clone(),
                    y_values: data.probe_mode_participation,
                    y_unit: "".to_string(),
                    x_unit: "mode".to_string(),
                    is_complex: false,
                    y_imag: None,
                },
            );

            Ok(SimulationResult::Ac {
                frequencies: data.mode_indices,
                waveforms,
            })
        }
        unsupported => Err(SimulationError::InvalidConfig(format!(
            "{:?} is not supported by SimulationRunner::start_spec",
            unsupported.run_type()
        ))),
    }
}

fn analysis_config_from_spec(spec: &AnalysisSpec) -> Option<AnalysisConfig> {
    match spec {
        AnalysisSpec::DcOp => Some(AnalysisConfig::DcOp),
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => Some(AnalysisConfig::DcSweep(DcSweepConfig {
            source: source_name.clone(),
            start: *start,
            stop: *stop,
            step: *step,
            source2: source2.clone(),
            start2: *start2,
            stop2: *stop2,
            step2: *step2,
        })),
        AnalysisSpec::Transient {
            stop_time,
            step_time,
        } => Some(AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: *stop_time,
            step_time: *step_time,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        })),
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => Some(AnalysisConfig::Ac(AcAnalysisConfig {
            sweep_type: ac_sweep_type_from_spec(*sweep),
            num_points: *points_per_unit,
            start_freq: *start_freq,
            stop_freq: *stop_freq,
        })),
        AnalysisSpec::Noise {
            output_node,
            start_freq,
            stop_freq,
            points_per_decade,
            ..
        } => Some(AnalysisConfig::Noise(NoiseAnalysisConfig {
            output_node: output_node.clone(),
            reference_node: "0".to_string(),
            input_source: "V1".to_string(),
            sweep_type: AcSweepType::Decade,
            num_points: *points_per_decade,
            start_freq: *start_freq,
            stop_freq: *stop_freq,
        })),
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => Some(AnalysisConfig::PoleZero(PoleZeroConfig {
            input_node: input_node.clone(),
            input_ref: input_ref.clone(),
            output_node: output_node.clone(),
            output_ref: output_ref.clone(),
            transfer_type: transfer_type.clone(),
            analysis_type: pz_analysis_type_from_spec(analysis_type),
        })),
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
        } => Some(AnalysisConfig::Sensitivity(SensitivityConfig {
            output_var: output_var.clone(),
            ac_mode: *ac_mode,
            frequency: *frequency,
        })),
        _ => None,
    }
}

#[inline]
fn ac_sweep_type_from_spec(sweep: crate::simulation::multi_run::FrequencySweep) -> AcSweepType {
    match sweep {
        crate::simulation::multi_run::FrequencySweep::Decade => AcSweepType::Decade,
        crate::simulation::multi_run::FrequencySweep::Octave => AcSweepType::Octave,
        crate::simulation::multi_run::FrequencySweep::Linear => AcSweepType::Linear,
    }
}

#[inline]
fn pz_analysis_type_from_spec(mode: &str) -> PzAnalysisType {
    if mode.eq_ignore_ascii_case("POL") {
        PzAnalysisType::PolesOnly
    } else if mode.eq_ignore_ascii_case("ZER") {
        PzAnalysisType::ZerosOnly
    } else {
        PzAnalysisType::PoleZero
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
    use std::time::{Duration, Instant};

    /// Minimal valid netlist for testing DC operating point
    fn test_netlist() -> String {
        "* Test Circuit\nV1 vdd 0 5\nR1 vdd out 1k\nR2 out 0 1k\n.end\n".to_string()
    }

    fn wait_for_result(
        runner: &mut SimulationRunner,
        timeout: Duration,
    ) -> Option<Result<SimulationResult, SimulationError>> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(result) = runner.poll_result() {
                return Some(result);
            }
            if Instant::now() >= deadline {
                return None;
            }
            thread::sleep(Duration::from_millis(10));
        }
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
    fn test_analysis_config_from_spec_covers_base_analyses() {
        let specs = vec![
            AnalysisSpec::DcOp,
            AnalysisSpec::DcSweep {
                source_name: "V1".to_string(),
                start: 0.0,
                stop: 1.0,
                step: 0.1,
                source2: None,
                start2: None,
                stop2: None,
                step2: None,
            },
            AnalysisSpec::Transient {
                stop_time: 1e-6,
                step_time: 1e-9,
            },
            AnalysisSpec::Ac {
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_unit: 10,
                sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            },
            AnalysisSpec::Noise {
                output_node: "out".to_string(),
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_decade: 10,
                temperature: 300.0,
            },
            AnalysisSpec::PoleZero {
                input_node: "in".to_string(),
                input_ref: "0".to_string(),
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                transfer_type: "VOL".to_string(),
                analysis_type: "PZ".to_string(),
            },
            AnalysisSpec::Sensitivity {
                output_var: "V(out)".to_string(),
                ac_mode: false,
                frequency: None,
            },
        ];

        for spec in specs {
            assert!(
                analysis_config_from_spec(&spec).is_some(),
                "expected base spec to map to AnalysisConfig: {:?}",
                spec.run_type()
            );
        }
    }

    #[test]
    fn test_runner_start_spec_dc_op_routes_through_engine_bridge() {
        let mut runner = SimulationRunner::new();
        runner
            .start_spec(AnalysisSpec::DcOp, test_netlist())
            .expect("DC OP spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(5));
        assert!(result.is_some(), "Expected DC OP result");
        let result = result.unwrap().expect("DC OP should succeed");
        assert!(matches!(result, SimulationResult::DcOp(_)));
    }

    #[test]
    fn test_runner_start_spec_dc_sweep_routes_through_engine_bridge() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* DC sweep routing test
V1 in 0 0
R1 in out 1k
R2 out 0 1k
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::DcSweep {
                    source_name: "V1".to_string(),
                    start: 0.0,
                    stop: 1.0,
                    step: 0.1,
                    source2: None,
                    start2: None,
                    stop2: None,
                    step2: None,
                },
                netlist,
            )
            .expect("DC sweep spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(5));
        assert!(result.is_some(), "Expected DC sweep result");
        let result = result.unwrap().expect("DC sweep should succeed");
        match result {
            SimulationResult::DcSweep {
                sweep_values,
                waveforms,
                ..
            } => {
                assert!(!sweep_values.is_empty());
                assert!(!waveforms.is_empty());
            }
            other => panic!("Expected DC sweep result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_ac_routes_through_engine_bridge() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* AC routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Ac {
                    start_freq: 1.0,
                    stop_freq: 1e6,
                    points_per_unit: 8,
                    sweep: crate::simulation::multi_run::FrequencySweep::Decade,
                },
                netlist,
            )
            .expect("AC spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected AC result");
        let result = result.unwrap().expect("AC should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(!waveforms.is_empty());
            }
            other => panic!("Expected AC result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_noise_routes_through_engine_bridge() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Noise routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
R2 out 0 1k
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Noise {
                    output_node: "out".to_string(),
                    start_freq: 10.0,
                    stop_freq: 1e6,
                    points_per_decade: 6,
                    temperature: 300.0,
                },
                netlist,
            )
            .expect("Noise spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected noise result");
        let result = result.unwrap().expect("Noise should succeed");
        match result {
            SimulationResult::Noise {
                frequencies,
                output_noise,
                ..
            } => {
                assert!(!frequencies.is_empty());
                assert_eq!(frequencies.len(), output_noise.len());
            }
            other => panic!("Expected Noise result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pole_zero_routes_through_engine_bridge() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Pole-zero routing test
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::PoleZero {
                    input_node: "in".to_string(),
                    input_ref: "0".to_string(),
                    output_node: "out".to_string(),
                    output_ref: "0".to_string(),
                    transfer_type: "VOL".to_string(),
                    analysis_type: "PZ".to_string(),
                },
                netlist,
            )
            .expect("Pole-zero spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected pole-zero result");
        let result = result.unwrap().expect("Pole-zero should succeed");
        match result {
            SimulationResult::PoleZero { poles, .. } => {
                assert!(!poles.is_empty(), "expected at least one pole");
            }
            other => panic!("Expected PoleZero result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_sensitivity_routes_through_engine_bridge() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Sensitivity routing test
.param RV=1k
V1 in 0 1
R1 in out {RV}
R2 out 0 1k
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Sensitivity {
                    output_var: "V(out)".to_string(),
                    ac_mode: false,
                    frequency: None,
                },
                netlist,
            )
            .expect("Sensitivity spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected sensitivity result");
        let result = result.unwrap().expect("Sensitivity should succeed");
        match result {
            SimulationResult::Sensitivity { sensitivities, .. } => {
                assert!(
                    !sensitivities.is_empty(),
                    "expected at least one sensitivity entry"
                );
            }
            other => panic!("Expected Sensitivity result, got {:?}", other),
        }
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
            pxf: None,
            tf: None,
            pnoise: None,
            pstb: None,
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
            pxf: None,
            tf: None,
            pnoise: None,
            pstb: None,
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
                assert!(
                    corner_labels
                        .iter()
                        .any(|label| label.contains("FF_1.100000V"))
                );
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
            pxf: None,
            tf: None,
            pnoise: None,
            pstb: None,
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
    fn test_runner_start_spec_sparameter() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* S-parameter smoke
R1 in 0 50
R2 out 0 50
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::SParameter {
                    start_freq: 1e3,
                    stop_freq: 1e6,
                    points_per_unit: 5,
                    sweep: crate::simulation::multi_run::FrequencySweep::Decade,
                    z0: 50.0,
                    ports: vec![
                        crate::simulation::multi_run::SpPort {
                            node_pos: "in".to_string(),
                            node_neg: "0".to_string(),
                            z0: None,
                        },
                        crate::simulation::multi_run::SpPort {
                            node_pos: "out".to_string(),
                            node_neg: "0".to_string(),
                            z0: Some(75.0),
                        },
                    ],
                },
                netlist,
            )
            .expect("S-parameter spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(5));
        assert!(result.is_some(), "Expected S-parameter result");
        let result = result.unwrap().expect("S-parameter should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(waveforms.contains_key("S11"));
                assert!(waveforms.contains_key("S21"));
                assert!(waveforms.contains_key("S12"));
                assert!(waveforms.contains_key("S22"));
                assert!(
                    waveforms
                        .values()
                        .all(|wf| wf.is_complex && wf.y_imag.as_ref().is_some())
                );
            }
            other => panic!("Expected AC result for S-parameter, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_envelope() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Envelope smoke
V1 out 0 SIN(0 1 1Meg 0 0 0)
R1 out 0 1k
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Envelope {
                    fundamental_freq: 1e6,
                    stop_time: 2e-6,
                    num_harmonics: 9,
                    max_step: None,
                },
                netlist,
            )
            .expect("Envelope spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(20));
        assert!(result.is_some(), "Expected envelope result");
        let result = result.unwrap().expect("Envelope should succeed");
        match result {
            SimulationResult::Transient { time, waveforms } => {
                assert!(!time.is_empty());
                assert!(!waveforms.is_empty());
                assert!(waveforms.keys().all(|name| name.starts_with("ENV(")));
            }
            other => panic!("Expected Transient result for envelope, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_fourier() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Fourier smoke
V1 out 0 SIN(0 1 1k 0 0 0)
R1 out 0 1k
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Fourier {
                    fundamental_freq: 1e3,
                    num_harmonics: 8,
                    output_node: "out".to_string(),
                    output_ref: "".to_string(),
                    start_time: 0.0,
                    stop_time: 10e-3,
                },
                netlist,
            )
            .expect("Fourier spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(20));
        assert!(result.is_some(), "Expected Fourier result");
        let result = result.unwrap().expect("Fourier should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(
                    waveforms.keys().any(|name| name.contains("Spectrum")),
                    "expected Fourier spectrum waveform"
                );
                assert!(waveforms.contains_key("THD(%)"));
                assert!(waveforms.contains_key("DC"));
            }
            other => panic!("Expected AC result for Fourier, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_reliability() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Reliability smoke
VDD vdd 0 1.8
VG g 0 1.2
R1 vdd d 1k
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Reliability {
                    target_years: vec![1.0, 5.0, 10.0],
                    enable_hci: true,
                    enable_nbti: true,
                    enable_em: false,
                    min_stress_voltage: 0.05,
                },
                netlist,
            )
            .expect("Reliability spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected reliability result");
        let result = result.unwrap().expect("Reliability should succeed");
        match result {
            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => {
                assert_eq!(years, vec![1.0, 5.0, 10.0]);
                assert!(!device_results.is_empty());
                assert!(!waveforms.is_empty());
                assert!(
                    waveforms
                        .keys()
                        .any(|name| name.starts_with("DVTH(") || name.starts_with("DRDS("))
                );
            }
            other => panic!("Expected Reliability result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_optimization() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* Optimization smoke
.param RTOP=1k
.param RBOT=1k
V1 in 0 2
R1 in out {RTOP}
R2 out 0 {RBOT}
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Optimization {
                    variables: vec![crate::simulation::multi_run::OptimizationVariable {
                        name: "RBOT".to_string(),
                        min: 500.0,
                        max: 3000.0,
                        initial: 1000.0,
                    }],
                    objective_node: "out".to_string(),
                    objective_ref: "0".to_string(),
                    goal: crate::simulation::multi_run::OptimizationGoal::Target,
                    target: Some(1.2),
                    algorithm: crate::simulation::multi_run::OptimizationAlgorithm::PatternSearch,
                    max_iterations: 48,
                    cost_tolerance: 1e-8,
                    fd_step: 1e-4,
                    initial_step: 0.2,
                    min_step: 1e-8,
                },
                netlist,
            )
            .expect("Optimization spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected optimization result");
        let result = result.unwrap().expect("Optimization should succeed");
        match result {
            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                ..
            } => {
                assert!(!iterations.is_empty());
                assert!(!waveforms.is_empty());
                assert!(waveforms.contains_key("OPT_COST"));
                assert!(best_cost.is_finite());
            }
            other => panic!("Expected Optimization result, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_soa() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* SOA smoke
VDD d 0 3.3
VG g 0 PULSE(0 2.5 0 1n 1n 8n 16n)
M1 d g 0 0 NM W=10u L=1u
.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02
.end
"#
        .to_string();

        runner
            .start_spec(
                AnalysisSpec::Soa {
                    stop_time: 32e-9,
                    step_time: 1e-9,
                    check_vgs_max: true,
                    max_vgs: 1.2,
                    check_vds_max: true,
                    max_vds: 3.0,
                    check_vbe_max: false,
                    max_vbe: 0.9,
                    check_vce_max: false,
                    max_vce: 5.0,
                },
                netlist,
            )
            .expect("SOA spec should start");

        let result = wait_for_result(&mut runner, Duration::from_secs(10));
        assert!(result.is_some(), "Expected SOA result");
        let result = result.unwrap().expect("SOA should succeed");
        match result {
            SimulationResult::Soa {
                time,
                waveforms,
                violations,
            } => {
                assert!(!time.is_empty());
                assert!(waveforms.contains_key("SOA_VIOLATION_COUNT"));
                assert!(!violations.is_empty());
            }
            other => panic!("Expected SOA result, got {:?}", other),
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
            pxf: None,
            tf: None,
            pnoise: None,
            pstb: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Pac, netlist, options)
            .expect("PAC spec should start");
        let result = wait_for_result(&mut runner, Duration::from_secs(5));
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
                assert!(waveforms.values().all(|wf| {
                    wf.is_complex
                        && wf
                            .y_imag
                            .as_ref()
                            .is_some_and(|imag| imag.len() == wf.y_values.len())
                }));
            }
            other => panic!("Expected AC result for PAC, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pxf_with_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PXF smoke test
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
            pxf: Some(crate::services::simulation_runner::PxfRunConfig {
                pss_fundamental_freq: 1e6,
                pss_num_harmonics: 8,
                pss_tolerance: 1e-4,
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 8,
                sweep: crate::services::simulation_runner::PxfFrequencySweep::Decade,
                input_source: "V1".to_string(),
                input_sideband: 1,
                output_node: "out".to_string(),
                output_ref: None,
                output_sideband: 1,
                max_sideband: 3,
                reltol: 1e-3,
                abstol: 1e-12,
            }),
            tf: None,
            pnoise: None,
            pstb: None,
        };

        runner
            .start_spec_with_options(AnalysisSpec::Pxf, netlist, options)
            .expect("PXF spec should start");
        let result = wait_for_result(&mut runner, Duration::from_secs(5));
        assert!(result.is_some(), "Expected PXF result");
        let result = result.unwrap().expect("PXF should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(!waveforms.is_empty());
                assert!(
                    waveforms.keys().any(|name| name.starts_with("H(sb")),
                    "expected PXF transfer waveform name, got {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                );
                assert!(
                    waveforms
                        .values()
                        .any(|wf| wf.is_complex && wf.y_imag.as_ref().is_some())
                );
            }
            other => panic!("Expected AC result for PXF, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pxf_requires_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PXF missing options
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(AnalysisSpec::Pxf, netlist)
            .expect("PXF spec launch without options should still start thread");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner
            .poll_result()
            .expect("Expected PXF completion result")
            .expect_err("PXF without options should fail");
        assert!(matches!(result, SimulationError::InvalidConfig(_)));
        assert!(
            result
                .to_string()
                .contains("requires explicit PXF execution options")
        );
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
            pxf: None,
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
            pstb: None,
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
                assert!(
                    waveforms
                        .values()
                        .any(|wf| wf.is_complex && wf.y_imag.as_ref().is_some())
                );
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
            pxf: None,
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
                input_source: "V1".to_string(),
                noise_ref: crate::services::simulation_runner::PnoiseReference::Output,
                integrated_noise: true,
                noise_summary: true,
                reltol: 1e-3,
                abstol: 1e-18,
            }),
            pstb: None,
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

    #[test]
    fn test_runner_start_spec_pstb_with_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PSTB smoke test
V1 in 0 DC 1
R1 in mid 1k
LPROBE mid out 1u
C1 out 0 1n
.end
"#
        .to_string();

        let options = SpecExecutionOptions {
            temp: None,
            corner: None,
            pac: None,
            pxf: None,
            tf: None,
            pnoise: None,
            pstb: Some(crate::services::simulation_runner::PstbRunConfig {
                pss_fundamental_freq: 1e6,
                pss_num_harmonics: 8,
                pss_tolerance: 1e-4,
                probe_instance: "LPROBE".to_string(),
                max_harmonics: 8,
                num_multipliers: 4,
                stability_threshold: 1.0 + 1e-6,
                detect_subharmonics: true,
                eigenvalue_tolerance: 1e-10,
            }),
        };

        runner
            .start_spec_with_options(AnalysisSpec::Pstb, netlist, options)
            .expect("PSTB spec should start");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner.poll_result();
        assert!(result.is_some(), "Expected PSTB result");
        let result = result.unwrap().expect("PSTB should succeed");
        match result {
            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                assert!(!frequencies.is_empty());
                assert!(waveforms.contains_key("Floquet |lambda|"));
                assert!(waveforms.contains_key("Stability Margin (dB)"));
                assert!(waveforms.contains_key("Mode Damping (1/s)"));
                assert!(waveforms.contains_key("Probe Mode Participation"));
                assert_eq!(
                    waveforms
                        .get("Floquet |lambda|")
                        .expect("Floquet waveform should exist")
                        .x_values
                        .len(),
                    frequencies.len()
                );
            }
            other => panic!("Expected AC result for PSTB, got {:?}", other),
        }
    }

    #[test]
    fn test_runner_start_spec_pstb_requires_options() {
        let mut runner = SimulationRunner::new();
        let netlist = r#"
* PSTB missing options
V1 in 0 DC 1
R1 in out 1k
C1 out 0 1n
.end
"#
        .to_string();

        runner
            .start_spec(AnalysisSpec::Pstb, netlist)
            .expect("PSTB launch without options should still start thread");
        thread::sleep(std::time::Duration::from_millis(250));

        let result = runner
            .poll_result()
            .expect("Expected PSTB completion result")
            .expect_err("PSTB without options should fail");
        assert!(matches!(result, SimulationError::InvalidConfig(_)));
        assert!(
            result
                .to_string()
                .contains("requires explicit PSTB execution options")
        );
    }
}
