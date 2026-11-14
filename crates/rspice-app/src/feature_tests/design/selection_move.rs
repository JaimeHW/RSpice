//! Moving a selection through the whole application: arming the move asks the application
//! whether a dialog holds the canvas.

use crate::state::Tool;
use crate::state::{ComponentType, Point, SchematicSnapshot};
use crate::workbench::app::arm_move_selection_in;
use crate::workbench::app::armed_move_selection_authority;
use crate::workbench::app_state::AppState;

fn selected_resistor() -> AppState {
    let mut state = AppState::default();
    let id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(id);
    state
}

/// The command acts at once: no modal stands between `M` and the canvas,
/// and the tool is live the same frame.
#[test]
fn arming_opens_no_dialog_and_hands_the_canvas_the_tool() {
    let mut state = selected_resistor();
    let baseline = SchematicSnapshot::capture(state.model.schematic.document());

    arm_move_selection_in(
        &mut state.design.move_selection,
        &mut state.design.stretch_selection,
        &mut state.design.array_selection,
        &mut state.model,
    );

    assert!(state.design.move_selection.armed);
    assert_eq!(
        state.model.schematic.session.editor.tool,
        Tool::MoveSelection
    );
    assert!(!state.dialog_modal_open());
    assert!(baseline.is_equal(&SchematicSnapshot::capture(
        state.model.schematic.document()
    )));
    assert!(armed_move_selection_authority(&state.design.move_selection, &state.model).is_ok());
}
