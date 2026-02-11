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

mod results;
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

    fn enabled_analysis_indices(state: &AppState) -> Vec<usize> {
        let mut indices: Vec<usize> = state.dialogs.enabled_analyses.iter().copied().collect();
        indices.sort_unstable();
        if indices.is_empty() {
            indices.push(state.dialogs.sim_active_tab.min(24));
        }
        indices
    }

    fn analysis_label_for_index(idx: usize) -> &'static str {
        match idx {
            0 => "DC Operating Point",
            1 => "Transient",
            2 => "AC",
            24 => "DISTO",
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
            AnalysisSpec::Tf
                | AnalysisSpec::Disto { .. }
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pstb
                | AnalysisSpec::Stb { .. }
                | AnalysisSpec::MonteCarlo
                | AnalysisSpec::Parametric
                | AnalysisSpec::Corner
                | AnalysisSpec::Pss { .. }
                | AnalysisSpec::HarmonicBalance { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::SParameter { .. }
                | AnalysisSpec::Envelope { .. }
                | AnalysisSpec::Fourier { .. }
                | AnalysisSpec::Reliability { .. }
                | AnalysisSpec::Optimization { .. }
                | AnalysisSpec::Soa { .. }
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
                    pac: None,
                    pxf: None,
                    tf: None,
                    pnoise: None,
                    pstb: None,
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
                    pac: None,
                    pxf: None,
                    tf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Pac => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: Some(Self::pac_run_config_from_dialog(state)?),
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pxf => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: Some(Self::pxf_run_config_from_dialog(state)?),
                tf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Tf => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: Some(Self::tf_run_config_from_dialog(state)?),
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pnoise => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: Some(Self::pnoise_run_config_from_dialog(state)?),
                pstb: None,
            }),
            AnalysisSpec::Pstb => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: Some(Self::pstb_run_config_from_dialog(state)?),
            }),
            _ => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: None,
            }),
        }
    }

    fn pac_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PacRunConfig, String> {
        use crate::services::simulation_runner::{PacFrequencySweep, PacRunConfig};

        let mut pac_state = state.dialogs.pac_state.clone();
        pac_state.ensure_initialized();
        let pac_cfg = pac_state
            .to_config()
            .map_err(|e| format!("invalid PAC settings: {}", e))?;

        let mut pss_state = state.dialogs.pss_state.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PAC: {}", e))?;

        let sweep = match pac_cfg.sweep_type {
            crate::simulation::dialog::pac::PacSweepType::Decade => PacFrequencySweep::Decade,
            crate::simulation::dialog::pac::PacSweepType::Octave => PacFrequencySweep::Octave,
            crate::simulation::dialog::pac::PacSweepType::Linear => PacFrequencySweep::Linear,
        };

        let output_ref =
            (!pac_cfg.output_ref.trim().is_empty()).then(|| pac_cfg.output_ref.clone());
        let (reltol, abstol) = Self::periodic_solver_tolerances(state);

        Ok(PacRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            start_freq: pac_cfg.start_freq,
            stop_freq: pac_cfg.stop_freq,
            points_per_unit: pac_cfg.num_points as usize,
            sweep,
            max_sideband: pac_cfg.max_sideband,
            input_source: pac_cfg.input_source,
            output_node: pac_cfg.output_node,
            output_ref,
            pac_magnitude: pac_cfg.pac_magnitude,
            include_dc: pac_cfg.include_dc,
            reltol,
            abstol,
        })
    }

    fn tf_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::TfRunConfig, String> {
        use crate::services::simulation_runner::{TfFrequencySweep, TfRunConfig};

        let mut xf_state = state.dialogs.xf_state.clone();
        xf_state.ensure_initialized();
        let xf_cfg = xf_state
            .to_config()
            .map_err(|e| format!("invalid transfer-function settings: {}", e))?;

        let sweep = match xf_cfg.sweep_type {
            crate::simulation::dialog::xf::XfSweepType::Decade => TfFrequencySweep::Decade,
            crate::simulation::dialog::xf::XfSweepType::Octave => TfFrequencySweep::Octave,
            crate::simulation::dialog::xf::XfSweepType::Linear => TfFrequencySweep::Linear,
        };

        let output_ref = (!xf_cfg.output_ref.trim().is_empty()).then(|| xf_cfg.output_ref.clone());

        Ok(TfRunConfig {
            start_freq: xf_cfg.start_freq,
            stop_freq: xf_cfg.stop_freq,
            points_per_unit: xf_cfg.num_points as usize,
            sweep,
            input_source: xf_cfg.input_source,
            output_node: xf_cfg.output_node,
            output_ref,
            group_delay: xf_cfg.group_delay,
            input_impedance: xf_cfg.input_impedance,
            output_impedance: xf_cfg.output_impedance,
        })
    }

    fn pnoise_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PnoiseRunConfig, String> {
        use crate::services::simulation_runner::{
            PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig,
        };

        let mut pnoise_state = state.dialogs.pnoise_state.clone();
        pnoise_state.ensure_initialized();
        let pnoise_cfg = pnoise_state
            .to_config()
            .map_err(|e| format!("invalid PNOISE settings: {}", e))?;

        let mut pss_state = state.dialogs.pss_state.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PNOISE: {}", e))?;

        let sweep = match pnoise_cfg.sweep_type {
            crate::simulation::dialog::pnoise::PnoiseSweepType::Decade => {
                PnoiseFrequencySweep::Decade
            }
            crate::simulation::dialog::pnoise::PnoiseSweepType::Octave => {
                PnoiseFrequencySweep::Octave
            }
            crate::simulation::dialog::pnoise::PnoiseSweepType::Linear => {
                PnoiseFrequencySweep::Linear
            }
        };

        let noise_ref = match pnoise_cfg.noise_ref {
            crate::simulation::dialog::pnoise::NoiseReferenceType::Output => {
                PnoiseReference::Output
            }
            crate::simulation::dialog::pnoise::NoiseReferenceType::Input => PnoiseReference::Input,
            crate::simulation::dialog::pnoise::NoiseReferenceType::Phase => PnoiseReference::Phase,
        };

        let output_ref =
            (!pnoise_cfg.output_ref.trim().is_empty()).then(|| pnoise_cfg.output_ref.clone());
        let (reltol, abstol) = Self::periodic_solver_tolerances(state);

        Ok(PnoiseRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            start_freq: pnoise_cfg.start_freq,
            stop_freq: pnoise_cfg.stop_freq,
            points_per_unit: pnoise_cfg.num_points as usize,
            sweep,
            max_sideband: pnoise_cfg.max_sideband,
            output_node: pnoise_cfg.output_node,
            output_ref,
            input_source: pnoise_cfg.input_source,
            noise_ref,
            integrated_noise: pnoise_cfg.integrated_noise,
            noise_summary: pnoise_cfg.noise_summary,
            reltol,
            abstol,
        })
    }

    fn pxf_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PxfRunConfig, String> {
        use crate::services::simulation_runner::{PxfFrequencySweep, PxfRunConfig};

        let mut pxf_state = state.dialogs.pxf_state.clone();
        pxf_state.ensure_initialized();
        let pxf_cfg = pxf_state
            .to_config()
            .map_err(|e| format!("invalid PXF settings: {}", e))?;

        let mut pss_state = state.dialogs.pss_state.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PXF: {}", e))?;

        let sweep = match pxf_cfg.sweep_type {
            crate::simulation::dialog::pxf::PxfSweepType::Decade => PxfFrequencySweep::Decade,
            crate::simulation::dialog::pxf::PxfSweepType::Octave => PxfFrequencySweep::Octave,
            crate::simulation::dialog::pxf::PxfSweepType::Linear => PxfFrequencySweep::Linear,
        };

        let output_ref =
            (!pxf_cfg.output_ref.trim().is_empty()).then(|| pxf_cfg.output_ref.clone());
        let (reltol, abstol) = Self::periodic_solver_tolerances(state);

        Ok(PxfRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            start_freq: pxf_cfg.start_freq,
            stop_freq: pxf_cfg.stop_freq,
            points_per_unit: pxf_cfg.num_points as usize,
            sweep,
            input_source: pxf_cfg.input_source,
            input_sideband: 1,
            output_node: pxf_cfg.output_node,
            output_ref,
            output_sideband: pxf_cfg.output_sideband,
            max_sideband: pxf_cfg.max_sideband,
            reltol,
            abstol,
        })
    }

    fn pstb_run_config_from_dialog(
        state: &AppState,
    ) -> Result<crate::services::simulation_runner::PstbRunConfig, String> {
        use crate::services::simulation_runner::PstbRunConfig;

        let mut pstb_state = state.dialogs.pstb_state.clone();
        pstb_state.ensure_initialized();
        let pstb_cfg = pstb_state
            .to_config()
            .map_err(|e| format!("invalid PSTB settings: {}", e))?;

        let mut pss_state = state.dialogs.pss_state.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings required for PSTB: {}", e))?;

        Ok(PstbRunConfig {
            pss_fundamental_freq: pss_cfg.fund_freq,
            pss_num_harmonics: pss_cfg.num_harmonics as usize,
            pss_tolerance: pss_cfg.stab_tol,
            probe_instance: pstb_cfg.probe,
            max_harmonics: pstb_cfg.max_harmonics as usize,
            num_multipliers: pstb_cfg.num_multipliers as usize,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        })
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
            24 => self.build_disto_spec(state),
            3 => {
                let (source2, start2, stop2, step2) = if state.dialogs.dc_nested {
                    let source2 = state.dialogs.dc_source2.trim();
                    if source2.is_empty() {
                        return Err(
                            "nested DC sweep requires a non-empty secondary sweep source"
                                .to_string(),
                        );
                    }
                    (
                        Some(source2.to_string()),
                        Some(
                            parse_spice_value_checked(&state.dialogs.dc_start2)
                                .map_err(|e| format!("invalid secondary start value: {}", e))?,
                        ),
                        Some(
                            parse_spice_value_checked(&state.dialogs.dc_stop2)
                                .map_err(|e| format!("invalid secondary stop value: {}", e))?,
                        ),
                        Some(
                            parse_spice_value_checked(&state.dialogs.dc_step2)
                                .map_err(|e| format!("invalid secondary step value: {}", e))?,
                        ),
                    )
                } else {
                    (None, None, None, None)
                };
                Ok(AnalysisSpec::DcSweep {
                    source_name: state.dialogs.dc_source.trim().to_string(),
                    start: parse_spice_value_checked(&state.dialogs.dc_start)
                        .map_err(|e| format!("invalid start value: {}", e))?,
                    stop: parse_spice_value_checked(&state.dialogs.dc_stop)
                        .map_err(|e| format!("invalid stop value: {}", e))?,
                    step: parse_spice_value_checked(&state.dialogs.dc_step)
                        .map_err(|e| format!("invalid step value: {}", e))?,
                    source2,
                    start2,
                    stop2,
                    step2,
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
            8 => self.build_pss_spec(state),
            9 => self.build_stb_spec(state),
            10 => self.build_temperature_sweep_spec(state),
            11 => self.build_harmonic_balance_spec(state),
            12 => self.build_sp_spec(state),
            13 => self.build_pac_spec(state),
            14 => self.build_pnoise_spec(state),
            15 => self.build_pxf_spec(state),
            16 => self.build_pstb_spec(state),
            17 => self.build_tf_spec(state),
            18 => self.build_corner_sweep_spec(state),
            19 => self.build_envelope_spec(state),
            20 => self.build_fourier_spec(state),
            21 => self.build_reliability_spec(state),
            22 => self.build_optimization_spec(state),
            23 => self.build_soa_spec(state),
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
                source2,
                start2,
                stop2,
                step2,
            } => Ok(AnalysisConfig::DcSweep(DcSweepConfig {
                source: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
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
            AnalysisSpec::Pss { .. } => self.build_pss_command(state),
            AnalysisSpec::Stb { .. } => self.build_stb_command(state),
            AnalysisSpec::HarmonicBalance { .. } => self.build_harmonic_balance_command(state),
            AnalysisSpec::SParameter { .. } => self.build_sp_command(state),
            AnalysisSpec::Envelope { .. } => self.build_envelope_command(state),
            AnalysisSpec::Fourier { .. } => self.build_fourier_command(state),
            AnalysisSpec::Reliability { .. } => self.build_reliability_command(state),
            AnalysisSpec::Optimization { .. } => self.build_optimization_command(state),
            AnalysisSpec::Soa { .. } => self.build_soa_command(state),
            AnalysisSpec::Disto { .. } => self.build_disto_command(state),
            AnalysisSpec::Pac => self.build_pac_command(state),
            AnalysisSpec::Pnoise => self.build_pnoise_command(state),
            AnalysisSpec::Pxf => self.build_pxf_command(state),
            AnalysisSpec::Pstb => self.build_pstb_command(state),
            AnalysisSpec::Tf => self.build_tf_command(state),
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

    fn build_pss_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pss_state = state.dialogs.pss_state.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings: {}", e))?;
        Ok(AnalysisSpec::Pss {
            fundamental_freq: pss_cfg.fund_freq,
            num_harmonics: pss_cfg.num_harmonics as usize,
            tolerance: pss_cfg.stab_tol,
        })
    }

    fn build_stb_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut stb_state = state.dialogs.stb_state.clone();
        stb_state.ensure_initialized();
        let stb_cfg = stb_state
            .to_config()
            .map_err(|e| format!("invalid STB settings: {}", e))?;
        Ok(AnalysisSpec::Stb {
            probe_node: stb_cfg.probe_source,
            start_freq: stb_cfg.start_freq,
            stop_freq: stb_cfg.stop_freq,
            points_per_decade: stb_cfg.points_per_decade as usize,
        })
    }

    fn build_harmonic_balance_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut hb_state = state.dialogs.hb_state.clone();
        hb_state.ensure_initialized();
        let hb_cfg = hb_state
            .to_config()
            .map_err(|e| format!("invalid harmonic balance settings: {}", e))?;
        let mut tones = Vec::with_capacity(1 + hb_cfg.additional_tones.len());
        let primary_name = if hb_cfg.fundamental_name.trim().is_empty() {
            "tone1".to_string()
        } else {
            hb_cfg.fundamental_name.trim().to_string()
        };
        let mut primary_tone =
            HbToneSpec::new(hb_cfg.fundamental_freq, hb_cfg.num_harmonics as usize)
                .with_name(primary_name);
        if let Some(source) = hb_cfg
            .fundamental_source
            .as_deref()
            .map(str::trim)
            .filter(|source| !source.is_empty())
        {
            primary_tone = primary_tone.with_source(source.to_string());
        }
        tones.push(primary_tone);
        for (idx, tone) in hb_cfg.additional_tones.iter().enumerate() {
            let label = if tone.name.trim().is_empty() {
                format!("tone{}", idx + 2)
            } else {
                tone.name.clone()
            };
            let mut tone_spec =
                HbToneSpec::new(tone.frequency, tone.harmonics as usize).with_name(label);
            if let Some(source) = tone
                .source
                .as_deref()
                .map(str::trim)
                .filter(|source| !source.is_empty())
            {
                tone_spec = tone_spec.with_source(source.to_string());
            }
            tones.push(tone_spec);
        }
        Ok(AnalysisSpec::HarmonicBalance {
            tones,
            reltol: hb_cfg.reltol,
            abstol: hb_cfg.abstol,
            max_iterations: hb_cfg.maxiter as usize,
            damping: hb_cfg.damping,
            oversample: hb_cfg.oversample as usize,
            max_mixing_order: hb_cfg.max_mixing_order as usize,
            use_krylov: matches!(
                hb_cfg.solver,
                crate::simulation::dialog::hb::HbSolverType::Krylov
            ),
            gmres_restart: hb_cfg.gmres_restart as usize,
            source_stepping: hb_cfg.source_stepping,
            verbose: hb_cfg.verbose,
        })
    }

    fn build_sp_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut sp_state = state.dialogs.sp_state.clone();
        sp_state.ensure_initialized();
        let sp_cfg = sp_state
            .to_config()
            .map_err(|e| format!("invalid S-parameter settings: {}", e))?;
        let ports = sp_cfg
            .ports
            .iter()
            .map(|port| SpPort {
                node_pos: port.node_pos.clone(),
                node_neg: port.node_neg.clone(),
                z0: port.z0,
            })
            .collect();
        Ok(AnalysisSpec::SParameter {
            start_freq: sp_cfg.start_freq,
            stop_freq: sp_cfg.stop_freq,
            points_per_unit: sp_cfg.num_points as usize,
            sweep: match sp_cfg.sweep_type {
                crate::simulation::dialog::sp::SpSweepType::Decade => FrequencySweep::Decade,
                crate::simulation::dialog::sp::SpSweepType::Octave => FrequencySweep::Octave,
                crate::simulation::dialog::sp::SpSweepType::Linear => FrequencySweep::Linear,
            },
            z0: sp_cfg.z0,
            ports,
        })
    }

    fn build_envelope_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut envelope_state = state.dialogs.envelope_state.clone();
        envelope_state.ensure_initialized();
        let envelope_cfg = envelope_state
            .to_config()
            .map_err(|e| format!("invalid envelope settings: {}", e))?;
        let max_step = (envelope_cfg.max_step > 0.0).then_some(envelope_cfg.max_step);
        Ok(AnalysisSpec::Envelope {
            fundamental_freq: envelope_cfg.fundamental_freq,
            stop_time: envelope_cfg.stop_time,
            num_harmonics: envelope_cfg.num_harmonics as usize,
            max_step,
        })
    }

    fn build_fourier_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut fourier_state = state.dialogs.fourier_state.clone();
        fourier_state.ensure_initialized();
        let fourier_cfg = fourier_state
            .to_config()
            .map_err(|e| format!("invalid Fourier settings: {}", e))?;
        Ok(AnalysisSpec::Fourier {
            fundamental_freq: fourier_cfg.fundamental_freq,
            num_harmonics: fourier_cfg.num_harmonics as usize,
            output_node: fourier_cfg.output_node.clone(),
            output_ref: fourier_cfg.output_ref.clone(),
            start_time: fourier_cfg.start_time,
            stop_time: fourier_cfg.stop_time,
        })
    }

    fn build_reliability_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut reliability_state = state.dialogs.reliability_state.clone();
        reliability_state.ensure_initialized();
        let reliability_cfg = reliability_state
            .to_config()
            .map_err(|e| format!("invalid reliability settings: {}", e))?;
        Ok(AnalysisSpec::Reliability {
            target_years: reliability_cfg.target_years,
            enable_hci: reliability_cfg.enable_hci,
            enable_nbti: reliability_cfg.enable_nbti,
            enable_em: reliability_cfg.enable_em,
            min_stress_voltage: reliability_cfg.min_stress_voltage,
        })
    }

    fn build_optimization_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut optimization_state = state.dialogs.optimization_state.clone();
        optimization_state.ensure_initialized();
        let cfg = optimization_state
            .to_config()
            .map_err(|e| format!("invalid optimization settings: {}", e))?;

        Ok(AnalysisSpec::Optimization {
            variables: cfg
                .variables
                .into_iter()
                .map(|var| OptimizationVariable {
                    name: var.name,
                    min: var.min,
                    max: var.max,
                    initial: var.initial,
                })
                .collect(),
            objective_node: cfg.objective_node,
            objective_ref: cfg.objective_ref,
            goal: match cfg.goal_mode {
                crate::simulation::dialog::optimization::OptimizationGoalMode::Minimize => {
                    OptimizationGoal::Minimize
                }
                crate::simulation::dialog::optimization::OptimizationGoalMode::Maximize => {
                    OptimizationGoal::Maximize
                }
                crate::simulation::dialog::optimization::OptimizationGoalMode::Target => {
                    OptimizationGoal::Target
                }
            },
            target: cfg.target_value,
            algorithm: match cfg.algorithm {
                crate::simulation::dialog::optimization::OptimizationAlgorithmMode::GradientDescent => {
                    OptimizationAlgorithm::GradientDescent
                }
                crate::simulation::dialog::optimization::OptimizationAlgorithmMode::PatternSearch => {
                    OptimizationAlgorithm::PatternSearch
                }
                crate::simulation::dialog::optimization::OptimizationAlgorithmMode::SimulatedAnnealing => {
                    OptimizationAlgorithm::SimulatedAnnealing
                }
            },
            max_iterations: cfg.max_iterations,
            cost_tolerance: cfg.cost_tolerance,
            fd_step: cfg.fd_step,
            initial_step: cfg.initial_step,
            min_step: cfg.min_step,
        })
    }

    fn build_soa_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut soa_state = state.dialogs.soa_state.clone();
        soa_state.ensure_initialized();
        let cfg = soa_state
            .to_config()
            .map_err(|e| format!("invalid SOA settings: {}", e))?;
        Ok(AnalysisSpec::Soa {
            stop_time: cfg.stop_time,
            step_time: cfg.step_time,
            check_vgs_max: cfg.check_vgs_max,
            max_vgs: cfg.max_vgs,
            check_vds_max: cfg.check_vds_max,
            max_vds: cfg.max_vds,
            check_vbe_max: cfg.check_vbe_max,
            max_vbe: cfg.max_vbe,
            check_vce_max: cfg.check_vce_max,
            max_vce: cfg.max_vce,
        })
    }

    fn build_pac_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pac_state = state.dialogs.pac_state.clone();
        pac_state.ensure_initialized();
        pac_state
            .to_config()
            .map_err(|e| format!("invalid PAC settings: {}", e))?;
        Ok(AnalysisSpec::Pac)
    }

    fn build_pnoise_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pnoise_state = state.dialogs.pnoise_state.clone();
        pnoise_state.ensure_initialized();
        pnoise_state
            .to_config()
            .map_err(|e| format!("invalid PNOISE settings: {}", e))?;
        Ok(AnalysisSpec::Pnoise)
    }

    fn build_pxf_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pxf_state = state.dialogs.pxf_state.clone();
        pxf_state.ensure_initialized();
        pxf_state
            .to_config()
            .map_err(|e| format!("invalid PXF settings: {}", e))?;
        Ok(AnalysisSpec::Pxf)
    }

    fn build_pstb_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut pstb_state = state.dialogs.pstb_state.clone();
        pstb_state.ensure_initialized();
        pstb_state
            .to_config()
            .map_err(|e| format!("invalid PSTB settings: {}", e))?;
        Ok(AnalysisSpec::Pstb)
    }

    fn build_tf_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        let mut xf_state = state.dialogs.xf_state.clone();
        xf_state.ensure_initialized();
        xf_state
            .to_config()
            .map_err(|e| format!("invalid transfer-function settings: {}", e))?;
        Ok(AnalysisSpec::Tf)
    }

    fn build_disto_spec(&self, state: &AppState) -> Result<AnalysisSpec, String> {
        Ok(AnalysisSpec::Disto {
            start_freq: parse_spice_value_checked(&state.dialogs.ac_fstart)
                .map_err(|e| format!("invalid DISTO start frequency: {}", e))?,
            stop_freq: parse_spice_value_checked(&state.dialogs.ac_fstop)
                .map_err(|e| format!("invalid DISTO stop frequency: {}", e))?,
            points_per_unit: Self::parse_positive_points(&state.dialogs.ac_points, "ac_points")?,
            sweep: Self::map_frequency_sweep(state.dialogs.ac_sweep_type),
            f2_over_f1: Self::parse_optional_spice_value(&state.dialogs.disto_f2_over_f1)
                .map_err(|e| format!("invalid DISTO f2/f1 ratio: {}", e))?,
        })
    }

    fn build_monte_carlo_command(&self, state: &AppState) -> Result<String, String> {
        let mut mc_state = state.dialogs.mc_state.clone();
        mc_state.ensure_initialized();
        let mc_cfg = mc_state
            .to_config()
            .map_err(|e| format!("invalid Monte Carlo settings: {}", e))?;

        let dist_keyword = match mc_cfg.distribution {
            crate::simulation::dialog::mc::McDistribution::Gaussian => "GAUSS",
            crate::simulation::dialog::mc::McDistribution::Uniform => "UNIFORM",
            crate::simulation::dialog::mc::McDistribution::WorstCase => "WORSTCASE",
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

    fn build_pss_command(&self, state: &AppState) -> Result<String, String> {
        let mut pss_state = state.dialogs.pss_state.clone();
        pss_state.ensure_initialized();
        let pss_cfg = pss_state
            .to_config()
            .map_err(|e| format!("invalid PSS settings: {}", e))?;
        Ok(pss_cfg.to_spice())
    }

    fn build_stb_command(&self, state: &AppState) -> Result<String, String> {
        let mut stb_state = state.dialogs.stb_state.clone();
        stb_state.ensure_initialized();
        let stb_cfg = stb_state
            .to_config()
            .map_err(|e| format!("invalid STB settings: {}", e))?;
        Ok(stb_cfg.to_spice())
    }

    fn build_harmonic_balance_command(&self, state: &AppState) -> Result<String, String> {
        let mut hb_state = state.dialogs.hb_state.clone();
        hb_state.ensure_initialized();
        let hb_cfg = hb_state
            .to_config()
            .map_err(|e| format!("invalid harmonic balance settings: {}", e))?;
        Ok(hb_cfg.to_spice())
    }

    fn build_sp_command(&self, state: &AppState) -> Result<String, String> {
        let mut sp_state = state.dialogs.sp_state.clone();
        sp_state.ensure_initialized();
        let sp_cfg = sp_state
            .to_config()
            .map_err(|e| format!("invalid S-parameter settings: {}", e))?;
        Ok(sp_cfg.to_spice())
    }

    fn build_envelope_command(&self, state: &AppState) -> Result<String, String> {
        let mut envelope_state = state.dialogs.envelope_state.clone();
        envelope_state.ensure_initialized();
        let envelope_cfg = envelope_state
            .to_config()
            .map_err(|e| format!("invalid envelope settings: {}", e))?;
        Ok(envelope_cfg.to_spice())
    }

    fn build_fourier_command(&self, state: &AppState) -> Result<String, String> {
        let mut fourier_state = state.dialogs.fourier_state.clone();
        fourier_state.ensure_initialized();
        let fourier_cfg = fourier_state
            .to_config()
            .map_err(|e| format!("invalid Fourier settings: {}", e))?;
        Ok(fourier_cfg.to_spice())
    }

    fn build_reliability_command(&self, state: &AppState) -> Result<String, String> {
        let mut reliability_state = state.dialogs.reliability_state.clone();
        reliability_state.ensure_initialized();
        let reliability_cfg = reliability_state
            .to_config()
            .map_err(|e| format!("invalid reliability settings: {}", e))?;
        Ok(reliability_cfg.to_spice())
    }

    fn build_optimization_command(&self, state: &AppState) -> Result<String, String> {
        let mut optimization_state = state.dialogs.optimization_state.clone();
        optimization_state.ensure_initialized();
        let optimization_cfg = optimization_state
            .to_config()
            .map_err(|e| format!("invalid optimization settings: {}", e))?;
        Ok(optimization_cfg.to_spice())
    }

    fn build_soa_command(&self, state: &AppState) -> Result<String, String> {
        let mut soa_state = state.dialogs.soa_state.clone();
        soa_state.ensure_initialized();
        let soa_cfg = soa_state
            .to_config()
            .map_err(|e| format!("invalid SOA settings: {}", e))?;
        Ok(soa_cfg.to_spice())
    }

    fn build_pac_command(&self, state: &AppState) -> Result<String, String> {
        let mut pac_state = state.dialogs.pac_state.clone();
        pac_state.ensure_initialized();
        let pac_cfg = pac_state
            .to_config()
            .map_err(|e| format!("invalid PAC settings: {}", e))?;
        Ok(pac_cfg.to_spice())
    }

    fn build_pnoise_command(&self, state: &AppState) -> Result<String, String> {
        let mut pnoise_state = state.dialogs.pnoise_state.clone();
        pnoise_state.ensure_initialized();
        let pnoise_cfg = pnoise_state
            .to_config()
            .map_err(|e| format!("invalid PNOISE settings: {}", e))?;
        Ok(pnoise_cfg.to_spice())
    }

    fn build_pxf_command(&self, state: &AppState) -> Result<String, String> {
        let mut pxf_state = state.dialogs.pxf_state.clone();
        pxf_state.ensure_initialized();
        let pxf_cfg = pxf_state
            .to_config()
            .map_err(|e| format!("invalid PXF settings: {}", e))?;
        Ok(pxf_cfg.to_spice())
    }

    fn build_pstb_command(&self, state: &AppState) -> Result<String, String> {
        let mut pstb_state = state.dialogs.pstb_state.clone();
        pstb_state.ensure_initialized();
        let pstb_cfg = pstb_state
            .to_config()
            .map_err(|e| format!("invalid PSTB settings: {}", e))?;
        Ok(pstb_cfg.to_spice())
    }

    fn build_tf_command(&self, state: &AppState) -> Result<String, String> {
        let mut xf_state = state.dialogs.xf_state.clone();
        xf_state.ensure_initialized();
        let xf_cfg = xf_state
            .to_config()
            .map_err(|e| format!("invalid transfer-function settings: {}", e))?;
        Ok(xf_cfg.to_spice())
    }

    fn build_disto_command(&self, state: &AppState) -> Result<String, String> {
        let spec = self.build_disto_spec(state)?;
        if let AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } = spec
        {
            let mut command = format!(
                ".disto {} {} {} {}",
                sweep.runner_keyword(),
                points_per_unit,
                start_freq,
                stop_freq
            );
            if let Some(ratio) = f2_over_f1 {
                command.push(' ');
                command.push_str(&ratio.to_string());
            }
            Ok(command)
        } else {
            Err("failed to build DISTO command".to_string())
        }
    }

    fn periodic_solver_tolerances(state: &AppState) -> (f64, f64) {
        let opts = &state.dialogs.simulation_options_config;
        (opts.reltol, opts.abstol)
    }

    fn apply_simulation_options_to_netlist(
        netlist: &str,
        options: &crate::simulation::dialog::SimulationOptions,
    ) -> String {
        let options_block = options.to_spice_options();
        let option_lines: Vec<&str> = options_block.lines().collect();
        if option_lines.len() <= 1 {
            return netlist.to_string();
        }

        let mut lines: Vec<String> = netlist.lines().map(|line| line.to_string()).collect();
        let insertion_idx = lines
            .iter()
            .position(|line| line.trim_start().to_ascii_lowercase().starts_with(".end"))
            .unwrap_or(lines.len());
        let injected_lines = option_lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        lines.splice(insertion_idx..insertion_idx, injected_lines);

        let mut merged = lines.join("\n");
        if netlist.ends_with('\n') {
            merged.push('\n');
        }
        merged
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
                    state.active_viewer = Self::preferred_viewer_for_analysis(analysis_type);

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
