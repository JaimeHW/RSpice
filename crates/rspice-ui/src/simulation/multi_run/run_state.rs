use super::{AnalysisRunType, AnalysisSpec};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
    /// Typed analysis specification (required for parameterized analyses)
    pub spec: Option<AnalysisSpec>,
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
            spec: None,
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

    /// Attach typed analysis spec and keep run_type consistent.
    pub fn with_spec(mut self, spec: AnalysisSpec) -> Self {
        self.run_type = spec.run_type();
        self.spec = Some(spec);
        self
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

    /// Validate run metadata and analysis spec.
    pub fn validate(&self) -> Result<(), String> {
        match &self.spec {
            Some(spec) => {
                if spec.run_type() != self.run_type {
                    return Err(format!(
                        "Run '{}' has mismatched run_type ({:?}) vs spec ({:?})",
                        self.name,
                        self.run_type,
                        spec.run_type()
                    ));
                }
                spec.validate()
            }
            None => {
                if self.run_type == AnalysisRunType::DcOp {
                    Ok(())
                } else {
                    Err(format!(
                        "Run '{}' ({:?}) is missing AnalysisSpec",
                        self.name, self.run_type
                    ))
                }
            }
        }
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

    /// Mark as skipped with explicit reason and timestamp
    pub fn skip_with_reason(&mut self, reason: impl Into<String>, timestamp: u64) {
        self.status = RunStatus::Skipped;
        self.error = Some(reason.into());
        self.end_time = Some(timestamp);
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
    /// Netlist source for simulation (required for execution)
    netlist: Option<String>,
}

impl RunQueue {
    /// Create a new queue
    pub fn new() -> Self {
        Self {
            stop_on_error: true,
            ..Default::default()
        }
    }

    /// Set netlist source for simulation
    pub fn with_netlist(mut self, netlist: impl Into<String>) -> Self {
        self.netlist = Some(netlist.into());
        self
    }

    /// Set netlist source (mutable)
    pub fn set_netlist(&mut self, netlist: impl Into<String>) {
        self.netlist = Some(netlist.into());
    }

    /// Get netlist source
    pub fn netlist(&self) -> Option<&str> {
        self.netlist.as_deref()
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

    /// Add a run using an explicit typed analysis spec.
    pub fn add_analysis(&mut self, spec: AnalysisSpec) -> u64 {
        let run_type = spec.run_type();
        let id = self.next_id;
        self.next_id += 1;

        let run = AnalysisRun::new(id, run_type).with_spec(spec);
        self.total_complexity += run_type.complexity();
        self.runs.push(run);
        id
    }

    /// Set or replace the spec for an existing run.
    pub fn set_spec(&mut self, run_id: u64, spec: AnalysisSpec) -> Result<(), String> {
        let run = self
            .get_mut(run_id)
            .ok_or_else(|| format!("Run {} not found", run_id))?;
        run.run_type = spec.run_type();
        run.spec = Some(spec);
        Ok(())
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
                self.skip_dependents(id, timestamp);
            }
        }
    }

    /// Skip all runs that depend on the given ID
    fn skip_dependents(&mut self, failed_id: u64, timestamp: u64) {
        let mut to_skip: VecDeque<u64> = VecDeque::new();
        to_skip.push_back(failed_id);

        while let Some(id) = to_skip.pop_front() {
            for run in &mut self.runs {
                if run.dependencies.contains(&id) && run.status == RunStatus::Pending {
                    run.skip_with_reason(
                        format!(
                            "Skipped because dependency run {} did not complete successfully",
                            id
                        ),
                        timestamp,
                    );
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

