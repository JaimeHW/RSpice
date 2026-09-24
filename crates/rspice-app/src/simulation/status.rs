//! Simulation Status and Progress Tracking
//!
//! Provides thread-safe status tracking for running simulations including:
//! - Current simulation phase (DC, transient, AC, etc.)
//! - Progress percentage
//! - Time estimates
//! - Error states

use crate::time_compat::Instant;
use std::time::Duration;

/// Availability of the execution backend, independent of a run's lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum EngineAvailability {
    Ready,
    #[default]
    Starting,
    /// An intentional stop retired the worker; an explicit run can restart it.
    Restartable,
    #[cfg(any(target_arch = "wasm32", test))]
    Unavailable(String),
}

impl EngineAvailability {
    pub(crate) fn failure_reason(&self) -> Option<&str> {
        match self {
            #[cfg(any(target_arch = "wasm32", test))]
            Self::Unavailable(reason) => Some(reason),
            _ => None,
        }
    }
}

//=============================================================================
// Simulation Status
//=============================================================================

pub use rspice_simulation_contract::progress::SimulationStatus;

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
}

impl Default for SimulationProgress {
    fn default() -> Self {
        Self {
            status: SimulationStatus::Idle,
            start_time: None,
            elapsed: Duration::ZERO,
            estimated_remaining: None,
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
