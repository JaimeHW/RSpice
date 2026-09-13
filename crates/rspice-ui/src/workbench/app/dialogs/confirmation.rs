//! Save-changes confirmation — the template instance of the RSpice modal
//! grammar: small surface, one sentence,
//! three explicit verbs. Ghost = Cancel, secondary = Don't save,
//! primary = Save.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use crate::workbench::app::{
    ConfirmationAction, ConfirmationResponse, ProjectReviewRequest, RSpiceApp,
};

impl RSpiceApp {
    pub(in crate::workbench) fn render_confirmation_dialog(&mut self, ctx: &Context) {
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
            crate::workbench::workflows::project_workflow::browser_file_operation_label(
                &self.state,
            )
        else {
            return;
        };

        let choice = Dialog::new(
            "FILE · BROWSER",
            "Browser file operation",
            "Cancel operation",
        )
        .description(
            "Review or cancel the pending browser file operation while protecting newer work from a late response.",
        )
        .size(DialogSize::Transaction)
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
            crate::workbench::workflows::project_workflow::cancel_pending_browser_file_operation(
                &mut self.state,
            );
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

        let exiting = pending_action == Some(ConfirmationAction::Exit);
        let choice = Dialog::new(
            if exiting { "RSPICE - SAFE EXIT" } else { "Schematic" },
            title,
            if exiting { "Save / retain" } else { "Save" },
        )
            .description(if exiting {
                "Choose whether to save project changes and retain recoverable Models & PDK authoring drafts before exiting."
            } else {
                "Choose whether to save the current schematic's unsaved changes before continuing."
            })
            .size(DialogSize::Transaction)
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
                .description(
                    "Discard explicit working changes in the active document while leaving other documents unchanged.",
                )
                .size(DialogSize::Transaction)
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
                        crate::workbench::workflows::project_workflow::confirm_revert_active_document(
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
                let model_draft_dirty = self.state.workbench.model_editor_has_unsaved_changes();
                let dirty = crate::workbench::lifecycle::project_lifecycle::dirty_document_count(
                    &self.state,
                ) + usize::from(model_draft_dirty);
                let running = self.state.simulation.has_active_execution();
                let primary = if dirty > 0 {
                    "Save all and close"
                } else {
                    "Close project"
                };
                let mut dialog = Dialog::new("PROJECT · SAFE SHUTDOWN", "Close project", primary)
                    .description(
                        "Review unsaved documents and active local simulation state before closing this project.",
                    )
                    .size(DialogSize::Confirmation)
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
                    if model_draft_dirty {
                        ui.label(
                            egui::RichText::new(
                                "Device model editor: one unsaved project-model candidate",
                            )
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.warn),
                        );
                    }
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
                        crate::workbench::workflows::project_workflow::close_project_discard(
                            &mut self.state,
                        );
                    }
                    DialogChoice::Secondary => {
                        self.state.dialogs.project_review_dialog.close();
                        crate::workbench::workflows::project_workflow::close_project_discard(
                            &mut self.state,
                        );
                    }
                    DialogChoice::Ghost | DialogChoice::Cancelled => {
                        self.state.dialogs.project_review_dialog.close();
                        self.state.workbench.cancel_project_close();
                    }
                    DialogChoice::None => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Close Project is three status lines over three verbs. Every one of
    /// them lands inside the centred 480 pt confirmation surface, and the
    /// verbs share one footer row at desktop width.
    #[test]
    fn close_project_review_renders_on_the_narrow_confirmation_surface() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.dialogs.project_review_dialog.show_close_project();
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1_280.0, 800.0));
        let mut output = None;
        for _ in 0..3 {
            output = Some(ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                },
                |ui| app.render_confirmation_dialog(ui.ctx()),
            ));
        }
        let painted: Vec<(String, egui::Rect)> = output
            .expect("three passes ran")
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::epaint::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => None,
            })
            .collect();
        let find = |label: &str| {
            painted
                .iter()
                .find(|(text, _)| text == label)
                .map(|(_, rect)| *rect)
                .unwrap_or_else(|| panic!("{label:?} was not painted: {painted:?}"))
        };

        let surface =
            egui::Rect::from_center_size(screen.center(), egui::vec2(480.0, screen.height()));
        let verbs = ["Cancel", "Close without saving", "Save all and close"].map(find);
        for rect in [find("Close project"), find("Unsaved: 1 document")]
            .into_iter()
            .chain(verbs)
        {
            assert!(surface.contains_rect(rect), "{rect:?} escapes {surface:?}");
        }
        for rect in &verbs[1..] {
            assert!((rect.center().y - verbs[0].center().y).abs() < 1.0);
        }
    }
}
