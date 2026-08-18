//! Tests for Rename View.

use super::*;
use crate::state::{Cell, Library, View, ViewType};

fn state_with_amplifier_schematic() -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amplifier");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state
}

#[test]
fn rename_view_dialog_starts_with_the_exact_active_view_identity() {
    let mut state = state_with_amplifier_schematic();

    state
        .open_rename_view_dialog("work", "amplifier", "schematic")
        .expect("a writable view opens Rename View");

    assert!(state.dialogs.rename_view_dialog);
    assert_eq!(state.dialogs.rename_view_library, "work");
    assert_eq!(state.dialogs.rename_view_cell, "amplifier");
    assert_eq!(state.dialogs.rename_view_current, "schematic");
    assert_eq!(state.dialogs.rename_view_name, "schematic");
    assert!(state.dialogs.rename_view_error.is_none());
    assert_eq!(
        state.dialogs.rename_view_library_revision,
        state.library_manager.revision()
    );
}

#[test]
fn rename_view_entry_point_rejects_missing_and_read_only_targets() {
    let mut state = state_with_amplifier_schematic();
    state.dialogs.rename_view_library = "sentinel".to_owned();

    assert!(
        state
            .open_rename_view_dialog("work", "amplifier", "missing")
            .is_err()
    );

    state
        .library_manager
        .get_library_mut("work")
        .expect("work library exists")
        .read_only = true;
    let error = state
        .open_rename_view_dialog("work", "amplifier", "schematic")
        .expect_err("a read-only library must fail closed");

    assert!(error.contains("read-only"), "{error}");
    assert!(!state.dialogs.rename_view_dialog);
    assert_eq!(state.dialogs.rename_view_library, "sentinel");
}

#[test]
fn rename_view_commit_rejects_an_intervening_catalog_change() {
    let mut state = state_with_amplifier_schematic();
    state
        .open_rename_view_dialog("work", "amplifier", "schematic")
        .expect("rename dialog opens");
    state.dialogs.rename_view_name = "schematic_v2".to_owned();
    state
        .library_manager
        .add_library(Library::new("intervening"));

    let error = state
        .commit_rename_view_dialog()
        .expect_err("a stale rename must fail closed");

    assert_eq!(error, LIBRARY_CATALOG_STALE_MESSAGE);
    let cell = state
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amplifier"))
        .expect("source cell remains");
    assert!(cell.get_view("schematic").is_some());
    assert!(cell.get_view("schematic_v2").is_none());
}
