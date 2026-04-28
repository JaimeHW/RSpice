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

