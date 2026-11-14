//! Check and save's transactions over the whole application: the save it
//! validates and commits, its rollbacks, and the report it reads.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{AdvisoryDisposition, ValidatedRevisionJournal, ValidatedRevisionRequest};
use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::check_and_save::{
    NEWER_CHANGES, revision_note_error, rollback_exact_journal,
};
use crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport;
use crate::workbench::app::project_transactions::{WholeStateValidation, run_design_checks_in};
use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::project_lifecycle::SaveScope;
use crate::workbench::workflows::project_workflow::SaveRequestOutcome;

impl RSpiceApp {
    /// Save the validated revision the open dialog reviewed, once its report
    /// is re-derived as the reader reviewed it.
    // The shell runs this until the save runs over the lifecycle's parts.
    pub(crate) fn commit_check_and_save(&mut self) {
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
        run_design_checks_in(&mut self.state);
        let fresh_report = match CheckAndSaveValidationReport::collect_in(&self.state) {
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
        match self.state.save_project_scope(SaveScope::ActiveDocument) {
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

/// A save that landed closes the form, as any command does once it has run;
/// the record says what was saved, and whether edits made while the save was
/// landing missed it.
pub(crate) fn complete_validated_save(state: &mut AppState, newer_changes: bool) {
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
