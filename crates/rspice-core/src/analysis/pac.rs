//! Periodic AC (PAC) Analysis Module
//!
//! PAC analysis performs small-signal AC analysis around a periodic steady-state
//! (PSS) operating point. This is essential for RF circuit characterization:
//!
//! - **Mixer conversion gain**: Input-to-output transfer at different sidebands
//! - **LNA noise figure**: With periodic large-signal interference  
//! - **PLL loop gain**: Stability analysis around limit cycle
//! - **Power amplifier intermodulation**: Small-signal response vs. drive level
//!
//! # Theory
//!
//! Unlike standard AC analysis which linearizes around a DC operating point,
//! PAC linearizes around a time-varying periodic operating point from PSS.
//! This results in a Linear Time-Varying (LTV) system characterized by a
//! **conversion matrix** that maps input sidebands to output sidebands.
//!
//! For a mixer with LO frequency f₀:
//! - Input at f₀ + Δf (RF) appears at output as DC + Δf (IF)
//! - The conversion matrix element `H[-1,0](Δf)` is the conversion gain
//!
//! # Algorithm
//!
//! 1. Solve the large-signal periodic operating point with harmonic balance
//! 2. Sample the periodically time-varying small-signal conductances and
//!    transform them to conversion-coupling spectra
//! 3. Solve the sideband-coupled admittance system at every sweep offset
//!
//! The solve lives in the engine, which owns the circuit: see
//! `Engine::run_pac` and `harmonic_balance::solver::periodic_ac`.
//!
//! # SPICE Syntax
//!
//! ```text
//! .PAC start=1Meg stop=1Gig pts=100 maxsideband=5
//! ```

mod config;
mod conversion_matrix;
mod result;
mod solver;

pub use config::{PacConfig, PacSweepType};
pub use conversion_matrix::{ConversionMatrix, SidebandTransfer};
pub use result::{PacResult, PacSidebandData};
pub use solver::PacError;

/// Signed sideband coordinate, evaluated with the solver's single rounding.
/// Callers validate finiteness; taking an absolute value or splitting the
/// multiply-add would change the retained spectral coordinate.
pub(crate) fn sideband_frequency(
    sideband: i32,
    fundamental: crate::Value,
    offset: crate::Value,
) -> crate::Value {
    f64::from(sideband).mul_add(fundamental, offset)
}
