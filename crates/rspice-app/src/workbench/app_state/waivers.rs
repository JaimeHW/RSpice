//! Waivers on design-rule findings, as the workbench records and states them.
//!
//! The design crate decides what a waiver is and which finding it covers.
//! This is where one is recorded against the cellview that stores it — an
//! edit of that cellview's document, on its own undo stack — where every
//! check takes the waivers it applies, and where every published finding is
//! stated again against the waivers as they now stand.
//!
//! A waiver does not change what a check finds, only whether a finding counts,
//! so the waivers are not an input to a check's digest: recording one leaves
//! every finding current, and the published findings are re-stated in place
//! rather than going stale until somebody re-checks.

use std::sync::{Arc, OnceLock};

use rspice_app_types::diagnostics::DiagnosticSuppression;
use rspice_design::drc::{
    CheckWaivers, DrcResult, DrcViolation, DrcWaiver, DrcWaivers, waiver_site,
};
use rspice_design::schematic::document::SchematicDocument;

use super::AppState;
use super::diagnostic_store::{
    DiagnosticLevel, DiagnosticRecord, DiagnosticSubject, FindingKey, mint_canonical,
};
use crate::state::{CellViewRef, ViewType};

/// The producer every record about a stored waiver carries.
const WAIVER_PRODUCER: &str = "rspice.design.waivers";

/// Where the waiver of one finding is stored, and what it names there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) struct WaiverTarget {
    /// The cellview whose document stores the waiver.
    pub(in crate::workbench) cell_view: CellViewRef,
    /// The code of the rule the finding is an instance of.
    pub(in crate::workbench) rule: Arc<str>,
    /// The object inside that cellview the finding is located at.
    pub(in crate::workbench) object: String,
}

/// What the page may do about a finding's waiver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum WaiverAction {
    /// The finding is not waived and its rule may be: record one.
    Waive(WaiverTarget),
    /// A waiver covers the finding: remove it, so the finding counts again.
    Remove(WaiverTarget),
    /// A stored waiver does nothing — its finding is gone, or its rule may
    /// not be waived or is not one this build has: delete it from the
    /// document that stores it.
    Delete(WaiverTarget),
}

impl WaiverTarget {
    /// Where a finding of `subject`'s check is waived, for a rule that may be
    /// waived at all.
    fn of_finding(subject: &CellViewRef, violation: &DrcViolation) -> Option<Self> {
        let rule = violation.violation_type;
        if !rule.rule().waivable {
            return None;
        }
        let (site, object) = waiver_site(&violation.location);
        Some(Self {
            cell_view: site.cloned().unwrap_or_else(|| subject.clone()),
            rule: Arc::from(rule.rule_code()),
            object,
        })
    }

    fn of_stored(cell_view: &CellViewRef, waiver: &DrcWaiver) -> Self {
        Self {
            cell_view: cell_view.clone(),
            rule: Arc::from(waiver.rule.as_str()),
            object: waiver.object.clone(),
        }
    }
}

/// What the page may do about the waiver of a finding of `subject`'s check:
/// remove the waiver covering it, waive it, or nothing, for a rule that may
/// not be waived.
pub(in crate::workbench) fn finding_waiver_action(
    subject: &CellViewRef,
    violation: &DrcViolation,
) -> Option<WaiverAction> {
    let target = WaiverTarget::of_finding(subject, violation)?;
    Some(if violation.waiver.is_some() {
        WaiverAction::Remove(target)
    } else {
        WaiverAction::Waive(target)
    })
}

/// The waiver covering a finding, as the record states it.
pub(in crate::workbench) fn suppression_of(waiver: &DrcWaiver) -> DiagnosticSuppression {
    DiagnosticSuppression {
        rule: waiver.rule.clone(),
        reason: waiver.reason.clone(),
        author: waiver.author.clone(),
        waived_at_ms: waiver.waived_at_ms,
    }
}

/// The operating-system user a waiver is recorded under, read once.
fn waiver_author() -> &'static str {
    static AUTHOR: OnceLock<String> = OnceLock::new();
    AUTHOR.get_or_init(|| {
        ["USERNAME", "USER", "LOGNAME"]
            .into_iter()
            .find_map(|name| {
                std::env::var(name)
                    .ok()
                    .map(|value| value.trim().to_owned())
                    .filter(|value| !value.is_empty())
            })
            .unwrap_or_else(|| "unknown user".to_owned())
    })
}

/// The cellview a workspace schematic buffer key names.
fn cell_view_of_key(key: &str) -> Option<CellViewRef> {
    let mut segments = key.splitn(3, '/');
    let (library, cell, view) = (segments.next()?, segments.next()?, segments.next()?);
    Some(CellViewRef::new(library, cell, view))
}

/// One record per waiver stored with `subject` that does not do what it
/// says: one the stored list could not be read for, one naming a rule this
/// build does not have, one on a rule that may not be waived, and one whose
/// finding is gone from `result`.
///
/// None of them is silently kept or silently deleted. Each says what it is
/// and offers to remove the waiver, except a list this build cannot read,
/// which it keeps as it is.
pub(in crate::workbench) fn stored_waiver_records(
    state: &AppState,
    subject: &CellViewRef,
    stored: &DrcWaivers,
    result: &DrcResult,
) -> Vec<DiagnosticRecord> {
    let project = state.workspace.content.project.id();
    let revision = state.workspace.content.project.revision().get();
    let document: Arc<str> = Arc::from(subject.key().as_str());
    let applied = CheckWaivers::for_subject(stored);
    let unmatched = applied.unmatched_in(result);
    let mut findings = Vec::new();
    if let Some(reason) = stored.unread() {
        findings.push(StoredWaiverFinding {
            code: "WAIVER-UNREADABLE",
            waiver: None,
            level: DiagnosticLevel::Warning,
            message: format!(
                "The waivers stored with {} cannot be read ({reason}); none of them is applied, \
                 and they are saved back unchanged.",
                subject.display_path()
            ),
        });
    }
    for waiver in stored.entries() {
        let finding = match waiver.violation_type() {
            None => StoredWaiverFinding {
                code: "WAIVER-UNKNOWN-RULE",
                waiver: Some(waiver),
                level: DiagnosticLevel::Info,
                message: format!(
                    "A waiver names {}, a rule this build does not have; it is kept and not \
                     applied.",
                    waiver.rule
                ),
            },
            Some(rule) if !rule.rule().waivable => StoredWaiverFinding {
                code: "WAIVER-NOT-WAIVABLE",
                waiver: Some(waiver),
                level: DiagnosticLevel::Warning,
                message: format!(
                    "The waiver of {} on {} is ignored: {} may not be waived, so the finding \
                     still counts.",
                    waiver.rule,
                    waiver.object,
                    rule.rule().title
                ),
            },
            Some(_) if unmatched.contains(&waiver) => StoredWaiverFinding {
                code: "WAIVER-UNMATCHED",
                waiver: Some(waiver),
                level: DiagnosticLevel::Info,
                message: format!(
                    "The waiver of {} on {} covers nothing: the finding it was recorded against \
                     is gone.",
                    waiver.rule, waiver.object
                ),
            },
            Some(_) => continue,
        };
        findings.push(finding);
    }
    let keys = findings
        .iter()
        .map(|finding| {
            FindingKey::new(
                WAIVER_PRODUCER,
                finding.code,
                Arc::clone(&document),
                finding.object(),
            )
        })
        .collect::<Vec<_>>();
    let canonicals = mint_canonical(&keys, revision, &format!("waivers:{}", subject.key()));
    findings
        .into_iter()
        .zip(keys)
        .zip(canonicals)
        .map(|((finding, key), canonical)| {
            let object = finding.object();
            DiagnosticRecord::new(
                canonical,
                finding.level,
                DiagnosticSubject::new(
                    project,
                    if object.is_empty() {
                        "Waivers".to_owned()
                    } else {
                        format!("Waiver \u{00b7} {object}")
                    },
                )
                .in_cell_view(subject.clone()),
                finding.message,
            )
            .keyed(key)
            .with_waiver_action(
                finding
                    .waiver
                    .map(|waiver| WaiverAction::Delete(WaiverTarget::of_stored(subject, waiver))),
            )
        })
        .collect()
}

/// One stored waiver, or the whole stored list, that does not do what it
/// says.
struct StoredWaiverFinding<'a> {
    code: &'static str,
    /// The waiver, where the finding is about one rather than the list.
    waiver: Option<&'a DrcWaiver>,
    level: DiagnosticLevel,
    message: String,
}

impl StoredWaiverFinding<'_> {
    /// What the finding is about, as its identity is keyed: the waiver's
    /// rule and object, or nothing for the list as a whole.
    fn object(&self) -> String {
        self.waiver
            .map(|waiver| format!("{} on {}", waiver.rule, waiver.object))
            .unwrap_or_default()
    }
}

impl AppState {
    /// The view in front of the reader, where it is a schematic.
    fn active_schematic_view(&self) -> Option<CellViewRef> {
        matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
        .then(|| self.workspace.content.active_view.clone())
    }

    /// Every schematic document the workspace holds, by cellview, the view in
    /// front of the reader read from its live editor.
    fn schematic_documents(&self) -> Vec<(CellViewRef, &SchematicDocument)> {
        let active = self.active_schematic_view();
        let mut documents = Vec::new();
        if let Some(active) = &active {
            documents.push((active.clone(), self.schematic.document()));
        }
        for (key, design) in &self.workspace.content.schematic_buffers {
            if active
                .as_ref()
                .is_some_and(|active| active.key().eq_ignore_ascii_case(key))
            {
                continue;
            }
            if let Some(cell_view) = cell_view_of_key(key) {
                documents.push((cell_view, design.document()));
            }
        }
        documents
    }

    /// The waivers stored with the schematic `cell_view`, where the
    /// workspace holds it.
    pub(in crate::workbench) fn schematic_waivers(
        &self,
        cell_view: &CellViewRef,
    ) -> Option<&DrcWaivers> {
        let key = cell_view.key();
        self.schematic_documents()
            .into_iter()
            .find(|(held, _)| held.key().eq_ignore_ascii_case(&key))
            .map(|(_, document)| &document.waivers)
    }

    /// The waivers a check of `subject` applies: its own, and each other
    /// schematic's, for the findings it states inside an instance of that
    /// schematic.
    pub(in crate::workbench) fn check_waivers(&self, subject: &CellViewRef) -> CheckWaivers {
        let subject_key = subject.key();
        let documents = self.schematic_documents();
        let empty = DrcWaivers::default();
        let own = documents
            .iter()
            .find(|(cell_view, _)| cell_view.key().eq_ignore_ascii_case(&subject_key))
            .map_or(&empty, |(_, document)| &document.waivers);
        documents
            .iter()
            .filter(|(cell_view, _)| !cell_view.key().eq_ignore_ascii_case(&subject_key))
            .fold(
                CheckWaivers::for_subject(own),
                |waivers, (cell_view, document)| {
                    waivers.instantiating(cell_view, &document.waivers)
                },
            )
    }

    /// Every waiver the documents open for editing store: each schematic's,
    /// and the symbol's in front of the reader. Taken before an undo or redo
    /// and compared after it, it says whether the step recorded or took
    /// away a waiver.
    pub(in crate::workbench) fn stored_waivers(&self) -> StoredWaivers {
        StoredWaivers {
            schematics: self
                .schematic_documents()
                .into_iter()
                .map(|(cell_view, document)| (cell_view.key(), document.waivers.clone()))
                .collect(),
            symbol: (self.workspace.content.active_view_type() == ViewType::Symbol)
                .then(|| self.load_active_symbol_document().ok())
                .flatten()
                .map(|document| document.waivers),
        }
    }

    /// State the findings again where a history step changed the waivers
    /// stored as they were in `before`. Returns whether a schematic's
    /// changed: those are the waivers the run gate applies.
    ///
    /// An undo restores the document whole, a waiver with it, and nothing
    /// about that tells the published findings: without this a finding whose
    /// waiver was undone would still be listed waived, and still not count.
    pub(in crate::workbench) fn restate_waived_findings(&mut self, before: &StoredWaivers) -> bool {
        let after = self.stored_waivers();
        if after.symbol != before.symbol {
            // The pin checks read the symbol document, waivers and all.
            self.run_active_symbol_pin_checks();
        }
        let schematics = after.schematics != before.schematics;
        if schematics {
            self.refresh_waived_findings();
        }
        schematics
    }

    /// Waive the finding at `target` for `reason`, as an edit of the document
    /// that stores it, and state every published finding again.
    pub(in crate::workbench) fn waive_finding(
        &mut self,
        target: &WaiverTarget,
        reason: &str,
    ) -> Result<(), String> {
        let waived_at_ms = crate::time_compat::checked_unix_time_ms()
            .map_err(|error| format!("The waiver was not recorded: {error}."))?;
        let waiver = DrcWaiver {
            rule: target.rule.to_string(),
            object: target.object.clone(),
            reason: reason.to_owned(),
            author: waiver_author().to_owned(),
            waived_at_ms,
        };
        self.edit_waivers(&target.cell_view, StoredWaiverEdit::Waive(waiver))
    }

    /// Remove the waiver stored at `target`, as an edit of the document that
    /// stores it, and state every published finding again.
    pub(in crate::workbench) fn remove_finding_waiver(
        &mut self,
        target: &WaiverTarget,
    ) -> Result<(), String> {
        self.edit_waivers(
            &target.cell_view,
            StoredWaiverEdit::Remove {
                rule: target.rule.to_string(),
                object: target.object.clone(),
            },
        )
    }

    fn edit_waivers(
        &mut self,
        cell_view: &CellViewRef,
        edit: StoredWaiverEdit,
    ) -> Result<(), String> {
        if self.workbench.safe_mode.project_read_only() {
            return Err("The project is read-only.".to_owned());
        }
        let key = cell_view.key();
        let active = self
            .workspace
            .content
            .active_view
            .key()
            .eq_ignore_ascii_case(&key);
        match self.workspace.content.active_view_type() {
            ViewType::Schematic | ViewType::Testbench if active => {
                edit.apply_to_schematic(&mut self.schematic)?;
            }
            ViewType::Symbol if active => {
                self.edit_active_symbol_waivers(edit)?;
                // The pin checks read the symbol document, waivers and all,
                // so they are asked again rather than left stale.
                self.run_active_symbol_pin_checks();
            }
            _ => {
                let held = self
                    .workspace
                    .content
                    .schematic_buffers
                    .keys()
                    .find(|held| held.eq_ignore_ascii_case(&key))
                    .cloned()
                    .ok_or_else(|| {
                        format!(
                            "{} is not open in this project; open it to change its waivers.",
                            cell_view.display_path()
                        )
                    })?;
                let mut editor = self
                    .workspace
                    .schematic_editor_mut(&held)
                    .ok_or_else(|| format!("{} could not be edited.", cell_view.display_path()))?;
                edit.apply_to_schematic(&mut editor.editor)?;
            }
        }
        self.workbench.preflight.invalidate();
        self.refresh_waived_findings();
        Ok(())
    }

    fn edit_active_symbol_waivers(&mut self, edit: StoredWaiverEdit) -> Result<(), String> {
        let before = self.load_active_symbol_document()?;
        let mut document = before.clone();
        edit.apply(&mut document.waivers)?;
        if document == before {
            return Ok(());
        }
        self.store_active_symbol_document(&document)?;
        self.record_symbol_edit(&before);
        Ok(())
    }
}

/// The waivers the documents open for editing store, as
/// [`AppState::stored_waivers`] takes them.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::workbench) struct StoredWaivers {
    schematics: Vec<(String, DrcWaivers)>,
    symbol: Option<DrcWaivers>,
}

/// One change to the waivers a document stores.
enum StoredWaiverEdit {
    Waive(DrcWaiver),
    Remove { rule: String, object: String },
}

impl StoredWaiverEdit {
    fn apply(self, waivers: &mut DrcWaivers) -> Result<(), String> {
        match self {
            Self::Waive(waiver) => waivers.waive(waiver).map_err(|refusal| refusal.to_string()),
            Self::Remove { rule, object } => waivers
                .remove(&rule, &object)
                .map(drop)
                .map_err(|refusal| refusal.to_string()),
        }
    }

    fn apply_to_schematic(
        self,
        schematic: &mut crate::state::SchematicState,
    ) -> Result<(), String> {
        match self {
            Self::Waive(waiver) => schematic.waive(waiver),
            Self::Remove { rule, object } => schematic.remove_waiver(&rule, &object).map(drop),
        }
    }
}
