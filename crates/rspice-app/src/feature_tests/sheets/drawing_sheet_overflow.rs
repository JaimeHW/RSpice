//! The overflow review's opener over a whole application.

use crate::workbench::app::dialogs::drawing_sheet_overflow::*;
use crate::workbench::app_state::AppState;

#[test]
fn review_opener_is_modal_and_idempotent_for_schematic_views() {
    let mut state = AppState::default();
    let open = &mut state.sheets.drawing_sheet_overflow_open;
    assert!(open_drawing_sheet_overflow_review(open, &state.model));
    assert!(!open_drawing_sheet_overflow_review(open, &state.model));
    assert!(state.sheets.drawing_sheet_overflow_open);
    assert!(state.dialog_modal_open());
}
