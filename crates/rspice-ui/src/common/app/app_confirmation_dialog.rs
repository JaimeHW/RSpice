//! Save-changes confirmation — the template instance of the VOLTA modal
//! grammar: small surface, one sentence,
//! three explicit verbs. Ghost = Cancel, secondary = Don't save,
//! primary = Save.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use super::{ConfirmationAction, ConfirmationResponse, ProjectReviewRequest, RSpiceApp};

impl RSpiceApp {
    pub(super) fn render_confirmation_dialog(&mut self, ctx: &Context) {
        if self.state.dialogs.confirmation_dialog.visible {
            self.render_save_confirmation_dialog(ctx);
        }
        #[cfg(target_arch = "wasm32")]
        self.render_pending_browser_file_operation(ctx);
        self.render_project_review_dialog(ctx);
    }

    #[cfg(target_arch = "wasm32")]
    fn render_pending_browser_file_operation(&mut self, ctx: &Context) {
        if self.state.dialogs.confirmation_dialog.visible {
            return;
        }
        let Some(message) =
            crate::common::project_workflow::browser_file_operation_label(&self.state)
        else {
            return;
        };

        let choice = Dialog::new(
            "FILE · BROWSER",
            "Browser file operation",
            "Cancel operation",
        )
        .size(DialogSize::Sm)
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new(message)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new(
                    "Canceling releases RSpice immediately. A late browser response cannot replace or save over newer work.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            crate::common::project_workflow::cancel_pending_browser_file_operation(&mut self.state);
        }
    }

    fn render_save_confirmation_dialog(&mut self, ctx: &Context) {
        let pending_action = self.state.dialogs.confirmation_dialog.pending_action;
        let title = pending_action
            .as_ref()
            .map(ConfirmationAction::dialog_title)
            .unwrap_or("Save changes?");
        let prompt = pending_action
            .as_ref()
            .map(ConfirmationAction::prompt_message)
            .unwrap_or(
                "The current schematic has unsaved changes.\nDo you want to save before continuing?",
            );

        let choice = Dialog::new("Schematic", title, "Save")
            .size(DialogSize::Sm)
            .secondary("Don't save")
            .ghost("Cancel")
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                for line in prompt.lines() {
                    ui.label(
                        egui::RichText::new(line)
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                }
            });

        let response = match choice {
            DialogChoice::Primary => Some(ConfirmationResponse::Yes),
            DialogChoice::Secondary => Some(ConfirmationResponse::No),
            DialogChoice::Ghost | DialogChoice::Cancelled => Some(ConfirmationResponse::Cancel),
            DialogChoice::None => None,
        };
        if let Some(r) = response {
            self.handle_confirmation_response(r);
        }
    }

    fn render_project_review_dialog(&mut self, ctx: &Context) {
        let Some(request) = self.state.dialogs.project_review_dialog.request.clone() else {
            return;
        };
        match request {
            ProjectReviewRequest::RevertActive(token) => {
                let document = token.document_label();
                let choice = Dialog::new(
                    "PROJECT · WORKING CHANGE",
                    "Revert active document",
                    "Revert document",
                )
                .size(DialogSize::Sm)
                .ghost("Cancel")
                .show(ctx, |ui| {
                    let t = Tokens::get(ui.ctx());
                    ui.label(
                        egui::RichText::new(format!("Document: {document}"))
                            .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Restore only this document's accepted engineering content. Other documents and their working changes remain untouched.",
                        )
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                    );
                });
                match choice {
                    DialogChoice::Primary => {
                        self.state.dialogs.project_review_dialog.close();
                        crate::common::project_workflow::confirm_revert_active_document(
                            &mut self.state,
                            &token,
                        );
                    }
                    DialogChoice::Ghost | DialogChoice::Cancelled => {
                        self.state.dialogs.project_review_dialog.close();
                    }
                    DialogChoice::Secondary | DialogChoice::None => {}
                }
            }
            ProjectReviewRequest::CloseProject => {
                let dirty = crate::common::project_lifecycle::dirty_document_count(&self.state);
                let running = self.state.simulation.is_running;
                let primary = if dirty > 0 {
                    "Save all and close"
                } else {
                    "Close project"
                };
                let mut dialog = Dialog::new("PROJECT · SAFE SHUTDOWN", "Close project", primary)
                    .size(DialogSize::Sm)
                    .ghost("Cancel")
                    .primary_enabled(!running);
                if dirty > 0 && !running {
                    dialog = dialog.secondary("Close without saving");
                }
                let choice = dialog.show(ctx, |ui| {
                    let t = Tokens::get(ui.ctx());
                    let documents = if dirty == 1 { "document" } else { "documents" };
                    ui.label(
                        egui::RichText::new(format!("Unsaved: {dirty} {documents}"))
                            .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                            .color(if dirty > 0 { t.color.warn } else { t.color.text }),
                    );
                    ui.label(
                        egui::RichText::new(if running {
                            "Local simulation: active. Stop or cancel it and wait for completion before closing this project."
                        } else {
                            "Local simulation: none active"
                        })
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(if running { t.color.warn } else { t.color.text_dim }),
                    );
                });
                match choice {
                    DialogChoice::Primary if dirty > 0 => {
                        self.begin_close_project_after_save();
                    }
                    DialogChoice::Primary => {
                        self.state.dialogs.project_review_dialog.close();
                        crate::common::project_workflow::close_project_discard(&mut self.state);
                    }
                    DialogChoice::Secondary => {
                        self.state.dialogs.project_review_dialog.close();
                        crate::common::project_workflow::close_project_discard(&mut self.state);
                    }
                    DialogChoice::Ghost | DialogChoice::Cancelled => {
                        self.state.dialogs.project_review_dialog.close();
                    }
                    DialogChoice::None => {}
                }
            }
        }
    }
}
