//! Expression Abstract Syntax Tree
//!
//! Represents mathematical expressions for behavioral sources.

use crate::Value;

/// Mathematical expression AST node
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Constant value
    Const(Value),
    /// Node voltage: V(node)
    NodeVoltage(String),
    /// Branch current: I(element)
    BranchCurrent(String),
    /// Binary operation
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary operation
    Unary { op: UnaryOp, operand: Box<Expr> },
    /// Function call
    Function { func: Function, args: Vec<Expr> },
    /// Time variable (for transient)
    Time,
    /// Frequency variable (for AC)
    Frequency,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    // Comparison
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    // Logical
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// Built-in functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    // Basic math
    Abs,
    Sqrt,
    Exp,
    Log,
    Log10,
    // Trigonometric
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2, // atan2(y, x) - two-argument arctangent
    Sinh,
    Cosh,
    Tanh,
    // Rounding
    Floor,
    Ceil,
    Round,
    // SPICE-specific
    Pwr,   // pwr(x, y) = |x|^y
    Pwrs,  // pwrs(x, y) = sign(x) * |x|^y
    Limit, // limit(x, lo, hi)
    Min,
    Max,
    Sign,  // sign(x) = -1, 0, or 1
    Uramp, // uramp(x) = max(0, x) - positive ramp
    Stp,   // stp(x) = 0 if x<0, 1 if x>=0 - step function
    Mod,   // mod(x, y) = x % y - modulo
    // Conditional
    If, // if(cond, then, else)
}

impl Expr {
    /// Create a constant expression
    pub fn constant(value: Value) -> Self {
        Expr::Const(value)
    }

    /// Create a node voltage reference
    pub fn voltage(node: impl Into<String>) -> Self {
        Expr::NodeVoltage(node.into())
    }

    /// Create a branch current reference
    pub fn current(element: impl Into<String>) -> Self {
        Expr::BranchCurrent(element.into())
    }

    /// Create an addition
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a subtraction
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Sub,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a multiplication
    pub fn mul(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a division
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Div,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a negation
    pub fn neg(operand: Expr) -> Self {
        Expr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(operand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_construction() {
        let expr = Expr::mul(Expr::voltage("2"), Expr::constant(2.0));

        match expr {
            Expr::Binary {
                op: BinaryOp::Mul, ..
            } => (),
            _ => panic!("Expected multiplication"),
        }
    }
}
