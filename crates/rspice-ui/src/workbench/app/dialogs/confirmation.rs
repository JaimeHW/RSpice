//! Save-changes confirmation — the template instance of the RSpice modal
//! grammar: small surface, one sentence,
//! three explicit verbs. Ghost = Cancel, secondary = Don't save,
//! primary = Save.

use egui::{Context, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use crate::workbench::app::{
    ConfirmationAction, ConfirmationResponse, ProjectReviewRequest, RSpiceApp,
};
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::lifecycle::project_lifecycle::ProjectDocumentId;

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
            ProjectReviewRequest::CloseProject => self.render_close_project_review(ctx),
        }
    }

    /// The close-project prompt. While a local run owns the project it asks
    /// for the run to stop; otherwise it is the standard save-changes
    /// question. A clean, idle project never opens it, and one that turns
    /// clean and idle while it is open, because its run ended with nothing
    /// unsaved, closes without asking again.
    fn render_close_project_review(&mut self, ctx: &Context) {
        let mut unsaved: Vec<(WorkbenchIcon, String)> =
            crate::workbench::lifecycle::project_lifecycle::dirty_documents(&self.state)
                .iter()
                .map(|document| (document_icon(document), document.label()))
                .collect();
        if self.state.workbench.model_editor_has_unsaved_changes() {
            unsaved.push((
                WorkbenchIcon::Models,
                "Device model editor · unsaved candidate".to_owned(),
            ));
        }
        if self.state.simulation.has_active_execution() {
            self.render_close_blocked_by_run(ctx, &unsaved);
        } else if unsaved.is_empty() {
            self.state.dialogs.project_review_dialog.close();
            crate::workbench::workflows::project_workflow::close_project_discard(&mut self.state);
        } else {
            self.render_close_save_prompt(ctx, &unsaved);
        }
    }

    fn render_close_save_prompt(&mut self, ctx: &Context, unsaved: &[(WorkbenchIcon, String)]) {
        let project = self.state.workspace.project.name().trim();
        let title = if project.is_empty() {
            "Save changes before closing?".to_owned()
        } else {
            format!("Save changes to \u{201c}{project}\u{201d}?")
        };
        let summary = match unsaved.len() {
            1 => "1 document has unsaved changes.".to_owned(),
            count => format!("{count} documents have unsaved changes."),
        };
        let choice = Dialog::prompt(
            &title,
            if unsaved.len() > 1 {
                "Save all"
            } else {
                "Save"
            },
        )
        .description(format!("{summary} Save them or close without saving."))
        .size(DialogSize::Confirmation)
        .secondary("Don't save")
        .secondary_leading()
        .ghost("Cancel")
        .show(ctx, |ui| {
            close_review_sentence(ui, None, &summary);
            unsaved_list(ui, unsaved);
        });
        match choice {
            DialogChoice::Primary => self.begin_close_project_after_save(),
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

    fn render_close_blocked_by_run(&mut self, ctx: &Context, unsaved: &[(WorkbenchIcon, String)]) {
        let stopping = self.state.simulation.cancellation_is_pending();
        let can_stop = self.state.simulation.can_request_abort_active_run();
        // With no stop to request, Cancel is the only verb, so it is the primary.
        let primary = if stopping {
            "Stopping\u{2026}"
        } else if can_stop {
            "Stop simulation"
        } else {
            "Cancel"
        };
        let mut dialog = Dialog::prompt("Stop the simulation to close this project", primary)
            .description("A local simulation has to stop before the project can close.")
            .size(DialogSize::Confirmation)
            .primary_enabled(!stopping);
        if stopping || can_stop {
            dialog = dialog.ghost("Cancel");
        }
        let choice = dialog.show(ctx, |ui| {
            close_review_sentence(
                ui,
                Some(WorkbenchIcon::Warning),
                "A local simulation is running. Stop it and wait for it to finish before closing.",
            );
            unsaved_list(ui, unsaved);
        });
        match choice {
            DialogChoice::Primary if can_stop => {
                if let Err(error) = self.state.simulation.request_abort_active_run() {
                    self.state.push_sim_message(ConsoleMessage::warning(error));
                }
            }
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.project_review_dialog.close();
                self.state.workbench.cancel_project_close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn document_icon(document: &ProjectDocumentId) -> WorkbenchIcon {
    match document {
        ProjectDocumentId::ProjectConfiguration => WorkbenchIcon::Project,
        ProjectDocumentId::CellView(_) => WorkbenchIcon::Design,
        ProjectDocumentId::SimulationPlan => WorkbenchIcon::Simulate,
        ProjectDocumentId::ResultHistory => WorkbenchIcon::Results,
        ProjectDocumentId::VerificationSpecifications => WorkbenchIcon::Verify,
        ProjectDocumentId::ModelCatalog => WorkbenchIcon::Models,
        ProjectDocumentId::NetlistSource => WorkbenchIcon::Netlist,
        ProjectDocumentId::StimulusLibrary => WorkbenchIcon::Source,
    }
}

/// The prompt's one sentence in body text, with an optional mark beside it.
fn close_review_sentence(ui: &mut Ui, mark: Option<WorkbenchIcon>, sentence: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        if let Some(mark) = mark {
            let (slot, _) = ui.allocate_exact_size(egui::vec2(14.0, 18.0), egui::Sense::hover());
            mark.paint(
                ui.painter(),
                egui::Rect::from_center_size(slot.center(), egui::Vec2::splat(14.0)),
                t.color.warn,
            );
        }
        ui.add(
            egui::Label::new(
                egui::RichText::new(sentence)
                    .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                    .color(t.color.text),
            )
            .wrap(),
        );
    });
}

/// The unsaved work a close would lose, one row per item: a kind mark and the
/// item's name. Five rows at most, so a large project keeps the prompt short.
fn unsaved_list(ui: &mut Ui, unsaved: &[(WorkbenchIcon, String)]) {
    const SHOWN_ROWS: usize = 5;
    const ROW_HEIGHT: f32 = 22.0;
    const NAME_INDENT: f32 = 22.0;
    if unsaved.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for (icon, name) in unsaved.iter().take(SHOWN_ROWS) {
            let (row, response) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), ROW_HEIGHT),
                egui::Sense::hover(),
            );
            icon.paint(
                ui.painter(),
                egui::Rect::from_center_size(
                    egui::pos2(row.left() + 7.0, row.center().y),
                    egui::Vec2::splat(14.0),
                ),
                t.color.text_dim,
            );
            let shown = crate::workbench::design_system::elide_text(
                ui,
                name,
                &font,
                row.width() - NAME_INDENT,
            );
            let elided = shown != *name;
            ui.painter().text(
                egui::pos2(row.left() + NAME_INDENT, row.center().y),
                egui::Align2::LEFT_CENTER,
                shown,
                font.clone(),
                t.color.text,
            );
            response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, name));
            if elided {
                response.on_hover_text(name);
            }
        }
        if unsaved.len() > SHOWN_ROWS {
            ui.horizontal(|ui| {
                ui.add_space(NAME_INDENT);
                ui.label(
                    egui::RichText::new(format!("and {} more", unsaved.len() - SHOWN_ROWS))
                        .font(font.clone())
                        .color(t.color.text_dim),
                );
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_close_review(app: &mut RSpiceApp, size: egui::Vec2) -> egui::FullOutput {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut output = None;
        for _ in 0..3 {
            output = Some(ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ui| app.render_confirmation_dialog(ui.ctx()),
            ));
        }
        output.expect("three passes ran")
    }

    fn visit_shapes(shape: &egui::epaint::Shape, visit: &mut impl FnMut(&egui::epaint::Shape)) {
        if let egui::epaint::Shape::Vec(shapes) = shape {
            for shape in shapes {
                visit_shapes(shape, visit);
            }
        } else {
            visit(shape);
        }
    }

    fn painted_text(output: &egui::FullOutput) -> Vec<(String, egui::Rect)> {
        let mut painted = Vec::new();
        for clipped in &output.shapes {
            visit_shapes(&clipped.shape, &mut |shape| {
                if let egui::epaint::Shape::Text(text) = shape {
                    painted.push((
                        text.galley.text().to_owned(),
                        egui::Rect::from_min_size(text.pos, text.galley.size()),
                    ));
                }
            });
        }
        painted
    }

    fn find(painted: &[(String, egui::Rect)], wanted: &str) -> egui::Rect {
        painted
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{wanted:?} was not painted: {painted:?}"))
    }

    /// A never-saved project named "lna-frontend", asked to close.
    fn unsaved_project_closing() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .project
            .rename("lna-frontend")
            .expect("valid project name");
        app.state.dialogs.project_review_dialog.show_close_project();
        app
    }

    /// Unsaved work asks the standard save-changes question on the narrow
    /// confirmation surface: the project in the title, the unsaved work in
    /// the body, and "Don't save" set apart from Cancel and Save.
    #[test]
    fn close_project_review_renders_on_the_narrow_confirmation_surface() {
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1_280.0, 800.0));
        let painted = painted_text(&render_close_review(
            &mut unsaved_project_closing(),
            screen.size(),
        ));
        let surface =
            egui::Rect::from_center_size(screen.center(), egui::vec2(480.0, screen.height()));
        let [dont_save, cancel, save] =
            ["Don't save", "Cancel", "Save"].map(|verb| find(&painted, verb));
        for rect in [
            find(&painted, "Save changes to \u{201c}lna-frontend\u{201d}?"),
            find(&painted, "1 document has unsaved changes."),
            find(&painted, "Project configuration"),
            dont_save,
            cancel,
            save,
        ] {
            assert!(surface.contains_rect(rect), "{rect:?} escapes {surface:?}");
        }
        for rect in [cancel, save] {
            assert!(
                (rect.center().y - dont_save.center().y).abs() < 1.0,
                "the verbs share one footer row"
            );
        }
        assert!(dont_save.right() < surface.center().x, "Don't save leads");
        assert!(
            surface.center().x < cancel.left() && cancel.right() < save.left(),
            "Cancel, then the primary, trail"
        );
        for retired in ["SAFE SHUTDOWN", "Local simulation", "Unsaved:"] {
            assert!(
                !painted.iter().any(|(text, _)| text.contains(retired)),
                "{retired:?} painted"
            );
        }
    }

    /// On a phone the verbs stack full-width, primary first.
    #[test]
    fn close_project_review_stacks_its_verbs_on_a_phone() {
        let painted = painted_text(&render_close_review(
            &mut unsaved_project_closing(),
            egui::vec2(390.0, 844.0),
        ));
        let [save, dont_save, cancel] =
            ["Save", "Don't save", "Cancel"].map(|verb| find(&painted, verb));
        assert!(
            save.center().y < dont_save.center().y && dont_save.center().y < cancel.center().y,
            "stacked primary, Don't save, Cancel: {save:?} {dont_save:?} {cancel:?}"
        );
        for rect in [save, dont_save, cancel] {
            assert!(
                (rect.center().x - 195.0).abs() < 1.5,
                "a full-width verb centres its label on the card: {rect:?}"
            );
        }
    }

    /// The body is one surface. Its frame used to repeat the surface fill
    /// over a rect as wide as the text, which showed as a band while the modal
    /// faded in; nothing between the header and the footer paints a fill now.
    #[test]
    fn close_project_review_body_paints_no_fill_band() {
        let output =
            render_close_review(&mut unsaved_project_closing(), egui::vec2(1_280.0, 800.0));
        let painted = painted_text(&output);
        let summary = find(&painted, "1 document has unsaved changes.");
        let last_row = find(&painted, "Project configuration");
        let body = egui::Rect::from_min_max(
            egui::pos2(0.0, summary.top() - 20.0),
            egui::pos2(1_280.0, last_row.bottom() + 20.0),
        );
        let mut bands = Vec::new();
        for clipped in &output.shapes {
            visit_shapes(&clipped.shape, &mut |shape| {
                if let egui::epaint::Shape::Rect(rect) = shape
                    && rect.fill != egui::Color32::TRANSPARENT
                    && rect.rect.width() > 24.0
                    && body.contains_rect(rect.rect)
                {
                    bands.push(rect.rect);
                }
            });
        }
        assert!(
            bands.is_empty(),
            "the body painted a fill of its own: {bands:?}"
        );
    }

    /// While a run owns the project the prompt asks for it to stop, and
    /// offers nothing that would save or discard.
    #[test]
    fn close_project_review_during_a_run_offers_stop_and_no_save_verbs() {
        let mut app = unsaved_project_closing();
        let run = app.state.simulation.start_run();
        run.mark_running().expect("fixture enters running state");
        let identity = run
            .execution_identity()
            .expect("running fixture has an identity");
        app.state.simulation.active_execution = Some(identity);

        let painted = painted_text(&render_close_review(&mut app, egui::vec2(1_280.0, 800.0)));
        let shows = |wanted: &str| painted.iter().any(|(text, _)| text == wanted);

        for expected in [
            "Stop the simulation to close this project",
            "Stop simulation",
            "Cancel",
            "Project configuration",
        ] {
            assert!(shows(expected), "{expected:?} missing: {painted:?}");
        }
        for verb in ["Save", "Save all", "Don't save"] {
            assert!(!shows(verb), "{verb:?} offered during a run");
        }
    }
}
