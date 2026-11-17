//! Project properties through whole application frames: opened from the
//! project's name, the palette or the inspector, it changes the name,
//! description, top cell and include search paths. What it applies survives
//! save and reopen, each Apply is one step of Undo, and runs without a
//! configuration start at the top it sets, as they do after Set as project
//! top in Cell actions.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use egui::accesskit::Role;

use super::super::replay::{Control, Replay, only};
use super::recovery_frames::painted;
use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference::{
    self, AMP_CELL, SCHEMATIC_VIEW, TOP_CELL, USER_LIBRARY,
};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// The inspector's action and the File menu's row; the palette lists the
/// command without its ellipsis.
const PROPERTIES: &str = "Project properties\u{2026}";
const PALETTE: &str = "Project properties";

/// The reference project saved at `folder`/reference.rspiceproj and shown
/// on Project > `page`.
fn saved_reference(folder: &Path, page: ProjectPage) -> (Replay, PathBuf) {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    let path = folder.join("reference.rspiceproj");
    assert!(app.state.save_project_to_path(&path));
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = page;
    (Replay::of(app), path)
}

/// Runs use the built-in default: no configuration is used for runs, so
/// they start at the project's top cell.
fn run_without_a_configuration(replay: &mut Replay) {
    let content = &mut replay.app.state.model.workspace.content;
    let mut catalog = (*content.configuration_sets).clone();
    catalog.deactivate().unwrap();
    content.replace_configuration_sets(catalog).unwrap();
}

/// The schematic runs start at.
fn runs_start_at(replay: &Replay) -> CellViewRef {
    replay
        .app
        .state
        .model
        .workspace
        .content
        .simulation_root_reference()
}

fn schematic(cell: &str) -> CellViewRef {
    CellViewRef::new(USER_LIBRARY, cell, SCHEMATIC_VIEW)
}

/// The name the Overview header shows, read from its link.
fn header_name(controls: &[Control]) -> String {
    controls
        .iter()
        .find(|control| control.role == Role::Link && control.label.starts_with("Project "))
        .map(|control| control.label["Project ".len()..].to_owned())
        .expect("the header names the project")
}

/// The project's description as Project properties shows it; the dialog is
/// closed again with Cancel.
fn shown_description(replay: &mut Replay) -> String {
    let opened = replay.choose_command(PALETTE);
    let description = only(&opened, "Description", Role::MultilineTextInput)
        .value
        .clone();
    replay.click("Cancel", Role::Button);
    description
}

/// Whether Project properties is open: its Name field is on screen.
fn is_open(controls: &[Control]) -> bool {
    controls
        .iter()
        .any(|control| control.role == Role::TextInput && control.label == "Name")
}

/// The name, description, top cell and an include search path, changed in
/// one Apply from the dialog the project's name opens, are what the project
/// opens with after it is saved and opened again.
#[test]
fn what_project_properties_applies_survives_save_and_reopen() {
    let folder = unique_temp_dir("project-properties-reopen");
    std::fs::create_dir_all(folder.join("models")).unwrap();
    let (mut replay, path) = saved_reference(&folder, ProjectPage::Overview);
    let name = header_name(&replay.frames(Vec::new()));

    let opened = replay.click(&format!("Project {name}"), Role::Link);
    assert_eq!(only(&opened, "Name", Role::TextInput).value, name);
    assert_eq!(
        only(&opened, "Description", Role::MultilineTextInput).value,
        ""
    );
    let shown = painted(&replay.frame(Vec::new()));
    for fact in [
        "Project properties",
        "user/top",
        "reference.rspiceproj",
        "Runs use configuration Reference hierarchy, which starts at user/top/schematic.",
    ] {
        assert!(shown.contains(fact), "the dialog shows {fact:?}:\n{shown}");
    }

    // The name has the caret with its text selected: typing replaces it.
    replay.type_text("Bandgap reference");
    replay.click("Description", Role::MultilineTextInput);
    replay.type_text("Bias for the converter tests");
    replay.click("Top cell", Role::ComboBox);
    replay.click("user/amp", Role::ListBoxOption);
    replay.click("Include search paths", Role::TextInput);
    replay.type_text("models");
    let added = replay.click("Add include search path", Role::Button);
    assert!(
        added
            .iter()
            .any(|control| control.role == Role::Button && control.label == "Remove models"),
        "the path is listed, with its remove button, once added"
    );
    let applied = replay.click("Apply", Role::Button);
    assert!(!is_open(&applied), "Apply closes the dialog");
    assert_eq!(header_name(&applied), "Bandgap reference");
    assert!(replay.app.state.save_project_to_path(&path));

    let mut reopened = RSpiceApp::test_instance();
    assert!(reopened.state.load_project_from_path(&path));
    reopened.state.workbench.activate(Workspace::Project);
    reopened.state.project.project_page = ProjectPage::Overview;
    let mut reopened = Replay::of(reopened);
    assert_eq!(
        header_name(&reopened.frames(Vec::new())),
        "Bandgap reference"
    );
    let opened = reopened.choose_command(PALETTE);
    assert_eq!(
        only(&opened, "Name", Role::TextInput).value,
        "Bandgap reference"
    );
    assert_eq!(
        only(&opened, "Description", Role::MultilineTextInput).value,
        "Bias for the converter tests"
    );
    assert!(
        opened
            .iter()
            .any(|control| control.role == Role::Button && control.label == "Remove models")
    );
    let shown = painted(&reopened.frame(Vec::new()));
    assert!(shown.contains("user/amp"), "{shown}");
    let project = &reopened.app.state.model.workspace.content.project;
    assert_eq!(
        (project.root_library.as_str(), project.top_cell.as_str()),
        (USER_LIBRARY, AMP_CELL)
    );
    assert_eq!(project.include_search_paths(), [PathBuf::from("models")]);
    std::fs::remove_dir_all(folder).ok();
}

/// With no configuration used for runs, runs start at the top cell the
/// dialog sets. Each Apply is one step of Undo, and Undo takes back the
/// top cell, the description and the name in turn.
#[test]
fn each_apply_is_one_step_of_undo_and_runs_start_at_the_top_it_sets() {
    let folder = unique_temp_dir("project-properties-undo");
    let (mut replay, _) = saved_reference(&folder, ProjectPage::Overview);
    run_without_a_configuration(&mut replay);
    let name = header_name(&replay.frames(Vec::new()));
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));

    replay.choose_command(PALETTE);
    let shown = painted(&replay.frame(Vec::new()));
    assert!(
        shown.contains("Runs start here: no configuration is used for runs."),
        "{shown}"
    );
    replay.type_text("Renamed");
    replay.click("Apply", Role::Button);

    // The inspector's Project actions open it too.
    replay.click(PROPERTIES, Role::Button);
    replay.click("Description", Role::MultilineTextInput);
    replay.type_text("Described");
    replay.click("Apply", Role::Button);

    replay.choose_command(PALETTE);
    replay.click("Top cell", Role::ComboBox);
    replay.click("user/amp", Role::ListBoxOption);
    let applied = replay.click("Apply", Role::Button);
    assert!(!is_open(&applied));
    assert_eq!(runs_start_at(&replay), schematic(AMP_CELL));

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));
    assert_eq!(shown_description(&mut replay), "Described");

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(shown_description(&mut replay), "");
    assert_eq!(header_name(&replay.frames(Vec::new())), "Renamed");

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(header_name(&replay.frames(Vec::new())), name);
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));

    // Redo puts them forward again, in the order they were made.
    replay.app.execute_command_key(ids::REDO);
    assert_eq!(header_name(&replay.frames(Vec::new())), "Renamed");
    replay.app.execute_command_key(ids::REDO);
    replay.app.execute_command_key(ids::REDO);
    assert_eq!(runs_start_at(&replay), schematic(AMP_CELL));
    assert_eq!(shown_description(&mut replay), "Described");
    std::fs::remove_dir_all(folder).ok();
}

/// Set as project top in Library > Cell actions moves where runs without a
/// configuration start, says so, and is one step of Undo.
#[test]
fn set_as_project_top_moves_where_runs_start() {
    let folder = unique_temp_dir("project-properties-set-top");
    let (mut replay, _) = saved_reference(&folder, ProjectPage::Library);
    run_without_a_configuration(&mut replay);
    replay
        .app
        .state
        .model
        .library_manager
        .select_view(USER_LIBRARY, AMP_CELL, SCHEMATIC_VIEW);

    click_named(&mut replay, "Cell actions");
    click_named(&mut replay, "Set as project top");
    assert_eq!(runs_start_at(&replay), schematic(AMP_CELL));
    let shown = painted(&replay.frame(Vec::new()));
    assert!(
        shown.contains("user/amp is the project top; runs without a configuration start there."),
        "{shown}"
    );

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));
    std::fs::remove_dir_all(folder).ok();
}

/// The control named `label`, whatever its role, pressed and released at
/// its centre.
fn click_named(replay: &mut Replay, label: &str) -> Vec<Control> {
    let controls = replay.frames(Vec::new());
    let at = controls
        .iter()
        .find(|control| control.label == label)
        .unwrap_or_else(|| panic!("no control named {label:?}"))
        .center;
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), press(true)]);
    replay.frames(vec![press(false)])
}
