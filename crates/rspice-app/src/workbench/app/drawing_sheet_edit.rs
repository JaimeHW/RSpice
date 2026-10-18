//! Editing the active sheet's authored drawing sheet.
//!
//! One edit is one transaction: the sheet's draft is taken from what the
//! sheet draws, the edit is staged into it, the draft is validated as a
//! whole, and the result is published to the design-management catalog as
//! one undo entry. The sheet panel edits the sheet this way, one property at
//! a time. Print and Export own output media and never write here.

mod draft;
mod transaction;

pub(crate) use draft::{
    BorderTemplateChoice, DrawingSheetAuthority, DrawingSheetDraft, DrawingSheetSection,
    DrawingSheetSetupState, DrawingSheetSupportRequest, FieldProblem,
    GovernedDrawingSheetAuthority, PageSetupScope, SheetDisplayUnit, SheetSizeChoice,
    TitleBlockTemplateChoice, ZoneModeChoice, current_size_choice_label, format_choice_label,
    format_sheet_length, format_um, parse_sheet_length, standard_orientation_advisory,
    standard_size_choice_label,
};
pub(crate) use transaction::{
    DrawingSheetEdit, apply_drawing_sheet_edit, apply_drawing_sheet_setup,
    drawing_sheet_setup_available, drawing_sheet_setup_available_for_state, mixed_size_advisory,
    open_drawing_sheet_setup, open_drawing_sheet_setup_for_state,
    open_drawing_sheet_setup_with_preset, resume_drawing_sheet_setup_after_support,
    validate_drawing_sheet_authority,
};
