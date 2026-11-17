//! Portable failures of the store a run writes its results to.

use serde::{Deserialize, Serialize};
use std::fmt;

use rspice_core::SimulationErrorCategory;
use rspice_core::result_sink::SinkStop;

use crate::run_set::format_bytes;

/// Why a run's results could not be kept. Nothing in the design is at fault: the volume, or in a
/// browser the memory, that holds the run's results store is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultStorageFailure {
    /// Refused before solving: the run writes at least `rows` rows, `needed` bytes, and the volume
    /// holding `folder` has `available` bytes free.
    Insufficient {
        rows: u64,
        needed: u64,
        available: u64,
        folder: String,
    },
    /// The storage filled before the run kept a row.
    Full {
        available: Option<u64>,
        folder: Option<String>,
    },
    /// Memory could not grow to hold what the store buffers.
    MemoryFull { bytes: Option<u64> },
    /// The store could not be created, written or read back, in the host's words.
    Io { cause: String },
}

impl ResultStorageFailure {
    /// The failure a results sink's refusal is, with the folder holding the store where the
    /// caller knows it.
    pub fn from_sink_stop(stop: SinkStop, folder: Option<String>) -> Self {
        match stop {
            SinkStop::DiskFull { available, .. } => Self::Full { available, folder },
            SinkStop::MemoryFull { bytes } => Self::MemoryFull { bytes },
            SinkStop::Io(cause) => Self::Io { cause },
        }
    }

    /// The stable error code, the one the engine gives the same stop.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Insufficient { .. } | Self::Full { .. } => "resource_limit",
            Self::MemoryFull { .. } => "allocation_failed",
            Self::Io { .. } => "output_commit_failed",
        }
    }

    /// The failure class, the one the engine gives the same stop.
    pub const fn category(&self) -> SimulationErrorCategory {
        match self {
            Self::Insufficient { .. } | Self::Full { .. } | Self::MemoryFull { .. } => {
                SimulationErrorCategory::ResourceLimit
            }
            Self::Io { .. } => SimulationErrorCategory::OutputCommit,
        }
    }
}

impl fmt::Display for ResultStorageFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Insufficient {
                rows,
                needed,
                available,
                folder,
            } => write!(
                formatter,
                "Results storage full: the run writes at least {rows} rows ({}) and the volume \
                 holding {folder} has {} free; it was refused before solving",
                format_bytes(*needed),
                format_bytes(*available)
            ),
            Self::Full { available, folder } => {
                formatter.write_str("Results storage full: ")?;
                match folder {
                    Some(folder) => write!(formatter, "the volume holding {folder}")?,
                    None => formatter.write_str("the results storage")?,
                }
                formatter.write_str(" filled before the run kept a row")?;
                match available {
                    Some(available) => write!(formatter, " ({} free)", format_bytes(*available)),
                    None => Ok(()),
                }
            }
            Self::MemoryFull { bytes: Some(bytes) } => write!(
                formatter,
                "Results storage full: results memory is full ({})",
                format_bytes(*bytes)
            ),
            Self::MemoryFull { bytes: None } => {
                formatter.write_str("Results storage full: results memory is full")
            }
            Self::Io { cause } => write!(formatter, "Results storage error: {cause}"),
        }
    }
}

impl std::error::Error for ResultStorageFailure {}
