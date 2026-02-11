/// Console message with severity level.
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message severity.
    pub level: ConsoleLevel,
    /// Timestamp (epoch seconds).
    pub timestamp: f64,
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
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Create an info message.
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Info,
            timestamp: Self::current_timestamp(),
            message: message.into(),
        }
    }

    /// Create a warning message.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Warning,
            timestamp: Self::current_timestamp(),
            message: message.into(),
        }
    }

    /// Create an error message.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: ConsoleLevel::Error,
            timestamp: Self::current_timestamp(),
            message: message.into(),
        }
    }
}
