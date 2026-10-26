//! Save-changes confirmation — the template instance of the RSpice modal
//! grammar: small surface, one sentence,
//! three explicit verbs. Ghost = Cancel, secondary = Don't save,
//! primary = Save.

use egui::{Context, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use crate::ui::panels::WorkbenchIcon;
use crate::workbench::app::dialogs::confirmation_state::{
    ConfirmationResponse, ProjectReviewRequest,
};
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::lifecycle::project_lifecycle::{
    ProjectCloseDestination, ProjectDocumentId, RevertReviewToken,
};

/// The project's side of the confirmations: what the host does once a
/// question is answered.
pub trait ConfirmationHost: WorkbenchHost {
    /// Carry out the pending action as the save-changes question was
    /// answered: save first, go ahead without saving, or stay.
    fn answer_confirmation(&mut self, response: ConfirmationResponse);

    /// Discard the active document's working changes, as the revert review
    /// `token` was issued for.
    fn revert_active_document(&mut self, token: &RevertReviewToken);

    /// Save the project, then close it to `destination`.
    fn close_project_after_save(&mut self, destination: ProjectCloseDestination);

    /// Close the project to `destination` without saving it.
    fn close_project_discarding(&mut self, destination: ProjectCloseDestination);

    /// What the browser file operation still waiting to finish is doing, if
    /// one is.
    #[cfg(target_arch = "wasm32")]
    fn pending_browser_file_operation(&self) -> Option<String>;

    /// Release the browser file operation still waiting to finish, so a late
    /// response can no longer change the session.
    #[cfg(target_arch = "wasm32")]
    fn cancel_browser_file_operation(&mut self);
}

pub fn render_confirmation_dialog(app: &mut impl ConfirmationHost, ctx: &Context) {
    if app.workbench().confirmation_dialog.visible {
        render_save_confirmation_dialog(app, ctx);
    }
    #[cfg(target_arch = "wasm32")]
    render_pending_browser_file_operation(app, ctx);
    render_project_review_dialog(app, ctx);
}

#[cfg(target_arch = "wasm32")]
fn render_pending_browser_file_operation(app: &mut impl ConfirmationHost, ctx: &Context) {
    if app.workbench().confirmation_dialog.visible {
        return;
    }
    let Some(message) = app.pending_browser_file_operation() else {
        return;
    };

    let choice = Dialog::new(
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
        app.cancel_browser_file_operation();
    }
}

fn render_save_confirmation_dialog(app: &mut impl ConfirmationHost, ctx: &Context) {
    let (title, consequence) = match app.workbench().confirmation_dialog.pending_action {
        Some(action) => (
            action.prompt_title(app.model().workspace.content.project.name().trim()),
            action.prompt_consequence(),
        ),
        None => (
            "Save changes?".to_owned(),
            "Your changes will be lost if you don't save them.",
        ),
    };
    let choice = Dialog::new(&title, "Save")
        .description(consequence)
        .size(DialogSize::Confirmation)
        .secondary("Don't save")
        .secondary_leading()
        .ghost("Cancel")
        .show(ctx, |ui| prompt_sentence(ui, None, consequence));

    let response = match choice {
        DialogChoice::Primary => Some(ConfirmationResponse::Yes),
        DialogChoice::Secondary => Some(ConfirmationResponse::No),
        DialogChoice::Ghost | DialogChoice::Cancelled => Some(ConfirmationResponse::Cancel),
        DialogChoice::None => None,
    };
    if let Some(r) = response {
        app.answer_confirmation(r);
    }
}

fn render_project_review_dialog(app: &mut impl ConfirmationHost, ctx: &Context) {
    let Some(request) = app.workbench().project_review_dialog.request.clone() else {
        return;
    };
    match request {
        ProjectReviewRequest::RevertActive(token) => {
            let document = token.document_label();
            let choice = Dialog::new(
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
                    app.workbench_mut().project_review_dialog.close();
                    app.revert_active_document(&token);
                }
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    app.workbench_mut().project_review_dialog.close();
                }
                DialogChoice::Secondary | DialogChoice::None => {}
            }
        }
        ProjectReviewRequest::CloseProject(destination) => {
            render_close_project_review(app, ctx, destination);
        }
    }
}

/// The close-project prompt. While a local run owns the project it asks
/// for the run to stop; otherwise it is the standard save-changes
/// question. A clean, idle project never opens it, and one that turns
/// clean and idle while it is open, because its run ended with nothing
/// unsaved, closes without asking again.
fn render_close_project_review(
    app: &mut impl ConfirmationHost,
    ctx: &Context,
    destination: ProjectCloseDestination,
) {
    let documents = crate::workbench::lifecycle::project_lifecycle::dirty_documents(app.session());
    // A never-saved project has no baseline to compare against, so all of
    // it is unsaved: the summary says that once, rather than listing the
    // configuration record that stands in for the whole project.
    let never_saved = app.platform().project_lifecycle.accepted().is_none();
    let mut unsaved: Vec<(WorkbenchIcon, String)> = if never_saved {
        Vec::new()
    } else {
        documents
            .iter()
            .map(|document| (document_icon(document), document.label()))
            .collect()
    };
    if app.model_draft_unsaved() {
        unsaved.push((WorkbenchIcon::Models, "Device model draft".to_owned()));
    }
    // A stimulus draft is runtime-only, so it is not an unsaved document
    // and saving the project would not publish it. Naming it is the only
    // honest thing to do: closing discards it, and nothing offered here
    // can apply a draft the audit strip may be refusing.
    for definition in app.unapplied_stimulus_drafts() {
        unsaved.push((
            WorkbenchIcon::Source,
            format!("Stimulus definition {definition}: draft not applied (discarded on close)"),
        ));
    }
    if app.model().simulation.execution.has_active_execution() {
        render_close_blocked_by_run(app, ctx, &unsaved);
    } else if documents.is_empty() && unsaved.is_empty() {
        app.workbench_mut().project_review_dialog.close();
        app.close_project_discarding(destination);
    } else {
        render_close_save_prompt(app, ctx, never_saved, &unsaved, destination);
    }
}

fn render_close_save_prompt(
    app: &mut impl ConfirmationHost,
    ctx: &Context,
    never_saved: bool,
    unsaved: &[(WorkbenchIcon, String)],
    destination: ProjectCloseDestination,
) {
    let project = app.model().workspace.content.project.name().trim();
    let title = if project.is_empty() {
        "Save changes before closing?".to_owned()
    } else {
        format!("Save changes to \u{201c}{project}\u{201d}?")
    };
    let summary = match (never_saved, unsaved.len()) {
        (true, _) => "This project has never been saved.".to_owned(),
        (false, 1) => "1 document has unsaved changes.".to_owned(),
        (false, count) => format!("{count} documents have unsaved changes."),
    };
    let choice = Dialog::new(
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
        prompt_sentence(ui, None, &summary);
        unsaved_list(ui, unsaved);
    });
    match choice {
        DialogChoice::Primary => app.close_project_after_save(destination),
        DialogChoice::Secondary => {
            app.workbench_mut().project_review_dialog.close();
            app.close_project_discarding(destination);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.workbench_mut().project_review_dialog.close();
        }
        DialogChoice::None => {}
    }
}

fn render_close_blocked_by_run(
    app: &mut impl WorkbenchHost,
    ctx: &Context,
    unsaved: &[(WorkbenchIcon, String)],
) {
    let stopping = app.model().simulation.cancellation_is_pending();
    let can_stop = app.model().simulation.can_request_abort_active_run();
    // With no stop to request, Cancel is the only verb, so it is the primary.
    let primary = if stopping {
        "Stopping\u{2026}"
    } else if can_stop {
        "Stop simulation"
    } else {
        "Cancel"
    };
    let mut dialog = Dialog::new("Stop the simulation to close this project", primary)
        .description("A local simulation has to stop before the project can close.")
        .size(DialogSize::Confirmation)
        .primary_enabled(!stopping);
    if stopping || can_stop {
        dialog = dialog.ghost("Cancel");
    }
    let choice = dialog.show(ctx, |ui| {
        prompt_sentence(
            ui,
            Some(WorkbenchIcon::Warning),
            "A local simulation is running. Stop it and wait for it to finish before closing.",
        );
        unsaved_list(ui, unsaved);
    });
    match choice {
        DialogChoice::Primary if can_stop => {
            if let Err(error) = app.model_mut().simulation.request_abort_active_run() {
                app.workbench_mut()
                    .prompt(format!("The run was not stopped: {error}."));
            }
        }
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.workbench_mut().project_review_dialog.close();
        }
        DialogChoice::Secondary | DialogChoice::None => {}
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
fn prompt_sentence(ui: &mut Ui, mark: Option<WorkbenchIcon>, sentence: &str) {
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
            let shown = crate::ui::panels::elide_text(ui, name, &font, row.width() - NAME_INDENT);
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
