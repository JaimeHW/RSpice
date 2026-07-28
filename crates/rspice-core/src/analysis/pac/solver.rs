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
    /// No PSS solution provided
    NoPssSolution,

    /// PSS solution has no period information
    InvalidPssSolution(String),

    /// Invalid configuration
    InvalidConfig(String),

    /// Frequency sweep is invalid
    InvalidSweep(String),

    /// Matrix solution failed
    MatrixSolverFailed(String),

    /// No input source specified
    NoInputSource,

    /// Input source not found in circuit
    InputSourceNotFound(String),
}

impl std::fmt::Display for PacError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacError::NoPssSolution => write!(f, "No PSS solution provided for PAC analysis"),
            PacError::InvalidPssSolution(s) => write!(f, "Invalid PSS solution: {}", s),
            PacError::InvalidConfig(s) => write!(f, "Invalid PAC configuration: {}", s),
            PacError::InvalidSweep(s) => write!(f, "Invalid frequency sweep: {}", s),
            PacError::MatrixSolverFailed(s) => write!(f, "Matrix solver failed: {}", s),
            PacError::NoInputSource => write!(f, "No input source specified for PAC analysis"),
            PacError::InputSourceNotFound(s) => write!(f, "Input source not found: {}", s),
        }
    }
}

impl std::error::Error for PacError {}
