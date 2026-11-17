//! The Library page worked from the keyboard alone: the arrows and type-ahead
//! move within a column, Left and Right between the columns, Enter opens, F2
//! renames, Delete reviews a deletion and Shift+F10 opens a row's menu; the
//! page's operations are commands the palette runs.

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use super::super::replay::{PRIMARY, Replay, only};
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::commands::{LibraryCommand, ProjectCommand};
use rspice_app_project::workbench::project::workspace_state::{LibraryDeletionTarget, ProjectPage};
use rspice_app_state::state::{Cell, CellViewRef, Library, View, ViewType};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

/// The Library page over `work`, whose `amp` holds a schematic and a symbol
/// and `bias` a schematic, beside the read-only `ref`; `amp`'s schematic is
/// selected and nothing has been pressed.
fn library_page() -> Replay {
    let mut app = RSpiceApp::test_instance();
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    let mut bias = Cell::new("bias");
    bias.add_view(View::new("schematic", ViewType::Schematic));
    let mut work = Library::new("work");
    work.add_cell(amp);
    work.add_cell(bias);
    app.state.model.library_manager.add_library(work);
    let mut reference = Library::new("ref");
    reference.read_only = true;
    app.state.model.library_manager.add_library(reference);
    app.state
        .model
        .library_manager
        .select_view("work", "amp", "schematic");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Library;
    let mut replay = Replay::of(app);
    replay.frames(Vec::new());
    replay
}

fn press(replay: &mut Replay, key: Key) {
    replay.key(key, Modifiers::NONE);
}

/// The library, cell and view the page has selected.
fn selected(replay: &Replay) -> [Option<&str>; 3] {
    let manager = &replay.app.state.model.library_manager;
    [
        manager.selected_library.as_deref(),
        manager.selected_cell.as_deref(),
        manager.selected_view.as_deref(),
    ]
}

#[test]
fn the_arrows_and_type_ahead_walk_the_columns_without_the_pointer() {
    let mut replay = library_page();

    // The page arrives with the keyboard in its Cell column.
    press(&mut replay, Key::ArrowDown);
    assert_eq!(selected(&replay)[..2], [Some("work"), Some("bias")]);
    press(&mut replay, Key::ArrowUp);
    assert_eq!(selected(&replay)[1], Some("amp"));
    replay.type_text("b");
    assert_eq!(selected(&replay)[1], Some("bias"), "type-ahead finds bias");
    press(&mut replay, Key::ArrowUp);

    press(&mut replay, Key::ArrowRight);
    press(&mut replay, Key::ArrowDown);
    assert_eq!(
        selected(&replay),
        [Some("work"), Some("amp"), Some("symbol")],
        "Right moves into the View column"
    );

    // The libraries stand by name: ref, the project's own user, work.
    press(&mut replay, Key::ArrowLeft);
    press(&mut replay, Key::ArrowLeft);
    press(&mut replay, Key::ArrowUp);
    press(&mut replay, Key::ArrowUp);
    assert_eq!(
        selected(&replay)[0],
        Some("ref"),
        "Left reaches the libraries"
    );
    replay.type_text("w");
    assert_eq!(selected(&replay)[0], Some("work"), "type-ahead finds work");
    press(&mut replay, Key::ArrowRight);
    press(&mut replay, Key::ArrowDown);
    assert_eq!(selected(&replay)[..2], [Some("work"), Some("bias")]);
}

#[test]
fn enter_opens_the_selected_view_or_a_cells_default_view() {
    let mut replay = library_page();
    press(&mut replay, Key::ArrowRight);
    press(&mut replay, Key::ArrowDown);
    press(&mut replay, Key::Enter);
    let state = &replay.app.state;
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert_eq!(
        state.model.workspace.content.active_view,
        CellViewRef::new("work", "amp", "symbol")
    );

    let mut replay = library_page();
    press(&mut replay, Key::ArrowDown);
    press(&mut replay, Key::Enter);
    assert_eq!(
        replay.app.state.model.workspace.content.active_view,
        CellViewRef::new("work", "bias", "schematic"),
        "Enter on a cell opens its schematic"
    );
}

#[test]
fn f2_renames_and_delete_reviews_the_row_the_keyboard_is_on() {
    let mut replay = library_page();
    press(&mut replay, Key::F2);
    let project = &replay.app.state.project;
    assert!(project.rename_cell_dialog);
    assert_eq!(project.rename_cell_current, "amp");

    let mut replay = library_page();
    press(&mut replay, Key::ArrowRight);
    press(&mut replay, Key::F2);
    let project = &replay.app.state.project;
    assert!(project.rename_view_dialog);
    assert_eq!(project.rename_view_current, "schematic");

    let mut replay = library_page();
    press(&mut replay, Key::ArrowDown);
    press(&mut replay, Key::Delete);
    assert_eq!(
        replay.app.state.project.library_deletion_review.target,
        Some(LibraryDeletionTarget::Cell {
            library: "work".to_owned(),
            cell: "bias".to_owned(),
        })
    );
}

#[test]
fn shift_f10_opens_the_rows_menu_with_the_bands_items() {
    let mut replay = library_page();
    let menu = replay.key(Key::F10, Modifiers::SHIFT);
    only(&menu, "Rename cell\u{2026}", Role::Button);
    only(&menu, "Delete cell\u{2026}", Role::Button);

    let mut replay = library_page();
    press(&mut replay, Key::ArrowRight);
    let menu = replay.key(Key::F10, Modifiers::SHIFT);
    only(&menu, "Where used\u{2026}", Role::Button);
    only(&menu, "Rename view\u{2026}", Role::Button);
}

/// The palette from the keyboard: opened, `label` typed into its search and
/// the row it finds run with Enter.
fn run_from_palette(replay: &mut Replay, label: &str) {
    replay.key(Key::K, PRIMARY);
    replay.type_text(label);
    press(replay, Key::Enter);
}

#[test]
fn the_library_operations_run_from_the_command_palette() {
    let mut replay = library_page();
    run_from_palette(&mut replay, "Rename library, cell or view");
    let project = &replay.app.state.project;
    assert!(project.rename_cell_dialog, "the cell the keyboard is on");
    assert_eq!(project.rename_cell_current, "amp");

    let mut replay = library_page();
    press(&mut replay, Key::ArrowRight);
    run_from_palette(&mut replay, "Delete library, cell or view");
    assert_eq!(
        replay.app.state.project.library_deletion_review.target,
        Some(LibraryDeletionTarget::View {
            library: "work".to_owned(),
            cell: "amp".to_owned(),
            view: "schematic".to_owned(),
        })
    );

    // Away from the page, what acts on its rows waits for it; the audit
    // history acts on the project's libraries and opens the page.
    let mut replay = library_page();
    replay.app.state.project.project_page = ProjectPage::Overview;
    let rename = ProjectCommand::Library(LibraryCommand::Rename).key();
    assert_eq!(
        replay.app.command_availability(rename),
        CommandAvailability::Disabled("open the Library page")
    );
    run_from_palette(&mut replay, "Library audit history");
    assert_eq!(replay.app.state.project.project_page, ProjectPage::Library);
    let shown = replay.frames(Vec::new());
    assert!(
        shown.iter().any(|control| control.label == "Audit history"),
        "the audit history opens: {shown:?}"
    );
}
