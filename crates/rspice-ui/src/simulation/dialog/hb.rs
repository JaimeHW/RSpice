//! Harmonic Balance (HB) Analysis Configuration
//!
//! Configuration for large-signal frequency-domain analysis (.hb).
//! HB finds the periodic steady-state by solving for Fourier coefficients
//! directly in the frequency domain - essential for RF/microwave design.
//!
//! # Commercial Features (Spectre-Compatible)
//!
//! - Single-tone and multi-tone excitation
//! - Configurable harmonic count and oversampling
//! - Newton/Krylov solver options
//! - Automated mixing product truncation
//!
//! # Example SPICE Output
//!
//! ```text
//! .hb 1G harmonics=9 oversample=2 reltol=1e-6
//! ```

mod config;
mod format;
mod render;
mod state;

pub use config::{HbConfig, HbSolverType, HbToneConfig};
pub use state::{HbDialogState, HbToneDialogState};
