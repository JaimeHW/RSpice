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

use std::collections::VecDeque;

use crate::common::app::{AppState, ConsoleMessage};
use crate::services::safety::SoAManager;
use crate::services::yield_manager::YieldAnalysisManager;
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, DcSweepConfig, NoiseAnalysisConfig, PoleZeroConfig,
    PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{AnalysisPlan, AnalysisRunType, AnalysisSpec, FrequencySweep};
use crate::simulation::reliability_engine::ReliabilityEngine;
use crate::simulation::runner::SpecExecutionOptions;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue};

#[derive(Debug, Clone)]
struct QueuedAnalysis {
    spec: AnalysisSpec,
    config: Option<AnalysisConfig>,
    spec_options: SpecExecutionOptions,
    analysis_line: String,
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
    /// Manager for safety checking (SOA)
    soa_manager: SoAManager,
    /// Engine for reliability analysis (Aging)
    reliability_engine: ReliabilityEngine,
    /// Current analysis config (stored for result processing when polling completes)
    current_config: Option<AnalysisConfig>,
    /// Current strongly-typed analysis spec (always set while running)
    current_spec: Option<AnalysisSpec>,

    // =========================================================================
    // Multi-Analysis Queue
    // =========================================================================
    /// Queue of pending analyses to run in current simulation batch
    pending_analyses: VecDeque<QueuedAnalysis>,
    /// Current analysis index (1-based for display: "Analysis 2/4")
    current_analysis_idx: usize,
    /// Total number of analyses in current batch
    total_analyses: usize,
    /// Cached netlist for multi-analysis runs (avoids regeneration)
    cached_netlist: Option<String>,
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
            soa_manager: SoAManager::new(),
            reliability_engine: ReliabilityEngine::new(),
            current_config: None,
            current_spec: None,
            pending_analyses: VecDeque::new(),
            current_analysis_idx: 0,
            total_analyses: 0,
            cached_netlist: None,
        }
    }

    /// Process simulation state updates
    ///
    /// Call this once per frame in the app update loop.
    pub fn update(&mut self, state: &mut AppState) {
        // Handle simulation trigger
        if state.simulation.trigger_simulation {
            log::info!("Simulation triggered! Tab={}", state.dialogs.sim_active_tab);
            state.simulation.trigger_simulation = false;
            self.start_simulation(state);
        }

        // Handle abort trigger
        if state.simulation.trigger_abort {
            log::info!("Simulation abort triggered!");
            state.simulation.trigger_abort = false;
            self.runner.abort();
            // Clear pending analyses since we're stopping
            self.pending_analyses.clear();
            self.cached_netlist = None;
            self.current_config = None;
            self.current_spec = None;
            state.simulation.status = "Aborted".to_string();
            state
                .console_messages
                .push(crate::common::app::ConsoleMessage::warning(
                    "Simulation aborted by user",
                ));
        }

        // Poll for completion
        self.poll_completion(state);

        // Update running state
        state.simulation.is_running = self.runner.is_running();
    }

    /// Start a new simulation batch
    ///
    /// Builds all enabled analyses into a queue and starts the first one.
    /// Subsequent analyses are started automatically upon completion.
    fn start_simulation(&mut self, state: &mut AppState) {
        log::info!("start_simulation called");

        self.pending_analyses.clear();
        let plan = match self.build_analysis_plan(state) {
            Ok(plan) => plan,
            Err(errors) => {
                for err in errors {
                    state.console_messages.push(ConsoleMessage::error(err));
                }
                state.simulation.status = "Configuration error".to_string();
                return;
            }
        };

        let queued = match self.build_queue_from_plan(state, &plan) {
            Ok(queue) => queue,
            Err(errors) => {
                for err in errors {
                    state.console_messages.push(ConsoleMessage::error(err));
                }
                state.simulation.status = "Configuration error".to_string();
                return;
            }
        };

        self.total_analyses = queued.len();
        if self.total_analyses == 0 {
            state.console_messages.push(ConsoleMessage::error(
                "No runnable analyses were selected".to_string(),
            ));
            state.simulation.status = "Configuration error".to_string();
            return;
        }
        self.current_analysis_idx = 0;

        let analysis_lines: Vec<String> = queued
            .iter()
            .map(|item| item.analysis_line.clone())
            .collect();
        let result = crate::simulation::netlist_gen::generate_netlist_with_analysis(
            &state.schematic,
            &analysis_lines,
        );
        let netlist = result.netlist.clone();

        if !result.errors.is_empty() {
            for err in result.errors {
                state.console_messages.push(ConsoleMessage::error(err));
            }
            state.simulation.status = "Netlist error".to_string();
            return;
        }
        for warning in result.warnings {
            state
                .console_messages
                .push(ConsoleMessage::warning(warning));
        }

        log::info!(
            "Generated netlist ({} bytes):\n{}",
            netlist.len(),
            &netlist[..netlist.len().min(500)]
        );

        // Populate cross-probe mapping for probe mode
        state
            .simulation
            .cross_probe
            .update(result.point_to_net, result.nets);
        log::info!(
            "Cross-probe mapping populated: {} points, {} nets",
            state.simulation.cross_probe.point_to_net.len(),
            state.simulation.cross_probe.net_to_points.len()
        );

        self.pending_analyses = queued.into_iter().collect();
        self.cached_netlist = Some(netlist);

        let queued_names: Vec<&'static str> = self
            .pending_analyses
            .iter()
            .map(|entry| self.analysis_name_for_spec(&entry.spec))
            .collect();
        log::info!(
            "Queued {} analyses for execution: {:?}",
            self.total_analyses,
            queued_names
        );

        // Create new run in Results Browser
        state.simulation.start_run();
        log::info!("Created new simulation run");

        // Log summary to console
        if self.total_analyses > 1 {
            state.console_messages.push(ConsoleMessage::info(format!(
                "Starting simulation batch: {} analyses",
                self.total_analyses
            )));
        }

        // Start the first analysis
        self.start_next_analysis(state);
    }

    /// Start the next analysis in the queue
    ///
    /// Called after start_simulation() initializes the queue, and again
    /// after each analysis completes until the queue is empty.
    fn start_next_analysis(&mut self, state: &mut AppState) {
        let Some(next_analysis) = self.pending_analyses.pop_front() else {
            // Queue exhausted - should not happen if called correctly
            log::warn!("start_next_analysis called with empty queue");
            return;
        };
        log::info!(
            "Starting queued analysis: {:?}",
            next_analysis.spec.run_type()
        );
        let spec = next_analysis.spec;
        let config = next_analysis.config;
        let spec_options = next_analysis.spec_options;
        let analysis_name = self.analysis_name_for_spec(&spec);

        self.current_analysis_idx += 1;
        self.current_config = config.clone();
        self.current_spec = Some(spec.clone());

        // Update status with multi-analysis progress
        let status_msg = if self.total_analyses > 1 {
            format!(
                "Analysis {}/{}: {}",
                self.current_analysis_idx, self.total_analyses, analysis_name
            )
        } else {
            analysis_name.to_string()
        };
        state.simulation.status = status_msg.clone();

        // Log to console
        state.console_messages.push(ConsoleMessage::info(format!(
            "Starting {}...",
            if self.total_analyses > 1 {
                format!(
                    "{} ({}/{})",
                    analysis_name, self.current_analysis_idx, self.total_analyses
                )
            } else {
                analysis_name.to_string()
            }
        )));

        // Use cached netlist
        let netlist = self
            .cached_netlist
            .clone()
            .expect("Netlist should be cached");

        // Start the simulation
        let start_result = if let Some(cfg) = config {
            self.runner.start(cfg, netlist)
        } else {
            self.runner
                .start_spec_with_options(spec, netlist, spec_options)
        };
        match start_result {
            Ok(()) => log::info!(
                "Analysis {}/{} started successfully",
                self.current_analysis_idx,
                self.total_analyses
            ),
            Err(e) => {
                log::error!("Failed to start simulation: {}", e);
                state.console_messages.push(ConsoleMessage::error(format!(
                    "Failed to start simulation: {}",
                    e
                )));
                // Mark run as failed but continue with remaining analyses
                if let Some(run) = state.simulation.active_run_mut() {
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

    /// Finish the simulation batch and clean up state
    fn finish_simulation_batch(&mut self, state: &mut AppState) {
        // Complete the run (syncs waveforms and selects first analysis)
        state.simulation.complete_run();

        // Clear cached netlist
        self.cached_netlist = None;
        self.current_config = None;
        self.current_spec = None;
        self.current_analysis_idx = 0;
        self.total_analyses = 0;

        state.simulation.status = "Complete".to_string();

        log::info!("Simulation batch completed");
    }

    fn enabled_analysis_indices(state: &AppState) -> Vec<usize> {
        let mut indices: Vec<usize> = state.dialogs.enabled_analyses.iter().copied().collect();
        indices.sort_unstable();
        if indices.is_empty() {
            indices.push(0);
        }
        indices
    }

    fn analysis_label_for_index(idx: usize) -> &'static str {
        match idx {
            0 => "DC Operating Point",
            1 => "Transient",
            2 => "AC",
            3 => "DC Sweep",
            4 => "Noise",
            5 => "Pole-Zero",
            6 => "Sensitivity",
            7 => "Monte Carlo",
            8 => "PSS",
            9 => "STB",
            10 => "Temperature Sweep",
            11 => "Harmonic Balance",
            12 => "S-Parameter",
            13 => "PAC",
            14 => "PNoise",
            15 => "PXF",
            16 => "PSTB",
            17 => "Transfer Function",
            18 => "Corner",
            19 => "Envelope",
            20 => "Fourier",
            21 => "Reliability",
            22 => "Optimization",
            23 => "Safety (SOA)",
            _ => "Unknown",
        }
    }

    fn build_analysis_plan(&self, state: &AppState) -> Result<AnalysisPlan, Vec<String>> {
        let mut plan = AnalysisPlan::new();
        plan.stop_on_error = false;

        let mut errors = Vec::new();
        for idx in Self::enabled_analysis_indices(state) {
            match self.build_analysis_spec_for_index(state, idx) {
                Ok(spec) => plan.analyses.push(spec),
                Err(e) => errors.push(format!("{}: {}", Self::analysis_label_for_index(idx), e)),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        plan.validate()?;
        Ok(plan)
    }

    fn build_queue_from_plan(
        &self,
        state: &AppState,
        plan: &AnalysisPlan,
    ) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
        let mut queue = Vec::with_capacity(plan.analyses.len());
        let mut errors = Vec::new();

        for spec in &plan.analyses {
            let analysis_line = match self.analysis_spec_to_spice_line(state, spec) {
                Ok(line) => line,
                Err(e) => {
                    errors.push(format!("{}: {}", spec.run_type().display_name(), e));
                    continue;
                }
            };
            let spec_options = match self.analysis_spec_execution_options(state, spec) {
                Ok(opts) => opts,
                Err(e) => {
                    errors.push(format!("{}: {}", spec.run_type().display_name(), e));
                    continue;
                }
            };

            if Self::executes_via_spec(spec) {
                queue.push(QueuedAnalysis {
                    spec: spec.clone(),
                    config: None,
                    spec_options,
                    analysis_line,
                });
                continue;
            }

            match self.analysis_spec_to_config(state, spec) {
                Ok(config) => {
                    if let Err(errs) = config.validate() {
                        errors.push(format!(
                            "{} config is invalid: {}",
                            spec.run_type().display_name(),
                            errs.join(", ")
                        ));
                    } else {
                        queue.push(QueuedAnalysis {
                            spec: spec.clone(),
                            config: Some(config),
                            spec_options,
                            analysis_line,
                        });
                    }
                }
                Err(e) => errors.push(format!("{}: {}", spec.run_type().display_name(), e)),
            }
        }

        if errors.is_empty() {
            Ok(queue)
        } else {
            Err(errors)
        }
    }

    fn executes_via_spec(spec: &AnalysisSpec) -> bool {
        matches!(
            spec,
            AnalysisSpec::MonteCarlo | AnalysisSpec::Parametric | AnalysisSpec::Corner
        )
    }

    fn analysis_spec_execution_options(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<SpecExecutionOptions, String> {
        match spec {
            AnalysisSpec::Parametric => {
                let mut temp_state = state.dialogs.temp_state.clone();
                temp_state.ensure_initialized();
                let temp_cfg = temp_state
                    .to_config()
                    .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    temp: Some(Self::temp_run_config_from_dialog(state, &temp_cfg)?),
                    corner: None,
                })
            }
            AnalysisSpec::Corner => {
                let mut corner_state = state.dialogs.corner_state.clone();
                corner_state.ensure_initialized();
                let corner_cfg = corner_state
                    .to_config()
                    .map_err(|e| format!("invalid corner settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    temp: None,
                    corner: Some(Self::corner_run_config_from_dialog(state, &corner_cfg)?),
                })
            }
            _ => Ok(SpecExecutionOptions::default()),
        }
    }

    fn temp_run_config_from_dialog(
        state: &AppState,
        temp_cfg: &crate::simulation::dialog::temp::TempConfig,
    ) -> Result<crate::services::simulation_runner::TempRunConfig, String> {
        use crate::services::simulation_runner::{
            CornerBaseMode, CornerFrequencySweep, TempRunConfig,
        };
        use crate::simulation::dialog::temp::TempBaseAnalysis;

        let temperatures_c = if !temp_cfg.specific_temps.is_empty() {
            temp_cfg.specific_temps.clone()
        } else {
            Self::expand_temperature_points(
                temp_cfg.temp_start,
                temp_cfg.temp_stop,
                temp_cfg.temp_step,
            )?
        };

        let base_mode = match temp_cfg.base_analysis {
            TempBaseAnalysis::Op => CornerBaseMode::Op,
            TempBaseAnalysis::Dc => {
                let source_name = state.dialogs.dc_source.trim();
                if source_name.is_empty() {
                    return Err(
                        "temperature sweep DC base analysis requires a non-empty sweep source"
                            .to_string(),
                    );
                }
                CornerBaseMode::DcSweep {
                    source_name: source_name.to_string(),
                    start: parse_spice_value_checked(&state.dialogs.dc_start)
                        .map_err(|e| format!("invalid temperature DC start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.dialogs.dc_stop)
                        .map_err(|e| format!("invalid temperature DC stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.dialogs.dc_step)
                        .map_err(|e| format!("invalid temperature DC step value: {}", e))?,
                }
            }
            TempBaseAnalysis::Transient => CornerBaseMode::Transient {
                stop_time: parse_spice_value_checked(&state.dialogs.tran_stop)
                    .map_err(|e| format!("invalid temperature transient stop time: {}", e))?,
                step_time: parse_spice_value_checked(&state.dialogs.tran_step)
                    .map_err(|e| format!("invalid temperature transient step time: {}", e))?,
            },
            TempBaseAnalysis::Ac => {
                let sweep = match Self::map_frequency_sweep(state.dialogs.ac_sweep_type) {
                    FrequencySweep::Decade => CornerFrequencySweep::Decade,
                    FrequencySweep::Octave => CornerFrequencySweep::Octave,
                    FrequencySweep::Linear => CornerFrequencySweep::Linear,
                };
                CornerBaseMode::Ac {
                    start_freq: parse_spice_value_checked(&state.dialogs.ac_fstart)
                        .map_err(|e| format!("invalid temperature AC start frequency: {}", e))?,
                    stop_freq: parse_spice_value_checked(&state.dialogs.ac_fstop)
                        .map_err(|e| format!("invalid temperature AC stop frequency: {}", e))?,
                    points_per_unit: Self::parse_positive_points(
                        &state.dialogs.ac_points,
                        "ac_points",
                    )
                    .map_err(|e| format!("invalid temperature AC points: {}", e))?,
                    sweep,
                }
            }
        };

        Ok(TempRunConfig {
            temperatures_c,
            base_mode,
        })
    }

    fn corner_run_config_from_dialog(
        state: &AppState,
        corner_cfg: &crate::simulation::dialog::corner::CornerConfig,
    ) -> Result<crate::services::simulation_runner::CornerRunConfig, String> {
        use crate::services::simulation_runner::{
            CornerBaseMode, CornerFrequencySweep, CornerProcess, CornerRunConfig,
        };
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, ProcessCorner};

        let process_corners = corner_cfg
            .process_corners
            .iter()
            .map(|corner| match corner {
                ProcessCorner::TT => CornerProcess::TT,
                ProcessCorner::SS => CornerProcess::SS,
                ProcessCorner::FF => CornerProcess::FF,
                ProcessCorner::SF => CornerProcess::SF,
                ProcessCorner::FS => CornerProcess::FS,
            })
            .collect();

        let nominal_voltage = match corner_cfg.voltages.len() {
            0 => None,
            1 => Some(corner_cfg.voltages[0]),
            n => Some(corner_cfg.voltages[n / 2]),
        };

        let base_mode = match corner_cfg.base_analysis {
            CornerBaseAnalysis::Op => CornerBaseMode::Op,
            CornerBaseAnalysis::Dc => {
                let source_name = state.dialogs.dc_source.trim();
                if source_name.is_empty() {
                    return Err(
                        "corner DC base analysis requires a non-empty sweep source".to_string()
                    );
                }
                CornerBaseMode::DcSweep {
                    source_name: source_name.to_string(),
                    start: parse_spice_value_checked(&state.dialogs.dc_start)
                        .map_err(|e| format!("invalid corner DC start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.dialogs.dc_stop)
                        .map_err(|e| format!("invalid corner DC stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.dialogs.dc_step)
                        .map_err(|e| format!("invalid corner DC step value: {}", e))?,
                }
            }
            CornerBaseAnalysis::Transient => CornerBaseMode::Transient {
                stop_time: parse_spice_value_checked(&state.dialogs.tran_stop)
                    .map_err(|e| format!("invalid corner transient stop time: {}", e))?,
                step_time: parse_spice_value_checked(&state.dialogs.tran_step)
                    .map_err(|e| format!("invalid corner transient step time: {}", e))?,
            },
            CornerBaseAnalysis::Ac => {
                let sweep = match Self::map_frequency_sweep(state.dialogs.ac_sweep_type) {
                    FrequencySweep::Decade => CornerFrequencySweep::Decade,
                    FrequencySweep::Octave => CornerFrequencySweep::Octave,
                    FrequencySweep::Linear => CornerFrequencySweep::Linear,
                };
                CornerBaseMode::Ac {
                    start_freq: parse_spice_value_checked(&state.dialogs.ac_fstart)
                        .map_err(|e| format!("invalid corner AC start frequency: {}", e))?,
                    stop_freq: parse_spice_value_checked(&state.dialogs.ac_fstop)
                        .map_err(|e| format!("invalid corner AC stop frequency: {}", e))?,
                    points_per_unit: Self::parse_positive_points(
                        &state.dialogs.ac_points,
                        "ac_points",
                    )
                    .map_err(|e| format!("invalid corner AC points: {}", e))?,
                    sweep,
                }
            }
        };

        Ok(CornerRunConfig {
            process_corners,
            voltages: corner_cfg.voltages.clone(),
            temperatures_c: corner_cfg.temperatures.clone(),
            full_matrix: corner_cfg.full_matrix,
            nominal_voltage,
            base_mode,
        })
    }

    fn build_analysis_spec_for_index(
        &self,
        state: &AppState,
        idx: usize,
    ) -> Result<AnalysisSpec, String> {
        match idx {
            0 => Ok(AnalysisSpec::DcOp),
            1 => Ok(AnalysisSpec::Transient {
                stop_time: parse_spice_value_checked(&state.dialogs.tran_stop)
                    .map_err(|e| format!("invalid stop time: {}", e))?,
                step_time: parse_spice_value_checked(&state.dialogs.tran_step)
                    .map_err(|e| format!("invalid step time: {}", e))?,
            }),
            2 => Ok(AnalysisSpec::Ac {
                start_freq: parse_spice_value_checked(&state.dialogs.ac_fstart)
                    .map_err(|e| format!("invalid start frequency: {}", e))?,
                stop_freq: parse_spice_value_checked(&state.dialogs.ac_fstop)
                    .map_err(|e| format!("invalid stop frequency: {}", e))?,
                points_per_unit: Self::parse_positive_points(
                    &state.dialogs.ac_points,
                    "ac_points",
                )?,
                sweep: Self::map_frequency_sweep(state.dialogs.ac_sweep_type),
            }),
            3 => {
                if state.dialogs.dc_nested {
                    return Err(
                        "nested DC sweep is not supported by the current simulation engine"
                            .to_string(),
                    );
                }
                Ok(AnalysisSpec::DcSweep {
                    source_name: state.dialogs.dc_source.trim().to_string(),
                    start: parse_spice_value_checked(&state.dialogs.dc_start)
                        .map_err(|e| format!("invalid start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.dialogs.dc_stop)
                        .map_err(|e| format!("invalid stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.dialogs.dc_step)
                        .map_err(|e| format!("invalid step value: {}", e))?,
                })
            }
            4 => Ok(AnalysisSpec::Noise {
                output_node: state.dialogs.noise_output.trim().to_string(),
                start_freq: parse_spice_value_checked(&state.dialogs.noise_fstart)
                    .map_err(|e| format!("invalid start frequency: {}", e))?,
                stop_freq: parse_spice_value_checked(&state.dialogs.noise_fstop)
                    .map_err(|e| format!("invalid stop frequency: {}", e))?,
                points_per_decade: Self::parse_positive_points(
                    &state.dialogs.ac_points,
                    "ac_points",
                )?,
                temperature: 300.0,
            }),
            5 => self.build_pole_zero_spec(state),
            6 => self.build_sensitivity_spec(state),
            7 => self.build_monte_carlo_spec(state),
            10 => self.build_temperature_sweep_spec(state),
            18 => self.build_corner_sweep_spec(state),
            _ => Err(
                "analysis is not implemented in the current UI simulation controller".to_string(),
            ),
        }
    }

    fn analysis_spec_to_config(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<AnalysisConfig, String> {
        match spec {
            AnalysisSpec::DcOp => Ok(AnalysisConfig::DcOp),
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => Ok(AnalysisConfig::DcSweep(DcSweepConfig {
                source: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: None,
                start2: None,
                stop2: None,
                step2: None,
            })),
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Ok(AnalysisConfig::Ac(AcAnalysisConfig {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                num_points: *points_per_unit,
                sweep_type: Self::map_ac_sweep(*sweep),
            })),
            AnalysisSpec::Transient {
                stop_time,
                step_time,
            } => Ok(AnalysisConfig::Transient(TransientAnalysisConfig {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: parse_spice_value_checked(&state.dialogs.tran_start)
                    .map_err(|e| format!("invalid start time: {}", e))?,
                max_timestep: Self::parse_optional_spice_value(&state.dialogs.tran_maxstep)
                    .map_err(|e| format!("invalid max step: {}", e))?,
                uic: state.dialogs.tran_uic,
            })),
            AnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                ..
            } => Ok(AnalysisConfig::Noise(NoiseAnalysisConfig {
                output_node: output_node.clone(),
                reference_node: state.dialogs.noise_ref.trim().to_string(),
                input_source: state.dialogs.noise_input.trim().to_string(),
                sweep_type: Self::map_ac_sweep(Self::map_frequency_sweep(
                    state.dialogs.ac_sweep_type,
                )),
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
            } => {
                let analysis_type = match analysis_type.trim().to_ascii_uppercase().as_str() {
                    "PZ" => PzAnalysisType::PoleZero,
                    "POL" => PzAnalysisType::PolesOnly,
                    "ZER" => PzAnalysisType::ZerosOnly,
                    other => {
                        return Err(format!(
                            "invalid pole-zero analysis type '{}': expected PZ, POL, or ZER",
                            other
                        ));
                    }
                };
                let transfer_type = transfer_type.trim().to_ascii_uppercase();
                if transfer_type != "VOL" && transfer_type != "CUR" {
                    return Err(format!(
                        "invalid pole-zero transfer type '{}': expected VOL or CUR",
                        transfer_type
                    ));
                }
                Ok(AnalysisConfig::PoleZero(PoleZeroConfig {
                    input_node: input_node.clone(),
                    input_ref: input_ref.clone(),
                    output_node: output_node.clone(),
                    output_ref: output_ref.clone(),
                    transfer_type,
                    analysis_type,
                }))
            }
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => Ok(AnalysisConfig::Sensitivity(SensitivityConfig {
                output_var: output_var.clone(),
                ac_mode: *ac_mode,
                frequency: *frequency,
            })),
            _ => Err(format!(
                "{:?} is not supported by the UI runner yet",
                spec.run_type()
            )),
        }
    }

    fn analysis_spec_to_spice_line(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
    ) -> Result<String, String> {
        match spec {
            AnalysisSpec::MonteCarlo => self.build_monte_carlo_command(state),
            AnalysisSpec::Parametric => self.build_temperature_step_command(state),
            AnalysisSpec::Corner => self.build_corner_temp_command(state),
            _ => self
                .analysis_spec_to_config(state, spec)
                .map(|cfg| cfg.to_spice()),
        }
    }

    fn build_monte_carlo_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut mc_state = state.dialogs.mc_state.clone();
        mc_state.ensure_initialized();
        mc_state
            .to_config()
            .map_err(|e| format!("invalid Monte Carlo settings: {}", e))?;
        Ok(AnalysisSpec::MonteCarlo)
    }

    fn build_temperature_sweep_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut temp_state = state.dialogs.temp_state.clone();
        temp_state.ensure_initialized();
        temp_state
            .to_config()
            .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
        Ok(AnalysisSpec::Parametric)
    }

    fn build_corner_sweep_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut corner_state = state.dialogs.corner_state.clone();
        corner_state.ensure_initialized();
        corner_state
            .to_config()
            .map_err(|e| format!("invalid corner settings: {}", e))?;
        Ok(AnalysisSpec::Corner)
    }

    fn build_monte_carlo_command(&self, state: &AppState) -> Result<String, String> {
        let mut mc_state = state.dialogs.mc_state.clone();
        mc_state.ensure_initialized();
        let mc_cfg = mc_state
            .to_config()
            .map_err(|e| format!("invalid Monte Carlo settings: {}", e))?;

        let dist_keyword =
            match mc_cfg.distribution {
                crate::simulation::dialog::mc::McDistribution::Gaussian => "GAUSS",
                crate::simulation::dialog::mc::McDistribution::Uniform => "UNIFORM",
                crate::simulation::dialog::mc::McDistribution::WorstCase => return Err(
                    "Monte Carlo 'Worst Case' distribution is not supported by the core .MC parser"
                        .to_string(),
                ),
            };
        let relative_spread = (mc_cfg.variation_pct / 100.0).abs();
        let mut cmd = format!(
            ".mc {} DIST {} SPREAD {:.12e}",
            mc_cfg.num_runs, dist_keyword, relative_spread
        );
        if mc_cfg.seed > 0 {
            cmd.push_str(&format!(" SEED {}", mc_cfg.seed));
        }
        Ok(cmd)
    }

    fn build_temperature_step_command(&self, state: &AppState) -> Result<String, String> {
        let mut temp_state = state.dialogs.temp_state.clone();
        temp_state.ensure_initialized();
        let temp_cfg = temp_state
            .to_config()
            .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;

        if !temp_cfg.specific_temps.is_empty() {
            let values: Vec<String> = temp_cfg
                .specific_temps
                .iter()
                .map(|t| format!("{:.12e}", t))
                .collect();
            Ok(format!(".step temp list {}", values.join(" ")))
        } else {
            Ok(format!(
                ".step temp {:.12e} {:.12e} {:.12e}",
                temp_cfg.temp_start, temp_cfg.temp_stop, temp_cfg.temp_step
            ))
        }
    }

    fn build_corner_temp_command(&self, state: &AppState) -> Result<String, String> {
        let mut corner_state = state.dialogs.corner_state.clone();
        corner_state.ensure_initialized();
        let corner_cfg = corner_state
            .to_config()
            .map_err(|e| format!("invalid corner settings: {}", e))?;

        if corner_cfg.temperatures.is_empty() {
            return Err("corner analysis requires at least one temperature".to_string());
        }
        let temps: Vec<String> = corner_cfg
            .temperatures
            .iter()
            .map(|temp| format!("{:.12e}", temp))
            .collect();
        Ok(format!(".temp {}", temps.join(" ")))
    }

    fn build_pole_zero_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pz_state = state.dialogs.pz_state.clone();
        pz_state.ensure_initialized();
        let pz_cfg = pz_state
            .to_config()
            .map_err(|e| format!("invalid pole-zero settings: {}", e))?;

        let analysis_type = match pz_cfg.analysis_type {
            crate::simulation::dialog::pz::PzAnalysisType::PolesAndZeros => {
                PzAnalysisType::PoleZero
            }
            crate::simulation::dialog::pz::PzAnalysisType::PolesOnly => PzAnalysisType::PolesOnly,
            crate::simulation::dialog::pz::PzAnalysisType::ZerosOnly => PzAnalysisType::ZerosOnly,
        };

        let transfer_type = match pz_cfg.transfer_type {
            crate::simulation::dialog::pz::PzTransferType::Voltage => "VOL",
            crate::simulation::dialog::pz::PzTransferType::Current => "CUR",
        };

        Ok(AnalysisSpec::PoleZero {
            input_node: pz_cfg.input_pos,
            input_ref: pz_cfg.input_neg,
            output_node: pz_cfg.output_pos,
            output_ref: pz_cfg.output_neg,
            transfer_type: transfer_type.to_string(),
            analysis_type: match analysis_type {
                PzAnalysisType::PoleZero => "PZ".to_string(),
                PzAnalysisType::PolesOnly => "POL".to_string(),
                PzAnalysisType::ZerosOnly => "ZER".to_string(),
            },
        })
    }

    fn build_sensitivity_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut sens_state = state.dialogs.sens_state.clone();
        sens_state.ensure_initialized();
        let sens_cfg = sens_state
            .to_config()
            .map_err(|e| format!("invalid sensitivity settings: {}", e))?;

        let ac_mode = matches!(
            sens_cfg.sens_type,
            crate::simulation::dialog::sens::SensType::Ac
        );

        Ok(AnalysisSpec::Sensitivity {
            output_var: sens_cfg.output_expr,
            ac_mode,
            frequency: ac_mode.then_some(sens_cfg.ac_freq),
        })
    }

    fn expand_temperature_points(start: f64, stop: f64, step: f64) -> Result<Vec<f64>, String> {
        if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
            return Err(
                "temperature sweep range requires finite start/stop/step values".to_string(),
            );
        }
        if step == 0.0 {
            return Err("temperature sweep step cannot be zero".to_string());
        }
        if (stop - start).abs() > 0.0 && (stop - start).signum() != step.signum() {
            return Err("temperature sweep step direction must match start/stop range".to_string());
        }

        if (stop - start).abs() == 0.0 {
            return Ok(vec![start]);
        }

        let mut values = Vec::new();
        let mut current = start;
        let tolerance = (step.abs() * 1e-12).max((start.abs().max(stop.abs())) * 1e-12);

        if step > 0.0 {
            while current <= stop + tolerance {
                values.push(current);
                current += step;
            }
        } else {
            while current >= stop - tolerance {
                values.push(current);
                current += step;
            }
        }

        if values.is_empty() {
            return Err("temperature sweep produced no points".to_string());
        }

        Ok(values)
    }

    fn parse_positive_points(raw: &str, field_name: &str) -> Result<usize, String> {
        let points = raw
            .trim()
            .parse::<usize>()
            .map_err(|_| format!("{} must be a positive integer", field_name))?;
        if points == 0 {
            return Err(format!("{} must be greater than zero", field_name));
        }
        Ok(points)
    }

    fn parse_optional_spice_value(raw: &str) -> Result<Option<f64>, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("auto") {
            return Ok(None);
        }
        parse_spice_value_checked(trimmed).map(Some)
    }

    fn map_frequency_sweep(idx: usize) -> FrequencySweep {
        match idx {
            1 => FrequencySweep::Octave,
            2 => FrequencySweep::Linear,
            _ => FrequencySweep::Decade,
        }
    }

    fn map_ac_sweep(sweep: FrequencySweep) -> AcSweepType {
        match sweep {
            FrequencySweep::Decade => AcSweepType::Decade,
            FrequencySweep::Octave => AcSweepType::Octave,
            FrequencySweep::Linear => AcSweepType::Linear,
        }
    }

    /// Get human-readable name for analysis type
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
            AnalysisRunType::Transient => AnalysisType::Transient,
            AnalysisRunType::Noise => AnalysisType::Noise,
            AnalysisRunType::Tf => AnalysisType::Sensitivity,
            AnalysisRunType::Sensitivity => AnalysisType::Sensitivity,
            AnalysisRunType::PoleZero => AnalysisType::PoleZero,
            AnalysisRunType::HarmonicBalance => AnalysisType::HarmonicBalance,
            AnalysisRunType::Pss => AnalysisType::Pss,
            AnalysisRunType::Pac => AnalysisType::Ac,
            AnalysisRunType::Pnoise => AnalysisType::Noise,
            AnalysisRunType::MonteCarlo => AnalysisType::MonteCarlo,
            AnalysisRunType::Parametric => AnalysisType::Parametric,
            AnalysisRunType::Corner => AnalysisType::Corner,
        }
    }

    /// Convert SimulationResult to AnalysisResult for storage in Results Browser
    ///
    /// Extracts data from the engine's SimulationResult and creates an AnalysisResult
    /// with the appropriate type and data for display.
    fn convert_to_analysis_result(
        &self,
        sim_result: &crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();
        self.convert_to_analysis_result_with_metadata(sim_result, analysis_type, &label)
    }

    fn convert_to_analysis_result_with_metadata(
        &self,
        sim_result: &crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        use crate::simulation::SimulationResult;
        use crate::state::WaveformData;

        match sim_result {
            SimulationResult::DcOp(dc_result) => {
                // Convert engine DcOpResult to state DcOpResult
                let mut node_voltages = Vec::new();
                for (name, value) in &dc_result.node_voltages {
                    node_voltages.push(OperatingPointValue {
                        name: format!("V({})", name),
                        value: *value,
                        unit: "V".to_string(),
                    });
                }

                let mut branch_currents = Vec::new();
                for (name, value) in &dc_result.branch_currents {
                    branch_currents.push(OperatingPointValue {
                        name: format!("I({})", name),
                        value: *value,
                        unit: "A".to_string(),
                    });
                }

                let state_dc_op = DcOpResult {
                    node_voltages,
                    branch_currents,
                    power_dissipation: Vec::new(),
                };

                AnalysisResult::new(1, analysis_type, label.to_string()).with_dc_op(state_dc_op)
            }

            SimulationResult::Transient { time, waveforms } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            time.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                // For AC analysis, store magnitude (not raw complex values)
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            format!("|{}|", name),
                            frequencies.clone(),
                            wf.magnitude(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::DcSweep {
                sweep_var: _,
                sweep_values,
                waveforms,
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            sweep_values.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                ..
            } => {
                let wf_data = vec![WaveformData::new(
                    "onoise".to_string(),
                    frequencies.clone(),
                    output_noise.clone(),
                    Self::color_for_index(0),
                )];
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::PoleZero { .. } => {
                // Pole-Zero results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::Sensitivity { .. } => {
                // Sensitivity results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::MonteCarlo { variables, .. } => {
                let wf_data: Vec<WaveformData> = variables
                    .iter()
                    .filter_map(|var| {
                        if var.histogram.is_empty() || var.bin_edges.len() < 2 {
                            return None;
                        }
                        let x: Vec<f64> = var
                            .bin_edges
                            .windows(2)
                            .map(|window| (window[0] + window[1]) * 0.5)
                            .collect();
                        let y: Vec<f64> = var.histogram.iter().map(|count| *count as f64).collect();
                        Some(WaveformData::new(
                            format!("hist({})", var.name),
                            x,
                            y,
                            Self::color_for_index(0),
                        ))
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Parametric {
                sweep_values,
                waveforms,
                ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            sweep_values.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Corner {
                x_values,
                waveforms,
                ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            x_values.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Empty { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
            }
        }
    }

    /// Get color for waveform trace by index
    fn color_for_index(idx: usize) -> String {
        const COLORS: &[&str] = &[
            "#3B82F6", // Blue
            "#10B981", // Green
            "#F97316", // Orange
            "#8B5CF6", // Purple
            "#EC4899", // Pink
            "#EAB308", // Yellow
            "#14B8A6", // Teal
            "#EF4444", // Red
        ];
        COLORS[idx % COLORS.len()].to_string()
    }

    /// Poll for simulation completion
    ///
    /// Checks if the current analysis has completed. On success, adds result
    /// to the run and starts the next queued analysis. When all analyses are
    /// done, finalizes the simulation batch.
    fn poll_completion(&mut self, state: &mut AppState) {
        // Update status display with multi-analysis progress
        let status = self.runner.status();
        if !matches!(status, SimulationStatus::Idle)
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
                        .unwrap_or("Analysis");
                    let completion_msg = if self.total_analyses > 1 {
                        format!(
                            "{} completed ({}/{})",
                            current_label, self.current_analysis_idx, self.total_analyses
                        )
                    } else {
                        "Simulation completed successfully".to_string()
                    };
                    state
                        .console_messages
                        .push(ConsoleMessage::info(completion_msg));

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
                    let analysis_result = if let Some(config) = &self.current_config {
                        self.convert_to_analysis_result(&sim_result, config)
                    } else {
                        self.convert_to_analysis_result_with_metadata(
                            &sim_result,
                            analysis_type,
                            current_label,
                        )
                    };
                    if let Some(run) = state.simulation.active_run_mut() {
                        run.add_analysis(analysis_result);
                        log::info!(
                            "Added analysis to run {} (now has {} analyses)",
                            run.id,
                            run.analyses.len()
                        );
                    }

                    // Update waveform data (legacy compatibility)
                    self.update_waveforms(state, &sim_result);

                    // Set axis labels based on current analysis type
                    let (x_label, x_unit, y_label, y_unit) = analysis_type.axis_info();
                    state.waveform_viewer.x_axis_label = x_label.to_string();
                    state.waveform_viewer.x_axis_unit = x_unit.to_string();
                    state.waveform_viewer.y_axis_label = y_label.to_string();
                    state.waveform_viewer.y_axis_unit = y_unit.to_string();

                    // --- Phase 10-11-12 Integration Glue (run once per analysis) ---

                    // Run Yield Analysis (if MC results are present)
                    state.simulation.yield_results = self
                        .yield_manager
                        .analyze(std::slice::from_ref(&sim_result))
                        .values()
                        .cloned()
                        .collect();

                    // Run SOA Checking
                    self.soa_manager.clear_violations();
                    state.simulation.soa_violations = self.soa_manager.violations().to_vec();

                    // Run Reliability Analysis
                    let stress_data = std::collections::HashMap::new();
                    state.simulation.reliability_results = self
                        .reliability_engine
                        .analyze_circuit(&stress_data, &[1.0, 5.0, 10.0]);

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
                        // All analyses complete - finalize the batch
                        if self.total_analyses > 1 {
                            state.console_messages.push(ConsoleMessage::info(format!(
                                "All {} analyses completed successfully",
                                self.total_analyses
                            )));
                        }
                        self.finish_simulation_batch(state);
                    }
                }
                Err(e) => {
                    state
                        .console_messages
                        .push(ConsoleMessage::error(format!("Analysis failed: {}", e)));

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
                    let failed_analysis =
                        AnalysisResult::failed(1, failed_type, failed_label, e.to_string());
                    if let Some(run) = state.simulation.active_run_mut() {
                        run.add_analysis(failed_analysis);
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
                        state.simulation.status = format!("Completed with errors");
                        self.finish_simulation_batch(state);
                    }
                }
            }
        }
    }

    /// Update waveform data from simulation results
    ///
    /// Converts each SimulationResult variant to WaveformData traces and
    /// populates the simulation state for display in the waveform viewer.
    ///
    /// Commercial simulator behavior:
    /// - Transient: X=time, Y=voltage/current
    /// - AC: X=frequency, Y=magnitude (with separate phase traces)
    /// - DC Sweep: X=sweep variable, Y=voltage
    /// - Noise: X=frequency, Y=noise spectral density (V²/Hz or A²/Hz)
    fn update_waveforms(&self, state: &mut AppState, result: &crate::simulation::SimulationResult) {
        use crate::simulation::SimulationResult;
        use crate::state::WaveformData;

        // Color palette for commercial-grade visualization
        const COLORS: &[&str] = &[
            "#3B82F6", // Blue
            "#10B981", // Green
            "#F97316", // Orange
            "#8B5CF6", // Purple
            "#EC4899", // Pink
            "#EAB308", // Yellow
            "#14B8A6", // Teal
            "#EF4444", // Red
        ];

        // Clear previous waveforms
        state.simulation.waveforms.clear();

        match result {
            SimulationResult::DcOp(dc_result) => {
                // DC OP: Display voltages in console
                // Note: DC annotation overlay requires position mapping from schematic
                // which is handled separately when the schematic state is available
                log::info!(
                    "DC OP result has {} node voltages",
                    dc_result.node_voltages.len()
                );
                for (node, voltage) in &dc_result.node_voltages {
                    log::info!("  V({}) = {:.6} V", node, voltage);
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(format!(
                            "V({}) = {:.6} V",
                            node, voltage
                        )));
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "DC OP: {} node voltages computed",
                        dc_result.node_voltages.len()
                    )));

                // Auto-show console panel so user sees results
                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Console;
            }

            SimulationResult::Transient { time, waveforms } => {
                // Transient: Create waveform traces with time as X-axis
                let time_vec: Vec<f64> = time.clone();

                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let color = COLORS[idx % COLORS.len()].to_string();
                    let waveform = WaveformData::new(
                        name.clone(),
                        time_vec.clone(),
                        wf_data.y_values.clone(),
                        color,
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Transient: {} points, {} waveforms",
                        time.len(),
                        waveforms.len()
                    )));

                // Auto-show waveform panel for better visibility
                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                // AC: Create magnitude traces (log-log or semi-log typically)
                // Commercial simulators show |V(node)| in dB and phase separately
                let freq_vec: Vec<f64> = frequencies.clone();

                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    // Magnitude trace - use magnitude() for complex data, not raw real values
                    let mag_name = format!("|{}|", name);
                    let color = COLORS[idx % COLORS.len()].to_string();

                    // For AC analysis, use the magnitude of complex waveform data
                    let magnitude = wf_data.magnitude();

                    let waveform = WaveformData::new(mag_name, freq_vec.clone(), magnitude, color);
                    state.simulation.waveforms.push(waveform);
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "AC: {} points, {} waveforms",
                        frequencies.len(),
                        waveforms.len()
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
            } => {
                // DC Sweep: sweep variable as X-axis
                let x_vec: Vec<f64> = sweep_values.clone();

                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let color = COLORS[idx % COLORS.len()].to_string();
                    let waveform = WaveformData::new(
                        name.clone(),
                        x_vec.clone(),
                        wf_data.y_values.clone(),
                        color,
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "DC Sweep ({}): {} points, {} waveforms",
                        sweep_var,
                        sweep_values.len(),
                        waveforms.len()
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
            } => {
                // Noise: frequency as X-axis, noise spectral density as Y
                let freq_vec: Vec<f64> = frequencies.clone();

                // Output noise trace
                if !output_noise.is_empty() {
                    let waveform = WaveformData::new(
                        "onoise".to_string(),
                        freq_vec.clone(),
                        output_noise.clone(),
                        COLORS[0].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                // Input-referred noise trace (if present)
                if let Some(inoise) = input_noise {
                    if !inoise.is_empty() {
                        let waveform = WaveformData::new(
                            "inoise".to_string(),
                            freq_vec.clone(),
                            inoise.clone(),
                            COLORS[1].to_string(),
                        );
                        state.simulation.waveforms.push(waveform);
                    }
                }

                // Per-source contributions
                for (idx, (source, values)) in contributors.iter().enumerate() {
                    let color = COLORS[(idx + 2) % COLORS.len()].to_string();
                    let waveform = WaveformData::new(
                        format!("noise({})", source),
                        freq_vec.clone(),
                        values.clone(),
                        color,
                    );
                    state.simulation.waveforms.push(waveform);
                }

                // Calculate integrated noise
                let integrated: f64 = output_noise.iter().sum::<f64>().sqrt();
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Noise: {} points, integrated output: {:.3e} V/√Hz",
                        frequencies.len(),
                        integrated
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::PoleZero { poles, zeros, gain } => {
                // Pole-Zero: Display in console (and optionally s-plane plot)
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Pole-Zero Analysis: DC gain = {:.4}",
                        gain
                    )));

                for (i, (re, im)) in poles.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        // Real pole
                        let freq = re.abs() / (2.0 * std::f64::consts::PI);
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "  Pole {}: {:.3e} rad/s ({:.3e} Hz)",
                                i + 1,
                                re,
                                freq
                            )));
                    } else {
                        // Complex pole
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "  Pole {}: {:.3e} ± j{:.3e} rad/s",
                                i + 1,
                                re,
                                im.abs()
                            )));
                    }
                }

                for (i, (re, im)) in zeros.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "  Zero {}: {:.3e} rad/s",
                                i + 1,
                                re
                            )));
                    } else {
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "  Zero {}: {:.3e} ± j{:.3e} rad/s",
                                i + 1,
                                re,
                                im.abs()
                            )));
                    }
                }
            }

            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                // Sensitivity: Display in console as table
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Sensitivity Analysis: {} parameters",
                        sensitivities.len()
                    )));

                // Sort by normalized sensitivity magnitude
                let mut sorted: Vec<_> = normalized.iter().collect();
                sorted.sort_by(|a, b| {
                    b.1.abs()
                        .partial_cmp(&a.1.abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                for (param, norm_sens) in sorted.iter().take(10) {
                    if let Some(sens) = sensitivities.get(*param) {
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "  {}: dV/d{} = {:.3e}, norm = {:.2}%",
                                param,
                                param,
                                sens,
                                **norm_sens * 100.0
                            )));
                    }
                }
            }

            SimulationResult::MonteCarlo {
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
            } => {
                for (idx, var) in variables.iter().enumerate() {
                    if var.histogram.is_empty() || var.bin_edges.len() < 2 {
                        continue;
                    }
                    let x: Vec<f64> = var
                        .bin_edges
                        .windows(2)
                        .map(|window| (window[0] + window[1]) * 0.5)
                        .collect();
                    let y: Vec<f64> = var.histogram.iter().map(|count| *count as f64).collect();
                    let waveform = WaveformData::new(
                        format!("hist({})", var.name),
                        x,
                        y,
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Monte Carlo: {}/{} runs converged ({} failed), all_converged={}",
                        runs_completed, runs_requested, num_failures, all_converged
                    )));

                for var in variables.iter().take(8) {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(format!(
                            "  {}: mean={:.6e}, sigma={:.6e}, min={:.6e}, max={:.6e}",
                            var.name, var.mean, var.std_dev, var.min, var.max
                        )));
                }

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = if state.simulation.waveforms.is_empty() {
                    crate::common::app::BottomPanelTab::Console
                } else {
                    crate::common::app::BottomPanelTab::Waveform
                };
            }

            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        sweep_values.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Parametric ({}): {} points, {} waveforms, {} failed points",
                        target,
                        sweep_values.len(),
                        waveforms.len(),
                        num_failures
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Corner {
                x_values,
                waveforms,
                num_failures,
                ..
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        x_values.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Corner sweep: {} points, {} waveforms, {} failed corners",
                        x_values.len(),
                        waveforms.len(),
                        num_failures
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Empty { .. } => {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(
                        "Analysis complete (no waveform data)".to_string(),
                    ));
            }
        }

        // Build node-to-waveform mapping for cross-probing
        state.simulation.node_to_waveform.clear();
        for (idx, wf) in state.simulation.waveforms.iter().enumerate() {
            state
                .simulation
                .node_to_waveform
                .insert(wf.name.clone(), idx);
        }
    }

    /// Check if a simulation is currently running
    pub fn is_running(&self) -> bool {
        self.runner.is_running()
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

/// Parse SPICE value string (e.g., "1k", "10u", "100n") to f64
fn parse_spice_value(s: &str) -> f64 {
    let s = s.trim();
    if s.is_empty() {
        return 0.0;
    }

    // Try direct parse first
    if let Ok(v) = s.parse::<f64>() {
        return v;
    }

    // Find where the number ends
    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    let (num_str, suffix) = s.split_at(num_end);
    let base: f64 = num_str.parse().unwrap_or(0.0);

    let multiplier = match suffix.to_lowercase().as_str() {
        "t" | "tera" => 1e12,
        "g" | "gig" => 1e9,
        "meg" | "m" if suffix.len() >= 3 => 1e6, // MEG is megHH, not milli
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "µ" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        "" => 1.0,
        _ => 1.0,
    };

    base * multiplier
}

fn parse_spice_value_checked(s: &str) -> Result<f64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("value is empty".to_string());
    }

    if let Ok(v) = s.parse::<f64>() {
        if v.is_finite() {
            return Ok(v);
        }
        return Err("value is not finite".to_string());
    }

    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    if num_end == 0 || num_end == s.len() && s.parse::<f64>().is_err() {
        return Err(format!("invalid numeric value '{}'", s));
    }

    let (num_str, suffix) = s.split_at(num_end);
    let base: f64 = num_str
        .parse()
        .map_err(|_| format!("invalid numeric value '{}'", s))?;

    let multiplier = match suffix.to_ascii_lowercase().as_str() {
        "t" | "tera" => 1e12,
        "g" | "gig" => 1e9,
        "meg" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        "" => 1.0,
        _ => return Err(format!("unsupported SPICE suffix '{}'", suffix)),
    };

    let value = base * multiplier;
    if value.is_finite() {
        Ok(value)
    } else {
        Err("value is not finite".to_string())
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Parse SPICE Value Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_parse_spice_value_plain_number() {
        assert!((parse_spice_value("100") - 100.0).abs() < 1e-10);
        assert!((parse_spice_value("1.5") - 1.5).abs() < 1e-10);
        assert!((parse_spice_value("-10") - (-10.0)).abs() < 1e-10);
    }

    #[test]
    fn test_parse_spice_value_scientific() {
        assert!((parse_spice_value("1e-9") - 1e-9).abs() < 1e-20);
        assert!((parse_spice_value("2.5E6") - 2.5e6).abs() < 1.0);
    }

    #[test]
    fn test_parse_spice_value_kilo() {
        assert!((parse_spice_value("1k") - 1000.0).abs() < 1e-10);
        assert!((parse_spice_value("10K") - 10000.0).abs() < 1e-10);
        assert!((parse_spice_value("4.7k") - 4700.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_spice_value_mega() {
        assert!((parse_spice_value("1Meg") - 1e6).abs() < 1.0);
        assert!((parse_spice_value("2.2meg") - 2.2e6).abs() < 1.0);
    }

    #[test]
    fn test_parse_spice_value_milli() {
        assert!((parse_spice_value("1m") - 1e-3).abs() < 1e-15);
        assert!((parse_spice_value("100m") - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_parse_spice_value_micro() {
        assert!((parse_spice_value("1u") - 1e-6).abs() < 1e-18);
        assert!((parse_spice_value("10u") - 10e-6).abs() < 1e-17);
    }

    #[test]
    fn test_parse_spice_value_nano() {
        assert!((parse_spice_value("1n") - 1e-9).abs() < 1e-21);
        assert!((parse_spice_value("100n") - 100e-9).abs() < 1e-18);
    }

    #[test]
    fn test_parse_spice_value_pico() {
        assert!((parse_spice_value("1p") - 1e-12).abs() < 1e-24);
        assert!((parse_spice_value("10p") - 10e-12).abs() < 1e-23);
    }

    #[test]
    fn test_parse_spice_value_femto() {
        assert!((parse_spice_value("1f") - 1e-15).abs() < 1e-27);
    }

    #[test]
    fn test_parse_spice_value_giga() {
        assert!((parse_spice_value("1G") - 1e9).abs() < 1.0);
        assert!((parse_spice_value("2.4G") - 2.4e9).abs() < 1.0);
    }

    #[test]
    fn test_parse_spice_value_empty() {
        assert_eq!(parse_spice_value(""), 0.0);
        assert_eq!(parse_spice_value("   "), 0.0);
    }

    #[test]
    fn test_parse_spice_value_checked_valid_suffix() {
        let value = parse_spice_value_checked("4.7k").expect("4.7k should parse");
        assert!((value - 4700.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_spice_value_checked_rejects_unknown_suffix() {
        let err = parse_spice_value_checked("10xyz").expect_err("unknown suffix must fail");
        assert!(err.contains("unsupported SPICE suffix"));
    }

    // -------------------------------------------------------------------------
    // Controller Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_controller_new() {
        let controller = SimulationController::new();
        assert!(!controller.is_running());
    }

    #[test]
    fn test_controller_default() {
        let controller = SimulationController::default();
        assert!(!controller.is_running());
    }

    #[test]
    fn test_controller_status_initial() {
        let controller = SimulationController::new();
        assert!(matches!(controller.status(), SimulationStatus::Idle));
    }

    #[test]
    fn test_analysis_name() {
        let controller = SimulationController::new();

        assert_eq!(
            controller.analysis_name(&AnalysisConfig::DcOp),
            "DC Operating Point"
        );

        let tran = AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: Some(1e-9),
            uic: false,
        });
        assert_eq!(controller.analysis_name(&tran), "Transient");
    }

    #[test]
    fn test_build_transient_config_uses_output_step_without_forcing_internal_max_step() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.tran_stop = "5m".to_string();
        state.dialogs.tran_step = "10n".to_string();
        state.dialogs.tran_start = "0".to_string();

        let spec = controller
            .build_analysis_spec_for_index(&state, 1)
            .expect("transient spec should build");
        let config = controller
            .analysis_spec_to_config(&state, &spec)
            .expect("transient config should build");
        let tran = match config {
            AnalysisConfig::Transient(tran) => tran,
            _ => panic!("Expected transient config"),
        };

        assert!((tran.stop_time - 5e-3).abs() < 1e-15);
        assert!((tran.step_time - 10e-9).abs() < 1e-18);
        assert_eq!(tran.max_timestep, None);
    }

    #[test]
    fn test_enabled_analysis_indices_defaults_to_dcop() {
        let state = AppState::default();
        let indices = SimulationController::enabled_analysis_indices(&state);
        assert_eq!(indices, vec![0]);
    }

    #[test]
    fn test_build_analysis_plan_rejects_unsupported_analysis_tab() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(11); // Harmonic Balance

        let errors = controller
            .build_analysis_plan(&state)
            .expect_err("unsupported analysis should fail planning");
        assert!(errors.iter().any(|e| e.contains("Harmonic Balance")));
    }

    #[test]
    fn test_build_analysis_plan_includes_supported_analyses_in_order() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.extend([1, 2, 4]);
        state.dialogs.tran_stop = "5m".to_string();
        state.dialogs.tran_step = "10n".to_string();
        state.dialogs.ac_fstart = "1".to_string();
        state.dialogs.ac_fstop = "1Meg".to_string();
        state.dialogs.ac_points = "20".to_string();
        state.dialogs.noise_output = "out".to_string();
        state.dialogs.noise_fstart = "10".to_string();
        state.dialogs.noise_fstop = "100Meg".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        assert_eq!(plan.analyses.len(), 3);
        assert!(matches!(plan.analyses[0], AnalysisSpec::Transient { .. }));
        assert!(matches!(plan.analyses[1], AnalysisSpec::Ac { .. }));
        assert!(matches!(plan.analyses[2], AnalysisSpec::Noise { .. }));
    }

    #[test]
    fn test_build_analysis_plan_rejects_nested_dc_sweep() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(3);
        state.dialogs.dc_nested = true;
        state.dialogs.dc_source = "V1".to_string();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1".to_string();
        state.dialogs.dc_step = "0.1".to_string();

        let errors = controller
            .build_analysis_plan(&state)
            .expect_err("nested sweep should fail until implemented");
        assert!(errors.iter().any(|e| e.contains("nested DC sweep")));
    }

    #[test]
    fn test_build_queue_from_plan_maps_pole_zero_config() {
        use crate::simulation::dialog::pz::{PzAnalysisType, PzConfig, PzTransferType};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(5);
        state.dialogs.pz_state = crate::simulation::dialog::pz::PzDialogState::from_config(
            &PzConfig::new("vin", "vout")
                .with_transfer(PzTransferType::Current)
                .with_type(PzAnalysisType::PolesOnly),
        );

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");
        assert_eq!(queue.len(), 1);

        match &queue[0].config {
            Some(AnalysisConfig::PoleZero(pz)) => {
                assert_eq!(pz.input_node, "VIN");
                assert_eq!(pz.output_node, "VOUT");
                assert_eq!(pz.transfer_type, "CUR");
                assert!(matches!(
                    pz.analysis_type,
                    crate::simulation::config::PzAnalysisType::PolesOnly
                ));
            }
            _ => panic!("Expected pole-zero config"),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_pole_zero_uses_dialog_configuration() {
        use crate::simulation::dialog::pz::{PzAnalysisType, PzConfig, PzTransferType};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.pz_state = crate::simulation::dialog::pz::PzDialogState::from_config(
            &PzConfig::new("in", "out")
                .with_transfer(PzTransferType::Current)
                .with_type(PzAnalysisType::ZerosOnly),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 5)
            .expect("pole-zero spec should build");
        match spec {
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => {
                assert_eq!(input_node, "IN");
                assert_eq!(input_ref, "0");
                assert_eq!(output_node, "OUT");
                assert_eq!(output_ref, "0");
                assert_eq!(transfer_type, "CUR");
                assert_eq!(analysis_type, "ZER");
            }
            _ => panic!("Expected pole-zero spec"),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_sensitivity_uses_dialog_configuration() {
        use crate::simulation::dialog::sens::{SensConfig, SensType};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.sens_state = crate::simulation::dialog::sens::SensDialogState::from_config(
            &SensConfig::new("V(out)")
                .with_type(SensType::Ac)
                .with_ac_freq(5e6),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 6)
            .expect("sensitivity spec should build");
        match spec {
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => {
                assert_eq!(output_var, "V(out)");
                assert!(ac_mode);
                assert_eq!(frequency, Some(5e6));
            }
            _ => panic!("Expected sensitivity spec"),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_monte_carlo_uses_dialog_validation() {
        use crate::simulation::dialog::mc::{McBaseAnalysis, McConfig, McDistribution};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.mc_state = crate::simulation::dialog::mc::McDialogState::from_config(
            &McConfig::new(64)
                .with_distribution(McDistribution::Gaussian)
                .with_base(McBaseAnalysis::Dc)
                .with_seed(1234),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 7)
            .expect("Monte Carlo spec should build");
        assert!(matches!(spec, AnalysisSpec::MonteCarlo));
    }

    #[test]
    fn test_build_analysis_spec_for_temperature_sweep_accepts_transient_base() {
        use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
            &TempConfig::new(-40.0, 125.0, 25.0).with_base(TempBaseAnalysis::Transient),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 10)
            .expect("Transient base should be accepted for temperature sweeps");
        assert!(matches!(spec, AnalysisSpec::Parametric));
    }

    #[test]
    fn test_build_analysis_spec_for_corner_accepts_process_and_voltage_sweeps() {
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.corner_state =
            crate::simulation::dialog::corner::CornerDialogState::from_config(
                &CornerConfig::default()
                    .with_process_corners(vec![ProcessCorner::TT, ProcessCorner::FF])
                    .with_voltages(vec![0.9, 1.0, 1.1])
                    .with_temperatures(vec![-40.0, 25.0, 125.0])
                    .with_base_analysis(CornerBaseAnalysis::Op),
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 18)
            .expect("corner spec should build for full PVT sweep");
        assert!(matches!(spec, AnalysisSpec::Corner));
    }

    #[test]
    fn test_build_queue_from_plan_stores_spec_executed_analyses_without_config() {
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};
        use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [7usize, 10usize, 18usize].into_iter().collect();
        state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
            &TempConfig::new(-40.0, 85.0, 25.0).with_base(TempBaseAnalysis::Op),
        );
        state.dialogs.corner_state =
            crate::simulation::dialog::corner::CornerDialogState::from_config(
                &CornerConfig::default()
                    .with_process_corners(vec![ProcessCorner::TT])
                    .with_base_analysis(CornerBaseAnalysis::Op),
            );

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 3);
        assert!(matches!(queue[0].spec, AnalysisSpec::MonteCarlo));
        assert!(queue[0].config.is_none());
        assert!(queue[0].spec_options.corner.is_none());
        assert!(queue[0].analysis_line.starts_with(".mc "));

        assert!(matches!(queue[1].spec, AnalysisSpec::Parametric));
        assert!(queue[1].config.is_none());
        assert!(queue[1].spec_options.corner.is_none());
        assert!(queue[1].spec_options.temp.is_some());
        assert!(matches!(
            queue[1]
                .spec_options
                .temp
                .as_ref()
                .expect("temperature options must be present")
                .base_mode,
            crate::services::simulation_runner::CornerBaseMode::Op
        ));
        assert!(queue[1].analysis_line.starts_with(".step temp "));

        assert!(matches!(queue[2].spec, AnalysisSpec::Corner));
        assert!(queue[2].config.is_none());
        assert!(queue[2].spec_options.corner.is_some());
        assert!(matches!(
            queue[2]
                .spec_options
                .corner
                .as_ref()
                .expect("corner options must be present")
                .base_mode,
            crate::services::simulation_runner::CornerBaseMode::Op
        ));
        assert!(queue[2].analysis_line.starts_with(".temp "));
    }

    #[test]
    fn test_build_analysis_spec_for_corner_accepts_transient_base_mode() {
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.corner_state =
            crate::simulation::dialog::corner::CornerDialogState::from_config(
                &CornerConfig::default().with_base_analysis(CornerBaseAnalysis::Transient),
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 18)
            .expect("corner transient base mode should be accepted");
        assert!(matches!(spec, AnalysisSpec::Corner));
    }

    #[test]
    fn test_build_queue_from_plan_maps_temperature_ac_base_mode() {
        use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [10usize].into_iter().collect();
        state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
            &TempConfig::new(-40.0, 125.0, 82.5).with_base(TempBaseAnalysis::Ac),
        );
        state.dialogs.ac_fstart = "1k".to_string();
        state.dialogs.ac_fstop = "10Meg".to_string();
        state.dialogs.ac_points = "12".to_string();
        state.dialogs.ac_sweep_type = 1; // octave

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        let temp = queue[0]
            .spec_options
            .temp
            .as_ref()
            .expect("temperature options must be present");
        match &temp.base_mode {
            crate::services::simulation_runner::CornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => {
                assert!((*start_freq - 1e3).abs() < 1e-12);
                assert!((*stop_freq - 1e7).abs() < 1e-4);
                assert_eq!(*points_per_unit, 12);
                assert!(matches!(
                    sweep,
                    crate::services::simulation_runner::CornerFrequencySweep::Octave
                ));
            }
            other => panic!("expected AC temp base mode, got {:?}", other),
        }
    }

    #[test]
    fn test_build_queue_from_plan_rejects_temperature_dc_without_source() {
        use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [10usize].into_iter().collect();
        state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
            &TempConfig::new(-40.0, 125.0, 25.0).with_base(TempBaseAnalysis::Dc),
        );
        state.dialogs.dc_source.clear();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1".to_string();
        state.dialogs.dc_step = "0.1".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let err = controller
            .build_queue_from_plan(&state, &plan)
            .expect_err("temperature DC base mode should require source");
        assert!(err.iter().any(|msg| msg.contains("non-empty sweep source")));
    }

    #[test]
    fn test_build_queue_from_plan_maps_corner_ac_base_mode() {
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [18usize].into_iter().collect();
        state.dialogs.corner_state =
            crate::simulation::dialog::corner::CornerDialogState::from_config(
                &CornerConfig::default()
                    .with_process_corners(vec![ProcessCorner::TT])
                    .with_base_analysis(CornerBaseAnalysis::Ac),
            );
        state.dialogs.ac_fstart = "1k".to_string();
        state.dialogs.ac_fstop = "10Meg".to_string();
        state.dialogs.ac_points = "12".to_string();
        state.dialogs.ac_sweep_type = 1; // octave

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        let corner = queue[0]
            .spec_options
            .corner
            .as_ref()
            .expect("corner options must be present");
        match &corner.base_mode {
            crate::services::simulation_runner::CornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => {
                assert!((*start_freq - 1e3).abs() < 1e-12);
                assert!((*stop_freq - 1e7).abs() < 1e-4);
                assert_eq!(*points_per_unit, 12);
                assert!(matches!(
                    sweep,
                    crate::services::simulation_runner::CornerFrequencySweep::Octave
                ));
            }
            other => panic!("expected AC corner base mode, got {:?}", other),
        }
    }

    #[test]
    fn test_build_queue_from_plan_maps_corner_dc_base_mode() {
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [18usize].into_iter().collect();
        state.dialogs.corner_state =
            crate::simulation::dialog::corner::CornerDialogState::from_config(
                &CornerConfig::default()
                    .with_process_corners(vec![ProcessCorner::TT])
                    .with_base_analysis(CornerBaseAnalysis::Dc),
            );
        state.dialogs.dc_source = "VDD".to_string();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1.2".to_string();
        state.dialogs.dc_step = "0.1".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        let corner = queue[0]
            .spec_options
            .corner
            .as_ref()
            .expect("corner options must be present");
        match &corner.base_mode {
            crate::services::simulation_runner::CornerBaseMode::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => {
                assert_eq!(source_name, "VDD");
                assert_eq!(*start, 0.0);
                assert_eq!(*stop, 1.2);
                assert_eq!(*step, 0.1);
            }
            other => panic!("expected DC corner base mode, got {:?}", other),
        }
    }

    #[test]
    fn test_build_queue_from_plan_rejects_corner_dc_without_source() {
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [18usize].into_iter().collect();
        state.dialogs.corner_state =
            crate::simulation::dialog::corner::CornerDialogState::from_config(
                &CornerConfig::default().with_base_analysis(CornerBaseAnalysis::Dc),
            );
        state.dialogs.dc_source.clear();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1".to_string();
        state.dialogs.dc_step = "0.1".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let err = controller
            .build_queue_from_plan(&state, &plan)
            .expect_err("corner DC base mode should require source");
        assert!(err.iter().any(|msg| msg.contains("non-empty sweep source")));
    }

    #[test]
    fn test_build_queue_from_plan_maps_transient_optional_maxstep_and_uic() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(1);
        state.dialogs.tran_stop = "10u".to_string();
        state.dialogs.tran_step = "1n".to_string();
        state.dialogs.tran_start = "500n".to_string();
        state.dialogs.tran_maxstep = "2n".to_string();
        state.dialogs.tran_uic = true;

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        match &queue[0].config {
            Some(AnalysisConfig::Transient(tran)) => {
                assert!((tran.start_time - 500e-9).abs() < 1e-18);
                assert_eq!(tran.max_timestep, Some(2e-9));
                assert!(tran.uic);
            }
            _ => panic!("Expected transient config"),
        }
    }

    // -------------------------------------------------------------------------
    // Config to AnalysisType Mapping Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_config_to_analysis_type_dc_op() {
        let controller = SimulationController::new();
        assert_eq!(
            controller.config_to_analysis_type(&AnalysisConfig::DcOp),
            crate::state::AnalysisType::DcOp
        );
    }

    #[test]
    fn test_config_to_analysis_type_dc_sweep() {
        let controller = SimulationController::new();
        let config = AnalysisConfig::DcSweep(DcSweepConfig {
            source: "V1".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 0.1,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        });
        assert_eq!(
            controller.config_to_analysis_type(&config),
            crate::state::AnalysisType::DcSweep
        );
    }

    #[test]
    fn test_config_to_analysis_type_transient() {
        let controller = SimulationController::new();
        let config = AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        });
        assert_eq!(
            controller.config_to_analysis_type(&config),
            crate::state::AnalysisType::Transient
        );
    }

    #[test]
    fn test_config_to_analysis_type_ac() {
        let controller = SimulationController::new();
        let config = AnalysisConfig::Ac(AcAnalysisConfig {
            start_freq: 1.0,
            stop_freq: 1e9,
            num_points: 101,
            sweep_type: AcSweepType::Decade,
        });
        assert_eq!(
            controller.config_to_analysis_type(&config),
            crate::state::AnalysisType::Ac
        );
    }

    #[test]
    fn test_config_to_analysis_type_all_variants() {
        use crate::simulation::config::{
            AcSweepType, NoiseAnalysisConfig, PoleZeroConfig, PzAnalysisType, SensitivityConfig,
        };
        let controller = SimulationController::new();

        // Noise - uses reference_node: String (not Option), sweep_type, num_points
        let noise_config = AnalysisConfig::Noise(NoiseAnalysisConfig {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "V1".to_string(),
            sweep_type: AcSweepType::Decade,
            num_points: 10,
            start_freq: 1.0,
            stop_freq: 1e6,
        });
        assert_eq!(
            controller.config_to_analysis_type(&noise_config),
            crate::state::AnalysisType::Noise
        );

        // PoleZero - uses input_node, input_ref, output_node, output_ref, transfer_type, analysis_type
        let pz_config = AnalysisConfig::PoleZero(PoleZeroConfig {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: PzAnalysisType::PoleZero,
        });
        assert_eq!(
            controller.config_to_analysis_type(&pz_config),
            crate::state::AnalysisType::PoleZero
        );

        // Sensitivity - uses output_var, ac_mode, frequency
        let sens_config = AnalysisConfig::Sensitivity(SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        });
        assert_eq!(
            controller.config_to_analysis_type(&sens_config),
            crate::state::AnalysisType::Sensitivity
        );
    }

    // -------------------------------------------------------------------------
    // Convert to Analysis Result Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_convert_dc_op_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::DcOpResult as EngineDcOpResult;

        let controller = SimulationController::new();
        let config = AnalysisConfig::DcOp;

        // Create engine DC OP result with sample data
        let mut engine_result = EngineDcOpResult::default();
        engine_result.node_voltages.insert("N001".to_string(), 5.0);
        engine_result.node_voltages.insert("N002".to_string(), 2.5);
        engine_result
            .branch_currents
            .insert("V1".to_string(), 0.001);

        let sim_result = SimulationResult::DcOp(engine_result);
        let analysis = controller.convert_to_analysis_result(&sim_result, &config);

        assert_eq!(analysis.analysis_type, crate::state::AnalysisType::DcOp);
        assert!(analysis.success);
        assert!(analysis.dc_op.is_some());

        let dc_op = analysis.dc_op.unwrap();
        assert_eq!(dc_op.node_voltages.len(), 2);
        assert_eq!(dc_op.branch_currents.len(), 1);

        // Verify node voltage conversion
        let v_n001 = dc_op.node_voltages.iter().find(|v| v.name == "V(N001)");
        assert!(v_n001.is_some());
        assert!((v_n001.unwrap().value - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_transient_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use std::collections::HashMap;

        let controller = SimulationController::new();
        let config = AnalysisConfig::Transient(TransientAnalysisConfig {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        });

        // Create engine transient result using proper constructor
        let time = vec![0.0, 1e-9, 2e-9, 3e-9];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            EngineWaveformData::new_time_domain("V(out)", time.clone(), vec![0.0, 1.0, 2.0, 3.0]),
        );

        let sim_result = SimulationResult::Transient { time, waveforms };
        let analysis = controller.convert_to_analysis_result(&sim_result, &config);

        assert_eq!(
            analysis.analysis_type,
            crate::state::AnalysisType::Transient
        );
        assert!(analysis.success);
        assert!(analysis.dc_op.is_none());
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].name, "V(out)");
        assert_eq!(analysis.waveforms[0].x.len(), 4);
        assert_eq!(analysis.waveforms[0].y.len(), 4);
    }

    #[test]
    fn test_convert_ac_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use std::collections::HashMap;

        let controller = SimulationController::new();
        let config = AnalysisConfig::Ac(AcAnalysisConfig {
            start_freq: 1.0,
            stop_freq: 1e6,
            num_points: 5,
            sweep_type: AcSweepType::Decade,
        });

        let frequencies = vec![1.0, 10.0, 100.0, 1000.0, 10000.0];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            EngineWaveformData::new_freq_domain(
                "V(out)",
                frequencies.clone(),
                vec![1.0, 0.9, 0.7, 0.5, 0.3],
            ),
        );

        let sim_result = SimulationResult::Ac {
            frequencies,
            waveforms,
        };
        let analysis = controller.convert_to_analysis_result(&sim_result, &config);

        assert_eq!(analysis.analysis_type, crate::state::AnalysisType::Ac);
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].name, "|V(out)|"); // Magnitude notation
    }

    #[test]
    fn test_convert_dc_sweep_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use std::collections::HashMap;

        let controller = SimulationController::new();
        let config = AnalysisConfig::DcSweep(DcSweepConfig {
            source: "V1".to_string(),
            start: 0.0,
            stop: 5.0,
            step: 1.0,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        });

        let sweep_values = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            EngineWaveformData::new_time_domain(
                "V(out)",
                sweep_values.clone(),
                vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5],
            ),
        );

        let sim_result = SimulationResult::DcSweep {
            sweep_var: "V1".to_string(),
            sweep_values,
            waveforms,
        };
        let analysis = controller.convert_to_analysis_result(&sim_result, &config);

        assert_eq!(analysis.analysis_type, crate::state::AnalysisType::DcSweep);
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].x.len(), 6);
    }

    #[test]
    fn test_convert_pole_zero_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::config::{PoleZeroConfig, PzAnalysisType};

        let controller = SimulationController::new();
        let config = AnalysisConfig::PoleZero(PoleZeroConfig {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: PzAnalysisType::PoleZero,
        });

        let sim_result = SimulationResult::PoleZero {
            poles: vec![(-1000.0, 0.0), (-500.0, 1000.0)],
            zeros: vec![(-100.0, 0.0)],
            gain: 10.0,
        };
        let analysis = controller.convert_to_analysis_result(&sim_result, &config);

        assert_eq!(analysis.analysis_type, crate::state::AnalysisType::PoleZero);
        assert!(analysis.waveforms.is_empty()); // PZ results are console-only
    }

    #[test]
    fn test_convert_monte_carlo_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::MonteCarloVariableResult;

        let controller = SimulationController::new();
        let sim_result = SimulationResult::MonteCarlo {
            runs_requested: 16,
            runs_completed: 15,
            num_failures: 1,
            all_converged: false,
            variables: vec![MonteCarloVariableResult {
                name: "V(out)".to_string(),
                mean: 0.99,
                std_dev: 0.02,
                min: 0.9,
                max: 1.05,
                histogram: vec![2, 5, 6, 2],
                bin_edges: vec![0.9, 0.95, 1.0, 1.05, 1.1],
            }],
        };

        let analysis = controller.convert_to_analysis_result_with_metadata(
            &sim_result,
            crate::state::AnalysisType::MonteCarlo,
            "Monte Carlo",
        );
        assert_eq!(
            analysis.analysis_type,
            crate::state::AnalysisType::MonteCarlo
        );
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].name, "hist(V(out))");
    }

    #[test]
    fn test_convert_parametric_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use std::collections::HashMap;

        let controller = SimulationController::new();
        let sweep_values = vec![-40.0, 25.0, 85.0];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            EngineWaveformData::new_time_domain(
                "V(out)",
                sweep_values.clone(),
                vec![1.1, 1.0, 0.9],
            ),
        );
        let sim_result = SimulationResult::Parametric {
            target: "TEMP".to_string(),
            sweep_values,
            waveforms,
            num_failures: 0,
        };

        let analysis = controller.convert_to_analysis_result_with_metadata(
            &sim_result,
            crate::state::AnalysisType::Parametric,
            "Parametric",
        );
        assert_eq!(
            analysis.analysis_type,
            crate::state::AnalysisType::Parametric
        );
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].x.len(), 3);
    }

    #[test]
    fn test_convert_corner_result() {
        use crate::simulation::SimulationResult;
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use std::collections::HashMap;

        let controller = SimulationController::new();
        let temperatures = vec![-40.0, 25.0, 125.0];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            EngineWaveformData::new_time_domain(
                "V(out)",
                temperatures.clone(),
                vec![1.2, 1.0, 0.8],
            ),
        );
        let sim_result = SimulationResult::Corner {
            x_values: temperatures.clone(),
            x_label: "Temperature".to_string(),
            x_unit: "C".to_string(),
            temperatures_c: temperatures,
            corner_labels: vec![
                "TT_1.000000V_-40.000000C".to_string(),
                "TT_1.000000V_25.000000C".to_string(),
                "TT_1.000000V_125.000000C".to_string(),
            ],
            waveforms,
            num_failures: 0,
        };

        let analysis = controller.convert_to_analysis_result_with_metadata(
            &sim_result,
            crate::state::AnalysisType::Corner,
            "Corner",
        );
        assert_eq!(analysis.analysis_type, crate::state::AnalysisType::Corner);
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].x.len(), 3);
    }

    #[test]
    fn test_color_for_index_cycles() {
        // Test that colors cycle properly
        let color0 = SimulationController::color_for_index(0);
        let color1 = SimulationController::color_for_index(1);
        let color8 = SimulationController::color_for_index(8); // Should wrap to 0

        assert_ne!(color0, color1);
        assert_eq!(color0, color8); // Wraps around after 8 colors
    }

    #[test]
    fn test_color_for_index_valid_hex() {
        for i in 0..8 {
            let color = SimulationController::color_for_index(i);
            assert!(color.starts_with('#'));
            assert_eq!(color.len(), 7); // #RRGGBB format
        }
    }

    #[test]
    fn test_current_config_none_initially() {
        let controller = SimulationController::new();
        assert!(controller.current_config.is_none());
        assert!(controller.current_spec.is_none());
    }
}
