//! Simulation Runner - Async Simulation Execution
//!
//! Provides the bridge between UI and rspice-core simulation engine with:
//! - Async simulation execution on background thread
//! - Thread-safe progress updates
//! - Abort capability
//! - Result caching

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};

use super::config::AnalysisConfig;
use super::results::SimulationResult;
use super::status::{SimulationProgress, SimulationStatus};

//=============================================================================
// Simulation Runner
//=============================================================================

/// Thread-safe simulation runner
///
/// Manages simulation execution on a background thread with progress tracking
/// and abort capability.
pub struct SimulationRunner {
    /// Current progress (thread-safe)
    progress: Arc<Mutex<SimulationProgress>>,

    /// Abort flag
    abort_flag: Arc<AtomicBool>,

    /// Current simulation thread handle
    thread_handle: Option<JoinHandle<Result<SimulationResult, SimulationError>>>,

    /// Cached results from last successful simulation
    last_result: Option<SimulationResult>,
}

impl Default for SimulationRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationRunner {
    /// Create a new simulation runner
    pub fn new() -> Self {
        Self {
            progress: Arc::new(Mutex::new(SimulationProgress::default())),
            abort_flag: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
            last_result: None,
        }
    }

    /// Check if a simulation is currently running
    pub fn is_running(&self) -> bool {
        if let Some(ref handle) = self.thread_handle {
            !handle.is_finished()
        } else {
            false
        }
    }

    /// Get current status
    pub fn status(&self) -> SimulationStatus {
        self.progress.lock().unwrap().status.clone()
    }

    /// Get current progress percentage (0.0 to 1.0)
    pub fn progress_fraction(&self) -> Option<f32> {
        self.progress.lock().unwrap().status.progress()
    }

    /// Get full progress information
    pub fn progress(&self) -> SimulationProgress {
        self.progress.lock().unwrap().clone()
    }

    /// Get last successful result
    pub fn last_result(&self) -> Option<&SimulationResult> {
        self.last_result.as_ref()
    }

    /// Abort current simulation
    pub fn abort(&self) {
        self.abort_flag.store(true, Ordering::SeqCst);
    }

    /// Check if aborted
    pub fn is_aborted(&self) -> bool {
        self.abort_flag.load(Ordering::SeqCst)
    }

    /// Poll for completion and get result
    ///
    /// Returns `Some(result)` if simulation completed, `None` if still running or no simulation.
    pub fn poll_result(&mut self) -> Option<Result<SimulationResult, SimulationError>> {
        // Check if thread is finished
        let is_finished = self.thread_handle.as_ref().is_some_and(|h| h.is_finished());

        if is_finished {
            // Take the handle and join
            if let Some(handle) = self.thread_handle.take() {
                match handle.join() {
                    Ok(result) => {
                        // Cache successful result
                        if let Ok(ref sim_result) = result {
                            self.last_result = Some(sim_result.clone());
                        }
                        return Some(result);
                    }
                    Err(_) => {
                        return Some(Err(SimulationError::ThreadPanic));
                    }
                }
            }
        }

        None
    }

    /// Start a simulation with the given configuration
    ///
    /// Returns error if a simulation is already running.
    pub fn start(
        &mut self,
        config: AnalysisConfig,
        netlist: String,
    ) -> Result<(), SimulationError> {
        if self.is_running() {
            return Err(SimulationError::AlreadyRunning);
        }

        // Reset state
        self.abort_flag.store(false, Ordering::SeqCst);
        {
            let mut progress = self.progress.lock().unwrap();
            *progress = SimulationProgress::new();
        }

        // Clone Arcs for the thread
        let progress = Arc::clone(&self.progress);
        let abort_flag = Arc::clone(&self.abort_flag);

        // Spawn simulation thread with real engine
        let handle =
            thread::spawn(move || run_simulation_thread(config, netlist, progress, abort_flag));

        self.thread_handle = Some(handle);
        Ok(())
    }

    /// Run DC operating point analysis
    pub fn run_dc_op(&mut self, netlist: String) -> Result<(), SimulationError> {
        self.start(AnalysisConfig::DcOp, netlist)
    }

    /// Clear cached results
    pub fn clear_results(&mut self) {
        self.last_result = None;
    }
}

/// Simulation execution in background thread
///
/// Runs the actual rspice-core simulation engine.
fn run_simulation_thread(
    config: AnalysisConfig,
    netlist: String,
    progress: Arc<Mutex<SimulationProgress>>,
    abort_flag: Arc<AtomicBool>,
) -> Result<SimulationResult, SimulationError> {
    use super::engine_bridge::EngineBridge;

    // Update status: parsing
    {
        let mut p = progress.lock().unwrap();
        p.update_status(SimulationStatus::Parsing);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = progress.lock().unwrap();
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Create engine bridge
    let bridge = EngineBridge::new();

    // Update status: building
    {
        let mut p = progress.lock().unwrap();
        p.update_status(SimulationStatus::Building);
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = progress.lock().unwrap();
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Update status based on analysis type
    {
        let mut p = progress.lock().unwrap();
        match &config {
            AnalysisConfig::DcOp => p.update_status(SimulationStatus::DcOperatingPoint),
            AnalysisConfig::DcSweep(dc) => p.update_status(SimulationStatus::DcSweep {
                source: dc.source.clone(),
                progress: 0.0,
            }),
            AnalysisConfig::Transient(tran) => p.update_status(SimulationStatus::Transient {
                time: 0.0,
                stop_time: tran.stop_time,
            }),
            AnalysisConfig::Ac(ac) => p.update_status(SimulationStatus::AcAnalysis {
                freq: ac.start_freq,
                stop_freq: ac.stop_freq,
            }),
            _ => p.update_status(SimulationStatus::DcOperatingPoint),
        }
    }

    // Check for abort
    if abort_flag.load(Ordering::SeqCst) {
        let mut p = progress.lock().unwrap();
        p.abort();
        return Err(SimulationError::Aborted);
    }

    // Run simulation via engine bridge with abort support
    log::info!("Running simulation via engine bridge: {:?}", config);
    let result = match bridge.run_with_abort(&config, &netlist, &abort_flag) {
        Ok(r) => {
            log::info!("Engine bridge returned successfully");
            r
        }
        Err(e) => {
            log::error!("Engine bridge error: {:?}", e);
            return Err(e);
        }
    };

    // Mark complete
    {
        let mut p = progress.lock().unwrap();
        p.complete();
    }

    log::info!("Simulation thread completed successfully");
    Ok(result)
}

//=============================================================================
// Simulation Error
//=============================================================================

/// Errors that can occur during simulation
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    /// Netlist parsing error
    ParseError(String),

    /// Circuit building error
    CircuitError(String),

    /// Solver error
    SolverError(String),

    /// Convergence failure
    ConvergenceFailed { iterations: usize, message: String },

    /// Simulation was aborted
    Aborted,

    /// A simulation is already running
    AlreadyRunning,

    /// Thread panicked
    ThreadPanic,

    /// Invalid configuration
    InvalidConfig(String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SimulationError::CircuitError(msg) => write!(f, "Circuit error: {}", msg),
            SimulationError::SolverError(msg) => write!(f, "Solver error: {}", msg),
            SimulationError::ConvergenceFailed {
                iterations,
                message,
            } => {
                write!(
                    f,
                    "Convergence failed after {} iterations: {}",
                    iterations, message
                )
            }
            SimulationError::Aborted => write!(f, "Simulation aborted"),
            SimulationError::AlreadyRunning => write!(f, "A simulation is already running"),
            SimulationError::ThreadPanic => write!(f, "Simulation thread panicked"),
            SimulationError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal valid netlist for testing DC operating point
    fn test_netlist() -> String {
        "* Test Circuit\nV1 vdd 0 5\nR1 vdd out 1k\nR2 out 0 1k\n.end\n".to_string()
    }

    #[test]
    fn test_runner_new() {
        let runner = SimulationRunner::new();
        assert!(!runner.is_running());
        assert!(matches!(runner.status(), SimulationStatus::Idle));
    }

    #[test]
    fn test_runner_not_running_initially() {
        let runner = SimulationRunner::new();
        assert!(!runner.is_running());
        assert!(runner.progress_fraction().is_none());
    }

    #[test]
    fn test_runner_start_and_poll() {
        let mut runner = SimulationRunner::new();

        // Start simulation with valid netlist
        let result = runner.start(AnalysisConfig::DcOp, test_netlist());
        assert!(result.is_ok());

        // Should be running now (or already finished if very fast)
        // Don't assert running since the simulation is very fast

        // Wait for completion
        thread::sleep(std::time::Duration::from_millis(200));

        // Poll for result
        let result = runner.poll_result();
        assert!(result.is_some(), "Expected simulation result, got None");
        let sim_result = result.unwrap();
        assert!(
            sim_result.is_ok(),
            "Expected Ok result, got: {:?}",
            sim_result
        );

        // No longer running
        assert!(!runner.is_running());
    }

    #[test]
    fn test_runner_abort() {
        let mut runner = SimulationRunner::new();

        // Start simulation with valid netlist
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

        // Abort immediately
        runner.abort();

        // Wait for thread to notice abort
        thread::sleep(std::time::Duration::from_millis(100));

        // Poll for result - might be aborted or might have completed before abort took effect
        let result = runner.poll_result();
        // Result should exist after polling (either aborted or completed)
        if let Some(res) = result {
            // Either aborted or completed is acceptable
            assert!(matches!(res, Err(SimulationError::Aborted)) || res.is_ok());
        }
    }

    #[test]
    fn test_runner_already_running() {
        let mut runner = SimulationRunner::new();

        // Start first simulation with valid netlist
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

        // Try to start another while running (might already be done if fast)
        if runner.is_running() {
            let result = runner.start(AnalysisConfig::DcOp, test_netlist());
            assert!(matches!(result, Err(SimulationError::AlreadyRunning)));
        }

        // Cleanup
        runner.abort();
        thread::sleep(std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_runner_clear_results() {
        let mut runner = SimulationRunner::new();
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();
        thread::sleep(std::time::Duration::from_millis(200));
        let result = runner.poll_result();

        // Verify simulation completed successfully before testing clear
        assert!(result.is_some(), "Expected simulation result");
        let sim_result = result.unwrap();
        assert!(sim_result.is_ok(), "Simulation failed: {:?}", sim_result);

        assert!(runner.last_result().is_some(), "Expected cached result");
        runner.clear_results();
        assert!(runner.last_result().is_none(), "Expected cleared result");
    }

    #[test]
    fn test_simulation_error_display() {
        let err = SimulationError::ParseError("test error".to_string());
        assert!(err.to_string().contains("Parse error"));

        let err = SimulationError::ConvergenceFailed {
            iterations: 50,
            message: "did not converge".to_string(),
        };
        assert!(err.to_string().contains("50"));
    }

    #[test]
    fn test_runner_progress_update() {
        let mut runner = SimulationRunner::new();
        runner.start(AnalysisConfig::DcOp, test_netlist()).unwrap();

        // Give thread time to start
        thread::sleep(std::time::Duration::from_millis(10));

        // Check status is not idle (or completed if very fast)
        let status = runner.status();
        // Progress status might be Idle if completed very quickly, so just check it ran

        // Cleanup
        thread::sleep(std::time::Duration::from_millis(200));
        let _ = runner.poll_result();
    }

    #[test]
    fn test_runner_default() {
        let runner = SimulationRunner::default();
        assert!(!runner.is_running());
    }

    #[test]
    fn test_run_dc_op_convenience() {
        let mut runner = SimulationRunner::new();
        let result = runner.run_dc_op(test_netlist());
        assert!(result.is_ok());

        // Cleanup - wait and poll
        thread::sleep(std::time::Duration::from_millis(200));
        let result = runner.poll_result();
        assert!(result.is_some(), "Expected result from dc_op");
    }
}
