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
    AnalysisPlan, AnalysisRunType, AnalysisSpec, FrequencySweep, OptimizationAlgorithm,
    OptimizationGoal, OptimizationVariable, SpPort,
};
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
        let tone2 = hb_cfg.additional_tones.first();
        Ok(AnalysisSpec::HarmonicBalance {
            tone1_freq: hb_cfg.fundamental_freq,
            tone1_harmonics: hb_cfg.num_harmonics as usize,
            tone2_freq: tone2.map(|tone| tone.frequency),
            tone2_harmonics: tone2.map(|tone| tone.harmonics as usize).unwrap_or(0),
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

    fn maybe_export_touchstone(
        &self,
        state: &mut AppState,
        result: &crate::simulation::SimulationResult,
    ) {
        let Some(crate::simulation::multi_run::AnalysisSpec::SParameter { z0, ports, .. }) =
            self.current_spec.as_ref()
        else {
            return;
        };

        let mut sp_state = state.dialogs.sp_state.clone();
        sp_state.ensure_initialized();
        let sp_cfg = match sp_state.to_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                state.push_sim_message(ConsoleMessage::warning(format!(
                    "Skipping Touchstone export: invalid S-parameter settings ({})",
                    e
                )));
                return;
            }
        };
        if !sp_cfg.touchstone_export {
            return;
        }

        let run_id = state.simulation.active_run().map(|run| run.id).unwrap_or(0);
        let z0_by_port: Vec<f64> = ports.iter().map(|port| port.z0.unwrap_or(*z0)).collect();
        let dataset = match Self::build_touchstone_dataset(
            result,
            *z0,
            &z0_by_port,
            sp_cfg.touchstone_version as usize,
        ) {
            Ok(dataset) => dataset,
            Err(e) => {
                state.push_sim_message(ConsoleMessage::warning(format!(
                    "Touchstone export skipped: {}",
                    e
                )));
                return;
            }
        };
        let num_ports = dataset
            .metadata
            .get("num_ports")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(2);
        let path =
            Self::touchstone_export_path(state, run_id, self.current_analysis_idx, num_ports);

        let writer = WaveformWriter::new(WaveformFormat::Touchstone);
        match writer.write(&dataset, &path) {
            Ok(()) => state.push_sim_message(ConsoleMessage::info(format!(
                "Exported Touchstone: {}",
                path.display()
            ))),
            Err(e) => state.push_sim_message(ConsoleMessage::warning(format!(
                "Touchstone export failed: {}",
                e
            ))),
        }
    }

    fn build_touchstone_dataset(
        result: &crate::simulation::SimulationResult,
        z0: f64,
        z0_by_port: &[f64],
        touchstone_version: usize,
    ) -> Result<WaveformDataset, String> {
        let (frequencies, waveforms) = match result {
            crate::simulation::SimulationResult::Ac {
                frequencies,
                waveforms,
            } => (frequencies, waveforms),
            _ => return Err("result is not frequency-domain S-parameter data".to_string()),
        };
        if frequencies.is_empty() {
            return Err("frequency vector is empty".to_string());
        }

        let mut entries: std::collections::HashMap<
            (usize, usize),
            &crate::simulation::results::WaveformData,
        > = std::collections::HashMap::new();
        let mut max_port = 0usize;
        for (name, waveform) in waveforms {
            let matrix_index = Self::parse_sparameter_waveform_name(name)
                .or_else(|| Self::parse_sparameter_waveform_name(&waveform.name));
            let Some((row, col)) = matrix_index else {
                continue;
            };
            if entries.insert((row, col), waveform).is_some() {
                return Err(format!(
                    "duplicate S-parameter waveform for S{}{}",
                    row, col
                ));
            }
            max_port = max_port.max(row).max(col);
        }
        if max_port < 2 {
            return Err("no complete S-parameter matrix waveforms found".to_string());
        }
        let port_references = if z0_by_port.is_empty() {
            vec![z0; max_port]
        } else if z0_by_port.len() == max_port {
            z0_by_port.to_vec()
        } else {
            return Err(format!(
                "expected {} per-port reference values, got {}",
                max_port,
                z0_by_port.len()
            ));
        };
        for (idx, value) in port_references.iter().enumerate() {
            if !value.is_finite() || *value <= 0.0 {
                return Err(format!(
                    "invalid Touchstone reference impedance for port {}",
                    idx + 1
                ));
            }
        }
        let has_non_uniform_reference = port_references
            .iter()
            .any(|value| (*value - port_references[0]).abs() > 1e-18);
        if touchstone_version < 2 && has_non_uniform_reference {
            return Err(
                "Touchstone v1 export does not support per-port reference impedance".to_string(),
            );
        }

        let mut dataset = WaveformDataset::new("S-Parameters");
        dataset.analysis = "S-Parameter".to_string();
        dataset
            .metadata
            .insert("z0".to_string(), format!("{}", port_references[0]));
        dataset.metadata.insert(
            "z0_ports".to_string(),
            port_references
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        dataset
            .metadata
            .insert("num_ports".to_string(), max_port.to_string());
        dataset.metadata.insert(
            "touchstone_version".to_string(),
            touchstone_version.to_string(),
        );

        let mut x = WaveformSignal::new("frequency", SignalType::Frequency);
        x.data = frequencies.clone();
        dataset.set_x(x);

        for row in 1..=max_port {
            for col in 1..=max_port {
                let name = Self::sparameter_name(row, col, max_port);
                let waveform = entries
                    .get(&(row, col))
                    .copied()
                    .ok_or_else(|| format!("missing {} waveform", name))?;
                let imag = waveform
                    .y_imag
                    .as_ref()
                    .ok_or_else(|| format!("{} waveform is missing imaginary component", name))?;
                if waveform.y_values.len() != frequencies.len() || imag.len() != frequencies.len() {
                    return Err(format!(
                        "{} waveform length mismatch (freq={}, re={}, im={})",
                        name,
                        frequencies.len(),
                        waveform.y_values.len(),
                        imag.len()
                    ));
                }
                Self::push_complex_signal_pair(&mut dataset, &name, waveform)?;
            }
        }

        Ok(dataset)
    }

    fn push_complex_signal_pair(
        dataset: &mut WaveformDataset,
        name: &str,
        waveform: &crate::simulation::results::WaveformData,
    ) -> Result<(), String> {
        let imag = waveform
            .y_imag
            .as_ref()
            .ok_or_else(|| format!("{} waveform is missing imaginary component", name))?;

        let mut real_signal = WaveformSignal::new(format!("{}_RE", name), SignalType::SParameter);
        real_signal.data = waveform.y_values.clone();
        dataset.add_signal(real_signal);

        let mut imag_signal = WaveformSignal::new(format!("{}_IM", name), SignalType::SParameter);
        imag_signal.data = imag.clone();
        dataset.add_signal(imag_signal);

        Ok(())
    }

    fn parse_sparameter_waveform_name(name: &str) -> Option<(usize, usize)> {
        let normalized = name.trim().to_ascii_uppercase().replace(' ', "");
        let rest = normalized.strip_prefix('S')?;
        if let Some(inner) = rest
            .strip_prefix('(')
            .and_then(|value| value.strip_suffix(')'))
        {
            let (row, col) = inner.split_once(',')?;
            let row = row.trim().parse::<usize>().ok()?;
            let col = col.trim().parse::<usize>().ok()?;
            return (row > 0 && col > 0).then_some((row, col));
        }
        if let Some((row, col)) = rest.split_once('_') {
            let row = row.trim().parse::<usize>().ok()?;
            let col = col.trim().parse::<usize>().ok()?;
            return (row > 0 && col > 0).then_some((row, col));
        }
        if rest.len() == 2 && rest.chars().all(|ch| ch.is_ascii_digit()) {
            let row = rest[0..1].parse::<usize>().ok()?;
            let col = rest[1..2].parse::<usize>().ok()?;
            return Some((row, col));
        }
        None
    }

    fn sparameter_name(row: usize, col: usize, num_ports: usize) -> String {
        if num_ports <= 9 {
            format!("S{}{}", row, col)
        } else {
            format!("S{}_{}", row, col)
        }
    }

    fn touchstone_export_path(
        state: &AppState,
        run_id: u64,
        analysis_idx: usize,
        num_ports: usize,
    ) -> PathBuf {
        let source_path = state
            .schematic
            .current_file
            .as_ref()
            .or(state.simulation.current_file.as_ref());
        let (base_dir, stem) = if let Some(path) = source_path {
            let dir = path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("untitled");
            (dir, stem.to_string())
        } else {
            (
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                "untitled".to_string(),
            )
        };

        base_dir.join(format!(
            "{}_run{:04}_sp{:02}.s{}p",
            stem,
            run_id,
            analysis_idx.max(1),
            num_ports.max(2)
        ))
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

            SimulationResult::Reliability {
                years, waveforms, ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            years.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Optimization {
                iterations,
                waveforms,
                ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            iterations.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Soa {
                time, waveforms, ..
            } => {
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
                    state.push_sim_message(ConsoleMessage::error(format!("Analysis failed: {}", e)));

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
                    state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "V({}) = {:.6} V",
                            node, voltage
                        )));
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "DC OP: {} node voltages computed",
                        dc_result.node_voltages.len()
                    )));

                // Auto-show log panel so user sees results
                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Log;
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
                self.populate_transient_post_views(state, time, waveforms);

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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
                self.populate_ac_post_views(state, frequencies, waveforms);

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "Noise: {} points, integrated output: {:.3e} V/√Hz",
                        frequencies.len(),
                        integrated
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::PoleZero { poles, zeros, gain } => {
                self.populate_pole_zero_view(state, poles, zeros, *gain);
                // Pole-Zero: Display in console (and optionally s-plane plot)
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "Pole-Zero Analysis: DC gain = {:.4}",
                        gain
                    )));

                for (i, (re, im)) in poles.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        // Real pole
                        let freq = re.abs() / (2.0 * std::f64::consts::PI);
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                                "  Pole {}: {:.3e} rad/s ({:.3e} Hz)",
                                i + 1,
                                re,
                                freq
                            )));
                    } else {
                        // Complex pole
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                                "  Pole {}: {:.3e} ± j{:.3e} rad/s",
                                i + 1,
                                re,
                                im.abs()
                            )));
                    }
                }

                for (i, (re, im)) in zeros.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                                "  Zero {}: {:.3e} rad/s",
                                i + 1,
                                re
                            )));
                    } else {
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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
                self.populate_monte_carlo_histograms(state, variables);
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

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "Monte Carlo: {}/{} runs converged ({} failed), all_converged={}",
                        runs_completed, runs_requested, num_failures, all_converged
                    )));

                for var in variables.iter().take(8) {
                    state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  {}: mean={:.6e}, sigma={:.6e}, min={:.6e}, max={:.6e}",
                            var.name, var.mean, var.std_dev, var.min, var.max
                        )));
                }

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = if state.simulation.waveforms.is_empty() {
                    crate::common::app::BottomPanelTab::Log
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

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
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

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "Corner sweep: {} points, {} waveforms, {} failed corners",
                        x_values.len(),
                        waveforms.len(),
                        num_failures
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        years.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.simulation.reliability_results = device_results.clone();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "Reliability: {} lifetime points, {} devices analyzed",
                        years.len(),
                        device_results.len()
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        iterations.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "Optimization: {} iterations, best cost {:.6e}, converged={}",
                        iterations.len(),
                        best_cost,
                        converged
                    )));
                for (name, value) in best_variables.iter().take(8) {
                    state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  {} = {:.6e}",
                            name, value
                        )));
                }

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Soa {
                time,
                waveforms,
                violations,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        time.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }
                state.simulation.soa_violations = violations.clone();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "SOA: {} sampled points, {} violations",
                        time.len(),
                        violations.len()
                    )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Empty { .. } => {
                state.push_sim_message(crate::common::app::ConsoleMessage::info(
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

    fn populate_transient_post_views(
        &self,
        state: &mut AppState,
        time: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
    ) {
        let Some((_name, waveform)) = Self::primary_waveform(waveforms, time.len()) else {
            return;
        };

        if let Some(bit_period) = Self::estimate_ui_period(time, &waveform.y_values) {
            let eye_data = crate::analysis::eye_diagram::data::EyeDataBuilder::new()
                .bit_period(bit_period)
                .ui_count(2)
                .skip_initial(2)
                .build(time, &waveform.y_values);
            if eye_data.trace_count() > 0 {
                state.eye_diagram_state.load_data(eye_data);
            }
        }

        if let Some((samples, sample_rate)) =
            Self::downsample_for_fft(time, &waveform.y_values, 4096)
        {
            let fft_data = crate::analysis::fft::FftData::from_time_domain(
                &format!("FFT({})", waveform.name),
                &samples,
                sample_rate,
                state.fft_state.window,
            );
            if !fft_data.is_empty() {
                state.fft_state.load_data(fft_data);
            }
        }
    }

    fn populate_ac_post_views(
        &self,
        state: &mut AppState,
        frequencies: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
    ) {
        let mut bode_data = crate::analysis::bode::BodeData::new();
        state.nyquist_state.clear();
        state.smith_chart_state.clear_traces();

        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        let mut loaded_nyquist = false;
        for name in names {
            let Some(waveform) = waveforms.get(&name) else {
                continue;
            };
            let Some(imag) = waveform.y_imag.as_ref() else {
                continue;
            };
            if waveform.y_values.len() != frequencies.len() || imag.len() != frequencies.len() {
                continue;
            }

            let response = crate::analysis::bode::data::FrequencyResponse::from_complex_arrays(
                &name,
                frequencies,
                &waveform.y_values,
                imag,
            );
            bode_data.add_response(response);

            let nyquist_curve =
                crate::analysis::nyquist::data::NyquistData::from_arrays(
                    &name,
                    frequencies,
                    &waveform.y_values,
                    imag,
                );
            if loaded_nyquist {
                state.nyquist_state.add_curve(nyquist_curve);
            } else {
                state.nyquist_state.load_data(nyquist_curve);
                loaded_nyquist = true;
            }

            if Self::is_sparameter_trace_name(&name) {
                state.smith_chart_state.load_sparam_data(
                    &name,
                    frequencies,
                    &waveform.y_values,
                    imag,
                );
            }
        }

        if bode_data.response_count() > 0 {
            bode_data.calculate_margins();
            state.bode_plot_state.load_data(bode_data);
        } else {
            state.bode_plot_state
                .load_data(crate::analysis::bode::BodeData::new());
        }
    }

    fn populate_pole_zero_view(
        &self,
        state: &mut AppState,
        poles: &[(f64, f64)],
        zeros: &[(f64, f64)],
        gain: f64,
    ) {
        let mut data = crate::analysis::pole_zero::data::PoleZeroData::new("Pole-Zero");
        data.gain = gain;
        for &(re, im) in poles {
            data.roots
                .push(crate::analysis::pole_zero::data::ComplexRoot::pole(re, im));
        }
        for &(re, im) in zeros {
            data.roots
                .push(crate::analysis::pole_zero::data::ComplexRoot::zero(re, im));
        }
        state.pole_zero_state.load_data(data);
    }

    fn populate_monte_carlo_histograms(
        &self,
        state: &mut AppState,
        variables: &[crate::simulation::results::MonteCarloVariableResult],
    ) {
        state.histogram_state.clear();

        for variable in variables {
            if variable.histogram.is_empty() || variable.bin_edges.len() != variable.histogram.len() + 1
            {
                continue;
            }

            let mut bins = Vec::with_capacity(variable.histogram.len());
            for (idx, count) in variable.histogram.iter().enumerate() {
                bins.push(crate::analysis::histogram::data::HistogramBin {
                    lower: variable.bin_edges[idx],
                    upper: variable.bin_edges[idx + 1],
                    count: *count,
                    weight: *count as f64,
                });
            }
            let total_count: usize = variable.histogram.iter().sum();
            let histogram = crate::analysis::histogram::data::Histogram {
                name: variable.name.clone(),
                bins,
                total_count,
                total_weight: total_count as f64,
                underflow: 0,
                overflow: 0,
                data_min: *variable.bin_edges.first().unwrap_or(&0.0),
                data_max: *variable.bin_edges.last().unwrap_or(&0.0),
            };

            if state.histogram_state.is_empty() {
                state.histogram_state.load_histogram(histogram);
            } else {
                state.histogram_state.add_histogram(histogram);
            }
        }
    }

    fn primary_waveform<'a>(
        waveforms: &'a std::collections::HashMap<String, crate::simulation::WaveformData>,
        expected_len: usize,
    ) -> Option<(&'a str, &'a crate::simulation::WaveformData)> {
        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        for name in names {
            let Some(waveform) = waveforms.get(&name) else {
                continue;
            };
            if waveform.y_values.len() == expected_len {
                return Some((waveform.name.as_str(), waveform));
            }
        }
        None
    }

    fn estimate_ui_period(time: &[f64], signal: &[f64]) -> Option<f64> {
        let n = time.len().min(signal.len());
        if n < 8 {
            return None;
        }

        let mut v_min = f64::INFINITY;
        let mut v_max = f64::NEG_INFINITY;
        for &v in signal.iter().take(n) {
            if v.is_finite() {
                v_min = v_min.min(v);
                v_max = v_max.max(v);
            }
        }
        if !v_min.is_finite() || !v_max.is_finite() || (v_max - v_min) <= 0.0 {
            return None;
        }

        let threshold = (v_min + v_max) * 0.5;
        let edges = crate::analysis::eye_diagram::data::find_edges(&time[..n], &signal[..n], threshold);
        if edges.len() < 3 {
            return None;
        }

        let mut rising_times: Vec<f64> = edges
            .iter()
            .filter(|edge| edge.rising)
            .map(|edge| edge.time)
            .collect();
        rising_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let edge_times: Vec<f64> = if rising_times.len() >= 3 {
            rising_times
        } else {
            let mut all: Vec<f64> = edges.iter().map(|edge| edge.time).collect();
            all.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            all
        };
        if edge_times.len() < 3 {
            return None;
        }

        let mut intervals = Vec::with_capacity(edge_times.len().saturating_sub(1));
        for pair in edge_times.windows(2) {
            let dt = pair[1] - pair[0];
            if dt.is_finite() && dt > 0.0 {
                intervals.push(dt);
            }
        }
        if intervals.is_empty() {
            return None;
        }
        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = intervals[intervals.len() / 2];
        (median.is_finite() && median > 0.0).then_some(median)
    }

    fn downsample_for_fft(
        time: &[f64],
        signal: &[f64],
        max_points: usize,
    ) -> Option<(Vec<f64>, f64)> {
        let n = time.len().min(signal.len());
        if n < 16 || max_points < 16 {
            return None;
        }
        let step = (n / max_points).max(1);

        let mut values = Vec::with_capacity((n / step) + 1);
        let mut times = Vec::with_capacity((n / step) + 1);
        for idx in (0..n).step_by(step) {
            let t = time[idx];
            let y = signal[idx];
            if t.is_finite() && y.is_finite() {
                times.push(t);
                values.push(y);
            }
        }
        if values.len() < 16 {
            return None;
        }

        let duration = times[times.len() - 1] - times[0];
        if !duration.is_finite() || duration <= 0.0 {
            return None;
        }
        let sample_rate = (values.len().saturating_sub(1) as f64) / duration;
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return None;
        }
        Some((values, sample_rate))
    }

    fn is_sparameter_trace_name(name: &str) -> bool {
        let normalized = name.trim_matches('|').to_ascii_uppercase();
        if !normalized.starts_with('S') {
            return false;
        }
        normalized[1..]
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .count()
            >= 2
    }

    fn preferred_viewer_for_analysis(
        analysis_type: AnalysisType,
    ) -> crate::viewers::ActiveViewer {
        match analysis_type {
            AnalysisType::DcOp => crate::viewers::ActiveViewer::Waveform,
            AnalysisType::DcSweep | AnalysisType::Transient | AnalysisType::Envelope => {
                crate::viewers::ActiveViewer::Waveform
            }
            AnalysisType::Ac
            | AnalysisType::Disto
            | AnalysisType::Tf
            | AnalysisType::Pac
            | AnalysisType::Pxf => {
                crate::viewers::ActiveViewer::BodePlot
            }
            AnalysisType::Noise | AnalysisType::Pnoise => crate::viewers::ActiveViewer::BodePlot,
            AnalysisType::PoleZero => crate::viewers::ActiveViewer::PoleZero,
            AnalysisType::Sensitivity => crate::viewers::ActiveViewer::Waveform,
            AnalysisType::Pstb | AnalysisType::Stb => crate::viewers::ActiveViewer::Nyquist,
            AnalysisType::MonteCarlo | AnalysisType::Corner | AnalysisType::Parametric => {
                crate::viewers::ActiveViewer::Histogram
            }
            AnalysisType::Reliability | AnalysisType::Optimization | AnalysisType::Soa => {
                crate::viewers::ActiveViewer::Waveform
            }
            AnalysisType::SParameter => crate::viewers::ActiveViewer::SmithChart,
            AnalysisType::Fourier => crate::viewers::ActiveViewer::Fft,
            AnalysisType::HarmonicBalance | AnalysisType::Pss => {
                crate::viewers::ActiveViewer::Waveform
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
    fn test_finish_simulation_batch_reports_failed_run_status() {
        let mut controller = SimulationController::new();
        let mut state = AppState::default();
        state.simulation.start_run().success = false;

        controller.finish_simulation_batch(&mut state);

        assert_eq!(state.simulation.status, "Completed with errors");
    }

    #[test]
    fn test_start_next_analysis_without_cached_netlist_reports_error_instead_of_panicking() {
        let mut controller = SimulationController::new();
        let mut state = AppState::default();
        state.simulation.start_run();

        controller.total_analyses = 1;
        controller.pending_analyses.push_back(QueuedAnalysis {
            spec: AnalysisSpec::DcOp,
            config: Some(AnalysisConfig::DcOp),
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".OP".to_string(),
        });
        controller.cached_netlist = None;

        controller.start_next_analysis(&mut state);

        assert_eq!(state.simulation.status, "Error");
        assert!(controller.pending_analyses.is_empty());
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("missing cached netlist")),
            "expected a user-visible missing-netlist error message"
        );
        assert!(
            state
                .simulation
                .active_run()
                .map(|run| !run.success)
                .unwrap_or(false),
            "active run should be marked failed"
        );
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
    fn test_apply_simulation_options_to_netlist_skips_default_options_block() {
        let netlist = "* test\nV1 in 0 dc 1\n.op\n.end\n";
        let opts = crate::simulation::dialog::SimulationOptions::default();
        let merged = SimulationController::apply_simulation_options_to_netlist(netlist, &opts);
        assert_eq!(merged, netlist);
    }

    #[test]
    fn test_apply_simulation_options_to_netlist_inserts_options_before_end() {
        let netlist = "* test\nV1 in 0 dc 1\n.op\n.end\n";
        let mut opts = crate::simulation::dialog::SimulationOptions::default();
        opts.reltol = 2e-4;
        opts.temp = 85.0;

        let merged = SimulationController::apply_simulation_options_to_netlist(netlist, &opts);
        assert!(merged.contains(".OPTIONS"));
        assert!(merged.contains("RELTOL=2.00e-4"));
        assert!(merged.contains("TEMP=85.00"));

        let options_pos = merged
            .find(".OPTIONS")
            .expect("options block should be present");
        let end_pos = merged.rfind(".end").expect(".end should still be present");
        assert!(options_pos < end_pos, "options block must precede .end");
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
    fn test_enabled_analysis_indices_defaults_to_active_tab_when_none_enabled() {
        let mut state = AppState::default();
        state.dialogs.sim_active_tab = 2;
        let indices = SimulationController::enabled_analysis_indices(&state);
        assert_eq!(indices, vec![2]);
    }

    #[test]
    fn test_build_analysis_plan_rejects_unimplemented_analysis_tab() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(99);

        let errors = controller
            .build_analysis_plan(&state)
            .expect_err("unsupported analysis should fail planning");
        assert!(errors.iter().any(|e| e.contains("Unknown")));
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
    fn test_build_analysis_plan_accepts_nested_dc_sweep() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(3);
        state.dialogs.dc_nested = true;
        state.dialogs.dc_source = "V1".to_string();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1".to_string();
        state.dialogs.dc_step = "0.1".to_string();
        state.dialogs.dc_source2 = "V2".to_string();
        state.dialogs.dc_start2 = "0".to_string();
        state.dialogs.dc_stop2 = "2".to_string();
        state.dialogs.dc_step2 = "0.5".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("nested sweep should build a valid plan");
        assert_eq!(plan.analyses.len(), 1);
        match &plan.analyses[0] {
            AnalysisSpec::DcSweep {
                source_name,
                source2,
                start2,
                stop2,
                step2,
                ..
            } => {
                assert_eq!(source_name, "V1");
                assert_eq!(source2.as_deref(), Some("V2"));
                assert_eq!(*start2, Some(0.0));
                assert_eq!(*stop2, Some(2.0));
                assert_eq!(*step2, Some(0.5));
            }
            other => panic!("expected DC sweep spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_plan_rejects_nested_dc_without_secondary_source() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(3);
        state.dialogs.dc_nested = true;
        state.dialogs.dc_source = "V1".to_string();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1".to_string();
        state.dialogs.dc_step = "0.1".to_string();
        state.dialogs.dc_source2.clear();
        state.dialogs.dc_start2 = "0".to_string();
        state.dialogs.dc_stop2 = "2".to_string();
        state.dialogs.dc_step2 = "0.5".to_string();

        let errors = controller
            .build_analysis_plan(&state)
            .expect_err("nested sweep with missing source2 should fail");
        assert!(errors.iter().any(|e| e.contains("secondary sweep source")));
    }

    #[test]
    fn test_build_queue_from_plan_maps_nested_dc_config() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(3);
        state.dialogs.dc_nested = true;
        state.dialogs.dc_source = "V1".to_string();
        state.dialogs.dc_start = "0".to_string();
        state.dialogs.dc_stop = "1".to_string();
        state.dialogs.dc_step = "0.1".to_string();
        state.dialogs.dc_source2 = "V2".to_string();
        state.dialogs.dc_start2 = "0".to_string();
        state.dialogs.dc_stop2 = "2".to_string();
        state.dialogs.dc_step2 = "0.5".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");
        assert_eq!(queue.len(), 1);
        match &queue[0].config {
            Some(AnalysisConfig::DcSweep(dc)) => {
                assert_eq!(dc.source, "V1");
                assert_eq!(dc.source2.as_deref(), Some("V2"));
                assert_eq!(dc.start2, Some(0.0));
                assert_eq!(dc.stop2, Some(2.0));
                assert_eq!(dc.step2, Some(0.5));
            }
            other => panic!("expected nested DC config, got {:?}", other),
        }
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
    fn test_build_analysis_spec_for_pss_uses_dialog_configuration() {
        use crate::simulation::dialog::pss::PssConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.pss_state = crate::simulation::dialog::pss::PssDialogState::from_config(
            &PssConfig::new(2.5e6).with_harmonics(15),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 8)
            .expect("PSS spec should build");
        match spec {
            AnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => {
                assert!((fundamental_freq - 2.5e6).abs() < 1e-6);
                assert_eq!(num_harmonics, 15);
                assert!((tolerance - 1e-3).abs() < 1e-15);
            }
            other => panic!("expected PSS spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_stb_uses_dialog_configuration() {
        use crate::simulation::dialog::stb::StbConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.stb_state = crate::simulation::dialog::stb::StbDialogState::from_config(
            &StbConfig::new("L1")
                .with_freq_range(10.0, 1e6)
                .with_points(12),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 9)
            .expect("STB spec should build");
        match spec {
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
            } => {
                assert_eq!(probe_node, "L1");
                assert!((start_freq - 10.0).abs() < 1e-12);
                assert!((stop_freq - 1e6).abs() < 1e-3);
                assert_eq!(points_per_decade, 12);
            }
            other => panic!("expected STB spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_harmonic_balance_uses_dialog_configuration() {
        use crate::simulation::dialog::hb::{HbConfig, HbSolverType, HbToneConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.hb_state = crate::simulation::dialog::hb::HbDialogState::from_config(
            &HbConfig::new(1.2e9, 11)
                .add_tone(HbToneConfig::new(900e6, 5))
                .with_solver(HbSolverType::Krylov)
                .with_oversample(6)
                .with_tolerance(2e-6)
                .with_source_stepping(true),
        );
        state.dialogs.hb_state.maxiter = "175".to_string();
        state.dialogs.hb_state.damping = "0.6".to_string();

        let spec = controller
            .build_analysis_spec_for_index(&state, 11)
            .expect("HB spec should build");
        match spec {
            AnalysisSpec::HarmonicBalance {
                tone1_freq,
                tone1_harmonics,
                tone2_freq,
                tone2_harmonics,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            } => {
                assert!((tone1_freq - 1.2e9).abs() < 1e-3);
                assert_eq!(tone1_harmonics, 11);
                assert_eq!(tone2_freq, Some(900e6));
                assert_eq!(tone2_harmonics, 5);
                assert!((reltol - 2e-6).abs() < 1e-18);
                assert!((abstol - 1e-12).abs() < 1e-24);
                assert_eq!(max_iterations, 175);
                assert!((damping - 0.6).abs() < 1e-15);
                assert_eq!(oversample, 6);
                assert_eq!(max_mixing_order, 5);
                assert!(use_krylov);
                assert_eq!(gmres_restart, 30);
                assert!(source_stepping);
                assert!(!verbose);
            }
            other => panic!("expected harmonic balance spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_sparameter_uses_dialog_configuration() {
        use crate::simulation::dialog::sp::{SpConfig, SpPortConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.sp_state = crate::simulation::dialog::sp::SpDialogState::from_config(
            &SpConfig::decade(1e6, 2e9, 20)
                .with_z0(75.0)
                .with_ports(vec![
                    SpPortConfig::single_ended(1, "rf_in"),
                    SpPortConfig::single_ended(2, "rf_out"),
                ]),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 12)
            .expect("S-parameter spec should build");
        match spec {
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            } => {
                assert!((start_freq - 1e6).abs() < 1e-6);
                assert!((stop_freq - 2e9).abs() < 1e-3);
                assert_eq!(points_per_unit, 20);
                assert!(matches!(sweep, FrequencySweep::Decade));
                assert!((z0 - 75.0).abs() < 1e-9);
                assert_eq!(ports.len(), 2);
                assert_eq!(ports[0].node_pos, "RF_IN");
                assert_eq!(ports[0].node_neg, "0");
                assert_eq!(ports[1].node_pos, "RF_OUT");
                assert_eq!(ports[1].node_neg, "0");
            }
            other => panic!("expected S-parameter spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_envelope_uses_dialog_configuration() {
        use crate::simulation::dialog::envelope::EnvelopeConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.envelope_state =
            crate::simulation::dialog::envelope::EnvelopeDialogState::from_config(
                &EnvelopeConfig::new(5e9, 2e-6).with_harmonics(13),
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 19)
            .expect("Envelope spec should build");
        match spec {
            AnalysisSpec::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            } => {
                assert!((fundamental_freq - 5e9).abs() < 1e-3);
                assert!((stop_time - 2e-6).abs() < 1e-15);
                assert_eq!(num_harmonics, 13);
                assert_eq!(max_step, None);
            }
            other => panic!("expected Envelope spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_fourier_uses_dialog_configuration() {
        use crate::simulation::dialog::fourier::FourierConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.fourier_state =
            crate::simulation::dialog::fourier::FourierDialogState::from_config(
                &FourierConfig::new(2e6, 15)
                    .with_output("outp")
                    .with_window(1e-6, 11e-6),
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 20)
            .expect("Fourier spec should build");
        match spec {
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            } => {
                assert!((fundamental_freq - 2e6).abs() < 1e-6);
                assert_eq!(num_harmonics, 15);
                assert_eq!(output_node, "OUTP");
                assert_eq!(output_ref, "");
                assert!((start_time - 1e-6).abs() < 1e-15);
                assert!((stop_time - 11e-6).abs() < 1e-15);
            }
            other => panic!("expected Fourier spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_reliability_uses_dialog_configuration() {
        use crate::simulation::dialog::reliability::ReliabilityConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.reliability_state =
            crate::simulation::dialog::reliability::ReliabilityDialogState::from_config(
                &ReliabilityConfig {
                    target_years: vec![2.0, 7.0, 15.0],
                    enable_hci: true,
                    enable_nbti: false,
                    enable_em: true,
                    min_stress_voltage: 0.2,
                },
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 21)
            .expect("Reliability spec should build");
        match spec {
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => {
                assert_eq!(target_years, vec![2.0, 7.0, 15.0]);
                assert!(enable_hci);
                assert!(!enable_nbti);
                assert!(enable_em);
                assert!((min_stress_voltage - 0.2).abs() < 1e-12);
            }
            other => panic!("expected Reliability spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_optimization_uses_dialog_configuration() {
        use crate::simulation::dialog::optimization::{
            OptimizationAlgorithmMode, OptimizationConfig, OptimizationGoalMode,
            OptimizationVariableConfig,
        };

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.optimization_state =
            crate::simulation::dialog::optimization::OptimizationDialogState::from_config(
                &OptimizationConfig {
                    variables: vec![OptimizationVariableConfig {
                        name: "RLOAD".to_string(),
                        min: 500.0,
                        max: 5_000.0,
                        initial: 1_000.0,
                    }],
                    objective_node: "out".to_string(),
                    objective_ref: "0".to_string(),
                    goal_mode: OptimizationGoalMode::Target,
                    target_value: Some(1.1),
                    algorithm: OptimizationAlgorithmMode::PatternSearch,
                    max_iterations: 80,
                    cost_tolerance: 1e-9,
                    fd_step: 1e-4,
                    initial_step: 0.2,
                    min_step: 1e-8,
                },
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 22)
            .expect("Optimization spec should build");
        match spec {
            AnalysisSpec::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                ..
            } => {
                assert_eq!(variables.len(), 1);
                assert_eq!(variables[0].name, "RLOAD");
                assert_eq!(objective_node, "out");
                assert_eq!(objective_ref, "0");
                assert!(matches!(goal, OptimizationGoal::Target));
                assert_eq!(target, Some(1.1));
                assert!(matches!(algorithm, OptimizationAlgorithm::PatternSearch));
                assert_eq!(max_iterations, 80);
                assert!((cost_tolerance - 1e-9).abs() < 1e-18);
            }
            other => panic!("expected Optimization spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_soa_uses_dialog_configuration() {
        use crate::simulation::dialog::soa::SoaConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.soa_state =
            crate::simulation::dialog::soa::SoaDialogState::from_config(&SoaConfig {
                stop_time: 2e-6,
                step_time: 5e-9,
                check_vgs_max: true,
                max_vgs: 1.6,
                check_vds_max: false,
                max_vds: 3.3,
                check_vbe_max: true,
                max_vbe: 0.8,
                check_vce_max: false,
                max_vce: 5.0,
            });

        let spec = controller
            .build_analysis_spec_for_index(&state, 23)
            .expect("SOA spec should build");
        match spec {
            AnalysisSpec::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                ..
            } => {
                assert!((stop_time - 2e-6).abs() < 1e-15);
                assert!((step_time - 5e-9).abs() < 1e-18);
                assert!(check_vgs_max);
                assert!((max_vgs - 1.6).abs() < 1e-12);
                assert!(!check_vds_max);
                assert!(check_vbe_max);
                assert!((max_vbe - 0.8).abs() < 1e-12);
                assert!(!check_vce_max);
            }
            other => panic!("expected SOA spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_analysis_spec_for_disto_uses_dialog_configuration() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.ac_fstart = "10".to_string();
        state.dialogs.ac_fstop = "10Meg".to_string();
        state.dialogs.ac_points = "12".to_string();
        state.dialogs.ac_sweep_type = 1; // octave
        state.dialogs.disto_f2_over_f1 = "1.75".to_string();

        let spec = controller
            .build_analysis_spec_for_index(&state, 24)
            .expect("DISTO spec should build");
        match spec {
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => {
                assert!((start_freq - 10.0).abs() < 1e-12);
                assert!((stop_freq - 10e6).abs() < 1e-6);
                assert_eq!(points_per_unit, 12);
                assert!(matches!(sweep, FrequencySweep::Octave));
                assert_eq!(f2_over_f1, Some(1.75));
            }
            other => panic!("expected DISTO spec, got {:?}", other),
        }
    }

    #[test]
    fn test_build_queue_from_plan_uses_executable_optimization_command() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(22);

        let plan = controller
            .build_analysis_plan(&state)
            .expect("optimization plan should build");
        assert_eq!(plan.analyses.len(), 1);

        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("optimization queue should build");
        assert_eq!(queue.len(), 1);
        assert!(
            queue[0].analysis_line.starts_with(".opt "),
            "optimization command must be emitted as executable SPICE, got: {}",
            queue[0].analysis_line
        );
        assert!(
            !queue[0].analysis_line.trim_start().starts_with('*'),
            "optimization command must not be commented out"
        );
    }

    #[test]
    fn test_build_queue_from_plan_uses_executable_soa_command() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(23);

        let plan = controller
            .build_analysis_plan(&state)
            .expect("soa plan should build");
        assert_eq!(plan.analyses.len(), 1);

        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("soa queue should build");
        assert_eq!(queue.len(), 1);
        assert!(
            queue[0].analysis_line.starts_with(".soa "),
            "soa command must be emitted as executable SPICE, got: {}",
            queue[0].analysis_line
        );
        assert!(
            !queue[0].analysis_line.trim_start().starts_with('*'),
            "soa command must not be commented out"
        );
    }

    #[test]
    fn test_build_queue_from_plan_routes_disto_via_spec_with_disto_command_line() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(24);
        state.dialogs.ac_fstart = "1".to_string();
        state.dialogs.ac_fstop = "1Meg".to_string();
        state.dialogs.ac_points = "10".to_string();
        state.dialogs.disto_f2_over_f1 = "1.5".to_string();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("disto plan should build");
        assert_eq!(plan.analyses.len(), 1);
        assert!(matches!(plan.analyses[0], AnalysisSpec::Disto { .. }));

        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("disto queue should build");
        assert_eq!(queue.len(), 1);
        assert!(queue[0].config.is_none(), "DISTO should execute via spec path");
        assert!(
            queue[0].analysis_line.starts_with(".disto "),
            "DISTO command should emit native DISTO command, got: {}",
            queue[0].analysis_line
        );
        assert!(
            queue[0].analysis_line.contains(" 1.5"),
            "DISTO command should include optional f2/f1 ratio when set, got: {}",
            queue[0].analysis_line
        );
    }

    #[test]
    fn test_build_queue_from_plan_routes_disto_without_optional_ratio() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(24);
        state.dialogs.ac_fstart = "10".to_string();
        state.dialogs.ac_fstop = "10Meg".to_string();
        state.dialogs.ac_points = "8".to_string();
        state.dialogs.disto_f2_over_f1 = String::new();

        let plan = controller
            .build_analysis_plan(&state)
            .expect("disto plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("disto queue should build");
        assert_eq!(queue.len(), 1);
        assert!(queue[0].analysis_line.starts_with(".disto "));
        assert_eq!(queue[0].analysis_line.split_whitespace().count(), 5);
    }

    #[test]
    fn test_build_analysis_spec_for_pac_accepts_valid_dialog_configuration() {
        use crate::simulation::dialog::pac::PacConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.pac_state = crate::simulation::dialog::pac::PacDialogState::from_config(
            &PacConfig::new(10e3, 5e6, 12)
                .with_input("V1")
                .with_output("OUT")
                .with_sidebands(3),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 13)
            .expect("PAC spec should build");
        assert!(matches!(spec, AnalysisSpec::Pac));
    }

    #[test]
    fn test_build_analysis_spec_for_pnoise_accepts_valid_dialog_configuration() {
        use crate::simulation::dialog::pnoise::{NoiseReferenceType, PnoiseConfig};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.pnoise_state =
            crate::simulation::dialog::pnoise::PnoiseDialogState::from_config(
                &PnoiseConfig::new(10.0, 10e6, 12)
                    .with_output("OUT")
                    .with_sidebands(3)
                    .with_noise_ref(NoiseReferenceType::Phase),
            );

        let spec = controller
            .build_analysis_spec_for_index(&state, 14)
            .expect("PNOISE spec should build");
        assert!(matches!(spec, AnalysisSpec::Pnoise));
    }

    #[test]
    fn test_build_analysis_spec_for_pxf_accepts_valid_dialog_configuration() {
        use crate::simulation::dialog::pxf::PxfConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.pxf_state = crate::simulation::dialog::pxf::PxfDialogState::from_config(
            &PxfConfig::new(10.0, 10e6, 12)
                .with_input("V1")
                .with_output("OUT", 1)
                .with_sidebands(3),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 15)
            .expect("PXF spec should build");
        assert!(matches!(spec, AnalysisSpec::Pxf));
    }

    #[test]
    fn test_build_analysis_spec_for_pstb_accepts_valid_dialog_configuration() {
        use crate::simulation::dialog::pstb::PstbConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.pstb_state = crate::simulation::dialog::pstb::PstbDialogState::from_config(
            &PstbConfig::new("lprobe")
                .with_harmonics(12)
                .with_multipliers(6)
                .with_annotate(false),
        );

        let spec = controller
            .build_analysis_spec_for_index(&state, 16)
            .expect("PSTB spec should build");
        assert!(matches!(spec, AnalysisSpec::Pstb));
    }

    #[test]
    fn test_build_analysis_spec_for_tf_accepts_valid_dialog_configuration() {
        use crate::simulation::dialog::xf::{XfConfig, XfSweepType};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        let mut cfg = XfConfig::new(1e3, 1e9, 20)
            .with_input("V1")
            .with_output("OUT")
            .with_group_delay(true);
        cfg.sweep_type = XfSweepType::Octave;
        cfg.input_impedance = true;
        cfg.output_impedance = true;
        state.dialogs.xf_state = crate::simulation::dialog::xf::XfDialogState::from_config(&cfg);

        let spec = controller
            .build_analysis_spec_for_index(&state, 17)
            .expect("TF spec should build");
        assert!(matches!(spec, AnalysisSpec::Tf));
    }

    #[test]
    fn test_build_queue_from_plan_emits_worst_case_monte_carlo_command() {
        use crate::simulation::dialog::mc::{McConfig, McDistribution};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses.insert(7);
        state.dialogs.mc_state = crate::simulation::dialog::mc::McDialogState::from_config(
            &McConfig::new(16)
                .with_distribution(McDistribution::WorstCase)
                .with_seed(9),
        );

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::MonteCarlo));
        assert!(
            queue[0].analysis_line.contains("DIST WORSTCASE"),
            "expected WORSTCASE distribution in .MC command"
        );
    }

    #[test]
    fn test_build_queue_from_plan_stores_pss_and_hb_as_spec_executed_runs() {
        use crate::simulation::dialog::hb::HbConfig;
        use crate::simulation::dialog::pac::PacConfig;
        use crate::simulation::dialog::pss::PssConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [8usize, 11usize, 13usize].into_iter().collect();
        state.dialogs.pss_state =
            crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(10e6));
        state.dialogs.hb_state =
            crate::simulation::dialog::hb::HbDialogState::from_config(&HbConfig::new(2.4e9, 9));
        state.dialogs.pac_state = crate::simulation::dialog::pac::PacDialogState::from_config(
            &PacConfig::new(1e3, 1e6, 8)
                .with_input("V1")
                .with_output("OUT")
                .with_sidebands(2),
        );
        state.dialogs.simulation_options_config.reltol = 2e-4;
        state.dialogs.simulation_options_config.abstol = 3e-11;

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 3);
        assert!(matches!(queue[0].spec, AnalysisSpec::Pss { .. }));
        assert!(queue[0].config.is_none());
        assert!(queue[0].analysis_line.starts_with(".pss "));

        assert!(matches!(
            queue[1].spec,
            AnalysisSpec::HarmonicBalance { .. }
        ));
        assert!(queue[1].config.is_none());
        assert!(queue[1].analysis_line.starts_with(".hb "));

        assert!(matches!(queue[2].spec, AnalysisSpec::Pac));
        assert!(queue[2].config.is_none());
        assert!(queue[2].analysis_line.starts_with(".pac "));
        assert!(queue[2].spec_options.pac.is_some());
        assert!(matches!(
            queue[2]
                .spec_options
                .pac
                .as_ref()
                .expect("PAC options should be present")
                .sweep,
            crate::services::simulation_runner::PacFrequencySweep::Decade
        ));
        let pac_cfg = queue[2]
            .spec_options
            .pac
            .as_ref()
            .expect("PAC options should be present");
        assert!((pac_cfg.reltol - 2e-4).abs() < 1e-18);
        assert!((pac_cfg.abstol - 3e-11).abs() < 1e-22);
    }

    #[test]
    fn test_build_queue_from_plan_stores_stb_as_spec_executed_run() {
        use crate::simulation::dialog::stb::StbConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [9usize].into_iter().collect();
        state.dialogs.stb_state = crate::simulation::dialog::stb::StbDialogState::from_config(
            &StbConfig::new("L1")
                .with_freq_range(1.0, 1e6)
                .with_points(16),
        );

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Stb { .. }));
        assert!(queue[0].config.is_none());
        assert!(queue[0].analysis_line.starts_with(".stb "));
        assert!(queue[0].spec_options.pac.is_none());
        assert!(queue[0].spec_options.pxf.is_none());
        assert!(queue[0].spec_options.tf.is_none());
        assert!(queue[0].spec_options.pnoise.is_none());
    }

    #[test]
    fn test_build_queue_from_plan_stores_pxf_as_spec_executed_run() {
        use crate::simulation::dialog::pss::PssConfig;
        use crate::simulation::dialog::pxf::PxfConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [15usize].into_iter().collect();
        state.dialogs.pss_state =
            crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(5e6));
        state.dialogs.pxf_state = crate::simulation::dialog::pxf::PxfDialogState::from_config(
            &PxfConfig::new(1e3, 1e6, 10)
                .with_input("V1")
                .with_output("OUT", 1)
                .with_sidebands(3),
        );
        state.dialogs.simulation_options_config.reltol = 7e-4;
        state.dialogs.simulation_options_config.abstol = 4e-12;

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Pxf));
        assert!(queue[0].config.is_none());
        assert!(queue[0].analysis_line.starts_with(".pxf "));
        let pxf_cfg = queue[0]
            .spec_options
            .pxf
            .as_ref()
            .expect("PXF options should be present");
        assert_eq!(pxf_cfg.input_source, "V1");
        assert_eq!(pxf_cfg.output_sideband, 1);
        assert_eq!(pxf_cfg.max_sideband, 3);
        assert!(matches!(
            pxf_cfg.sweep,
            crate::services::simulation_runner::PxfFrequencySweep::Decade
        ));
        assert!((pxf_cfg.reltol - 7e-4).abs() < 1e-18);
        assert!((pxf_cfg.abstol - 4e-12).abs() < 1e-24);
    }

    #[test]
    fn test_build_queue_from_plan_stores_pstb_as_spec_executed_run() {
        use crate::simulation::dialog::pss::PssConfig;
        use crate::simulation::dialog::pstb::PstbConfig;

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [16usize].into_iter().collect();
        state.dialogs.pss_state =
            crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(6e6));
        state.dialogs.pstb_state = crate::simulation::dialog::pstb::PstbDialogState::from_config(
            &PstbConfig::new("LPROBE")
                .with_harmonics(12)
                .with_multipliers(4),
        );

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Pstb));
        assert!(queue[0].config.is_none());
        assert!(queue[0].analysis_line.starts_with(".pstb "));

        let pstb_cfg = queue[0]
            .spec_options
            .pstb
            .as_ref()
            .expect("PSTB options should be present");
        assert_eq!(pstb_cfg.probe_instance, "LPROBE");
        assert_eq!(pstb_cfg.max_harmonics, 12);
        assert_eq!(pstb_cfg.num_multipliers, 4);
        assert!((pstb_cfg.pss_fundamental_freq - 6e6).abs() < 1e-6);
    }

    #[test]
    fn test_build_queue_from_plan_stores_tf_and_pnoise_as_spec_executed_runs() {
        use crate::simulation::dialog::pnoise::{NoiseReferenceType, PnoiseConfig};
        use crate::simulation::dialog::pss::PssConfig;
        use crate::simulation::dialog::xf::{XfConfig, XfSweepType};

        let controller = SimulationController::new();
        let mut state = AppState::default();
        state.dialogs.enabled_analyses = [14usize, 17usize].into_iter().collect();

        state.dialogs.pss_state =
            crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(5e6));
        state.dialogs.pnoise_state =
            crate::simulation::dialog::pnoise::PnoiseDialogState::from_config(
                &PnoiseConfig::new(10.0, 10e6, 10)
                    .with_output("OUT")
                    .with_input("V1")
                    .with_sidebands(2)
                    .with_noise_ref(NoiseReferenceType::Phase),
            );
        state.dialogs.simulation_options_config.reltol = 9e-4;
        state.dialogs.simulation_options_config.abstol = 6e-13;

        let mut xf_cfg = XfConfig::new(1e3, 1e8, 8)
            .with_input("V1")
            .with_output("OUT")
            .with_group_delay(true);
        xf_cfg.sweep_type = XfSweepType::Linear;
        xf_cfg.input_impedance = true;
        xf_cfg.output_impedance = true;
        state.dialogs.xf_state = crate::simulation::dialog::xf::XfDialogState::from_config(&xf_cfg);

        let plan = controller
            .build_analysis_plan(&state)
            .expect("plan should build");
        let queue = controller
            .build_queue_from_plan(&state, &plan)
            .expect("queue should build");

        assert_eq!(queue.len(), 2);
        assert!(matches!(queue[0].spec, AnalysisSpec::Pnoise));
        assert!(queue[0].config.is_none());
        assert!(queue[0].analysis_line.starts_with(".pnoise "));
        assert!(queue[0].spec_options.pnoise.is_some());
        let pnoise_cfg = queue[0]
            .spec_options
            .pnoise
            .as_ref()
            .expect("PNOISE options should be present");
        assert!(matches!(
            pnoise_cfg.noise_ref,
            crate::services::simulation_runner::PnoiseReference::Phase
        ));
        assert_eq!(pnoise_cfg.input_source, "V1");
        assert!((pnoise_cfg.reltol - 9e-4).abs() < 1e-18);
        assert!((pnoise_cfg.abstol - 6e-13).abs() < 1e-25);

        assert!(matches!(queue[1].spec, AnalysisSpec::Tf));
        assert!(queue[1].config.is_none());
        assert!(queue[1].analysis_line.starts_with(".xf "));
        assert!(queue[1].spec_options.tf.is_some());
        let tf_cfg = queue[1]
            .spec_options
            .tf
            .as_ref()
            .expect("TF options should be present");
        assert!(tf_cfg.group_delay);
        assert!(tf_cfg.input_impedance);
        assert!(tf_cfg.output_impedance);
        assert!(matches!(
            tf_cfg.sweep,
            crate::services::simulation_runner::TfFrequencySweep::Linear
        ));
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

    #[test]
    fn test_spec_to_analysis_type_preserves_advanced_categories() {
        let controller = SimulationController::new();
        let cases = [
            (AnalysisSpec::Tf, crate::state::AnalysisType::Tf),
            (
                AnalysisSpec::Disto {
                    start_freq: 1e3,
                    stop_freq: 1e6,
                    points_per_unit: 10,
                    sweep: FrequencySweep::Decade,
                    f2_over_f1: Some(1.5),
                },
                crate::state::AnalysisType::Disto,
            ),
            (AnalysisSpec::Pac, crate::state::AnalysisType::Pac),
            (AnalysisSpec::Pnoise, crate::state::AnalysisType::Pnoise),
            (AnalysisSpec::Pxf, crate::state::AnalysisType::Pxf),
            (AnalysisSpec::Pstb, crate::state::AnalysisType::Pstb),
            (
                AnalysisSpec::Stb {
                    probe_node: "L1".to_string(),
                    start_freq: 1.0,
                    stop_freq: 1e6,
                    points_per_decade: 10,
                },
                crate::state::AnalysisType::Stb,
            ),
            (
                AnalysisSpec::Reliability {
                    target_years: vec![1.0, 5.0],
                    enable_hci: true,
                    enable_nbti: false,
                    enable_em: false,
                    min_stress_voltage: 0.05,
                },
                crate::state::AnalysisType::Reliability,
            ),
            (
                AnalysisSpec::Optimization {
                    variables: vec![OptimizationVariable {
                        name: "X".to_string(),
                        min: 0.0,
                        max: 1.0,
                        initial: 0.5,
                    }],
                    objective_node: "out".to_string(),
                    objective_ref: "0".to_string(),
                    goal: OptimizationGoal::Minimize,
                    target: None,
                    algorithm: OptimizationAlgorithm::PatternSearch,
                    max_iterations: 10,
                    cost_tolerance: 1e-6,
                    fd_step: 1e-3,
                    initial_step: 0.1,
                    min_step: 1e-5,
                },
                crate::state::AnalysisType::Optimization,
            ),
            (
                AnalysisSpec::Soa {
                    stop_time: 1e-6,
                    step_time: 1e-9,
                    check_vgs_max: true,
                    max_vgs: 1.2,
                    check_vds_max: true,
                    max_vds: 3.3,
                    check_vbe_max: false,
                    max_vbe: 0.9,
                    check_vce_max: false,
                    max_vce: 5.0,
                },
                crate::state::AnalysisType::Soa,
            ),
            (
                AnalysisSpec::SParameter {
                    start_freq: 1e6,
                    stop_freq: 1e9,
                    points_per_unit: 10,
                    sweep: FrequencySweep::Decade,
                    z0: 50.0,
                    ports: vec![
                        SpPort {
                            node_pos: "in".to_string(),
                            node_neg: "0".to_string(),
                            z0: None,
                        },
                        SpPort {
                            node_pos: "out".to_string(),
                            node_neg: "0".to_string(),
                            z0: Some(60.0),
                        },
                    ],
                },
                crate::state::AnalysisType::SParameter,
            ),
            (
                AnalysisSpec::Envelope {
                    fundamental_freq: 1e9,
                    stop_time: 1e-6,
                    num_harmonics: 9,
                    max_step: None,
                },
                crate::state::AnalysisType::Envelope,
            ),
            (
                AnalysisSpec::Fourier {
                    fundamental_freq: 1e6,
                    num_harmonics: 11,
                    output_node: "out".to_string(),
                    output_ref: "0".to_string(),
                    start_time: 0.0,
                    stop_time: 10e-6,
                },
                crate::state::AnalysisType::Fourier,
            ),
        ];

        for (spec, expected) in cases {
            assert_eq!(
                controller.spec_to_analysis_type(&spec),
                expected,
                "unexpected analysis type mapping for {:?}",
                spec.run_type()
            );
        }
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
    fn test_convert_monte_carlo_result() {
        use crate::simulation::results::MonteCarloVariableResult;
        use crate::simulation::SimulationResult;

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
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
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
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
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

    #[test]
    fn test_build_touchstone_dataset_from_sparameter_ac_result() {
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
        use std::collections::HashMap;

        let freqs = vec![1e6, 2e6];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "S11".to_string(),
            EngineWaveformData::new_complex("S11", freqs.clone(), vec![0.1, 0.2], vec![0.01, 0.02]),
        );
        waveforms.insert(
            "S21".to_string(),
            EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9, 0.8], vec![0.0, -0.1]),
        );
        waveforms.insert(
            "S12".to_string(),
            EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02, 0.03], vec![0.0, 0.0]),
        );
        waveforms.insert(
            "S22".to_string(),
            EngineWaveformData::new_complex(
                "S22",
                freqs.clone(),
                vec![0.2, 0.3],
                vec![-0.01, -0.02],
            ),
        );

        let result = SimulationResult::Ac {
            frequencies: freqs.clone(),
            waveforms,
        };
        let dataset =
            SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 50.0], 2)
                .expect("touchstone dataset should build");

        assert_eq!(dataset.point_count(), 2);
        assert_eq!(dataset.signal_count(), 8);
        assert_eq!(
            dataset
                .metadata
                .get("num_ports")
                .cloned()
                .unwrap_or_default(),
            "2"
        );
        assert_eq!(
            dataset
                .metadata
                .get("touchstone_version")
                .cloned()
                .unwrap_or_default(),
            "2"
        );
    }

    #[test]
    fn test_build_touchstone_dataset_records_per_port_reference_metadata() {
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
        use std::collections::HashMap;

        let freqs = vec![1e6];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "S11".to_string(),
            EngineWaveformData::new_complex("S11", freqs.clone(), vec![0.1], vec![0.0]),
        );
        waveforms.insert(
            "S21".to_string(),
            EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9], vec![0.0]),
        );
        waveforms.insert(
            "S12".to_string(),
            EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02], vec![0.0]),
        );
        waveforms.insert(
            "S22".to_string(),
            EngineWaveformData::new_complex("S22", freqs.clone(), vec![0.2], vec![0.0]),
        );

        let result = SimulationResult::Ac {
            frequencies: freqs,
            waveforms,
        };
        let dataset =
            SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 75.0], 2)
                .expect("touchstone dataset should include per-port z0");
        assert_eq!(
            dataset.metadata.get("z0_ports").map(String::as_str),
            Some("50,75")
        );
    }

    #[test]
    fn test_build_touchstone_dataset_rejects_non_uniform_reference_for_v1() {
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
        use std::collections::HashMap;

        let freqs = vec![1e6];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            "S11".to_string(),
            EngineWaveformData::new_complex("S11", freqs.clone(), vec![0.1], vec![0.0]),
        );
        waveforms.insert(
            "S21".to_string(),
            EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9], vec![0.0]),
        );
        waveforms.insert(
            "S12".to_string(),
            EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02], vec![0.0]),
        );
        waveforms.insert(
            "S22".to_string(),
            EngineWaveformData::new_complex("S22", freqs.clone(), vec![0.2], vec![0.0]),
        );

        let result = SimulationResult::Ac {
            frequencies: freqs,
            waveforms,
        };
        let err = SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 75.0], 1)
            .expect_err("touchstone v1 must reject non-uniform z0");
        assert!(err.contains("v1 export does not support per-port reference impedance"));
    }

    #[test]
    fn test_build_touchstone_dataset_requires_complex_components() {
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
        use std::collections::HashMap;

        let freqs = vec![1e6, 2e6];
        let mut waveforms = HashMap::new();
        // Real-only S11 should fail conversion.
        waveforms.insert(
            "S11".to_string(),
            EngineWaveformData::new_freq_domain("S11", freqs.clone(), vec![0.1, 0.2]),
        );
        waveforms.insert(
            "S21".to_string(),
            EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9, 0.8], vec![0.0, 0.0]),
        );
        waveforms.insert(
            "S12".to_string(),
            EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02, 0.03], vec![0.0, 0.0]),
        );
        waveforms.insert(
            "S22".to_string(),
            EngineWaveformData::new_complex(
                "S22",
                freqs.clone(),
                vec![0.2, 0.3],
                vec![-0.01, -0.02],
            ),
        );

        let result = SimulationResult::Ac {
            frequencies: freqs,
            waveforms,
        };
        let err = SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 50.0], 1)
            .expect_err("missing imag should fail");
        assert!(err.contains("missing imaginary component"));
    }

    #[test]
    fn test_build_touchstone_dataset_from_three_port_result() {
        use crate::simulation::results::WaveformData as EngineWaveformData;
        use crate::simulation::SimulationResult;
        use std::collections::HashMap;

        let freqs = vec![1e6, 2e6];
        let mut waveforms = HashMap::new();
        for row in 1..=3 {
            for col in 1..=3 {
                let name = format!("S{}_{}", row, col);
                waveforms.insert(
                    name.clone(),
                    EngineWaveformData::new_complex(
                        name.clone(),
                        freqs.clone(),
                        vec![0.1 * row as f64, 0.2 * col as f64],
                        vec![0.01 * col as f64, -0.02 * row as f64],
                    ),
                );
            }
        }

        let result = SimulationResult::Ac {
            frequencies: freqs.clone(),
            waveforms,
        };
        let dataset =
            SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 60.0, 50.0], 2)
                .expect("touchstone dataset should build for three ports");

        assert_eq!(dataset.point_count(), 2);
        assert_eq!(dataset.signal_count(), 18);
        assert_eq!(
            dataset
                .metadata
                .get("num_ports")
                .cloned()
                .unwrap_or_default(),
            "3"
        );
    }

    #[test]
    fn test_touchstone_export_path_uses_schematic_file_directory() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(PathBuf::from("C:\\proj\\rf\\amp_top.rsch"));

        let path = SimulationController::touchstone_export_path(&state, 7, 2, 2);
        let normalized = path.to_string_lossy().replace('\\', "/");
        assert!(
            normalized.ends_with("C:/proj/rf/amp_top_run0007_sp02.s2p"),
            "unexpected export path: {}",
            normalized
        );
    }

    #[test]
    fn test_touchstone_export_path_uses_port_count_extension() {
        let mut state = AppState::default();
        state.schematic.current_file = Some(PathBuf::from("C:\\proj\\rf\\amp_top.rsch"));

        let path = SimulationController::touchstone_export_path(&state, 7, 2, 3);
        let normalized = path.to_string_lossy().replace('\\', "/");
        assert!(
            normalized.ends_with("C:/proj/rf/amp_top_run0007_sp02.s3p"),
            "unexpected export path: {}",
            normalized
        );
    }
}
