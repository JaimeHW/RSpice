//! Exact result documents and viewer-independent result rules.
//!
//! Presentation and execution use these contracts without owning their data.

pub mod analysis_type;
pub mod calculator;
pub mod convergence_attribution;
pub mod dc_mismatch;
pub mod dc_sweep;
pub mod family_measurements;
pub mod family_metadata;
pub mod fft;
pub mod interpolation;
pub mod measurements;
pub mod monte_carlo;
pub mod operating_point;
pub mod optimization;
pub mod report_document;
pub mod result_import;
pub mod safety;
pub mod sampling;
pub mod stability;
pub mod validation;
pub mod viewer_catalog;
pub mod visualization_document;
pub mod visualization_raster;
pub mod yield_analysis;
