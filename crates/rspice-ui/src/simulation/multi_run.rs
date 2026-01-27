//! Multi-Run Orchestration
//!
//! Analysis sequence queuing and automated simulation workflow management.
//!
//! # Features
//!
//! - Queue multiple analyses for sequential execution
//! - Dependency-aware ordering (e.g., DC OP before AC)
//! - Progress tracking with cancellation support
//! - Result aggregation across runs
//! - Corner sweep automation

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// =============================================================================
// Analysis Run Types
// =============================================================================

/// Type of analysis run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisRunType {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep,
    /// AC analysis
    Ac,
    /// Transient analysis
    Transient,
    /// Noise analysis
    Noise,
    /// Transfer function
    Tf,
    /// Sensitivity analysis
    Sensitivity,
    /// Pole-zero analysis
    PoleZero,
    /// Harmonic balance
    HarmonicBalance,
    /// Periodic steady-state
    Pss,
    /// Periodic AC
    Pac,
    /// Periodic noise
    Pnoise,
    /// Monte Carlo
    MonteCarlo,
    /// Parametric
    Parametric,
    /// Corner analysis
    Corner,
}

impl AnalysisRunType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisRunType::DcOp => "DC Operating Point",
            AnalysisRunType::DcSweep => "DC Sweep",
            AnalysisRunType::Ac => "AC Analysis",
            AnalysisRunType::Transient => "Transient",
            AnalysisRunType::Noise => "Noise",
            AnalysisRunType::Tf => "Transfer Function",
            AnalysisRunType::Sensitivity => "Sensitivity",
            AnalysisRunType::PoleZero => "Pole-Zero",
            AnalysisRunType::HarmonicBalance => "Harmonic Balance",
            AnalysisRunType::Pss => "PSS",
            AnalysisRunType::Pac => "PAC",
            AnalysisRunType::Pnoise => "PNoise",
            AnalysisRunType::MonteCarlo => "Monte Carlo",
            AnalysisRunType::Parametric => "Parametric",
            AnalysisRunType::Corner => "Corner",
        }
    }

    /// Whether this analysis requires a prior DC OP
    pub fn requires_dc_op(&self) -> bool {
        matches!(
            self,
            AnalysisRunType::Ac
                | AnalysisRunType::Noise
                | AnalysisRunType::Tf
                | AnalysisRunType::Sensitivity
                | AnalysisRunType::PoleZero
        )
    }

    /// Whether this analysis requires PSS first
    pub fn requires_pss(&self) -> bool {
        matches!(self, AnalysisRunType::Pac | AnalysisRunType::Pnoise)
    }

    /// Estimated relative complexity (for progress estimation)
    pub fn complexity(&self) -> u32 {
        match self {
            AnalysisRunType::DcOp => 1,
            AnalysisRunType::Tf => 1,
            AnalysisRunType::DcSweep => 5,
            AnalysisRunType::Ac => 3,
            AnalysisRunType::Transient => 10,
            AnalysisRunType::Noise => 5,
            AnalysisRunType::Sensitivity => 3,
            AnalysisRunType::PoleZero => 3,
            AnalysisRunType::HarmonicBalance => 15,
            AnalysisRunType::Pss => 20,
            AnalysisRunType::Pac => 5,
            AnalysisRunType::Pnoise => 10,
            AnalysisRunType::MonteCarlo => 50,
            AnalysisRunType::Parametric => 30,
            AnalysisRunType::Corner => 25,
        }
    }
}

/// Status of an analysis run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RunStatus {
    /// Waiting in queue
    #[default]
    Pending,
    /// Currently executing
    Running,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
    /// Skipped due to dependency failure
    Skipped,
}

impl RunStatus {
    /// Whether this status indicates completion (success or failure)
    pub fn is_done(&self) -> bool {
        matches!(
            self,
            RunStatus::Completed | RunStatus::Failed | RunStatus::Cancelled | RunStatus::Skipped
        )
    }

    /// Whether this status indicates success
    pub fn is_success(&self) -> bool {
        *self == RunStatus::Completed
    }
}

// =============================================================================
// Analysis Run
// =============================================================================

/// A single analysis run in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRun {
    /// Unique run ID
    pub id: u64,
    /// Human-readable name
    pub name: String,
    /// Analysis type
    pub run_type: AnalysisRunType,
    /// Run status
    pub status: RunStatus,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Error message if failed
    pub error: Option<String>,
    /// Dependencies (run IDs that must complete first)
    pub dependencies: Vec<u64>,
    /// Corner name (if part of corner sweep)
    pub corner: Option<String>,
    /// Iteration number (if part of parametric/MC)
    pub iteration: Option<usize>,
    /// Start timestamp
    pub start_time: Option<u64>,
    /// End timestamp
    pub end_time: Option<u64>,
}

impl Default for AnalysisRun {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            run_type: AnalysisRunType::DcOp,
            status: RunStatus::Pending,
            progress: 0,
            error: None,
            dependencies: Vec::new(),
            corner: None,
            iteration: None,
            start_time: None,
            end_time: None,
        }
    }
}

impl AnalysisRun {
    /// Create a new analysis run
    pub fn new(id: u64, run_type: AnalysisRunType) -> Self {
        Self {
            id,
            name: run_type.display_name().to_string(),
            run_type,
            ..Default::default()
        }
    }

    /// Set name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep_id: u64) -> Self {
        self.dependencies.push(dep_id);
        self
    }

    /// Set corner
    pub fn with_corner(mut self, corner: impl Into<String>) -> Self {
        self.corner = Some(corner.into());
        self
    }

    /// Set iteration
    pub fn with_iteration(mut self, iter: usize) -> Self {
        self.iteration = Some(iter);
        self
    }

    /// Mark as started
    pub fn start(&mut self, timestamp: u64) {
        self.status = RunStatus::Running;
        self.start_time = Some(timestamp);
        self.progress = 0;
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
    }

    /// Mark as completed
    pub fn complete(&mut self, timestamp: u64) {
        self.status = RunStatus::Completed;
        self.end_time = Some(timestamp);
        self.progress = 100;
    }

    /// Mark as failed
    pub fn fail(&mut self, error: impl Into<String>, timestamp: u64) {
        self.status = RunStatus::Failed;
        self.error = Some(error.into());
        self.end_time = Some(timestamp);
    }

    /// Mark as cancelled
    pub fn cancel(&mut self, timestamp: u64) {
        self.status = RunStatus::Cancelled;
        self.end_time = Some(timestamp);
    }

    /// Mark as skipped
    pub fn skip(&mut self) {
        self.status = RunStatus::Skipped;
    }

    /// Get elapsed time in seconds
    pub fn elapsed(&self) -> Option<u64> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None,
        }
    }

    /// Check if all dependencies are completed
    pub fn dependencies_met(&self, completed_ids: &[u64]) -> bool {
        self.dependencies.iter().all(|d| completed_ids.contains(d))
    }
}

// =============================================================================
// Run Queue
// =============================================================================

/// Queue of analysis runs with orchestration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunQueue {
    /// All runs in the queue
    runs: Vec<AnalysisRun>,
    /// Next run ID
    next_id: u64,
    /// Currently running run ID
    pub current_run: Option<u64>,
    /// Whether queue is paused
    pub paused: bool,
    /// Whether to stop on first error
    pub stop_on_error: bool,
    /// Total estimated complexity
    total_complexity: u32,
    /// Completed complexity
    completed_complexity: u32,
}

impl RunQueue {
    /// Create a new queue
    pub fn new() -> Self {
        Self {
            stop_on_error: true,
            ..Default::default()
        }
    }

    /// Add a run to the queue
    pub fn add(&mut self, run_type: AnalysisRunType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let run = AnalysisRun::new(id, run_type);
        self.total_complexity += run_type.complexity();
        self.runs.push(run);
        id
    }

    /// Add a run with automatic dependency resolution
    pub fn add_with_deps(&mut self, run_type: AnalysisRunType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut run = AnalysisRun::new(id, run_type);

        // Auto-add DC OP dependency if needed
        if run_type.requires_dc_op() {
            if let Some(dc_op) = self.find_run_by_type(AnalysisRunType::DcOp) {
                run.dependencies.push(dc_op);
            } else {
                // Auto-insert DC OP
                let dc_id = self.add(AnalysisRunType::DcOp);
                run.dependencies.push(dc_id);
            }
        }

        // Auto-add PSS dependency if needed
        if run_type.requires_pss() {
            if let Some(pss) = self.find_run_by_type(AnalysisRunType::Pss) {
                run.dependencies.push(pss);
            } else {
                // Auto-insert PSS with DC OP
                let pss_id = self.add_with_deps(AnalysisRunType::Pss);
                run.dependencies.push(pss_id);
            }
        }

        self.total_complexity += run_type.complexity();
        self.runs.push(run);
        id
    }

    /// Find a run by type
    fn find_run_by_type(&self, run_type: AnalysisRunType) -> Option<u64> {
        self.runs
            .iter()
            .find(|r| r.run_type == run_type)
            .map(|r| r.id)
    }

    /// Get a run by ID
    pub fn get(&self, id: u64) -> Option<&AnalysisRun> {
        self.runs.iter().find(|r| r.id == id)
    }

    /// Get mutable run by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut AnalysisRun> {
        self.runs.iter_mut().find(|r| r.id == id)
    }

    /// Get the next runnable analysis
    pub fn next_runnable(&self) -> Option<u64> {
        if self.paused {
            return None;
        }

        let completed: Vec<u64> = self
            .runs
            .iter()
            .filter(|r| r.status.is_success())
            .map(|r| r.id)
            .collect();

        self.runs
            .iter()
            .find(|r| r.status == RunStatus::Pending && r.dependencies_met(&completed))
            .map(|r| r.id)
    }

    /// Start the next runnable analysis
    pub fn start_next(&mut self, timestamp: u64) -> Option<u64> {
        if let Some(id) = self.next_runnable() {
            if let Some(run) = self.get_mut(id) {
                run.start(timestamp);
                self.current_run = Some(id);
                return Some(id);
            }
        }
        None
    }

    /// Complete the current run
    pub fn complete_current(&mut self, timestamp: u64) {
        if let Some(id) = self.current_run {
            if let Some(run) = self.get_mut(id) {
                let complexity = run.run_type.complexity();
                run.complete(timestamp);
                self.completed_complexity += complexity;
            }
            self.current_run = None;
        }
    }

    /// Fail the current run
    pub fn fail_current(&mut self, error: impl Into<String>, timestamp: u64) {
        if let Some(id) = self.current_run {
            if let Some(run) = self.get_mut(id) {
                run.fail(error, timestamp);
            }
            self.current_run = None;

            // Skip dependent runs if stop_on_error
            if self.stop_on_error {
                self.skip_dependents(id);
            }
        }
    }

    /// Skip all runs that depend on the given ID
    fn skip_dependents(&mut self, failed_id: u64) {
        let mut to_skip: VecDeque<u64> = VecDeque::new();
        to_skip.push_back(failed_id);

        while let Some(id) = to_skip.pop_front() {
            for run in &mut self.runs {
                if run.dependencies.contains(&id) && run.status == RunStatus::Pending {
                    run.skip();
                    to_skip.push_back(run.id);
                }
            }
        }
    }

    /// Cancel all pending runs
    pub fn cancel_all(&mut self, timestamp: u64) {
        if let Some(id) = self.current_run {
            if let Some(run) = self.get_mut(id) {
                run.cancel(timestamp);
            }
        }
        self.current_run = None;

        for run in &mut self.runs {
            if run.status == RunStatus::Pending {
                run.status = RunStatus::Cancelled;
            }
        }
    }

    /// Pause the queue
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume the queue
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Get overall progress (0-100)
    pub fn overall_progress(&self) -> u8 {
        if self.total_complexity == 0 {
            return 0;
        }

        let current_progress = self
            .current_run
            .and_then(|id| self.get(id))
            .map(|r| (r.progress as u32 * r.run_type.complexity()) / 100)
            .unwrap_or(0);

        let total = self.completed_complexity + current_progress;
        ((total * 100) / self.total_complexity) as u8
    }

    /// Count runs by status
    pub fn count_by_status(&self, status: RunStatus) -> usize {
        self.runs.iter().filter(|r| r.status == status).count()
    }

    /// Get all runs
    pub fn runs(&self) -> &[AnalysisRun] {
        &self.runs
    }

    /// Get pending runs
    pub fn pending(&self) -> Vec<&AnalysisRun> {
        self.runs
            .iter()
            .filter(|r| r.status == RunStatus::Pending)
            .collect()
    }

    /// Get completed runs
    pub fn completed(&self) -> Vec<&AnalysisRun> {
        self.runs
            .iter()
            .filter(|r| r.status == RunStatus::Completed)
            .collect()
    }

    /// Get failed runs
    pub fn failed(&self) -> Vec<&AnalysisRun> {
        self.runs
            .iter()
            .filter(|r| r.status == RunStatus::Failed)
            .collect()
    }

    /// Is queue empty?
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// Total run count
    pub fn len(&self) -> usize {
        self.runs.len()
    }

    /// Is queue done (all runs finished)?
    pub fn is_done(&self) -> bool {
        self.runs.iter().all(|r| r.status.is_done())
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        self.runs.clear();
        self.current_run = None;
        self.total_complexity = 0;
        self.completed_complexity = 0;
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AnalysisRunType Tests
    // =========================================================================

    #[test]
    fn test_run_type_display() {
        assert_eq!(AnalysisRunType::DcOp.display_name(), "DC Operating Point");
        assert_eq!(AnalysisRunType::Transient.display_name(), "Transient");
    }

    #[test]
    fn test_run_type_requires_dc_op() {
        assert!(AnalysisRunType::Ac.requires_dc_op());
        assert!(AnalysisRunType::Noise.requires_dc_op());
        assert!(!AnalysisRunType::Transient.requires_dc_op());
        assert!(!AnalysisRunType::DcOp.requires_dc_op());
    }

    #[test]
    fn test_run_type_requires_pss() {
        assert!(AnalysisRunType::Pac.requires_pss());
        assert!(AnalysisRunType::Pnoise.requires_pss());
        assert!(!AnalysisRunType::Ac.requires_pss());
    }

    // =========================================================================
    // AnalysisRun Tests
    // =========================================================================

    #[test]
    fn test_run_creation() {
        let run = AnalysisRun::new(1, AnalysisRunType::Ac);
        assert_eq!(run.id, 1);
        assert_eq!(run.run_type, AnalysisRunType::Ac);
        assert_eq!(run.status, RunStatus::Pending);
    }

    #[test]
    fn test_run_lifecycle() {
        let mut run = AnalysisRun::new(1, AnalysisRunType::DcOp);

        run.start(1000);
        assert_eq!(run.status, RunStatus::Running);
        assert_eq!(run.start_time, Some(1000));

        run.update_progress(50);
        assert_eq!(run.progress, 50);

        run.complete(1010);
        assert_eq!(run.status, RunStatus::Completed);
        assert_eq!(run.elapsed(), Some(10));
    }

    #[test]
    fn test_run_failure() {
        let mut run = AnalysisRun::new(1, AnalysisRunType::DcOp);
        run.start(1000);
        run.fail("Convergence failure", 1005);

        assert_eq!(run.status, RunStatus::Failed);
        assert_eq!(run.error, Some("Convergence failure".to_string()));
    }

    #[test]
    fn test_run_dependencies() {
        let run = AnalysisRun::new(3, AnalysisRunType::Ac)
            .with_dependency(1)
            .with_dependency(2);

        assert!(!run.dependencies_met(&[1]));
        assert!(run.dependencies_met(&[1, 2]));
        assert!(run.dependencies_met(&[1, 2, 3]));
    }

    // =========================================================================
    // RunQueue Tests
    // =========================================================================

    #[test]
    fn test_queue_creation() {
        let queue = RunQueue::new();
        assert!(queue.is_empty());
        assert!(queue.stop_on_error);
    }

    #[test]
    fn test_queue_add() {
        let mut queue = RunQueue::new();
        let id = queue.add(AnalysisRunType::DcOp);

        assert_eq!(queue.len(), 1);
        assert!(queue.get(id).is_some());
    }

    #[test]
    fn test_queue_auto_deps() {
        let mut queue = RunQueue::new();
        let ac_id = queue.add_with_deps(AnalysisRunType::Ac);

        // Should auto-add DC OP
        assert_eq!(queue.len(), 2);

        let ac_run = queue.get(ac_id).unwrap();
        assert!(!ac_run.dependencies.is_empty());
    }

    #[test]
    fn test_queue_execution() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::Transient);

        // Start first
        let id = queue.start_next(1000).unwrap();
        assert_eq!(queue.current_run, Some(id));

        // Complete it
        queue.complete_current(1010);
        assert!(queue.current_run.is_none());
        assert_eq!(queue.count_by_status(RunStatus::Completed), 1);

        // Start second
        let id2 = queue.start_next(1010).unwrap();
        assert_ne!(id, id2);
    }

    #[test]
    fn test_queue_dependency_ordering() {
        let mut queue = RunQueue::new();
        let dc_id = queue.add(AnalysisRunType::DcOp);
        let ac_id = queue.add(AnalysisRunType::Ac);

        // Add dependency manually
        if let Some(ac) = queue.get_mut(ac_id) {
            ac.dependencies.push(dc_id);
        }

        // Only DC should be runnable
        assert_eq!(queue.next_runnable(), Some(dc_id));

        // Complete DC
        queue.start_next(1000);
        queue.complete_current(1010);

        // Now AC is runnable
        assert_eq!(queue.next_runnable(), Some(ac_id));
    }

    #[test]
    fn test_queue_failure_skip() {
        let mut queue = RunQueue::new();
        let id1 = queue.add(AnalysisRunType::DcOp);
        let id2 = queue.add(AnalysisRunType::Ac);

        // Make AC depend on DC
        if let Some(ac) = queue.get_mut(id2) {
            ac.dependencies.push(id1);
        }

        // Start and fail DC
        queue.start_next(1000);
        queue.fail_current("Error", 1005);

        // AC should be skipped
        assert_eq!(queue.get(id2).unwrap().status, RunStatus::Skipped);
    }

    #[test]
    fn test_queue_progress() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::DcOp);

        assert_eq!(queue.overall_progress(), 0);

        queue.start_next(1000);
        queue.complete_current(1010);

        assert_eq!(queue.overall_progress(), 50);

        queue.start_next(1010);
        queue.complete_current(1020);

        assert_eq!(queue.overall_progress(), 100);
    }

    #[test]
    fn test_queue_pause_resume() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        queue.pause();
        assert!(queue.paused);
        assert!(queue.next_runnable().is_none());

        queue.resume();
        assert!(!queue.paused);
        assert!(queue.next_runnable().is_some());
    }

    #[test]
    fn test_queue_cancel_all() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::Ac);
        queue.start_next(1000);

        queue.cancel_all(1005);

        assert_eq!(queue.count_by_status(RunStatus::Cancelled), 2);
    }

    #[test]
    fn test_queue_is_done() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        assert!(!queue.is_done());

        queue.start_next(1000);
        queue.complete_current(1010);

        assert!(queue.is_done());
    }
}
