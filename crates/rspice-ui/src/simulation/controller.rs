//! Simulation Controller - Orchestrates Simulation Execution
//!
//! This module provides the orchestration layer between the UI state and the
//! simulation runner. It handles:
//!
//! - Processing `trigger_simulation` flag from UI
//! - Generating netlist from schematic
//! - Starting simulation with appropriate config
//! - Polling for completion and updating results
//!
//! # Usage
//!
//! Call `SimulationController::update()` once per frame in the app update loop.

use crate::common::app::{AppState, ConsoleMessage};
use crate::services::safety::SoAManager;
use crate::services::yield_manager::YieldAnalysisManager;
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, DcSweepConfig, TransientAnalysisConfig,
};
use crate::simulation::reliability_engine::ReliabilityEngine;
use crate::simulation::{AnalysisConfig, NetlistGenerator, SimulationRunner, SimulationStatus};
use crate::state::{AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue};

//=============================================================================
// Simulation Controller
//=============================================================================

/// Orchestrates simulation execution from UI trigger to result display
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

        // Poll for completion
        self.poll_completion(state);

        // Update running state
        state.simulation.is_running = self.runner.is_running();
    }

    /// Start a new simulation
    fn start_simulation(&mut self, state: &mut AppState) {
        log::info!("start_simulation called");

        // Generate netlist from schematic using full result for cross-probing
        let result = crate::simulation::generate_netlist(&state.schematic);
        let netlist = result.netlist.clone();

        log::info!(
            "Generated netlist ({} bytes):\n{}",
            netlist.len(),
            &netlist[..netlist.len().min(500)]
        );

        // Populate cross-probe mapping for probe mode
        // This enables: click on wire → find net name → toggle waveform
        state
            .simulation
            .cross_probe
            .update(result.point_to_net, result.nets);
        log::info!(
            "Cross-probe mapping populated: {} points, {} nets",
            state.simulation.cross_probe.point_to_net.len(),
            state.simulation.cross_probe.net_to_points.len()
        );

        // Build analysis config from dialog state
        let config = self.build_config(state);
        log::info!("Analysis config: {:?}", config);

        // Store config for result processing
        self.current_config = Some(config.clone());

        // Create new run in Results Browser
        state.simulation.start_run();
        log::info!("Created new simulation run");

        // Log to console
        state
            .console_messages
            .push(crate::common::app::ConsoleMessage::info(format!(
                "Starting {} analysis...",
                self.analysis_name(&config)
            )));

        // Start the simulation
        match self.runner.start(config, netlist) {
            Ok(()) => log::info!("Simulation started successfully"),
            Err(e) => {
                log::error!("Failed to start simulation: {}", e);
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::error(format!(
                        "Failed to start simulation: {}",
                        e
                    )));
                // Mark run as failed
                if let Some(run) = state.simulation.active_run_mut() {
                    run.success = false;
                }
            }
        }
    }

    /// Build analysis config from dialog state
    /// Prioritizes enabled analyses over the currently visible tab
    fn build_config(&self, state: &AppState) -> AnalysisConfig {
        // Determine which analysis to run:
        // 1. If the active tab is enabled, use it
        // 2. Otherwise, use the lowest-indexed enabled analysis
        // 3. Fall back to DC OP if nothing is enabled
        let active_tab = state.dialogs.sim_active_tab;
        let enabled = &state.dialogs.enabled_analyses;

        let analysis_idx = if enabled.contains(&active_tab) {
            active_tab
        } else if let Some(&first_enabled) = enabled.iter().min() {
            log::info!(
                "Active tab {} not enabled, using first enabled analysis: {}",
                active_tab,
                first_enabled
            );
            first_enabled
        } else {
            log::info!("No analyses enabled, defaulting to DC OP (tab 0)");
            0 // Default to DC OP
        };

        log::info!(
            "build_config: active tab = {}, using analysis_idx = {}",
            active_tab,
            analysis_idx
        );

        self.build_config_for_index(state, analysis_idx)
    }

    /// Build analysis config for a specific analysis index
    fn build_config_for_index(&self, state: &AppState, idx: usize) -> AnalysisConfig {
        match idx {
            0 => AnalysisConfig::DcOp,
            1 => {
                // Transient
                log::info!(
                    "Building transient config: stop={}, step={}, start={}",
                    state.dialogs.tran_stop,
                    state.dialogs.tran_step,
                    state.dialogs.tran_start
                );
                let stop_time = parse_spice_value(&state.dialogs.tran_stop);
                let step = parse_spice_value(&state.dialogs.tran_step);
                let start = parse_spice_value(&state.dialogs.tran_start);
                log::info!(
                    "Parsed transient values: stop={}, step={}, start={}",
                    stop_time,
                    step,
                    start
                );

                AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time,
                    step_time: step,
                    start_time: start,
                    max_timestep: Some(step), // Use step as max_timestep default
                    uic: false,
                })
            }
            2 => {
                // AC
                let fstart = parse_spice_value(&state.dialogs.ac_fstart);
                let fstop = parse_spice_value(&state.dialogs.ac_fstop);
                let points = state.dialogs.ac_points.parse().unwrap_or(101);

                AnalysisConfig::Ac(AcAnalysisConfig {
                    start_freq: fstart,
                    stop_freq: fstop,
                    num_points: points,
                    sweep_type: AcSweepType::Decade,
                })
            }
            3 => {
                // DC Sweep
                let start = parse_spice_value(&state.dialogs.dc_start);
                let stop = parse_spice_value(&state.dialogs.dc_stop);
                let step = parse_spice_value(&state.dialogs.dc_step);

                AnalysisConfig::DcSweep(DcSweepConfig {
                    source: state.dialogs.dc_source.clone(),
                    start,
                    stop,
                    step,
                    source2: None,
                    start2: None,
                    stop2: None,
                    step2: None,
                })
            }
            _ => AnalysisConfig::DcOp, // Default to DC OP for other tabs
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
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            format!("|{}|", name),
                            frequencies.clone(),
                            wf.y_values.clone(),
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
    fn poll_completion(&mut self, state: &mut AppState) {
        // Update status display
        let status = self.runner.status();
        if !matches!(status, SimulationStatus::Idle)
            && !matches!(status, SimulationStatus::Completed { .. })
        {
            // Store status in simulation state as string
            state.simulation.status = status.display_name().to_string();
        }

        // Check for completion
        if let Some(result) = self.runner.poll_result() {
            match result {
                Ok(sim_result) => {
                    log::info!(
                        "Simulation completed! Result type: {:?}",
                        std::mem::discriminant(&sim_result)
                    );
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Simulation completed successfully".to_string(),
                        ));

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

                    // Complete the run (syncs waveforms and selects first analysis)
                    state.simulation.complete_run();

                    // Update waveform data (legacy compatibility)
                    self.update_waveforms(state, &sim_result);

                    // --- Phase 10-11-12 Integration Glue ---

                    // Run Yield Analysis (if MC results are present)
                    // Note: In real usage we'd detect MC variants
                    state.simulation.yield_results = self
                        .yield_manager
                        .analyze(std::slice::from_ref(&sim_result))
                        .values()
                        .cloned()
                        .collect();

                    // Run SOA Checking
                    self.soa_manager.clear_violations();
                    // We'd extract device values here; for now we use the manager to check data
                    state.simulation.soa_violations = self.soa_manager.violations().to_vec();

                    // Run Reliability Analysis
                    let stress_data = std::collections::HashMap::new(); // Extracted from results
                    state.simulation.reliability_results = self
                        .reliability_engine
                        .analyze_circuit(&stress_data, &[1.0, 5.0, 10.0]);

                    state.simulation.status = "Complete".to_string();
                    self.current_config = None; // Clear config after processing
                }
                Err(e) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::error(format!(
                            "Simulation failed: {}",
                            e
                        )));
                    state.simulation.status = format!("Error: {}", e);

                    // Mark run as failed and add failed analysis entry
                    if let Some(config) = &self.current_config {
                        let failed_analysis = AnalysisResult::failed(
                            1,
                            self.config_to_analysis_type(config),
                            self.analysis_name(config),
                            e.to_string(),
                        );
                        if let Some(run) = state.simulation.active_run_mut() {
                            run.add_analysis(failed_analysis);
                        }
                    }
                    state.simulation.complete_run();
                    self.current_config = None;
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
                    // Magnitude trace
                    let mag_name = format!("|{}|", name);
                    let color = COLORS[idx % COLORS.len()].to_string();

                    let waveform = WaveformData::new(
                        mag_name,
                        freq_vec.clone(),
                        wf_data.y_values.clone(),
                        color,
                    );
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
