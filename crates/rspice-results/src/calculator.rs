//! Waveform expression values, evaluation, and mathematical functions.
//!
//! Expression syntax, typed evaluation and mathematical functions live here.
//! Callers supply their numeric literal policy and bind live waveforms.

pub mod ast;
mod complex_functions;
mod complex_ops;
pub mod evaluator;
pub mod functions;
pub mod parser;
pub mod value;

use crate::interpolation;
use num_complex::Complex64;

/// Apply the calculator's named scalar or waveform function.
pub fn dispatch_function(
    name: &str,
    args: Vec<value::CalcValue>,
) -> Result<value::CalcValue, evaluator::EvaluationError> {
    complex_functions::dispatch(name, args)
}

/// Preserve a nonfinite complex sample as a missing rectangular value.
pub fn finite_or_hole(value: Complex64) -> Complex64 {
    value::hole(value)
}
