//! Stretching a selection through the whole application: arming the stretch asks the
//! application whether a dialog holds the canvas.

use crate::workbench::app::arm_stretch_selection_in;
use crate::workbench::app_state::AppState;
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
