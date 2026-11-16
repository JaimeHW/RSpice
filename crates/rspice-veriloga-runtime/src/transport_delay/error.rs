//! Failures at the accepted-history barrier, before physical state changes.

use std::collections::TryReserveError;
use std::fmt;

/// A transport-history candidate cannot be accepted. Resource refusals retain
/// their metadata so an owning solver can distinguish them from invalid state
/// without interpreting diagnostic text. Checkpoint decoding has a separate
/// validation contract: an oversized persisted image is malformed input.
#[derive(Debug)]
pub enum DelayAcceptanceError {
    /// The candidate's values, clock, or configuration violate the history contract.
    Validation(String),
    /// Retained ordinary, left-limit, and derivative-order records after pruning.
    RecordLimit { requested: usize, limit: usize },
    /// Storage for a valid candidate could not be reserved. `bytes` is what
    /// the grown history buffers needed in all.
    Allocation {
        bytes: usize,
        source: TryReserveError,
    },
}

impl From<String> for DelayAcceptanceError {
    fn from(message: String) -> Self {
        Self::Validation(message)
    }
}

impl From<&str> for DelayAcceptanceError {
    fn from(message: &str) -> Self {
        Self::Validation(message.into())
    }
}

impl fmt::Display for DelayAcceptanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) => formatter.write_str(message),
            Self::RecordLimit { requested, limit } => write!(
                formatter,
                "delay history requires {requested} accepted records inside its configured horizon; supported limit is {limit}"
            ),
            Self::Allocation { bytes, source } => write!(
                formatter,
                "delay history allocation failed: {bytes} bytes were needed: {source}"
            ),
        }
    }
}

impl std::error::Error for DelayAcceptanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Allocation { source, .. } => Some(source),
            _ => None,
        }
    }
}
