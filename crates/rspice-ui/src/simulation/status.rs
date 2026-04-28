//! Simulation Status and Progress Tracking
//!
//! Provides thread-safe status tracking for running simulations including:
//! - Current simulation phase (DC, transient, AC, etc.)
//! - Progress percentage
//! - Time estimates
//! - Error states

use std::time::{Duration, Instant};

//=============================================================================
// Simulation Status
//=============================================================================

/// Current status of the simulation
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SimulationStatus {
    /// No simulation running
    #[default]
    Idle,

    /// Simulation queued but not started
    Queued,

    /// Parsing netlist
    Parsing,

    /// Building circuit matrix
    Building,

    /// DC operating point calculation
    DcOperatingPoint,

    /// DC sweep in progress
    DcSweep { source: String, progress: f32 },

    /// Transient analysis in progress
    Transient { time: f64, stop_time: f64 },

    /// AC analysis in progress
    AcAnalysis { freq: f64, stop_freq: f64 },

    /// Noise analysis in progress
    NoiseAnalysis { freq: f64, stop_freq: f64 },

    /// Pole-zero analysis
    PoleZero,

    /// Sensitivity analysis
    Sensitivity,

    /// Post-processing results
    PostProcessing,

    /// Simulation completed successfully
    Completed { elapsed: Duration },

    /// Simulation failed with error
    Failed { message: String, elapsed: Duration },

    /// Simulation was aborted by user
    Aborted { elapsed: Duration },

    /// Convergence failure at specific point
    ConvergenceFailed {
        iteration: usize,
        time_or_freq: Option<f64>,
        elapsed: Duration,
    },
}

impl SimulationStatus {
    /// Check if simulation is currently running
    pub fn is_running(&self) -> bool {
        matches!(
            self,
            SimulationStatus::Parsing
                | SimulationStatus::Building
                | SimulationStatus::DcOperatingPoint
                | SimulationStatus::DcSweep { .. }
                | SimulationStatus::Transient { .. }
                | SimulationStatus::AcAnalysis { .. }
                | SimulationStatus::NoiseAnalysis { .. }
                | SimulationStatus::PoleZero
                | SimulationStatus::Sensitivity
                | SimulationStatus::PostProcessing
        )
    }

    /// Check if simulation is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            SimulationStatus::Idle
                | SimulationStatus::Completed { .. }
                | SimulationStatus::Failed { .. }
                | SimulationStatus::Aborted { .. }
                | SimulationStatus::ConvergenceFailed { .. }
        )
    }

    /// Check if simulation completed successfully
    pub fn is_success(&self) -> bool {
        matches!(self, SimulationStatus::Completed { .. })
    }

    /// Get display name for the current status
    pub fn display_name(&self) -> &'static str {
        match self {
            SimulationStatus::Idle => "Idle",
            SimulationStatus::Queued => "Queued",
            SimulationStatus::Parsing => "Parsing Netlist",
            SimulationStatus::Building => "Building Circuit",
            SimulationStatus::DcOperatingPoint => "DC Operating Point",
            SimulationStatus::DcSweep { .. } => "DC Sweep",
            SimulationStatus::Transient { .. } => "Transient Analysis",
            SimulationStatus::AcAnalysis { .. } => "AC Analysis",
            SimulationStatus::NoiseAnalysis { .. } => "Noise Analysis",
            SimulationStatus::PoleZero => "Pole-Zero Analysis",
            SimulationStatus::Sensitivity => "Sensitivity Analysis",
            SimulationStatus::PostProcessing => "Post-Processing",
            SimulationStatus::Completed { .. } => "Completed",
            SimulationStatus::Failed { .. } => "Failed",
            SimulationStatus::Aborted { .. } => "Aborted",
            SimulationStatus::ConvergenceFailed { .. } => "Convergence Failed",
        }
    }

    /// Get progress percentage (0.0 to 1.0) if applicable
    pub fn progress(&self) -> Option<f32> {
        match self {
            SimulationStatus::DcSweep { progress, .. } => Some(*progress),
            SimulationStatus::Transient { time, stop_time } => {
                if *stop_time > 0.0 {
                    Some((*time / *stop_time) as f32)
                } else {
                    None
                }
            }
            SimulationStatus::AcAnalysis { freq, stop_freq } => {
                if *stop_freq > 0.0 && *freq > 0.0 {
                    // Log scale progress
                    let log_progress = freq.log10() / stop_freq.log10();
                    Some(log_progress as f32)
                } else {
                    None
                }
            }
            SimulationStatus::NoiseAnalysis { freq, stop_freq } => {
                if *stop_freq > 0.0 && *freq > 0.0 {
                    let log_progress = freq.log10() / stop_freq.log10();
                    Some(log_progress as f32)
                } else {
                    None
                }
            }
            SimulationStatus::Completed { .. } => Some(1.0),
            _ => None,
        }
    }
}

//=============================================================================
// Simulation Progress
//=============================================================================

/// Detailed progress information for a running simulation
#[derive(Debug, Clone)]
pub struct SimulationProgress {
    /// Current status
    pub status: SimulationStatus,

    /// When the simulation started
    pub start_time: Option<Instant>,

    /// Total elapsed time
    pub elapsed: Duration,

    /// Estimated time remaining (if calculable)
    pub estimated_remaining: Option<Duration>,

    /// Number of Newton-Raphson iterations
    pub nr_iterations: usize,

    /// Number of timesteps (for transient)
    pub timesteps: usize,

    /// Number of frequency points (for AC/Noise)
    pub freq_points: usize,

    /// Current memory usage in bytes
    pub memory_bytes: usize,

    /// Last message from the solver
    pub message: Option<String>,
}

impl Default for SimulationProgress {
    fn default() -> Self {
        Self {
            status: SimulationStatus::Idle,
            start_time: None,
            elapsed: Duration::ZERO,
            estimated_remaining: None,
            nr_iterations: 0,
            timesteps: 0,
            freq_points: 0,
            memory_bytes: 0,
            message: None,
        }
    }
}

impl SimulationProgress {
    /// Create a new progress tracker, starting now
    pub fn new() -> Self {
        Self {
            start_time: Some(Instant::now()),
            ..Default::default()
        }
    }

    /// Start the simulation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.status = SimulationStatus::Parsing;
    }

    /// Update elapsed time
    pub fn update_elapsed(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed = start.elapsed();
        }
    }

    /// Update status and recalculate ETA
    pub fn update_status(&mut self, status: SimulationStatus) {
        self.update_elapsed();

        // Calculate ETA based on progress
        if let Some(progress) = status.progress()
            && progress > 0.01
        {
            let elapsed_secs = self.elapsed.as_secs_f64();
            let total_estimated = elapsed_secs / progress as f64;
            let remaining = total_estimated - elapsed_secs;
            if remaining > 0.0 {
                self.estimated_remaining = Some(Duration::from_secs_f64(remaining));
            }
        }

        self.status = status;
    }

    /// Mark as completed
    pub fn complete(&mut self) {
        self.update_elapsed();
        self.status = SimulationStatus::Completed {
            elapsed: self.elapsed,
        };
        self.estimated_remaining = Some(Duration::ZERO);
    }

    /// Mark as failed
    pub fn fail(&mut self, message: String) {
        self.update_elapsed();
        self.status = SimulationStatus::Failed {
            message: message.clone(),
            elapsed: self.elapsed,
        };
        self.message = Some(message);
    }

    /// Mark as aborted
    pub fn abort(&mut self) {
        self.update_elapsed();
        self.status = SimulationStatus::Aborted {
            elapsed: self.elapsed,
        };
    }

    /// Get progress as percentage (0-100)
    pub fn progress_percent(&self) -> Option<f32> {
        self.status.progress().map(|p| p * 100.0)
    }

    /// Format elapsed time as string
    pub fn elapsed_string(&self) -> String {
        format_duration(self.elapsed)
    }

    /// Format ETA as string
    pub fn eta_string(&self) -> Option<String> {
        self.estimated_remaining.map(format_duration)
    }
}

/// Format a duration as a human-readable string
fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    if secs < 60 {
        format!("{:.1}s", d.as_secs_f64())
    } else if secs < 3600 {
        format!("{}m {:02}s", secs / 60, secs % 60)
    } else {
        format!(
            "{}h {:02}m {:02}s",
            secs / 3600,
            (secs % 3600) / 60,
            secs % 60
        )
    }
}

//=============================================================================
// Tests
//=============================================================================
