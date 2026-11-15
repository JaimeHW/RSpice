//! Placing a stimulus drawn through a whole application.

use crate::workbench::RSpiceApp;
use crate::workbench::app::arm_placement_from_definition;
use rspice_app_state::state::Tool;
use rspice_app_state::state::stimulus_library::definition::StimulusDefinition;
use rspice_app_state::state::stimulus_library::provenance::ProvenanceState;
use rspice_app_state::state::{ComponentType, Point};

fn app_with_definition() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut definition = StimulusDefinition::new(
        "sensor_drive",
        ComponentType::VoltageSourceSin,
        rspice_app_state::state::stimulus_library::now_unix_ms,
    )
    .expect("definition");
    definition.value = "0".to_owned();
    definition.params = "va=3m freq=1k".to_owned();
    app.state
        .model
        .workspace
        .content
        .stimulus_library
        .insert(definition)
        .expect("insert");
    app
}

#[test]
fn arming_carries_the_saved_revision_onto_the_next_placement() {
    let mut app = app_with_definition();
    arm_placement_from_definition(
        &mut app.state.model,
        &mut app.state.workbench,
        "sensor_drive",
    )
    .expect("armed");

    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::VoltageSourceSin)
    );
    let armed = app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_stimulus
        .clone()
        .expect("a definition is armed");
    assert_eq!(armed.definition(), "sensor_drive");
    assert_eq!(armed.revision(), 1);

    let id = app
        .state
        .model
        .schematic
        .add_stimulus_component(&armed, Point::new(6, 6));
    let component = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("placed");
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .provenance_state(component),
        ProvenanceState::Adopted { revision: 1 }
    );
}

#[test]
fn arming_refuses_a_definition_the_library_does_not_hold() {
    let mut app = app_with_definition();
    let refusal =
        arm_placement_from_definition(&mut app.state.model, &mut app.state.workbench, "absent")
            .expect_err("nothing to arm");
    assert!(refusal.contains("defines no 'absent'"), "{refusal}");
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_stimulus
            .is_none()
    );
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
}

#[test]
fn arming_refuses_a_read_only_view() {
    let mut app = app_with_definition();
    app.state.model.schematic.session.read_only = true;
    let refusal = arm_placement_from_definition(
        &mut app.state.model,
        &mut app.state.workbench,
        "sensor_drive",
    )
    .expect_err("a read-only sheet takes no placement");
    assert!(refusal.contains("read-only"), "{refusal}");
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_stimulus
            .is_none()
    );
}
