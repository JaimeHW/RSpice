//! Console messages.
//!
//! The legacy value the simulation controller still hands to
//! `AppState::push_sim_message`. Nothing else uses it: every other site
//! records through `AppState::record` or answers through `AppState::prompt`.

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
