// =============================================================================
// Error Types
// =============================================================================

/// Configuration error types
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// I/O error reading/writing file
    Io(String),
    /// JSON parse error
    Parse(String),
    /// JSON serialize error  
    Serialize(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(msg) => write!(f, "I/O error: {}", msg),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::Serialize(msg) => write!(f, "Serialize error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}
