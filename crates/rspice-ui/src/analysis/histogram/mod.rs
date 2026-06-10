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
pub mod state;
pub mod statistics;

pub use data::{Histogram, HistogramBuilder};
pub use state::{HistogramDisplayMode, HistogramState};
pub use statistics::HistogramStats;


// =============================================================================
// Tests
// =============================================================================
