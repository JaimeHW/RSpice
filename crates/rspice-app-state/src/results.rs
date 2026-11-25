//! Portable result document exports and application test fixtures.

#[cfg(any(test, feature = "test-support"))]
pub mod safety;
pub use rspice_results::{
    report_document, report_figure, viewer_catalog, visualization_document, visualization_raster,
};

/// Verify's verdict on a specification, and the spelling Verify states its
/// values, limits and margins in, for a page that shows its verdicts without
/// judging them again.
pub use rspice_results::verification::{RowStatus, difference_text, format_value, limit_text};
