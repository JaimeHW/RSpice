//! Tests for Rename View.

use super::*;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{Cell, Library, View};
use crate::workbench::project::library_dialogs::shown;
use crate::workbench::project::tests::with_cx;
use crate::workbench::{AppState, RSpiceApp};

fn state_with_amplifier_schematic() -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amplifier");
    cell.add_view(View::new("schematic", crate::state::ViewType::Schematic));
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state
}

#[test]
fn rename_view_dialog_starts_with_the_exact_active_view_identity() {
    let mut state = state_with_amplifier_schematic();
    let state = &mut state;

    state
        .project
        .open_rename_view_dialog(&state.model, "work", "amplifier", "schematic")
        .expect("a writable view opens Rename View");

    assert!(state.project.rename_view_dialog);
    assert_eq!(state.project.rename_view_library, "work");
    assert_eq!(state.project.rename_view_cell, "amplifier");
    assert_eq!(state.project.rename_view_current, "schematic");
    assert_eq!(state.project.rename_view_name, "schematic");
    assert!(state.project.rename_view_error.is_none());
    assert_eq!(
        state.project.rename_view_library_revision,
        state.model.library_manager.revision()
    );
}

#[test]
fn rename_view_entry_point_rejects_missing_and_read_only_targets() {
    let mut state = state_with_amplifier_schematic();
    let state = &mut state;
    state.project.rename_view_library = "sentinel".to_owned();

    assert!(
        state
            .project
            .open_rename_view_dialog(&state.model, "work", "amplifier", "missing")
            .is_err()
    );

    state
        .model
        .library_manager
        .get_library_mut("work")
        .expect("work library exists")
        .read_only = true;
    let error = state
        .project
        .open_rename_view_dialog(&state.model, "work", "amplifier", "schematic")
        .expect_err("a read-only library must fail closed");

    assert!(error.contains("read-only"), "{error}");
    assert!(!state.project.rename_view_dialog);
    assert_eq!(state.project.rename_view_library, "sentinel");
}

#[test]
fn rename_view_commit_rejects_an_intervening_catalog_change() {
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_amplifier_schematic();
    let state = &mut app.state;
    state
        .project
        .open_rename_view_dialog(&state.model, "work", "amplifier", "schematic")
        .expect("rename dialog opens");
    state.project.rename_view_name = "schematic_v2".to_owned();
    state
        .model
        .library_manager
        .add_library(Library::new("intervening"));

    let error =
        with_cx(&mut app, commit_rename_view_dialog).expect_err("a stale rename must fail closed");

    assert_eq!(error, LIBRARY_CATALOG_STALE_MESSAGE);
    let cell = app
        .state
        .model
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amplifier"))
        .expect("source cell remains");
    assert!(cell.get_view("schematic").is_some());
    assert!(cell.get_view("schematic_v2").is_none());
}

/// The reference design with its active view's rename typed into the
/// dialog, and the active view's library, cell and view.
fn renaming_the_active_view() -> (RSpiceApp, String, String, String) {
    let mut app = RSpiceApp::test_instance();
    app.state = crate::workbench::examples::hierarchy_reference::build().state;
    let active = app.state.model.workspace.content.active_view.clone();
    let state = &mut app.state;
    state
        .project
        .open_rename_view_dialog(&state.model, &active.library, &active.cell, &active.view)
        .expect("the active view can be renamed");
    state.project.rename_view_name = format!("{}_v2", active.view);
    (app, active.library, active.cell, active.view)
}

/// A rename carried out in the feature's context, with what it left handed
/// to the shell, ends where the rename carried out in place did: what it
/// left is carried out first, and only then is the rename recorded.
#[test]
fn a_renamed_view_states_what_the_rename_left_before_the_rename_itself() {
    let (mut in_place, library, cell, view) = renaming_the_active_view();
    let name = format!("{view}_v2");
    let remapped = in_place
        .state
        .rename_view(&library, &cell, &view, &name)
        .expect("the rename succeeds in place");
    let notice =
        format!("Renamed view '{view}' to '{name}'; {remapped} instance references now name it.");
    in_place
        .state
        .record(LogSeverity::Info, LogSource::Studio, notice.clone());

    let (mut handed, _, _, _) = renaming_the_active_view();
    let (handed_remapped, pruned) = with_cx(&mut handed, commit_rename_view_dialog)
        .expect("the rename succeeds in the feature's context")
        .expect("the name changed");
    assert_eq!(handed_remapped, remapped);
    handed
        .state
        .workbench
        .outbox
        .push(ProjectRequest::CatalogChanged {
            pruned,
            mutation: None,
            notice,
        });
    handed.after_contribution();

    assert_eq!(shown(&handed), shown(&in_place));
}
