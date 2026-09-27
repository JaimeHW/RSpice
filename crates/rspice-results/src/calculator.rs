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
pub mod retained;
#[cfg(feature = "engine-evidence")]
pub mod spice_parser;
pub mod value;

use crate::interpolation;
