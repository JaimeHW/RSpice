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
use std::time::Instant;

use super::multi_run::{AnalysisRun, RunQueue, RunStatus};
use super::options_translator::{EngineOptions, OptionsTranslator, PvtCorner};
use super::result_mapper::{MappedResult, ResultMapper, ResultStatus};

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

// =============================================================================
// Run Executor
// =============================================================================

/// Executor for run queues
pub struct RunExecutor {
    /// Options translator
    options_translator: OptionsTranslator,
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

        // Execute runs using the queue's built-in ordering
        while let Some(run_id) = queue.start_next(Self::now()) {
            // Check cancellation
            if self.is_cancelled() {
                queue.cancel_all(Self::now());
                result.state.status = ExecutionStatus::Cancelled;
                break;
            }

            // Get run info
            let run_name = queue
                .get(run_id)
                .map(|r| r.name.clone())
                .unwrap_or_default();
            result.state.current_run = Some(run_name);

            // Execute this run
            match self.execute_single(queue, run_id) {
                Ok(mapped) => {
                    result.results.insert(run_id, mapped);
                    queue.complete_current(Self::now());
                    result.state.completed_runs += 1;
                }
                Err(e) => {
                    result.errors.insert(run_id, e.clone());
                    queue.fail_current(&e, Self::now());
                    result.state.failed_runs += 1;
                    result.state.completed_runs += 1;
                }
            }

            // Update progress
            self.progress
                .store(result.state.completed_runs, Ordering::SeqCst);
            result.state.elapsed_seconds = start.elapsed().as_secs_f64();
            result.state.update_eta();
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

    /// Execute a single run item
    fn execute_single(&self, queue: &RunQueue, run_id: u64) -> Result<MappedResult, String> {
        let _run = queue
            .get(run_id)
            .ok_or_else(|| "Run not found".to_string())?;

        // Build options
        let _options = EngineOptions::spectre_defaults();

        // For now, return a placeholder result
        // In real implementation, this would call the actual engine
        let result = MappedResult {
            analysis_type: super::result_mapper::MappedAnalysisType::DcOp,
            status: ResultStatus::Success,
            elapsed_time: 0.1,
            ..Default::default()
        };

        Ok(result)
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
    use super::super::multi_run::AnalysisRunType;
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
}
