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
//! A `.HB` card carries the tone frequencies and nothing else — see
//! [`HbConfig::to_spice`] for where the rest of the configuration goes.
//!
//! ```text
//! .hb 1G 2G
//! ```

mod config;
mod format;
mod state;

pub use config::{HbConfig, HbSolverType, HbToneConfig};
pub use state::HbDialogState;
