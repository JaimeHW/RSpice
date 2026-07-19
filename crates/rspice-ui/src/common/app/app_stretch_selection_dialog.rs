//! Mockup-owned Stretch selection transaction.
//!
//! The dialog freezes document authority and configures the orthogonality
//! contract. Arming transfers exclusive intent to the schematic canvas, where
//! a candidate is previewed and committed as one undoable mutation.

use egui::{Context, Ui};

use crate::state::{Point, StretchOrthogonalPolicy, StretchTarget, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    SchematicCommandPreview, schematic_command_workflow, select_with_response,
};

use super::app_schematic_command_dialog::{
    DISCARD_DETAIL, DISCARD_TITLE, FOOTER_NOTE, field_label, read_only_value, snap_label,
};
use super::{
    AppState, ConsoleMessage, RSpiceApp, SchematicEditAuthority, StretchSelectionDialogState,
};

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
    if state.schematic.read_only || state.active_view_read_only() {
        state.push_user_message(ConsoleMessage::warning(
            "Stretch selection is unavailable because the active schematic is read-only."
                .to_owned(),
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
        super::cancel_armed_move_selection(state);
    }
    state
        .dialogs
        .stretch_selection
        .open(SchematicEditAuthority::capture(state), target);
}

impl RSpiceApp {
    pub(super) fn render_stretch_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.stretch_selection.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let selection = target_summary(&self.state);
        let snap = snap_label(self.state.schematic.document_policy.grid_pitch);
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
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            workflow_body(
                ui,
                &selection,
                snap,
                validation_message.as_deref(),
                &mut self.state.dialogs.stretch_selection,
            )
        });
        match choice {
            DialogChoice::Primary => {
                if validate_draft(&self.state).can_commit() {
                    self.state.dialogs.stretch_selection.arm();
                    crate::workbench::commands::arm_schematic_tool(
                        &mut self.state.schematic,
                        Tool::StretchSelection,
                    );
                    crate::schematic::view::request_schematic_canvas_focus(ctx);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Stretch selection armed with {}; choose an anchor and destination on the {snap} grid.",
                        self.state.dialogs.stretch_selection.policy.label()
                    )));
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.stretch_selection.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    if state.schematic.read_only || state.active_view_read_only() {
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
    let Some(target) = draft.target else {
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
    if !draft.armed || state.schematic.tool != Tool::StretchSelection {
        return Err("Stretch selection is not armed.".to_owned());
    }
    match validate_draft(state) {
        DraftValidation::Valid => Ok(()),
        DraftValidation::Invalid(message) => Err(message),
    }
}

pub(crate) fn cancel_armed_stretch_selection(state: &mut AppState) {
    state.dialogs.stretch_selection.close();
    if state.schematic.tool == Tool::StretchSelection {
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
    }
}

fn workflow_body(
    ui: &mut Ui,
    selection: &str,
    snap: &str,
    validation_message: Option<&str>,
    draft: &mut StretchSelectionDialogState,
) -> Option<egui::Id> {
    let preview = SchematicCommandPreview {
        subject: selection,
        location: "anchor and destination pending",
        electrical_outcome: match draft.policy {
            StretchOrthogonalPolicy::PreserveOrthogonal => "orthogonal segment update",
            StretchOrthogonalPolicy::AllowDiagonal => "diagonal segment update",
        },
        grid: snap,
    };
    let focus = schematic_command_workflow(
        ui,
        "STRETCH",
        preview,
        if validation_message.is_some() {
            "blocked"
        } else {
            "legal preview"
        },
        validation_message.is_none(),
        |ui| {
            read_only_value(ui, "Selection", selection);
            ui.add_space(9.0);
            let labels = StretchOrthogonalPolicy::ALL.map(|policy| policy.label().to_owned());
            let output = field_label(ui, "Orthogonal policy", |ui| {
                select_with_response(
                    ui,
                    "stretch-selection-orthogonal-policy",
                    "Orthogonal policy",
                    draft.policy.label(),
                    &labels,
                    ui.available_width(),
                )
            });
            ui.add_space(9.0);
            read_only_value(ui, "Snap", snap);
            ui.add_space(12.0);
            let t = Tokens::get(ui.ctx());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(validation_message.unwrap_or(FOOTER_NOTE))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(if validation_message.is_some() {
                            t.color.err
                        } else {
                            t.color.text_faint
                        }),
                )
                .wrap(),
            );
            output
        },
    );
    if let Some(index) = focus.picked {
        let next = StretchOrthogonalPolicy::ALL[index];
        if next != draft.policy {
            draft.policy = next;
            draft.mark_edited();
        }
    }
    Some(focus.response.id)
}

fn target_summary(state: &AppState) -> String {
    match state.dialogs.stretch_selection.target {
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

pub(crate) fn stretch_delta_for_policy(
    delta: Point,
    target: StretchTarget,
    policy: StretchOrthogonalPolicy,
    state: &AppState,
) -> Point {
    if policy == StretchOrthogonalPolicy::AllowDiagonal {
        return delta;
    }
    let segment_end = match target {
        StretchTarget::WireSegment { segment_index, .. }
        | StretchTarget::BusSegment { segment_index, .. } => segment_index.checked_add(1),
        StretchTarget::DocumentationShapePoint { .. } => return delta,
    };
    let Some(segment_end) = segment_end else {
        return delta;
    };
    let segment = match target {
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        } => state
            .schematic
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .and_then(|wire| wire.points.get(segment_index..=segment_end)),
        StretchTarget::BusSegment {
            bus_id,
            segment_index,
        } => state
            .schematic
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .and_then(|bus| bus.points.get(segment_index..=segment_end)),
        StretchTarget::DocumentationShapePoint { .. } => return delta,
    };
    let Some(segment) = segment else {
        return delta;
    };
    if segment[0].x == segment[1].x {
        Point::new(delta.x, 0)
    } else if segment[0].y == segment[1].y {
        Point::new(0, delta.y)
    } else {
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{SchematicGridPitch, Wire};

    #[test]
    fn open_freezes_authority_and_exact_default_policy() {
        let mut state = AppState::default();
        let wire = Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]);
        state.schematic.wires.push(wire);
        state.schematic.selection.select_wire(7);

        open_stretch_selection_dialog(&mut state);

        let draft = &state.dialogs.stretch_selection;
        assert!(draft.open);
        assert_eq!(draft.policy, StretchOrthogonalPolicy::PreserveOrthogonal);
        assert_eq!(
            draft.target,
            Some(StretchTarget::WireSegment {
                wire_id: 7,
                segment_index: 0,
            })
        );
        assert!(draft.authority.is_some());
    }

    #[test]
    fn preserve_orthogonal_projects_motion_perpendicular_to_segment() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]));
        let target = StretchTarget::WireSegment {
            wire_id: 7,
            segment_index: 0,
        };
        assert_eq!(
            stretch_delta_for_policy(
                Point::new(30, 40),
                target,
                StretchOrthogonalPolicy::PreserveOrthogonal,
                &state,
            ),
            Point::new(0, 40)
        );
        assert_eq!(
            snap_label(SchematicGridPitch::Mil50),
            "50 mil",
            "shared mockup snap copy remains exact"
        );
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
