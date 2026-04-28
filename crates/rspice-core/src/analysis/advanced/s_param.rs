//! S-Parameter Analysis
//!
//! Computes scattering parameters for RF/microwave circuit characterization.
//! S-parameters describe the electrical behavior of linear networks in terms
//! of incident and reflected waves.
//!
//! # Theory
//!
//! For an N-port network with reference impedance Z₀:
//!
//! ```text
//! a = (V + Z₀·I) / (2·√Z₀)    (incident wave)
//! b = (V - Z₀·I) / (2·√Z₀)    (reflected wave)
//!
//! b = S · a
//!
//! S_ij = b_i / a_j  when a_k = 0 for k ≠ j
//! ```
//!
//! # Common S-Parameters (2-port)
//!
//! - **S₁₁**: Input reflection coefficient
//! - **S₂₁**: Forward transmission (gain)
//! - **S₁₂**: Reverse transmission (isolation)  
//! - **S₂₂**: Output reflection coefficient
//!
//! # Example
//!
//! ```ignore
//! .SP DEC 10 1MEG 10GIG
//! ```

use crate::Value;
use std::f64::consts::PI;

mod complex;
mod config;
mod matching;
mod matrix;
mod stability;
mod touchstone;

pub use complex::Complex;
pub use config::{FrequencySweep, SParameterAnalyzer, SParameterConfig};
pub use matching::{
    SourceImpedance, available_power, delivered_power, l_section_match, mismatch_loss_db,
    optimal_load_impedance, optimal_source_impedance, reactance_to_component,
    reflection_coefficient, renormalize_2port, renormalize_s11, transducer_gain_db,
};
pub use matrix::{Port, SMatrix, SParameterResult};
pub use stability::{GainAnalysis, StabilityAnalysis};
pub use touchstone::{TouchstoneExporter, TouchstoneFormat, TouchstoneFreqUnit};
