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
use crate::common::export_workflow::ExportWorkflowIo;
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
use crate::simulation::runner::SimulationError;
use crate::simulation::runner::SpecExecutionOptions;
use crate::simulation::{AnalysisConfig, SimulationRunner, SimulationStatus};
use crate::state::{
    AnalysisResult, AnalysisType, DcOpResult, OperatingPointValue, SimulationRunIntent,
};

mod analysis_commands;
mod analysis_helpers;
mod analysis_plan;
mod analysis_run_config;
mod analysis_spec_build;
mod manual_deck;
mod results_convert;
mod results_post;
mod results_update;
pub(crate) mod spice_value;
mod touchstone;
mod transient_post;
pub(crate) use transient_post::DerivedViewerLoadState;

use self::spice_value::parse_spice_value_checked;

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
    /// Stable run ID that owns the in-flight batch.
    current_run_id: Option<u64>,

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
    /// Runtime coordinator for transient-derived viewer data (eye/FFT).
    transient_post: transient_post::TransientPostCoordinator,
    /// App-state design epoch this controller's runner/queue belong to.
    design_execution_epoch: u64,
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
            current_run_id: None,
            pending_analyses: VecDeque::new(),
            current_analysis_idx: 0,
            total_analyses: 0,
            cached_netlist: None,
            transient_post: transient_post::TransientPostCoordinator::default(),
            design_execution_epoch: 0,
        }
    }

    /// Process simulation state updates
    ///
    /// Call this once per frame in the app update loop.
    pub(crate) fn update(
        &mut self,
        state: &mut AppState,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) {
        self.reset_if_design_replaced(state);

        // Handle simulation trigger
        if state.simulation.trigger_simulation {
            log::info!(
                "Simulation triggered ({} analyses enabled)",
                state.sim_setup.enabled.len()
            );
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
            self.current_run_id = None;
            state.shell.netlist.pending_manual_run_id = None;
            state.shell.netlist.pending_run_buffer = None;
            state.simulation.status = "Aborted".to_string();
            state.push_sim_message(crate::common::app::ConsoleMessage::warning(
                "Simulation aborted by user",
            ));
        }

        // Poll for completion
        self.poll_completion(state, export_io);

        // Apply/cancel background transient post-processing work after any
        // selection changes that happened during the previous frame.
        self.sync_transient_post_views(state);

        // Update running state
        let is_running = self.runner.is_running();
        state.simulation.progress =
            Self::ui_progress_fraction(self.runner.progress_fraction(), is_running);
        state.simulation.is_running = is_running;
    }

    /// Start a new simulation batch
    ///
    /// Builds all enabled analyses into a queue and starts the first one.
    /// Subsequent analyses are started automatically upon completion.
    fn start_simulation(&mut self, state: &mut AppState) {
        log::info!("start_simulation called");
        self.design_execution_epoch = state.design_execution_epoch;

        match state.simulation.run_intent {
            SimulationRunIntent::SimulateRunSet => self.start_simulate_run_set(state),
            SimulationRunIntent::ManualDeck => self.start_manual_deck_simulation(state),
        }
    }

    fn analysis_source_path(state: &AppState) -> Option<PathBuf> {
        match state.simulation.run_intent {
            SimulationRunIntent::ManualDeck => state.workspace.netlist_source_path.clone(),
            SimulationRunIntent::SimulateRunSet => state.schematic.current_file.clone(),
        }
    }

    fn reset_if_design_replaced(&mut self, state: &mut AppState) {
        if self.design_execution_epoch == state.design_execution_epoch {
            return;
        }

        self.reset_for_design_replacement();
        self.design_execution_epoch = state.design_execution_epoch;
    }

    fn reset_for_design_replacement(&mut self) {
        self.runner.reset_for_design_replacement();
        self.pending_analyses.clear();
        self.cached_netlist = None;
        self.current_config = None;
        self.current_spec = None;
        self.current_run_id = None;
        self.current_analysis_idx = 0;
        self.total_analyses = 0;
        self.transient_post = transient_post::TransientPostCoordinator::default();
    }

    fn start_manual_deck_simulation(&mut self, state: &mut AppState) {
        if let Some(reason) = state.manual_deck_run_block_reason() {
            state.push_sim_message(ConsoleMessage::warning(reason));
            state.simulation.status = "Run blocked".to_string();
            return;
        }

        self.pending_analyses.clear();
        let source = state
            .workspace
            .netlist_source
            .clone()
            .unwrap_or_else(|| state.simulation.netlist_content.clone());
        let queued = match manual_deck::build_manual_deck_queue(state, &source) {
            Ok(queue) => queue,
            Err(errors) => {
                for err in errors {
                    state.push_sim_message(ConsoleMessage::error(err));
                }
                state.simulation.status = "Configuration error".to_string();
                return;
            }
        };
        state.push_sim_message(ConsoleMessage::info(
            "Running manually edited netlist source".to_string(),
        ));
        self.begin_simulation_batch(
            state,
            queued,
            manual_deck::compose_manual_deck_source(&source),
            source,
        );
    }

    fn start_simulate_run_set(&mut self, state: &mut AppState) {
        // Backstop direct trigger paths (tuner re-sim, automation, tests).
        if let Some(reason) = state.simulation_run_preflight_block_reason() {
            state.push_sim_message(ConsoleMessage::warning(reason));
            state.simulation.status = "Run blocked".to_string();
            return;
        }

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

        // Simulate actions always regenerate from the schematic and the
        // selected run set. The Netlist tab owns manual deck execution.
        let mut netlist = {
            let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
                &state.library_manager,
                &state.workspace.schematic_buffers,
            );
            let result = crate::simulation::netlist_gen::generate_netlist_hierarchical(
                &state.schematic,
                &analysis_lines,
                &hierarchy,
            );

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

            // Populate cross-probe mapping for probe mode. The schematic
            // keeps its own copy so the inspector's terminal-net rows and
            // the simulate panel's net count resolve without reaching
            // simulation state (the node map is small — nodes only).
            state.schematic.net_mapping = result.point_to_net.clone();
            state.simulation.cross_probe.update(
                result.point_to_net,
                result.nets,
                result.net_segments,
            );
            log::info!(
                "Cross-probe mapping populated: {} points, {} nets",
                state.simulation.cross_probe.point_to_net.len(),
                state.simulation.cross_probe.net_to_points.len()
            );

            result.netlist
        };

        netlist = Self::apply_simulation_options_to_netlist(&netlist, &state.sim_setup.options);

        log::info!(
            "Prepared netlist ({} bytes):\n{}",
            netlist.len(),
            &netlist[..netlist.len().min(500)]
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
        self.current_run_id = Some(state.simulation.start_run().id);
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

    fn begin_simulation_batch(
        &mut self,
        state: &mut AppState,
        queued: Vec<QueuedAnalysis>,
        netlist: String,
        pending_manual_source: String,
    ) {
        self.total_analyses = queued.len();
        if self.total_analyses == 0 {
            state.push_sim_message(ConsoleMessage::error(
                "No runnable analyses were selected".to_string(),
            ));
            state.simulation.status = "Configuration error".to_string();
            return;
        }
        self.current_analysis_idx = 0;

        log::info!(
            "Prepared netlist ({} bytes):\n{}",
            netlist.len(),
            &netlist[..netlist.len().min(500)]
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

        let run_id = state.simulation.start_run().id;
        self.current_run_id = Some(run_id);
        state.shell.netlist.pending_manual_run_id = Some(run_id);
        state.shell.netlist.pending_run_buffer = Some(pending_manual_source);
        state.simulation.reliability_results.clear();
        state.simulation.soa_violations.clear();
        log::info!("Created new simulation run");

        if self.total_analyses > 1 {
            state.push_sim_message(ConsoleMessage::info(format!(
                "Starting simulation batch: {} analyses",
                self.total_analyses
            )));
        }

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
            let target_run_id = self.target_run_id(state);
            if let Some(run_id) = target_run_id
                && let Some(run) = state.simulation.run_by_id_mut(run_id)
            {
                run.success = false;
            }
            self.pending_analyses.clear();
            self.finish_simulation_batch(state);
            state.simulation.status = "Error".to_string();
            return;
        };

        // Start the simulation
        let source_path = Self::analysis_source_path(state);
        let start_result = if let Some(cfg) = config {
            self.runner
                .start_with_source_path(cfg, netlist, source_path)
        } else {
            self.runner.start_spec_with_options_with_source_path(
                spec,
                netlist,
                spec_options,
                source_path,
            )
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
                let target_run_id = self.target_run_id(state);
                if let Some(run_id) = target_run_id
                    && let Some(run) = state.simulation.run_by_id_mut(run_id)
                {
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

    fn target_run_id(&self, state: &AppState) -> Option<u64> {
        self.current_run_id
            .or_else(|| state.simulation.runs.first().map(|run| run.id))
    }

    /// Finish the simulation batch and clean up state
    fn finish_simulation_batch(&mut self, state: &mut AppState) {
        let completed_run_id = self.target_run_id(state);
        let run_success = completed_run_id
            .and_then(|run_id| state.simulation.run_by_id(run_id))
            .map(|run| run.success)
            .or_else(|| state.simulation.active_run().map(|run| run.success))
            .unwrap_or(true);

        // Complete the run (syncs waveforms and selects first analysis)
        if let Some(run_id) = completed_run_id {
            state.simulation.select_run_by_id(run_id);
            state.simulation.complete_run();
        } else {
            state.simulation.complete_run();
        }
        Self::promote_manual_deck_baseline(state, run_success, completed_run_id);

        // Clear cached netlist
        self.cached_netlist = None;
        self.current_config = None;
        self.current_spec = None;
        self.current_run_id = None;
        self.current_analysis_idx = 0;
        self.total_analyses = 0;

        state.simulation.status = if run_success {
            "Complete".to_string()
        } else {
            "Completed with errors".to_string()
        };

        log::info!("Simulation batch completed");
    }

    fn promote_manual_deck_baseline(
        state: &mut AppState,
        run_success: bool,
        completed_run_id: Option<u64>,
    ) {
        let pending_matches = state.shell.netlist.pending_manual_run_id.is_some()
            && state.shell.netlist.pending_manual_run_id == completed_run_id;
        if !pending_matches {
            return;
        }

        let pending_buffer = state.shell.netlist.pending_run_buffer.take();
        state.shell.netlist.pending_manual_run_id = None;

        if run_success && let Some(buffer) = pending_buffer {
            let current_buffer = state.simulation.netlist_content.clone();
            let param_values = Self::manual_deck_param_values(&buffer);
            state.shell.netlist.last_run_buffer = Some(buffer);
            state.shell.netlist.last_run_params = param_values;
            if let Some(baseline) = state.shell.netlist.last_run_buffer.as_deref() {
                state.shell.netlist.edited_lines =
                    Self::changed_lines_against_baseline(&current_buffer, baseline);
            }
        }
    }

    fn changed_lines_against_baseline(
        current: &str,
        baseline: &str,
    ) -> std::collections::HashSet<usize> {
        let current_lines: Vec<&str> = current.lines().collect();
        let baseline_lines: Vec<&str> = baseline.lines().collect();
        let max_len = current_lines.len().max(baseline_lines.len());
        let mut changed = std::collections::HashSet::new();

        for idx in 0..max_len {
            if current_lines.get(idx) != baseline_lines.get(idx) {
                changed.insert(idx);
            }
        }

        changed
    }

    fn manual_deck_param_values(buffer: &str) -> std::collections::HashMap<String, f64> {
        let mut out = std::collections::HashMap::new();
        for line in buffer.lines() {
            let trimmed = line.trim_start();
            let lower = trimmed.to_ascii_lowercase();
            if !(lower.starts_with(".param") || lower.starts_with(".parameter")) {
                continue;
            }
            let Some(after_cmd) = trimmed.find(char::is_whitespace) else {
                continue;
            };
            for (name, raw) in Self::manual_deck_param_assignments(&trimmed[after_cmd..]) {
                if raw.starts_with('{') {
                    continue;
                }
                if let Ok(value) = crate::properties::engineering::parse_engineering_value(raw) {
                    out.insert(name.to_ascii_lowercase(), value);
                }
            }
        }
        out
    }

    fn manual_deck_param_assignments(line: &str) -> Vec<(String, &str)> {
        let bytes = line.as_bytes();
        let mut assignments = Vec::new();
        let mut i = 0usize;

        while i < bytes.len() {
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] == b';' || bytes[i] == b'$' {
                break;
            }

            let name_start = i;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch.is_whitespace() || ch == '=' {
                    break;
                }
                i += 1;
            }
            if name_start == i {
                break;
            }
            let name = line[name_start..i].to_string();

            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] != b'=' {
                break;
            }
            i += 1;
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() {
                break;
            }

            let value_start = i;
            if bytes[i] == b'{' {
                let mut depth = 0i32;
                while i < bytes.len() {
                    match bytes[i] {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                i += 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            } else {
                while i < bytes.len() && !(bytes[i] as char).is_whitespace() {
                    i += 1;
                }
            }
            assignments.push((name, line[value_start..i].trim()));
        }

        assignments
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
    fn poll_completion(
        &mut self,
        state: &mut AppState,
        export_io: &(impl ExportWorkflowIo + ?Sized),
    ) {
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
                    let target_run_id = self.target_run_id(state);

                    self.apply_result_side_effects(state, &sim_result);
                    if let crate::simulation::SimulationResult::Transient {
                        time, waveforms, ..
                    } = &sim_result
                    {
                        self.populate_transient_post_views(state, time, waveforms);
                    }

                    // Optional Touchstone export for S-parameter analyses.
                    let export_run_id = target_run_id
                        .or_else(|| state.simulation.active_run().map(|run| run.id))
                        .unwrap_or(0);
                    self.maybe_export_touchstone_for_run(
                        state,
                        &sim_result,
                        export_io,
                        export_run_id,
                    );

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

                    let analysis_result = if let Some(config) = &self.current_config {
                        self.convert_to_analysis_result_owned(sim_result, config)
                    } else {
                        self.convert_to_analysis_result_with_metadata_owned(
                            sim_result,
                            analysis_type,
                            current_label,
                        )
                    };
                    if let Some(run_id) = target_run_id
                        && let Some(run) = state.simulation.run_by_id_mut(run_id)
                    {
                        run.add_analysis(analysis_result);
                        log::info!(
                            "Added analysis to run {} (now has {} analyses)",
                            run.id,
                            run.analyses.len()
                        );
                    }

                    // Display the just-completed analysis without rebuilding waveform buffers.
                    if let Some(run_id) = target_run_id {
                        state.simulation.select_latest_analysis_in_run(run_id);
                    } else {
                        state.simulation.select_latest_analysis();
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
                Err(SimulationError::Aborted) => {
                    log::info!("Analysis aborted; discarding cancellation completion result");
                    self.pending_analyses.clear();
                    self.cached_netlist = None;
                    self.current_config = None;
                    self.current_spec = None;
                    self.current_run_id = None;
                    state.shell.netlist.pending_manual_run_id = None;
                    state.shell.netlist.pending_run_buffer = None;
                    state.simulation.status = "Aborted".to_string();
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
                    let target_run_id = self.target_run_id(state);
                    if let Some(run_id) = target_run_id
                        && let Some(run) = state.simulation.run_by_id_mut(run_id)
                    {
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
                        state.simulation.status = "Completed with errors".to_string();
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

    fn ui_progress_fraction(progress: Option<f32>, is_running: bool) -> f64 {
        progress
            .map(|value| f64::from(value).clamp(0.0, 1.0))
            .unwrap_or_else(|| if is_running { 0.08 } else { 0.0 })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
    use crate::common::simulation_analysis_tabs::TAB_DC_OP;
    use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
    use crate::state::{ComponentType, Point};
    use std::cell::RefCell;
    use std::path::Path;

    #[derive(Debug, Default)]
    struct MockExportWorkflowIo {
        writes: RefCell<Vec<(PathBuf, String)>>,
    }

    impl ExportWorkflowIo for MockExportWorkflowIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(None)
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.writes
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_string()));
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    fn state_with_current_drc_error() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));

        let mut result = DrcResult::new();
        result.add_violation(DrcViolation::new(
            1,
            DrcViolationType::MissingGround,
            "missing ground",
            DrcLocation::Global,
        ));
        result.completed = true;
        state.dialogs.drc_results = Some(result);
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        state
    }

    fn synthetic_sparameter_result() -> crate::simulation::SimulationResult {
        let frequencies = vec![1.0e6, 2.0e6];
        let mut waveforms = std::collections::HashMap::new();
        for (name, real, imag) in [
            ("S11", vec![0.1, 0.2], vec![0.0, 0.0]),
            ("S21", vec![0.3, 0.4], vec![0.01, 0.02]),
            ("S12", vec![0.5, 0.6], vec![0.03, 0.04]),
            ("S22", vec![0.7, 0.8], vec![0.05, 0.06]),
        ] {
            waveforms.insert(
                name.to_string(),
                crate::simulation::results::WaveformData::new_complex(
                    name,
                    frequencies.clone(),
                    real,
                    imag,
                ),
            );
        }

        crate::simulation::SimulationResult::Ac {
            frequencies,
            waveforms,
            measurements: Vec::new(),
        }
    }

    fn synthetic_dc_op_result() -> crate::simulation::SimulationResult {
        let mut result = crate::simulation::results::DcOpResult::default();
        result.node_voltages.insert("out".to_string(), 1.25);
        crate::simulation::SimulationResult::DcOp(result)
    }

    fn exact_vec(values: &[f64]) -> Vec<f64> {
        let mut result = Vec::with_capacity(values.len());
        result.extend_from_slice(values);
        assert_eq!(
            result.capacity(),
            result.len(),
            "test vectors must be tightly allocated so pointer reuse is meaningful"
        );
        result
    }

    #[test]
    fn direct_trigger_is_blocked_by_current_drc_errors() {
        let mut state = state_with_current_drc_error();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();

        state.simulation.request_simulate_run_set();
        controller.update(&mut state, &export_io);

        assert!(!state.simulation.trigger_simulation);
        assert!(!state.simulation.is_running);
        assert_eq!(state.simulation.status, "Run blocked");
    }

    #[test]
    fn design_context_reset_discards_pending_controller_result() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Ok(synthetic_dc_op_result()))
            .expect("seed old pending result");

        state.clear_design_execution_context();
        controller.update(&mut state, &export_io);

        assert!(
            !state.simulation.has_results(),
            "stale result from previous design should be discarded"
        );
        assert_eq!(state.log_buffer.len(), 0);
        assert_eq!(state.simulation.status, "");
        assert!(!controller.is_running());
    }

    #[test]
    fn abort_trigger_discards_worker_aborted_result_without_failed_analysis() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        state.simulation.status = "Running".to_string();
        state.simulation.trigger_abort = true;
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Err(crate::simulation::runner::SimulationError::Aborted))
            .expect("seed worker abort result");

        controller.update(&mut state, &export_io);

        assert_eq!(state.simulation.status, "Aborted");
        let run = state.simulation.active_run().expect("active run remains");
        assert!(
            run.analyses.is_empty(),
            "aborted worker result must not be recorded as a failed analysis: {:?}",
            run.analyses
        );
        assert!(run.success);
    }

    #[test]
    fn abort_trigger_discards_unpolled_success_result() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        state.simulation.status = "Running".to_string();
        state.simulation.trigger_abort = true;
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Ok(synthetic_dc_op_result()))
            .expect("seed unpolled success result");

        controller.update(&mut state, &export_io);

        assert_eq!(state.simulation.status, "Aborted");
        let run = state.simulation.active_run().expect("active run remains");
        assert!(
            run.analyses.is_empty(),
            "success result that arrived before abort poll must not be recorded: {:?}",
            run.analyses
        );
        assert!(run.success);
    }

    #[test]
    fn completed_result_attaches_to_started_run_when_active_selection_changes() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        let older_run_id = state.simulation.start_run().id;
        let started_run_id = state.simulation.start_run().id;
        assert!(
            state.simulation.select_run(1),
            "user can inspect an older run while a newer run is in flight"
        );
        controller.current_spec = Some(AnalysisSpec::DcOp);
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;
        controller
            .runner
            .store_pending_result(Ok(synthetic_dc_op_result()))
            .expect("seed completed run result");

        controller.update(&mut state, &export_io);

        let older_run = state
            .simulation
            .run_by_id(older_run_id)
            .expect("older run remains");
        let started_run = state
            .simulation
            .run_by_id(started_run_id)
            .expect("started run remains");
        assert!(
            older_run.analyses.is_empty(),
            "completed analysis must not contaminate the selected historical run"
        );
        assert_eq!(started_run.analyses.len(), 1);
        assert_eq!(
            state.simulation.active_run().map(|run| run.id),
            Some(started_run_id)
        );
    }

    #[test]
    fn completed_transient_result_reuses_owned_waveform_buffers_in_run_history() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        controller.current_spec = Some(AnalysisSpec::Transient {
            stop_time: 2.0e-9,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        });
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;

        let time = exact_vec(&[0.0, 1.0e-9, 2.0e-9]);
        let time_ptr = time.as_ptr();
        let values = exact_vec(&[0.0, 0.5, 1.0]);
        let values_ptr = values.as_ptr();
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            crate::simulation::results::WaveformData {
                name: "V(out)".to_string(),
                x_values: Vec::new(),
                y_values: values,
                y_unit: "V".to_string(),
                x_unit: "s".to_string(),
                is_complex: false,
                y_imag: None,
            },
        );

        controller
            .runner
            .store_pending_result(Ok(crate::simulation::SimulationResult::Transient {
                time,
                waveforms,
                measurements: Vec::new(),
            }))
            .expect("seed completed transient result");

        controller.update(&mut state, &export_io);

        let analysis = state
            .simulation
            .active_analysis()
            .expect("completed transient analysis is selected");
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "V(out)")
            .expect("transient waveform is stored in run history");

        assert_eq!(
            waveform.x.iter().as_slice().as_ptr(),
            time_ptr,
            "time vector should move into run history instead of being copied"
        );
        assert_eq!(
            waveform.y.iter().as_slice().as_ptr(),
            values_ptr,
            "sample vector should move into run history instead of being copied"
        );
    }

    #[test]
    fn completed_dc_sweep_result_reuses_owned_shared_axis_buffers_in_run_history() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        let export_io = MockExportWorkflowIo::default();
        state.simulation.start_run();
        controller.current_spec = Some(AnalysisSpec::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 2.0,
            step: 1.0,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        });
        controller.current_analysis_idx = 1;
        controller.total_analyses = 1;

        let sweep_values = exact_vec(&[0.0, 1.0, 2.0]);
        let sweep_ptr = sweep_values.as_ptr();
        let values = exact_vec(&[0.1, 0.2, 0.3]);
        let values_ptr = values.as_ptr();
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(out)".to_string(),
            crate::simulation::results::WaveformData {
                name: "V(out)".to_string(),
                x_values: Vec::new(),
                y_values: values,
                y_unit: "V".to_string(),
                x_unit: "V".to_string(),
                is_complex: false,
                y_imag: None,
            },
        );

        controller
            .runner
            .store_pending_result(Ok(crate::simulation::SimulationResult::DcSweep {
                sweep_var: "V1".to_string(),
                sweep_values,
                waveforms,
                measurements: Vec::new(),
            }))
            .expect("seed completed DC sweep result");

        controller.update(&mut state, &export_io);

        let analysis = state
            .simulation
            .active_analysis()
            .expect("completed DC sweep analysis is selected");
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "V(out)")
            .expect("DC sweep waveform is stored in run history");

        assert_eq!(
            waveform.x.iter().as_slice().as_ptr(),
            sweep_ptr,
            "shared sweep axis should move into run history instead of being copied"
        );
        assert_eq!(
            waveform.y.iter().as_slice().as_ptr(),
            values_ptr,
            "sweep samples should move into run history instead of being copied"
        );
    }

    #[test]
    fn touchstone_auto_export_uses_export_workflow_io() {
        let mut state = AppState::default();
        state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(
            &crate::simulation::dialog::SpConfig::default(),
        );
        state.schematic.current_file = Some(PathBuf::from("designs").join("amp.sch"));
        state.simulation.start_run();

        let mut controller = SimulationController::new();
        controller.current_spec = Some(AnalysisSpec::SParameter {
            start_freq: 1.0e6,
            stop_freq: 2.0e6,
            points_per_unit: 2,
            sweep: FrequencySweep::Linear,
            z0: 50.0,
            ports: vec![
                SpPort {
                    node_pos: "IN".to_string(),
                    node_neg: "0".to_string(),
                    z0: None,
                },
                SpPort {
                    node_pos: "OUT".to_string(),
                    node_neg: "0".to_string(),
                    z0: Some(75.0),
                },
            ],
        });
        controller.current_analysis_idx = 1;

        let export_io = MockExportWorkflowIo::default();
        controller.maybe_export_touchstone_for_run(
            &mut state,
            &synthetic_sparameter_result(),
            &export_io,
            1,
        );

        let writes = export_io.writes.borrow();
        assert_eq!(writes.len(), 1);
        assert_eq!(
            writes[0].0,
            PathBuf::from("designs").join("amp_run0001_sp01.s2p")
        );
        assert!(writes[0].1.contains("[Version] 2.0"));
        assert!(
            writes[0]
                .1
                .contains("[Reference] 5.000000000000e1 7.500000000000e1")
        );
    }

    #[test]
    fn touchstone_native_completion_message_confirms_file_export() {
        let message =
            SimulationController::touchstone_export_completed_message(Path::new("amp.s2p"));
        assert!(message.contains("Exported Touchstone"));
        assert!(message.contains("amp.s2p"));
    }

    #[test]
    fn ac_result_conversion_retains_complex_components_for_export() {
        let controller = SimulationController::new();
        let result = synthetic_sparameter_result();

        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            result,
            AnalysisType::SParameter,
            "SP",
        );
        let magnitude = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == "|S11|")
            .expect("magnitude trace exists");
        let complex = magnitude
            .complex
            .as_ref()
            .expect("magnitude trace retains complex source data");

        assert_eq!(complex.source_name, "S11");
        assert_eq!(&*complex.real, &[0.1, 0.2]);
        assert_eq!(&*complex.imag, &[0.0, 0.0]);
    }

    #[test]
    fn ac_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
        let controller = SimulationController::new();
        let frequencies = vec![1.0, 10.0, 100.0];
        let mut waveforms = std::collections::HashMap::new();
        waveforms.insert(
            "V(bad_real)".to_string(),
            crate::simulation::results::WaveformData::new_complex(
                "V(bad_real)",
                frequencies.clone(),
                vec![1.0, 2.0],
                vec![0.0, 0.0, 0.0],
            ),
        );
        waveforms.insert(
            "V(bad_imag)".to_string(),
            crate::simulation::results::WaveformData::new_complex(
                "V(bad_imag)",
                frequencies.clone(),
                vec![1.0, 2.0, 3.0],
                vec![0.0, 0.0],
            ),
        );
        waveforms.insert(
            "V(good)".to_string(),
            crate::simulation::results::WaveformData::new_complex(
                "V(good)",
                frequencies.clone(),
                vec![3.0, 4.0, 5.0],
                vec![4.0, 3.0, 0.0],
            ),
        );

        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements: Vec::new(),
            },
            AnalysisType::Ac,
            "AC",
        );

        let names: Vec<_> = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect();
        assert_eq!(names, vec!["|V(good)|", "phase(V(good))"]);
        assert!(
            analysis
                .waveforms
                .iter()
                .all(|waveform| waveform.x.len() == waveform.y.len()),
            "converted AC traces must never pair mismatched x/y arrays"
        );
    }

    #[test]
    fn noise_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
        let controller = SimulationController::new();
        let mut contributors = std::collections::HashMap::new();
        contributors.insert("good".to_string(), vec![1.0e-18, 2.0e-18, 3.0e-18]);
        contributors.insert("bad".to_string(), vec![1.0e-18, 2.0e-18]);

        let analysis = controller.convert_to_analysis_result_with_metadata_owned(
            crate::simulation::SimulationResult::Noise {
                frequencies: vec![1.0, 10.0, 100.0],
                output_noise: vec![2.0e-18, 3.0e-18],
                input_noise: Some(vec![1.0e-18, 1.5e-18, 2.0e-18]),
                contributors,
                summary: None,
            },
            AnalysisType::Noise,
            "Noise",
        );

        let names: Vec<_> = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect();
        assert_eq!(names, vec!["inoise", "noise(good)"]);
        assert!(
            analysis
                .waveforms
                .iter()
                .all(|waveform| waveform.x.len() == waveform.y.len()),
            "converted noise traces must never pair mismatched x/y arrays"
        );
    }

    #[test]
    fn manual_deck_trigger_runs_deck_analysis_without_enabled_run_set() {
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
        state.simulation.request_manual_deck_run();
        let mut controller = SimulationController::new();

        controller.start_simulation(&mut state);
        let total_analyses = controller.total_analyses;
        let current_spec = controller.current_spec.clone();
        let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
        let run_count = state.simulation.runs.len();
        let status = state.simulation.status.clone();
        controller.abort();

        assert_eq!(total_analyses, 1);
        assert!(matches!(current_spec, Some(AnalysisSpec::DcOp)));
        assert!(cached_netlist.contains(".op\n.end"));
        assert_eq!(run_count, 1);
        assert_eq!(status, "DC Operating Point");
    }

    #[test]
    fn manual_deck_run_preserves_editor_source_without_ui_option_injection() {
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.sim_setup.options.reltol = 1.0e-4;
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
        state.simulation.request_manual_deck_run();
        let mut controller = SimulationController::new();

        controller.start_simulation(&mut state);
        let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
        controller.abort();

        assert_eq!(cached_netlist, "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
        assert!(!cached_netlist.contains(".OPTIONS"));
        assert!(!cached_netlist.contains("RELTOL"));
    }

    #[test]
    fn manual_deck_runs_use_imported_netlist_origin_for_relative_includes() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
        state.workspace.netlist_source =
            Some("deck\n.include models.lib\nV1 out 0 1\n.op\n.end\n".to_string());
        state.workspace.netlist_source_path = Some(PathBuf::from("decks").join("bias.cir"));

        assert_eq!(
            SimulationController::analysis_source_path(&state).as_deref(),
            Some(std::path::Path::new("decks").join("bias.cir").as_path())
        );
    }

    #[test]
    fn manual_deck_runs_do_not_fall_back_to_schematic_path() {
        let mut state = AppState::default();
        state.simulation.run_intent = SimulationRunIntent::ManualDeck;
        state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
        state.workspace.netlist_source = Some("deck\nV1 out 0 1\n.op\n.end\n".to_string());

        assert!(
            SimulationController::analysis_source_path(&state).is_none(),
            "manual netlist text without an origin must not resolve includes from the schematic file"
        );
    }

    #[test]
    fn simulate_run_set_does_not_run_manual_deck_source() {
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.sim_setup.enabled.insert(TAB_DC_OP);
        state.workspace.netlist_source =
            Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
        state.simulation.request_simulate_run_set();
        let mut controller = SimulationController::new();

        controller.start_simulation(&mut state);
        let status = state.simulation.status.clone();
        let run_count = state.simulation.runs.len();
        let total_analyses = controller.total_analyses;
        controller.abort();

        assert_eq!(status, "Run blocked");
        assert_eq!(run_count, 0);
        assert_eq!(total_analyses, 0);
    }

    #[test]
    fn successful_manual_deck_run_promotes_pending_baseline() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        state.simulation.netlist_content =
            "deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string();
        let run_id = state.simulation.start_run().id;
        state.shell.netlist.pending_manual_run_id = Some(run_id);
        state.shell.netlist.pending_run_buffer =
            Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string());
        state.shell.netlist.edited_lines.insert(0);

        controller.finish_simulation_batch(&mut state);

        assert_eq!(
            state.shell.netlist.last_run_buffer.as_deref(),
            Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n")
        );
        assert!((state.shell.netlist.last_run_params["r"] - 1e3).abs() < 1e-9);
        assert!((state.shell.netlist.last_run_params["cl"] - 2e-12).abs() < 1e-21);
        assert!(!state.shell.netlist.last_run_params.contains_key("expr"));
        assert!(state.shell.netlist.pending_manual_run_id.is_none());
        assert!(state.shell.netlist.pending_run_buffer.is_none());
        assert!(state.shell.netlist.edited_lines.is_empty());
    }

    #[test]
    fn failed_manual_deck_run_keeps_previous_baseline() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        state.shell.netlist.last_run_buffer = Some("old\n.op\n.end\n".to_string());
        let run = state.simulation.start_run();
        let run_id = run.id;
        run.success = false;
        state.shell.netlist.pending_manual_run_id = Some(run_id);
        state.shell.netlist.pending_run_buffer = Some("new\n.op\n.end\n".to_string());

        controller.finish_simulation_batch(&mut state);

        assert_eq!(
            state.shell.netlist.last_run_buffer.as_deref(),
            Some("old\n.op\n.end\n")
        );
        assert!(state.shell.netlist.pending_manual_run_id.is_none());
        assert!(state.shell.netlist.pending_run_buffer.is_none());
    }

    #[test]
    fn successful_manual_deck_run_preserves_post_launch_diff_pips() {
        let mut state = AppState::default();
        let mut controller = SimulationController::new();
        state.simulation.netlist_content = "deck\n.op\nR1 out 0 2k\n.end\n".to_string();
        let run_id = state.simulation.start_run().id;
        state.shell.netlist.pending_manual_run_id = Some(run_id);
        state.shell.netlist.pending_run_buffer = Some("deck\n.op\nR1 out 0 1k\n.end\n".to_string());

        controller.finish_simulation_batch(&mut state);

        assert_eq!(
            state.shell.netlist.last_run_buffer.as_deref(),
            Some("deck\n.op\nR1 out 0 1k\n.end\n")
        );
        assert!(state.shell.netlist.edited_lines.contains(&2));
        assert_eq!(state.shell.netlist.edited_lines.len(), 1);
    }

    #[test]
    fn ui_progress_fraction_uses_runner_fraction_or_running_floor() {
        assert!((SimulationController::ui_progress_fraction(Some(0.42), true) - 0.42).abs() < 1e-6);
        assert_eq!(SimulationController::ui_progress_fraction(None, true), 0.08);
        assert_eq!(SimulationController::ui_progress_fraction(None, false), 0.0);
        assert_eq!(
            SimulationController::ui_progress_fraction(Some(1.2), true),
            1.0
        );
    }
}
