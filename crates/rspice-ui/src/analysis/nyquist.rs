//! Nyquist Plot Viewer
//!
//! Commercial-grade Nyquist plot visualization for stability analysis.
//!
//! # Features
//!
//! - Complex plane (Re vs Im) plotting
//! - Critical point (-1, 0) marking
//! - Unity circle overlay
//! - Encirclement counting for stability
//! - Loop gain visualization
//!
//! # Architecture
//!
//! Integrates with Bode data for AC/STB analysis.

pub mod data;
pub mod state;

pub use data::NyquistData;
pub use state::NyquistState;

// =============================================================================
// Tests
// =============================================================================
