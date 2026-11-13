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
use crate::workbench::app::dialogs::selection::array::{
    ArraySelectionDialogState, cancel_armed_array_selection_in,
};
use crate::workbench::app::dialogs::selection::stretch::{
    StretchSelectionDialogState, cancel_armed_stretch_selection_in,
};
#[cfg(test)]
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::Model;

/// Authority and interaction state of the armed Move tool. The design
/// snapshot remains immutable while the tool is armed; pointer and keyboard
/// movement is accumulated as a preview delta and committed only once.
///
/// `mode` outlives one command on purpose: it is a tool option on the canvas
/// strip, and a tool option a reader set two commands ago is still the one
/// they meant.
#[derive(Debug, Clone, Default)]
pub(crate) struct MoveSelectionDialogState {
    pub(crate) armed: bool,
    pub(crate) mode: crate::state::MoveSelectionMode,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) canvas: rspice_schematic_editor::session::transform::TransformCanvasSession,
    pub(crate) generation: u64,
}

impl MoveSelectionDialogState {
    pub(crate) fn arm(&mut self, authority: SchematicEditAuthority) {
        *self = Self {
            armed: true,
            mode: self.mode,
            authority: Some(authority),
            canvas: Default::default(),
            generation: self.generation.wrapping_add(1),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self {
            generation: self.generation.wrapping_add(1),
            mode: self.mode,
            ..Self::default()
        };
    }
}

const COMMAND: &str = "Move";

/// [`arm_move_selection_in`] over the whole state, the shape the canvas
/// tests arm through.
#[cfg(test)]
pub(crate) fn arm_move_selection(state: &mut AppState) {
    let design = &mut state.design;
    arm_move_selection_in(
        &mut design.move_selection,
        &mut design.stretch_selection,
        &mut design.array_selection,
        &mut state.model,
    );
}

/// Arm Move against an immutable document and selection baseline, over the
/// three canvas tools' drafts and the model. The design feature's command
/// calls this single boundary so palette, menu and shortcut activation
/// cannot drift; arming Move stops an armed Stretch or Create array.
pub(crate) fn arm_move_selection_in(
    draft: &mut MoveSelectionDialogState,
    stretch: &mut StretchSelectionDialogState,
    array: &mut ArraySelectionDialogState,
    model: &mut Model,
) {
    // Move is greyed on a read-only schematic and with nothing movable
    // selected, so neither is answered.
    if model.schematic_edit_read_only() || !model.schematic.has_live_movable_selection() {
        return;
    }
    if stretch.armed {
        cancel_armed_stretch_selection_in(stretch, model);
    }
    if array.armed {
        cancel_armed_array_selection_in(array, model);
    }
    draft.arm(SchematicEditAuthority::capture(&*model));
    model.schematic.arm_tool(Tool::MoveSelection);
}

fn validate_draft(draft: &MoveSelectionDialogState, model: &Model) -> Result<(), String> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let Some(authority) = draft.authority.as_ref() else {
        return Err("Move is not armed.".to_owned());
    };
    authority.validate(model, COMMAND)?;
    if !model.schematic.has_live_movable_selection() {
        return Err("No movable selected object remains.".to_owned());
    }
    Ok(())
}

/// Fail-closed authority check used again at the canvas commit boundary.
pub(crate) fn armed_move_selection_authority(
    draft: &MoveSelectionDialogState,
    model: &Model,
) -> Result<(), String> {
    if !draft.armed || model.schematic.session.editor.tool != Tool::MoveSelection {
        return Err("Move is not armed.".to_owned());
    }
    validate_draft(draft, model)
}

/// [`cancel_armed_move_selection_in`] for the session.
#[cfg(test)]
pub(crate) fn cancel_armed_move_selection(state: &mut AppState) {
    cancel_armed_move_selection_in(&mut state.design.move_selection, &mut state.model);
}

/// End an armed Move: close its dialog, and let its tool go if the
/// canvas still holds it.
pub(crate) fn cancel_armed_move_selection_in(
    dialog: &mut MoveSelectionDialogState,
    model: &mut Model,
) {
    dialog.close();
    if model.schematic.session.editor.tool == Tool::MoveSelection {
        model.schematic.cancel_tool();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point, SchematicSnapshot};

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

        arm_move_selection(&mut state);

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

    #[test]
    fn arming_freezes_the_selection_and_the_whole_design_snapshot() {
        let mut state = selected_resistor();
        arm_move_selection(&mut state);
        let authority = state
            .design
            .move_selection
            .authority
            .as_ref()
            .expect("captured authority");
        assert_eq!(
            authority.selection,
            state.model.schematic.session.editor.selection
        );
        assert!(authority.snapshot.is_equal(&SchematicSnapshot::capture(
            state.model.schematic.document()
        )));
    }

    #[test]
    fn validation_fails_closed_when_non_electrical_geometry_changes() {
        let mut state = selected_resistor();
        arm_move_selection(&mut state);
        state.model.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
        assert!(validate_draft(&state.design.move_selection, &state.model).is_err());
    }

    /// Nothing to move is a refusal, not an armed tool waiting for a drag.
    #[test]
    fn an_empty_selection_refuses_instead_of_arming() {
        let mut state = AppState::default();
        arm_move_selection(&mut state);
        assert!(!state.design.move_selection.armed);
        assert_ne!(
            state.model.schematic.session.editor.tool,
            Tool::MoveSelection
        );
    }
}
