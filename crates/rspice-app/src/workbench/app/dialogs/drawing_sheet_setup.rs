//! Permanent authored drawing-sheet setup.
//!
//! This workflow is intentionally separate from hardcopy Page Setup: it edits
//! the physical sheet saved with the design, while Print and Export own output
//! media, fitting, pagination and device capabilities.

mod commit;
mod render;
mod state;

#[cfg(test)]
pub(crate) use commit::apply_drawing_sheet_setup;
pub(crate) use commit::{
    DrawingSheetEdit, apply_drawing_sheet_edit, drawing_sheet_setup_available,
    drawing_sheet_setup_available_for_state, open_drawing_sheet_setup,
    open_drawing_sheet_setup_for_custom_size, open_drawing_sheet_setup_for_state,
    open_drawing_sheet_setup_with_preset, validate_drawing_sheet_authority,
};
pub(crate) use render::{current_size_choice_label, standard_size_choice_label};
pub(crate) use state::{
    DrawingSheetAuthority, DrawingSheetSetupState, GovernedDrawingSheetAuthority,
};
