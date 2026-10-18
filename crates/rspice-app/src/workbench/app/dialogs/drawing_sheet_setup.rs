//! Permanent authored drawing-sheet setup.
//!
//! This workflow is intentionally separate from hardcopy Page Setup: it edits
//! the physical sheet saved with the design, while Print and Export own output
//! media, fitting, pagination and device capabilities. Its draft, validation
//! and publication are the sheet's own edit transaction
//! (`crate::workbench::app::drawing_sheet_edit`).

mod render;
