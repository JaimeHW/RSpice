//! Tests for the library membership dialogs.

use super::*;
use crate::state::{Cell, CellViewRef, Library, OpenCellView, View, ViewType};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::project::tests::harness;

fn harness_with_writable_work_library() -> FeatureHarness<ProjectFeature> {
    let mut harness = harness();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amplifier");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    harness.model.library_manager.add_library(library);
    harness
}

#[test]
fn library_dialog_entry_points_capture_the_exact_catalog_revision() {
    let mut harness = harness_with_writable_work_library();
    let harness = &mut harness;

    harness.state.open_new_library_dialog(&harness.model);
    harness
        .state
        .open_rename_library_dialog(&harness.model, "work")
        .expect("a writable library opens Rename Library");
    harness
        .state
        .open_delete_library_review(&harness.model, "work")
        .expect("an existing library opens the deletion review");

    assert!(harness.state.new_library_dialog);
    assert!(harness.state.rename_library_dialog);
    assert!(harness.state.delete_library_dialog);
    assert_eq!(harness.state.rename_library_current, "work");
    assert_eq!(harness.state.rename_library_name, "work");
    assert_eq!(harness.state.delete_library_target, "work");
    for revision in [
        harness.state.new_library_library_revision,
        harness.state.rename_library_library_revision,
        harness.state.delete_library_library_revision,
    ] {
        assert_eq!(revision, harness.model.library_manager.revision());
    }
}

#[test]
fn library_dialog_entry_points_reject_missing_and_read_only_targets() {
    let mut harness = harness_with_writable_work_library();
    let harness = &mut harness;
    harness.state.rename_library_current = "sentinel".to_owned();
    harness.state.delete_library_target = "sentinel".to_owned();

    assert!(
        harness
            .state
            .open_rename_library_dialog(&harness.model, "missing")
            .is_err()
    );
    assert!(
        harness
            .state
            .open_delete_library_review(&harness.model, "missing")
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
        .open_rename_library_dialog(&harness.model, "work")
        .expect_err("a read-only library must fail closed");

    assert!(error.contains("read-only"), "{error}");
    assert!(!harness.state.rename_library_dialog);
    assert!(!harness.state.delete_library_dialog);
    assert_eq!(harness.state.rename_library_current, "sentinel");
    assert_eq!(harness.state.delete_library_target, "sentinel");
}

#[test]
fn library_dialog_commits_reject_an_intervening_catalog_change() {
    let mut harness = harness_with_writable_work_library();
    let harness = &mut harness;
    harness.state.open_new_library_dialog(&harness.model);
    harness.state.new_library_name = "vendor".to_owned();
    harness
        .state
        .open_rename_library_dialog(&harness.model, "work")
        .expect("rename dialog opens");
    harness.state.rename_library_name = "work_v2".to_owned();
    harness
        .state
        .open_delete_library_review(&harness.model, "work")
        .expect("deletion review opens");
    harness
        .model
        .library_manager
        .add_library(Library::new("intervening"));

    let Err(deletion) = commit_delete_library_review(&mut harness.cx()) else {
        panic!("a stale deletion must fail closed");
    };
    for error in [
        commit_new_library_dialog(&mut harness.cx())
            .expect_err("a stale creation must fail closed"),
        commit_rename_library_dialog(&mut harness.cx())
            .expect_err("a stale rename must fail closed"),
        deletion,
    ] {
        assert_eq!(error, LIBRARY_CATALOG_STALE_MESSAGE);
    }
    let libraries = &harness.model.library_manager;
    assert!(libraries.get_library("vendor").is_none());
    assert!(libraries.get_library("work").is_some());
    assert!(libraries.get_library("work_v2").is_none());
}

#[test]
fn the_deletion_review_reports_the_same_blocker_the_transaction_enforces() {
    let mut harness = harness_with_writable_work_library();
    let harness = &mut harness;
    harness.model.workspace.content.project.root_library = "work".to_owned();
    harness.model.workspace.content.project.top_cell = "amplifier".to_owned();
    harness
        .state
        .open_delete_library_review(&harness.model, "work")
        .expect("deletion review opens");

    let blocker = harness
        .model
        .library_deletion_blocker("work")
        .expect("the project root library is undeletable");
    let Err(committed) = commit_delete_library_review(&mut harness.cx()) else {
        panic!("the transaction refuses exactly what the review states");
    };

    assert_eq!(blocker, committed);
    assert!(harness.model.library_manager.get_library("work").is_some());
}

#[test]
fn the_deletion_review_counts_what_the_library_owns() {
    let mut harness = harness_with_writable_work_library();
    harness.model.workspace.content.open_views = vec![OpenCellView::new(
        CellViewRef::new("work", "amplifier", "schematic"),
        ViewType::Schematic,
    )];
    harness.model.schematic.add_library_cell_component(
        crate::state::Point::origin(),
        crate::state::LibraryCellInstance::new("work", "amplifier", "schematic"),
    );

    let impact = library_deletion_impact(&harness.model, "work");

    assert_eq!(impact.cells, 1);
    assert_eq!(impact.views, 1);
    assert_eq!(impact.open_views, 1);
    assert_eq!(impact.instance_references, 1);
    assert_eq!(impact.source_bundles, 0);
    assert_eq!(impact.configuration_roots, 0);
    assert!(!impact.project_root);
}
