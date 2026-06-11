//! Compiler error types and result aliases
//!
//! Provides comprehensive error types for all compilation phases with
//! rich diagnostic information for user-friendly error messages.

use crate::source::Span;
use thiserror::Error;

/// Result type alias for compiler operations
pub type CompileResult<T> = Result<T, CompileError>;

/// Main compiler error type
#[derive(Debug, Error)]
pub enum CompileError {
    /// I/O error (file not found, permission denied, etc.)
    #[error("I/O error: {message}")]
    IoError { message: String },

    /// Lexical analysis error
    #[error("{}", .0)]
    Lexer(#[from] LexerError),

    /// Parsing error
    #[error("{}", .0)]
    Parser(#[from] ParseError),

    /// Semantic analysis error
    #[error("{}", .0)]
    Semantic(#[from] SemanticError),

    /// Code generation error
    #[error("{}", .0)]
    CodeGen(#[from] CodeGenError),

    /// Multiple errors collected during compilation
    #[error("Compilation failed with {} error(s)", .0.len())]
    Multiple(Vec<CompileError>),
}

impl CompileError {
    /// Create an I/O error
    pub fn io_error(message: impl Into<String>) -> Self {
        Self::IoError {
            message: message.into(),
        }
    }

    /// Create a multiple error from a vec of errors
    pub fn multiple(errors: Vec<CompileError>) -> Self {
        if errors.len() == 1 {
            errors.into_iter().next().unwrap()
        } else {
            Self::Multiple(errors)
        }
    }
}

/// Lexer error with source location
#[derive(Debug, Error)]
#[error("Lexer error at offset {}: {kind}", span.start)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub span: Span,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Types of lexer errors
#[derive(Debug, Error)]
pub enum LexerErrorKind {
    #[error("Unexpected character: '{0}'")]
    UnexpectedChar(char),

    #[error("Unterminated string literal")]
    UnterminatedString,

    #[error("Unterminated block comment")]
    UnterminatedComment,

    #[error("Invalid number literal: {0}")]
    InvalidNumber(String),

    #[error("Invalid escape sequence: '\\{0}'")]
    InvalidEscape(char),

    #[error("Unterminated compiler directive")]
    UnterminatedDirective,

    #[error("Invalid preprocessor directive: `{0}")]
    InvalidDirective(String),
}

/// Parser error with source location
#[derive(Debug, Error)]
#[error("Parse error at offset {}: {kind}", span.start)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn expected(expected: impl Into<String>, found: impl Into<String>, span: Span) -> Self {
        Self::new(
            ParseErrorKind::Expected {
                expected: expected.into(),
                found: found.into(),
            },
            span,
        )
    }
}

/// Types of parser errors
#[derive(Debug, Error)]
pub enum ParseErrorKind {
    #[error("Expected {expected}, found {found}")]
    Expected { expected: String, found: String },

    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Invalid module declaration")]
    InvalidModule,

    #[error("Invalid port declaration")]
    InvalidPort,

    #[error("Invalid parameter declaration: {0}")]
    InvalidParameter(String),

    #[error("Invalid analog statement")]
    InvalidAnalogStatement,

    #[error("Invalid expression")]
    InvalidExpression,

    #[error("Invalid branch access")]
    InvalidBranchAccess,

    #[error("Duplicate {kind} declaration: '{name}'")]
    Duplicate { kind: String, name: String },

    #[error("Invalid discipline declaration")]
    InvalidDiscipline,

    #[error("Invalid nature declaration")]
    InvalidNature,
}

/// Semantic analysis error
#[derive(Debug, Error)]
#[error("Semantic error at offset {}: {kind}", span.start)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub span: Span,
}

impl SemanticError {
    pub fn new(kind: SemanticErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Types of semantic errors
#[derive(Debug, Error)]
pub enum SemanticErrorKind {
    #[error("Undeclared symbol: '{name}'")]
    UndeclaredSymbol { name: smol_str::SmolStr },

    #[error("Undefined module: '{0}'")]
    UndefinedModule(String),

    #[error("Undefined discipline: '{0}'")]
    UndefinedDiscipline(String),

    #[error("Type mismatch in {context}: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
        context: String,
    },

    #[error("Duplicate symbol: '{name}' already defined")]
    DuplicateSymbol {
        name: smol_str::SmolStr,
        first_defined: Span,
    },

    #[error("Port '{0}' has no discipline assigned")]
    PortNoDiscipline(String),

    #[error("Invalid contribution: cannot contribute to {0}")]
    InvalidContribution(String),

    #[error("Invalid branch: {0}")]
    InvalidBranch(String),

    #[error("Invalid node reference: '{name}' is a {kind}, not a node")]
    InvalidNodeReference {
        name: smol_str::SmolStr,
        kind: String,
    },

    #[error("Parameter '{name}' out of range: value {value} not in {range}")]
    ParameterOutOfRange {
        name: smol_str::SmolStr,
        value: f64,
        range: String,
    },

    #[error("Invalid condition type: expected boolean, found {found}")]
    InvalidCondition { found: String },

    #[error("Incompatible disciplines: {0} and {1}")]
    IncompatibleDisciplines(String, String),

    #[error("Invalid analog operator context: {0}")]
    InvalidAnalogOperator(String),

    #[error("Circular dependency in {0}")]
    CircularDependency(String),

    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),

    #[error("Function '{name}' expects {expected} argument(s), got {got}")]
    ArgumentCountMismatch {
        name: String,
        expected: String,
        got: usize,
    },

    #[error("Unknown function: '{0}'")]
    UnknownFunction(String),
}

/// Code generation error
#[derive(Debug, Error)]
#[error("Code generation error: {kind}")]
pub struct CodeGenError {
    pub kind: CodeGenErrorKind,
    pub span: Option<Span>,
}

impl CodeGenError {
    pub fn new(kind: CodeGenErrorKind) -> Self {
        Self { kind, span: None }
    }

    pub fn with_span(kind: CodeGenErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }
}

/// Types of code generation errors
#[derive(Debug, Error)]
pub enum CodeGenErrorKind {
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Failed to generate derivative for: {0}")]
    DerivativeFailed(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Cannot compile expression: {0}")]
    InvalidExpression(String),
}

impl CompileError {
    /// Create a semantic error
    pub fn semantic(error: SemanticError) -> Self {
        Self::Semantic(error)
    }
}

// Convenient From implementations for building errors
impl From<std::io::Error> for CompileError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError {
            message: e.to_string(),
        }
    }
}
