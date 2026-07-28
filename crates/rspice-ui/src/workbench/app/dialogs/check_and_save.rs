//! Mockup-owned transactional Check and save operation.

use egui::{Align, Align2, Context, Frame, Layout, Margin, Sense, Stroke, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionJournal,
    ValidatedRevisionRequest, ValidatedSchematicRevisionId,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::project_workflow::{SaveRequestOutcome, save_active_for_continuation};

use crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport;
use crate::workbench::app::dialogs::operation_primitives::{
    CONTEXT_WIDTH, SURFACE_HEIGHT, TRANSACTION_HEIGHT, ellipsized_text, operation_steps,
    paint_body_dividers,
};
use crate::workbench::app::dialogs::review_primitives::{
    input_field, purpose_line, read_only_field,
};
use crate::workbench::app::dialogs::schematic_command::{DISCARD_DETAIL, DISCARD_TITLE};
use crate::workbench::app::{AppState, RSpiceApp};

const EYEBROW: &str = "SCHEMATIC \u{00b7} TRANSACTIONAL VALIDATION";
const TITLE: &str = "Check and save schematic";
const PRIMARY: &str = "Save validated revision";
const DESCRIPTION: &str =
    "Run connectivity, parameter, hierarchy, model, pin, and naming checks before explicit save.";
const SAVING_TITLE: &str = "Validated revision save in progress";
const SAVING_DETAIL: &str = "The browser owns the canonical save request. Keep this dialog open until publication completes.";
const CHECK_AND_SAVE_BODY_HEIGHT: f32 = 291.0;
const CHECK_AND_SAVE_SURFACE_HEIGHT: f32 = SURFACE_HEIGHT + 61.0;

#[derive(Clone, Copy)]
struct CheckAndSavePreconditions<'a> {
    project: &'a str,
    revision: &'a str,
    source_current: bool,
    authority_resolved: bool,
    dependencies_enumerated: bool,
}

pub(crate) fn open_check_and_save_dialog(state: &mut AppState) {
    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::warning(
            "Check and save requires an open project.".to_owned(),
        ));
        return;
    }
    if state.schematic.read_only
        || state.active_view_read_only()
        || state.workbench.safe_mode.project_read_only()
    {
        state.push_user_message(ConsoleMessage::warning(
            "Check and save is unavailable because the active project or schematic is read-only."
                .to_owned(),
        ));
        return;
    }
    state.sync_active_schematic_to_workspace();
    crate::workbench::menu_bar::run_design_rule_check(state);
    match CheckAndSaveValidationReport::collect(state) {
        Ok(report) => {
            let blockers = report.blockers().len();
            let advisories = report.advisories().len();
            state.push_user_message(if blockers == 0 {
                ConsoleMessage::info(format!(
                    "Check and save validation completed: 0 blockers, {advisories} advisories"
                ))
            } else {
                ConsoleMessage::warning(format!(
                    "Check and save validation found {blockers} blockers and {advisories} advisories"
                ))
            });
            for finding in report.blockers() {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "Check and save blocker · {} · {}",
                    finding.source, finding.message
                )));
            }
            for finding in report.advisories() {
                state.push_user_message(ConsoleMessage::info(format!(
                    "Check and save advisory · {} · {}",
                    finding.source, finding.message
                )));
            }
            state.dialogs.check_and_save.open(report);
        }
        Err(error) => state.push_user_message(ConsoleMessage::warning(format!(
            "Check and save validation could not be completed: {error}"
        ))),
    }
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_check_and_save_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.check_and_save.open {
            return;
        }
        let Some(report) = self.state.dialogs.check_and_save.report.clone() else {
            self.state.dialogs.check_and_save.validation_error = Some(
                "Validation evidence is unavailable; close and reopen Check and save.".to_owned(),
            );
            return;
        };
        let project = self.state.workspace.project.display_name().to_owned();
        let revision = self.state.workspace.project.revision().get().to_string();
        let note_error = revision_note_error(&self.state.dialogs.check_and_save.revision_note);
        let displayed_note_error = self
            .state
            .dialogs
            .check_and_save
            .revision_note_touched
            .then(|| note_error.clone())
            .flatten();
        let discard_confirm = self.state.dialogs.check_and_save.discard_confirm;
        let pending = self.state.dialogs.check_and_save.is_pending();
        let save_receipt = self.state.dialogs.check_and_save.save_receipt.clone();
        let validation_error = self.state.dialogs.check_and_save.validation_error.clone();
        let has_transaction = discard_confirm
            || pending
            || save_receipt.is_some()
            || validation_error.is_some()
            || !report.blockers().is_empty();
        let primary_label = if save_receipt.is_some() {
            "Close"
        } else {
            PRIMARY
        };
        let primary_enabled =
            save_receipt.is_some() || (!pending && report.can_save() && note_error.is_none());
        let source_current = report.source_is_current(&self.state);
        let authority_resolved = report.authority_is_resolved();
        let dependencies_enumerated = report.dependencies_are_enumerated();
        let preconditions = CheckAndSavePreconditions {
            project: &project,
            revision: &revision,
            source_current,
            authority_resolved,
            dependencies_enumerated,
        };
        let mut dialog = Dialog::new(EYEBROW, TITLE, primary_label)
            .description(DESCRIPTION)
            .size(DialogSize::SimulationWorkflow)
            .initial_height(
                CHECK_AND_SAVE_SURFACE_HEIGHT
                    + if has_transaction {
                        TRANSACTION_HEIGHT
                    } else {
                        0.0
                    },
            )
            .flush_body()
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .ghost_enabled(!pending)
            .primary_enabled(primary_enabled)
            .initial_focus(DialogInitialFocus::BodyControl);
        if self.state.dialogs.check_and_save.dirty && !discard_confirm {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if pending {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                SAVING_TITLE,
                SAVING_DETAIL,
            );
        } else if let Some(receipt) = save_receipt.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Complete,
                "Validated revision saved",
                receipt,
            );
        } else if let Some(error) = validation_error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Validated revision was not saved",
                error,
            );
        } else if let Some(blocker) = report.blockers().first() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Validation blockers must be resolved",
                &blocker.message,
            );
        }

        let mut note_changed = false;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let response = check_and_save_body(
                ui,
                &report,
                &mut self.state.dialogs.check_and_save.revision_note,
                displayed_note_error.as_deref(),
                pending || save_receipt.is_some(),
                preconditions,
            );
            note_changed = response.changed();
            Some(response.id)
        });
        if note_changed {
            self.state.dialogs.check_and_save.mark_edited();
        }
        match choice {
            DialogChoice::Primary if save_receipt.is_some() => {
                self.state.dialogs.check_and_save.close();
            }
            DialogChoice::Primary => self.commit_check_and_save(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.check_and_save.attempt_close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    fn commit_check_and_save(&mut self) {
        // A cancelled destination, a transient publication failure, or a stale
        // validation receipt must remain retryable without forcing the user to
        // manufacture an edit to the revision note first.
        self.state.dialogs.check_and_save.validation_error = None;
        let note = self
            .state
            .dialogs
            .check_and_save
            .revision_note
            .trim()
            .to_owned();
        if let Some(error) = revision_note_error(&note) {
            self.state.dialogs.check_and_save.validation_error = Some(error);
            return;
        }
        let Some(opened_report) = self.state.dialogs.check_and_save.report.clone() else {
            self.state.dialogs.check_and_save.validation_error =
                Some("Validation evidence is unavailable; nothing was saved.".to_owned());
            return;
        };
        let fresh_report = match CheckAndSaveValidationReport::collect(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.dialogs.check_and_save.validation_error = Some(format!(
                    "Validation could not be re-derived before save: {error}"
                ));
                return;
            }
        };
        if fresh_report.receipt_digest() != opened_report.receipt_digest() {
            self.state.dialogs.check_and_save.report = Some(fresh_report);
            self.state.dialogs.check_and_save.validation_error = Some(
                "The project changed after validation. Review the refreshed findings before saving."
                    .to_owned(),
            );
            return;
        }
        if !fresh_report.can_save() {
            self.state.dialogs.check_and_save.validation_error =
                Some(fresh_report.blockers().first().map_or_else(
                    || "Validation blockers remain.".to_owned(),
                    |item| item.message.clone(),
                ));
            return;
        }

        let original_journal = self.state.schematic.validated_revisions.clone();
        let original_dirty = self.state.schematic.is_dirty;
        let target_view_key = fresh_report.active_view().key();
        if original_journal.is_empty()
            && let Some(accepted) =
                crate::workbench::project_lifecycle::accepted_active_schematic(&self.state)
            && let Err(error) = self.state.schematic.seed_accepted_revision_baseline(
                &accepted,
                fresh_report.project_id(),
                fresh_report.project_revision(),
                &fresh_report.active_view().key(),
            )
        {
            self.state.dialogs.check_and_save.validation_error = Some(format!(
                "The accepted revision baseline could not be retained: {error}"
            ));
            return;
        }
        let dispositions = fresh_report
            .advisories()
            .iter()
            .map(|finding| AdvisoryDisposition::accepted(&finding.identity, &note))
            .collect::<Vec<_>>();
        let request = ValidatedRevisionRequest {
            project_id: fresh_report.project_id().to_owned(),
            project_revision: fresh_report.project_revision(),
            view_identity: fresh_report.active_view().key(),
            revision_note: note,
            author: "Local project editor".to_owned(),
            validation_receipt_digest: fresh_report.receipt_digest(),
            finding_counts: fresh_report.counts(),
            dependencies: fresh_report.dependencies().to_vec(),
            advisory_dispositions: dispositions,
        };
        let revision_id = match self.state.schematic.append_validated_revision(request) {
            Ok(id) => id,
            Err(error) => {
                self.state.schematic.validated_revisions = original_journal;
                self.state.schematic.is_dirty = original_dirty;
                self.state.dialogs.check_and_save.validation_error =
                    Some(format!("Validated revision could not be recorded: {error}"));
                return;
            }
        };
        let expected_journal = self.state.schematic.validated_revisions.clone();
        let expected_design_digest = match self.state.schematic.validated_design_content_digest() {
            Ok(digest) => digest,
            Err(error) => {
                self.state.schematic.validated_revisions = original_journal;
                self.state.schematic.is_dirty = original_dirty;
                self.state.dialogs.check_and_save.validation_error = Some(format!(
                    "The guarded working-design digest could not be recorded: {error}"
                ));
                return;
            }
        };
        self.state.sync_active_schematic_to_workspace();
        match save_active_for_continuation(&mut self.state) {
            SaveRequestOutcome::CanonicalComplete => {
                complete_validated_save(&mut self.state, revision_id, false);
            }
            #[cfg(target_arch = "wasm32")]
            SaveRequestOutcome::CanonicalPending(transaction) => {
                self.state.dialogs.check_and_save.pending_transaction = Some(transaction);
                self.state.dialogs.check_and_save.pending_revision_id = Some(revision_id);
                self.state.dialogs.check_and_save.pending_view_key = Some(target_view_key);
                self.state.dialogs.check_and_save.pending_original_journal = Some(original_journal);
                self.state.dialogs.check_and_save.pending_expected_journal = Some(expected_journal);
                self.state
                    .dialogs
                    .check_and_save
                    .pending_expected_design_digest = Some(expected_design_digest);
                self.state.dialogs.check_and_save.pending_original_dirty = original_dirty;
                self.state.dialogs.check_and_save.validation_error = None;
            }
            SaveRequestOutcome::Cancelled => rollback_validated_save(
                &mut self.state,
                &target_view_key,
                original_journal,
                expected_journal,
                expected_design_digest,
                original_dirty,
                "The save destination was cancelled. No validated revision was committed.",
            ),
            SaveRequestOutcome::Failed(error) => rollback_validated_save(
                &mut self.state,
                &target_view_key,
                original_journal,
                expected_journal,
                expected_design_digest,
                original_dirty,
                &format!("The validated revision could not be saved: {error}"),
            ),
            SaveRequestOutcome::CopyOnly | SaveRequestOutcome::CopyPending => {
                rollback_validated_save(
                    &mut self.state,
                    &target_view_key,
                    original_journal,
                    expected_journal,
                    expected_design_digest,
                    original_dirty,
                    "Only a project copy was produced; the active validated revision was not committed.",
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            SaveRequestOutcome::CanonicalPending(_) => rollback_validated_save(
                &mut self.state,
                &target_view_key,
                original_journal,
                expected_journal,
                expected_design_digest,
                original_dirty,
                "The native save did not complete synchronously; no validated revision was committed.",
            ),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::workbench::app) fn handle_check_and_save_continuation(
        &mut self,
        event: &crate::workbench::project_workflow::SaveContinuationEvent,
    ) -> bool {
        let dialog = &self.state.dialogs.check_and_save;
        let Some(expected) = dialog.pending_transaction else {
            return false;
        };
        if event.transaction() != expected {
            return false;
        }
        let revision_id = dialog.pending_revision_id;
        match event {
            crate::workbench::project_workflow::SaveContinuationEvent::Saved(_) => {
                if let Some(id) = revision_id {
                    complete_validated_save(&mut self.state, id, false);
                } else {
                    rollback_pending_validated_save(
                        &mut self.state,
                        "The browser save completed without a validated revision identity. Reopen the project before retrying.",
                    );
                }
            }
            crate::workbench::project_workflow::SaveContinuationEvent::SavedWithNewerChanges(_) => {
                if let Some(id) = revision_id {
                    complete_validated_save(&mut self.state, id, true);
                } else {
                    rollback_pending_validated_save(
                        &mut self.state,
                        "The browser save completed without a validated revision identity. Reopen the project before retrying.",
                    );
                }
            }
            crate::workbench::project_workflow::SaveContinuationEvent::Cancelled(_) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    "The browser save was cancelled. No validated revision was committed.",
                );
            }
            crate::workbench::project_workflow::SaveContinuationEvent::Conflict(_) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    "The canonical browser project changed outside RSpice. Reopen it or save an independent copy; no validated revision was committed locally.",
                );
            }
            crate::workbench::project_workflow::SaveContinuationEvent::Failed(_, error) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    &format!("The validated revision could not be saved: {error}"),
                );
            }
            crate::workbench::project_workflow::SaveContinuationEvent::PublishedButNotAdopted(
                _,
                error,
            ) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    &format!(
                        "RSpice could not adopt the published bytes as the canonical project: {error}. The destination may contain the revision; reopen it before retrying."
                    ),
                );
            }
        }
        true
    }
}

fn revision_note_error(note: &str) -> Option<String> {
    let trimmed = note.trim();
    if trimmed.is_empty() {
        return Some("Enter a revision note before saving.".to_owned());
    }
    if trimmed.chars().count() > MAX_VALIDATED_REVISION_NOTE_LEN {
        return Some(format!(
            "Revision note must be {MAX_VALIDATED_REVISION_NOTE_LEN} characters or fewer."
        ));
    }
    None
}

fn complete_validated_save(
    state: &mut AppState,
    revision_id: ValidatedSchematicRevisionId,
    newer_changes: bool,
) {
    clear_pending_save_state(state);
    state.dialogs.check_and_save.saved_with_newer_changes = newer_changes;
    state.dialogs.check_and_save.save_receipt = Some(if newer_changes {
        format!(
            "Revision {} was saved from the validated snapshot. Newer working changes remain unsaved.",
            revision_id.as_uuid()
        )
    } else {
        format!(
            "Revision {} was saved with its validation receipt and restorable design snapshot.",
            revision_id.as_uuid()
        )
    });
    state.dialogs.check_and_save.validation_error = None;
    state.dialogs.check_and_save.dirty = false;
    state.dialogs.check_and_save.discard_confirm = false;
    state.push_user_message(ConsoleMessage::info(format!(
        "Saved validated schematic revision {}",
        revision_id.as_uuid()
    )));
}

fn rollback_validated_save(
    state: &mut AppState,
    target_view_key: &str,
    original_journal: ValidatedRevisionJournal,
    expected_journal: ValidatedRevisionJournal,
    expected_design_digest: crate::product::ContentDigest,
    original_dirty: bool,
    message: &str,
) {
    clear_pending_save_state(state);
    let active_key = state.workspace.active_schematic_reference().key();
    let rollback_result = if active_key == target_view_key {
        rollback_exact_journal(
            &mut state.schematic,
            &original_journal,
            &expected_journal,
            expected_design_digest,
            original_dirty,
        )
        .inspect(|()| state.sync_active_schematic_to_workspace())
    } else {
        state
            .workspace
            .schematic_buffers
            .get_mut(target_view_key)
            .ok_or_else(|| "the validated source document is no longer open".to_owned())
            .and_then(|schematic| {
                rollback_exact_journal(
                    schematic,
                    &original_journal,
                    &expected_journal,
                    expected_design_digest,
                    original_dirty,
                )
            })
    };
    let detail = match rollback_result {
        Ok(()) => match crate::workbench::project_lifecycle::refresh_registry(state) {
            Ok(()) => message.to_owned(),
            Err(error) => format!(
                "{message} The journal rollback completed, but the project dirty-state registry could not be refreshed: {error}. Reopen the project before retrying."
            ),
        },
        Err(error) => format!(
            "{message} The local journal was left unchanged because {error}; close and reopen the project before retrying."
        ),
    };
    state.dialogs.check_and_save.validation_error = Some(detail.clone());
    state.push_user_message(ConsoleMessage::warning(detail));
}

fn rollback_exact_journal(
    schematic: &mut crate::state::SchematicState,
    original_journal: &ValidatedRevisionJournal,
    expected_journal: &ValidatedRevisionJournal,
    expected_design_digest: crate::product::ContentDigest,
    original_dirty: bool,
) -> Result<(), String> {
    if &schematic.validated_revisions != expected_journal {
        return Err("its validated revision history changed after publication began".to_owned());
    }
    let current_design_digest = schematic
        .validated_design_content_digest()
        .map_err(|error| format!("its working-design guard could not be verified: {error}"))?;
    schematic.validated_revisions = original_journal.clone();
    schematic.is_dirty = original_dirty || current_design_digest != expected_design_digest;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn rollback_pending_validated_save(state: &mut AppState, message: &str) {
    let Some(view_key) = state.dialogs.check_and_save.pending_view_key.take() else {
        clear_pending_save_state(state);
        let detail = format!(
            "{message} The validated source identity is unavailable; close and reopen the project before retrying."
        );
        state.dialogs.check_and_save.validation_error = Some(detail.clone());
        state.push_user_message(ConsoleMessage::warning(detail));
        return;
    };
    let Some(journal) = state.dialogs.check_and_save.pending_original_journal.take() else {
        clear_pending_save_state(state);
        let detail = format!(
            "{message} The local rollback journal is unavailable; close and reopen the project before retrying."
        );
        state.dialogs.check_and_save.validation_error = Some(detail.clone());
        state.push_user_message(ConsoleMessage::warning(detail));
        return;
    };
    let Some(expected_journal) = state.dialogs.check_and_save.pending_expected_journal.take()
    else {
        clear_pending_save_state(state);
        let detail = format!(
            "{message} The publication journal guard is unavailable; close and reopen the project before retrying."
        );
        state.dialogs.check_and_save.validation_error = Some(detail.clone());
        state.push_user_message(ConsoleMessage::warning(detail));
        return;
    };
    let Some(expected_design_digest) = state
        .dialogs
        .check_and_save
        .pending_expected_design_digest
        .take()
    else {
        clear_pending_save_state(state);
        let detail = format!(
            "{message} The working-design guard is unavailable; close and reopen the project before retrying."
        );
        state.dialogs.check_and_save.validation_error = Some(detail.clone());
        state.push_user_message(ConsoleMessage::warning(detail));
        return;
    };
    let dirty = state.dialogs.check_and_save.pending_original_dirty;
    rollback_validated_save(
        state,
        &view_key,
        journal,
        expected_journal,
        expected_design_digest,
        dirty,
        message,
    );
}

fn clear_pending_save_state(state: &mut AppState) {
    #[cfg(target_arch = "wasm32")]
    {
        state.dialogs.check_and_save.pending_transaction = None;
        state.dialogs.check_and_save.pending_revision_id = None;
        state.dialogs.check_and_save.pending_view_key = None;
        state.dialogs.check_and_save.pending_original_journal = None;
        state.dialogs.check_and_save.pending_expected_journal = None;
        state.dialogs.check_and_save.pending_expected_design_digest = None;
        state.dialogs.check_and_save.pending_original_dirty = false;
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = state;
}

fn check_and_save_body(
    ui: &mut Ui,
    report: &CheckAndSaveValidationReport,
    note: &mut String,
    note_error: Option<&str>,
    read_only: bool,
    preconditions: CheckAndSavePreconditions<'_>,
) -> egui::Response {
    purpose_line(ui, DESCRIPTION);
    operation_steps(ui);
    let width = ui.available_width();
    let stacked = check_and_save_uses_stacked_layout(ui.ctx().content_rect().width());
    let context_width = CONTEXT_WIDTH.min((width * 0.42).max(220.0));
    let form_width = if stacked {
        width
    } else {
        (width - context_width).max(1.0)
    };
    if stacked {
        let form = ui.allocate_ui_with_layout(
            vec2(form_width, CHECK_AND_SAVE_BODY_HEIGHT),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_min_size(vec2(form_width, CHECK_AND_SAVE_BODY_HEIGHT));
                check_and_save_form(ui, report, note, note_error, read_only)
            },
        );
        ui.painter().hline(
            form.response.rect.x_range(),
            form.response.rect.bottom(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
        );
        preconditions_panel(ui, report, CHECK_AND_SAVE_BODY_HEIGHT, preconditions);
        return form.inner;
    }
    let body = ui.allocate_ui_with_layout(
        vec2(width, CHECK_AND_SAVE_BODY_HEIGHT),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let response = ui
                .allocate_ui_with_layout(
                    vec2(form_width, CHECK_AND_SAVE_BODY_HEIGHT),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_min_size(vec2(form_width, CHECK_AND_SAVE_BODY_HEIGHT));
                        check_and_save_form(ui, report, note, note_error, read_only)
                    },
                )
                .inner;
            ui.allocate_ui_with_layout(
                vec2(context_width, CHECK_AND_SAVE_BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(context_width, CHECK_AND_SAVE_BODY_HEIGHT));
                    preconditions_panel(ui, report, CHECK_AND_SAVE_BODY_HEIGHT, preconditions);
                },
            );
            response
        },
    );
    paint_body_dividers(ui, body.response.rect, form_width);
    body.inner
}

const fn check_and_save_uses_stacked_layout(viewport_width: f32) -> bool {
    viewport_width <= 760.0
}

fn check_and_save_form(
    ui: &mut Ui,
    report: &CheckAndSaveValidationReport,
    note: &mut String,
    note_error: Option<&str>,
    read_only: bool,
) -> egui::Response {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let mut note_response = None;
    Frame::NONE
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            let field_width = ui.available_width();
            ui.allocate_ui(vec2(field_width, 47.0), |ui| {
                let response = read_only_field(
                    ui,
                    "Checks",
                    &report.checks_label(),
                    &checks_review_description(report),
                );
                let tooltip = checks_review_description(report);
                if !tooltip.is_empty() {
                    response.on_hover_text(tooltip);
                }
            });
            ui.add_space(9.0);
            ui.allocate_ui(vec2(field_width, 47.0), |ui| {
                read_only_field(
                    ui,
                    "Documents",
                    report.document_scope_label(),
                    "The exact canonical publication scope",
                );
            });
            ui.add_space(9.0);
            ui.allocate_ui(vec2(field_width, 47.0), |ui| {
                note_response = Some(
                    ui
                    .add_enabled_ui(!read_only, |ui| {
                        input_field(
                            ui,
                            "Revision note",
                            note,
                            "Update input-pair compensation",
                            note_error,
                            "Required revision history note; saving records this note as the disposition for every disclosed advisory",
                        )
                    })
                    .inner,
                );
            });
        });
    section_heading(ui, "Exact change preview", None);
    preview_table(ui, report);
    note_response.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
}

fn preconditions_panel(
    ui: &mut Ui,
    report: &CheckAndSaveValidationReport,
    height: f32,
    preconditions: CheckAndSavePreconditions<'_>,
) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(height);
        ui.spacing_mut().item_spacing.y = 0.0;
        let passed = [
            preconditions.source_current,
            preconditions.authority_resolved,
            preconditions.dependencies_enumerated,
        ]
        .into_iter()
        .filter(|passed| *passed)
        .count();
        let status = format!("{passed} / 3");
        section_heading(ui, "Preconditions", Some((&status, passed == 3)));
        fact_row(ui, "Project", preconditions.project);
        fact_row(ui, "Revision", preconditions.revision);
        fact_row(ui, "Change boundary", "owned source + dependency graph");
        checklist_row(ui, "Source revision current", preconditions.source_current);
        checklist_row(ui, "Permissions resolved", preconditions.authority_resolved);
        checklist_row(
            ui,
            "Dependencies enumerated",
            preconditions.dependencies_enumerated,
        );
        checklist_row(
            ui,
            "Commit produces an auditable result",
            preconditions.source_current
                && preconditions.authority_resolved
                && preconditions.dependencies_enumerated
                && report.can_save(),
        );
    });
}

fn section_heading(ui: &mut Ui, label: &str, status: Option<(&str, bool)>) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label.to_ascii_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    if let Some((status, passed)) = status {
        let font = theme::mono(tokens::FS_0, FontWeight::Medium);
        let text_width = ui
            .painter()
            .layout_no_wrap(status.to_owned(), font.clone(), t.color.ok)
            .size()
            .x;
        ui.painter().circle_filled(
            rect.right_center() - vec2(10.0 + text_width + 7.5, 0.0),
            2.5,
            if passed { t.color.ok } else { t.color.err },
        );
        ui.painter().text(
            rect.right_center() - vec2(10.0, 0.0),
            Align2::RIGHT_CENTER,
            status,
            font,
            if passed { t.color.ok } else { t.color.err },
        );
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label);
    });
}

fn preview_table(ui: &mut Ui, report: &CheckAndSaveValidationReport) {
    let headers = [
        "Owned object",
        "Reviewed operation",
        "Dependent effect",
        "Recovery",
    ];
    let rows = [
        [
            "active schematic working document".to_owned(),
            "validate then save explicit revision".to_owned(),
            "prior saved revision and results remain retained".to_owned(),
            "revert new working revision".to_owned(),
        ],
        [
            "Source revision".to_owned(),
            format!(
                "revision {} \u{2192} new operation identity",
                report.project_revision()
            ),
            "stable IDs and provenance retained".to_owned(),
            "prior state remains addressable".to_owned(),
        ],
    ];
    table_row(ui, &headers, true);
    for row in rows {
        let values = row.iter().map(String::as_str).collect::<Vec<_>>();
        table_row(ui, &values, false);
    }
}

fn table_row(ui: &mut Ui, values: &[&str], header: bool) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 28.0 };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(
        rect,
        0.0,
        if header {
            t.color.bg_elevated
        } else {
            t.color.bg_app
        },
    );
    let width = rect.width() / values.len().max(1) as f32;
    let mut truncated = false;
    for (index, value) in values.iter().enumerate() {
        let left = rect.left() + index as f32 * width;
        let clip = egui::Rect::from_min_max(
            egui::pos2(left + 8.0, rect.top()),
            egui::pos2((left + width - 6.0).min(rect.right()), rect.bottom()),
        );
        let font = theme::sans(
            tokens::FS_0,
            if header {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        );
        let display_source = if header {
            value.to_ascii_uppercase()
        } else {
            (*value).to_owned()
        };
        let (display_value, was_truncated) =
            ellipsized_text(ui, &display_source, clip.width(), &font, t.color.text_dim);
        truncated |= was_truncated;
        ui.painter().with_clip_rect(clip).text(
            clip.left_center(),
            Align2::LEFT_CENTER,
            display_value,
            font,
            t.color.text_dim,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(values.join("; "));
    });
    if truncated {
        response.on_hover_text(values.join(" · "));
    }
}

fn checks_review_description(report: &CheckAndSaveValidationReport) -> String {
    let mut detail = String::from(
        "Blocking and advisory findings from the frozen validation receipt. Saving records the revision note as the explicit disposition for each advisory.",
    );
    for finding in report.blockers().iter().chain(report.advisories()) {
        detail.push('\n');
        detail.push_str(match finding.level {
            crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveFindingLevel::Blocker => "BLOCKER",
            crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveFindingLevel::Advisory => "ADVISORY",
        });
        detail.push_str(" · ");
        detail.push_str(&finding.identity);
        detail.push_str(" · ");
        detail.push_str(&finding.message);
    }
    detail
}

fn fact_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 28.0), Sense::hover());
    let response_id = response.id;
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let columns_width = (rect.width() - 28.0).max(0.0);
    let label_width = (columns_width * (0.8 / 2.2)).max(76.0).min(columns_width);
    let clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 10.0 + label_width + 8.0, rect.top()),
        egui::pos2(rect.right() - 10.0, rect.bottom()),
    );
    let value_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let (display_value, truncated) =
        ellipsized_text(ui, value, clip.width(), &value_font, t.color.text);
    ui.painter().with_clip_rect(clip).text(
        clip.left_center(),
        Align2::LEFT_CENTER,
        display_value,
        value_font,
        t.color.text,
    );
    ui.ctx().accesskit_node_builder(response_id, |node| {
        node.set_label(label);
        node.set_value(value);
    });
    if truncated {
        response.on_hover_text(value);
    }
}

fn checklist_row(ui: &mut Ui, label: &str, passed: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 23.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().circle_filled(
        rect.left_center() + vec2(11.0, 0.0),
        2.0,
        if passed { t.color.ok } else { t.color.err },
    );
    ui.painter().text(
        rect.left_center() + vec2(20.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if passed {
            t.color.text_dim
        } else {
            t.color.err
        },
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_value(if passed { "pass" } else { "blocked" });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ContentDigest;
    use crate::state::{ValidatedRevisionRequest, ValidationFindingCounts};
    use uuid::Uuid;

    #[test]
    fn mockup_contract_strings_are_exact() {
        assert_eq!(TITLE, "Check and save schematic");
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} TRANSACTIONAL VALIDATION");
        assert_eq!(PRIMARY, "Save validated revision");
        assert_eq!(
            DESCRIPTION,
            "Run connectivity, parameter, hierarchy, model, pin, and naming checks before explicit save."
        );
    }

    #[test]
    fn revision_note_is_required_and_bounded() {
        assert!(revision_note_error(" ").is_some());
        assert!(revision_note_error("Validated compensation update").is_none());
        assert!(revision_note_error(&"x".repeat(MAX_VALIDATED_REVISION_NOTE_LEN + 1)).is_some());
    }

    #[test]
    fn operation_layout_stacks_only_at_the_mockup_breakpoint() {
        assert!(!check_and_save_uses_stacked_layout(761.0));
        assert!(check_and_save_uses_stacked_layout(760.0));
    }

    #[test]
    fn rollback_requires_the_exact_post_append_journal() {
        let mut schematic = crate::state::SchematicState::default();
        let original = schematic.validated_revisions.clone();
        let revision_request = |note: &str, marker: u8| ValidatedRevisionRequest {
            project_id: Uuid::new_v4().to_string(),
            project_revision: u64::from(marker),
            view_identity: "user/top/schematic".to_owned(),
            revision_note: note.to_owned(),
            author: "Test editor".to_owned(),
            validation_receipt_digest: ContentDigest::from_bytes([marker; 32]),
            finding_counts: ValidationFindingCounts::default(),
            dependencies: Vec::new(),
            advisory_dispositions: Vec::new(),
        };
        schematic
            .append_validated_revision(revision_request("Validated source", 7))
            .expect("append guarded revision");
        let expected = schematic.validated_revisions.clone();
        let expected_design_digest = schematic
            .validated_design_content_digest()
            .expect("design digest");

        rollback_exact_journal(
            &mut schematic,
            &original,
            &expected,
            expected_design_digest,
            false,
        )
        .expect("exact journal rolls back");
        assert_eq!(schematic.validated_revisions, original);

        schematic
            .append_validated_revision(revision_request("Newer revision", 9))
            .expect("append newer revision");
        let retained = schematic.validated_revisions.clone();
        assert!(
            rollback_exact_journal(
                &mut schematic,
                &original,
                &expected,
                expected_design_digest,
                false,
            )
            .is_err()
        );
        assert_eq!(schematic.validated_revisions, retained);

        let guarded_design = schematic
            .validated_design_content_digest()
            .expect("guarded design digest");
        schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(10, 10),
        );
        rollback_exact_journal(&mut schematic, &original, &retained, guarded_design, false)
            .expect("journal rollback preserves a newer design edit");
        assert!(schematic.is_dirty);
        assert_eq!(schematic.validated_revisions, original);
    }
}
