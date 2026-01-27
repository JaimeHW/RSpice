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
pub mod rendering;
pub mod state;

pub use data::NyquistData;
pub use rendering::render_nyquist_plot;
pub use state::NyquistState;

use crate::common::app::AppState;
use egui::Ui;

/// Render the Nyquist plot panel
pub fn render_nyquist_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_nyquist_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _state = NyquistState::new();
    }

    #[test]
    fn test_nyquist_data_creation() {
        let data = NyquistData::new("Test");
        assert!(data.is_empty());
    }
}
