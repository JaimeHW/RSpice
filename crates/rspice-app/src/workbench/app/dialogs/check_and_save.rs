//! Mockup-owned transactional Check and save operation.

use egui::{Align, Align2, Context, Frame, Layout, Margin, Sense, Stroke, Ui, vec2};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionJournal,
    ValidatedRevisionRequest,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, Problem,
    paint_problem_mark,
};
use crate::workbench::workflows::project_workflow::{
    SaveRequestOutcome, save_active_for_continuation,
};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport;
use crate::workbench::app::dialogs::operation_primitives::{
    CONTEXT_WIDTH, SURFACE_HEIGHT, TRANSACTION_HEIGHT, ellipsized_text, operation_steps,
    paint_body_dividers,
};
use crate::workbench::app::dialogs::review_primitives::{
    input_field, purpose_line, read_only_field,
};
use crate::workbench::app::dialogs::schematic_command::{DISCARD_DETAIL, DISCARD_TITLE};
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::Command;

const TITLE: &str = "Check and save schematic";
const PRIMARY: &str = "Save validated revision";
const DESCRIPTION: &str =
    "Run connectivity, parameter, hierarchy, model, pin, and naming checks before explicit save.";
const SAVING_TITLE: &str = "Saving";
const SAVING_DETAIL: &str = "Keep this dialog open until the save completes.";
const NEWER_CHANGES_HINT: &str = "Edits made after validation are still unsaved.";
/// The form column: its 10 pt frame margins over three 47 pt fields with
/// 9 pt between them. It was 291 while the change preview sat under the
/// fields — a 29 pt heading over three 28 pt rows of constants — and holding
/// that number once the preview went would have left the band as bare
/// surface instead of as a table.
const CHECK_AND_SAVE_BODY_HEIGHT: f32 = 179.0;
/// Side of a leading problem mark, and the advance from its left edge to the
/// text behind it. Read from the kit rather than restated here, so a marked
/// line in this dialog sits where a marked line sits anywhere else, now and
/// after the kit moves it.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};
/// The first frame's height hint: the shared operation surface less the
/// 112 pt the change preview used to add to the body under it.
const CHECK_AND_SAVE_SURFACE_HEIGHT: f32 = SURFACE_HEIGHT - 51.0;

#[derive(Clone, Copy)]
struct CheckAndSavePreconditions<'a> {
    project: &'a str,
    revision: &'a str,
    source_current: bool,
    saved_snapshot: bool,
}

pub(crate) fn open_check_and_save_dialog(state: &mut AppState) {
    if !state.project_lifecycle.is_open() {
        state.prompt("Check and save needs an open project.");
        return;
    }
    // Check and save is greyed on a read-only project or schematic.
    if state.schematic.session.read_only
        || state.active_view_read_only()
        || state.workbench.safe_mode.project_read_only()
    {
        return;
    }
    state.sync_active_schematic_to_workspace();
    crate::workbench::menu_bar::run_design_rule_check(state);
    match CheckAndSaveValidationReport::collect(state) {
        Ok(report) => {
            // One line for the check. The dialog lists every finding, and
            // Problems lists the design-check ones where they can be visited.
            let blockers = report.blockers().len();
            let advisories = report.advisories().len();
            state.record(
                if blockers == 0 {
                    LogSeverity::Info
                } else {
                    LogSeverity::Warning
                },
                LogSource::Studio,
                format!(
                    "Check and save: {blockers} blocker{}, {advisories} advisor{}.",
                    if blockers == 1 { "" } else { "s" },
                    if advisories == 1 { "y" } else { "ies" }
                ),
            );
            state.dialogs.check_and_save.open(report);
        }
        Err(error) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Check and save could not complete its checks: {error}"),
            );
        }
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_check_and_save_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.check_and_save.open {
            return;
        }
        let Some(report) = self.state.dialogs.check_and_save.report.clone() else {
            self.state.dialogs.check_and_save.validation_error = Some(
                "Validation evidence is unavailable; close and reopen Check and save.".to_owned(),
            );
            return;
        };
        let project = self
            .state
            .workspace
            .content
            .project
            .display_name()
            .to_owned();
        let revision = report.project_revision().to_string();
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
        let saved = self.state.dialogs.check_and_save.saved;
        let saved_with_newer_changes = self.state.dialogs.check_and_save.saved_with_newer_changes;
        let validation_error = self.state.dialogs.check_and_save.validation_error.clone();
        let has_transaction = discard_confirm
            || pending
            || validation_error.is_some()
            || !report.blockers().is_empty();
        let primary_label = if saved { "Close" } else { PRIMARY };
        let primary_enabled = saved || (!pending && report.can_save() && note_error.is_none());
        let source_current = report.source_is_current(&self.state);
        let preconditions = CheckAndSavePreconditions {
            project: &project,
            revision: &revision,
            source_current,
            saved_snapshot: saved,
        };
        let mut dialog = Dialog::new(TITLE, primary_label)
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
            .ghost_enabled(!pending)
            .primary_enabled(primary_enabled)
            .initial_focus(DialogInitialFocus::BodyControl);
        if !saved {
            dialog = dialog.ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            });
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
        } else if saved {
            // The body already says the snapshot was saved and the console
            // names the revision. Only edits the snapshot missed need a word.
            if saved_with_newer_changes {
                dialog = dialog.hint(NEWER_CHANGES_HINT);
            }
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

        // The dialog is modal, so the author cannot go and select the stale
        // instance first. The row therefore names one and repairs it in place,
        // and the receipt is re-derived afterwards because it described the
        // design as it stood before the repair.
        let repair_instance = report
            .stale_interface_instances()
            .first()
            .filter(|_| !pending && !saved)
            .cloned();
        let mut note_changed = false;
        let mut repair_requested = false;
        let mut response = dialog.show_transaction(ctx, |ui| {
            let response = check_and_save_body(
                ui,
                &report,
                &mut self.state.dialogs.check_and_save.revision_note,
                displayed_note_error.as_deref(),
                pending || saved,
                preconditions,
                StaleInterfaceRepair {
                    instance: repair_instance.as_deref(),
                    requested: &mut repair_requested,
                },
            );
            note_changed = response.changed();
            Some(response.id)
        });
        if note_changed {
            self.state.dialogs.check_and_save.mark_edited();
        }
        if let Some(instance) = repair_instance.filter(|_| repair_requested) {
            self.repair_stale_instance_interface(&instance);
        }
        match response.choice {
            DialogChoice::Primary if saved => {
                self.state.dialogs.check_and_save.close();
            }
            DialogChoice::Primary => self.commit_check_and_save(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.check_and_save.attempt_close();
                if self.state.dialogs.check_and_save.open {
                    response.retain_cancel_focus(DialogInitialFocus::Ghost);
                }
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    /// Select the named stale instance and run the one repair operation.
    ///
    /// The operation reports its own outcome, and the frozen validation
    /// receipt is re-derived because it described the design before it ran.
    fn repair_stale_instance_interface(&mut self, instance: &str) {
        let Some(component_id) = self
            .state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name.eq_ignore_ascii_case(instance))
            .map(|component| component.id)
        else {
            self.state.prompt(format!(
                "Instance '{instance}' is no longer in the active schematic."
            ));
            return;
        };
        self.state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(component_id);
        let outcome = self.state.schematic.update_selected_instance_interface(
            &self.state.library_manager,
            &self.state.workspace.content.schematic_buffers,
        );
        self.state.prompt(match outcome {
            Ok(summary) => summary,
            Err(error) => error.to_string(),
        });
        match CheckAndSaveValidationReport::collect(&self.state) {
            Ok(report) => self.state.dialogs.check_and_save.report = Some(report),
            Err(error) => {
                self.state.dialogs.check_and_save.validation_error = Some(error);
            }
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

        let original_journal = self.state.schematic.document().validated_revisions.clone();
        let original_dirty = self.state.schematic.session.is_dirty;
        let target_view_key = fresh_report.active_view().key();
        if original_journal.is_empty()
            && let Some(accepted) =
                crate::workbench::lifecycle::project_lifecycle::accepted_active_schematic(
                    &self.state,
                )
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
        let (revision_id, original_journal, expected_journal, expected_design_digest) = match self
            .state
            .schematic
            .record_validated_save_revision(request, original_journal, original_dirty)
        {
            Ok(recorded) => recorded,
            Err(error) => {
                self.state.dialogs.check_and_save.validation_error = Some(error);
                return;
            }
        };
        self.state.sync_active_schematic_to_workspace();
        match save_active_for_continuation(&mut self.state) {
            SaveRequestOutcome::CanonicalComplete => {
                // Only a browser save that is still landing keeps the
                // revision's id; this one has landed.
                let _ = revision_id;
                complete_validated_save(&mut self.state, false);
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
                "The save did not finish; no validated revision was saved.",
            ),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::workbench) fn handle_check_and_save_continuation(
        &mut self,
        event: &rspice_project::lifecycle::SaveContinuationEvent,
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
            rspice_project::lifecycle::SaveContinuationEvent::Saved(_) => {
                if revision_id.is_some() {
                    complete_validated_save(&mut self.state, false);
                } else {
                    rollback_pending_validated_save(
                        &mut self.state,
                        "The browser save completed without a validated revision identity. Reopen the project before retrying.",
                    );
                }
            }
            rspice_project::lifecycle::SaveContinuationEvent::SavedWithNewerChanges(_) => {
                if revision_id.is_some() {
                    complete_validated_save(&mut self.state, true);
                } else {
                    rollback_pending_validated_save(
                        &mut self.state,
                        "The browser save completed without a validated revision identity. Reopen the project before retrying.",
                    );
                }
            }
            rspice_project::lifecycle::SaveContinuationEvent::Cancelled(_) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    "The browser save was cancelled. No validated revision was committed.",
                );
            }
            rspice_project::lifecycle::SaveContinuationEvent::Conflict(_) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    "The project changed outside RSpice. Reopen it or save a separate copy; no validated revision was saved.",
                );
            }
            rspice_project::lifecycle::SaveContinuationEvent::Failed(_, error) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    &format!("The validated revision could not be saved: {error}"),
                );
            }
            rspice_project::lifecycle::SaveContinuationEvent::PublishedButNotAdopted(_, error) => {
                rollback_pending_validated_save(
                    &mut self.state,
                    &format!(
                        "The project was written but RSpice could not reopen it: {error}. The destination may hold the revision; reopen it before retrying."
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

fn complete_validated_save(state: &mut AppState, newer_changes: bool) {
    clear_pending_save_state(state);
    state.dialogs.check_and_save.saved = true;
    state.dialogs.check_and_save.saved_with_newer_changes = newer_changes;
    state.dialogs.check_and_save.validation_error = None;
    state.dialogs.check_and_save.dirty = false;
    state.dialogs.check_and_save.discard_confirm = false;
    let message = match state.dialogs.check_and_save.report.as_ref() {
        Some(report) => format!(
            "Saved a validated revision of {}.",
            report.document_scope_label()
        ),
        None => "Saved a validated revision.".to_owned(),
    };
    state.record(LogSeverity::Info, LogSource::Studio, message);
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
    let active_key = state.workspace.content.active_schematic_reference().key();
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
            .schematic_editor_mut(target_view_key)
            .ok_or_else(|| "the validated source document is no longer open".to_owned())
            .and_then(|mut schematic| {
                rollback_exact_journal(
                    &mut schematic.editor,
                    &original_journal,
                    &expected_journal,
                    expected_design_digest,
                    original_dirty,
                )
            })
    };
    let detail = match rollback_result {
        Ok(()) => match crate::workbench::lifecycle::project_lifecycle::refresh_registry(state) {
            Ok(()) => message.to_owned(),
            Err(error) => format!(
                "{message} The revision was taken back, but the project's unsaved-changes state could not be refreshed: {error}. Reopen the project before retrying."
            ),
        },
        Err(error) => format!(
            "{message} The revision could not be taken back because {error}; close and reopen the project before retrying."
        ),
    };
    state.dialogs.check_and_save.validation_error = Some(detail.clone());
    state.record(LogSeverity::Warning, LogSource::Studio, detail);
}

fn rollback_exact_journal(
    schematic: &mut crate::state::SchematicState,
    original_journal: &ValidatedRevisionJournal,
    expected_journal: &ValidatedRevisionJournal,
    expected_design_digest: crate::product::ContentDigest,
    original_dirty: bool,
) -> Result<(), String> {
    schematic.rollback_validated_save_journal(
        original_journal,
        expected_journal,
        expected_design_digest,
        original_dirty,
    )
}

#[cfg(target_arch = "wasm32")]
fn rollback_pending_validated_save(state: &mut AppState, message: &str) {
    let dialog = &mut state.dialogs.check_and_save;
    let pending = (
        dialog.pending_view_key.take(),
        dialog.pending_original_journal.take(),
        dialog.pending_expected_journal.take(),
        dialog.pending_expected_design_digest.take(),
    );
    let (Some(view_key), Some(journal), Some(expected_journal), Some(expected_design_digest)) =
        pending
    else {
        clear_pending_save_state(state);
        let detail = format!(
            "{message} The revision could not be taken back because the save that held it is gone; close and reopen the project before retrying."
        );
        state.dialogs.check_and_save.validation_error = Some(detail.clone());
        state.record(LogSeverity::Warning, LogSource::Studio, detail);
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

/// The one stale instance the checks can repair in place, and the flag the
/// row sets when the author asks for it.
struct StaleInterfaceRepair<'a> {
    instance: Option<&'a str>,
    requested: &'a mut bool,
}

fn check_and_save_body(
    ui: &mut Ui,
    report: &CheckAndSaveValidationReport,
    note: &mut String,
    note_error: Option<&str>,
    read_only: bool,
    preconditions: CheckAndSavePreconditions<'_>,
    repair: StaleInterfaceRepair<'_>,
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
                check_and_save_form(ui, report, note, note_error, read_only, repair)
            },
        );
        ui.painter().hline(
            form.response.rect.x_range(),
            form.response.rect.bottom(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
        );
        // Stacked, the panel sits under the form rather than beside it, so
        // the form's height is not a measure it has to match: holding it to
        // one left the two facts of the snapshot over 94 points of bare
        // panel at 560.
        preconditions_panel(ui, 0.0, preconditions);
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
                        check_and_save_form(ui, report, note, note_error, read_only, repair)
                    },
                )
                .inner;
            ui.allocate_ui_with_layout(
                vec2(context_width, CHECK_AND_SAVE_BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(context_width, CHECK_AND_SAVE_BODY_HEIGHT));
                    preconditions_panel(ui, CHECK_AND_SAVE_BODY_HEIGHT, preconditions);
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
    repair: StaleInterfaceRepair<'_>,
) -> egui::Response {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
    let mut note_response = None;
    let StaleInterfaceRepair {
        instance: repair_instance,
        requested: repair_requested,
    } = repair;
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
            if let Some(instance) = repair_instance {
                ui.add_space(9.0);
                *repair_requested = stale_interface_repair_row(ui, instance);
            }
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
    note_response.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
}

/// The blocker the checks can clear without closing the dialog.
///
/// The button carries the command's own label, so the row and the Design
/// command it runs cannot drift into naming the same repair differently.
fn stale_interface_repair_row(ui: &mut Ui, instance: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut requested = false;
    Frame::NONE
        .fill(t.color.bg_elevated)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                requested = ui
                    .add(egui::Button::new(
                        Command::UpdateInstanceInterface.spec().label,
                    ))
                    .on_hover_text(format!(
                        "Select {instance} and re-bind it to the interface its master presents now"
                    ))
                    .clicked();
                let font = theme::sans(tokens::FS_0, FontWeight::Regular);
                let text = format!("{instance}: pins differ from master");
                let galley = ui
                    .painter()
                    .layout_no_wrap(text.clone(), font, t.color.text);
                let (rect, response) = ui.allocate_exact_size(
                    vec2(galley.size().x + MARK_ADVANCE, galley.size().y),
                    Sense::hover(),
                );
                // The blocker the mark states is painted text only, so it had
                // no node of its own and was never announced.
                let announcement = Problem::Failure.announce(&text);
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        &announcement,
                    )
                });
                paint_problem_mark(
                    ui.painter(),
                    egui::Rect::from_center_size(
                        egui::pos2(rect.left() + MARK_SIDE * 0.5, rect.center().y),
                        egui::Vec2::splat(MARK_SIDE),
                    ),
                    Problem::Failure,
                );
                ui.painter().galley(
                    rect.left_top() + vec2(MARK_ADVANCE, 0.0),
                    galley,
                    t.color.text,
                );
            });
        });
    requested
}

fn preconditions_panel(ui: &mut Ui, height: f32, preconditions: CheckAndSavePreconditions<'_>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(height);
        ui.spacing_mut().item_spacing.y = 0.0;
        section_heading(
            ui,
            if preconditions.saved_snapshot {
                "Saved snapshot"
            } else {
                "Validated snapshot"
            },
        );
        fact_row(ui, "Project", preconditions.project);
        fact_row(ui, "Revision", preconditions.revision);
        // The gates a blocker already states are not repeated here: the Checks
        // field counts them and the strip names the first one. Only an edit
        // made after the checks ran is invisible everywhere else, and it is
        // what makes the save refuse.
        if !preconditions.saved_snapshot && !preconditions.source_current {
            unmet_line(ui, "The design changed after these checks ran.");
        }
    });
}

/// The one precondition the rest of the dialog cannot state.
fn unmet_line(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 28.0), Sense::hover());
    paint_problem_mark(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 10.0 + MARK_SIDE * 0.5, rect.center().y),
            egui::Vec2::splat(MARK_SIDE),
        ),
        Problem::Warning,
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0 + MARK_ADVANCE, 0.0),
        Align2::LEFT_CENTER,
        text,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let announcement = Problem::Warning.announce(text);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
}

fn section_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    // `widget_info` is what gives the node its role and its bounds. The
    // builder on its own only creates a default node, and a node with no
    // bounds cannot be hit-tested, focused or routed to, so it is dropped
    // before it ever reaches assistive technology.
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label);
        // A `Role::Label`'s text is filed under `value`, so without this the
        // heading carries its own title twice and is announced twice.
        node.clear_value();
    });
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
    // The builder alone would leave a roleless, bounds-less node that no
    // assistive technology can reach, which is how both facts of the snapshot
    // went missing. `widget_info` supplies the role and the bounds, and the
    // builder then states the pair the row paints.
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    ui.ctx().accesskit_node_builder(response_id, |node| {
        node.set_label(label);
        node.set_value(value);
    });
    if truncated {
        response.on_hover_text(value);
    }
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
        assert_eq!(PRIMARY, "Save validated revision");
        assert_eq!(
            DESCRIPTION,
            "Run connectivity, parameter, hierarchy, model, pin, and naming checks before explicit save."
        );
    }

    /// Check and save states no status in colour.
    ///
    /// Colour here is for data identity, a failing verdict, an error
    /// attached to the input it rejects, and the selected item. A status
    /// dot, a success tone and an informational tone are none of those, and
    /// the kit's `Success` and `Neutral` marks are being withdrawn — a
    /// caller added here after they go would not compile, but one added
    /// before them is exactly how they survive.
    #[test]
    fn this_dialog_states_no_status_in_colour() {
        let source = crate::source_guard::production_source(include_str!("check_and_save.rs"));
        let found = [
            "colored_label",
            "status_dot(",
            "StatusMark::Success",
            "StatusMark::Neutral",
            ".color(t.color.ok",
            ".color(t.color.info",
        ]
        .into_iter()
        .filter(|pattern| source.contains(pattern))
        .collect::<Vec<_>>();
        assert!(
            found.is_empty(),
            "Check and save paints status in colour again: {found:?}"
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

    /// Everything Check and save paints must also reach assistive technology.
    ///
    /// The row helpers used to write their text straight into an AccessKit
    /// node builder with no `widget_info` call in front of it. That leaves a
    /// node with no role and, decisively, no bounds, and a bounds-less node is
    /// dropped by every consumer of the tree: the facts of the snapshot and
    /// the whole change preview were unreadable. Each case below therefore
    /// asserts the bounds as well as the text, because the text alone was
    /// always there.
    #[test]
    fn every_painted_row_is_published_with_bounds() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.schematic);
        open_check_and_save_dialog(&mut app.state);
        assert!(app.state.dialogs.check_and_save.open);

        let mut render = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1280.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| app.render_frame_dialogs(ui.ctx()),
            )
        };
        let _ = render();
        let nodes = render()
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        // Every case is collected rather than asserted on the spot, so one
        // run names every row that stopped publishing instead of only the
        // first.
        let mut unpublished = Vec::new();
        let mut published = |what: &str, matches: &dyn Fn(&egui::accesskit::Node) -> bool| {
            let found = nodes
                .iter()
                .map(|(_, node)| node)
                .find(|node| matches(node));
            match found {
                None => unpublished.push(format!("{what}: no node carries it")),
                Some(node) => match node.bounds() {
                    None => unpublished.push(format!("{what}: published without bounds")),
                    Some(bounds) if bounds.x1 <= bounds.x0 || bounds.y1 <= bounds.y0 => {
                        unpublished.push(format!("{what}: published with an empty rectangle"));
                    }
                    Some(_) => {}
                },
            }
        };

        // The section heading, named once: a `Role::Label`'s text is filed
        // under `value`, so a heading that still carries one is being
        // announced twice over.
        published("Validated snapshot", &|node| {
            node.role() == egui::accesskit::Role::Heading
                && node.label() == Some("Validated snapshot")
                && node.value().is_none()
        });

        // The two facts of the snapshot, each as the pair it paints.
        for fact in ["Project", "Revision"] {
            published(fact, &|node| {
                node.label() == Some(fact) && node.value().is_some_and(|value| !value.is_empty())
            });
        }

        assert!(
            unpublished.is_empty(),
            "Check and save paints rows that never reach the accessibility tree:\n  {}",
            unpublished.join("\n  ")
        );

        // The change preview is gone: four columns of the same two sentences
        // about what the save retains, at every width and for every design.
        // It said nothing a reader could act on and every cell ellipsized.
        let still_painted = nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .filter(|label| {
                label.contains("Exact change preview")
                    || label.contains("Owned object")
                    || label.contains("remain retained")
            })
            .collect::<Vec<_>>();
        assert!(
            still_painted.is_empty(),
            "the change preview is back: {still_painted:?}"
        );
    }

    /// Check and save as PNGs, wide and narrow, for a human to measure.
    ///
    /// `RSPICE_RASTER_DIR` names the directory; the system temp directory
    /// otherwise. The narrow pass is the one that shows whether the stacked
    /// layout still fits a 560 pt viewport.
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_check_and_save_for_review() {
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for size in [egui::vec2(1440.0, 900.0), egui::vec2(560.0, 900.0)] {
            let mut app = RSpiceApp::test_instance();
            app.state.ui.preferences.set_toggle(
                crate::workbench::preferences::TogglePreference::ReducedMotion,
                true,
            );
            app.state.project_lifecycle.authority.open_session();
            app.state
                .workbench
                .activate(crate::workbench::state::Workspace::Design);
            crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.schematic);
            open_check_and_save_dialog(&mut app.state);
            assert!(app.state.dialogs.check_and_save.open);
            let canvas = crate::ui::raster::render(size, |ui, _| {
                app.prepare_frame(ui.ctx());
                app.render_frame_chrome(ui);
                app.render_frame_dialogs(ui.ctx());
            });
            let name = format!("check-and-save-{}x{}", size.x as u32, size.y as u32);
            std::fs::write(
                directory.join(format!("{name}.png")),
                canvas.png(size.y as usize),
            )
            .expect("write the render");
        }
    }

    #[test]
    fn rollback_requires_the_exact_post_append_journal() {
        let mut schematic = crate::state::SchematicState::default();
        let original = schematic.document().validated_revisions.clone();
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
        let expected = schematic.document().validated_revisions.clone();
        let expected_design_digest = schematic
            .editor_ref()
            .design
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
        assert_eq!(schematic.document().validated_revisions, original);

        schematic
            .append_validated_revision(revision_request("Newer revision", 9))
            .expect("append newer revision");
        let retained = schematic.document().validated_revisions.clone();
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
        assert_eq!(schematic.document().validated_revisions, retained);

        let guarded_design = schematic
            .editor_ref()
            .design
            .validated_design_content_digest()
            .expect("guarded design digest");
        schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(10, 10),
        );
        rollback_exact_journal(&mut schematic, &original, &retained, guarded_design, false)
            .expect("journal rollback preserves a newer design edit");
        assert!(schematic.session.is_dirty);
        assert_eq!(schematic.document().validated_revisions, original);
    }
}
