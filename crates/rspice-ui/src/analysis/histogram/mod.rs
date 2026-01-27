//! Histogram Viewer
//!
//! Commercial-grade histogram visualization for statistical analysis.
//!
//! # Features
//!
//! - Binning with automatic or user-specified ranges
//! - Normal/log scale vertical axis
//! - PDF/CDF overlay modes
//! - Statistical measurements (mean, std dev, percentiles)
//! - Multi-histogram overlay comparison
//! - Monte Carlo analysis integration
//!
//! # Architecture
//!
//! Follows Cadence-style statistical analysis workflow.

pub mod data;
pub mod rendering;
pub mod state;
pub mod statistics;

pub use data::{Histogram, HistogramBuilder};
pub use rendering::render_histogram;
pub use state::{HistogramDisplayMode, HistogramState};
pub use statistics::HistogramStats;

use crate::common::app::AppState;
use egui::Ui;

/// Render the histogram panel
pub fn render_histogram_panel(ui: &mut Ui, app_state: &mut AppState) {
    rendering::render_histogram_viewer(ui, app_state);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _mode = HistogramDisplayMode::default();
        let _state = HistogramState::new();
    }

    #[test]
    fn test_histogram_creation() {
        let hist = HistogramBuilder::new()
            .bin_count(20)
            .build(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(hist.bin_count() > 0);
    }
}
