use crate::state::ComponentType;

/// Errors that can occur during symbol loading or parsing
#[derive(Debug, Clone)]
pub enum SymbolError {
    /// Failed to read file
    IoError { path: String, message: String },
    /// Failed to parse SVG content
    ParseError(String),
    /// Symbol not found in library
    NotFound(ComponentType),
}

impl std::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolError::IoError { path, message } => {
                write!(f, "Failed to read '{}': {}", path, message)
            }
            SymbolError::ParseError(msg) => write!(f, "SVG parse error: {}", msg),
            SymbolError::NotFound(t) => write!(f, "Symbol not found for {:?}", t),
        }
    }
}

impl std::error::Error for SymbolError {}
