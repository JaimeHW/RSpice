//! The sheets feature observed through a whole application: its commands,
//! the sheet actions, one sheet edit, the sheet strip, the inspector's Sheet
//! section and the drawing-sheet dialogs, each over the session the shell
//! builds.

mod commands;
mod drawing_sheet_defaults;
mod drawing_sheet_overflow;
mod drawing_sheet_presets;
mod drawing_sheet_preview;
mod drawing_sheet_support;
mod sheet_actions;
mod sheet_edit;
// The panel's tests paint through the native-only inspector harness, and
// the browser runner runs no plain test.
#[cfg(not(target_arch = "wasm32"))]
mod sheet_inspector;
mod sheet_strip;

use crate::workbench::app::drawing_sheet_edit::{DrawingSheetEdit, apply_drawing_sheet_edit_on};
use crate::workbench::app_state::AppState;

/// [`apply_drawing_sheet_edit_on`] over the whole state, as tests hold it.
fn apply_drawing_sheet_edit(
    state: &mut AppState,
    edit: DrawingSheetEdit,
) -> Result<String, String> {
    apply_drawing_sheet_edit_on(&mut state.model, &state.workbench, edit)
}
