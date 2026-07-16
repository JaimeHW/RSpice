//! Expression evaluator for SPICE .PARAM statements
//!
//! Supports:
//! - Basic arithmetic: +, -, *, /, ** (power)
//! - Parentheses for grouping
//! - Built-in functions: sqrt, sin, cos, tan, exp, log, log10, abs, min, max
//! - Parameter substitution from context

use crate::Value;
use std::collections::HashMap;

mod api;
mod behavioral;
mod context;
mod error;
mod eval;
mod parser;
mod types;

#[cfg(test)]
mod tests;

use parser::ExprParser;

pub use api::{
    eval_expression, eval_expression_complex, eval_simple, eval_simple_complex, parse_expression,
};
pub use behavioral::{
    ParameterCircuitProbe, ParameterCircuitProbeKind,
    behavioral_expression_references_runtime_quantity, parameter_expression_circuit_probe,
    prepare_behavioral_expression, validate_global_parameter_expressions,
};
pub use context::{
    DEFAULT_RANDOM_SEED, ExpressionDialect, FunctionDef, ParamContext, ParameterRedefinitionPolicy,
    RandomState, StatisticalParamMode,
};
pub use error::ExprError;
pub use eval::{evaluate, evaluate_complex};
pub use types::{BinOpKind, ComplexValue, Expr, UnaryOpKind};
