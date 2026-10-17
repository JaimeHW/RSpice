//! Move: an armed canvas tool, not a dialog.
//!
//! `M` arms the tool at once, as Virtuoso's `M` does, and the canvas takes the
//! anchor and the destination. Arming mutates nothing: the interaction layer
//! accumulates a snapped delta and commits the configured movement exactly
//! once.
//!
//! The one choice the command still has — what happens to the wires attached
//! to what moves — is on the tool's options strip over the canvas, where it
//! can be changed with the command already running. A modal in front of a
//! drag is a modal the reader dismisses without reading.

use crate::state::Tool;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app_state::AppState;

const COMMAND: &str = "Move";

/// Arm Move against an immutable document and selection baseline. Commands
/// call this single boundary so palette, menu and shortcut activation cannot
/// drift.
pub(crate) fn arm_move_selection(state: &mut AppState) {
    // Move is greyed on a read-only schematic and with nothing movable
    // selected, so neither is answered.
    if state.schematic_edit_read_only() || !state.schematic.has_live_movable_selection() {
        return;
    }
    if state.dialogs.stretch_selection.armed {
        crate::workbench::app::cancel_armed_stretch_selection(state);
    }
    if state.dialogs.array_selection.armed {
        crate::workbench::app::cancel_armed_array_selection(state);
    }
    state
        .dialogs
        .move_selection
        .arm(SchematicEditAuthority::capture(state));
    state.schematic.arm_tool(Tool::MoveSelection);
}

fn validate_draft(state: &AppState) -> Result<(), String> {
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let draft = &state.dialogs.move_selection;
    let Some(authority) = draft.authority.as_ref() else {
        return Err("Move is not armed.".to_owned());
    };
    authority.validate(state, COMMAND)?;
    if !state.schematic.has_live_movable_selection() {
        return Err("No movable selected object remains.".to_owned());
    }
    Ok(())
}

/// Fail-closed authority check used again at the canvas commit boundary.
pub(crate) fn armed_move_selection_authority(state: &AppState) -> Result<(), String> {
    let draft = &state.dialogs.move_selection;
    if !draft.armed || state.schematic.session.editor.tool != Tool::MoveSelection {
        return Err("Move is not armed.".to_owned());
    }
    validate_draft(state)
}

pub(crate) fn cancel_armed_move_selection(state: &mut AppState) {
    state.dialogs.move_selection.close();
    if state.schematic.session.editor.tool == Tool::MoveSelection {
        state.schematic.cancel_tool();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point, SchematicSnapshot};

    fn selected_resistor() -> AppState {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state
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
        let baseline = SchematicSnapshot::capture(state.schematic.document());

        arm_move_selection(&mut state);

        assert!(state.dialogs.move_selection.armed);
        assert_eq!(state.schematic.session.editor.tool, Tool::MoveSelection);
        assert!(!state.dialogs.application_modal_open());
        assert!(baseline.is_equal(&SchematicSnapshot::capture(state.schematic.document())));
        assert!(armed_move_selection_authority(&state).is_ok());
    }

    #[test]
    fn arming_freezes_the_selection_and_the_whole_design_snapshot() {
        let mut state = selected_resistor();
        arm_move_selection(&mut state);
        let authority = state
            .dialogs
            .move_selection
            .authority
            .as_ref()
            .expect("captured authority");
        assert_eq!(
            authority.selection,
            state.schematic.session.editor.selection
        );
        assert!(
            authority
                .snapshot
                .is_equal(&SchematicSnapshot::capture(state.schematic.document()))
        );
    }

    #[test]
    fn validation_fails_closed_when_non_electrical_geometry_changes() {
        let mut state = selected_resistor();
        arm_move_selection(&mut state);
        state.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
        assert!(validate_draft(&state).is_err());
    }

    /// Nothing to move is a refusal, not an armed tool waiting for a drag.
    #[test]
    fn an_empty_selection_refuses_instead_of_arming() {
        let mut state = AppState::default();
        arm_move_selection(&mut state);
        assert!(!state.dialogs.move_selection.armed);
        assert_ne!(state.schematic.session.editor.tool, Tool::MoveSelection);
    }
}
