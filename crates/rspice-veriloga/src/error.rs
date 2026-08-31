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

    /// Module selection error: the requested module does not exist, or a
    /// multi-module file was compiled without naming a module
    #[error("Module selection error: {0}")]
    ModuleSelection(String),

    /// Invalid or unsafe file-system-free source bundle.
    #[error("Virtual source error: {0}")]
    VirtualSource(#[from] crate::virtual_source::VirtualSourceError),

    /// A caller-required optimized runtime backend was unavailable or rejected
    /// the compiled model, and interpreter fallback was explicitly disabled.
    #[error(transparent)]
    BackendQualification(#[from] crate::runtime_report::BackendQualificationError),

    /// An explicitly configured compiler performance budget was exceeded.
    #[error(transparent)]
    PerformanceBudget(#[from] crate::metrics::PerformanceBudgetExceeded),

    /// Compilation was stopped at a cooperative cancellation checkpoint.
    #[error(transparent)]
    Cancelled(#[from] crate::metrics::PipelineCancelled),

    /// Multiple errors collected during compilation. The display enumerates
    /// every collected diagnostic: the whole point of accumulating instead of
    /// stopping at the first error is that the author sees all of them.
    #[error("Compilation failed with {} error(s):\n{}", .0.len(), join_collected_errors(.0))]
    Multiple(Vec<CompileError>),
}

/// One collected diagnostic per line, in the order the analyzer recorded them.
fn join_collected_errors(errors: &[CompileError]) -> String {
    errors
        .iter()
        .map(|error| format!("  - {error}"))
        .collect::<Vec<_>>()
        .join("\n")
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

    #[error("Invalid event expression: {0}")]
    InvalidEventExpression(String),

    #[error("Invalid expression")]
    InvalidExpression,

    #[error("Invalid number literal: {0}")]
    InvalidNumber(String),

    #[error("Invalid branch access")]
    InvalidBranchAccess,

    #[error("Duplicate {kind} declaration: '{name}'")]
    Duplicate { kind: String, name: String },

    #[error("Invalid discipline declaration")]
    InvalidDiscipline,

    #[error("Invalid nature declaration")]
    InvalidNature,

    #[error("Unsupported {context}: {found}")]
    UnsupportedConstruct { context: String, found: String },

    /// A `generate` region, its `endgenerate` terminator, or a `genvar`
    /// declaration. The keywords lex, but this compiler has no generate
    /// elaborator, so the construct is refused by name instead of failing
    /// later as an unrecognized module item.
    #[error(
        "Unsupported construct '{keyword}': generate regions are not supported by the RSpice Verilog-A compiler"
    )]
    UnsupportedGenerate { keyword: String },

    /// A reserved IEEE 1364 / Verilog-AMS *digital* keyword, refused at the
    /// position where it introduces its construct.
    ///
    /// RSpice compiles the continuous subset of Verilog-AMS. The digital
    /// keywords are lexed so that a digital source stops on the construct the
    /// author has to remove, rather than being mistaken for an identifier and
    /// dying as an unrecognized module item.
    #[error("Verilog-AMS digital construct not yet supported: `{keyword}`")]
    UnsupportedAmsConstruct { keyword: String },
}

impl LexerErrorKind {
    /// Stable diagnostic code for this error kind.
    ///
    /// A code is compiler-owned diagnostic identity: editors group, filter and
    /// document a diagnostic by its code, so a shipped code never changes even
    /// when the rendered message does.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnexpectedChar(_) => "VA-LEX-UNEXPECTED-CHARACTER",
            Self::UnterminatedString => "VA-LEX-UNTERMINATED-STRING",
            Self::UnterminatedComment => "VA-LEX-UNTERMINATED-COMMENT",
            Self::InvalidNumber(_) => "VA-LEX-INVALID-NUMBER",
            Self::InvalidEscape(_) => "VA-LEX-INVALID-ESCAPE",
            Self::UnterminatedDirective => "VA-LEX-UNTERMINATED-DIRECTIVE",
            Self::InvalidDirective(_) => "VA-LEX-INVALID-DIRECTIVE",
        }
    }
}

impl ParseErrorKind {
    /// Stable diagnostic code for this error kind. See [`LexerErrorKind::code`].
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Expected { .. } => "VA-PARSE-EXPECTED",
            Self::UnexpectedToken(_) => "VA-PARSE-UNEXPECTED-TOKEN",
            Self::UnexpectedEof => "VA-PARSE-UNEXPECTED-EOF",
            Self::InvalidModule => "VA-PARSE-INVALID-MODULE",
            Self::InvalidPort => "VA-PARSE-INVALID-PORT",
            Self::InvalidParameter(_) => "VA-PARSE-INVALID-PARAMETER",
            Self::InvalidAnalogStatement => "VA-PARSE-INVALID-ANALOG-STATEMENT",
            Self::InvalidEventExpression(_) => "VA-PARSE-INVALID-EVENT-EXPRESSION",
            Self::InvalidExpression => "VA-PARSE-INVALID-EXPRESSION",
            Self::InvalidNumber(_) => "VA-PARSE-INVALID-NUMBER",
            Self::InvalidBranchAccess => "VA-PARSE-INVALID-BRANCH-ACCESS",
            Self::Duplicate { .. } => "VA-PARSE-DUPLICATE-DECLARATION",
            Self::InvalidDiscipline => "VA-PARSE-INVALID-DISCIPLINE",
            Self::InvalidNature => "VA-PARSE-INVALID-NATURE",
            Self::UnsupportedConstruct { .. } => "VA-PARSE-UNSUPPORTED-CONSTRUCT",
            Self::UnsupportedGenerate { .. } => "VA-PARSE-UNSUPPORTED-GENERATE",
            Self::UnsupportedAmsConstruct { .. } => "VA-PARSE-UNSUPPORTED-AMS-DIGITAL",
        }
    }
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

    #[error("Invalid expression: {0}")]
    InvalidExpression(String),

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

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Array index out of bounds: {0}")]
    IndexOutOfBounds(String),
}

impl SemanticErrorKind {
    /// Stable diagnostic code for this error kind. See [`LexerErrorKind::code`].
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UndeclaredSymbol { .. } => "VA-SEM-UNDECLARED-SYMBOL",
            Self::UndefinedModule(_) => "VA-SEM-UNDEFINED-MODULE",
            Self::UndefinedDiscipline(_) => "VA-SEM-UNDEFINED-DISCIPLINE",
            Self::TypeMismatch { .. } => "VA-SEM-TYPE-MISMATCH",
            Self::DuplicateSymbol { .. } => "VA-SEM-DUPLICATE-SYMBOL",
            Self::PortNoDiscipline(_) => "VA-SEM-PORT-WITHOUT-DISCIPLINE",
            Self::InvalidContribution(_) => "VA-SEM-INVALID-CONTRIBUTION",
            Self::InvalidBranch(_) => "VA-SEM-INVALID-BRANCH",
            Self::InvalidNodeReference { .. } => "VA-SEM-INVALID-NODE-REFERENCE",
            Self::ParameterOutOfRange { .. } => "VA-SEM-PARAMETER-OUT-OF-RANGE",
            Self::InvalidCondition { .. } => "VA-SEM-INVALID-CONDITION",
            Self::IncompatibleDisciplines(_, _) => "VA-SEM-INCOMPATIBLE-DISCIPLINES",
            Self::InvalidAnalogOperator(_) => "VA-SEM-INVALID-ANALOG-OPERATOR",
            Self::InvalidExpression(_) => "VA-SEM-INVALID-EXPRESSION",
            Self::CircularDependency(_) => "VA-SEM-CIRCULAR-DEPENDENCY",
            Self::MissingAttribute(_) => "VA-SEM-MISSING-ATTRIBUTE",
            Self::ArgumentCountMismatch { .. } => "VA-SEM-ARGUMENT-COUNT",
            Self::UnknownFunction(_) => "VA-SEM-UNKNOWN-FUNCTION",
            Self::UnsupportedFeature(_) => "VA-SEM-UNSUPPORTED-FEATURE",
            Self::IndexOutOfBounds(_) => "VA-SEM-INDEX-OUT-OF-BOUNDS",
        }
    }
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

impl CodeGenErrorKind {
    /// Stable diagnostic code for this error kind. See [`LexerErrorKind::code`].
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedFeature(_) => "VA-CODEGEN-UNSUPPORTED-FEATURE",
            Self::DerivativeFailed(_) => "VA-CODEGEN-DERIVATIVE-FAILED",
            Self::Internal(_) => "VA-CODEGEN-INTERNAL",
            Self::InvalidExpression(_) => "VA-CODEGEN-INVALID-EXPRESSION",
        }
    }
}

/// Diagnostic code for a failure that carries no phase-specific error kind.
const COMPILE_INPUT_CODE: &str = "VA-INPUT-IO";
/// Diagnostic code for an invalid or unsafe virtual source bundle.
const COMPILE_VIRTUAL_SOURCE_CODE: &str = "VA-INPUT-VIRTUAL-SOURCE";
/// Diagnostic code for a cooperative cancellation checkpoint.
const COMPILE_CANCELLED_CODE: &str = "VA-INPUT-CANCELLED";
/// Diagnostic code for a module that could not be selected.
const COMPILE_MODULE_SELECTION_CODE: &str = "VA-MODULE-SELECTION";
/// Diagnostic code for a required backend that did not qualify.
const COMPILE_BACKEND_QUALIFICATION_CODE: &str = "VA-BACKEND-QUALIFICATION";
/// Diagnostic code for an exceeded compiler performance budget.
const COMPILE_PERFORMANCE_BUDGET_CODE: &str = "VA-BUDGET-EXCEEDED";
/// Diagnostic code for a collected failure that carries no errors of its own.
const COMPILE_MULTIPLE_CODE: &str = "VA-COMPILE-FAILED";

impl CompileError {
    /// Create a semantic error
    pub fn semantic(error: SemanticError) -> Self {
        Self::Semantic(error)
    }

    /// Stable diagnostic code for this failure. See [`LexerErrorKind::code`].
    ///
    /// [`Self::Multiple`] reports the code of its first collected error: a
    /// collection is not itself a diagnosable condition, and every caller that
    /// needs each code flattens the collection through
    /// [`crate::compile_diagnostics`] instead.
    pub fn diagnostic_code(&self) -> &'static str {
        match self {
            Self::IoError { .. } => COMPILE_INPUT_CODE,
            Self::Lexer(error) => error.kind.code(),
            Self::Parser(error) => error.kind.code(),
            Self::Semantic(error) => error.kind.code(),
            Self::CodeGen(error) => error.kind.code(),
            Self::ModuleSelection(_) => COMPILE_MODULE_SELECTION_CODE,
            Self::VirtualSource(_) => COMPILE_VIRTUAL_SOURCE_CODE,
            Self::BackendQualification(_) => COMPILE_BACKEND_QUALIFICATION_CODE,
            Self::PerformanceBudget(_) => COMPILE_PERFORMANCE_BUDGET_CODE,
            Self::Cancelled(_) => COMPILE_CANCELLED_CODE,
            Self::Multiple(errors) => errors
                .first()
                .map_or(COMPILE_MULTIPLE_CODE, Self::diagnostic_code),
        }
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
