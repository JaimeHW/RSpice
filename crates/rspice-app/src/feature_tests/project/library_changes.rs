//! A library catalog change carried out in the project feature's context lent
//! from RSpiceApp, with what it left handed to the shell, ends where the
//! change carried out in place did: renaming a cell, a view or a library,
//! and deleting a library.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{Cell, CellViewRef, Library, OpenCellView, SchematicState, View, ViewType};
use crate::workbench::app_state::model::LibraryRemoval;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};
use crate::workbench::project::library_dialogs::cell_ops::commit_rename_cell_dialog;
use crate::workbench::project::library_dialogs::library_ops::{
    commit_delete_library_review, commit_rename_library_dialog,
};
use crate::workbench::project::library_dialogs::rename_view::commit_rename_view_dialog;
use crate::workbench::{AppState, RSpiceApp};

use super::{published, shown, with_cx};

/// The reference design with its active cell's rename typed into the
/// dialog, and the active cell.
fn renaming_the_active_cell() -> (RSpiceApp, String, String) {
    let mut app = RSpiceApp::test_instance();
    app.state = crate::workbench::examples::hierarchy_reference::build().state;
    let active = app.state.model.workspace.content.active_view.clone();
    let (library, cell) = (active.library.clone(), active.cell.clone());
    let state = &mut app.state;
    state
        .project
        .open_rename_cell_dialog(&state.model, &library, &cell)
        .expect("the active cell can be renamed");
    state.project.rename_cell_name = format!("{cell}_v2");
    (app, library, cell)
}

/// A rename carried out in the feature's context, with what it left
/// handed to the shell, ends where the rename carried out in place did:
/// what it left is carried out first, and only then is the rename
/// recorded.
#[test]
fn a_renamed_cell_states_what_the_rename_left_before_the_rename_itself() {
    let (mut in_place, library, cell) = renaming_the_active_cell();
    let name = format!("{cell}_v2");
    let remapped = in_place
        .state
        .rename_cell(&library, &cell, &name)
        .expect("the rename succeeds in place");
    in_place.state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("Renamed cell '{cell}' to '{name}'; {remapped} instance references now name it."),
    );

    let (mut handed, _, _) = renaming_the_active_cell();
    let (handed_remapped, pruned) = with_cx(&mut handed, commit_rename_cell_dialog)
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
                "Renamed cell '{cell}' to '{name}'; {remapped} instance references now name it."
            ),
        });
    handed.after_contribution();

    assert_eq!(shown(&handed), shown(&in_place));
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
