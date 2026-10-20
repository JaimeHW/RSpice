//! Check and save: what the design checks found, the documents the save
//! covers, and the revision note it records.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    AdvisoryDisposition, MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionJournal,
    ValidatedRevisionRequest,
};
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::workflows::project_workflow::{
    SaveRequestOutcome, save_active_for_continuation,
};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::check_and_save_validation::{
    CheckAndSaveValidationReport, run_design_checks,
};
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::Command;

/// Retained state for the Check and save form.
/// Validation evidence is frozen when the dialog opens and re-derived before
/// publication; the editable revision note never mutates the live document
/// until a canonical save request has been accepted.
#[derive(Debug, Clone, Default)]
pub(crate) struct CheckAndSaveDialogState {
    pub(crate) open: bool,
    pub(crate) revision_note: String,
    pub(crate) revision_note_touched: bool,
    pub(crate) report: Option<
        crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport,
    >,
    pub(crate) validation_error: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_transaction: Option<crate::product::TransactionId>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_revision_id: Option<crate::state::ValidatedSchematicRevisionId>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_view_key: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_original_journal: Option<crate::state::ValidatedRevisionJournal>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_expected_journal: Option<crate::state::ValidatedRevisionJournal>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_expected_design_digest: Option<crate::product::ContentDigest>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_original_dirty: bool,
}

impl CheckAndSaveDialogState {
    pub(crate) fn open(
        &mut self,
        report: crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport,
    ) {
        *self = Self {
            open: true,
            report: Some(report),
            ..Self::default()
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        if self.is_pending() {
            return;
        }
        self.revision_note_touched = true;
        self.validation_error = None;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn is_pending(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            self.pending_transaction.is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }
}

const TITLE: &str = "Check and save schematic";
const PRIMARY: &str = "Save";
/// The sentence assistive technology reads for the whole form.
const DESCRIPTION: &str = "The design checks have run; save the project as a revision with a note.";
const SAVING: &str = "Saving. Keep this dialog open until the save completes.";
const STALE_CHECKS: &str = "The design changed after these checks ran. Save runs them again first.";
const NEWER_CHANGES: &str = "Edits made after validation are still unsaved.";
const SHOW_IN_PROBLEMS: &str = "Show in Problems";
const NOTE_FIELD: &str = "check-and-save-revision-note";

/// Everything the footer's status is decided from.
#[derive(Clone, Copy)]
struct CheckAndSaveStatus<'a> {
    pending: bool,
    save_error: Option<&'a str>,
    first_blocker: Option<&'a str>,
    note_error: Option<&'a str>,
    note_touched: bool,
    checks_current: bool,
}

/// What the footer says: a save in flight, why Save is refused, or, while
/// the note is still untouched, that it is needed, as a plain fact. A
/// refusal on a form nobody has typed in yet reads as a fault they made.
fn check_and_save_status(status: CheckAndSaveStatus<'_>) -> Result<Option<&str>, &str> {
    if status.pending {
        return Ok(Some(SAVING));
    }
    if let Some(error) = status.save_error.or(status.first_blocker) {
        return Err(error);
    }
    if let Some(error) = status.note_error {
        return if status.note_touched {
            Err(error)
        } else {
            Ok(Some(error))
        };
    }
    if !status.checks_current {
        return Ok(Some(STALE_CHECKS));
    }
    Ok(None)
}

/// The checks' verdict in words: "No blockers or advisories", or the two
/// counts.
fn checks_summary(report: &CheckAndSaveValidationReport) -> String {
    let blockers = report.blockers().len();
    let advisories = report.advisories().len();
    if blockers == 0 && advisories == 0 {
        return "No blockers or advisories".to_owned();
    }
    format!(
        "{blockers} blocker{}, {advisories} advisor{}",
        if blockers == 1 { "" } else { "s" },
        if advisories == 1 { "y" } else { "ies" }
    )
}

pub(crate) fn open_check_and_save_dialog(state: &mut AppState) {
    if !state.platform.project_lifecycle.is_open() {
        state.prompt("Check and save needs an open project.");
        return;
    }
    // Check and save is greyed on a read-only project or schematic.
    if state.model.schematic.session.read_only
        || state.active_view_read_only()
        || state.model.safe_mode.project_read_only()
    {
        return;
    }
    state.sync_active_schematic_to_workspace();
    run_design_checks(state);
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
            state.project.check_and_save.open(report);
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
        if !self.state.project.check_and_save.open {
            return;
        }
        let Some(report) = self.state.project.check_and_save.report.clone() else {
            self.state.project.check_and_save.validation_error = Some(
                "Validation results are unavailable; close and reopen Check and save.".to_owned(),
            );
            return;
        };
        let dialog = &self.state.project.check_and_save;
        let pending = dialog.is_pending();
        let note_error = revision_note_error(&dialog.revision_note);
        let save_error = dialog.validation_error.clone();
        let first_blocker = report.blockers().first().map(|item| item.message.clone());
        let status = check_and_save_status(CheckAndSaveStatus {
            pending,
            save_error: save_error.as_deref(),
            first_blocker: first_blocker.as_deref(),
            note_error: note_error.as_deref(),
            note_touched: dialog.revision_note_touched,
            checks_current: report.source_is_current(&self.state),
        });
        let primary_enabled = !pending && report.can_save() && note_error.is_none();
        let checks = checks_summary(&report);
        let has_findings = !report.blockers().is_empty() || !report.advisories().is_empty();
        // The dialog is modal, so the author cannot go and select the stale
        // instance first. The row therefore names one and repairs it in place,
        // and the receipt is re-derived afterwards because it described the
        // design as it stood before the repair.
        let repair_instance = report
            .stale_interface_instances()
            .first()
            .filter(|_| !pending)
            .cloned();
        let mut note_changed = false;
        let mut repair_requested = false;
        let mut show_problems = false;
        let note = egui::Id::new(NOTE_FIELD);
        let choice = CommandForm::new(TITLE, PRIMARY)
            .describe(DESCRIPTION)
            .primary_enabled(primary_enabled)
            .cancel_enabled(!pending)
            .fields_enabled(!pending)
            .status(status)
            .show(ctx, |rows| {
                if has_findings {
                    show_problems = rows.value_action("Checks", &checks, SHOW_IN_PROBLEMS);
                } else {
                    rows.value("Checks", &checks);
                }
                if let Some(instance) = repair_instance.as_deref() {
                    repair_requested = rows.value_action(
                        "Instance",
                        &format!("{instance} is out of date"),
                        Command::UpdateInstanceInterface.spec().label,
                    );
                }
                rows.value("Documents", report.document_scope_label());
                note_changed = rows
                    .text(
                        "Revision note",
                        note,
                        &mut self.state.project.check_and_save.revision_note,
                        "",
                    )
                    .changed();
                Some(note)
            });
        if note_changed {
            self.state.project.check_and_save.mark_edited();
        }
        if let Some(instance) = repair_instance.filter(|_| repair_requested) {
            self.repair_stale_instance_interface(&instance);
        }
        if show_problems {
            // The findings are visited on the Problems page, which the modal
            // would otherwise cover.
            self.state.project.check_and_save.close();
            self.state.show_problem_findings();
            return;
        }
        match choice {
            DialogChoice::Primary => self.commit_check_and_save(),
            // A save in flight cannot be abandoned half-done; Esc waits.
            DialogChoice::Ghost | DialogChoice::Cancelled if !pending => {
                self.state.project.check_and_save.close();
            }
            DialogChoice::Ghost
            | DialogChoice::Cancelled
            | DialogChoice::Secondary
            | DialogChoice::None => {}
        }
    }

    /// Select the named stale instance and run the one repair operation.
    ///
    /// The operation reports its own outcome, and the frozen validation
    /// receipt is re-derived because it described the design before it ran.
    fn repair_stale_instance_interface(&mut self, instance: &str) {
        let Some(component_id) = self
            .state
            .model
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
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(component_id);
        let outcome = self
            .state
            .model
            .schematic
            .update_selected_instance_interface(
                &self.state.model.library_manager,
                &self.state.model.workspace.content.schematic_buffers,
            );
        self.state.prompt(match outcome {
            Ok(summary) => summary,
            Err(error) => error.to_string(),
        });
        run_design_checks(&mut self.state);
        match CheckAndSaveValidationReport::collect(&self.state) {
            Ok(report) => self.state.project.check_and_save.report = Some(report),
            Err(error) => {
                self.state.project.check_and_save.validation_error = Some(error);
            }
        }
    }

    fn commit_check_and_save(&mut self) {
        // A cancelled destination, a transient publication failure, or a stale
        // validation receipt must remain retryable without forcing the user to
        // manufacture an edit to the revision note first.
        self.state.project.check_and_save.validation_error = None;
        let note = self
            .state
            .project
            .check_and_save
            .revision_note
            .trim()
            .to_owned();
        if let Some(error) = revision_note_error(&note) {
            self.state.project.check_and_save.validation_error = Some(error);
            return;
        }
        let Some(opened_report) = self.state.project.check_and_save.report.clone() else {
            self.state.project.check_and_save.validation_error =
                Some("Validation results are unavailable; nothing was saved.".to_owned());
            return;
        };
        // Checks nothing a check still describes, so an unchanged design is
        // re-derived from the evidence the reader reviewed.
        run_design_checks(&mut self.state);
        let fresh_report = match CheckAndSaveValidationReport::collect(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.project.check_and_save.validation_error = Some(format!(
                    "Validation could not be re-derived before save: {error}"
                ));
                return;
            }
        };
        if fresh_report.receipt_digest() != opened_report.receipt_digest() {
            self.state.project.check_and_save.report = Some(fresh_report);
            self.state.project.check_and_save.validation_error = Some(
                "The project changed after validation. Review the refreshed findings before saving."
                    .to_owned(),
            );
            return;
        }
        if !fresh_report.can_save() {
            self.state.project.check_and_save.validation_error =
                Some(fresh_report.blockers().first().map_or_else(
                    || "Validation blockers remain.".to_owned(),
                    |item| item.message.clone(),
                ));
            return;
        }

        let original_journal = self
            .state
            .model
            .schematic
            .document()
            .validated_revisions
            .clone();
        let original_dirty = self.state.model.schematic.session.is_dirty;
        let target_view_key = fresh_report.active_view().key();
        if original_journal.is_empty()
            && let Some(accepted) =
                crate::workbench::lifecycle::project_lifecycle::accepted_active_schematic(
                    &self.state.model,
                    &self.state.platform,
                )
            && let Err(error) = self.state.model.schematic.seed_accepted_revision_baseline(
                &accepted,
                fresh_report.project_id(),
                fresh_report.project_revision(),
                &fresh_report.active_view().key(),
            )
        {
            self.state.project.check_and_save.validation_error = Some(format!(
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
            .model
            .schematic
            .record_validated_save_revision(request, original_journal, original_dirty)
        {
            Ok(recorded) => recorded,
            Err(error) => {
                self.state.project.check_and_save.validation_error = Some(error);
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
                self.state.project.check_and_save.pending_transaction = Some(transaction);
                self.state.project.check_and_save.pending_revision_id = Some(revision_id);
                self.state.project.check_and_save.pending_view_key = Some(target_view_key);
                self.state.project.check_and_save.pending_original_journal = Some(original_journal);
                self.state.project.check_and_save.pending_expected_journal = Some(expected_journal);
                self.state
                    .project
                    .check_and_save
                    .pending_expected_design_digest = Some(expected_design_digest);
                self.state.project.check_and_save.pending_original_dirty = original_dirty;
                self.state.project.check_and_save.validation_error = None;
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
        let dialog = &self.state.project.check_and_save;
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

/// A save that landed closes the form, as any command does once it has run;
/// the record says what was saved, and whether edits made while the save was
/// landing missed it.
fn complete_validated_save(state: &mut AppState, newer_changes: bool) {
    clear_pending_save_state(state);
    let mut message = match state.project.check_and_save.report.as_ref() {
        Some(report) => format!(
            "Saved a validated revision of {}.",
            report.document_scope_label()
        ),
        None => "Saved a validated revision.".to_owned(),
    };
    if newer_changes {
        message.push(' ');
        message.push_str(NEWER_CHANGES);
    }
    state.project.check_and_save.close();
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
    let active_key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let rollback_result = if active_key == target_view_key {
        rollback_exact_journal(
            &mut state.model.schematic,
            &original_journal,
            &expected_journal,
            expected_design_digest,
            original_dirty,
        )
        .inspect(|()| state.sync_active_schematic_to_workspace())
    } else {
        state
            .model
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
    state.project.check_and_save.validation_error = Some(detail.clone());
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
    let dialog = &mut state.project.check_and_save;
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
        state.project.check_and_save.validation_error = Some(detail.clone());
        state.record(LogSeverity::Warning, LogSource::Studio, detail);
        return;
    };
    let dirty = state.project.check_and_save.pending_original_dirty;
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
        state.project.check_and_save.pending_transaction = None;
        state.project.check_and_save.pending_revision_id = None;
        state.project.check_and_save.pending_view_key = None;
        state.project.check_and_save.pending_original_journal = None;
        state.project.check_and_save.pending_expected_journal = None;
        state.project.check_and_save.pending_expected_design_digest = None;
        state.project.check_and_save.pending_original_dirty = false;
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = state;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ContentDigest;
    use crate::state::{ValidatedRevisionRequest, ValidationFindingCounts};
    use uuid::Uuid;

    fn status(
        note: &str,
        note_touched: bool,
        first_blocker: Option<&'static str>,
    ) -> Result<Option<&'static str>, String> {
        let note_error = revision_note_error(note);
        check_and_save_status(CheckAndSaveStatus {
            pending: false,
            save_error: None,
            first_blocker,
            note_error: note_error.as_deref(),
            note_touched,
            checks_current: true,
        })
        .map(|fact| fact.map(|_| "fact"))
        .map_err(str::to_owned)
    }

    /// Save is off until a note is typed, and the footer says why: as a
    /// plain fact on a form nobody has typed in, as a refusal once the note
    /// was typed and emptied again, and a blocker outranks both.
    #[test]
    fn a_disabled_save_says_why_without_blaming_an_untouched_form() {
        assert_eq!(status("", false, None), Ok(Some("fact")));
        assert!(status("", true, None).is_err());
        assert_eq!(status("Compensation update", true, None), Ok(None));
        assert_eq!(
            status("", false, Some("R1 is open")),
            Err("R1 is open".to_owned())
        );
    }

    #[test]
    fn revision_note_is_required_and_bounded() {
        assert!(revision_note_error(" ").is_some());
        assert!(revision_note_error("Validated compensation update").is_none());
        assert!(revision_note_error(&"x".repeat(MAX_VALIDATED_REVISION_NOTE_LEN + 1)).is_some());
    }

    /// A save that landed closes the form, and edits that missed it are
    /// said in the record rather than in a dialog left open to say it.
    #[test]
    fn a_landed_save_closes_the_form_and_records_what_it_missed() {
        let mut state = AppState::default();
        state.project.check_and_save.open = true;
        state.project.check_and_save.revision_note = "Compensation update".to_owned();
        complete_validated_save(&mut state, true);
        assert!(!state.project.check_and_save.open);
        assert!(state.project.check_and_save.revision_note.is_empty());
        let line = state
            .model
            .log_buffer
            .entries()
            .last()
            .expect("the save is recorded");
        assert!(line.message.starts_with("Saved a validated revision"));
        assert!(line.message.ends_with(NEWER_CHANGES), "{}", line.message);
    }

    /// Check and save is a command form: its rows on one label grid, and
    /// none of the wizard it was drawn as — four step tabs, a purpose line, a
    /// "Validated snapshot" side table — nor the sample revision note.
    #[test]
    fn check_and_save_is_a_command_form_without_the_wizard() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        rspice_app_state::workbench::examples::load_example(
            "RC Lowpass Filter",
            &mut app.state.model.schematic,
        );
        open_check_and_save_dialog(&mut app.state);
        assert!(app.state.project.check_and_save.open);

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

        // The two facts, each as the pair it paints, and the note field.
        for fact in ["Checks", "Documents"] {
            published(fact, &|node| {
                node.label() == Some(fact) && node.value().is_some_and(|value| !value.is_empty())
            });
        }
        published("Revision note", &|node| {
            node.label() == Some("Revision note")
        });

        assert!(
            unpublished.is_empty(),
            "Check and save paints rows that never reach the accessibility tree:\n  {}",
            unpublished.join("\n  ")
        );

        let wizard = nodes
            .iter()
            .flat_map(|(_, node)| [node.label(), node.value()])
            .flatten()
            .filter(|text| {
                text.contains("Scope")
                    || text.contains("Difference")
                    || text.contains("snapshot")
                    || text.contains("Update input-pair compensation")
                    || text.contains("before explicit save")
            })
            .map(str::to_owned)
            .collect::<Vec<_>>();
        assert!(wizard.is_empty(), "the wizard is back: {wizard:?}");
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
