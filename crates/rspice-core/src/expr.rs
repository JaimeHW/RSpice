//! Expression evaluation for behavioral sources
//!
//! This module provides a fast bytecode-based expression evaluator for B-sources.
//! The design avoids string parsing in the hot loop by compiling expressions
//! to bytecode during the "link" phase.
//!
//! Example: B1 1 0 V=V(2)*I(L1) compiles to bytecode that efficiently
//! evaluates the expression using node voltages and branch currents.

mod ast;
#[doc(hidden)]
pub mod behavioral_derivative;
mod compiler;
mod derivative;
mod file_table;
mod parser;
mod power;
mod time_derivatives;
mod time_enclosure;
mod vm;

pub use ast::{BinaryOp, Expr, Function, LookupInterpolation, LookupTable, UnaryOp};
pub use compiler::compile;
#[doc(hidden)]
pub use compiler::{
    compile_time_expression, constant_over_time, constant_value, function_uses_implicit_time,
};
#[doc(hidden)]
pub use derivative::Derivative;
pub(crate) use derivative::derivative_pair;
#[doc(hidden)]
pub use file_table::file_lookup_dependency;
pub use file_table::{resolve_file_lookup_functions, resolve_file_lookup_functions_with_limits};
pub use parser::{ParseError, parse_expression, parse_expression_strict};
#[doc(hidden)]
pub use parser::{ParseExpressionWithAbortError, parse_expression_strict_with_abort};
#[doc(hidden)]
pub use power::{normalize_expression_boundary, ordered_limit, ordered_sign};
pub(crate) use power::{
    real_function_pow, real_function_pow_with_derivative, real_function_pwr,
    real_function_pwr_with_derivative, real_function_pwrs, real_function_pwrs_with_derivative,
    real_pow, real_pow_with_derivative,
};
#[doc(hidden)]
pub use time_derivatives::{TimeDerivativeError, TimeDerivatives};
#[doc(hidden)]
pub use time_enclosure::{TimeEnclosure, TimeInterval};
#[doc(hidden)]
pub use vm::{
    AcceptedSdtState, EXPR_ZERO_TOLERANCE, LOGARITHM_MIN_ARGUMENT, LogarithmDomain,
    XYCE_ATANH_EPSILON, XYCE_TANH_SATURATION_THRESHOLD, spice_exp_parameters,
    spice_waveform_is_periodic, spice_waveform_max_tone_cycles, spice_waveform_minimum_interval,
    spice_waveform_parameters,
};
pub use vm::{CompiledExpr, Context, Instruction, Vm};
pub(crate) use vm::{
    lookup_table_interpolate_with_derivative, spice_waveform_value_and_time_derivative,
};
