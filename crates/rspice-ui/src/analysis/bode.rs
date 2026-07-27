//! Bode Plot Viewer
//!
//! Commercial-grade Bode plot visualization for frequency response analysis.
//!
//! # Features
//!
//! - Magnitude (dB) and phase plots
//! - Log frequency axis
//! - Gain/phase margin calculation
//! - Cursor readout with interpolation
//! - Multiple transfer function overlay
//!
//! # Architecture
//!
//! Follows Cadence-style AC analysis visualization.

pub mod data;
pub mod state;

pub use data::{BodeData, FrequencyResponse};
pub use state::{BodeDisplayMode, BodePlotState};

// =============================================================================
// Tests
// =============================================================================
