//! Create array through the whole application: when it is offered, what it
//! names the copies, and what it reports.

use crate::feature_tests::replay::{PRIMARY, Replay};
use crate::workbench::app::RSpiceApp;
use egui::accesskit::Role;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{ComponentType, Point, SchematicArrayKind, SchematicProbe};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

/// A selection holding a probe greys Create array and says why, rather than
/// opening a form whose first canvas click is refused for that reason.
#[test]
fn create_array_is_greyed_with_its_reason_while_a_probe_is_selected() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let schematic = &mut app.state.model.schematic;
    let part = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(900, Point::new(0, 40), "P1", None).expect("a probe is well formed"),
    );
    let selection = &mut schematic.session.editor.selection;
    selection.select_only_component(part);
    selection.select_probe(900);

    assert_eq!(
        app.command_availability(DesignCommand::ArraySelection.key()),
        CommandAvailability::Disabled(
            "probes are not copied into an array; leave them out of the selection"
        )
    );

    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(part);
    assert_eq!(
        app.command_availability(DesignCommand::ArraySelection.key()),
        CommandAvailability::Available
    );
}

/// A replay with one resistor selected and Create array open over it.
fn create_array_open() -> Replay {
    let mut replay = Replay::of(RSpiceApp::test_instance());
    replay.app.state.workbench.activate(Workspace::Design);
    let schematic = &mut replay.app.state.model.schematic;
    let part = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic
        .session
        .editor
        .selection
        .select_only_component(part);
    replay
        .app
        .execute_command_key(DesignCommand::ArraySelection.key());
    replay
}

/// The count alone says whether the copies make a row, a column or a grid:
/// the form has no second control that could disagree with it, and a count
/// of 4 x 2 arms a rectangular array.
#[test]
fn the_count_alone_sets_a_row_or_a_grid() {
    let mut replay = create_array_open();
    let controls = replay.frames(Vec::new());
    assert!(
        !controls
            .iter()
            .any(|control| control.label == "Array" && control.role == Role::ComboBox),
        "a part's array offers no arrangement to choose"
    );
    assert_eq!(
        replay.app.state.design.array_selection.kind,
        SchematicArrayKind::Linear
    );
    replay.click("Count", Role::TextInput);
    replay.key(egui::Key::A, PRIMARY);
    replay.type_text("4x2");
    replay.click("Create", Role::Button);

    let draft = &replay.app.state.design.array_selection;
    assert!(draft.armed, "a 4 x 2 count arms the canvas");
    assert_eq!(draft.kind, SchematicArrayKind::Rectangular);
}
