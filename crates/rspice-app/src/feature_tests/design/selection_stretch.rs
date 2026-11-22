//! Stretching a selection through the whole application: arming the stretch asks the
//! application whether a dialog holds the canvas.

use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::dialogs::selection::stretch::arm_stretch_selection_in;
use rspice_app_state::state::StretchOrthogonalPolicy;
use rspice_app_state::state::Tool;
use rspice_app_state::state::{Point, StretchTarget, Wire};

fn selected_wire_segment() -> AppState {
    let mut state = AppState::default();
    let wire = Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]);
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(wire);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire(7);
    state
}

#[test]
fn arming_opens_no_dialog_and_resolves_the_target_at_once() {
    let mut state = selected_wire_segment();

    arm_stretch_selection_in(
        &mut state.design.stretch_selection,
        &mut state.design.move_selection,
        &mut state.design.array_selection,
        state.design.schematic_visibility.wire_routing,
        &mut state.model,
        &mut state.workbench,
    );

    let draft = &state.design.stretch_selection;
    assert!(draft.armed);
    assert!(!state.dialog_modal_open());
    assert_eq!(draft.policy, StretchOrthogonalPolicy::PreserveOrthogonal);
    assert_eq!(
        draft.canvas.target,
        Some(StretchTarget::WireSegment {
            wire_id: 7,
            segment_index: 0,
        })
    );
    assert!(draft.authority.is_some());
    assert_eq!(
        state.model.schematic.session.editor.tool,
        Tool::StretchSelection
    );
}

/// A part with its two wires is a block: no one handle resolves, so Stretch
/// is greyed and its reason names the one handle it moves and points to
/// Move's Connected move, which carries a block with its wires. One wire
/// alone is a handle, and Stretch comes back.
#[test]
fn stretch_on_a_block_says_why_and_points_to_connected_move() {
    use rspice_app_design::workbench::design::commands::DesignCommand;
    use rspice_app_state::state::ComponentType;
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

    let mut app = crate::workbench::app::RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let schematic = &mut app.state.model.schematic;
    let part = schematic.add_component(ComponentType::Resistor, Point::origin());
    let wires = &mut schematic.document_mut_for_test().wires;
    wires.push(Wire::new(7, vec![Point::new(20, 0), Point::new(80, 0)]));
    wires.push(Wire::new(8, vec![Point::new(-20, 0), Point::new(-80, 0)]));
    let selection = &mut schematic.session.editor.selection;
    selection.select_only_component(part);
    selection.select_wire(7);
    selection.select_wire(8);

    assert_eq!(
        app.command_availability(DesignCommand::StretchSelection.key()),
        CommandAvailability::Disabled(
            "select one wire segment or shape corner; Move (Connected move) carries a block with its wires"
        )
    );
    assert_eq!(
        app.command_availability(DesignCommand::MoveSelection.key()),
        CommandAvailability::Available,
        "the Move the reason points to takes the same block"
    );
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_wire(7);
    assert_eq!(
        app.command_availability(DesignCommand::StretchSelection.key()),
        CommandAvailability::Available
    );
}
