//! Application-owned result services and portable document exports.

pub(crate) mod optimization;
pub(crate) mod safety;
pub(crate) use rspice_results::{
    operating_point, report_document, stability, viewer_catalog, visualization_document,
    visualization_raster,
};
