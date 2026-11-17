//! The Library page worked from the keyboard alone: the arrows and type-ahead
//! move within a column, Left and Right between the columns, Enter opens, F2
//! renames, Delete reviews a deletion and Shift+F10 opens a row's menu; the
//! page's operations are commands the palette runs. A cell dragged onto a
//! library opens Copy Cell aimed at it.

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

/// Ctrl+A chooses every cell of the column, Delete opens one review of all
/// of them, Enter confirms it, and one Undo puts all of them back.
#[test]
fn twelve_cells_chosen_together_are_one_review_and_one_undo() {
    let mut app = RSpiceApp::test_instance();
    let mut bulk = Library::new("bulk");
    for n in 1..=12 {
        let mut cell = Cell::new(format!("cell{n:02}"));
        cell.add_view(View::new("schematic", ViewType::Schematic));
        bulk.add_cell(cell);
    }
    app.state.model.library_manager.add_library(bulk);
    app.state
        .model
        .library_manager
        .select_cell("bulk", "cell01");
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Library;
    let mut replay = Replay::of(app);
    replay.frames(Vec::new());
    let cells = |replay: &Replay| {
        replay
            .app
            .state
            .model
            .library_manager
            .get_library("bulk")
            .map(|library| library.cell_count())
    };

    replay.key(Key::A, PRIMARY);
    press(&mut replay, Key::Delete);
    assert_eq!(
        replay
            .app
            .state
            .project
            .library_deletion_review
            .targets()
            .len(),
        12,
        "one review holds every chosen cell"
    );
    press(&mut replay, Key::Enter);
    assert_eq!(cells(&replay), Some(0), "the review deletes all twelve");
    replay.key(Key::Z, PRIMARY);
    assert_eq!(cells(&replay), Some(12), "one Undo puts all twelve back");
    replay.key(Key::Z, PRIMARY | Modifiers::SHIFT);
    assert_eq!(cells(&replay), Some(0), "one Redo deletes them again");
}

/// The Library page with `analog` beside `work`: its `divider` holds a
/// schematic, its `ldo` a schematic and a symbol.
fn library_page_with_analog() -> Replay {
    let mut replay = library_page();
    let mut divider = Cell::new("divider");
    divider.add_view(View::new("schematic", ViewType::Schematic));
    let mut ldo = Cell::new("ldo");
    ldo.add_view(View::new("schematic", ViewType::Schematic));
    ldo.add_view(View::new("symbol", ViewType::Symbol));
    let mut analog = Library::new("analog");
    analog.add_cell(divider);
    analog.add_cell(ldo);
    replay.app.state.model.library_manager.add_library(analog);
    replay.frames(Vec::new());
    replay
}

/// The labels of the listed rows: a library's reads "name, cells".
fn rows(controls: &[super::super::replay::Control]) -> Vec<&str> {
    controls
        .iter()
        .filter(|control| control.role == Role::ListBoxOption)
        .map(|control| control.label.as_str())
        .collect()
}

/// The find searches every library from the keyboard: Shift+Tab reaches it
/// from the lists, the selection moves to the first cell it turns up, the
/// Library column lists only the libraries holding one, and Enter takes the
/// keyboard to that cell.
#[test]
fn the_find_searches_every_library_and_selects_what_it_finds() {
    let mut replay = library_page_with_analog();
    // From the Cell column back past the Library column and the view type.
    for _ in 0..3 {
        replay.key(Key::Tab, Modifiers::SHIFT);
    }
    replay.type_text("divid");
    assert_eq!(replay.app.state.project.project_library_filter, "divid");
    assert_eq!(
        selected(&replay),
        [Some("analog"), Some("divider"), Some("schematic")],
        "the find selects what it turns up in another library"
    );
    let controls = replay.frames(Vec::new());
    let listed = rows(&controls);
    assert!(listed.contains(&"analog, 1"), "{listed:?}");
    assert!(
        !listed.iter().any(|row| row.starts_with("work")),
        "a library the find turns nothing up in is not listed: {listed:?}"
    );

    press(&mut replay, Key::Enter);
    press(&mut replay, Key::F2);
    let project = &replay.app.state.project;
    assert!(
        project.rename_cell_dialog,
        "Enter took the keyboard to the cell"
    );
    assert_eq!(project.rename_cell_current, "divider");
}

/// The view type narrows the cells to those holding a view of it and the
/// views to that type, in every library; the selected view gives way to one
/// of the type.
#[test]
fn the_view_type_narrows_every_library_to_cells_holding_one() {
    let mut replay = library_page_with_analog();
    for _ in 0..2 {
        replay.key(Key::Tab, Modifiers::SHIFT);
    }
    replay.type_text("sym");
    assert_eq!(
        replay.app.state.project.library_browser.view_type,
        Some(ViewType::Symbol)
    );
    assert_eq!(
        selected(&replay),
        [Some("work"), Some("amp"), Some("symbol")],
        "amp holds a symbol, which is now its selected view"
    );
    let controls = replay.frames(Vec::new());
    let listed = rows(&controls);
    for row in ["analog, 1", "work, 1", "amp, 2", "symbol, symbol"] {
        assert!(listed.contains(&row), "{row:?} in {listed:?}");
    }
    assert!(
        !listed
            .iter()
            .any(|row| row.starts_with("bias") || row.starts_with("schematic")),
        "{listed:?}"
    );
}

/// Every word one frame states, labelled or not.
fn words(output: &egui::FullOutput) -> Vec<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .map(str::to_owned)
        .collect()
}

/// A find that hides the selected view hides it everywhere: the strip, the
/// band and the inspector name none, and the selection waits behind the find
/// for it to be cleared.
#[test]
fn a_view_the_find_hides_is_described_nowhere() {
    let mut replay = library_page();
    let path = "work/amp/schematic";
    let said = words(&replay.frame(Vec::new()));
    assert!(
        said.iter().any(|text| text == path),
        "the page states the selection: {said:#?}"
    );
    for _ in 0..3 {
        replay.key(Key::Tab, Modifiers::SHIFT);
    }
    replay.type_text("nothing");
    assert_eq!(replay.app.state.project.project_library_filter, "nothing");
    replay.frames(Vec::new());
    let said = words(&replay.frame(Vec::new()));
    assert!(
        said.iter()
            .any(|text| text == "Select a view to inspect or open"),
        "the band asks for a view: {said:#?}"
    );
    assert!(
        !said.iter().any(|text| text.contains(path)),
        "nothing describes the hidden view: {said:#?}"
    );
    assert_eq!(
        selected(&replay),
        [Some("work"), Some("amp"), Some("schematic")],
        "the selection waits behind the find"
    );
}

/// A press on `from`, carried across in steps and let go on `to`.
fn drag(replay: &mut Replay, from: egui::Pos2, to: egui::Pos2) {
    let button = |at: egui::Pos2, pressed: bool| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(from), button(from, true)]);
    for step in 1..=6 {
        let at = from + (to - from) * (step as f32 / 6.0);
        replay.frames(vec![egui::Event::PointerMoved(at)]);
    }
    replay.frames(vec![button(to, false)]);
}

/// A cell dragged onto another library opens Copy Cell aimed at it, under
/// the cell's own name; one dragged onto a read-only library opens nothing.
#[test]
fn a_cell_dragged_onto_a_library_opens_copy_cell_aimed_at_it() {
    let mut replay = library_page_with_analog();
    let controls = replay.frames(Vec::new());
    let bias = only(&controls, "bias, 1", Role::ListBoxOption).center;
    let analog = only(&controls, "analog, 2", Role::ListBoxOption).center;
    let reference = only(&controls, "ref, 0, read-only", Role::ListBoxOption).center;

    drag(&mut replay, bias, reference);
    assert!(
        !replay.app.state.project.copy_cell_dialog,
        "a read-only library takes no copy"
    );

    drag(&mut replay, bias, analog);
    let project = &replay.app.state.project;
    assert!(project.copy_cell_dialog, "Copy Cell opens");
    assert_eq!(
        (
            project.copy_cell_source_library.as_str(),
            project.copy_cell_source_cell.as_str()
        ),
        ("work", "bias")
    );
    assert_eq!(project.copy_cell_target_library, "analog");
    assert_eq!(project.copy_cell_name, "bias");
}

/// On a phone the page is wider than the screen, and the band still ends
/// at the screen's edge: Open stands whole beside the two menus, and the
/// selection's path, with no room left, gives way to them.
#[test]
fn on_a_phone_the_band_keeps_open_on_the_screen() {
    let mut replay = library_page().sized(egui::vec2(390.0, 844.0));
    replay.app.state.workbench.coarse_pointer = true;
    let controls = replay.frames(Vec::new());
    let open = only(&controls, "Open schematic", Role::Button);
    assert!(
        open.center.x <= 390.0 - 40.0,
        "Open is whole on the screen: {:?}",
        open.center
    );
    only(&controls, "Cell actions", Role::Button);
}
