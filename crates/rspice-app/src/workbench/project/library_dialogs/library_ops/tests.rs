//! Tests for the library membership dialogs.

use super::*;
use crate::state::{Cell, CellViewRef, Library, OpenCellView, SchematicState, View, ViewType};
use crate::workbench::project::library_dialogs::shown;
use crate::workbench::project::tests::with_cx;
use crate::workbench::{AppState, RSpiceApp};

fn state_with_writable_work_library() -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amplifier");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state
}

fn app_over(state: AppState) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = state;
    app
}

#[test]
fn library_dialog_entry_points_capture_the_exact_catalog_revision() {
    let mut state = state_with_writable_work_library();
    let state = &mut state;

    state.project.open_new_library_dialog(&state.model);
    state
        .project
        .open_rename_library_dialog(&state.model, "work")
        .expect("a writable library opens Rename Library");
    state
        .project
        .open_delete_library_review(&state.model, "work")
        .expect("an existing library opens the deletion review");

    assert!(state.project.new_library_dialog);
    assert!(state.project.rename_library_dialog);
    assert!(state.project.delete_library_dialog);
    assert_eq!(state.project.rename_library_current, "work");
    assert_eq!(state.project.rename_library_name, "work");
    assert_eq!(state.project.delete_library_target, "work");
    for revision in [
        state.project.new_library_library_revision,
        state.project.rename_library_library_revision,
        state.project.delete_library_library_revision,
    ] {
        assert_eq!(revision, state.model.library_manager.revision());
    }
}

#[test]
fn library_dialog_entry_points_reject_missing_and_read_only_targets() {
    let mut state = state_with_writable_work_library();
    let state = &mut state;
    state.project.rename_library_current = "sentinel".to_owned();
    state.project.delete_library_target = "sentinel".to_owned();

    assert!(
        state
            .project
            .open_rename_library_dialog(&state.model, "missing")
            .is_err()
    );
    assert!(
        state
            .project
            .open_delete_library_review(&state.model, "missing")
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
        .open_rename_library_dialog(&state.model, "work")
        .expect_err("a read-only library must fail closed");

    assert!(error.contains("read-only"), "{error}");
    assert!(!state.project.rename_library_dialog);
    assert!(!state.project.delete_library_dialog);
    assert_eq!(state.project.rename_library_current, "sentinel");
    assert_eq!(state.project.delete_library_target, "sentinel");
}

#[test]
fn library_dialog_commits_reject_an_intervening_catalog_change() {
    let mut app = app_over(state_with_writable_work_library());
    let state = &mut app.state;
    state.project.open_new_library_dialog(&state.model);
    state.project.new_library_name = "vendor".to_owned();
    state
        .project
        .open_rename_library_dialog(&state.model, "work")
        .expect("rename dialog opens");
    state.project.rename_library_name = "work_v2".to_owned();
    state
        .project
        .open_delete_library_review(&state.model, "work")
        .expect("deletion review opens");
    state
        .model
        .library_manager
        .add_library(Library::new("intervening"));

    let Err(deletion) = with_cx(&mut app, commit_delete_library_review) else {
        panic!("a stale deletion must fail closed");
    };
    for error in [
        with_cx(&mut app, commit_new_library_dialog)
            .expect_err("a stale creation must fail closed"),
        with_cx(&mut app, commit_rename_library_dialog)
            .expect_err("a stale rename must fail closed"),
        deletion,
    ] {
        assert_eq!(error, LIBRARY_CATALOG_STALE_MESSAGE);
    }
    let libraries = &app.state.model.library_manager;
    assert!(libraries.get_library("vendor").is_none());
    assert!(libraries.get_library("work").is_some());
    assert!(libraries.get_library("work_v2").is_none());
}

#[test]
fn the_deletion_review_reports_the_same_blocker_the_transaction_enforces() {
    let mut app = app_over(state_with_writable_work_library());
    let state = &mut app.state;
    state.model.workspace.content.project.root_library = "work".to_owned();
    state.model.workspace.content.project.top_cell = "amplifier".to_owned();
    state
        .project
        .open_delete_library_review(&state.model, "work")
        .expect("deletion review opens");

    let blocker = app
        .state
        .model
        .library_deletion_blocker("work")
        .expect("the project root library is undeletable");
    let Err(committed) = with_cx(&mut app, commit_delete_library_review) else {
        panic!("the transaction refuses exactly what the review states");
    };

    assert_eq!(blocker, committed);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .is_some()
    );
}

#[test]
fn the_deletion_review_counts_what_the_library_owns() {
    let mut state = state_with_writable_work_library();
    state.model.workspace.content.open_views = vec![OpenCellView::new(
        CellViewRef::new("work", "amplifier", "schematic"),
        ViewType::Schematic,
    )];
    state.model.schematic.add_library_cell_component(
        crate::state::Point::origin(),
        crate::state::LibraryCellInstance::new("work", "amplifier", "schematic"),
    );

    let impact = library_deletion_impact(&state.model, "work");

    assert_eq!(impact.cells, 1);
    assert_eq!(impact.views, 1);
    assert_eq!(impact.open_views, 1);
    assert_eq!(impact.instance_references, 1);
    assert_eq!(impact.source_bundles, 0);
    assert_eq!(impact.configuration_roots, 0);
    assert!(!impact.project_root);
}

/// The reference design with its active library's rename typed into the
/// dialog, and the active library.
fn renaming_the_active_library() -> (RSpiceApp, String) {
    let mut app = RSpiceApp::test_instance();
    app.state = crate::workbench::examples::hierarchy_reference::build().state;
    let library = app
        .state
        .model
        .workspace
        .content
        .active_view
        .library
        .clone();
    let state = &mut app.state;
    state
        .project
        .open_rename_library_dialog(&state.model, &library)
        .expect("the active library can be renamed");
    state.project.rename_library_name = format!("{library}_v2");
    (app, library)
}

/// A rename carried out in the feature's context, with what it left handed
/// to the shell, ends where the rename carried out in place did: what it
/// left is carried out first, and only then is the rename recorded.
#[test]
fn a_renamed_library_states_what_the_rename_left_before_the_rename_itself() {
    let (mut in_place, library) = renaming_the_active_library();
    let name = format!("{library}_v2");
    let remapped = in_place
        .state
        .rename_library(&library, &name)
        .expect("the rename succeeds in place");
    in_place.state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Renamed library '{library}' to '{name}'; {remapped} instance references now name it."
        ),
    );

    let (mut handed, _) = renaming_the_active_library();
    let (handed_remapped, pruned) = with_cx(&mut handed, commit_rename_library_dialog)
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
            notice: format!(
                "Renamed library '{library}' to '{name}'; {remapped} instance references now name it."
            ),
        });
    handed.after_contribution();

    assert_eq!(shown(&handed), shown(&in_place));
}

/// A project rooted in a library of its own, with the deletion review open
/// on the library its active view is in, and that library.
fn deleting_the_active_library() -> (RSpiceApp, String) {
    let mut app = RSpiceApp::test_instance();
    app.state = AppState::default();
    let state = &mut app.state;
    let survivor = CellViewRef::new("spare", "keep", "schematic");
    let mut spare = Library::new("spare");
    let mut keep = Cell::new("keep");
    keep.add_view(View::new("schematic", ViewType::Schematic));
    spare.add_cell(keep);
    state.model.library_manager.add_library(spare);
    state
        .model
        .workspace
        .insert_schematic_editor(survivor.key(), SchematicState::default());
    let content = &mut state.model.workspace.content;
    content
        .open_views
        .push(OpenCellView::new(survivor, ViewType::Schematic));
    content.project.root_library = "spare".to_owned();
    content.project.top_cell = "keep".to_owned();
    content.project_sources = Default::default();
    content.configuration_sets = Default::default();
    let library = content.active_view.library.clone();
    state
        .project
        .open_delete_library_review(&state.model, &library)
        .expect("the active library can be deleted");
    (app, library)
}

/// What the deletion published: the project's library receipts, the
/// catalog's revision and the design's execution epoch.
fn published(app: &RSpiceApp) -> (Vec<(crate::state::ProjectLibraryMutation, u64)>, u64, u64) {
    let model = &app.state.model;
    (
        model
            .workspace
            .content
            .project
            .library_mutation_audit()
            .iter()
            .map(|receipt| {
                (
                    receipt.mutation().clone(),
                    receipt.target_library_revision(),
                )
            })
            .collect(),
        model.library_manager.revision(),
        model.design_execution_epoch,
    )
}

/// A deletion carried out in the feature's context, with what it left and
/// the revision it reserved handed to the shell, ends where the deletion
/// carried out in place did: what it left is carried out first, then the
/// revision is published, and only then is the deletion recorded.
#[test]
fn a_deleted_library_publishes_its_revision_between_what_it_left_and_its_own_record() {
    let (mut in_place, library) = deleting_the_active_library();
    let cells = in_place
        .state
        .delete_library(&library)
        .expect("the deletion succeeds in place");
    let notice = |cells: usize| {
        format!(
            "Deleted library '{library}' and its {cells} cell{}.",
            if cells == 1 { "" } else { "s" }
        )
    };
    in_place
        .state
        .record(LogSeverity::Info, LogSource::Studio, notice(cells));

    let (mut handed, _) = deleting_the_active_library();
    let Ok(LibraryRemoval {
        cells: handed_cells,
        pruned,
        mutation,
    }) = with_cx(&mut handed, commit_delete_library_review)
    else {
        panic!("the deletion succeeds in the feature's context");
    };
    assert_eq!(handed_cells, cells);
    handed
        .state
        .workbench
        .outbox
        .push(ProjectRequest::CatalogChanged {
            pruned,
            mutation: Some(ReservedLibraryRevision::new(mutation)),
            notice: notice(cells),
        });
    handed.after_contribution();

    assert!(
        handed
            .state
            .model
            .library_manager
            .get_library(&library)
            .is_none()
    );
    assert_eq!(shown(&handed), shown(&in_place));
    assert_eq!(published(&handed), published(&in_place));
}
