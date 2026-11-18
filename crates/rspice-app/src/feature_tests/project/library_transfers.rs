//! Copying a library, a view, or a cell with the cells it instantiates,
//! moving a cell, editing a library's or a cell's properties and opening a
//! view read-only: what each leaves in the catalog and the drawings, and
//! what a save writes and the reopened project reads back.

#![cfg(not(target_arch = "wasm32"))]

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::commands::ProjectRequest;
use rspice_app_project::workbench::project::library_dialogs::properties::commit_library_properties;
use rspice_app_project::workbench::project::library_dialogs::transfer::{
    TransferKind, commit_library_transfer,
};
use rspice_app_project::workbench::surfaces::project::library::{
    LibraryIntent, apply_library_intent,
};
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, OpenCellView, Point,
    SchematicState, View, ViewType,
};
use rspice_app_state::workbench::app_state::model::HierarchyCopy;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

use super::with_cx;

const BOTH: HierarchyCopy = HierarchyCopy {
    include_children: true,
    repoint: true,
};

fn schematic(library: &str, cell: &str) -> CellViewRef {
    CellViewRef::new(library, cell, "schematic")
}

/// `parts` holds `amp`, whose schematic places `stage`, whose schematic
/// places `leaf`; `user/top`, in front, places `amp`, and `stage` is open in
/// a tab too. `work` is a writable library with nothing in it. As just
/// saved.
fn hierarchy() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let state = &mut app.state;
    let mut parts = Library::new("parts");
    for name in ["amp", "stage", "leaf"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        parts.add_cell(cell);
    }
    state.model.library_manager.add_library(parts);
    state
        .model
        .library_manager
        .add_library(Library::new("work"));

    let placing = |library: &str, cell: &str| {
        let mut drawing = SchematicState::default();
        drawing.add_library_cell_component(
            Point::new(10, 10),
            LibraryCellInstance::new(library, cell, "schematic"),
        );
        drawing
    };
    let mut leaf = SchematicState::default();
    leaf.add_component(ComponentType::Resistor, Point::new(0, 0));
    let top = schematic("user", "top");
    let top_drawing = placing("parts", "amp");
    let workspace = &mut state.model.workspace;
    workspace.insert_schematic_editor(top.key(), top_drawing.clone());
    workspace.insert_schematic_editor(schematic("parts", "amp").key(), placing("parts", "stage"));
    workspace.insert_schematic_editor(schematic("parts", "stage").key(), placing("parts", "leaf"));
    workspace.insert_schematic_editor(schematic("parts", "leaf").key(), leaf);
    workspace.content.open_views = [&top, &schematic("parts", "stage")]
        .into_iter()
        .map(|reference| OpenCellView::new(reference.clone(), ViewType::Schematic))
        .collect();
    workspace.content.active_view = top;
    workspace.content.reroot_active_occurrence();
    state.model.schematic = top_drawing;
    state.model.workspace.mark_all_clean();
    state.model.schematic.session.is_dirty = false;
    state.open_project_session();
    state.workbench.activate(Workspace::Project);
    app
}

/// The cells `library`/`cell`'s schematic places, as library and cell, read
/// from the editor for the drawing in front.
fn placed(app: &RSpiceApp, library: &str, cell: &str) -> Vec<(String, String)> {
    let reference = schematic(library, cell);
    let content = &app.state.model.workspace.content;
    let drawing = if content.active_schematic_reference() == reference {
        app.state.model.schematic.clone()
    } else {
        app.state
            .model
            .workspace
            .clone_schematic_editor(&reference.key())
            .unwrap_or_else(|| panic!("{} has a drawing", reference.display_path()))
    };
    drawing
        .document()
        .components
        .iter()
        .filter_map(|component| component.library_cell.as_ref())
        .map(|binding| (binding.library.clone(), binding.cell.clone()))
        .collect()
}

fn pair(library: &str, cell: &str) -> Vec<(String, String)> {
    vec![(library.to_owned(), cell.to_owned())]
}

/// `app`'s project saved to a file of its own and opened again in a new
/// session.
fn saved_and_reopened(app: &mut RSpiceApp, label: &str) -> RSpiceApp {
    let folder = unique_temp_dir(label);
    let path = folder.join("transfers.rspiceproj");
    assert!(
        app.state.save_project_to_path(&path),
        "the project is saved"
    );
    let mut reopened = RSpiceApp::test_instance();
    assert!(
        reopened.state.load_project_from_path(&path),
        "the saved project opens"
    );
    std::fs::remove_dir_all(folder).ok();
    reopened
}

fn cells(app: &RSpiceApp, library: &str) -> Vec<String> {
    app.state
        .model
        .library_manager
        .get_library(library)
        .map(|library| {
            library
                .cells_sorted()
                .into_iter()
                .map(|cell| cell.name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// A cell copied with the cells it instantiates into another library makes
/// the whole hierarchy there, each copy placing the copies; the originals
/// still place each other. The reopened project reads the same, and one
/// Undo takes every copy back.
#[test]
fn a_cell_copied_with_its_hierarchy_places_only_the_copies() {
    let mut app = hierarchy();
    let (outcome, _) = app
        .state
        .model
        .copy_cell_hierarchy("parts", "amp", "work", "amp", BOTH);
    assert_eq!(outcome, Ok((3, 3)), "three cells of one view each");
    let reopened = saved_and_reopened(&mut app, "library-hierarchy-copy");
    for app in [&app, &reopened] {
        assert_eq!(cells(app, "work"), ["amp", "leaf", "stage"]);
        assert_eq!(placed(app, "work", "amp"), pair("work", "stage"));
        assert_eq!(placed(app, "work", "stage"), pair("work", "leaf"));
        assert_eq!(placed(app, "parts", "amp"), pair("parts", "stage"));
        assert_eq!(placed(app, "parts", "stage"), pair("parts", "leaf"));
    }
    assert!(app.command_key_enabled(ids::UNDO));
    app.execute_command_key(ids::UNDO);
    assert!(cells(&app, "work").is_empty(), "one Undo takes every copy");
    assert!(!app.state.model.can_undo_project_design());
}

/// Left pointing where they were, the copies place the cells they were
/// copied from.
#[test]
fn a_hierarchy_copied_without_repointing_places_the_originals() {
    let mut app = hierarchy();
    let hierarchy_only = HierarchyCopy {
        include_children: true,
        repoint: false,
    };
    let (outcome, _) =
        app.state
            .model
            .copy_cell_hierarchy("parts", "amp", "work", "amp_copy", hierarchy_only);
    assert_eq!(outcome.map(|(cells, _)| cells), Ok(3));
    assert_eq!(placed(&app, "work", "amp_copy"), pair("parts", "stage"));
    assert_eq!(placed(&app, "work", "stage"), pair("parts", "leaf"));
}

/// A cell the hierarchy reaches whose name the destination already holds
/// refuses the copy whole: nothing is copied.
#[test]
fn a_child_the_destination_already_names_refuses_the_whole_copy() {
    let mut app = hierarchy();
    let mut leaf = Cell::new("leaf");
    leaf.add_view(View::new("schematic", ViewType::Schematic));
    app.state
        .model
        .library_manager
        .edit_library("work")
        .expect("work is writable")
        .add_cell(leaf);
    let (outcome, _) = app
        .state
        .model
        .copy_cell_hierarchy("parts", "amp", "work", "amp", BOTH);
    let refusal = outcome.expect_err("the copy is refused");
    assert!(
        refusal.contains("parts/leaf, which the copied cell instantiates"),
        "{refusal}"
    );
    assert_eq!(cells(&app, "work"), ["leaf"], "nothing was copied");
}

/// A copied library is a writable library of the project whose cells place
/// each other, and the reopened project holds it.
#[test]
fn a_copied_library_places_its_own_cells() {
    let mut app = hierarchy();
    let (outcome, _) = app.state.model.copy_library("parts", "parts_v2");
    assert_eq!(outcome, Ok(3));
    let reopened = saved_and_reopened(&mut app, "library-copy");
    for app in [&app, &reopened] {
        let copy = app
            .state
            .model
            .library_manager
            .get_library("parts_v2")
            .expect("the copy is in the catalog");
        assert!(!copy.read_only);
        assert_eq!(cells(app, "parts_v2"), ["amp", "leaf", "stage"]);
        assert_eq!(placed(app, "parts_v2", "amp"), pair("parts_v2", "stage"));
        assert_eq!(placed(app, "parts_v2", "stage"), pair("parts_v2", "leaf"));
        assert_eq!(placed(app, "parts", "amp"), pair("parts", "stage"));
    }
}

/// Copy view through its form: a copy into a cell that does not exist
/// makes the cell, the drawing comes with it unchanged, and a second copy
/// under the same name is refused on the form.
#[test]
fn a_view_copied_into_a_new_cell_makes_the_cell() {
    let mut app = hierarchy();
    let source = schematic("parts", "stage");
    app.state
        .project
        .open_copy_view_dialog(&app.state.model, &source)
        .expect("Copy view opens");
    let form = &mut app.state.project.library_transfer;
    assert_eq!(form.kind, Some(TransferKind::CopyView));
    assert_eq!(form.target_library, "parts", "its own library is writable");
    form.target_library = "work".to_owned();
    form.target_cell = "buffer".to_owned();
    form.name = "schematic".to_owned();
    with_cx(&mut app, commit_library_transfer).expect("the view is copied");
    app.after_contribution();
    let reopened = saved_and_reopened(&mut app, "library-view-copy");
    for app in [&app, &reopened] {
        assert_eq!(cells(app, "work"), ["buffer"]);
        assert_eq!(placed(app, "work", "buffer"), pair("parts", "leaf"));
    }

    app.state
        .project
        .open_copy_view_dialog(&app.state.model, &source)
        .expect("Copy view opens again");
    let form = &mut app.state.project.library_transfer;
    form.target_library = "work".to_owned();
    form.target_cell = "buffer".to_owned();
    form.name = "schematic".to_owned();
    let refusal = with_cx(&mut app, commit_library_transfer).expect_err("the name is taken");
    assert!(refusal.contains("already"), "{refusal}");
}

/// A moved cell takes every placement, its open tab and, for the top cell,
/// the project top with it, and the reopened project reads the same.
#[test]
fn a_moved_cell_takes_its_placements_its_tab_and_the_top_with_it() {
    let mut app = hierarchy();
    app.state
        .project
        .open_move_cell_dialog(&app.state.model, "parts", "stage")
        .expect("Move cell opens");
    assert_ne!(
        app.state.project.library_transfer.target_library, "parts",
        "a cell moves to another library"
    );
    app.state.project.library_transfer.target_library = "work".to_owned();
    with_cx(&mut app, commit_library_transfer).expect("the cell is moved");
    app.after_contribution();
    assert_eq!(
        app.state
            .move_cell("user", "top", "work", "top")
            .expect("the top cell is moved"),
        0,
        "nothing places the top"
    );
    let reopened = saved_and_reopened(&mut app, "library-move");
    for app in [&app, &reopened] {
        assert_eq!(cells(app, "parts"), ["amp", "leaf"]);
        assert_eq!(cells(app, "work"), ["stage", "top"]);
        assert_eq!(placed(app, "parts", "amp"), pair("work", "stage"));
        assert_eq!(placed(app, "work", "stage"), pair("parts", "leaf"));
        let project = &app.state.model.workspace.content.project;
        assert_eq!(
            (project.root_library.as_str(), project.top_cell.as_str()),
            ("work", "top")
        );
    }
    let tabs: Vec<_> = app
        .state
        .model
        .workspace
        .content
        .open_views
        .iter()
        .map(|open| open.reference.display_path())
        .collect();
    assert_eq!(tabs, ["work/top/schematic", "work/stage/schematic"]);
}

/// Library properties and Cell properties apply through their form: a
/// description for each and the library made read-only, after which the
/// cell takes no edit. The reopened project reads all three.
#[test]
fn library_and_cell_properties_survive_save_and_reopen() {
    let mut app = hierarchy();
    let apply = |app: &mut RSpiceApp, cell: Option<&str>, edit: &dyn Fn(&mut RSpiceApp)| {
        app.state
            .project
            .open_library_properties_dialog(&app.state.model, "parts", cell)
            .expect("the properties open");
        edit(app);
        with_cx(app, commit_library_properties)
    };
    apply(&mut app, None, &|app| {
        app.state.project.library_properties.description = "Analog parts".to_owned();
    })
    .expect("the library is described");
    apply(&mut app, Some("amp"), &|app| {
        app.state.project.library_properties.description = "Two-stage amplifier".to_owned();
    })
    .expect("the cell is described");
    apply(&mut app, None, &|app| {
        app.state.project.library_properties.read_only = true;
    })
    .expect("the library is made read-only");
    let refusal = apply(&mut app, Some("amp"), &|app| {
        app.state.project.library_properties.description = "Changed".to_owned();
    })
    .expect_err("a read-only library's cell takes no edit");
    assert!(refusal.contains("read-only"), "{refusal}");

    let reopened = saved_and_reopened(&mut app, "library-properties");
    for app in [&app, &reopened] {
        let parts = app
            .state
            .model
            .library_manager
            .get_library("parts")
            .expect("parts is in the catalog");
        assert_eq!(parts.description(), "Analog parts");
        assert!(parts.read_only);
        assert_eq!(
            parts.get_cell("amp").map(|cell| cell.description.as_str()),
            Some("Two-stage amplifier")
        );
    }
    let mut reopened = reopened;
    reopened
        .state
        .model
        .edit_library_properties("parts", "Analog parts", false)
        .expect("a read-only library the project keeps is made writable again");
    assert!(
        !reopened
            .state
            .model
            .library_manager
            .get_library("parts")
            .unwrap()
            .read_only
    );
    for _ in 0..3 {
        app.execute_command_key(ids::UNDO);
    }
    let parts = app
        .state
        .model
        .library_manager
        .get_library("parts")
        .unwrap();
    assert!(!parts.read_only && parts.description().is_empty());
    assert_eq!(
        parts.get_cell("amp").map(|cell| cell.description.as_str()),
        Some("")
    );
}

/// A project's first save stamps every view; a later one stamps only the
/// views it writes with a change, and the reopened project reads the
/// stamps back.
#[test]
fn a_save_stamps_the_views_it_wrote_with_a_change() {
    let mut app = hierarchy();
    let saved_at = |app: &RSpiceApp, cell: &str| {
        app.state
            .model
            .library_manager
            .get_library("parts")
            .and_then(|library| library.get_cell(cell))
            .and_then(|cell| cell.get_view("schematic"))
            .and_then(|view| view.saved_at)
    };
    let folder = unique_temp_dir("library-saved-at");
    let path = folder.join("stamps.rspiceproj");
    assert!(app.state.save_project_to_path(&path));
    for cell in ["amp", "stage", "leaf"] {
        assert!(saved_at(&app, cell).is_some(), "{cell} is stamped");
    }
    // An old stamp on both, then a change to the stage alone.
    for cell in ["stage", "leaf"] {
        app.state
            .model
            .library_manager
            .record_view_saved_runtime("parts", cell, "schematic", 1);
    }
    let mut stage = app
        .state
        .model
        .workspace
        .clone_schematic_editor(&schematic("parts", "stage").key())
        .expect("the stage has a drawing");
    stage.add_component(ComponentType::Capacitor, Point::new(40, 40));
    app.state
        .model
        .workspace
        .insert_schematic_editor(schematic("parts", "stage").key(), stage);
    assert!(app.state.save_project_to_path(&path));
    let stamped = saved_at(&app, "stage").expect("the stage is stamped");
    assert!(stamped > 1, "the changed stage takes the save's time");
    assert_eq!(
        saved_at(&app, "leaf"),
        Some(1),
        "the unchanged leaf keeps its"
    );

    let mut reopened = RSpiceApp::test_instance();
    assert!(reopened.state.load_project_from_path(&path));
    assert_eq!(saved_at(&reopened, "stage"), Some(stamped));
    assert_eq!(saved_at(&reopened, "leaf"), Some(1));
    std::fs::remove_dir_all(folder).ok();
}

/// Open read-only opens the view in Design as a reference that refuses
/// edits; Open on it afterwards opens it to edit.
#[test]
fn open_read_only_opens_a_reference_that_refuses_edits() {
    let mut app = hierarchy();
    let stage = schematic("parts", "stage");
    let ctx = egui::Context::default();
    with_cx(&mut app, |cx| {
        apply_library_intent(&ctx, cx, LibraryIntent::OpenReadOnly(stage.clone()));
    });
    app.after_contribution();
    let content = &app.state.model.workspace.content;
    assert_eq!(content.active_view, stage);
    assert!(content.active_read_only_reference());
    assert!(app.state.model.active_view_read_only());
    assert_eq!(app.state.workbench.workspace, Workspace::Design);

    app.state
        .workbench
        .outbox
        .push(ProjectRequest::OpenView(stage.clone(), ViewType::Schematic));
    app.after_contribution();
    assert_eq!(app.state.model.workspace.content.active_view, stage);
    assert!(
        !app.state.model.active_view_read_only(),
        "Open from the project opens to edit"
    );
}
