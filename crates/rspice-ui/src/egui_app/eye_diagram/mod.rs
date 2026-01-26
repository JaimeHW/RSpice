//! Eye Diagram Viewer
//!
//! Commercial-grade eye diagram visualization for signal integrity analysis.
//!
//! # Features
//!
//! - Overlay of signal transitions aligned to bit period
//! - Persistence/density display mode
//! - Eye opening measurements (height, width, area)
//! - Jitter calculation (RJ, DJ, TJ)
//! - Rise/fall time measurement
//! - Q-factor from BER
//! - Mask testing support
//!
//! # Architecture
//!
//! Follows Cadence-style signal integrity analysis workflow.

pub mod data;
pub mod measurements;
pub mod rendering;
pub mod state;

pub use data::{EyeData, EyeTrace};
pub use measurements::EyeMeasurements;
pub use rendering::render_eye_diagram;
pub use state::{EyeDiagramState, EyeDisplayMode};

use crate::egui_app::app::AppState;
use egui::Ui;

/// Render the eye diagram panel
pub fn render_eye_diagram_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_eye_diagram_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _mode = EyeDisplayMode::default();
        let _state = EyeDiagramState::new();
    }
}
