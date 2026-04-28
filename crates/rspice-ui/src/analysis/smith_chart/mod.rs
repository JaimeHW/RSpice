//! Smith Chart Viewer
//!
//! Commercial-grade Smith chart visualization for RF/microwave circuit analysis.
//!
//! # Features
//!
//! - Impedance (Z) and Admittance (Y) chart modes
//! - Combined Z-Y overlay mode
//! - Normalized and denormalized impedance display
//! - S-parameter data overlay
//! - VSWR circles
//! - Constant resistance and reactance circles
//! - Interactive marker with impedance readout
//! - Reference impedance selection (default 50Ω)
//!
//! # Architecture
//!
//! This follows the same architecture as Cadence Virtuoso ADE and
//! other commercial RF design tools.

pub mod complex;
pub mod impedance;
pub mod rendering;
pub mod state;

pub use complex::Complex;
pub use impedance::{Admittance, Impedance};
pub use rendering::render_smith_chart;
pub use state::{SmithChartMode, SmithChartState};

// Re-export for convenient access
use crate::common::app::AppState;
use egui::Ui;

/// Render the Smith chart panel
pub fn render_smith_chart_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_smith_chart_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================

