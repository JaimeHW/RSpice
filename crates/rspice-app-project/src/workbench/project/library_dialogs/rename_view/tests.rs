//! Tests for Rename View.

use super::*;
use crate::state::{Cell, Library, View};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::project::tests::harness;

fn harness_with_amplifier_schematic() -> FeatureHarness<ProjectFeature> {
    let mut harness = harness();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amplifier");
    cell.add_view(View::new("schematic", crate::state::ViewType::Schematic));
    library.add_cell(cell);
    harness.model.library_manager.add_library(library);
    harness
}

#[test]
fn rename_view_dialog_starts_with_the_exact_active_view_identity() {
    let mut harness = harness_with_amplifier_schematic();
    let harness = &mut harness;

    harness
        .state
        .open_rename_view_dialog(&harness.model, "work", "amplifier", "schematic")
        .expect("a writable view opens Rename View");

    assert!(harness.state.rename_view_dialog);
    assert_eq!(harness.state.rename_view_library, "work");
    assert_eq!(harness.state.rename_view_cell, "amplifier");
    assert_eq!(harness.state.rename_view_current, "schematic");
    assert_eq!(harness.state.rename_view_name, "schematic");
    assert!(harness.state.rename_view_error.is_none());
    assert_eq!(
        harness.state.rename_view_library_revision,
        harness.model.library_manager.revision()
    );
}

#[test]
fn rename_view_entry_point_rejects_missing_and_read_only_targets() {
    let mut harness = harness_with_amplifier_schematic();
    let harness = &mut harness;
    harness.state.rename_view_library = "sentinel".to_owned();

    assert!(
        harness
            .state
            .open_rename_view_dialog(&harness.model, "work", "amplifier", "missing")
            .is_err()
    );

    harness
        .model
        .library_manager
        .get_library_mut("work")
        .expect("work library exists")
        .read_only = true;
    let error = harness
        .state
        .open_rename_view_dialog(&harness.model, "work", "amplifier", "schematic")
        .expect_err("a read-only library must fail closed");

    assert!(error.contains("read-only"), "{error}");
    assert!(!harness.state.rename_view_dialog);
    assert_eq!(harness.state.rename_view_library, "sentinel");
}

#[test]
fn rename_view_commit_rejects_an_intervening_catalog_change() {
    let mut harness = harness_with_amplifier_schematic();
    let harness = &mut harness;
    harness
        .state
        .open_rename_view_dialog(&harness.model, "work", "amplifier", "schematic")
        .expect("rename dialog opens");
    harness.state.rename_view_name = "schematic_v2".to_owned();
    harness
        .model
        .library_manager
        .add_library(Library::new("intervening"));

    let error =
        commit_rename_view_dialog(&mut harness.cx()).expect_err("a stale rename must fail closed");

    assert_eq!(error, LIBRARY_CATALOG_STALE_MESSAGE);
    let cell = harness
        .model
        .library_manager
        .get_library("work")
        .and_then(|library| library.get_cell("amplifier"))
        .expect("source cell remains");
    assert!(cell.get_view("schematic").is_some());
    assert!(cell.get_view("schematic_v2").is_none());
}
