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
mod file_table;
mod parser;
mod power;
mod vm;

pub use ast::{BinaryOp, Expr, Function, LookupInterpolation, LookupTable, UnaryOp};
pub use compiler::compile;
pub use file_table::resolve_file_lookup_functions;
pub use parser::{ParseError, parse_expression, parse_expression_strict};
pub(crate) use power::{normalize_expression_boundary, real_pow, real_pow_with_derivative};
pub use vm::{CompiledExpr, Context, Instruction, Vm};
