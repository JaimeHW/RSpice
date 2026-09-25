//! Portable simulation progress vocabulary and worker wire status.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Current status of the simulation
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SimulationStatus {
    /// No simulation running
    #[default]
    Idle,

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

    /// Simulation was aborted by user
    Aborted { elapsed: Duration },
}

impl SimulationStatus {
    /// Get display name for the current status
    pub fn display_name(&self) -> &'static str {
        match self {
            SimulationStatus::Idle => "Idle",
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
            SimulationStatus::Aborted { .. } => "Aborted",
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerProgressStatus {
    Idle,
    Parsing,
    Building,
    DcOperatingPoint,
    DcSweep { source: String, progress: f32 },
    Transient { time: f64, stop_time: f64 },
    AcAnalysis { freq: f64, stop_freq: f64 },
    NoiseAnalysis { freq: f64, stop_freq: f64 },
    PoleZero,
    Sensitivity,
    PostProcessing,
    Completed,
    Aborted,
}

impl From<&SimulationStatus> for WorkerProgressStatus {
    fn from(value: &SimulationStatus) -> Self {
        match value {
            SimulationStatus::Idle => Self::Idle,
            SimulationStatus::Parsing => Self::Parsing,
            SimulationStatus::Building => Self::Building,
            SimulationStatus::DcOperatingPoint => Self::DcOperatingPoint,
            SimulationStatus::DcSweep { source, progress } => Self::DcSweep {
                source: source.clone(),
                progress: *progress,
            },
            SimulationStatus::Transient { time, stop_time } => Self::Transient {
                time: *time,
                stop_time: *stop_time,
            },
            SimulationStatus::AcAnalysis { freq, stop_freq } => Self::AcAnalysis {
                freq: *freq,
                stop_freq: *stop_freq,
            },
            SimulationStatus::NoiseAnalysis { freq, stop_freq } => Self::NoiseAnalysis {
                freq: *freq,
                stop_freq: *stop_freq,
            },
            SimulationStatus::PoleZero => Self::PoleZero,
            SimulationStatus::Sensitivity => Self::Sensitivity,
            SimulationStatus::PostProcessing => Self::PostProcessing,
            SimulationStatus::Completed { .. } => Self::Completed,
            SimulationStatus::Aborted { .. } => Self::Aborted,
        }
    }
}

impl From<WorkerProgressStatus> for SimulationStatus {
    fn from(value: WorkerProgressStatus) -> Self {
        match value {
            WorkerProgressStatus::Idle => Self::Idle,
            WorkerProgressStatus::Parsing => Self::Parsing,
            WorkerProgressStatus::Building => Self::Building,
            WorkerProgressStatus::DcOperatingPoint => Self::DcOperatingPoint,
            WorkerProgressStatus::DcSweep { source, progress } => {
                Self::DcSweep { source, progress }
            }
            WorkerProgressStatus::Transient { time, stop_time } => {
                Self::Transient { time, stop_time }
            }
            WorkerProgressStatus::AcAnalysis { freq, stop_freq } => {
                Self::AcAnalysis { freq, stop_freq }
            }
            WorkerProgressStatus::NoiseAnalysis { freq, stop_freq } => {
                Self::NoiseAnalysis { freq, stop_freq }
            }
            WorkerProgressStatus::PoleZero => Self::PoleZero,
            WorkerProgressStatus::Sensitivity => Self::Sensitivity,
            WorkerProgressStatus::PostProcessing => Self::PostProcessing,
            WorkerProgressStatus::Completed => Self::Completed {
                elapsed: std::time::Duration::ZERO,
            },
            WorkerProgressStatus::Aborted => Self::Aborted {
                elapsed: std::time::Duration::ZERO,
            },
        }
    }
}

/// Progress message sent across the browser worker boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerProgressSnapshot {
    pub id: u64,
    pub status: WorkerProgressStatus,
    pub progress: Option<f32>,
    pub elapsed_ms: u64,
}

impl WorkerProgressSnapshot {
    /// Capture the portable status and elapsed duration without retaining a
    /// host-specific live progress tracker in the wire contract.
    pub fn from_status(id: u64, status: &SimulationStatus, elapsed: Duration) -> Self {
        let elapsed_ms = elapsed.as_millis().min(u128::from(u64::MAX)) as u64;
        Self {
            id,
            status: WorkerProgressStatus::from(status),
            progress: status.progress(),
            elapsed_ms,
        }
    }
}
