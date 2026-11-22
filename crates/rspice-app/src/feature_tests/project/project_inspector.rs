//! The project inspector through whole application frames: with nothing on
//! the page selected, the project's own name stands in the inspector and is
//! edited there, applied as the project's properties are anywhere.

#![cfg(not(target_arch = "wasm32"))]

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use super::super::replay::{PRIMARY, Replay, only};
use super::recovery_frames::painted;
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::properties::PROJECT_READ_ONLY;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// The Project workspace on `page`, a project open.
fn on_page(page: ProjectPage) -> Replay {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = page;
    Replay::of(app)
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

/// A name typed into the inspector's Name renames the project when the
/// field is left with Enter, and not before; Undo takes it back in one step.
#[test]
fn a_name_typed_in_the_inspector_renames_the_project_as_one_step_of_undo() {
    let mut replay = on_page(ProjectPage::Overview);
    let before = project_name(&replay);
    let shown = replay.frames(Vec::new());
    assert_eq!(only(&shown, "Name", Role::TextInput).value, before);

    replay.click("Name", Role::TextInput);
    replay.key(Key::A, PRIMARY);
    replay.type_text("Bandgap reference");
    assert_eq!(
        project_name(&replay),
        before,
        "nothing is applied while the field is edited"
    );
    let left = replay.key(Key::Enter, Modifiers::NONE);
    assert_eq!(project_name(&replay), "Bandgap reference");
    assert_eq!(
        only(&left, "Name", Role::TextInput).value,
        "Bandgap reference"
    );

    replay.app.execute_command_key(ids::UNDO);
    assert_eq!(project_name(&replay), before);
    let undone = replay.frames(Vec::new());
    assert_eq!(only(&undone, "Name", Role::TextInput).value, before);
}

/// A project open read-only shows its name as a fact that takes no typing,
/// and says why on hover.
#[test]
fn a_read_only_project_refuses_an_edit_in_the_inspector() {
    let mut replay = on_page(ProjectPage::Overview);
    let model = &mut replay.app.state.model;
    model.safe_mode.active = true;
    model.safe_mode.applied.open_project_read_only = true;
    let name = project_name(&replay);
    let shown = replay.frames(Vec::new());
    for label in ["Name", "Description"] {
        assert!(
            !shown
                .iter()
                .any(|control| control.role == Role::TextInput && control.label == label),
            "{label} takes no typing in a read-only project"
        );
    }
    let at = shown
        .iter()
        .find(|control| control.label == "Name" && control.value == name)
        .expect("the inspector states the project's name")
        .center;
    let mut hovered = String::new();
    for frame in 0..20 {
        let events = if frame == 0 {
            vec![egui::Event::PointerMoved(at)]
        } else {
            Vec::new()
        };
        hovered = painted(&replay.frame(events));
        if hovered.contains(PROJECT_READ_ONLY) {
            break;
        }
    }
    assert!(hovered.contains(PROJECT_READ_ONLY), "{hovered}");
    assert_eq!(project_name(&replay), name);
}
