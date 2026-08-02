//! Permanent authored drawing-sheet setup.
//!
//! This workflow is intentionally separate from hardcopy Page Setup: it edits
//! the physical sheet saved with the design, while Print and Export own output
//! media, fitting, pagination and device capabilities.

mod commit;
mod render;
mod state;

pub(crate) use commit::{
    drawing_sheet_setup_available, open_drawing_sheet_setup, open_drawing_sheet_setup_for_state,
    open_drawing_sheet_setup_with_preset, validate_drawing_sheet_authority,
};
pub(crate) use state::{
    DrawingSheetAuthority, DrawingSheetSetupState, GovernedDrawingSheetAuthority,
};
