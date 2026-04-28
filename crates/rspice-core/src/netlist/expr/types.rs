use super::*;

// Expression AST
//=============================================================================

/// Expression node in the AST
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric literal
    Number(Value),
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
