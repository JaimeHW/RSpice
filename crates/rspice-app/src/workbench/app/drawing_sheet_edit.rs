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
    current_size_choice_label, format_sheet_length, parse_sheet_length, sheet_unit_suffix,
    standard_orientation_advisory, standard_size_choice_label,
};
pub(crate) use transaction::{
    DrawingSheetEdit, apply_drawing_sheet_edit, drawing_sheet_editable, mixed_size_advisory,
};
