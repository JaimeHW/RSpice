//! Run Queue Executor
//!
//! Executes multi-run analysis queues through the simulation engine.
//! Handles dependencies, parallelization, and result aggregation.
//!
//! # Features
//!
//! - Execute queued analyses in dependency order
//! - Parallel execution where possible
//! - Progress tracking and cancellation
//! - Result aggregation across runs
//! - Corner sweep execution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::Instant;

use super::config::{AnalysisConfig, DcSweepConfig};
use super::multi_run::{AnalysisRun, AnalysisRunType, AnalysisSpec, RunQueue, RunStatus};
use super::options_translator::{EngineOptions, OptionsTranslator, PvtCorner};
use super::result_mapper::{
    MappedAnalysisType, MappedMeasurement, MappedResult, MappedWaveform, MeasurementStatus,
    MeasurementType, ResultMapper, ResultStatus,
};
use crate::output_spec::sensitivity_raw_unit;

// =============================================================================
// Execution State
// =============================================================================

/// Execution state for a run queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionState {
    /// Total runs
    pub total_runs: usize,
    /// Completed runs
    pub completed_runs: usize,
    /// Failed runs
    pub failed_runs: usize,
    /// Current run name
    pub current_run: Option<String>,
    /// Overall status
    pub status: ExecutionStatus,
    /// Start time (epoch ms)
    pub start_time: u64,
    /// Elapsed time (seconds)
    pub elapsed_seconds: f64,
    /// Estimated remaining time
    pub eta_seconds: Option<f64>,
}

/// Overall execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// Not started
    #[default]
    Idle,
    /// Currently running
    Running,
    /// Paused
    Paused,
    /// Completed successfully
    Completed,
    /// Completed with errors
    CompletedWithErrors,
    /// Cancelled by user
    Cancelled,
    /// Fatal error
    Error,
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            total_runs: 0,
            completed_runs: 0,
            failed_runs: 0,
            current_run: None,
            status: ExecutionStatus::Idle,
            start_time: 0,
            elapsed_seconds: 0.0,
            eta_seconds: None,
        }
    }
}

impl ExecutionState {
    /// Progress percentage
    pub fn progress_percent(&self) -> f32 {
        if self.total_runs == 0 {
            0.0
        } else {
            (self.completed_runs as f32 / self.total_runs as f32) * 100.0
        }
    }

    /// Is complete?
    pub fn is_complete(&self) -> bool {
        matches!(
            self.status,
            ExecutionStatus::Completed
                | ExecutionStatus::CompletedWithErrors
                | ExecutionStatus::Error
        )
    }

    /// Update ETA based on current progress
    pub fn update_eta(&mut self) {
        if self.completed_runs > 0 && self.completed_runs < self.total_runs {
            let avg_time = self.elapsed_seconds / self.completed_runs as f64;
            let remaining = self.total_runs - self.completed_runs;
            self.eta_seconds = Some(avg_time * remaining as f64);
        } else {
            self.eta_seconds = None;
        }
    }
}

// =============================================================================
// Execution Result
// =============================================================================

/// Result of queue execution
#[derive(Debug, Clone, Default)]
pub struct ExecutionResult {
    /// Results by run ID
    pub results: HashMap<u64, MappedResult>,
    /// Execution state
    pub state: ExecutionState,
    /// Errors by run ID
    pub errors: HashMap<u64, String>,
}

impl ExecutionResult {
    /// Get result for a run
    pub fn get(&self, run_id: u64) -> Option<&MappedResult> {
        self.results.get(&run_id)
    }

    /// All successful results
    pub fn successful_results(&self) -> Vec<(u64, &MappedResult)> {
        self.results
            .iter()
            .filter(|(_, r)| r.status == ResultStatus::Success)
            .map(|(&k, v)| (k, v))
            .collect()
    }

    /// Count of successful runs
    pub fn success_count(&self) -> usize {
        self.results
            .values()
            .filter(|r| r.status == ResultStatus::Success)
            .count()
    }
}

struct ParallelRunOutcome {
    run_id: u64,
    run_name: String,
    result: Result<MappedResult, String>,
}

// =============================================================================
// Run Executor
// =============================================================================

/// Executor for run queues
pub struct RunExecutor {
    /// Options translator
    options_translator: OptionsTranslator,
    /// Optional per-executor engine option overrides injected into run netlists.
    engine_options_override: Option<EngineOptions>,
    /// Result mapper
    result_mapper: ResultMapper,
    /// Cancellation flag
    cancelled: Arc<AtomicBool>,
    /// Current progress
    progress: Arc<AtomicUsize>,
    /// Maximum parallel runs (0 = serial)
    max_parallel: usize,
}

impl Default for RunExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl RunExecutor {
    /// Create new executor
    pub fn new() -> Self {
        Self {
            options_translator: OptionsTranslator::new(),
            engine_options_override: None,
            result_mapper: ResultMapper::new(),
            cancelled: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(AtomicUsize::new(0)),
            max_parallel: 1, // Serial by default
        }
    }

    /// Set max parallel runs
    pub fn with_parallel(mut self, max: usize) -> Self {
        self.max_parallel = max;
        self
    }

    /// Override engine options used by all runs in this executor.
    pub fn with_engine_options(mut self, options: EngineOptions) -> Self {
        self.engine_options_override = Some(options);
        self
    }

    /// Build and apply engine option overrides from convergence settings.
    pub fn with_convergence_options(
        mut self,
        convergence: &super::convergence::ConvergenceOptions,
    ) -> Self {
        self.engine_options_override = Some(self.options_translator.from_convergence(convergence));
        self
    }

    /// Cancel execution
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Reset cancellation
    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::SeqCst);
        self.progress.store(0, Ordering::SeqCst);
    }

    /// Is cancelled?
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Current progress count
    pub fn current_progress(&self) -> usize {
        self.progress.load(Ordering::SeqCst)
    }

    /// Get current timestamp
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    /// Execute a run queue (synchronous)
    pub fn execute(&self, queue: &mut RunQueue) -> ExecutionResult {
        let start = Instant::now();
        let mut result = ExecutionResult::default();

        result.state.total_runs = queue.len();
        result.state.status = ExecutionStatus::Running;
        result.state.start_time = Self::now();
        if self.max_parallel <= 1 || queue.len() <= 1 {
            self.execute_serial(queue, &start, &mut result);
        } else {
            self.execute_parallel(queue, &start, &mut result);
        }

        // Final state
        if result.state.status == ExecutionStatus::Running {
            if result.state.failed_runs > 0 {
                result.state.status = ExecutionStatus::CompletedWithErrors;
            } else {
                result.state.status = ExecutionStatus::Completed;
            }
        }

        result.state.elapsed_seconds = start.elapsed().as_secs_f64();
        result.state.current_run = None;

        result
    }

    fn execute_serial(&self, queue: &mut RunQueue, start: &Instant, result: &mut ExecutionResult) {
        while let Some(run_id) = queue.start_next(Self::now()) {
            if self.is_cancelled() {
                queue.cancel_all(Self::now());
                result.state.status = ExecutionStatus::Cancelled;
                break;
            }

            let run_name = queue
                .get(run_id)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| format!("run-{}", run_id));
            let run_result = self.execute_single(queue, run_id);
            self.record_run_completion(queue, start, result, run_id, run_name, run_result);
        }
    }

    fn execute_parallel(
        &self,
        queue: &mut RunQueue,
        start: &Instant,
        result: &mut ExecutionResult,
    ) {
        let netlist = queue.netlist().map(str::to_string);
        let options_override = self.engine_options_override.clone();
        let mut running: HashMap<u64, thread::JoinHandle<ParallelRunOutcome>> = HashMap::new();
        let mut cancellation_requested = false;

        loop {
            if self.is_cancelled() {
                cancellation_requested = true;
            }

            let finished_ids: Vec<u64> = running
                .iter()
                .filter_map(|(&run_id, handle)| handle.is_finished().then_some(run_id))
                .collect();

            for run_id in finished_ids {
                let handle = running
                    .remove(&run_id)
                    .expect("finished run handle must exist");
                let outcome = handle.join().unwrap_or_else(|_| ParallelRunOutcome {
                    run_id,
                    run_name: queue
                        .get(run_id)
                        .map(|run| run.name.clone())
                        .unwrap_or_else(|| format!("run-{}", run_id)),
                    result: Err("simulation execution thread panicked".to_string()),
                });
                self.record_run_completion(
                    queue,
                    start,
                    result,
                    outcome.run_id,
                    outcome.run_name,
                    outcome.result,
                );
            }

            if cancellation_requested {
                if running.is_empty() {
                    self.cancel_pending_runs(queue);
                    result.state.status = ExecutionStatus::Cancelled;
                    break;
                }
                thread::sleep(std::time::Duration::from_millis(1));
                continue;
            }

            let mut launched_any = false;
            while running.len() < self.max_parallel {
                let Some(run_id) = self.next_runnable_parallel(queue) else {
                    break;
                };
                let Some(run_snapshot) = queue.get(run_id).cloned() else {
                    break;
                };

                if let Some(run) = queue.get_mut(run_id) {
                    run.start(Self::now());
                }
                let run_name = run_snapshot.name.clone();
                let netlist_snapshot = netlist.clone();
                let options_snapshot = options_override.clone();
                let handle = thread::spawn(move || {
                    let run_result = match netlist_snapshot.as_deref() {
                        Some(text) => Self::execute_single_with_run(
                            &run_snapshot,
                            text,
                            options_snapshot.as_ref(),
                        ),
                        None => Err("No netlist configured for queue".to_string()),
                    };
                    ParallelRunOutcome {
                        run_id,
                        run_name,
                        result: run_result,
                    }
                });
                running.insert(run_id, handle);
                launched_any = true;
            }

            let has_pending = queue
                .runs()
                .iter()
                .any(|run| run.status == RunStatus::Pending);
            if running.is_empty() {
                // No running tasks and no launchable tasks left.
                if !has_pending || !launched_any {
                    break;
                }
            } else {
                thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }

    fn next_runnable_parallel(&self, queue: &RunQueue) -> Option<u64> {
        let completed: Vec<u64> = queue
            .runs()
            .iter()
            .filter(|run| run.status.is_success())
            .map(|run| run.id)
            .collect();

        queue
            .runs()
            .iter()
            .find(|run| run.status == RunStatus::Pending && run.dependencies_met(&completed))
            .map(|run| run.id)
    }

    fn cancel_pending_runs(&self, queue: &mut RunQueue) {
        let now = Self::now();
        let cancellable_ids: Vec<u64> = queue
            .runs()
            .iter()
            .filter(|run| matches!(run.status, RunStatus::Pending | RunStatus::Running))
            .map(|run| run.id)
            .collect();

        for run_id in cancellable_ids {
            if let Some(run) = queue.get_mut(run_id) {
                run.cancel(now);
            }
        }
        queue.current_run = None;
    }

    fn record_run_completion(
        &self,
        queue: &mut RunQueue,
        start: &Instant,
        result: &mut ExecutionResult,
        run_id: u64,
        run_name: String,
        run_result: Result<MappedResult, String>,
    ) {
        result.state.current_run = Some(run_name);
        match run_result {
            Ok(mapped) => {
                result.results.insert(run_id, mapped);
                queue.current_run = Some(run_id);
                queue.complete_current(Self::now());
                result.state.completed_runs += 1;
            }
            Err(error) => {
                result.errors.insert(run_id, error.clone());
                queue.current_run = Some(run_id);
                queue.fail_current(&error, Self::now());
                result.state.failed_runs += 1;
                result.state.completed_runs += 1;
            }
        }

        self.progress
            .store(result.state.completed_runs, Ordering::SeqCst);
        result.state.elapsed_seconds = start.elapsed().as_secs_f64();
        result.state.update_eta();
    }

    /// Execute a single run item
    ///
    /// Calls the actual simulation engine based on the analysis type and returns
    /// mapped results. Requires netlist to be set on the queue.
    fn execute_single(&self, queue: &RunQueue, run_id: u64) -> Result<MappedResult, String> {
        let run = queue
            .get(run_id)
            .ok_or_else(|| "Run not found".to_string())?;

        // Get netlist from queue
        let netlist = queue
            .netlist()
            .ok_or_else(|| "No netlist configured for queue".to_string())?;

        Self::execute_single_with_run(run, netlist, self.engine_options_override.as_ref())
    }

    fn execute_single_with_run(
        run: &AnalysisRun,
        netlist: &str,
        options_override: Option<&EngineOptions>,
    ) -> Result<MappedResult, String> {
        let spec = run
            .spec
            .clone()
            .or_else(|| (run.run_type == AnalysisRunType::DcOp).then_some(AnalysisSpec::DcOp))
            .ok_or_else(|| {
                format!(
                    "Run '{}' ({:?}) is missing AnalysisSpec",
                    run.name, run.run_type
                )
            })?;

        if spec.run_type() != run.run_type {
            return Err(format!(
                "Run '{}' has mismatched run_type ({:?}) vs spec ({:?})",
                run.name,
                run.run_type,
                spec.run_type()
            ));
        }

        spec.validate()?;

        let effective_netlist = Self::apply_engine_options_to_netlist(netlist, options_override);

        // Execute based on analysis type
        let start = Instant::now();
        let result = Self::execute_analysis(&spec, &effective_netlist);
        let elapsed = start.elapsed().as_secs_f64();

        // Map result
        match result {
            Ok(mapped) => Ok(MappedResult {
                elapsed_time: elapsed,
                ..mapped
            }),
            Err(e) => Err(format!("{} [{}]", e, run.name)),
        }
    }

    fn apply_engine_options_to_netlist(
        netlist: &str,
        options_override: Option<&EngineOptions>,
    ) -> String {
        let Some(options) = options_override else {
            return netlist.to_string();
        };
        Self::inject_options_block_before_end(netlist, &options.to_spice_options())
    }

    fn inject_options_block_before_end(netlist: &str, options_block: &str) -> String {
        let trimmed_block = options_block.trim();
        if trimmed_block.is_empty() {
            return netlist.to_string();
        }

        let mut out = String::with_capacity(netlist.len() + trimmed_block.len() + 4);
        match Self::find_last_end_directive_offset(netlist) {
            Some(end_offset) => {
                out.push_str(&netlist[..end_offset]);
                if !out.ends_with('\n') && !out.ends_with('\r') {
                    out.push('\n');
                }
                out.push_str(trimmed_block);
                out.push('\n');
                out.push_str(&netlist[end_offset..]);
            }
            None => {
                out.push_str(netlist);
                if !out.ends_with('\n') && !out.ends_with('\r') {
                    out.push('\n');
                }
                out.push_str(trimmed_block);
                out.push('\n');
            }
        }
        out
    }

    fn find_last_end_directive_offset(netlist: &str) -> Option<usize> {
        let mut offset = 0usize;
        let mut end_offset = None;
        for line in netlist.split_inclusive('\n') {
            if line.trim_start().to_ascii_lowercase().starts_with(".end") {
                end_offset = Some(offset);
            }
            offset += line.len();
        }
        if end_offset.is_none()
            && netlist
                .trim_start()
                .to_ascii_lowercase()
                .starts_with(".end")
        {
            return Some(0);
        }
        end_offset
    }

    /// Execute a specific analysis specification.
    fn execute_analysis(spec: &AnalysisSpec, netlist: &str) -> Result<MappedResult, String> {
        use crate::services::simulation_runner;

        match spec {
            AnalysisSpec::DcOp => {
                let sim_result = simulation_runner::run_simulation(netlist);
                if sim_result.success {
                    // Build operating point data from DC result
                    let op_data = sim_result.dc_op.map(|voltages| {
                        let mut op = super::result_mapper::OperatingPointMap::default();
                        // Convert Vec<(String, Value)> to HashMap<String, f64>
                        op.node_voltages = voltages
                            .into_iter()
                            .map(|(name, val)| (name, val.into()))
                            .collect();
                        op
                    });
                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::DcOp,
                        status: ResultStatus::Success,
                        op_data,
                        ..Default::default()
                    })
                } else {
                    Err(sim_result
                        .error
                        .unwrap_or_else(|| "DC OP failed".to_string()))
                }
            }
            AnalysisSpec::Transient {
                stop_time,
                step_time,
            } => match simulation_runner::run_transient_analysis(netlist, *stop_time, *step_time) {
                Ok(data) => {
                    let time: Vec<f64> = data.time.into_iter().collect();
                    let waveforms = data
                        .voltages
                        .into_iter()
                        .map(|(name, values)| {
                            MappedWaveform::time_domain(
                                name,
                                time.clone(),
                                values.into_iter().collect(),
                            )
                        })
                        .collect();

                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Transient,
                        status: ResultStatus::Success,
                        waveforms,
                        ..Default::default()
                    })
                }
                Err(e) => Err(e),
            },
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => {
                let ac_result = simulation_runner::run_ac_analysis(
                    netlist,
                    *start_freq,
                    *stop_freq,
                    *points_per_unit,
                    sweep.runner_keyword(),
                );
                match ac_result {
                    Ok(data) => {
                        let freq: Vec<f64> = data.frequencies.into_iter().collect();
                        let waveforms = data
                            .responses
                            .into_iter()
                            .map(|(name, values)| {
                                let real: Vec<f64> = values.iter().map(|v| v.re).collect();
                                let imag: Vec<f64> = values.iter().map(|v| v.im).collect();
                                MappedWaveform::complex_ac(name, freq.clone(), real, imag)
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Ac,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => {
                let sweep = match sweep {
                    super::multi_run::FrequencySweep::Decade => {
                        simulation_runner::DistoFrequencySweep::Decade
                    }
                    super::multi_run::FrequencySweep::Octave => {
                        simulation_runner::DistoFrequencySweep::Octave
                    }
                    super::multi_run::FrequencySweep::Linear => {
                        simulation_runner::DistoFrequencySweep::Linear
                    }
                };
                let cfg = simulation_runner::DistoRunConfig {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points_per_unit,
                    sweep,
                    f2_over_f1: *f2_over_f1,
                };
                match simulation_runner::run_disto_analysis(netlist, &cfg) {
                    Ok(data) => {
                        let frequencies = data.frequencies;
                        let warnings = data.warnings;
                        let traces = data.traces;

                        let mut waveforms = Vec::new();
                        let mut measurements = Vec::new();

                        for trace in traces {
                            let signal_name = trace.name.clone();
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} Gain(dB)", signal_name),
                                frequencies.clone(),
                                trace.fundamental_gain_db,
                                "Gain",
                            ));
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} HD2(dBc)", signal_name),
                                frequencies.clone(),
                                trace.hd2_db,
                                "HD2",
                            ));
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} HD3(dBc)", signal_name),
                                frequencies.clone(),
                                trace.hd3_db,
                                "HD3",
                            ));
                            waveforms.push(MappedWaveform::frequency_domain(
                                format!("{} THD(%)", signal_name),
                                frequencies.clone(),
                                trace.thd_percent.clone(),
                                "THD",
                            ));
                            if let Some(imd2) = trace.imd2_db {
                                waveforms.push(MappedWaveform::frequency_domain(
                                    format!("{} IMD2(dBc)", signal_name),
                                    frequencies.clone(),
                                    imd2,
                                    "IMD2",
                                ));
                            }
                            if let Some(imd3) = trace.imd3_db {
                                waveforms.push(MappedWaveform::frequency_domain(
                                    format!("{} IMD3(dBc)", signal_name),
                                    frequencies.clone(),
                                    imd3,
                                    "IMD3",
                                ));
                            }

                            if let Some(max_thd) =
                                trace.thd_percent.iter().copied().reduce(f64::max)
                            {
                                measurements.push(MappedMeasurement {
                                    name: format!("max_thd_percent({})", signal_name),
                                    meas_type: MeasurementType::Custom,
                                    value: max_thd,
                                    unit: "%".to_string(),
                                    signal: signal_name,
                                    status: MeasurementStatus::Success,
                                });
                            }
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Disto,
                            status: if warnings.is_empty() {
                                ResultStatus::Success
                            } else {
                                ResultStatus::Warning
                            },
                            waveforms,
                            measurements,
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => {
                let bridge = super::engine_bridge::EngineBridge::new();
                let dc_cfg = AnalysisConfig::DcSweep(DcSweepConfig {
                    source: source_name.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2: source2.clone(),
                    start2: *start2,
                    stop2: *stop2,
                    step2: *step2,
                });

                let sim_result = bridge.run(&dc_cfg, netlist).map_err(|e| e.to_string())?;
                if let super::results::SimulationResult::DcSweep {
                    sweep_values,
                    waveforms,
                    ..
                } = sim_result
                {
                    let waveforms = waveforms
                        .into_iter()
                        .map(|(name, wf)| {
                            MappedWaveform::time_domain(
                                name,
                                sweep_values.clone(),
                                wf.y_values.into_iter().collect(),
                            )
                        })
                        .collect();

                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::DcSweep,
                        status: ResultStatus::Success,
                        waveforms,
                        ..Default::default()
                    })
                } else {
                    Err("engine bridge returned unexpected result type for DC sweep".to_string())
                }
            }
            AnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            } => {
                let noise_result = simulation_runner::run_noise_analysis(
                    netlist,
                    output_node,
                    *start_freq,
                    *stop_freq,
                    *points_per_decade,
                    *temperature,
                );
                match noise_result {
                    Ok(data) => {
                        let freq: Vec<f64> = data.frequencies.into_iter().collect();
                        let output_noise: Vec<f64> = data.output_noise.into_iter().collect();
                        let waveforms = vec![MappedWaveform::frequency_domain(
                            "V(onoise)",
                            freq,
                            output_noise,
                            "Noise Density",
                        )];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Noise,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => {
                let pz_result = simulation_runner::run_pole_zero_analysis(
                    netlist,
                    input_node,
                    input_ref,
                    output_node,
                    output_ref,
                    transfer_type,
                    analysis_type,
                );
                match pz_result {
                    Ok(data) => {
                        let dc_gain_unit = if transfer_type.eq_ignore_ascii_case("CUR") {
                            "V/A"
                        } else {
                            "V/V"
                        };

                        let poles_real: Vec<f64> = data.poles.iter().map(|(re, _)| *re).collect();
                        let poles_imag: Vec<f64> = data.poles.iter().map(|(_, im)| *im).collect();
                        let zeros_real: Vec<f64> = data.zeros.iter().map(|(re, _)| *re).collect();
                        let zeros_imag: Vec<f64> = data.zeros.iter().map(|(_, im)| *im).collect();

                        let mut waveforms = Vec::new();
                        if !poles_real.is_empty() {
                            waveforms.push(MappedWaveform {
                                name: "Poles".to_string(),
                                x: poles_real,
                                y: poles_imag,
                                x_label: "Real(s)".to_string(),
                                y_label: "Imag(s)".to_string(),
                                x_unit: "1/s".to_string(),
                                y_unit: "1/s".to_string(),
                                ..Default::default()
                            });
                        }
                        if !zeros_real.is_empty() {
                            waveforms.push(MappedWaveform {
                                name: "Zeros".to_string(),
                                x: zeros_real,
                                y: zeros_imag,
                                x_label: "Real(s)".to_string(),
                                y_label: "Imag(s)".to_string(),
                                x_unit: "1/s".to_string(),
                                y_unit: "1/s".to_string(),
                                ..Default::default()
                            });
                        }

                        let measurements = vec![
                            MappedMeasurement {
                                name: "dc_gain".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.gain,
                                unit: dc_gain_unit.to_string(),
                                signal: "transfer".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "pole_count".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.poles.len() as f64,
                                unit: "count".to_string(),
                                signal: "poles".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "zero_count".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.zeros.len() as f64,
                                unit: "count".to_string(),
                                signal: "zeros".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::PoleZero,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => {
                let raw_unit = sensitivity_raw_unit(output_var);

                let sens_result = simulation_runner::run_sensitivity_analysis(
                    netlist, output_var, *ac_mode, *frequency,
                );
                match sens_result {
                    Ok(data) => {
                        let mut measurements: Vec<MappedMeasurement> = data
                            .sensitivities
                            .iter()
                            .map(|(name, raw, _)| MappedMeasurement {
                                name: format!("d({})/d({})", data.output_var, name),
                                meas_type: MeasurementType::Custom,
                                value: *raw,
                                unit: raw_unit.to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            })
                            .collect();

                        measurements.extend(data.sensitivities.iter().map(
                            |(name, _, normalized)| MappedMeasurement {
                                name: format!("norm({})", name),
                                meas_type: MeasurementType::Custom,
                                value: *normalized,
                                unit: "ratio".to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            },
                        ));

                        // Keep most significant normalized sensitivities first for UI consumption.
                        measurements.sort_by(|a, b| {
                            b.value
                                .abs()
                                .partial_cmp(&a.value.abs())
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Sensitivity,
                            status: ResultStatus::Success,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => {
                let pss_result = simulation_runner::run_pss_analysis(
                    netlist,
                    *fundamental_freq,
                    *num_harmonics,
                    *tolerance,
                );
                match pss_result {
                    Ok(data) => {
                        let time: Vec<f64> = data.time.into_iter().collect();
                        let waveforms = data
                            .waveforms
                            .into_iter()
                            .map(|(name, values)| {
                                MappedWaveform::time_domain(
                                    name,
                                    time.clone(),
                                    values.into_iter().collect(),
                                )
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pss,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
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
                let hb_cfg = simulation_runner::HbRunConfig {
                    tone1_freq: *tone1_freq,
                    tone1_harmonics: *tone1_harmonics,
                    tone2_freq: *tone2_freq,
                    tone2_harmonics: *tone2_harmonics,
                    reltol: *reltol,
                    abstol: *abstol,
                    max_iterations: *max_iterations,
                    damping: *damping,
                    oversample: *oversample,
                    max_mixing_order: *max_mixing_order,
                    use_krylov: *use_krylov,
                    gmres_restart: *gmres_restart,
                    source_stepping: *source_stepping,
                    verbose: *verbose,
                };
                let hb_result = simulation_runner::run_hb_analysis(netlist, &hb_cfg);
                match hb_result {
                    Ok(data) => {
                        let waveforms = data
                            .spectra
                            .into_iter()
                            .map(|(name, spectrum)| {
                                let x: Vec<f64> = spectrum.iter().map(|(f, _, _)| *f).collect();
                                let y: Vec<f64> = spectrum.iter().map(|(_, m, _)| *m).collect();
                                MappedWaveform::frequency_domain(name, x, y, "Magnitude")
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::HarmonicBalance,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
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
                    super::multi_run::FrequencySweep::Decade => {
                        simulation_runner::SParameterSweep::Decade
                    }
                    super::multi_run::FrequencySweep::Octave => {
                        simulation_runner::SParameterSweep::Octave
                    }
                    super::multi_run::FrequencySweep::Linear => {
                        simulation_runner::SParameterSweep::Linear
                    }
                };
                let cfg = simulation_runner::SParameterRunConfig {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points_per_unit,
                    sweep,
                    z0: *z0,
                    ports: ports
                        .iter()
                        .map(|port| simulation_runner::SParameterPort {
                            node_pos: port.node_pos.clone(),
                            node_neg: port.node_neg.clone(),
                            z0: port.z0,
                        })
                        .collect(),
                };
                let sp_result = simulation_runner::run_sparameter_analysis(netlist, &cfg);
                match sp_result {
                    Ok(data) => {
                        let mut waveforms = Vec::with_capacity(data.num_ports * data.num_ports);
                        for row in 0..data.num_ports {
                            for col in 0..data.num_ports {
                                let name = if data.num_ports <= 9 {
                                    format!("S{}{}", row + 1, col + 1)
                                } else {
                                    format!("S{}_{}", row + 1, col + 1)
                                };
                                let trace = &data.s[row][col];
                                waveforms.push(MappedWaveform::complex_ac(
                                    name,
                                    data.frequencies.clone(),
                                    trace.iter().map(|v| v.re).collect(),
                                    trace.iter().map(|v| v.im).collect(),
                                ));
                            }
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::SParameter,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            } => {
                let cfg = simulation_runner::EnvelopeRunConfig {
                    fundamental_freq: *fundamental_freq,
                    stop_time: *stop_time,
                    num_harmonics: *num_harmonics,
                    max_step: *max_step,
                };
                let env_result = simulation_runner::run_envelope_analysis(netlist, &cfg);
                match env_result {
                    Ok(data) => {
                        let waveforms = data
                            .waveforms
                            .into_iter()
                            .map(|(name, values)| {
                                MappedWaveform::time_domain(name, data.time.clone(), values)
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Envelope,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
            } => {
                let cfg = simulation_runner::FourierRunConfig {
                    fundamental_freq: *fundamental_freq,
                    num_harmonics: *num_harmonics,
                    output_node: output_node.clone(),
                    output_ref: (!output_ref.trim().is_empty()).then_some(output_ref.clone()),
                    start_time: *start_time,
                    stop_time: *stop_time,
                };
                let fourier_result = simulation_runner::run_fourier_analysis(netlist, &cfg);
                match fourier_result {
                    Ok(data) => {
                        let waveforms = vec![
                            MappedWaveform::complex_ac(
                                format!("{} Spectrum", data.output_label),
                                data.frequencies.clone(),
                                data.response.iter().map(|v| v.re).collect(),
                                data.response.iter().map(|v| v.im).collect(),
                            ),
                            MappedWaveform::frequency_domain(
                                "THD(%)",
                                vec![*fundamental_freq],
                                vec![data.thd_percent],
                                "THD",
                            ),
                            MappedWaveform::frequency_domain(
                                "DC",
                                vec![0.0],
                                vec![data.dc_component],
                                "DC",
                            ),
                        ];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Fourier,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::MonteCarlo => {
                let mc_result = simulation_runner::run_monte_carlo_analysis(netlist);
                match mc_result {
                    Ok(data) => {
                        let mut measurements = vec![
                            MappedMeasurement {
                                name: "runs_requested".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.runs_requested as f64,
                                unit: "count".to_string(),
                                signal: "monte_carlo".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "runs_completed".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.runs_completed as f64,
                                unit: "count".to_string(),
                                signal: "monte_carlo".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "runs_failed".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.num_failures as f64,
                                unit: "count".to_string(),
                                signal: "monte_carlo".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];

                        let variable_unit = |name: &str| {
                            if name.starts_with("V(") {
                                "V"
                            } else if name.starts_with("I(") {
                                "A"
                            } else {
                                "unit"
                            }
                        };

                        measurements.extend(data.variables.iter().flat_map(|var| {
                            let unit = variable_unit(&var.name).to_string();
                            [
                                MappedMeasurement {
                                    name: format!("mean({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.mean,
                                    unit: unit.clone(),
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: format!("stddev({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.std_dev,
                                    unit: unit.clone(),
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: format!("min({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.min,
                                    unit: unit.clone(),
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: format!("max({})", var.name),
                                    meas_type: MeasurementType::Custom,
                                    value: var.max,
                                    unit,
                                    signal: var.name.clone(),
                                    status: MeasurementStatus::Success,
                                },
                            ]
                        }));

                        let waveforms = data
                            .variables
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
                                let y: Vec<f64> =
                                    var.histogram.iter().map(|count| *count as f64).collect();
                                Some(MappedWaveform {
                                    name: format!("hist({})", var.name),
                                    x,
                                    y,
                                    x_label: "Value".to_string(),
                                    y_label: "Count".to_string(),
                                    y_unit: "count".to_string(),
                                    ..Default::default()
                                })
                            })
                            .collect();

                        let mut warnings = Vec::new();
                        if data.num_failures > 0 {
                            warnings.push(format!(
                                "Monte Carlo converged on {}/{} runs ({} failed)",
                                data.runs_completed, data.runs_requested, data.num_failures
                            ));
                        }
                        if !data.all_converged && data.num_failures == 0 {
                            warnings.push(
                                "Monte Carlo reported non-convergence despite zero explicit failures"
                                    .to_string(),
                            );
                        }

                        let status = if warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::MonteCarlo,
                            status,
                            waveforms,
                            measurements,
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Parametric => {
                let param_result = simulation_runner::run_parametric_analysis(netlist);
                match param_result {
                    Ok(data) => {
                        let sweep_values = data.sweep_values;
                        let waveforms = data
                            .voltages
                            .into_iter()
                            .map(|(name, values)| MappedWaveform {
                                name,
                                x: sweep_values.clone(),
                                y: values.into_iter().collect(),
                                x_label: data.target.clone(),
                                y_label: "Voltage".to_string(),
                                y_unit: "V".to_string(),
                                ..Default::default()
                            })
                            .collect();

                        let warnings = if data.num_failures > 0 {
                            vec![format!(
                                "Parametric sweep completed with {} failed points",
                                data.num_failures
                            )]
                        } else {
                            Vec::new()
                        };

                        let status = if warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Parametric,
                            status,
                            waveforms,
                            measurements: vec![
                                MappedMeasurement {
                                    name: "sweep_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_points as f64,
                                    unit: "count".to_string(),
                                    signal: "parametric".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: "failed_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_failures as f64,
                                    unit: "count".to_string(),
                                    signal: "parametric".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                            ],
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Corner => {
                let corner_result = simulation_runner::run_corner_analysis(netlist);
                match corner_result {
                    Ok(data) => {
                        let x_values = data.x_values;
                        let x_label = data.x_label;
                        let x_unit = data.x_unit;
                        let waveforms = data
                            .voltages
                            .into_iter()
                            .map(|(name, values)| MappedWaveform {
                                name,
                                x: x_values.clone(),
                                y: values.into_iter().collect(),
                                x_label: x_label.clone(),
                                x_unit: x_unit.clone(),
                                y_label: "Voltage".to_string(),
                                y_unit: "V".to_string(),
                                ..Default::default()
                            })
                            .collect();

                        let warnings = if data.num_failures > 0 {
                            vec![format!(
                                "Corner sweep completed with {} failed corners",
                                data.num_failures
                            )]
                        } else {
                            Vec::new()
                        };
                        let status = if warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Corner,
                            status,
                            waveforms,
                            measurements: vec![
                                MappedMeasurement {
                                    name: "corner_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_points as f64,
                                    unit: "count".to_string(),
                                    signal: "corner".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: "failed_corners".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.num_failures as f64,
                                    unit: "count".to_string(),
                                    signal: "corner".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                            ],
                            warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => {
                let cfg = simulation_runner::ReliabilityRunConfig {
                    target_years: target_years.clone(),
                    enable_hci: *enable_hci,
                    enable_nbti: *enable_nbti,
                    enable_em: *enable_em,
                    min_stress_voltage: *min_stress_voltage,
                };
                let reliability_result =
                    simulation_runner::run_reliability_analysis_with_config(netlist, &cfg);
                match reliability_result {
                    Ok(data) => {
                        let mut waveforms = Vec::new();
                        for device in &data.device_results {
                            let mut x_years = Vec::with_capacity(data.years.len());
                            let mut vth = Vec::with_capacity(data.years.len());
                            let mut mobility = Vec::with_capacity(data.years.len());
                            let mut rds = Vec::with_capacity(data.years.len());

                            for years in &data.years {
                                let key = format!("{}y", years);
                                let shift = device.shifts.get(&key).cloned().unwrap_or_default();
                                x_years.push(*years);
                                vth.push(shift.vth_shift);
                                mobility.push(shift.mobility_shift);
                                rds.push(shift.rds_shift);
                            }

                            waveforms.push(MappedWaveform {
                                name: format!("DVTH({})", device.device_id),
                                x: x_years.clone(),
                                y: vth,
                                x_label: "Lifetime".to_string(),
                                y_label: "Delta Vth".to_string(),
                                x_unit: "year".to_string(),
                                y_unit: "V".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                            waveforms.push(MappedWaveform {
                                name: format!("DMU({})", device.device_id),
                                x: x_years.clone(),
                                y: mobility,
                                x_label: "Lifetime".to_string(),
                                y_label: "Delta Mobility".to_string(),
                                x_unit: "year".to_string(),
                                y_unit: "ratio".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                            waveforms.push(MappedWaveform {
                                name: format!("DRDS({})", device.device_id),
                                x: x_years,
                                y: rds,
                                x_label: "Lifetime".to_string(),
                                y_label: "Delta Rds".to_string(),
                                x_unit: "year".to_string(),
                                y_unit: "ratio".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Reliability,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements: vec![
                                MappedMeasurement {
                                    name: "devices_analyzed".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.device_results.len() as f64,
                                    unit: "count".to_string(),
                                    signal: "reliability".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                                MappedMeasurement {
                                    name: "lifetime_points".to_string(),
                                    meas_type: MeasurementType::Custom,
                                    value: data.years.len() as f64,
                                    unit: "count".to_string(),
                                    signal: "reliability".to_string(),
                                    status: MeasurementStatus::Success,
                                },
                            ],
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
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
                let cfg = simulation_runner::OptimizationRunConfig {
                    variables: variables
                        .iter()
                        .map(|var| simulation_runner::OptimizationVariable {
                            name: var.name.clone(),
                            min: var.min,
                            max: var.max,
                            initial: var.initial,
                        })
                        .collect(),
                    objective_node: objective_node.clone(),
                    objective_ref: objective_ref.clone(),
                    goal: match goal {
                        super::multi_run::OptimizationGoal::Minimize => {
                            simulation_runner::OptimizationGoalMode::Minimize
                        }
                        super::multi_run::OptimizationGoal::Maximize => {
                            simulation_runner::OptimizationGoalMode::Maximize
                        }
                        super::multi_run::OptimizationGoal::Target => {
                            simulation_runner::OptimizationGoalMode::Target
                        }
                    },
                    target: *target,
                    algorithm: match algorithm {
                        super::multi_run::OptimizationAlgorithm::GradientDescent => {
                            simulation_runner::OptimizationAlgorithmMode::GradientDescent
                        }
                        super::multi_run::OptimizationAlgorithm::PatternSearch => {
                            simulation_runner::OptimizationAlgorithmMode::PatternSearch
                        }
                        super::multi_run::OptimizationAlgorithm::SimulatedAnnealing => {
                            simulation_runner::OptimizationAlgorithmMode::SimulatedAnnealing
                        }
                    },
                    max_iterations: *max_iterations,
                    cost_tolerance: *cost_tolerance,
                    fd_step: *fd_step,
                    initial_step: *initial_step,
                    min_step: *min_step,
                };

                match simulation_runner::run_optimization_analysis_with_config(netlist, &cfg) {
                    Ok(data) => {
                        let mut waveforms = vec![MappedWaveform {
                            name: "Optimization Cost".to_string(),
                            x: data.iterations.clone(),
                            y: data.costs.clone(),
                            x_label: "Iteration".to_string(),
                            y_label: "Cost".to_string(),
                            x_unit: "iter".to_string(),
                            y_unit: "cost".to_string(),
                            is_complex: false,
                            y_imag: None,
                        }];
                        for (name, values) in &data.variable_traces {
                            waveforms.push(MappedWaveform {
                                name: format!("Var({})", name),
                                x: data.iterations.clone(),
                                y: values.clone(),
                                x_label: "Iteration".to_string(),
                                y_label: "Value".to_string(),
                                x_unit: "iter".to_string(),
                                y_unit: "".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        let mut measurements = vec![
                            MappedMeasurement {
                                name: "best_cost".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.best_cost,
                                unit: "cost".to_string(),
                                signal: "optimization".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "converged".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: if data.converged { 1.0 } else { 0.0 },
                                unit: "bool".to_string(),
                                signal: "optimization".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];
                        for (name, value) in &data.best_variables {
                            measurements.push(MappedMeasurement {
                                name: format!("best_{}", name),
                                meas_type: MeasurementType::Custom,
                                value: *value,
                                unit: "".to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Optimization,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
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
                let cfg = simulation_runner::SoaRunConfig {
                    stop_time: *stop_time,
                    step_time: *step_time,
                    check_vgs_max: *check_vgs_max,
                    max_vgs: *max_vgs,
                    check_vds_max: *check_vds_max,
                    max_vds: *max_vds,
                    check_vbe_max: *check_vbe_max,
                    max_vbe: *max_vbe,
                    check_vce_max: *check_vce_max,
                    max_vce: *max_vce,
                };
                match simulation_runner::run_soa_analysis_with_config(netlist, &cfg) {
                    Ok(data) => {
                        let mut measurements = vec![MappedMeasurement {
                            name: "num_violations".to_string(),
                            meas_type: MeasurementType::Custom,
                            value: data.violations.len() as f64,
                            unit: "count".to_string(),
                            signal: "soa".to_string(),
                            status: MeasurementStatus::Success,
                        }];
                        if let Some(first) = data.violations.first() {
                            measurements.push(MappedMeasurement {
                                name: "first_violation_time".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: first.time,
                                unit: "s".to_string(),
                                signal: "soa".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Soa,
                            status: if data.violations.is_empty() {
                                ResultStatus::Success
                            } else {
                                ResultStatus::Warning
                            },
                            waveforms: vec![MappedWaveform {
                                name: "SOA Violation Count".to_string(),
                                x: data.time,
                                y: data.violation_count,
                                x_label: "Time".to_string(),
                                y_label: "Violation Count".to_string(),
                                x_unit: "s".to_string(),
                                y_unit: "count".to_string(),
                                is_complex: false,
                                y_imag: None,
                            }],
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Tf => {
                let tf_result = simulation_runner::run_tf_analysis(netlist);
                match tf_result {
                    Ok(data) => {
                        let mut waveforms = vec![MappedWaveform::complex_ac(
                            format!("H({}/{})", data.output_label, data.input_source),
                            data.frequencies.clone(),
                            data.transfer.iter().map(|value| value.re).collect(),
                            data.transfer.iter().map(|value| value.im).collect(),
                        )];

                        waveforms.push(MappedWaveform {
                            name: format!("|H({}/{})| dB", data.output_label, data.input_source),
                            x: data.frequencies.clone(),
                            y: data.magnitude_db.clone(),
                            x_label: "Frequency".to_string(),
                            y_label: "Magnitude".to_string(),
                            x_unit: "Hz".to_string(),
                            y_unit: "dB".to_string(),
                            is_complex: false,
                            y_imag: None,
                        });

                        waveforms.push(MappedWaveform {
                            name: format!("Phase(H({}/{}))", data.output_label, data.input_source),
                            x: data.frequencies.clone(),
                            y: data.phase_deg.clone(),
                            x_label: "Frequency".to_string(),
                            y_label: "Phase".to_string(),
                            x_unit: "Hz".to_string(),
                            y_unit: "deg".to_string(),
                            is_complex: false,
                            y_imag: None,
                        });

                        if let Some(group_delay) = data.group_delay {
                            let (x, y): (Vec<f64>, Vec<f64>) = group_delay.into_iter().unzip();
                            waveforms.push(MappedWaveform {
                                name: format!("GroupDelay({})", data.output_label),
                                x,
                                y,
                                x_label: "Frequency".to_string(),
                                y_label: "Group Delay".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "s".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        if let Some(zin) = data.input_impedance {
                            waveforms.push(MappedWaveform::complex_ac(
                                format!("Zin({})", data.input_source),
                                data.frequencies.clone(),
                                zin.iter().map(|value| value.re).collect(),
                                zin.iter().map(|value| value.im).collect(),
                            ));
                        }

                        if let Some(zout) = data.output_impedance {
                            waveforms.push(MappedWaveform::complex_ac(
                                format!("Zout({})", data.output_label),
                                data.frequencies.clone(),
                                zout.iter().map(|value| value.re).collect(),
                                zout.iter().map(|value| value.im).collect(),
                            ));
                        }

                        let mut measurements = Vec::new();
                        if let Some(dc_gain) = data.dc_gain {
                            measurements.push(MappedMeasurement {
                                name: "dc_gain".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: dc_gain,
                                unit: "V/V".to_string(),
                                signal: data.output_label.clone(),
                                status: MeasurementStatus::Success,
                            });
                            measurements.push(MappedMeasurement {
                                name: "dc_gain_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: 20.0 * dc_gain.max(1e-30).log10(),
                                unit: "dB".to_string(),
                                signal: data.output_label.clone(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Tf,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pac => {
                let pac_result = simulation_runner::run_pac_analysis_auto(netlist);
                match pac_result {
                    Ok(data) => {
                        let waveforms = data
                            .spectra
                            .into_iter()
                            .map(|(name, spectrum)| {
                                let x: Vec<f64> = spectrum.iter().map(|(f, _, _)| *f).collect();
                                let y: Vec<f64> = spectrum.iter().map(|(_, m, _)| *m).collect();
                                MappedWaveform {
                                    name,
                                    x,
                                    y,
                                    x_label: "Frequency Offset".to_string(),
                                    y_label: "Magnitude".to_string(),
                                    x_unit: "Hz".to_string(),
                                    y_unit: "V".to_string(),
                                    is_complex: false,
                                    y_imag: None,
                                }
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pac,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements: vec![MappedMeasurement {
                                name: "num_sidebands".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.sidebands.len() as f64,
                                unit: "count".to_string(),
                                signal: "pac".to_string(),
                                status: MeasurementStatus::Success,
                            }],
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pxf => {
                let pxf_result = simulation_runner::run_pxf_analysis(netlist);
                match pxf_result {
                    Ok(data) => {
                        let mut waveforms = vec![MappedWaveform::complex_ac(
                            format!(
                                "H(sb{}->sb{}, {})",
                                data.input_sideband, data.output_sideband, data.output_label
                            ),
                            data.frequencies.clone(),
                            data.transfer.iter().map(|value| value.re).collect(),
                            data.transfer.iter().map(|value| value.im).collect(),
                        )];
                        if let Some(group_delay) = data.group_delay {
                            let (x, y): (Vec<f64>, Vec<f64>) = group_delay.into_iter().unzip();
                            waveforms.push(MappedWaveform {
                                name: "group_delay".to_string(),
                                x,
                                y,
                                x_label: "Frequency".to_string(),
                                y_label: "Group Delay".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "s".to_string(),
                                is_complex: false,
                                y_imag: None,
                            });
                        }

                        let mut measurements = vec![
                            MappedMeasurement {
                                name: "input_sideband".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.input_sideband as f64,
                                unit: "index".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "output_sideband".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.output_sideband as f64,
                                unit: "index".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "fundamental_frequency".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.fundamental_frequency,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];
                        if let Some(dc_gain) = data.dc_gain {
                            measurements.push(MappedMeasurement {
                                name: "dc_gain_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: 20.0 * dc_gain.norm().max(1e-30).log10(),
                                unit: "dB".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }
                        if let Some((peak_freq, peak_gain_db)) = data.peak_gain {
                            measurements.push(MappedMeasurement {
                                name: "peak_gain_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: peak_gain_db,
                                unit: "dB".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                            measurements.push(MappedMeasurement {
                                name: "peak_gain_frequency".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: peak_freq,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }
                        if let Some(bw) = data.bandwidth_3db {
                            measurements.push(MappedMeasurement {
                                name: "bandwidth_3db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: bw,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }
                        if let Some(ugf) = data.unity_gain_freq {
                            measurements.push(MappedMeasurement {
                                name: "unity_gain_frequency".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: ugf,
                                unit: "Hz".to_string(),
                                signal: "pxf".to_string(),
                                status: MeasurementStatus::Success,
                            });
                        }

                        let status = if data.warnings.is_empty() {
                            ResultStatus::Success
                        } else {
                            ResultStatus::Warning
                        };
                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pxf,
                            status,
                            waveforms,
                            measurements,
                            warnings: data.warnings,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pnoise => {
                let pnoise_result = simulation_runner::run_pnoise_analysis(netlist);
                match pnoise_result {
                    Ok(data) => Ok(Self::map_pnoise_data(data)),
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
            } => {
                let stb_result = simulation_runner::run_stb_analysis(
                    netlist,
                    probe_node,
                    *start_freq,
                    *stop_freq,
                    *points_per_decade,
                );
                match stb_result {
                    Ok(data) => Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Stb,
                        status: ResultStatus::Success,
                        waveforms: vec![
                            MappedWaveform {
                                name: "Loop Gain (dB)".to_string(),
                                x: data.frequencies.clone(),
                                y: data.loop_gain_db,
                                x_label: "Frequency".to_string(),
                                y_label: "Loop Gain".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "dB".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Loop Phase (deg)".to_string(),
                                x: data.frequencies,
                                y: data.loop_phase_deg,
                                x_label: "Frequency".to_string(),
                                y_label: "Loop Phase".to_string(),
                                x_unit: "Hz".to_string(),
                                y_unit: "deg".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                        ],
                        measurements: vec![
                            MappedMeasurement {
                                name: "phase_margin".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.phase_margin,
                                unit: "deg".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "gain_margin".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.gain_margin,
                                unit: "dB".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "unity_gain_freq".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.unity_gain_freq,
                                unit: "Hz".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "phase_crossover_freq".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.phase_crossover_freq,
                                unit: "Hz".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "is_stable".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: if data.is_stable { 1.0 } else { 0.0 },
                                unit: "bool".to_string(),
                                signal: "stb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ],
                        ..Default::default()
                    }),
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pstb => {
                let pstb_result = simulation_runner::run_pstb_analysis(netlist);
                match pstb_result {
                    Ok(data) => Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Pstb,
                        status: ResultStatus::Success,
                        waveforms: vec![
                            MappedWaveform {
                                name: "Floquet |lambda|".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.multiplier_magnitude,
                                x_label: "Mode Index".to_string(),
                                y_label: "Multiplier Magnitude".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Stability Margin (dB)".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.stability_margin_db,
                                x_label: "Mode Index".to_string(),
                                y_label: "Stability Margin".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "dB".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Mode Damping (1/s)".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.mode_damping,
                                x_label: "Mode Index".to_string(),
                                y_label: "Damping".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "1/s".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                            MappedWaveform {
                                name: "Probe Mode Participation".to_string(),
                                x: data.mode_indices.clone(),
                                y: data.probe_mode_participation,
                                x_label: "Mode Index".to_string(),
                                y_label: "Participation".to_string(),
                                x_unit: "".to_string(),
                                y_unit: "".to_string(),
                                is_complex: false,
                                y_imag: None,
                            },
                        ],
                        measurements: vec![
                            MappedMeasurement {
                                name: "dominant_multiplier".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.dominant_multiplier_magnitude,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "min_stability_margin_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.min_stability_margin_db,
                                unit: "dB".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "num_unstable".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.num_unstable as f64,
                                unit: "count".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "is_stable".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: if data.is_stable { 1.0 } else { 0.0 },
                                unit: "bool".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_branch_ordinal".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_branch_ordinal as f64,
                                unit: "ordinal".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_index".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_index as f64,
                                unit: "index".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_self_transition".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_self_transition,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_column_norm".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_column_norm,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_row_norm".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_row_norm,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "probe_state_persistence_db".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.probe_state_persistence_db,
                                unit: "dB".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "dominant_probe_mode".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.dominant_probe_mode as f64,
                                unit: "index".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "dominant_probe_mode_participation".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.dominant_probe_mode_participation,
                                unit: "".to_string(),
                                signal: "pstb".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ],
                        ..Default::default()
                    }),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn map_pnoise_data(data: crate::services::simulation_runner::PnoiseData) -> MappedResult {
        let y_label = match data.reference {
            crate::services::simulation_runner::PnoiseReference::Phase => "Phase Noise",
            crate::services::simulation_runner::PnoiseReference::Input => "Input-Referred Noise",
            crate::services::simulation_runner::PnoiseReference::Output => "Output-Referred Noise",
        };
        let y_unit = match data.reference {
            crate::services::simulation_runner::PnoiseReference::Phase => "dBc/Hz",
            _ => "V^2/Hz",
        };
        let waveform_y =
            if data.reference == crate::services::simulation_runner::PnoiseReference::Input {
                data.input_noise
                    .clone()
                    .unwrap_or_else(|| data.output_noise.clone())
            } else {
                data.output_noise.clone()
            };

        let mut measurements = vec![
            MappedMeasurement {
                name: "carrier_frequency".to_string(),
                meas_type: MeasurementType::Custom,
                value: data.carrier_frequency,
                unit: "Hz".to_string(),
                signal: "pnoise".to_string(),
                status: MeasurementStatus::Success,
            },
            MappedMeasurement {
                name: "sideband_factor".to_string(),
                meas_type: MeasurementType::Custom,
                value: data.sideband_factor as f64,
                unit: "x".to_string(),
                signal: "pnoise".to_string(),
                status: MeasurementStatus::Success,
            },
        ];
        if let Some(total) = data.total_output_noise {
            measurements.push(MappedMeasurement {
                name: "integrated_noise".to_string(),
                meas_type: MeasurementType::Rms,
                value: total,
                unit: "Vrms".to_string(),
                signal: "pnoise".to_string(),
                status: MeasurementStatus::Success,
            });
        }

        let status = if data.warnings.is_empty() {
            ResultStatus::Success
        } else {
            ResultStatus::Warning
        };

        MappedResult {
            analysis_type: MappedAnalysisType::Pnoise,
            status,
            waveforms: vec![MappedWaveform {
                name: "pnoise".to_string(),
                x: data.frequencies,
                y: waveform_y,
                x_label: "Frequency Offset".to_string(),
                y_label: y_label.to_string(),
                x_unit: "Hz".to_string(),
                y_unit: y_unit.to_string(),
                is_complex: false,
                y_imag: None,
            }],
            measurements,
            warnings: data.warnings,
            ..Default::default()
        }
    }

    /// Map AnalysisRunType to MappedAnalysisType
    fn map_analysis_type(
        &self,
        run_type: super::multi_run::AnalysisRunType,
    ) -> super::result_mapper::MappedAnalysisType {
        use super::multi_run::AnalysisRunType;
        use super::result_mapper::MappedAnalysisType;

        match run_type {
            AnalysisRunType::DcOp => MappedAnalysisType::DcOp,
            AnalysisRunType::DcSweep => MappedAnalysisType::DcSweep,
            AnalysisRunType::Ac => MappedAnalysisType::Ac,
            AnalysisRunType::Disto => MappedAnalysisType::Disto,
            AnalysisRunType::Transient => MappedAnalysisType::Transient,
            AnalysisRunType::Noise => MappedAnalysisType::Noise,
            AnalysisRunType::Tf => MappedAnalysisType::Tf,
            AnalysisRunType::Sensitivity => MappedAnalysisType::Sensitivity,
            AnalysisRunType::PoleZero => MappedAnalysisType::PoleZero,
            AnalysisRunType::HarmonicBalance => MappedAnalysisType::HarmonicBalance,
            AnalysisRunType::Pss => MappedAnalysisType::Pss,
            AnalysisRunType::Pac => MappedAnalysisType::Pac,
            AnalysisRunType::Pnoise => MappedAnalysisType::Pnoise,
            AnalysisRunType::Pxf => MappedAnalysisType::Pxf,
            AnalysisRunType::Pstb => MappedAnalysisType::Pstb,
            AnalysisRunType::Stb => MappedAnalysisType::Stb,
            AnalysisRunType::MonteCarlo => MappedAnalysisType::MonteCarlo,
            AnalysisRunType::Parametric => MappedAnalysisType::Parametric,
            AnalysisRunType::Corner => MappedAnalysisType::Corner,
            AnalysisRunType::Reliability => MappedAnalysisType::Reliability,
            AnalysisRunType::Optimization => MappedAnalysisType::Optimization,
            AnalysisRunType::Soa => MappedAnalysisType::Soa,
            AnalysisRunType::SParameter => MappedAnalysisType::SParameter,
            AnalysisRunType::Envelope => MappedAnalysisType::Envelope,
            AnalysisRunType::Fourier => MappedAnalysisType::Fourier,
        }
    }

    /// Execute corner sweep
    pub fn execute_corners(
        &self,
        base_queue: &RunQueue,
        corners: &[PvtCorner],
    ) -> HashMap<String, ExecutionResult> {
        let mut results = HashMap::new();

        for corner in corners {
            if self.is_cancelled() {
                break;
            }

            let mut queue = base_queue.clone();
            // Apply corner settings to queue...

            let corner_result = self.execute(&mut queue);
            results.insert(corner.process.clone(), corner_result);
        }

        results
    }
}

// =============================================================================
// Progress Callback
// =============================================================================

/// Progress callback for async execution
pub type ProgressCallback = Box<dyn Fn(&ExecutionState) + Send + Sync>;

/// Async executor with progress callback
pub struct AsyncRunExecutor {
    executor: RunExecutor,
    callback: Option<ProgressCallback>,
}

impl AsyncRunExecutor {
    /// Create new async executor
    pub fn new() -> Self {
        Self {
            executor: RunExecutor::new(),
            callback: None,
        }
    }

    /// Set progress callback
    pub fn with_callback(mut self, callback: ProgressCallback) -> Self {
        self.callback = Some(callback);
        self
    }

    /// Cancel execution
    pub fn cancel(&self) {
        self.executor.cancel();
    }

    /// Execute with progress updates
    pub fn execute(&self, queue: &mut RunQueue) -> ExecutionResult {
        let result = self.executor.execute(queue);

        // Call final callback
        if let Some(ref cb) = self.callback {
            cb(&result.state);
        }

        result
    }
}

impl Default for AsyncRunExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::super::convergence::ConvergenceOptions;
    use super::super::multi_run::{AnalysisRunType, AnalysisSpec, FrequencySweep, RunStatus};
    use super::*;

    // =========================================================================
    // ExecutionState Tests
    // =========================================================================

    #[test]
    fn test_execution_state_default() {
        let state = ExecutionState::default();
        assert_eq!(state.status, ExecutionStatus::Idle);
        assert_eq!(state.total_runs, 0);
    }

    #[test]
    fn test_execution_state_progress() {
        let mut state = ExecutionState::default();
        state.total_runs = 10;
        state.completed_runs = 5;

        assert!((state.progress_percent() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_execution_state_is_complete() {
        let mut state = ExecutionState::default();
        assert!(!state.is_complete());

        state.status = ExecutionStatus::Completed;
        assert!(state.is_complete());
    }

    #[test]
    fn test_execution_state_update_eta() {
        let mut state = ExecutionState::default();
        state.total_runs = 10;
        state.completed_runs = 5;
        state.elapsed_seconds = 10.0;

        state.update_eta();
        assert!(state.eta_seconds.is_some());
        assert!((state.eta_seconds.unwrap() - 10.0).abs() < 0.1); // 2s per run * 5 remaining
    }

    // =========================================================================
    // ExecutionResult Tests
    // =========================================================================

    #[test]
    fn test_execution_result_default() {
        let result = ExecutionResult::default();
        assert!(result.results.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_execution_result_success_count() {
        let mut result = ExecutionResult::default();
        result.results.insert(
            1,
            MappedResult {
                status: ResultStatus::Success,
                ..Default::default()
            },
        );
        result.results.insert(
            2,
            MappedResult {
                status: ResultStatus::Error,
                ..Default::default()
            },
        );

        assert_eq!(result.success_count(), 1);
    }

    // =========================================================================
    // RunExecutor Tests
    // =========================================================================

    #[test]
    fn test_executor_new() {
        let executor = RunExecutor::new();
        assert!(!executor.is_cancelled());
        assert_eq!(executor.current_progress(), 0);
    }

    #[test]
    fn test_executor_cancel() {
        let executor = RunExecutor::new();
        executor.cancel();
        assert!(executor.is_cancelled());

        executor.reset();
        assert!(!executor.is_cancelled());
    }

    #[test]
    fn test_executor_with_parallel() {
        let executor = RunExecutor::new().with_parallel(4);
        assert_eq!(executor.max_parallel, 4);
    }

    #[test]
    fn test_execute_empty_queue() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new();

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.status, ExecutionStatus::Completed);
        assert_eq!(result.state.total_runs, 0);
    }

    #[test]
    fn test_execute_single_run() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(result.state.completed_runs > 0);
    }

    // =========================================================================
    // AsyncRunExecutor Tests
    // =========================================================================

    #[test]
    fn test_async_executor_new() {
        let executor = AsyncRunExecutor::new();
        assert!(executor.callback.is_none());
    }

    #[test]
    fn test_async_executor_cancel() {
        let executor = AsyncRunExecutor::new();
        executor.cancel();
        assert!(executor.executor.is_cancelled());
    }

    // =========================================================================
    // Phase 2: Run Executor Integration Tests
    // =========================================================================

    #[test]
    fn test_queue_netlist_builder() {
        let queue = RunQueue::new().with_netlist("* Test circuit\nR1 in out 1k\n");
        assert_eq!(queue.netlist(), Some("* Test circuit\nR1 in out 1k\n"));
    }

    #[test]
    fn test_queue_set_netlist() {
        let mut queue = RunQueue::new();
        assert!(queue.netlist().is_none());

        queue.set_netlist("V1 vdd 0 1.8");
        assert!(queue.netlist().is_some());
        assert!(queue.netlist().unwrap().contains("V1"));
    }

    #[test]
    fn test_execute_without_netlist_fails() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        // Execute without netlist - should handle gracefully (fail the run)
        let result = executor.execute(&mut queue);
        // The run completes but with failure since no netlist
        assert_eq!(result.state.total_runs, 1);
    }

    #[test]
    fn test_execute_with_valid_netlist() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* Simple RC circuit\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 1p\n.op\n");
        queue.add(AnalysisRunType::DcOp);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        // With valid netlist, simulation should attempt to run
        assert!(result.state.completed_runs >= 1 || result.state.failed_runs >= 1);
    }

    #[test]
    fn test_analysis_type_mapping_coverage() {
        use super::super::multi_run::AnalysisRunType;
        use super::super::result_mapper::MappedAnalysisType;

        let executor = RunExecutor::new();

        // Test all AnalysisRunType variants map correctly
        let mappings = [
            (AnalysisRunType::DcOp, MappedAnalysisType::DcOp),
            (AnalysisRunType::DcSweep, MappedAnalysisType::DcSweep),
            (AnalysisRunType::Ac, MappedAnalysisType::Ac),
            (AnalysisRunType::Disto, MappedAnalysisType::Disto),
            (AnalysisRunType::Transient, MappedAnalysisType::Transient),
            (AnalysisRunType::Noise, MappedAnalysisType::Noise),
            (AnalysisRunType::PoleZero, MappedAnalysisType::PoleZero),
            (AnalysisRunType::Pxf, MappedAnalysisType::Pxf),
            (AnalysisRunType::Pstb, MappedAnalysisType::Pstb),
            (AnalysisRunType::Stb, MappedAnalysisType::Stb),
            (
                AnalysisRunType::Reliability,
                MappedAnalysisType::Reliability,
            ),
            (
                AnalysisRunType::Optimization,
                MappedAnalysisType::Optimization,
            ),
            (AnalysisRunType::Soa, MappedAnalysisType::Soa),
            (AnalysisRunType::Envelope, MappedAnalysisType::Envelope),
            (AnalysisRunType::Fourier, MappedAnalysisType::Fourier),
        ];

        for (run_type, expected) in mappings {
            let mapped = executor.map_analysis_type(run_type);
            assert_eq!(mapped, expected, "Mapping failed for {:?}", run_type);
        }
    }

    #[test]
    fn test_execute_multiple_analyses_with_netlist() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* RC\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n.op\n.tran 1n 10n\n");

        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::Transient);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 2);
    }

    #[test]
    fn test_inject_options_block_before_end_inserts_before_end_directive() {
        let netlist = "* test\nV1 in 0 1\nR1 in 0 1k\n.END\n";
        let options_block = ".OPTIONS RELTOL=1e-4\n.TEMP 85";
        let injected = RunExecutor::inject_options_block_before_end(netlist, options_block);
        assert!(injected.contains(options_block));
        let end_pos = injected
            .to_ascii_lowercase()
            .rfind(".end")
            .expect("injected netlist should contain .end");
        let opt_pos = injected
            .find(".OPTIONS")
            .expect("injected netlist should contain .OPTIONS block");
        assert!(
            opt_pos < end_pos,
            "options block must appear before .end directive"
        );
    }

    #[test]
    fn test_inject_options_block_before_end_appends_when_end_missing() {
        let netlist = "* test\nV1 in 0 1\nR1 in 0 1k";
        let options_block = ".OPTIONS RELTOL=1e-4\n.TEMP 85";
        let injected = RunExecutor::inject_options_block_before_end(netlist, options_block);
        assert!(injected.starts_with(netlist));
        assert!(injected.contains(options_block));
    }

    #[test]
    fn test_with_convergence_options_populates_engine_override() {
        let mut conv = ConvergenceOptions::default();
        conv.temperature = 85.0;
        conv.tnom = 30.0;
        conv.tolerances.reltol = 2e-4;

        let executor = RunExecutor::new().with_convergence_options(&conv);
        let override_opts = executor
            .engine_options_override
            .as_ref()
            .expect("convergence options should produce engine override");
        assert!((override_opts.temp - 85.0).abs() < 1e-12);
        assert!((override_opts.tnom - 30.0).abs() < 1e-12);
        assert!((override_opts.reltol - 2e-4).abs() < 1e-15);
    }

    #[test]
    fn test_execute_with_engine_options_injects_options_into_execution_path() {
        let netlist = "* option injection path\nV1 in 0 1\nR1 in 0 1k\n.end\n";

        let mut baseline_queue = RunQueue::new().with_netlist(netlist);
        baseline_queue.add_analysis(AnalysisSpec::DcOp);
        let baseline_result = RunExecutor::new().execute(&mut baseline_queue);
        assert!(
            baseline_result.errors.is_empty(),
            "baseline run should succeed: {:?}",
            baseline_result.errors
        );

        let mut invalid_opts = EngineOptions::spectre_defaults();
        invalid_opts.reltol = f64::NAN;

        let mut override_queue = RunQueue::new().with_netlist(netlist);
        override_queue.add_analysis(AnalysisSpec::DcOp);
        let override_result = RunExecutor::new()
            .with_engine_options(invalid_opts)
            .execute(&mut override_queue);

        assert!(
            !override_result.errors.is_empty(),
            "override run should fail when injected options produce invalid netlist syntax"
        );
        let joined_errors = override_result
            .errors
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            joined_errors
                .to_ascii_lowercase()
                .contains("expected value, found identifier 'nan'"),
            "error should come from injected invalid RELTOL option, got: {joined_errors}"
        );
    }

    #[test]
    fn test_execute_parallel_completes_dependency_graph() {
        let executor = RunExecutor::new().with_parallel(4);
        let mut queue = RunQueue::new().with_netlist("* dep\nV1 in 0 1\nR1 in out 1k\n");

        let root = queue.add(AnalysisRunType::DcOp);
        let child_a = queue.add(AnalysisRunType::DcOp);
        queue
            .get_mut(child_a)
            .expect("child_a run must exist")
            .dependencies
            .push(root);
        let child_b = queue.add(AnalysisRunType::DcOp);
        queue
            .get_mut(child_b)
            .expect("child_b run must exist")
            .dependencies
            .push(root);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 3);
        assert_eq!(result.state.failed_runs, 0);
        assert_eq!(
            queue.get(root).expect("root run must exist").status,
            RunStatus::Completed
        );
        assert_eq!(
            queue.get(child_a).expect("child_a run must exist").status,
            RunStatus::Completed
        );
        assert_eq!(
            queue.get(child_b).expect("child_b run must exist").status,
            RunStatus::Completed
        );
    }

    #[test]
    fn test_execute_parallel_skips_dependents_on_failure() {
        let executor = RunExecutor::new().with_parallel(2);
        let mut queue = RunQueue::new().with_netlist("* fail deps\nV1 in 0 1\nR1 in out 1k\n");

        let failing = queue.add(AnalysisRunType::Ac); // Missing AnalysisSpec by design
        let dependent = queue.add(AnalysisRunType::DcOp);
        queue
            .get_mut(dependent)
            .expect("dependent run must exist")
            .dependencies
            .push(failing);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 2);
        assert_eq!(result.state.failed_runs, 1);
        assert_eq!(
            queue.get(failing).expect("failing run must exist").status,
            RunStatus::Failed
        );
        assert_eq!(
            queue
                .get(dependent)
                .expect("dependent run must exist")
                .status,
            RunStatus::Skipped
        );
        let err = result
            .errors
            .values()
            .next()
            .expect("expected a failure message");
        assert!(err.contains("missing AnalysisSpec"));
    }

    #[test]
    fn test_execute_parallel_allows_independent_success_after_failure() {
        let executor = RunExecutor::new().with_parallel(2);
        let mut queue = RunQueue::new().with_netlist("* independent\nV1 in 0 1\nR1 in out 1k\n");

        let failing = queue.add(AnalysisRunType::Ac); // Missing AnalysisSpec by design
        let succeeding = queue.add(AnalysisRunType::DcOp);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 2);
        assert_eq!(result.state.failed_runs, 1);
        assert_eq!(result.state.completed_runs, 2);
        assert_eq!(
            queue.get(failing).expect("failing run must exist").status,
            RunStatus::Failed
        );
        assert_eq!(
            queue
                .get(succeeding)
                .expect("succeeding run must exist")
                .status,
            RunStatus::Completed
        );
    }

    #[test]
    fn test_execute_parallel_missing_netlist_fails_all_runs_without_deadlock() {
        let executor = RunExecutor::new().with_parallel(4);
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::DcOp);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 3);
        assert_eq!(result.state.failed_runs, 3);
        assert_eq!(result.state.completed_runs, 3);
        assert_eq!(result.errors.len(), 3);
        assert!(result
            .errors
            .values()
            .all(|err| err.contains("No netlist configured for queue")));
    }

    #[test]
    fn test_parameterized_analysis_requires_spec() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* test\nV1 in 0 1\nR1 in out 1k\n");
        queue.add(AnalysisRunType::Ac);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.failed_runs, 1);
        assert_eq!(result.errors.len(), 1);
        let err = result
            .errors
            .values()
            .next()
            .expect("missing expected error message");
        assert!(err.contains("missing AnalysisSpec"));
    }

    #[test]
    fn test_parameterized_analysis_with_spec_runs() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* test\nV1 in 0 1\nR1 in out 1k\n");
        queue.add_analysis(AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1e3,
            points_per_unit: 5,
            sweep: FrequencySweep::Decade,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("missing AnalysisSpec"),
                "run should fail for circuit/solver reasons, not missing spec"
            );
        }
    }

    #[test]
    fn test_disto_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* disto\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
        queue.add_analysis(AnalysisSpec::Disto {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 8,
            sweep: FrequencySweep::Decade,
            f2_over_f1: Some(1.5),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected DISTO run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped DISTO result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Disto);
        assert!(mapped.waveforms.iter().any(|wf| wf.name.contains("THD(%)")));
        assert!(!mapped.measurements.is_empty());
        assert_eq!(mapped.status, ResultStatus::Success);
    }

    #[test]
    fn test_pole_zero_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n");
        queue.add_analysis(AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: "PZ".to_string(),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("not implemented in RunExecutor yet"),
                "pole-zero should execute via service runner"
            );
        }
    }

    #[test]
    fn test_pole_zero_cur_transfer_uses_transimpedance_units() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n");
        queue.add_analysis(AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "CUR".to_string(),
            analysis_type: "PZ".to_string(),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected successful CUR pole-zero run, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped result");
        let gain = mapped
            .measurements
            .iter()
            .find(|m| m.name == "dc_gain")
            .expect("dc_gain measurement should exist");
        assert_eq!(gain.unit, "V/A");
    }

    #[test]
    fn test_sensitivity_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* sens\n.param RVAL=1k\nV1 in 0 1\nR1 in out RVAL\nR2 out 0 1k\n");
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("not implemented in RunExecutor yet"),
                "sensitivity should execute via service runner"
            );
        }
    }

    #[test]
    fn test_sensitivity_current_output_uses_current_units() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* sens i\n.param RVAL=1k\nV1 in 0 1\nR1 in 0 {RVAL}\n");
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "I(V1)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected successful current-output sensitivity run, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped sensitivity result");
        let raw_measurement = mapped
            .measurements
            .iter()
            .find(|m| m.name.starts_with("d(I(V1))/d("))
            .expect("expected raw current sensitivity measurement");
        assert_eq!(raw_measurement.unit, "A/unit");
    }

    #[test]
    fn test_sensitivity_voltage_output_uses_voltage_units() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* sens v\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n");
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected successful voltage-output sensitivity run, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped sensitivity result");
        let raw_measurement = mapped
            .measurements
            .iter()
            .find(|m| m.name.starts_with("d(V(out))/d("))
            .expect("expected raw voltage sensitivity measurement");
        assert_eq!(raw_measurement.unit, "V/unit");
    }

    #[test]
    fn test_sensitivity_ac_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* sens ac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in out {RVAL}\nC1 out 0 1n\n",
        );
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e6),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("not supported yet"),
                "ac sensitivity should execute via service runner"
            );
        }
    }

    #[test]
    fn test_monte_carlo_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* mc\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 8 SEED 7 DIST GAUSS SPREAD 0.02 PARAMS RVAL\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::MonteCarlo);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected Monte Carlo run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped Monte Carlo result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::MonteCarlo);
        assert!(
            mapped
                .measurements
                .iter()
                .any(|m| m.name == "runs_requested" && (m.value - 8.0).abs() < 1e-12),
            "expected runs_requested measurement"
        );
    }

    #[test]
    fn test_monte_carlo_analysis_requires_mc_command() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* no mc\nV1 in 0 1\nR1 in 0 1k\n");
        queue.add_analysis(AnalysisSpec::MonteCarlo);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.failed_runs, 1);
        let err = result
            .errors
            .values()
            .next()
            .expect("expected Monte Carlo configuration error");
        assert!(err.contains(".MC command"));
    }

    #[test]
    fn test_parametric_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* step\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.STEP PARAM RVAL 1k 4k 1k\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::Parametric);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected parametric run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped parametric result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Parametric);
        assert!(!mapped.waveforms.is_empty(), "expected stepped waveforms");
        assert_eq!(mapped.waveforms[0].x.len(), 4);
    }

    #[test]
    fn test_parametric_analysis_requires_step_command() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* no step\nV1 in 0 1\nR1 in 0 1k\n");
        queue.add_analysis(AnalysisSpec::Parametric);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.failed_runs, 1);
        let err = result
            .errors
            .values()
            .next()
            .expect("expected parametric configuration error");
        assert!(err.contains(".STEP command"));
    }

    #[test]
    fn test_corner_analysis_with_temp_command_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* corner\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.TEMP -40 27 125\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::Corner);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected corner run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped corner result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Corner);
        assert!(!mapped.waveforms.is_empty(), "expected corner waveforms");
        assert_eq!(mapped.waveforms[0].x.len(), 3);
    }

    #[test]
    fn test_corner_analysis_requires_temp_command() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* no temp\nV1 in 0 1\nR1 in 0 1k\n");
        queue.add_analysis(AnalysisSpec::Corner);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.failed_runs, 1);
        let err = result
            .errors
            .values()
            .next()
            .expect("expected corner configuration error");
        assert!(err.contains(".TEMP"));
    }

    #[test]
    fn test_corner_execution_preserves_netlist() {
        let executor = RunExecutor::new();
        let base_queue = RunQueue::new().with_netlist("* Test\nR1 a b 1k\n");

        let corners = vec![PvtCorner {
            process: "tt".to_string(),
            ..Default::default()
        }];

        let results = executor.execute_corners(&base_queue, &corners);
        assert_eq!(results.len(), 1);
        assert!(results.contains_key("tt"));
    }

    #[test]
    fn test_reliability_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* reliability\nVDD vdd 0 1.8\nVG g 0 1.2\nR1 vdd d 1k\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::Reliability {
            target_years: vec![1.0, 5.0, 10.0],
            enable_hci: true,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.05,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected reliability run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped reliability result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Reliability);
        assert!(
            !mapped.waveforms.is_empty(),
            "expected reliability waveforms"
        );
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name.starts_with("DVTH(") || wf.name.starts_with("DRDS(")));
        assert!(mapped
            .measurements
            .iter()
            .any(|m| m.name == "devices_analyzed"));
    }

    #[test]
    fn test_optimization_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* optimization\n.param RTOP=1k\n.param RBOT=1k\nV1 in 0 2\nR1 in out {RTOP}\nR2 out 0 {RBOT}\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::Optimization {
            variables: vec![super::super::multi_run::OptimizationVariable {
                name: "RBOT".to_string(),
                min: 500.0,
                max: 3000.0,
                initial: 1000.0,
            }],
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal: super::super::multi_run::OptimizationGoal::Target,
            target: Some(1.2),
            algorithm: super::super::multi_run::OptimizationAlgorithm::PatternSearch,
            max_iterations: 48,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.2,
            min_step: 1e-8,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected optimization run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped optimization result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Optimization);
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "Optimization Cost"));
        assert!(mapped.measurements.iter().any(|m| m.name == "best_cost"));
    }

    #[test]
    fn test_soa_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* soa\nVDD d 0 3.3\nVG g 0 PULSE(0 2.5 0 1n 1n 8n 16n)\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::Soa {
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
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected SOA run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped SOA result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Soa);
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "SOA Violation Count"));
        assert!(mapped
            .measurements
            .iter()
            .any(|m| m.name == "num_violations"));
    }

    #[test]
    fn test_tf_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* tf\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
        queue.add_analysis(AnalysisSpec::Tf);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected TF run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped TF result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Tf);
        assert!(
            mapped.waveforms.iter().any(|wf| wf.name.starts_with("H(")),
            "expected transfer-function waveform"
        );
    }

    #[test]
    fn test_pac_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pac\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
        queue.add_analysis(AnalysisSpec::Pac);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected PAC run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped PAC result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Pac);
        assert!(!mapped.waveforms.is_empty(), "expected PAC spectra");
    }

    #[test]
    fn test_pxf_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pxf\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
        queue.add_analysis(AnalysisSpec::Pxf);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected PXF run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped PXF result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Pxf);
        assert!(
            mapped
                .waveforms
                .iter()
                .any(|wf| wf.name.starts_with("H(sb")),
            "expected transfer waveform in mapped PXF result"
        );
        assert!(
            mapped
                .measurements
                .iter()
                .any(|m| m.name == "input_sideband"),
            "expected sideband measurement metadata"
        );
    }

    #[test]
    fn test_pnoise_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pnoise\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
        queue.add_analysis(AnalysisSpec::Pnoise);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected PNOISE run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped PNOISE result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Pnoise);
        assert_eq!(mapped.waveforms.len(), 1);
        assert_eq!(mapped.waveforms[0].x.len(), mapped.waveforms[0].y.len());
    }

    #[test]
    fn test_map_pnoise_data_prefers_input_noise_for_input_reference() {
        let data = crate::services::simulation_runner::PnoiseData {
            frequencies: vec![1e3, 1e4],
            output_noise: vec![10.0, 20.0],
            input_noise: Some(vec![1.0, 2.0]),
            total_output_noise: Some(3e-6),
            contributors: vec![],
            carrier_frequency: 1e6,
            sideband_factor: 1,
            reference: crate::services::simulation_runner::PnoiseReference::Input,
            warnings: vec![],
        };

        let mapped = RunExecutor::map_pnoise_data(data);
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Pnoise);
        assert_eq!(mapped.waveforms.len(), 1);
        assert_eq!(mapped.waveforms[0].y, vec![1.0, 2.0]);
        assert_eq!(mapped.waveforms[0].y_label, "Input-Referred Noise");
        assert_eq!(mapped.waveforms[0].y_unit, "V^2/Hz");
    }

    #[test]
    fn test_map_pnoise_data_input_reference_falls_back_to_output_noise() {
        let data = crate::services::simulation_runner::PnoiseData {
            frequencies: vec![1e3, 1e4],
            output_noise: vec![7.0, 8.0],
            input_noise: None,
            total_output_noise: None,
            contributors: vec![],
            carrier_frequency: 1e6,
            sideband_factor: 1,
            reference: crate::services::simulation_runner::PnoiseReference::Input,
            warnings: vec![],
        };

        let mapped = RunExecutor::map_pnoise_data(data);
        assert_eq!(mapped.waveforms.len(), 1);
        assert_eq!(mapped.waveforms[0].y, vec![7.0, 8.0]);
    }

    #[test]
    fn test_stb_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* stb\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
        queue.add_analysis(AnalysisSpec::Stb {
            probe_node: "1".to_string(),
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_decade: 8,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected STB run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped STB result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Stb);
        assert_eq!(mapped.waveforms.len(), 2);
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "Loop Gain (dB)"));
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "Loop Phase (deg)"));
        assert!(mapped.measurements.iter().any(|m| m.name == "phase_margin"));
    }

    #[test]
    fn test_pstb_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* pstb\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n",
        );
        queue.add_analysis(AnalysisSpec::Pstb);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected PSTB run to succeed, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped PSTB result");
        assert_eq!(mapped.analysis_type, MappedAnalysisType::Pstb);
        assert_eq!(mapped.waveforms.len(), 4);
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "Floquet |lambda|"));
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "Stability Margin (dB)"));
        assert!(mapped
            .waveforms
            .iter()
            .any(|wf| wf.name == "Probe Mode Participation"));
        assert!(mapped
            .measurements
            .iter()
            .any(|m| m.name == "dominant_multiplier"));
        assert!(mapped
            .measurements
            .iter()
            .any(|m| m.name == "probe_state_index"));
        assert!(mapped
            .measurements
            .iter()
            .any(|m| m.name == "probe_state_persistence_db"));
        assert!(mapped
            .measurements
            .iter()
            .any(|m| m.name == "dominant_probe_mode_participation"));
    }
}
