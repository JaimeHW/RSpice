//! Live simulation progress and elapsed-time tracking for native and Wasm hosts.

pub use rspice_simulation_contract::progress::SimulationStatus;
use std::time::Duration;
use web_time::Instant;

/// Detailed progress information for a running simulation
#[derive(Debug, Clone)]
pub struct SimulationProgress {
    /// Current status
    pub status: SimulationStatus,

    /// When the simulation started
    start_time: Option<Instant>,

    /// Total elapsed time
    pub elapsed: Duration,
}

impl Default for SimulationProgress {
    fn default() -> Self {
        Self {
            status: SimulationStatus::Idle,
            start_time: None,
            elapsed: Duration::ZERO,
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

    /// Update status and elapsed time
    pub fn update_status(&mut self, status: SimulationStatus) {
        self.update_elapsed();
        self.status = status;
    }

    /// Mark as completed
    pub fn complete(&mut self) {
        self.update_elapsed();
        self.status = SimulationStatus::Completed {
            elapsed: self.elapsed,
        };
    }

    /// Mark as aborted
    pub fn abort(&mut self) {
        self.update_elapsed();
        self.status = SimulationStatus::Aborted {
            elapsed: self.elapsed,
        };
    }
}
