//! Waveform expression values, evaluation, and mathematical functions.
//!
//! Expression syntax, typed evaluation and mathematical functions live here.
//! Callers supply their numeric literal policy and bind live waveforms.

pub mod ast;
pub mod catalogue;
mod complex_functions;
mod complex_ops;
#[cfg(feature = "engine-evidence")]
pub mod datasets;
pub mod evaluator;
mod frequency_measures;
pub mod functions;
mod jitter;
mod measures;
pub mod parser;
pub mod retained;
mod series;
mod spectral;
#[cfg(feature = "engine-evidence")]
pub mod spice_parser;
mod statistics;
mod time_measures;
pub mod value;

use crate::interpolation;
