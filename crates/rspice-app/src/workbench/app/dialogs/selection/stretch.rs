//! Stretch: an armed canvas tool, not a dialog.
//!
//! `S` arms the tool on the selected conductor segment or control point, as
//! Virtuoso's `S` does, and the canvas previews the candidate and commits it
//! as one undoable mutation. Arming mutates nothing.
//!
//! The one choice it keeps — whether the segments either side stay
//! orthogonal — is a check box on the tool's options strip. It starts from the
//! sheet's wire-routing style, so a sheet drawn at 45 degrees does not have to
//! turn it off on every stretch.

use crate::state::{StretchOrthogonalPolicy, Tool};

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app_state::AppState;
use rspice_schematic_editor::session::visibility::SchematicWireRoutingStyle;

const COMMAND: &str = "Stretch";

/// What the sheet's own routing style says about diagonals. A stretch that
/// defaulted to orthogonal on a 45-degree sheet would refuse the first drag
/// the reader tried.
fn policy_for_sheet(state: &AppState) -> StretchOrthogonalPolicy {
    match state.ui.schematic_visibility.wire_routing {
        SchematicWireRoutingStyle::Orthogonal => StretchOrthogonalPolicy::PreserveOrthogonal,
        SchematicWireRoutingStyle::FortyFiveDegree | SchematicWireRoutingStyle::FreeAngle => {
            StretchOrthogonalPolicy::AllowDiagonal
        }
    }
}

pub(crate) fn arm_stretch_selection(state: &mut AppState) {
    // Stretch is greyed on a read-only schematic and with nothing to stretch,
    // so those two return without a word.
    if state.schematic_edit_read_only() {
        return;
    }
    if !state.schematic.session.editor.selection.probes.is_empty() {
        state.prompt("A probe marker cannot be stretched. Move it instead.");
        return;
    }
    let Some(target) = state.schematic.default_stretch_target() else {
        return;
    };
    if state.dialogs.move_selection.armed {
        crate::workbench::app::cancel_armed_move_selection(state);
    }
    if state.dialogs.array_selection.armed {
        crate::workbench::app::cancel_armed_array_selection(state);
    }
    let policy = policy_for_sheet(state);
    state
        .dialogs
        .stretch_selection
        .arm(SchematicEditAuthority::capture(state), target, policy);
    state.schematic.arm_tool(Tool::StretchSelection);
}

fn validate_draft(state: &AppState) -> Result<(), String> {
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let draft = &state.dialogs.stretch_selection;
    let Some(authority) = draft.authority.as_ref() else {
        return Err("Stretch is not armed.".to_owned());
    };
    authority.validate(state, COMMAND)?;
    let Some(target) = draft.canvas.target else {
        return Err("Stretch is not armed.".to_owned());
    };
    if !state.schematic.is_stretch_target_eligible(target) {
        return Err(
            "What this stretch was armed on is no longer there. Select it again.".to_owned(),
        );
    }
    Ok(())
}

pub(crate) fn armed_stretch_selection_authority(state: &AppState) -> Result<(), String> {
    let draft = &state.dialogs.stretch_selection;
    if !draft.armed || state.schematic.session.editor.tool != Tool::StretchSelection {
        return Err("Stretch is not armed.".to_owned());
    }
    validate_draft(state)
}

pub(crate) fn cancel_armed_stretch_selection(state: &mut AppState) {
    state.dialogs.stretch_selection.close();
    if state.schematic.session.editor.tool == Tool::StretchSelection {
        state.schematic.cancel_tool();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Point, StretchTarget, Wire};

    fn selected_wire_segment() -> AppState {
        let mut state = AppState::default();
        let wire = Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]);
        state.schematic.document_mut_for_test().wires.push(wire);
        state.schematic.session.editor.selection.select_wire(7);
        state
    }

    #[test]
    fn arming_opens_no_dialog_and_resolves_the_target_at_once() {
        let mut state = selected_wire_segment();

        arm_stretch_selection(&mut state);

        let draft = &state.dialogs.stretch_selection;
        assert!(draft.armed);
        assert!(!state.dialogs.application_modal_open());
        assert_eq!(draft.policy, StretchOrthogonalPolicy::PreserveOrthogonal);
        assert_eq!(
            draft.canvas.target,
            Some(StretchTarget::WireSegment {
                wire_id: 7,
                segment_index: 0,
            })
        );
        assert!(draft.authority.is_some());
        assert_eq!(state.schematic.session.editor.tool, Tool::StretchSelection);
    }

    /// The check box starts where the sheet is drawn: a 45-degree sheet does
    /// not have to clear it on every stretch.
    #[test]
    fn the_orthogonal_rule_starts_from_the_sheets_routing_style() {
        let mut state = selected_wire_segment();
        state.ui.schematic_visibility.wire_routing = SchematicWireRoutingStyle::FortyFiveDegree;

        arm_stretch_selection(&mut state);

        assert_eq!(
            state.dialogs.stretch_selection.policy,
            StretchOrthogonalPolicy::AllowDiagonal
        );
    }

    #[test]
    fn nothing_stretchable_refuses_instead_of_arming() {
        let mut state = AppState::default();
        arm_stretch_selection(&mut state);
        assert!(!state.dialogs.stretch_selection.armed);
        assert_ne!(state.schematic.session.editor.tool, Tool::StretchSelection);
    }
}
