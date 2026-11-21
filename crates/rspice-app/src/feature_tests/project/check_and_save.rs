//! Check and save through the whole application: a landed save closes the
//! form with what it missed.

use crate::workbench::app_state::AppState;
use crate::workbench::session::project_transactions::check_and_save::complete_validated_save;
use rspice_app_project::workbench::app::dialogs::check_and_save::NEWER_CHANGES;

/// A save that landed closes the form, and edits that missed it are
/// said in the record rather than in a dialog left open to say it.
#[test]
fn a_landed_save_closes_the_form_and_records_what_it_missed() {
    let mut state = AppState::default();
    state.project.check_and_save.open = true;
    state.project.check_and_save.revision_note = "Compensation update".to_owned();
    complete_validated_save(&mut state, true);
    assert!(!state.project.check_and_save.open);
    assert!(state.project.check_and_save.revision_note.is_empty());
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the save is recorded");
    assert!(line.message.starts_with("Saved a validated revision"));
    assert!(line.message.ends_with(NEWER_CHANGES), "{}", line.message);
}
