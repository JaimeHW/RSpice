//! Calculator Abstract Syntax Tree (AST)
//!
//! Defines the structure of expressions for the waveform calculator.
//! Supports scalars, waveform references, and algebraic operations.

use serde::{Deserialize, Serialize};

/// Expression node in the AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalculatorExpr {
    /// Numeric literal (scalar)
    Number(f64),
    /// Waveform reference (signal name, optional file/run ID)
    /// Format: V("node"), I("dev"), v("/sub/node"), etc.
    WaveformRef {
        signal: String,
        /// Optional dataset/run identifier (e.g., "tran-001")
        dataset: Option<String>,
    },
    /// Binary operation
    BinaryOp {
        op: BinaryOp,
        left: Box<CalculatorExpr>,
        right: Box<CalculatorExpr>,
    },
    /// Unary operation
    UnaryOp {
        op: UnaryOp,
        operand: Box<CalculatorExpr>,
    },
    /// Function call
    FunctionCall {
        name: String,
        args: Vec<CalculatorExpr>,
    },
    /// Constant (TIME, FREQ)
    Constant(CalculatorConstant),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Pow, // ^ or **
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg, // -
}

/// Simulation constants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalculatorConstant {
    Time,
    Frequency,
}

impl CalculatorExpr {
    /// Helper to create a number node
    pub fn number(val: f64) -> Self {
        Self::Number(val)
    }

    /// Helper to create a binary operation
    pub fn binary(op: BinaryOp, left: Self, right: Self) -> Self {
        Self::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Helper to create a unary operation
    pub fn unary(op: UnaryOp, operand: Self) -> Self {
        Self::UnaryOp {
            op,
            operand: Box::new(operand),
        }
    }

    /// Helper to create a function call
    pub fn func(name: &str, args: Vec<Self>) -> Self {
        Self::FunctionCall {
            name: name.to_string(),
            args,
        }
    }

    /// Helper to create a waveform reference
    pub fn wave(signal: &str) -> Self {
        Self::WaveformRef {
            signal: signal.to_string(),
            dataset: None,
        }
    }
}
