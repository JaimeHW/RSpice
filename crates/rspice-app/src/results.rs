//! Portable result document exports and application test fixtures.

#[cfg(test)]
pub(crate) mod safety;
pub(crate) use rspice_results::{
    report_document, viewer_catalog, visualization_document, visualization_raster,
};
