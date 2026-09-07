//! Expression AST and the numeric type it evaluates to.
//!
//! [`Expr`] is the parsed tree; [`BinOpKind`] and [`UnaryOpKind`] cover
//! arithmetic, comparison, and boolean operators. [`ComplexValue`] — the
//! crate-wide alias for `Complex64` — is the evaluation type: expressions are
//! always evaluated complex so AC-analysis callers can project the imaginary
//! component, with real-valued callers simply taking `re`.

use super::*;

// Expression AST
//=============================================================================

/// True when a value carries no imaginary part.
///
/// The test is exact rather than tolerant, and deliberately so: an imaginary
/// component here is either structurally zero — every operand along the way
/// was real — or it is a result the caller must not silently drop. A
/// tolerance would turn a small but genuine imaginary part into a real
/// answer.
#[inline]
pub fn is_real(value: ComplexValue) -> bool {
    value.im == 0.0
}

/// Expression node in the AST
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric literal
    Number(Value),
    /// Complex numeric literal
    ComplexNumber(ComplexValue),
    /// Quoted string literal used by file-backed expression functions.
    ///
    /// Parameter expressions are normally numeric, but Xyce permits a
    /// file-backed TABLE expression (for example, `table("wave.dat")`) to
    /// flow through a runtime parameter.  Keeping the literal in this AST
    /// lets the behavioral expansion pass preserve it for the strict
    /// runtime compiler, which resolves the file-backed lookup.
    StringLiteral(String),
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

impl Expr {
    /// Check before retaining a parser- or substitution-produced tree. Cloning
    /// and dropping this public boxed AST still use the native call stack.
    pub(crate) fn ensure_stack_safe_depth(&self) -> Result<(), ExprError> {
        let mut pending = vec![(self, 1)];
        while let Some((node, depth)) = pending.pop() {
            if depth > crate::resource::MAX_EXPRESSION_TREE_DEPTH {
                return Err(ExprError::InvalidArgument(format!(
                    "Expression tree exceeds the stack safety limit of {}",
                    crate::resource::MAX_EXPRESSION_TREE_DEPTH
                )));
            }
            match node {
                Expr::BinOp { left, right, .. } => {
                    pending.push((right, depth + 1));
                    pending.push((left, depth + 1));
                }
                Expr::UnaryOp { operand, .. } => pending.push((operand, depth + 1)),
                Expr::FnCall { args, .. } => {
                    pending.extend(args.iter().map(|arg| (arg, depth + 1)));
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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
