//! Tests for the library membership dialogs.

use super::*;
use crate::state::{Cell, Library, View, ViewType};

fn state_with_writable_work_library() -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amplifier");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state
}

#[test]
fn library_dialog_entry_points_capture_the_exact_catalog_revision() {
    let mut state = state_with_writable_work_library();

    state.open_new_library_dialog();
    state
        .open_rename_library_dialog("work")
        .expect("a writable library opens Rename Library");
    state
        .open_delete_library_review("work")
        .expect("an existing library opens the deletion review");

    assert!(state.dialogs.new_library_dialog);
    assert!(state.dialogs.rename_library_dialog);
    assert!(state.dialogs.delete_library_dialog);
    assert_eq!(state.dialogs.rename_library_current, "work");
    assert_eq!(state.dialogs.rename_library_name, "work");
    assert_eq!(state.dialogs.delete_library_target, "work");
    for revision in [
        state.dialogs.new_library_library_revision,
        state.dialogs.rename_library_library_revision,
        state.dialogs.delete_library_library_revision,
    ] {
        assert_eq!(revision, state.library_manager.revision());
    }
}

#[test]
fn library_dialog_entry_points_reject_missing_and_read_only_targets() {
    let mut state = state_with_writable_work_library();
    state.dialogs.rename_library_current = "sentinel".to_owned();
    state.dialogs.delete_library_target = "sentinel".to_owned();

    assert!(state.open_rename_library_dialog("missing").is_err());
    assert!(state.open_delete_library_review("missing").is_err());

    state
        .library_manager
        .get_library_mut("work")
        .expect("work library exists")
        .read_only = true;
    let error = state
        .open_rename_library_dialog("work")
        .expect_err("a read-only library must fail closed");

    assert!(error.contains("read-only"), "{error}");
    assert!(!state.dialogs.rename_library_dialog);
    assert!(!state.dialogs.delete_library_dialog);
    assert_eq!(state.dialogs.rename_library_current, "sentinel");
    assert_eq!(state.dialogs.delete_library_target, "sentinel");
}

#[test]
fn library_dialog_commits_reject_an_intervening_catalog_change() {
    let mut state = state_with_writable_work_library();
    state.open_new_library_dialog();
    state.dialogs.new_library_name = "vendor".to_owned();
    state
        .open_rename_library_dialog("work")
        .expect("rename dialog opens");
    state.dialogs.rename_library_name = "work_v2".to_owned();
    state
        .open_delete_library_review("work")
        .expect("deletion review opens");
    state
        .library_manager
        .add_library(Library::new("intervening"));

    for error in [
        state
            .commit_new_library_dialog()
            .expect_err("a stale creation must fail closed"),
        state
            .commit_rename_library_dialog()
            .expect_err("a stale rename must fail closed"),
        state
            .commit_delete_library_review()
            .expect_err("a stale deletion must fail closed"),
    ] {
        assert_eq!(error, LIBRARY_CATALOG_STALE_MESSAGE);
    }
    assert!(state.library_manager.get_library("vendor").is_none());
    assert!(state.library_manager.get_library("work").is_some());
    assert!(state.library_manager.get_library("work_v2").is_none());
}

#[test]
fn the_deletion_review_reports_the_same_blocker_the_transaction_enforces() {
    let mut state = state_with_writable_work_library();
    state.workspace.project.root_library = "work".to_owned();
    state.workspace.project.top_cell = "amplifier".to_owned();
    state
        .open_delete_library_review("work")
        .expect("deletion review opens");

    let blocker = state
        .library_deletion_blocker("work")
        .expect("the project root library is undeletable");
    let committed = state
        .commit_delete_library_review()
        .expect_err("the transaction refuses exactly what the review states");

    assert_eq!(blocker, committed);
    assert!(state.library_manager.get_library("work").is_some());
}

#[test]
fn the_deletion_review_counts_what_the_library_owns() {
    let mut state = state_with_writable_work_library();
    state.workspace.open_views = vec![crate::state::OpenCellView::new(
        crate::state::CellViewRef::new("work", "amplifier", "schematic"),
        ViewType::Schematic,
    )];
    state.schematic.add_library_cell_component(
        crate::state::Point::origin(),
        crate::state::LibraryCellInstance::new("work", "amplifier", "schematic"),
    );

    let impact = library_deletion_impact(&state, "work");

    assert_eq!(impact.cells, 1);
    assert_eq!(impact.views, 1);
    assert_eq!(impact.open_views, 1);
    assert_eq!(impact.instance_references, 1);
    assert_eq!(impact.source_bundles, 0);
    assert_eq!(impact.configuration_roots, 0);
    assert!(!impact.project_root);
}
