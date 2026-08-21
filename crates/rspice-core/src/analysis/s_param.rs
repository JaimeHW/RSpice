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

use crate::{Complex64, Value};
use std::f64::consts::PI;

mod extract;
mod matrix;
mod network;
mod noise_params;
mod ports;
mod stability;
mod touchstone;

pub use extract::{ExtractError, extract_s_matrix};
pub use matrix::{Port, SMatrix, SParameterResult};
pub use network::{
    NetworkError, invert_complex_matrix, invert_complex_matrix_with_abort,
    s_column_from_port_voltages, s_from_y, s_from_y_with_abort, y_from_s,
};
pub use noise_params::{TwoPortNoise, derive_two_port_noise};
pub use ports::{
    PortError, PortRealization, SParameterPort, collect_ports, normalize_ports, set_excitations,
};
pub use stability::{GainAnalysis, StabilityAnalysis};
pub use touchstone::{
    TouchstoneFormat, TouchstoneFrequencyUnit, TouchstoneInput, touchstone, touchstone_extension,
};

/// Complex ratio that reads a vanishing divisor as zero rather than infinity.
///
/// The wave ratios in this module divide by a quantity that reaches zero
/// exactly where the ratio stops describing a measurable network — port
/// impedances that cancel, a renormalization onto a reflection the network
/// already presents. The callers are written against a divide that answers
/// zero there, and their own guards (`|Γ| >= 1`, `|S12|² > 1e-30`) are keyed
/// to that answer, so the floor is kept verbatim from the hand-rolled complex
/// type this module used to carry. `Complex64`'s own `/` yields infinities and
/// NaNs instead, which would propagate through every dB conversion downstream.
fn wave_ratio(numerator: Complex64, divisor: Complex64) -> Complex64 {
    let denom = divisor.norm_sqr();
    if denom < 1e-30 {
        return Complex64::ZERO;
    }
    Complex64::new(
        (numerator.re * divisor.re + numerator.im * divisor.im) / denom,
        (numerator.im * divisor.re - numerator.re * divisor.im) / denom,
    )
}
