//! Pole-Zero Plot Module
//!
//! Commercial-grade pole-zero map visualization for transfer function analysis.
//!
//! # Features
//!
//! - Complex plane rendering with poles (×) and zeros (○)
//! - Unit circle overlay (z-domain stability boundary)
//! - Left half-plane shading (s-domain stability region)
//! - Stability indicators
//! - Natural frequency and Q-factor annotations
//! - Multiple transfer function comparison
//!
//! # Architecture
//!
//! Follows Cadence Spectre's pole-zero analysis visualization.

pub mod data;
pub mod rendering;
pub mod state;

pub use data::{ComplexRoot, PoleZeroData, RootType};
pub use rendering::render_pz_plot;
pub use state::PoleZeroState;

use crate::common::app::AppState;
use egui::Ui;

/// Render the pole-zero plot panel
pub fn render_pz_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_pz_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _state = PoleZeroState::new();
    }

    #[test]
    fn test_pz_data_creation() {
        let data = PoleZeroData::new("Test TF");
        assert!(data.is_empty());
    }

    #[test]
    fn test_root_types() {
        let pole = ComplexRoot::pole(0.0, 1.0);
        assert_eq!(pole.root_type, RootType::Pole);

        let zero = ComplexRoot::zero(-1.0, 0.0);
        assert_eq!(zero.root_type, RootType::Zero);
    }
}
