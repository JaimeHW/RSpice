//! Engine error types.

use thiserror::Error;

use crate::resource::ResourceLimitError;
use crate::solver::SolverError;

/// Stable machine-readable code for a simulation failure.
///
/// Display messages are intended for people and may gain additional context.
/// Service and language-binding integrations should branch on this code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SimulationErrorCode {
    /// The supplied [`super::SimulationConfig`] violates an invariant.
    InvalidConfiguration,
    /// A configured resource budget was exceeded.
    ResourceLimit,
    /// Circuit construction or device evaluation failed.
    CircuitError,
    /// The numerical solver failed.
    SolverError,
    /// A netlist-dependent simulation operation failed.
    NetlistError,
    /// An iterative analysis exhausted its convergence strategy.
    ConvergenceError,
    /// The caller cancelled the operation.
    Aborted,
}

impl SimulationErrorCode {
    /// Stable snake-case representation used by API and report payloads.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidConfiguration => "invalid_configuration",
            Self::ResourceLimit => "resource_limit",
            Self::CircuitError => "circuit_error",
            Self::SolverError => "solver_error",
            Self::NetlistError => "netlist_error",
            Self::ConvergenceError => "convergence_error",
            Self::Aborted => "aborted",
        }
    }
}

impl std::fmt::Display for SimulationErrorCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable high-level category for a simulation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SimulationErrorCategory {
    Configuration,
    ResourceLimit,
    Simulation,
    Solver,
    Netlist,
    Convergence,
    Cancellation,
}

impl SimulationErrorCategory {
    /// Stable snake-case representation used by API and report payloads.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::ResourceLimit => "resource_limit",
            Self::Simulation => "simulation",
            Self::Solver => "solver",
            Self::Netlist => "netlist",
            Self::Convergence => "convergence",
            Self::Cancellation => "cancellation",
        }
    }
}

impl std::fmt::Display for SimulationErrorCategory {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Portable metadata for a [`SimulationError`].
///
/// `retryable` is deliberately conservative: it is true only when a fresh
/// request context may safely retry the same workload without changing the
/// netlist or configuration. Deterministic numerical and resource failures are
/// false to prevent automatic retry storms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct SimulationErrorDescriptor {
    pub code: SimulationErrorCode,
    pub category: SimulationErrorCategory,
    pub retryable: bool,
    pub iterations: Option<usize>,
    pub resource_limit: Option<ResourceLimitError>,
}
/// Simulation errors
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Invalid simulation configuration: {0}")]
    Configuration(#[from] super::SimulationConfigError),

    #[error(transparent)]
    ResourceLimit(#[from] crate::resource::ResourceLimitError),

    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error("Solver error: {0}")]
    Solver(#[from] crate::solver::SolverError),

    #[error("Netlist error: {0}")]
    Netlist(String),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Simulation aborted by user")]
    Aborted,
}

impl SimulationError {
    /// Return stable metadata without requiring consumers to parse the display
    /// message or duplicate knowledge of nested error variants.
    pub fn descriptor(&self) -> SimulationErrorDescriptor {
        let (code, category, retryable) = match self {
            Self::Configuration(super::SimulationConfigError::ResourceLimit(_))
            | Self::ResourceLimit(_) => (
                SimulationErrorCode::ResourceLimit,
                SimulationErrorCategory::ResourceLimit,
                false,
            ),
            Self::Configuration(_) => (
                SimulationErrorCode::InvalidConfiguration,
                SimulationErrorCategory::Configuration,
                false,
            ),
            Self::Circuit(_) => (
                SimulationErrorCode::CircuitError,
                SimulationErrorCategory::Simulation,
                false,
            ),
            Self::Solver(_) => (
                SimulationErrorCode::SolverError,
                SimulationErrorCategory::Solver,
                false,
            ),
            Self::Netlist(_) => (
                SimulationErrorCode::NetlistError,
                SimulationErrorCategory::Netlist,
                false,
            ),
            Self::ConvergenceFailed(_) => (
                SimulationErrorCode::ConvergenceError,
                SimulationErrorCategory::Convergence,
                false,
            ),
            Self::Aborted => (
                SimulationErrorCode::Aborted,
                SimulationErrorCategory::Cancellation,
                true,
            ),
        };

        SimulationErrorDescriptor {
            code,
            category,
            retryable,
            iterations: match self {
                Self::ConvergenceFailed(iterations)
                | Self::Solver(SolverError::ConvergenceFailed(iterations)) => Some(*iterations),
                _ => None,
            },
            resource_limit: match self {
                Self::Configuration(super::SimulationConfigError::ResourceLimit(error))
                | Self::ResourceLimit(error) => Some(*error),
                _ => None,
            },
        }
    }
}
