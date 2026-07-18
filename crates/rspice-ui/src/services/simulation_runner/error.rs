use rspice_core::abort_signal::AbortSignal;

/// Typed error returned by cancellable simulation-service APIs.
///
/// Legacy synchronous APIs continue to expose `String` for compatibility,
/// but production execution uses this type so cancellation and resource
/// exhaustion can never be mistaken for configuration or solver failures.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ServiceRunError {
    /// Cooperative cancellation was requested.
    #[error("Simulation aborted")]
    Aborted,
    /// A configurable production resource budget was exceeded.
    #[error(transparent)]
    ResourceLimit(#[from] rspice_core::ResourceLimitError),
    /// Validation, parsing, circuit, or solver failure.
    #[error("{0}")]
    Failure(String),
}

/// Result type for cancellable simulation-service APIs.
pub type ServiceRunResult<T> = Result<T, ServiceRunError>;

impl ServiceRunError {
    /// Preserve typed cancellation while adding analysis-specific context to
    /// all other core errors.
    pub fn from_core(context: &str, error: rspice_core::SimulationError) -> Self {
        match error {
            rspice_core::SimulationError::Aborted => Self::Aborted,
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => Self::ResourceLimit(error),
            other => Self::Failure(format!("{context}: {other}")),
        }
    }

    /// Create a typed resource-limit error without relying on display-string
    /// parsing in UI-owned expansion code.
    pub fn resource_limit(
        resource: rspice_core::ResourceKind,
        requested: usize,
        limit: usize,
    ) -> Self {
        Self::ResourceLimit(rspice_core::ResourceLimitError {
            resource,
            requested,
            limit,
        })
    }

    /// Add context to ordinary failures while preserving cancellation and
    /// structured resource-limit errors.
    pub fn with_context(self, context: &str) -> Self {
        match self {
            Self::Failure(message) => Self::Failure(format!("{context}: {message}")),
            other => other,
        }
    }

    #[inline]
    pub fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted)
    }
}

impl From<rspice_core::SimulationError> for ServiceRunError {
    fn from(error: rspice_core::SimulationError) -> Self {
        match error {
            rspice_core::SimulationError::Aborted => Self::Aborted,
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => Self::ResourceLimit(error),
            other => Self::Failure(other.to_string()),
        }
    }
}

impl From<String> for ServiceRunError {
    fn from(error: String) -> Self {
        Self::Failure(error)
    }
}

impl From<&str> for ServiceRunError {
    fn from(error: &str) -> Self {
        Self::Failure(error.to_string())
    }
}

#[inline]
pub(crate) fn ensure_not_aborted(abort: &dyn AbortSignal) -> ServiceRunResult<()> {
    if abort.is_aborted() {
        Err(ServiceRunError::Aborted)
    } else {
        Ok(())
    }
}

/// Poll cancellation at a bounded cadence inside large scalar loops.
///
/// Callers retain explicit entry/exit and outer-stage checks. This helper
/// bounds cancellation latency without paying for a virtual/atomic poll on
/// every copied or transformed sample.
#[inline]
pub(crate) fn poll_periodically(abort: &dyn AbortSignal, index: usize) -> ServiceRunResult<()> {
    const POLL_STRIDE: usize = 64;
    if index.is_multiple_of(POLL_STRIDE) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_abort_remains_typed() {
        assert_eq!(
            ServiceRunError::from_core("AC analysis error", rspice_core::SimulationError::Aborted),
            ServiceRunError::Aborted
        );
        assert_eq!(
            ServiceRunError::from(rspice_core::SimulationError::Aborted),
            ServiceRunError::Aborted
        );
    }

    #[test]
    fn non_abort_core_error_keeps_context() {
        let error = ServiceRunError::from_core(
            "AC analysis error",
            rspice_core::SimulationError::Circuit("invalid circuit".to_string()),
        );

        assert!(matches!(error, ServiceRunError::Failure(_)));
        assert!(error.to_string().contains("AC analysis error"));
        assert!(error.to_string().contains("invalid circuit"));
    }

    #[test]
    fn core_resource_limit_remains_structured() {
        let core_error = rspice_core::ResourceLimitError {
            resource: rspice_core::ResourceKind::BatchRuns,
            requested: 11,
            limit: 10,
        };

        assert_eq!(
            ServiceRunError::from_core(
                "Parametric analysis error",
                rspice_core::SimulationError::ResourceLimit(core_error),
            ),
            ServiceRunError::ResourceLimit(core_error)
        );
        assert_eq!(
            ServiceRunError::ResourceLimit(core_error).with_context("ignored context"),
            ServiceRunError::ResourceLimit(core_error)
        );
    }
}
