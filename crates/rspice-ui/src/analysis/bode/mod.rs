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
pub mod rendering;
pub mod state;

pub use data::{BodeData, FrequencyResponse};
pub use rendering::render_bode_plot;
pub use state::{BodeDisplayMode, BodePlotState};

use crate::common::app::AppState;
use egui::Ui;

/// Render the Bode plot panel
pub fn render_bode_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_bode_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================
