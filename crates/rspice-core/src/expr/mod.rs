//! Expression evaluation for behavioral sources
//!
//! This module provides a fast bytecode-based expression evaluator for B-sources.
//! The design avoids string parsing in the hot loop by compiling expressions
//! to bytecode during the "link" phase.
//!
//! Example: B1 1 0 V=V(2)*I(L1) compiles to bytecode that efficiently
//! evaluates the expression using node voltages and branch currents.

mod ast;
mod compiler;
mod parser;
mod vm;

pub use ast::{Expr, BinaryOp, UnaryOp, Function};
pub use compiler::compile;
pub use parser::parse_expression;
pub use vm::{Vm, CompiledExpr, Context, Instruction};
