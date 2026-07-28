//! Console messages.
//!
//! The legacy console value model. New messages should be logged through
//! [`crate::diagnostics::LogBuffer`], which this mirrors into and which
//! carries the timestamp the console renders.

/// Console message with severity level.
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message severity.
    pub level: ConsoleLevel,
    /// Message content.
    pub message: String,
}

/// Console message severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleLevel {
    Info,
    Warning,
    Error,
}

impl ConsoleMessage {
    /// Create an info message.
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Info,
            message: message.into(),
        }
    }

    /// Create a warning message.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Warning,
            message: message.into(),
        }
    }

    /// Create an error message.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Error,
            message: message.into(),
        }
    }
}
