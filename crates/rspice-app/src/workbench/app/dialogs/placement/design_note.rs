//! App-owned document authority and modal lifetime for note placement.

use egui::Context;
use rspice_schematic_editor::annotations::note_placement;

use crate::state::{DesignNote, DesignNoteRenderContext, PendingDesignNotePlacement, Point, Tool};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC · DOCUMENTATION";
const TITLE: &str = "Place text or design note";
const PRIMARY: &str = "Arm text tool";
const DESCRIPTION: &str =
    "Create plain documentation, property display, requirement link, or governed review note.";
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No schematic documentation has been changed.";

#[derive(Debug)]
enum DraftValidation {
    Invalid(String),
    Valid(PendingDesignNotePlacement),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid(_) => None,
        }
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_design_note_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.design_note.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let preview_text = design_note_preview_text(&self.state);
        let discard_confirm = self.state.dialogs.design_note.discard_confirm;
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
            .initial_focus(DialogInitialFocus::Control(note_placement::text_id()));
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let mut response = dialog.show_transaction(ctx, |ui| {
            let draft = &mut self.state.dialogs.design_note;
            let response = note_placement::show(
                ui,
                validation_message.as_deref(),
                &preview_text,
                &mut draft.kind,
                &mut draft.text,
            );
            if response.edited {
                draft.mark_edited();
            }
            response.focus
        });
        match response.choice {
            DialogChoice::Primary => {
                if let DraftValidation::Valid(pending) = validate_draft(&self.state) {
                    self.state.schematic.session.editor.pending_design_note = Some(pending);
                    self.state.schematic.arm_tool(Tool::DesignNote);
                    self.state.dialogs.design_note.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.design_note.attempt_close();
                if self.state.dialogs.design_note.open {
                    response.retain_cancel_focus(DialogInitialFocus::Ghost);
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.design_note;
    if state.schematic_edit_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    if draft.design_execution_epoch != state.design_execution_epoch {
        return DraftValidation::Invalid(
            "The design document changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if draft.active_schematic_epoch != state.active_schematic_epoch {
        return DraftValidation::Invalid(
            "The active schematic buffer changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if draft.topology_version != state.schematic.topology_version() {
        return DraftValidation::Invalid(
            "The schematic topology changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    if draft.view_path != state.workspace.content.active_view.display_path() {
        return DraftValidation::Invalid(
            "The active cell/view changed. Close and reopen Place text or note.".to_owned(),
        );
    }
    match PendingDesignNotePlacement::new(
        draft.kind,
        draft.text.clone(),
        draft.topology_version,
        &state.schematic.document().design_notes,
    ) {
        Ok(pending) => DraftValidation::Valid(pending.with_document_authority(
            draft.design_execution_epoch,
            draft.active_schematic_epoch,
            draft.view_path.clone(),
        )),
        Err(error) => DraftValidation::Invalid(error.to_string()),
    }
}

fn design_note_preview_text(state: &AppState) -> String {
    let draft = &state.dialogs.design_note;
    let source = if draft.text.trim().is_empty() {
        "Bias network"
    } else {
        draft.text.trim()
    };
    let Ok(note) = DesignNote::new(0, Point::origin(), draft.kind, source) else {
        return source.to_owned();
    };
    let view_path = state.workspace.content.active_view.display_path();
    note.rendered_text(&DesignNoteRenderContext {
        view_path: &view_path,
        component_count: state.schematic.document().components.len(),
        conductor_count: state.schematic.document().wires.len()
            + state.schematic.document().buses.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_input_reaches_design_note_text() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let mut app = RSpiceApp::test_instance();
        app.state.dialogs.design_note.open(
            app.state.design_execution_epoch,
            app.state.active_schematic_epoch,
            app.state.schematic.topology_version(),
            app.state.workspace.content.active_view.display_path(),
        );
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                events: vec![
                    egui::Event::Key {
                        key: egui::Key::A,
                        physical_key: Some(egui::Key::A),
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                    },
                    egui::Event::Paste("Browser recovery qualification".to_owned()),
                ],
                ..Default::default()
            },
            |ctx| app.render_design_note_dialog(ctx),
        );
        assert_eq!(
            app.state.dialogs.design_note.text,
            "Browser recovery qualification"
        );
        assert!(app.state.dialogs.design_note.dirty);
        assert!(app.state.schematic.document().design_notes.is_empty());
    }

    #[test]
    fn valid_draft_freezes_authority_without_mutating_document() {
        let mut state = AppState::default();
        state.dialogs.design_note.open(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.content.active_view.display_path(),
        );
        let DraftValidation::Valid(pending) = validate_draft(&state) else {
            panic!("valid draft");
        };
        assert_eq!(pending.text, "Bias network");
        assert!(pending.document_authority.is_some());
        assert!(state.schematic.document().design_notes.is_empty());
    }

    #[test]
    fn stale_and_read_only_drafts_fail_closed() {
        let mut state = AppState::default();
        state.dialogs.design_note.open(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.content.active_view.display_path(),
        );
        state.schematic.session.read_only = true;
        assert!(matches!(
            validate_draft(&state),
            DraftValidation::Invalid(_)
        ));
        state.schematic.session.read_only = false;
        state.design_execution_epoch += 1;
        assert!(matches!(
            validate_draft(&state),
            DraftValidation::Invalid(_)
        ));
    }
}
