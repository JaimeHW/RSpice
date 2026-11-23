//! Project properties through whole application frames: File > Project
//! properties… shows the project inspector beside the Overview with its Name
//! taking the keys; the inspector changes the name, description and top
//! cell, and the Dependencies page the include search paths. What they apply
//! survives save and reopen, each change is one step of Undo, and runs
//! without a configuration start at the top it sets, as they do after Set as
//! project top in a cell's menu.

#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use super::super::replay::{Control, PRIMARY, Replay, only};
use super::recovery_frames::painted;
use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference::{
    self, AMP_CELL, SCHEMATIC_VIEW, TOP_CELL, USER_LIBRARY,
};
use rspice_app_project::workbench::project::library_dialogs::library_files::answer_next_folder_pick;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::state::CellViewRef;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// The command as the palette lists it, without its ellipsis.
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

fn project_name(replay: &Replay) -> String {
    replay
        .app
        .state
        .model
        .workspace
        .content
        .project
        .properties()
        .name
}

/// What the inspector's field `label` holds.
fn field(replay: &mut Replay, label: &str) -> String {
    only(&replay.frames(Vec::new()), label, Role::TextInput)
        .value
        .clone()
}

/// Type `text` over what the inspector's field `label` holds, and leave it
/// with Enter, which applies it.
fn retype(replay: &mut Replay, label: &str, text: &str) {
    replay.click(label, Role::TextInput);
    replay.key(Key::A, PRIMARY);
    replay.type_text(text);
    replay.key(Key::Enter, Modifiers::NONE);
}

/// The inspector's Top cell set to `cell` of the user library.
fn choose_top(replay: &mut Replay, cell: &str) {
    replay.click("Top cell", Role::ComboBox);
    click_named(replay, &format!("{USER_LIBRARY}/{cell}"));
}

/// File > Project properties…, from any page and with the inspector hidden,
/// shows the project inspector beside the Overview, with nothing selected
/// and the project's Name taking the keys: what is typed renames the
/// project.
#[test]
fn project_properties_lands_on_the_inspector_with_the_name_taking_the_keys() {
    let folder = unique_temp_dir("project-properties-command");
    let (mut replay, _) = saved_reference(&folder, ProjectPage::Dependencies);
    replay.app.state.workbench.inspector_visible = false;
    replay.frames(Vec::new());

    replay.choose_command(PALETTE);
    let state = &replay.app.state;
    assert_eq!(state.workbench.workspace, Workspace::Project);
    assert_eq!(state.project.project_page, ProjectPage::Overview);
    assert!(state.workbench.inspector_visible, "the inspector is shown");
    let shown = replay.frames(Vec::new());
    assert_eq!(
        only(&shown, "Name", Role::TextInput).value,
        project_name(&replay)
    );

    replay.key(Key::A, PRIMARY);
    replay.type_text("Bandgap reference");
    replay.key(Key::Enter, Modifiers::NONE);
    assert_eq!(project_name(&replay), "Bandgap reference");
    std::fs::remove_dir_all(folder).ok();
}

/// The name, description and top cell changed in the inspector, and an
/// include search path added on the Dependencies page, are what the project
/// opens with after it is saved and opened again.
#[test]
fn what_project_properties_applies_survives_save_and_reopen() {
    let folder = unique_temp_dir("project-properties-reopen");
    std::fs::create_dir_all(folder.join("models")).unwrap();
    let (mut replay, path) = saved_reference(&folder, ProjectPage::Overview);

    retype(&mut replay, "Name", "Bandgap reference");
    retype(&mut replay, "Description", "Bias for the converter tests");
    choose_top(&mut replay, AMP_CELL);
    replay.app.state.project.project_page = ProjectPage::Dependencies;
    answer_next_folder_pick(&folder.join("models"));
    replay.click("Add", Role::Button);
    click_named(&mut replay, "Include path\u{2026}");
    assert!(replay.app.state.save_project_to_path(&path));
    // The project is let go before it is opened again, so the second
    // session does not find it held and open it read-only.
    drop(replay);

    let mut reopened = RSpiceApp::test_instance();
    assert!(reopened.state.load_project_from_path(&path));
    reopened.state.workbench.activate(Workspace::Project);
    reopened.state.project.project_page = ProjectPage::Overview;
    let mut reopened = Replay::of(reopened);
    assert_eq!(field(&mut reopened, "Name"), "Bandgap reference");
    assert_eq!(
        field(&mut reopened, "Description"),
        "Bias for the converter tests"
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
/// inspector sets. Each change is one step of Undo, and Undo takes back the
/// top cell, the description and the name in turn.
#[test]
fn each_change_is_one_step_of_undo_and_runs_start_at_the_top_it_sets() {
    let folder = unique_temp_dir("project-properties-undo");
    let (mut replay, _) = saved_reference(&folder, ProjectPage::Overview);
    run_without_a_configuration(&mut replay);
    let name = project_name(&replay);
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));

    retype(&mut replay, "Name", "Renamed");
    retype(&mut replay, "Description", "Described");
    choose_top(&mut replay, AMP_CELL);
    assert_eq!(runs_start_at(&replay), schematic(AMP_CELL));

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));
    assert_eq!(field(&mut replay, "Description"), "Described");

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(field(&mut replay, "Description"), "");
    assert_eq!(project_name(&replay), "Renamed");

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(project_name(&replay), name);
    assert_eq!(runs_start_at(&replay), schematic(TOP_CELL));

    // Redo puts them forward again, in the order they were made.
    replay.app.execute_command_key(ids::REDO);
    assert_eq!(project_name(&replay), "Renamed");
    replay.app.execute_command_key(ids::REDO);
    replay.app.execute_command_key(ids::REDO);
    assert_eq!(runs_start_at(&replay), schematic(AMP_CELL));
    assert_eq!(field(&mut replay, "Description"), "Described");
    std::fs::remove_dir_all(folder).ok();
}

/// Set as project top in a cell's menu on the Library page moves where runs
/// without a configuration start, says so, and is one step of Undo.
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

    replay.frames(Vec::new());
    replay.key(egui::Key::F10, egui::Modifiers::SHIFT);
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
