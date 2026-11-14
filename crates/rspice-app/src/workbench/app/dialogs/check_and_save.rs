//! Check and save: what the design checks found, the documents the save
//! covers, and the revision note it records.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{MAX_VALIDATED_REVISION_NOTE_LEN, ValidatedRevisionJournal};
use crate::ui::widgets::{CommandForm, DialogChoice};

use crate::workbench::app::dialogs::check_and_save_validation::{
    CheckAndSaveValidationReport, ValidationReads, run_design_checks,
};
use crate::workbench::app_state::{Model, PlatformState, SlotFacts};
use crate::workbench::framework::command_registry::{Registry, ids};
use crate::workbench::framework::feature::{CodePort, DesignPort};
use crate::workbench::lifecycle::project_lifecycle::ResultPresentationSource;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::state::WorkbenchSession;

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
pub(crate) const NEWER_CHANGES: &str = "Edits made after validation are still unsaved.";
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

/// What Check and save reads and writes: its dialog, the model and the
/// session it prompts on, the platform services' state, what the model's
/// design checks read of the features, and what its report reads of the
/// results workspace, design and code.
pub(crate) struct CheckAndSaveParts<'a> {
    pub(crate) dialog: &'a mut CheckAndSaveDialogState,
    pub(crate) model: &'a mut Model,
    pub(crate) platform: &'a PlatformState,
    pub(crate) wb: &'a mut WorkbenchSession,
    pub(crate) facts: SlotFacts,
    pub(crate) presentation: &'a dyn ResultPresentationSource,
    pub(crate) design: &'a dyn DesignPort,
    pub(crate) code: &'a dyn CodePort,
    /// Every command, for the label of the design feature's repair.
    pub(crate) commands: &'static Registry,
}

pub(crate) fn open_check_and_save_dialog(parts: &mut CheckAndSaveParts<'_>) {
    if !parts.platform.project_lifecycle.is_open() {
        parts.wb.prompt("Check and save needs an open project.");
        return;
    }
    // Check and save is greyed on a read-only project or schematic.
    if parts.model.schematic.session.read_only
        || parts.model.active_view_read_only()
        || parts.model.safe_mode.project_read_only()
    {
        return;
    }
    if let Some(prompt) = parts.model.sync_active_schematic_to_workspace() {
        parts.wb.prompt(prompt);
    }
    parts.run_design_checks();
    match CheckAndSaveValidationReport::collect(&parts.reads()) {
        Ok(report) => {
            // One line for the check. The dialog lists every finding, and
            // Problems lists the design-check ones where they can be visited.
            let blockers = report.blockers().len();
            let advisories = report.advisories().len();
            parts.model.record(
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
            parts.dialog.open(report);
        }
        Err(error) => {
            parts.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Check and save could not complete its checks: {error}"),
            );
        }
    }
}

impl CheckAndSaveParts<'_> {
    /// What the report reads, as the parts stand now.
    fn reads(&self) -> ValidationReads<'_> {
        ValidationReads {
            model: &*self.model,
            platform: self.platform,
            facts: self.facts,
            workspace: self.wb.workspace,
            presentation: self.presentation,
            design: self.design,
            code: self.code,
        }
    }

    /// Run the design checks the report decides from.
    fn run_design_checks(&mut self) {
        run_design_checks(self.model, self.platform, self.facts, self.wb);
    }

    /// Draw the dialog. Saving the validated revision is asked of the shell,
    /// which carries it out as the dialog's step returns.
    pub(crate) fn render_check_and_save_dialog(&mut self, ctx: &Context) {
        if !self.dialog.open {
            return;
        }
        let Some(report) = self.dialog.report.clone() else {
            self.dialog.validation_error = Some(
                "Validation results are unavailable; close and reopen Check and save.".to_owned(),
            );
            return;
        };
        let dialog = &*self.dialog;
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
            checks_current: report.source_is_current(&self.reads()),
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
                // The repair is the design feature's command, named by the
                // label it declares.
                if let Some(instance) = repair_instance.as_deref()
                    && let Some(repair) = self.commands.spec(ids::UPDATE_INSTANCE_INTERFACE)
                {
                    repair_requested = rows.value_action(
                        "Instance",
                        &format!("{instance} is out of date"),
                        repair.label,
                    );
                }
                rows.value("Documents", report.document_scope_label());
                note_changed = rows
                    .text("Revision note", note, &mut self.dialog.revision_note, "")
                    .changed();
                Some(note)
            });
        if note_changed {
            self.dialog.mark_edited();
        }
        if let Some(instance) = repair_instance.filter(|_| repair_requested) {
            self.repair_stale_instance_interface(&instance);
        }
        if show_problems {
            // The findings are visited on the Problems page, which the modal
            // would otherwise cover.
            self.dialog.close();
            self.wb.show_problem_findings();
            return;
        }
        match choice {
            DialogChoice::Primary => self.wb.outbox.push(ProjectRequest::CommitCheckAndSave),
            // A save in flight cannot be abandoned half-done; Esc waits.
            DialogChoice::Ghost | DialogChoice::Cancelled if !pending => {
                self.dialog.close();
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
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name.eq_ignore_ascii_case(instance))
            .map(|component| component.id)
        else {
            self.wb.prompt(format!(
                "Instance '{instance}' is no longer in the active schematic."
            ));
            return;
        };
        self.model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(component_id);
        let outcome = self.model.schematic.update_selected_instance_interface(
            &self.model.library_manager,
            &self.model.workspace.content.schematic_buffers,
        );
        self.wb.prompt(match outcome {
            Ok(summary) => summary,
            Err(error) => error.to_string(),
        });
        self.run_design_checks();
        match CheckAndSaveValidationReport::collect(&self.reads()) {
            Ok(report) => self.dialog.report = Some(report),
            Err(error) => {
                self.dialog.validation_error = Some(error);
            }
        }
    }
}

pub(crate) fn revision_note_error(note: &str) -> Option<String> {
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

pub(crate) fn rollback_exact_journal(
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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    use crate::product::ContentDigest;
    use crate::state::{ValidatedRevisionRequest, ValidationFindingCounts};

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
