use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustBackendError {
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
            source: source.into(),
            module: module.into(),
            message: message.into(),
        }
    }

    pub fn is_unsupported(&self) -> bool {
        self.message
            .starts_with("unsupported Verilog-A construct for Rust backend:")
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
