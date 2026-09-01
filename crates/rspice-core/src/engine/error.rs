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
    /// A behavioral expression names an invalid node or branch-current operand.
    BehavioralReferenceError,
    /// The numerical solver failed.
    SolverError,
    /// A netlist-dependent simulation operation failed.
    NetlistError,
    /// An authored output symbol is valid, but the selected analysis result
    /// cannot supply it.
    RequestedSignalUnavailable,
    /// An analysis result's signal names and numeric payload disagree with
    /// the schema promised by that result type.
    ResultSchemaMismatch,
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
            Self::BehavioralReferenceError => "behavioral_reference_error",
            Self::SolverError => "solver_error",
            Self::NetlistError => "netlist_error",
            Self::RequestedSignalUnavailable => "requested_signal_unavailable",
            Self::ResultSchemaMismatch => "result_schema_mismatch",
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
    Output,
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
            Self::Output => "output",
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

/// A well-formed authored output symbol that is absent from one analysis
/// result.
///
/// The original spelling is retained verbatim so frontends never have to
/// reverse a canonicalized registry name to identify the failing request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestedSignalUnavailableError {
    pub signal: String,
    pub analysis: String,
    pub coordinate: Option<String>,
}

impl RequestedSignalUnavailableError {
    pub fn new(
        signal: impl Into<String>,
        analysis: impl Into<String>,
        coordinate: Option<String>,
    ) -> Self {
        Self {
            signal: signal.into(),
            analysis: analysis.into(),
            coordinate,
        }
    }
}

impl std::fmt::Display for RequestedSignalUnavailableError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "requested signal '{}' is unavailable for {} analysis",
            self.signal, self.analysis
        )?;
        if let Some(coordinate) = &self.coordinate {
            write!(formatter, " at {coordinate}")?;
        }
        Ok(())
    }
}

impl std::error::Error for RequestedSignalUnavailableError {}

/// An internally produced analysis result whose signal registry and numeric
/// payload do not satisfy the result type's public schema.
///
/// Names are retained in their original order because ordering is part of the
/// result contract. The optional coordinate identifies the particular sweep,
/// frequency, time, or other analysis point at which the mismatch occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultSchemaMismatchError {
    pub analysis: String,
    pub coordinate: Option<String>,
    pub signal_family: String,
    pub expected_names: Vec<String>,
    pub actual_names: Vec<String>,
    pub expected_value_count: usize,
    pub actual_value_count: usize,
}

impl ResultSchemaMismatchError {
    pub fn new(
        analysis: impl Into<String>,
        coordinate: Option<String>,
        signal_family: impl Into<String>,
        expected_names: Vec<String>,
        actual_names: Vec<String>,
        expected_value_count: usize,
        actual_value_count: usize,
    ) -> Self {
        Self {
            analysis: analysis.into(),
            coordinate,
            signal_family: signal_family.into(),
            expected_names,
            actual_names,
            expected_value_count,
            actual_value_count,
        }
    }
}

impl std::fmt::Display for ResultSchemaMismatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "result schema mismatch for {} analysis",
            self.analysis
        )?;
        if let Some(coordinate) = &self.coordinate {
            write!(formatter, " at {coordinate}")?;
        }
        write!(
            formatter,
            " in {}: expected names {:?} with {} value(s), got names {:?} with {} value(s)",
            self.signal_family,
            self.expected_names,
            self.expected_value_count,
            self.actual_names,
            self.actual_value_count
        )
    }
}

impl std::error::Error for ResultSchemaMismatchError {}

/// Simulation errors
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Invalid simulation configuration: {0}")]
    Configuration(#[from] super::SimulationConfigError),

    #[error(transparent)]
    ResourceLimit(#[from] crate::resource::ResourceLimitError),

    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error(transparent)]
    BehavioralReference(Box<crate::device::BehavioralReferenceError>),

    #[error("Solver error: {0}")]
    Solver(#[from] crate::solver::SolverError),

    #[error("Netlist error: {0}")]
    Netlist(String),

    #[error(transparent)]
    RequestedSignalUnavailable(#[from] RequestedSignalUnavailableError),

    #[error(transparent)]
    ResultSchemaMismatch(Box<ResultSchemaMismatchError>),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Simulation aborted by user")]
    Aborted,
}

impl From<ResultSchemaMismatchError> for SimulationError {
    fn from(error: ResultSchemaMismatchError) -> Self {
        Self::ResultSchemaMismatch(Box::new(error))
    }
}

impl From<crate::circuit::CircuitError> for SimulationError {
    fn from(error: crate::circuit::CircuitError) -> Self {
        match error {
            crate::circuit::CircuitError::BehavioralReference(error) => {
                Self::BehavioralReference(error)
            }
            error => Self::Circuit(error.to_string()),
        }
    }
}

/// The device layer cannot name this type, so the conversion lives here.
///
/// `device` is ranked below `engine`, which means the generated Verilog-A
/// adapter returns its own [`BuiltinInstantiationError`] and the engine widens
/// it at the boundary. Reading down is what the layer order permits; the
/// reverse is what `tests/module_layering.rs` rejects.
///
/// [`BuiltinInstantiationError`]: crate::device::veriloga_builtins::BuiltinInstantiationError
#[cfg(feature = "veriloga-builtins-base")]
impl From<crate::device::veriloga_builtins::BuiltinInstantiationError> for SimulationError {
    fn from(error: crate::device::veriloga_builtins::BuiltinInstantiationError) -> Self {
        Self::Circuit(error.0)
    }
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
            Self::BehavioralReference(_) => (
                SimulationErrorCode::BehavioralReferenceError,
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
            Self::RequestedSignalUnavailable(_) => (
                SimulationErrorCode::RequestedSignalUnavailable,
                SimulationErrorCategory::Output,
                false,
            ),
            Self::ResultSchemaMismatch(_) => (
                SimulationErrorCode::ResultSchemaMismatch,
                SimulationErrorCategory::Output,
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

    /// Construct a typed missing-output error while retaining the authored
    /// signal spelling and optional analysis coordinate.
    pub fn requested_signal_unavailable(
        signal: impl Into<String>,
        analysis: impl Into<String>,
        coordinate: Option<String>,
    ) -> Self {
        RequestedSignalUnavailableError::new(signal, analysis, coordinate).into()
    }

    /// Construct a typed result-schema error while retaining both ordered
    /// signal registries and their associated payload cardinalities.
    pub fn result_schema_mismatch(
        analysis: impl Into<String>,
        coordinate: Option<String>,
        signal_family: impl Into<String>,
        expected_names: Vec<String>,
        actual_names: Vec<String>,
        expected_value_count: usize,
        actual_value_count: usize,
    ) -> Self {
        ResultSchemaMismatchError::new(
            analysis,
            coordinate,
            signal_family,
            expected_names,
            actual_names,
            expected_value_count,
            actual_value_count,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_signal_descriptor_and_message_preserve_authored_symbol() {
        let error = SimulationError::requested_signal_unavailable(
            "@Mdriver[Id]",
            "DC",
            Some("sweep point 3 (7.0000000000000000e-1)".to_string()),
        );
        assert_eq!(
            error.to_string(),
            "requested signal '@Mdriver[Id]' is unavailable for DC analysis at sweep point 3 (7.0000000000000000e-1)"
        );
        let descriptor = error.descriptor();
        assert_eq!(
            descriptor.code,
            SimulationErrorCode::RequestedSignalUnavailable
        );
        assert_eq!(descriptor.category, SimulationErrorCategory::Output);
        assert!(!descriptor.retryable);
        let SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("typed unavailable-signal variant was lost");
        };
        assert_eq!(detail.signal, "@Mdriver[Id]");
        assert_eq!(detail.analysis, "DC");
    }
}
