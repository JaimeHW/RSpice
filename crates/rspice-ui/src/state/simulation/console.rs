/// Console message severity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageSeverity {
    Info,
    Warning,
    Error,
    Success,
}

/// Console log message
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleMessage {
    /// Message severity
    pub severity: MessageSeverity,

    /// Message content
    pub message: String,

    /// Timestamp (seconds since simulation start)
    pub timestamp: Option<f64>,
}

impl ConsoleMessage {
    /// Get current timestamp as epoch seconds
    fn current_timestamp() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0)
    }

    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Info,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Warning,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Error,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }

    /// Create a success message
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            severity: MessageSeverity::Success,
            message: message.into(),
            timestamp: Some(Self::current_timestamp()),
        }
    }
}
