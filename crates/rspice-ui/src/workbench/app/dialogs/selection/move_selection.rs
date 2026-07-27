//! Mockup-owned Move selection transaction.
//!
//! The dialog owns configuration and authority only. Arming does not mutate
//! the document; the schematic interaction layer accumulates a snapped
//! pointer/keyboard delta and commits the configured movement exactly once.

use egui::{Context, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::schematic::view::SchematicSymbolContext;
use crate::state::{MoveSelectionMode, Point, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    SchematicCommandPreview, schematic_command_workflow, select_with_response,
};

use crate::workbench::app::{AppState, MoveSelectionDialogState, RSpiceApp, SchematicEditAuthority};

const EYEBROW: &str = "SCHEMATIC \u{00b7} CONNECTIVITY PRESERVING";
const TITLE: &str = "Move selection";
const PRIMARY: &str = "Arm move tool";
const DESCRIPTION: &str = "Move selected objects with connected wires following and preview any resulting geometry or hierarchy violations.";
use crate::workbench::app::dialogs::schematic_command::{
    DISCARD_DETAIL, DISCARD_TITLE, FOOTER_NOTE, field_label, read_only_value, snap_label,
};

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

/// Open the exact mockup transaction against an immutable document and
/// selection baseline. Commands call this single boundary so palette, menu,
/// and shortcut activation cannot drift.
pub(crate) fn open_move_selection_dialog(state: &mut AppState) {
    if state.schematic.read_only || state.active_view_read_only() {
        state.push_user_message(ConsoleMessage::warning(
            "Move selection is unavailable because the active schematic is read-only.".to_owned(),
        ));
        return;
    }
    if !state.schematic.has_live_movable_selection() {
        state.push_user_message(ConsoleMessage::warning(
            "Select at least one movable schematic object before opening Move selection."
                .to_owned(),
        ));
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
        .open(SchematicEditAuthority::capture(state));
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_move_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.move_selection.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let summary = selection_summary(&self.state);
        let snap = snap_label(self.state.schematic.document_policy.grid_pitch);
        let discard_confirm = self.state.dialogs.move_selection.discard_confirm;
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
                &summary,
                snap,
                validation_message.as_deref(),
                &mut self.state.dialogs.move_selection,
            )
        });
        match choice {
            DialogChoice::Primary => {
                if validate_draft(&self.state).can_commit() {
                    self.state.dialogs.move_selection.arm();
                    self.state.schematic.arm_tool(Tool::MoveSelection);
                    crate::schematic::view::request_schematic_canvas_focus(ctx);
                    let snap = snap_label(self.state.schematic.document_policy.grid_pitch);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Move selection armed in {} mode; choose an anchor and destination on the {snap} grid.",
                        self.state.dialogs.move_selection.mode.label()
                    )));
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.move_selection.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    if state.schematic.read_only || state.active_view_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    let draft = &state.dialogs.move_selection;
    let Some(authority) = draft.authority.as_ref() else {
        return DraftValidation::Invalid(
            "The retained design baseline is unavailable. Close and reopen Move selection."
                .to_owned(),
        );
    };
    if let Err(message) = authority.validate(state, TITLE) {
        return DraftValidation::Invalid(message);
    }
    if !state.schematic.has_live_movable_selection() {
        return DraftValidation::Invalid("No movable selected object remains.".to_owned());
    }
    DraftValidation::Valid
}

/// Fail-closed authority check used again after the dialog has closed and the
/// move tool is armed.
pub(crate) fn armed_move_selection_authority(state: &AppState) -> Result<(), String> {
    let draft = &state.dialogs.move_selection;
    if !draft.armed || state.schematic.tool != Tool::MoveSelection {
        return Err("Move selection is not armed.".to_owned());
    }
    match validate_draft(state) {
        DraftValidation::Valid => Ok(()),
        DraftValidation::Invalid(message) => Err(message),
    }
}

pub(crate) fn cancel_armed_move_selection(state: &mut AppState) {
    state.dialogs.move_selection.close();
    if state.schematic.tool == Tool::MoveSelection {
        state.schematic.cancel_tool();
    }
}

fn workflow_body(
    ui: &mut Ui,
    summary: &str,
    snap: &str,
    validation_message: Option<&str>,
    draft: &mut MoveSelectionDialogState,
) -> Option<egui::Id> {
    let preview = SchematicCommandPreview {
        subject: summary,
        location: "anchor and destination pending",
        electrical_outcome: "connectivity-preserving transform",
        grid: snap,
    };
    let focus = schematic_command_workflow(
        ui,
        "MOVE",
        preview,
        if validation_message.is_some() {
            "blocked"
        } else {
            "legal preview"
        },
        validation_message.is_none(),
        |ui| {
            let labels = MoveSelectionMode::ALL.map(|mode| mode.label().to_owned());
            let output = field_label(ui, "Mode", |ui| {
                select_with_response(
                    ui,
                    "move-selection-mode",
                    "Move mode",
                    draft.mode.label(),
                    &labels,
                    ui.available_width(),
                )
            });
            ui.add_space(9.0);
            read_only_value(ui, "Snap", snap);
            ui.add_space(9.0);
            read_only_value(ui, "Selection", summary);
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
        let next = MoveSelectionMode::ALL[index];
        if next != draft.mode {
            draft.mode = next;
            draft.mark_edited();
        }
    }
    Some(focus.response.id)
}

fn selection_summary(state: &AppState) -> String {
    let selection = &state.schematic.selection;
    let count = state.schematic.live_movable_selection_count();
    let symbol_context = SchematicSymbolContext::from_state(state);
    let terminals: std::collections::HashSet<Point> = state
        .schematic
        .components
        .iter()
        .filter(|component| selection.has_component(component.id))
        .flat_map(|component| symbol_context.terminal_points(component))
        .collect();
    let attached = state
        .schematic
        .wires
        .iter()
        .filter(|wire| {
            !selection.has_wire(wire.id)
                && wire.points.iter().any(|point| terminals.contains(point))
        })
        .count();
    if let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| selection.has_component(component.id))
        .filter(|_| count == 1)
    {
        format!("{} \u{00b7} {attached} attached wires", component.name)
    } else {
        format!("{count} selected objects \u{00b7} {attached} attached wires")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point, SchematicSnapshot};

    #[test]
    fn open_freezes_selection_and_complete_design_snapshot() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.schematic.selection.select_component(id);
        open_move_selection_dialog(&mut state);
        let draft = &state.dialogs.move_selection;
        assert!(draft.open);
        let authority = draft.authority.as_ref().expect("captured authority");
        assert_eq!(authority.selection, state.schematic.selection);
        assert!(
            authority
                .snapshot
                .is_equal(&SchematicSnapshot::capture(&state.schematic))
        );
    }

    #[test]
    fn validation_fails_closed_when_non_electrical_geometry_changes() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.schematic.selection.select_component(id);
        open_move_selection_dialog(&mut state);
        state.schematic.components[0].value = "2k".to_owned();
        assert!(matches!(
            validate_draft(&state),
            DraftValidation::Invalid(_)
        ));
    }

    #[test]
    fn arming_retains_selection_and_does_not_mutate_design() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.schematic.selection.select_component(id);
        open_move_selection_dialog(&mut state);
        let baseline = SchematicSnapshot::capture(&state.schematic);
        state.dialogs.move_selection.arm();
        state.schematic.arm_tool(Tool::MoveSelection);
        assert!(baseline.is_equal(&SchematicSnapshot::capture(&state.schematic)));
        assert!(state.schematic.selection.has_component(id));
        assert!(armed_move_selection_authority(&state).is_ok());
    }

    #[test]
    fn edited_mode_requires_explicit_discard_confirmation() {
        let mut draft = MoveSelectionDialogState {
            open: true,
            ..MoveSelectionDialogState::default()
        };
        draft.mark_edited();
        assert!(!draft.attempt_close());
        assert!(draft.open);
        assert!(draft.discard_confirm);
        assert!(draft.attempt_close());
        assert!(!draft.open);
    }
}
