//! Simulation Status and Progress Tracking
//!
//! Provides thread-safe status tracking for running simulations including:
//! - Current simulation phase (DC, transient, AC, etc.)
//! - Progress percentage
//! - Time estimates
//! - Error states

use crate::time_compat::Instant;
use std::time::Duration;

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
    pub(crate) start_time: Option<Instant>,

    /// Total elapsed time
    pub elapsed: Duration,

    /// Estimated time remaining (if calculable)
    pub estimated_remaining: Option<Duration>,

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

    /// Fold the engine's completed fraction into the live status line, so
    /// long transients show real progress instead of a frozen t=0.
    pub fn observe_engine_fraction(&mut self, fraction: f64) {
        let fraction = fraction.clamp(0.0, 1.0);
        let updated = match &self.status {
            SimulationStatus::Transient { stop_time, .. } => Some(SimulationStatus::Transient {
                time: fraction * *stop_time,
                stop_time: *stop_time,
            }),
            SimulationStatus::DcSweep { source, .. } => Some(SimulationStatus::DcSweep {
                source: source.clone(),
                progress: fraction as f32,
            }),
            _ => None,
        };
        if let Some(status) = updated {
            self.update_status(status);
        }
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

    /// Mark as aborted
    pub fn abort(&mut self) {
        self.update_elapsed();
        self.status = SimulationStatus::Aborted {
            elapsed: self.elapsed,
        };
    }
}

//=============================================================================
// Tests
//=============================================================================
