//=============================================================================
// Errors
//=============================================================================

/// Errors that can occur during expression parsing/evaluation
#[derive(Debug, Clone, PartialEq)]
pub enum ExprError {
    InvalidNumber(String),
    UnexpectedChar(char),
    MissingCloseParen,
    MissingCloseQuote,
    TrailingInput(String),
    UndefinedParam(String),
    UnknownFunction(String),
    WrongArgCount(String),
    InvalidArgument(String),
    DivisionByZero,
}

impl std::fmt::Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ExprError::UnexpectedChar(c) => write!(f, "Unexpected character: '{}'", c),
            ExprError::MissingCloseParen => write!(f, "Missing closing parenthesis"),
            ExprError::MissingCloseQuote => write!(f, "Missing closing single quote"),
            ExprError::TrailingInput(s) => write!(f, "Trailing input: {}", s),
            ExprError::UndefinedParam(s) => write!(f, "Undefined parameter: {}", s),
            ExprError::UnknownFunction(s) => write!(f, "Unknown function: {}", s),
            ExprError::WrongArgCount(s) => write!(f, "Wrong argument count for function: {}", s),
            ExprError::InvalidArgument(s) => write!(f, "Invalid argument: {}", s),
            ExprError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl std::error::Error for ExprError {}
