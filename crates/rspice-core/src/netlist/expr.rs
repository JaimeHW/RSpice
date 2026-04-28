//! Expression evaluator for SPICE .PARAM statements
//!
//! Supports:
//! - Basic arithmetic: +, -, *, /, ** (power)
//! - Parentheses for grouping
//! - Built-in functions: sqrt, sin, cos, tan, exp, log, log10, abs, min, max
//! - Parameter substitution from context

use super::lexer::parse_spice_value;
use crate::Value;
use std::collections::HashMap;

mod api;
mod context;
mod error;
mod eval;
mod parser;
mod types;

use parser::ExprParser;

pub use api::{eval_expression, eval_simple, parse_expression};
pub use context::{FunctionDef, ParamContext};
pub use error::ExprError;
pub use eval::evaluate;
pub use types::{BinOpKind, Expr, UnaryOpKind};
