//! PAC error types.
//!
//! The PAC solve itself lives in the engine (`Engine::run_pac`), which owns
//! the circuit: it computes the harmonic-balance operating point, samples the
//! periodically time-varying small-signal linearization, and solves the
//! sideband-coupled conversion system per sweep offset
//! (`harmonic_balance::solver::periodic_ac`). This module keeps the
//! analysis-level error vocabulary shared by configuration and result types.

//=============================================================================
// PAC Error Types
//=============================================================================

/// Errors that can occur during PAC analysis
#[derive(Debug, Clone)]
pub enum PacError {
    /// Result axes or derived metadata are invalid
    InvalidResult(String),

    /// A result allocation could not be satisfied
    AllocationFailed(String),
}

impl std::fmt::Display for PacError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacError::InvalidResult(s) => write!(f, "Invalid PAC result: {s}"),
            PacError::AllocationFailed(s) => write!(f, "PAC result allocation failed: {s}"),
        }
    }
}

impl std::error::Error for PacError {}
