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
use std::path::PathBuf;

use crate::common::app::{AppState, ConsoleMessage};
use crate::io::{SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter};
use crate::services::yield_manager::YieldAnalysisManager;
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, DcSweepConfig, NoiseAnalysisConfig, PoleZeroConfig,
    PzAnalysisType, SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{
    AnalysisPlan, AnalysisRunType, AnalysisSpec, FrequencySweep, HbToneSpec, OptimizationAlgorithm,
    OptimizationGoal, OptimizationVariable, SpPort,
};
use crate::simulation::runner::SpecExecutionOptions;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue};

mod analysis_commands;
mod analysis_helpers;
mod analysis_plan;
mod analysis_run_config;
mod analysis_spec_build;
mod results_convert;
mod results_post;
mod results_update;
mod spice_value;
mod touchstone;

use self::spice_value::{parse_spice_value, parse_spice_value_checked};

#[cfg(test)]
mod tests;

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
            state.push_sim_message(crate::common::app::ConsoleMessage::warning(
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
                    state.push_sim_message(ConsoleMessage::error(err));
                }
                state.simulation.status = "Configuration error".to_string();
                return;
            }
        };

        let queued = match self.build_queue_from_plan(state, &plan) {
            Ok(queue) => queue,
            Err(errors) => {
                for err in errors {
                    state.push_sim_message(ConsoleMessage::error(err));
                }
                state.simulation.status = "Configuration error".to_string();
                return;
            }
        };

        self.total_analyses = queued.len();
        if self.total_analyses == 0 {
            state.push_sim_message(ConsoleMessage::error(
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
        let mut netlist = result.netlist.clone();

        if !result.errors.is_empty() {
            for err in result.errors {
                state.push_sim_message(ConsoleMessage::error(err));
            }
            state.simulation.status = "Netlist error".to_string();
            return;
        }
        for warning in result.warnings {
            state.push_sim_message(ConsoleMessage::warning(warning));
        }

        netlist = Self::apply_simulation_options_to_netlist(
            &netlist,
            &state.dialogs.simulation_options_config,
        );

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
        state.simulation.reliability_results.clear();
        state.simulation.soa_violations.clear();
        log::info!("Created new simulation run");

        // Log summary to console
        if self.total_analyses > 1 {
            state.push_sim_message(ConsoleMessage::info(format!(
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
        state.push_sim_message(ConsoleMessage::info(format!(
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

        // Use cached netlist. If this is unexpectedly missing, fail gracefully
        // instead of panicking so the UI can recover.
        let Some(netlist) = self.cached_netlist.clone() else {
            let message = format!(
                "Internal error: missing cached netlist while starting {}",
                analysis_name
            );
            log::error!("{}", message);
            state.push_sim_message(ConsoleMessage::error(message));
            if let Some(run) = state.simulation.active_run_mut() {
                run.success = false;
            }
            self.pending_analyses.clear();
            self.finish_simulation_batch(state);
            state.simulation.status = "Error".to_string();
            return;
        };

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
                state.push_sim_message(ConsoleMessage::error(format!(
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
        let run_success = state
            .simulation
            .active_run()
            .map(|run| run.success)
            .unwrap_or(true);

        // Complete the run (syncs waveforms and selects first analysis)
        state.simulation.complete_run();

        // Clear cached netlist
        self.cached_netlist = None;
        self.current_config = None;
        self.current_spec = None;
        self.current_analysis_idx = 0;
        self.total_analyses = 0;

        state.simulation.status = if run_success {
            "Complete".to_string()
        } else {
            "Completed with errors".to_string()
        };

        log::info!("Simulation batch completed");
    }

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
            AnalysisRunType::Disto => AnalysisType::Disto,
            AnalysisRunType::Transient => AnalysisType::Transient,
            AnalysisRunType::Noise => AnalysisType::Noise,
            AnalysisRunType::Tf => AnalysisType::Tf,
            AnalysisRunType::Sensitivity => AnalysisType::Sensitivity,
            AnalysisRunType::PoleZero => AnalysisType::PoleZero,
            AnalysisRunType::HarmonicBalance => AnalysisType::HarmonicBalance,
            AnalysisRunType::Pss => AnalysisType::Pss,
            AnalysisRunType::Pac => AnalysisType::Pac,
            AnalysisRunType::Pnoise => AnalysisType::Pnoise,
            AnalysisRunType::Pxf => AnalysisType::Pxf,
            AnalysisRunType::Pstb => AnalysisType::Pstb,
            AnalysisRunType::Stb => AnalysisType::Stb,
            AnalysisRunType::MonteCarlo => AnalysisType::MonteCarlo,
            AnalysisRunType::Parametric => AnalysisType::Parametric,
            AnalysisRunType::Corner => AnalysisType::Corner,
            AnalysisRunType::Reliability => AnalysisType::Reliability,
            AnalysisRunType::Optimization => AnalysisType::Optimization,
            AnalysisRunType::Soa => AnalysisType::Soa,
            AnalysisRunType::SParameter => AnalysisType::SParameter,
            AnalysisRunType::Envelope => AnalysisType::Envelope,
            AnalysisRunType::Fourier => AnalysisType::Fourier,
        }
    }

    /// Convert SimulationResult to AnalysisResult for storage in Results Browser
    ///
    /// Extracts data from the engine's SimulationResult and creates an AnalysisResult
    /// with the appropriate type and data for display.
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
                    state.push_sim_message(ConsoleMessage::info(completion_msg));

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

                    // Optional Touchstone export for S-parameter analyses.
                    self.maybe_export_touchstone(state, &sim_result);

                    // Set axis labels based on current analysis type
                    let (x_label, x_unit, y_label, y_unit) = analysis_type.axis_info();
                    state.waveform_viewer.x_axis_label = x_label.to_string();
                    state.waveform_viewer.x_axis_unit = x_unit.to_string();
                    state.waveform_viewer.y_axis_label = y_label.to_string();
                    state.waveform_viewer.y_axis_unit = y_unit.to_string();
                    state.open_preferred_viewer_for_analysis(analysis_type);

                    // --- Phase 10-11-12 Integration Glue (run once per analysis) ---

                    // Run Yield Analysis (if MC results are present)
                    state.simulation.yield_results = self
                        .yield_manager
                        .analyze(std::slice::from_ref(&sim_result))
                        .values()
                        .cloned()
                        .collect();

                    // Reliability results are populated by dedicated reliability analysis runs.
                    if let crate::simulation::SimulationResult::Reliability {
                        device_results, ..
                    } = &sim_result
                    {
                        state.simulation.reliability_results = device_results.clone();
                    }
                    if let crate::simulation::SimulationResult::Soa { violations, .. } = &sim_result
                    {
                        state.simulation.soa_violations = violations.clone();
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
                        // All analyses complete - finalize the batch
                        if self.total_analyses > 1 {
                            state.push_sim_message(ConsoleMessage::info(format!(
                                "All {} analyses completed successfully",
                                self.total_analyses
                            )));
                        }
                        self.finish_simulation_batch(state);
                    }
                }
                Err(e) => {
                    state
                        .push_sim_message(ConsoleMessage::error(format!("Analysis failed: {}", e)));

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
