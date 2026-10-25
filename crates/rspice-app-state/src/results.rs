//! Portable result document exports and application test fixtures.

#[cfg(any(test, feature = "test-support"))]
pub mod safety;
pub use rspice_results::{
    report_document, viewer_catalog, visualization_document, visualization_raster,
};
