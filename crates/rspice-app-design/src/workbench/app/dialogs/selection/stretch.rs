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
use crate::workbench::app::dialogs::selection::array::{
    ArraySelectionDialogState, cancel_armed_array_selection_in,
};
use crate::workbench::app::dialogs::selection::move_selection::{
    MoveSelectionDialogState, cancel_armed_move_selection_in,
};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;
use rspice_schematic_editor::session::visibility::SchematicWireRoutingStyle;

/// Authority and interaction state of the armed Stretch tool. The selected
/// conductor segment or documentation control point is resolved before
/// mutation and remains stable through the live canvas preview and the
/// atomic commit.
#[derive(Debug, Clone, Default)]
pub struct StretchSelectionDialogState {
    pub armed: bool,
    pub policy: crate::state::StretchOrthogonalPolicy,
    pub authority: Option<SchematicEditAuthority>,
    pub canvas: rspice_schematic_editor::session::stretch::StretchCanvasSession,
    pub generation: u64,
}

impl StretchSelectionDialogState {
    pub(crate) fn arm(
        &mut self,
        authority: SchematicEditAuthority,
        target: crate::state::StretchTarget,
        policy: crate::state::StretchOrthogonalPolicy,
    ) {
        *self = Self {
            armed: true,
            policy,
            authority: Some(authority),
            canvas: rspice_schematic_editor::session::stretch::StretchCanvasSession {
                target: Some(target),
                gesture: Default::default(),
            },
            generation: self.generation.wrapping_add(1),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self {
            generation: self.generation.wrapping_add(1),
            ..Self::default()
        };
    }
}

const COMMAND: &str = "Stretch";

/// What the sheet's own routing style says about diagonals. A stretch that
/// defaulted to orthogonal on a 45-degree sheet would refuse the first drag
/// the reader tried.
fn policy_for_sheet(wire_routing: SchematicWireRoutingStyle) -> StretchOrthogonalPolicy {
    match wire_routing {
        SchematicWireRoutingStyle::Orthogonal => StretchOrthogonalPolicy::PreserveOrthogonal,
        SchematicWireRoutingStyle::FortyFiveDegree | SchematicWireRoutingStyle::FreeAngle => {
            StretchOrthogonalPolicy::AllowDiagonal
        }
    }
}

/// Arm Stretch over the three canvas tools' drafts, the sheet's wire
/// routing, the model and the session: arming Stretch stops an armed Move
/// or Create array.
pub fn arm_stretch_selection_in(
    draft: &mut StretchSelectionDialogState,
    move_draft: &mut MoveSelectionDialogState,
    array: &mut ArraySelectionDialogState,
    wire_routing: SchematicWireRoutingStyle,
    model: &mut Model,
    wb: &mut WorkbenchSession,
) {
    // Stretch is greyed on a read-only schematic and with nothing to stretch,
    // so those two return without a word.
    if model.schematic_edit_read_only() {
        return;
    }
    if !model.schematic.session.editor.selection.probes.is_empty() {
        wb.prompt("A probe marker cannot be stretched. Move it instead.");
        return;
    }
    let Some(target) = model.schematic.default_stretch_target() else {
        return;
    };
    if move_draft.armed {
        cancel_armed_move_selection_in(move_draft, model);
    }
    if array.armed {
        cancel_armed_array_selection_in(array, model);
    }
    let policy = policy_for_sheet(wire_routing);
    draft.arm(SchematicEditAuthority::capture(&*model), target, policy);
    model.schematic.arm_tool(Tool::StretchSelection);
}

fn validate_draft(draft: &StretchSelectionDialogState, model: &Model) -> Result<(), String> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    let Some(authority) = draft.authority.as_ref() else {
        return Err("Stretch is not armed.".to_owned());
    };
    authority.validate(model, COMMAND)?;
    let Some(target) = draft.canvas.target else {
        return Err("Stretch is not armed.".to_owned());
    };
    if !model.schematic.is_stretch_target_eligible(target) {
        return Err(
            "What this stretch was armed on is no longer there. Select it again.".to_owned(),
        );
    }
    Ok(())
}

pub fn armed_stretch_selection_authority(
    draft: &StretchSelectionDialogState,
    model: &Model,
) -> Result<(), String> {
    if !draft.armed || model.schematic.session.editor.tool != Tool::StretchSelection {
        return Err("Stretch is not armed.".to_owned());
    }
    validate_draft(draft, model)
}

/// End an armed Stretch: close its dialog, and let its tool go if the
/// canvas still holds it.
pub fn cancel_armed_stretch_selection_in(
    dialog: &mut StretchSelectionDialogState,
    model: &mut Model,
) {
    dialog.close();
    if model.schematic.session.editor.tool == Tool::StretchSelection {
        model.schematic.cancel_tool();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Point, Wire};
    use crate::workbench::design::test_host::DesignTestHost;

    fn selected_wire_segment() -> DesignTestHost {
        let mut state = DesignTestHost::default();
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

    /// The check box starts where the sheet is drawn: a 45-degree sheet does
    /// not have to clear it on every stretch.
    #[test]
    fn the_orthogonal_rule_starts_from_the_sheets_routing_style() {
        let mut state = selected_wire_segment();
        state.design.schematic_visibility.wire_routing = SchematicWireRoutingStyle::FortyFiveDegree;

        arm_stretch_selection_in(
            &mut state.design.stretch_selection,
            &mut state.design.move_selection,
            &mut state.design.array_selection,
            state.design.schematic_visibility.wire_routing,
            &mut state.model,
            &mut state.workbench,
        );

        assert_eq!(
            state.design.stretch_selection.policy,
            StretchOrthogonalPolicy::AllowDiagonal
        );
    }

    #[test]
    fn nothing_stretchable_refuses_instead_of_arming() {
        let mut state = DesignTestHost::default();
        arm_stretch_selection_in(
            &mut state.design.stretch_selection,
            &mut state.design.move_selection,
            &mut state.design.array_selection,
            state.design.schematic_visibility.wire_routing,
            &mut state.model,
            &mut state.workbench,
        );
        assert!(!state.design.stretch_selection.armed);
        assert_ne!(
            state.model.schematic.session.editor.tool,
            Tool::StretchSelection
        );
    }
}
