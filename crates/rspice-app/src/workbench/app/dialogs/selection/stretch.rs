//! Mockup-owned Stretch selection transaction.
//!
//! The dialog freezes document authority and configures the orthogonality
//! contract. Arming transfers exclusive intent to the schematic canvas, where
//! a candidate is previewed and committed as one undoable mutation.

use egui::Context;
use rspice_schematic_editor::selection_forms;
use rspice_schematic_editor::view::grid::snap_label;

use crate::diagnostics::ConsoleMessage;
use crate::state::{StretchTarget, Tool};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use crate::workbench::app::dialogs::schematic_command::{DISCARD_DETAIL, DISCARD_TITLE};
use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} GEOMETRY EDIT";
const TITLE: &str = "Stretch selection";
const PRIMARY: &str = "Arm stretch tool";
const DESCRIPTION: &str = "Stretch wires, buses, shapes, and parameterized geometry while keeping unaffected anchors fixed.";

#[derive(Debug)]
enum DraftValidation {
    Invalid(String),
    Valid,
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid)
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid => None,
        }
    }
}

pub(crate) fn open_stretch_selection_dialog(state: &mut AppState) {
    if state.schematic_edit_read_only() {
        state.push_user_message(ConsoleMessage::warning(
            "Stretch selection is unavailable because the active schematic is read-only."
                .to_owned(),
        ));
        return;
    }
    if !state.schematic.session.editor.selection.probes.is_empty() {
        state.push_user_message(ConsoleMessage::warning(
            "Probe markers cannot be stretched; move the retained probe marker instead.".to_owned(),
        ));
        return;
    }
    let Some(target) = state.schematic.default_stretch_target() else {
        state.push_user_message(ConsoleMessage::warning(
            "Select one stretchable wire, bus, or documentation shape before opening Stretch selection."
                .to_owned(),
        ));
        return;
    };
    if state.dialogs.move_selection.armed {
        crate::workbench::app::cancel_armed_move_selection(state);
    }
    if state.dialogs.array_selection.armed {
        crate::workbench::app::cancel_armed_array_selection(state);
    }
    state
        .dialogs
        .stretch_selection
        .open(SchematicEditAuthority::capture(state), target);
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_stretch_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.stretch_selection.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let selection = target_summary(&self.state);
        let snap = snap_label(self.state.schematic.document().document_policy.grid_pitch);
        let discard_confirm = self.state.dialogs.stretch_selection.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let mut response = dialog.show_transaction(ctx, |ui| {
            let draft = &mut self.state.dialogs.stretch_selection;
            let (focus, edited) = selection_forms::show_stretch(
                ui,
                &selection,
                snap,
                validation_message.as_deref(),
                &mut draft.policy,
            );
            if edited {
                draft.mark_edited();
            }
            focus
        });
        match response.choice {
            DialogChoice::Primary => {
                if validate_draft(&self.state).can_commit() {
                    self.state.dialogs.stretch_selection.arm();
                    self.state.schematic.arm_tool(Tool::StretchSelection);
                    rspice_schematic_editor::view::canvas::request_focus(ctx);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Stretch selection armed with {}; choose an anchor and destination on the {snap} grid.",
                        self.state.dialogs.stretch_selection.policy.label()
                    )));
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.stretch_selection.attempt_close();
                if self.state.dialogs.stretch_selection.open {
                    response.retain_cancel_focus(DialogInitialFocus::Ghost);
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    if state.schematic_edit_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    let draft = &state.dialogs.stretch_selection;
    let Some(authority) = draft.authority.as_ref() else {
        return DraftValidation::Invalid(
            "The retained design baseline is unavailable. Close and reopen Stretch selection."
                .to_owned(),
        );
    };
    if let Err(message) = authority.validate(state, TITLE) {
        return DraftValidation::Invalid(message);
    }
    let Some(target) = draft.canvas.target else {
        return DraftValidation::Invalid(
            "No stretch target is retained. Close and reopen Stretch selection.".to_owned(),
        );
    };
    if !state.schematic.is_stretch_target_eligible(target) {
        return DraftValidation::Invalid(
            "The retained stretch target is no longer eligible. Close and reopen Stretch selection."
                .to_owned(),
        );
    }
    DraftValidation::Valid
}

pub(crate) fn armed_stretch_selection_authority(state: &AppState) -> Result<(), String> {
    let draft = &state.dialogs.stretch_selection;
    if !draft.armed || state.schematic.session.editor.tool != Tool::StretchSelection {
        return Err("Stretch selection is not armed.".to_owned());
    }
    match validate_draft(state) {
        DraftValidation::Valid => Ok(()),
        DraftValidation::Invalid(message) => Err(message),
    }
}

pub(crate) fn cancel_armed_stretch_selection(state: &mut AppState) {
    state.dialogs.stretch_selection.close();
    if state.schematic.session.editor.tool == Tool::StretchSelection {
        state.schematic.cancel_tool();
    }
}

fn target_summary(state: &AppState) -> String {
    match state.dialogs.stretch_selection.canvas.target {
        Some(StretchTarget::WireSegment {
            wire_id,
            segment_index,
        }) => format!("wire {wire_id} \u{00b7} segment {}", segment_index + 1),
        Some(StretchTarget::BusSegment {
            bus_id,
            segment_index,
        }) => format!("bus {bus_id} \u{00b7} segment {}", segment_index + 1),
        Some(StretchTarget::DocumentationShapePoint {
            shape_id,
            point_index,
        }) => {
            let kind = state
                .schematic
                .document()
                .documentation_shapes
                .iter()
                .find(|shape| shape.id == shape_id)
                .map_or("documentation shape", |shape| shape.geometry.kind().label());
            format!(
                "{kind} {shape_id} \u{00b7} control point {}",
                point_index + 1
            )
        }
        None => "No stretch target".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Point, StretchOrthogonalPolicy, Wire};
    use crate::workbench::app::dialogs::state::StretchSelectionDialogState;

    #[test]
    fn open_freezes_authority_and_exact_default_policy() {
        let mut state = AppState::default();
        let wire = Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]);
        state.schematic.document_mut_for_test().wires.push(wire);
        state.schematic.session.editor.selection.select_wire(7);

        open_stretch_selection_dialog(&mut state);

        let draft = &state.dialogs.stretch_selection;
        assert!(draft.open);
        assert_eq!(draft.policy, StretchOrthogonalPolicy::PreserveOrthogonal);
        assert_eq!(
            draft.canvas.target,
            Some(StretchTarget::WireSegment {
                wire_id: 7,
                segment_index: 0,
            })
        );
        assert!(draft.authority.is_some());
    }

    #[test]
    fn edited_policy_requires_two_close_attempts() {
        let mut draft = StretchSelectionDialogState::default();
        draft.open = true;
        draft.mark_edited();
        assert!(!draft.attempt_close());
        assert!(draft.open);
        assert!(draft.discard_confirm);
        assert!(draft.attempt_close());
        assert!(!draft.open);
    }
}
