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
//! The locus is the loop gain a stability run measures — see
//! [`state::NyquistState`] for why there is exactly one of it.

pub mod state;
pub mod view;

pub use rspice_results::nyquist::{
    EncirclementCount, NyquistData, NyquistMargin, closed_loop_rhp_poles,
};
pub use state::NyquistState;
