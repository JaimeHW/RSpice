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
