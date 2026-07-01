use super::*;

// Expression AST
//=============================================================================

/// Numeric value used by Xyce-compatible parameter expressions.
///
/// Scalar expression consumers use the real component.  The imaginary
/// component is retained in parameter context so `.PRINT` expressions can
/// project it with Xyce's `re()`/`img()` functions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexValue {
    pub re: Value,
    pub im: Value,
}

impl ComplexValue {
    #[inline]
    pub const fn new(re: Value, im: Value) -> Self {
        Self { re, im }
    }

    #[inline]
    pub const fn real(re: Value) -> Self {
        Self { re, im: 0.0 }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::real(0.0)
    }

    #[inline]
    pub fn is_real(self) -> bool {
        self.im == 0.0
    }

    #[inline]
    pub fn magnitude(self) -> Value {
        self.re.hypot(self.im)
    }

    #[inline]
    pub fn real_projection(self) -> Value {
        self.re
    }
}

impl From<Value> for ComplexValue {
    fn from(value: Value) -> Self {
        Self::real(value)
    }
}

/// Expression node in the AST
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric literal
    Number(Value),
    /// Complex numeric literal
    ComplexNumber(ComplexValue),
    /// Parameter reference
    Param(String),
    /// Binary operation
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary operation
    UnaryOp { op: UnaryOpKind, operand: Box<Expr> },
    /// Function call
    FnCall { name: String, args: Vec<Expr> },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    // Comparison
    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=
    Eq, // ==
    Ne, // !=
    // Boolean
    And, // &&
    Or,  // ||
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Neg, // -
    Pos, // +
    Not, // !
}
