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
use crate::simulation::multi_run::{AnalysisPlan, AnalysisSpec, FrequencySweep};
use crate::simulation::reliability_engine::ReliabilityEngine;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue};

#[derive(Debug, Clone)]
struct QueuedAnalysis {
    spec: AnalysisSpec,
    config: AnalysisConfig,
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

        let analysis_lines: Vec<String> =
            queued.iter().map(|item| item.config.to_spice()).collect();
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
            .map(|entry| self.analysis_name(&entry.config))
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
        let config = next_analysis.config;

        self.current_analysis_idx += 1;
        self.current_config = Some(config.clone());

        // Update status with multi-analysis progress
        let status_msg = if self.total_analyses > 1 {
            format!(
                "Analysis {}/{}: {}",
                self.current_analysis_idx,
                self.total_analyses,
                self.analysis_name(&config)
            )
        } else {
            self.analysis_name(&config).to_string()
        };
        state.simulation.status = status_msg.clone();

        // Log to console
        state.console_messages.push(ConsoleMessage::info(format!(
            "Starting {}...",
            if self.total_analyses > 1 {
                format!(
                    "{} ({}/{})",
                    self.analysis_name(&config),
                    self.current_analysis_idx,
                    self.total_analyses
                )
            } else {
                self.analysis_name(&config).to_string()
            }
        )));

        // Use cached netlist
        let netlist = self
            .cached_netlist
            .clone()
            .expect("Netlist should be cached");

        // Start the simulation
        match self.runner.start(config, netlist) {
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
                            config,
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

    /// Convert SimulationResult to AnalysisResult for storage in Results Browser
    ///
    /// Extracts data from the engine's SimulationResult and creates an AnalysisResult
    /// with the appropriate type and data for display.
    fn convert_to_analysis_result(
        &self,
        sim_result: &crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        use crate::simulation::SimulationResult;
        use crate::state::WaveformData;

        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();

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

                AnalysisResult::new(1, analysis_type, label).with_dc_op(state_dc_op)
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
                AnalysisResult::new(1, analysis_type, label).with_waveforms(wf_data)
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
                AnalysisResult::new(1, analysis_type, label).with_waveforms(wf_data)
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
                AnalysisResult::new(1, analysis_type, label).with_waveforms(wf_data)
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
                AnalysisResult::new(1, analysis_type, label).with_waveforms(wf_data)
            }

            SimulationResult::PoleZero { .. } => {
                // Pole-Zero results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label)
            }

            SimulationResult::Sensitivity { .. } => {
                // Sensitivity results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label)
            }

            SimulationResult::Empty { .. } => AnalysisResult::new(1, analysis_type, label),
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
                    let completion_msg = if self.total_analyses > 1 {
                        format!(
                            "{} completed ({}/{})",
                            self.current_config
                                .as_ref()
                                .map(|c| self.analysis_name(c))
                                .unwrap_or("Analysis"),
                            self.current_analysis_idx,
                            self.total_analyses
                        )
                    } else {
                        "Simulation completed successfully".to_string()
                    };
                    state
                        .console_messages
                        .push(ConsoleMessage::info(completion_msg));

                    // Convert SimulationResult to AnalysisResult and add to run
                    if let Some(config) = &self.current_config {
                        let analysis_result = self.convert_to_analysis_result(&sim_result, config);
                        if let Some(run) = state.simulation.active_run_mut() {
                            run.add_analysis(analysis_result);
                            log::info!(
                                "Added analysis to run {} (now has {} analyses)",
                                run.id,
                                run.analyses.len()
                            );
                        }
                    }

                    // Update waveform data (legacy compatibility)
                    self.update_waveforms(state, &sim_result);

                    // Set axis labels based on current analysis type
                    if let Some(config) = &self.current_config {
                        let analysis_type = self.config_to_analysis_type(config);
                        let (x_label, x_unit, y_label, y_unit) = analysis_type.axis_info();
                        state.waveform_viewer.x_axis_label = x_label.to_string();
                        state.waveform_viewer.x_axis_unit = x_unit.to_string();
                        state.waveform_viewer.y_axis_label = y_label.to_string();
                        state.waveform_viewer.y_axis_unit = y_unit.to_string();
                    }

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
                    if let Some(config) = &self.current_config {
                        let failed_analysis = AnalysisResult::failed(
                            1,
                            self.config_to_analysis_type(config),
                            self.analysis_name(config),
                            e.to_string(),
                        );
                        if let Some(run) = state.simulation.active_run_mut() {
                            run.add_analysis(failed_analysis);
                            run.success = false;
                        }
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
            AnalysisConfig::PoleZero(pz) => {
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
            AnalysisConfig::Transient(tran) => {
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
        use crate::simulation::results::DcOpResult as EngineDcOpResult;
        use crate::simulation::SimulationResult;

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
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
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
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
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
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
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
        use crate::simulation::config::{PoleZeroConfig, PzAnalysisType};
        use crate::simulation::SimulationResult;

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
    }
}
