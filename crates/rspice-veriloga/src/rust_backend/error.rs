//! Errors raised while generating Rust from canonical IR.
//!
//! A [`RustBackendError`] always names the source file and module it came
//! from, because generation runs over a whole corpus and an error that does
//! not identify its device is not actionable.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustBackendErrorKind {
    Unsupported,
    PerformanceBudget,
    Cancelled,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustBackendError {
    pub kind: RustBackendErrorKind,
    pub source: String,
    pub module: String,
    pub message: String,
}

impl RustBackendError {
    pub fn unsupported(
        source: impl Into<String>,
        module: impl Into<String>,
        feature: impl Into<String>,
    ) -> Self {
        Self {
            kind: RustBackendErrorKind::Unsupported,
            source: source.into(),
            module: module.into(),
            message: format!(
                "unsupported Verilog-A construct for Rust backend: {}",
                feature.into()
            ),
        }
    }

    pub fn internal(
        source: impl Into<String>,
        module: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: RustBackendErrorKind::Internal,
            source: source.into(),
            module: module.into(),
            message: message.into(),
        }
    }

    pub fn performance_budget(
        source: impl Into<String>,
        module: impl Into<String>,
        error: crate::metrics::PerformanceBudgetExceeded,
    ) -> Self {
        Self {
            kind: RustBackendErrorKind::PerformanceBudget,
            source: source.into(),
            module: module.into(),
            message: error.to_string(),
        }
    }

    pub fn cancelled(
        source: impl Into<String>,
        module: impl Into<String>,
        error: crate::metrics::PipelineCancelled,
    ) -> Self {
        Self {
            kind: RustBackendErrorKind::Cancelled,
            source: source.into(),
            module: module.into(),
            message: error.to_string(),
        }
    }

    pub fn is_unsupported(&self) -> bool {
        self.kind == RustBackendErrorKind::Unsupported
    }
}

impl fmt::Display for RustBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rust backend error in {} module {}: {}",
            self.source, self.module, self.message
        )
    }
}

impl std::error::Error for RustBackendError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_classification_does_not_depend_on_display_text() {
        let error = RustBackendError {
            kind: RustBackendErrorKind::Unsupported,
            source: "model.va".to_string(),
            module: "model".to_string(),
            message: "typed unsupported error with deliberately different wording".to_string(),
        };

        assert!(error.is_unsupported());
    }

    #[test]
    fn internal_error_is_not_a_backend_fallback_signal() {
        let error = RustBackendError::internal(
            "model.va",
            "model",
            "unsupported Verilog-A construct for Rust backend: misleading text",
        );

        assert!(!error.is_unsupported());
    }
}
