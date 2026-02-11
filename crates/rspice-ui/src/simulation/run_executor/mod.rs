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

mod analysis;

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
        self.progress.store(0, Ordering::SeqCst);

        result.state.total_runs = queue.len();
        result.state.status = ExecutionStatus::Running;
        result.state.start_time = Self::now();
        if self.max_parallel <= 1 || queue.len() <= 1 {
            self.execute_serial(queue, &start, &mut result);
        } else {
            self.execute_parallel(queue, &start, &mut result);
        }
        self.finalize_queue_state(queue, &start, &mut result);

        // Final state
        if result.state.status == ExecutionStatus::Running {
            let has_incomplete_runs = queue
                .runs()
                .iter()
                .any(|run| matches!(run.status, RunStatus::Pending | RunStatus::Running));
            let skipped_runs = queue.count_by_status(RunStatus::Skipped);
            if has_incomplete_runs {
                result.state.status = if queue.paused {
                    ExecutionStatus::Paused
                } else {
                    ExecutionStatus::Error
                };
            } else if result.state.failed_runs > 0 || skipped_runs > 0 {
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
                let Some(handle) = running.remove(&run_id) else {
                    let run_name = queue
                        .get(run_id)
                        .map(|run| run.name.clone())
                        .unwrap_or_else(|| format!("run-{}", run_id));
                    self.record_run_completion(
                        queue,
                        start,
                        result,
                        run_id,
                        run_name,
                        Err("internal executor error: finished run handle missing".to_string()),
                    );
                    continue;
                };
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

    fn finalize_queue_state(
        &self,
        queue: &mut RunQueue,
        start: &Instant,
        result: &mut ExecutionResult,
    ) {
        self.skip_blocked_pending_runs(queue);

        for run in queue
            .runs()
            .iter()
            .filter(|run| run.status == RunStatus::Skipped)
        {
            let default = format!(
                "Run '{}' was skipped because dependencies were not satisfied",
                run.name
            );
            result
                .errors
                .entry(run.id)
                .or_insert_with(|| run.error.clone().unwrap_or(default));
        }

        result.state.completed_runs = queue
            .runs()
            .iter()
            .filter(|run| run.status.is_done())
            .count();
        result.state.failed_runs = queue.count_by_status(RunStatus::Failed);
        self.progress
            .store(result.state.completed_runs, Ordering::SeqCst);
        result.state.elapsed_seconds = start.elapsed().as_secs_f64();
        result.state.update_eta();
    }

    fn skip_blocked_pending_runs(&self, queue: &mut RunQueue) {
        let run_statuses: HashMap<u64, RunStatus> = queue
            .runs()
            .iter()
            .map(|run| (run.id, run.status))
            .collect();
        let blocked: Vec<(u64, String)> = queue
            .runs()
            .iter()
            .filter(|run| run.status == RunStatus::Pending)
            .filter_map(|run| {
                Self::blocked_dependency_reason(run, &run_statuses).map(|reason| (run.id, reason))
            })
            .collect();

        if blocked.is_empty() {
            return;
        }

        let now = Self::now();
        for (run_id, reason) in blocked {
            if let Some(run) = queue.get_mut(run_id) {
                run.skip_with_reason(reason, now);
            }
        }
    }

    fn blocked_dependency_reason(
        run: &AnalysisRun,
        run_statuses: &HashMap<u64, RunStatus>,
    ) -> Option<String> {
        let blockers: Vec<String> = run
            .dependencies
            .iter()
            .filter_map(|dependency_id| match run_statuses.get(dependency_id) {
                None => Some(format!("dependency run {} is missing", dependency_id)),
                Some(status) if status.is_success() => None,
                Some(status) => Some(format!(
                    "dependency run {} ended with status {}",
                    dependency_id,
                    Self::run_status_label(*status)
                )),
            })
            .collect();

        if blockers.is_empty() {
            None
        } else {
            Some(format!(
                "Blocked by unresolved dependencies: {}",
                blockers.join(", ")
            ))
        }
    }

    fn run_status_label(status: RunStatus) -> &'static str {
        match status {
            RunStatus::Pending => "Pending",
            RunStatus::Running => "Running",
            RunStatus::Completed => "Completed",
            RunStatus::Failed => "Failed",
            RunStatus::Cancelled => "Cancelled",
            RunStatus::Skipped => "Skipped",
        }
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
#[cfg(test)]
mod tests;
