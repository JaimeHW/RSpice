//! Create symbol from schematic, carried out through whole frames of the
//! application on a project that has just been created.

use egui::accesskit::Role;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::RSpiceApp;
use crate::workbench::app::project_transactions::open_new_project_dialog;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::library_dialogs::new_view::{
    CREATABLE_VIEW_TYPES, CREATE_SYMBOL_TITLE, view_type_label,
};
use crate::workbench::state::Workspace;

use super::super::replay::{Control, Replay};

/// A project just created under the defaults New project offers, holding
/// only its top cell's schematic, with the Design workspace in front.
fn new_project() -> Replay {
    let mut app = RSpiceApp::test_instance();
    open_new_project_dialog(&mut app.state);
    app.state.project.new_project.name = "untitled".to_owned();
    let params = app.state.project.new_project.params();
    app.state
        .create_new_project_with(&params)
        .expect("the default identity creates a project");
    // As the shell closes New project once it has created the project.
    app.state.project.new_project = Default::default();
    app.state.workbench.activate(Workspace::Design);
    Replay::of(app)
}

/// The view type the catalog records for `reference`.
fn catalog_type(app: &RSpiceApp, reference: &CellViewRef) -> Option<ViewType> {
    app.state
        .model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
}

/// Whether a frame drew the symbol editor's canvas, and whether it drew the
/// schematic canvas.
fn canvases(controls: &[Control]) -> (bool, bool) {
    let canvas = |prefix: &str| {
        controls
            .iter()
            .any(|control| control.role == Role::Canvas && control.label.starts_with(prefix))
    };
    (canvas("Symbol editor canvas."), canvas("Schematic canvas"))
}

/// Create symbol on a new project's schematic adds a symbol view to the top
/// cell and opens it in the symbol editor, not in a schematic canvas.
#[test]
fn create_symbol_on_a_new_project_opens_the_symbol_editor() {
    let mut replay = new_project();
    let top = replay.app.state.model.workspace.content.active_view.clone();
    assert_eq!((top.cell.as_str(), top.view.as_str()), ("top", "schematic"));
    replay.frames(Vec::new());

    replay
        .app
        .execute_command_key(ProjectCommand::CreateSymbol.key());
    replay.frames(Vec::new());
    let controls = replay.click("Create", Role::Button);

    let symbol = CellViewRef::new(&top.library, &top.cell, "symbol");
    assert_eq!(
        catalog_type(&replay.app, &symbol),
        Some(ViewType::Symbol),
        "the library holds a symbol view named symbol"
    );
    let content = &replay.app.state.model.workspace.content;
    assert_eq!(content.active_view, symbol);
    assert_eq!(content.active_view_type(), Some(ViewType::Symbol));
    assert_eq!(
        canvases(&controls),
        (true, false),
        "the symbol editor is drawn in place of the schematic canvas"
    );
}

/// Opening a view the library does not hold is refused on the status bar;
/// no schematic document is made up for it.
#[test]
fn a_view_the_library_does_not_hold_is_refused_not_opened_as_a_schematic() {
    let mut replay = new_project();
    let top = replay.app.state.model.workspace.content.active_view.clone();
    let gone = CellViewRef::new(&top.library, &top.cell, "symbol");

    replay.app.state.open_workspace_view(gone.clone());

    let content = &replay.app.state.model.workspace.content;
    assert_eq!(content.active_view, top);
    assert!(content.open_views.iter().all(|open| open.reference != gone));
    assert!(!content.schematic_buffers.contains_key(&gone.key()));
    assert_eq!(catalog_type(&replay.app, &gone), None);
    assert!(
        replay
            .app
            .state
            .prompt_text()
            .is_some_and(|prompt| prompt.contains("is not in the library")),
        "{:?}",
        replay.app.state.prompt_text()
    );
}

/// Create symbol is a form of its own: titled for what it makes, naming the
/// cell it makes it for, with no view type to choose.
#[test]
fn create_symbol_is_its_own_form_without_a_type_to_choose() {
    let mut replay = new_project();
    replay
        .app
        .execute_command_key(ProjectCommand::CreateSymbol.key());
    let controls = replay.frames(Vec::new());

    let dialogs = controls
        .iter()
        .filter(|control| control.role == Role::Dialog)
        .map(|control| control.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(dialogs, [CREATE_SYMBOL_TITLE]);
    assert!(
        controls.iter().any(|control| control.role == Role::Label
            && control.label == "Cell"
            && control.value == "top"),
        "the form names the cell"
    );
    let types = CREATABLE_VIEW_TYPES.map(view_type_label);
    let offered = controls
        .iter()
        .filter(|control| {
            control.role == Role::RadioButton && types.contains(&control.label.as_str())
        })
        .map(|control| control.label.as_str())
        .collect::<Vec<_>>();
    assert!(offered.is_empty(), "a type switcher is drawn: {offered:?}");
}

/// Create symbol on a cell that already has a symbol opens that symbol in
/// the symbol editor and offers no second one.
#[test]
fn create_symbol_opens_the_symbol_the_cell_already_has() {
    let mut replay = new_project();
    replay.frames(Vec::new());
    replay
        .app
        .execute_command_key(ProjectCommand::CreateSymbol.key());
    replay.click("Create", Role::Button);
    let symbol = replay.app.state.model.workspace.content.active_view.clone();
    let top = CellViewRef::new(&symbol.library, &symbol.cell, "schematic");
    replay.app.state.open_workspace_view(top.clone());
    assert_eq!(replay.app.state.model.workspace.content.active_view, top);
    let views_before = replay
        .app
        .state
        .model
        .library_manager
        .get_library(&top.library)
        .and_then(|library| library.get_cell(&top.cell))
        .map(|cell| cell.views.len());

    replay
        .app
        .execute_command_key(ProjectCommand::CreateSymbol.key());
    let controls = replay.frames(Vec::new());

    assert!(
        controls.iter().all(|control| control.role != Role::Dialog),
        "no form is offered for a second symbol"
    );
    let content = &replay.app.state.model.workspace.content;
    assert_eq!(content.active_view, symbol);
    assert_eq!(content.active_view_type(), Some(ViewType::Symbol));
    assert_eq!(canvases(&controls), (true, false));
    assert_eq!(
        replay
            .app
            .state
            .model
            .library_manager
            .get_library(&top.library)
            .and_then(|library| library.get_cell(&top.cell))
            .map(|cell| cell.views.len()),
        views_before
    );
}
