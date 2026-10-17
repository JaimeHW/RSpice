//! Runtime design-check evidence keyed by exact project cell/view identity.
//!
//! Interactive checks are derived evidence: they do not dirty project bytes or
//! masquerade as a validated save. Each receipt binds the checker profile and
//! everything its check read, so switching documents can never relabel
//! another cell's findings as current, and drawing in a cellview the check
//! never read never stales them.

use std::collections::BTreeMap;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::product::{ContentDigest, ProjectId};
use crate::state::{
    CanonicalCellViewOwnerKey, CellViewRef, ViewType, canonical_cell_view_owner_key,
};
use rspice_design::drc::{CheckReach, DrcConfig, DrcLocation, DrcResult, DrcViolationType};

use super::AppState;
use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject, FindingKey,
    RelatedPlace, mint_canonical,
};
use super::waivers;

mod live;
mod signoff;
pub(in crate::workbench) use live::live_check_digest;

/// Producer identity every schematic and electrical rule finding carries.
const DESIGN_RULE_PRODUCER: &str = "rspice.design.rules";

const DESIGN_CHECK_DIGEST_DOMAIN: &[u8] = b"rspice-interactive-design-check/v1\0";

#[derive(Debug, Clone)]
pub(crate) struct CellViewCheckReceipt {
    pub(crate) project_id: ProjectId,
    pub(crate) subject: CellViewRef,
    pub(crate) input_digest: ContentDigest,
    pub(crate) result: DrcResult,
    /// How much of the design the check stated.
    pub(crate) scope: CheckScope,
    /// Where it stands in the order the checks were published in, which is
    /// the order a waiver edit states them again in: a later check took over
    /// the findings it shares with an earlier one, and still does after.
    sequence: u64,
}

/// How much of the design one check states, which is what it takes over from
/// the checks published before it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CheckScope {
    /// The subject's own findings: Check Cellview, and Check and Save.
    Cellview,
    /// The subject and every cellview its walk reaches.
    Hierarchy,
    /// The configured design, walked from the simulation root, and every
    /// schematic of the project's own libraries the walk never reaches:
    /// Check Design.
    Design,
    /// The live rules, on the drawing an author is editing.
    Live,
}

impl CheckScope {
    const fn reach(self) -> CheckReach {
        match self {
            Self::Cellview | Self::Live => CheckReach::Cellview,
            Self::Hierarchy | Self::Design => CheckReach::Hierarchy,
        }
    }
}

/// What one design-rule pass states: for each cellview, the rules whose
/// findings about objects drawn in it the pass decides, and every finding it
/// states.
///
/// A pass published with its coverage takes over every finding an earlier
/// pass states under it, so a finding is listed once, by the newest check
/// that looked at it. That is how Check Cellview, Check Design and the
/// checks beside them compose: a check of `amp` after a check of the whole
/// design states `amp`'s findings itself, and the design check's statements
/// of them at `X1` and `X2` leave the list; a check of the design after that
/// takes them back.
///
/// A pass takes over only what it computed. Where it decides a rule for a
/// cellview, a finding of that rule it does not state is one it looked for
/// and did not find. Where it does not — the rules only the walk from the
/// configured root computes, for every pass narrower than that walk — it
/// takes over the findings it states again and leaves every other standing,
/// because it never looked for them.
#[derive(Debug, Clone, Default)]
pub(in crate::workbench) struct RuleCoverage {
    cellviews: BTreeMap<CanonicalCellViewOwnerKey, RuleScope>,
    /// Each finding the pass states, by rule, the cellview its object is
    /// drawn in, and the object as a waiver names it.
    stated: std::collections::BTreeSet<(String, CanonicalCellViewOwnerKey, String)>,
}

/// Which rules a pass decides about one cellview.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleScope {
    /// Every rule a check of the cellview's contents decides: all but
    /// whether the cellview is used, which only a check of the whole design
    /// can say, and the rules only the walk from the configured root
    /// computes.
    Contents,
    /// Every rule.
    Every,
    /// Only whether the cellview is used.
    Use,
    /// The rules checked while the cellview is drawn, less those only the
    /// walk from the configured root computes.
    Live,
}

impl RuleScope {
    fn covers(self, code: &str) -> bool {
        let unused = DrcViolationType::UnusedCellView.rule_code();
        let is = |rules: &[DrcViolationType]| rules.iter().any(|rule| rule.rule_code() == code);
        let walked = is(&rspice_design::drc::HIERARCHY_RULES);
        match self {
            Self::Every => true,
            Self::Contents => code != unused && !walked,
            Self::Use => code == unused,
            Self::Live => is(&rspice_design::drc::LIVE_RULES) && !walked,
        }
    }
}

impl RuleCoverage {
    fn state(&mut self, cell_view: &CellViewRef, scope: RuleScope) {
        self.cellviews.insert(owner_key(cell_view), scope);
    }

    /// Record every finding `result`, a pass of `subject`, states.
    fn restate(&mut self, subject: &CellViewRef, result: &DrcResult) {
        self.stated
            .extend(result.violations().iter().map(|violation| {
                let (drawn_in, object) = rspice_design::drc::waiver_site(&violation.location);
                (
                    violation.violation_type.rule_code().to_owned(),
                    owner_key(drawn_in.unwrap_or(subject)),
                    object,
                )
            }));
    }

    /// Whether this pass states the finding of rule `code` about `object`,
    /// drawn in `drawn_in`.
    fn takes_over(&self, code: &str, drawn_in: &CanonicalCellViewOwnerKey, object: &str) -> bool {
        self.cellviews
            .get(drawn_in)
            .is_some_and(|scope| scope.covers(code))
            || self
                .stated
                .contains(&(code.to_owned(), drawn_in.clone(), object.to_owned()))
    }

    /// Whether this pass states the finding `record` holds.
    pub(in crate::workbench) fn covers(&self, record: &DiagnosticRecord) -> bool {
        let Some(drawn_in) = record.drawn_in.as_ref() else {
            return false;
        };
        match record.drawn_object.as_deref() {
            Some(object) => self.takes_over(&record.canonical.code, drawn_in, object),
            None => self
                .cellviews
                .get(drawn_in)
                .is_some_and(|scope| scope.covers(&record.canonical.code)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum DesignCheckStatus<'a> {
    NotRun,
    Current(&'a CellViewCheckReceipt),
    Stale(&'a CellViewCheckReceipt),
    Unavailable {
        last: Option<&'a CellViewCheckReceipt>,
        reason: String,
    },
}

impl DesignCheckStatus<'_> {
    pub(crate) fn current_receipt(&self) -> Option<&CellViewCheckReceipt> {
        match self {
            Self::Current(receipt) => Some(receipt),
            Self::NotRun | Self::Stale(_) | Self::Unavailable { .. } => None,
        }
    }

    pub(crate) fn last_receipt(&self) -> Option<&CellViewCheckReceipt> {
        match self {
            Self::Current(receipt) | Self::Stale(receipt) => Some(receipt),
            Self::Unavailable { last, .. } => *last,
            Self::NotRun => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DesignCheckRuntime {
    receipts: BTreeMap<CanonicalCellViewOwnerKey, CellViewCheckReceipt>,
    /// The live checks' passes, by the cellview each was of.
    live: BTreeMap<CanonicalCellViewOwnerKey, CellViewCheckReceipt>,
    /// Where the drawing in front of the reader stands against its live
    /// check.
    drawing: live::DrawingState,
    /// The sequence the next receipt is stored under.
    published: u64,
}

impl DesignCheckRuntime {
    fn receipt(&self, subject: &CellViewRef) -> Option<&CellViewCheckReceipt> {
        self.receipts.get(&owner_key(subject))
    }

    fn insert(&mut self, mut receipt: CellViewCheckReceipt) {
        receipt.sequence = self.published;
        self.published = self.published.wrapping_add(1);
        self.receipts.insert(owner_key(&receipt.subject), receipt);
    }

    fn insert_live(&mut self, mut receipt: CellViewCheckReceipt) {
        receipt.sequence = self.published;
        self.published = self.published.wrapping_add(1);
        self.live.insert(owner_key(&receipt.subject), receipt);
    }

    pub(crate) fn clear(&mut self) {
        self.receipts.clear();
        self.live.clear();
        self.drawing = live::DrawingState::default();
    }

    fn clear_subject(&mut self, subject: &CellViewRef) {
        self.receipts.remove(&owner_key(subject));
        self.live.remove(&owner_key(subject));
    }
}

impl AppState {
    /// Check Cellview: the open cellview's own findings.
    ///
    /// The hierarchy below is walked only for what no single cellview can
    /// answer — whether the design reaches ground and whether a cycle runs
    /// through it — so it answers in the time the one drawing takes.
    pub(crate) fn check_active_cellview(&mut self) -> Result<DrcResult, String> {
        if !matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return Err("only a schematic or a testbench has design rules to check".to_owned());
        }
        let subject = self.workspace.content.active_schematic_reference();
        self.run_design_check(subject, CheckScope::Cellview)
    }

    /// Check Design: the configured design, walked from the simulation root,
    /// with every schematic of the project's own libraries it never reaches.
    pub(crate) fn check_design(&mut self) -> Result<DrcResult, String> {
        let root = self.workspace.content.simulation_root_reference();
        self.run_design_check(root, CheckScope::Design)
    }

    /// Check `subject` as far as `scope` reaches, unless a check of it that
    /// reached exactly as far still describes the design.
    ///
    /// For a workflow that decides from the checks rather than being one:
    /// Check and Save and the run's preflight read the receipt this leaves
    /// current, and the page lists what that check published, so none of
    /// them can say something else about the same design.
    pub(in crate::workbench) fn ensure_design_check(
        &mut self,
        subject: CellViewRef,
        scope: CheckScope,
    ) -> Result<(), String> {
        if self.current_design_check(&subject, scope).is_some() {
            return Ok(());
        }
        self.run_design_check(subject, scope).map(|_| ())
    }

    /// The findings of the check of `subject` that reached as far as
    /// `scope`, while they still describe the design.
    pub(in crate::workbench) fn current_design_check(
        &self,
        subject: &CellViewRef,
        scope: CheckScope,
    ) -> Option<&DrcResult> {
        match self.design_check_status(subject) {
            DesignCheckStatus::Current(receipt) if receipt.scope == scope => Some(&receipt.result),
            DesignCheckStatus::Current(_)
            | DesignCheckStatus::NotRun
            | DesignCheckStatus::Stale(_)
            | DesignCheckStatus::Unavailable { .. } => None,
        }
    }

    /// The open cellview and every cellview its walk reaches.
    pub(crate) fn run_active_design_checks(&mut self) -> Result<DrcResult, String> {
        let subject = self.workspace.content.active_schematic_reference();
        self.run_design_check(subject, CheckScope::Hierarchy)
    }

    fn run_design_check(
        &mut self,
        subject: CellViewRef,
        scope: CheckScope,
    ) -> Result<DrcResult, String> {
        if self.workspace.annotation_restoration_error().is_some() {
            self.sync_active_schematic_to_workspace();
        }
        let config = self.design_check_config(&subject);
        // Checks run over the design as configured, not over the editor
        // buffer. That is what makes two sheets with coincident authored
        // coordinates two nets: the projection namespaces the pages apart
        // before anything electrical is read off them.
        let projection = self
            .workspace
            .design_projection(
                &self.library_manager,
                &self.workspace.content.active_view,
                &self.schematic,
            )
            .map_err(|error| error.to_string())?;
        let checked = projected_buffer(&projection, &subject)?;
        let hierarchy = rspice_design::hierarchy::HierarchySource::from_design_projection(
            self.library_manager.catalog(),
            &projection,
        );
        let mut result = rspice_design::drc::run_check_of_cellview(
            checked,
            &subject,
            &hierarchy,
            config.clone(),
            scope.reach(),
        );
        if scope == CheckScope::Design {
            let reads = rspice_design::drc::check_reads(checked, &subject, &hierarchy);
            rspice_design::drc::append_unused_cellviews(
                &mut result,
                &subject,
                project_schematics(self),
                &reads,
                &config,
            );
        }
        self.publish_design_check_result(subject, config, result.clone(), scope)?;
        Ok(result)
    }

    pub(crate) fn publish_active_design_check_result(
        &mut self,
        result: DrcResult,
    ) -> Result<(), String> {
        if !matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return Err(
                "schematic design-check evidence can only be published from a schematic or testbench"
                    .to_owned(),
            );
        }
        let subject = self.workspace.content.active_schematic_reference();
        let config = self.design_check_config(&subject);
        self.publish_design_check_result(subject, config, result, CheckScope::Hierarchy)
    }

    /// The configuration the design rules are checked under for the
    /// configured simulation root — the profile the Check command uses for
    /// that cell view, so a pass run by either is the same evidence, and the
    /// one the run gate judges the root under.
    pub(crate) fn configured_root_design_check_config(&self) -> DrcConfig {
        self.design_check_config(&self.workspace.content.simulation_root_reference())
    }

    /// Publish `result` as Check Design's findings about the configured
    /// simulation root, for a test that needs findings no drawing produces.
    #[cfg(test)]
    pub(in crate::workbench) fn publish_configured_root_design_check_result(
        &mut self,
        result: DrcResult,
    ) -> Result<(), String> {
        let subject = self.workspace.content.simulation_root_reference();
        let config = self.design_check_config(&subject);
        self.publish_design_check_result(subject, config, result, CheckScope::Design)
    }

    fn publish_design_check_result(
        &mut self,
        subject: CellViewRef,
        config: DrcConfig,
        mut result: DrcResult,
        scope: CheckScope,
    ) -> Result<(), String> {
        if !result.completed {
            return Err(
                "an incomplete design-check result cannot become current evidence".to_owned(),
            );
        }
        // The waivers as the documents store them now, whoever produced the
        // pass: a check states them itself, and a pass handed in from
        // elsewhere, or re-stated after a waiver changed, is held to the same
        // ones. Applying them again to a pass that already states them
        // changes nothing.
        config.waivers.apply(&mut result);
        let is_active_subject =
            owner_key(&subject) == owner_key(&self.workspace.content.active_schematic_reference());
        let input_digest = self.diagnostic_live_digest(&DiagnosticInputs::DesignCheck {
            subject: subject.clone(),
        })?;
        // Canonical records are derived before anything is stored, because
        // one of them needs to ask the live drawing where its object is and
        // both insertions take the state mutably.
        let records = design_rule_records(self, &subject, &result, StoredWaivers::Stated);
        let coverage = check_coverage(self, &subject, scope, &result)?;
        // Published before the receipt is stored, because the store can
        // refuse a batch and the receipt cannot: a receipt stored first would
        // put findings on the canvas and the check pill that the Problems
        // page, refusing them, does not list.
        self.publish_superseding(
            DiagnosticInputs::DesignCheck {
                subject: subject.clone(),
            },
            records,
            &coverage,
        )?;
        self.design_checks.insert(CellViewCheckReceipt {
            project_id: self.workspace.content.project.id(),
            subject,
            input_digest,
            result,
            scope,
            sequence: 0,
        });
        if is_active_subject {
            self.refresh_active_design_check_projection();
        }
        Ok(())
    }

    /// What the rule findings of the check of `subject` are called, in the
    /// order the check reported them: [`DrcResult::violations`]' own, so a
    /// surface holding the result names the finding at an ordinal.
    ///
    /// Minted from the receipt the way its records were, rather than read
    /// off the subject's batch, because a later check may have taken a
    /// finding over: the identity is structural, so it is the record's
    /// wherever the store now holds it, and the batch no longer lines up with
    /// the result.
    ///
    /// Empty where no check of `subject` is on record. A symbol view's
    /// findings are never among them: they are the pin checks', owned by
    /// [`DiagnosticInputs::SymbolPins`] and named there.
    ///
    /// The canvas names its markers by [`Self::canvas_finding_identities`],
    /// since they may be two passes' findings; this names one pass's.
    #[cfg(test)]
    pub(crate) fn design_rule_finding_identities(&self, subject: &CellViewRef) -> Vec<uuid::Uuid> {
        let Some(receipt) = self.design_checks.receipt(subject) else {
            return Vec::new();
        };
        finding_identities(&receipt.subject, &receipt.result)
    }

    /// What the findings the canvas marks are called, in the order the
    /// canvas holds them. They may be two passes' findings composed, so they
    /// are named from that set rather than from either pass's receipt.
    pub(crate) fn canvas_finding_identities(&self) -> Vec<uuid::Uuid> {
        let Some(result) = &self.dialogs.drc_results else {
            return Vec::new();
        };
        finding_identities(&self.workspace.content.active_schematic_reference(), result)
    }

    /// The worst rule finding published for `subject`, first among equals in
    /// the order the checks reported them — where a worst-first tour starts.
    pub(in crate::workbench) fn worst_design_rule_finding(
        &self,
        subject: &CellViewRef,
    ) -> Option<uuid::Uuid> {
        let published = DiagnosticInputs::DesignCheck {
            subject: subject.clone(),
        };
        self.diagnostics
            .batch(&published)?
            .records
            .iter()
            .fold(
                None,
                |worst: Option<&DiagnosticRecord>, record| match worst {
                    Some(worst) if worst.level >= record.level => Some(worst),
                    _ => Some(record),
                },
            )
            .map(|record| record.canonical.diagnostic_id)
    }

    pub(crate) fn design_check_status(&self, subject: &CellViewRef) -> DesignCheckStatus<'_> {
        let last = self.design_checks.receipt(subject);
        let Some(receipt) = last else {
            return DesignCheckStatus::NotRun;
        };
        if receipt.project_id != self.workspace.content.project.id() {
            return DesignCheckStatus::Stale(receipt);
        }
        // Through the store's memo, so the canvas and the status bar asking
        // every frame cost nothing while the design stands still.
        let published = DiagnosticInputs::DesignCheck {
            subject: subject.clone(),
        };
        match self.diagnostic_live_digest(&published) {
            Ok(digest) if digest == receipt.input_digest => DesignCheckStatus::Current(receipt),
            Ok(_) => DesignCheckStatus::Stale(receipt),
            Err(reason) => DesignCheckStatus::Unavailable {
                last: Some(receipt),
                reason,
            },
        }
    }

    pub(crate) fn active_design_check_status(&self) -> DesignCheckStatus<'_> {
        if !matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return DesignCheckStatus::NotRun;
        }
        self.design_check_status(&self.workspace.content.active_schematic_reference())
    }

    /// Where Check Design's evidence about the configured design stands: the
    /// receipt is filed under the simulation root it walked from.
    pub(crate) fn simulation_root_design_check_status(&self) -> DesignCheckStatus<'_> {
        self.design_check_status(&self.workspace.content.simulation_root_reference())
    }

    pub(crate) fn clear_active_design_check(&mut self) {
        if !matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            self.refresh_active_design_check_projection();
            return;
        }
        let subject = self.workspace.content.active_schematic_reference();
        self.design_checks.clear_subject(&subject);
        self.withdraw_diagnostics(&DiagnosticInputs::LiveCheck {
            subject: subject.clone(),
        });
        self.withdraw_diagnostics(&DiagnosticInputs::DesignCheck { subject });
        // What the cleared passes had taken over from the checks still
        // standing is theirs again.
        self.restate_design_checks();
    }

    /// State every current design-rule pass again, in the order the passes
    /// were published: after a waiver is recorded or removed, after an undo
    /// or redo that may have done either, and after a pass is withdrawn.
    ///
    /// Against the waivers as they now stand, each finding says whether it
    /// counts. And each pass takes over again what it took over when it was
    /// published, from the passes still standing: a withdrawn pass — a live
    /// pass when live checking is turned off, a cellview's checks when they
    /// are cleared — had taken findings over from older passes that still
    /// describe the design, and those findings are listed again rather than
    /// leaving the list while the run gate still counts them.
    ///
    /// Only a current pass is re-stated. Its findings still describe the
    /// design, and a waiver changes whether each counts, not what was found,
    /// so the pass keeps its digest and stays current. A stale pass is left
    /// as it is: re-stating it would publish findings about a design that has
    /// moved under the digest of the one in front of the reader.
    pub(in crate::workbench) fn restate_design_checks(&mut self) {
        // In the order they were published, so each still takes over from the
        // checks before it what it took over then.
        let mut subjects = self
            .design_checks
            .receipts
            .values()
            .map(|receipt| (receipt.sequence, false, receipt.subject.clone()))
            .chain(
                self.design_checks
                    .live
                    .values()
                    .map(|receipt| (receipt.sequence, true, receipt.subject.clone())),
            )
            .collect::<Vec<_>>();
        subjects.sort_by_key(|(sequence, _, _)| *sequence);
        for (_, live, subject) in subjects {
            if live {
                self.restate_live_check(subject);
                continue;
            }
            let DesignCheckStatus::Current(receipt) = self.design_check_status(&subject) else {
                continue;
            };
            let result = receipt.result.clone();
            let scope = receipt.scope;
            let config = self.design_check_config(&subject);
            if let Err(error) = self.publish_design_check_result(subject, config, result, scope) {
                self.record(
                    crate::diagnostics::LogSeverity::Warning,
                    crate::diagnostics::LogSource::Studio,
                    format!("Findings could not be stated again: {error}"),
                );
            }
        }
    }

    pub(crate) fn clear_all_design_checks(&mut self) {
        let published: Vec<DiagnosticInputs> =
            self.design_checks
                .receipts
                .values()
                .map(|receipt| DiagnosticInputs::DesignCheck {
                    subject: receipt.subject.clone(),
                })
                .chain(self.design_checks.live.values().map(|receipt| {
                    DiagnosticInputs::LiveCheck {
                        subject: receipt.subject.clone(),
                    }
                }))
                .collect();
        self.design_checks.clear();
        for inputs in &published {
            self.withdraw_diagnostics(inputs);
        }
    }

    /// Keep the legacy interactive DRC projection scoped to the exact active
    /// schematic. Remaining canvas/status consumers can therefore never show
    /// a different cell/view's findings while they are migrated to receipts.
    pub(crate) fn refresh_active_design_check_projection(&mut self) {
        if !matches!(
            self.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            self.dialogs.drc_checked_version = 0;
            self.dialogs.drc_results = None;
            return;
        }
        let current =
            self.current_drawing_findings(&self.workspace.content.active_schematic_reference());
        self.dialogs.drc_checked_version = current
            .as_ref()
            .map_or(0, |_| self.schematic.topology_version());
        self.dialogs.drc_results = current;
    }

    /// The findings that stand in `subject`'s drawing: those of every pass of
    /// it that still describes it, the newer pass stating whatever both
    /// cover — the set the page lists as current for the drawing.
    ///
    /// A finding fixed while drawing therefore leaves the canvas with its
    /// row, and one a live pass restated is marked once, where the live pass
    /// found it. A pass the drawing has moved away from marks nothing.
    fn current_drawing_findings(&self, subject: &CellViewRef) -> Option<DrcResult> {
        let explicit = match self.design_check_status(subject) {
            DesignCheckStatus::Current(receipt) => Some(receipt),
            DesignCheckStatus::NotRun
            | DesignCheckStatus::Stale(_)
            | DesignCheckStatus::Unavailable { .. } => None,
        };
        let (explicit, live) = match (explicit, self.current_live_receipt(subject)) {
            (None, None) => return None,
            (Some(only), None) | (None, Some(only)) => return Some(only.result.clone()),
            // A check of the cellview states every rule a live pass does.
            (Some(explicit), Some(live)) if explicit.sequence > live.sequence => {
                return Some(explicit.result.clone());
            }
            (Some(explicit), Some(live)) => (explicit, live),
        };
        // Taken over as the Problems page takes it over: by the rule the live
        // pass decides here, or by the live pass stating the finding again.
        let mut coverage = RuleCoverage::default();
        coverage.state(subject, RuleScope::Live);
        coverage.restate(subject, &live.result);
        let mut composed = DrcResult::new();
        for violation in explicit.result.violations() {
            let (drawn_in, object) = rspice_design::drc::waiver_site(&violation.location);
            let taken_over = coverage.takes_over(
                violation.violation_type.rule_code(),
                &owner_key(drawn_in.unwrap_or(subject)),
                &object,
            );
            if !taken_over {
                composed.add_violation(violation.clone());
            }
        }
        for violation in live.result.violations() {
            composed.add_violation(violation.clone());
        }
        composed.completed = true;
        composed.duration_ms = live.result.duration_ms;
        Some(composed)
    }
}

/// One canonical record per rule finding.
///
/// Each route is resolved in the drawing the object is drawn in — the live
/// editor when that is the view in front of the reader, the workspace's copy
/// of it otherwise — and names that view and the sheet the object is on, so
/// following it opens the view and brings the sheet forward before it selects
/// anything. A finding about a view that is not open used to be published with
/// no route at all, and one resolved against whichever view was open would
/// have pointed into the wrong coordinate space. Every other object a finding
/// involves is published beside it, routed the same way.
fn design_rule_records(
    state: &AppState,
    subject: &CellViewRef,
    result: &DrcResult,
    stored_waivers: StoredWaivers,
) -> Vec<DiagnosticRecord> {
    let project = state.workspace.content.project.id();
    let revision = state.workspace.content.project.revision().get();
    let document_id = subject.key();
    let keys = finding_keys(subject, result);
    let canonicals = mint_canonical(&keys, revision, &format!("design-check:{document_id}"));
    let route_to = |location: &DrcLocation| {
        let (cell_view, location) = drawn_place(subject, location);
        crate::schematic::view::violations::location_anchor_in(state, cell_view, location)
            .map(DiagnosticRoute::Anchor)
    };
    let mut records = result
        .violations()
        .iter()
        .zip(keys)
        .zip(canonicals)
        .map(|((violation, key), mut canonical)| {
            let route = route_to(&violation.location);
            let related_places = violation
                .related
                .iter()
                .map(|related| {
                    let (cell_view, location) = drawn_place(subject, &related.location);
                    RelatedPlace {
                        label: format!(
                            "{} \u{00b7} {}",
                            violation.related_name(related),
                            related.relation.describe()
                        ),
                        cell_view: cell_view.clone(),
                        location: location.clone(),
                        route: route_to(&related.location),
                    }
                })
                .collect::<Vec<_>>();
            canonical.suppression = violation.waiver.as_ref().map(waivers::suppression_of);
            let (drawn_in, object) = rspice_design::drc::waiver_site(&violation.location);
            let drawn_in = drawn_in.unwrap_or(subject);
            // A finding about a whole cellview is stated in that cellview, not
            // in the one whose check found it.
            let cell_view = match &violation.location {
                DrcLocation::CellView { reference } => reference.clone(),
                _ => subject.clone(),
            };
            DiagnosticRecord::new(
                canonical,
                violation.severity.into(),
                DiagnosticSubject::new(project, violation.location.display())
                    .in_cell_view(cell_view),
                violation.message.clone(),
            )
            .keyed(key)
            .drawn_in(drawn_in)
            .drawn_object(object)
            .with_details(violation.violation_type.suggested_fix())
            .with_rule(violation.violation_type.rule().explanation)
            .with_route(route)
            .with_related_places(related_places)
            .with_waiver_action(waivers::finding_waiver_action(subject, violation))
        })
        .collect::<Vec<_>>();
    // The subject's own waivers that do not do what they say are stated with
    // the pass that found that out.
    if stored_waivers == StoredWaivers::Stated
        && let Some(stored) = state.schematic_waivers(subject)
    {
        records.extend(
            waivers::stored_waiver_records(state, subject, stored, result)
                .into_iter()
                .map(|record| record.drawn_in(subject)),
        );
    }
    records
}

/// The cellview a location in a check of `subject` is drawn in, and where it
/// is there: one stated inside an instance is drawn in that instance's master.
fn drawn_place<'a>(
    subject: &'a CellViewRef,
    location: &'a DrcLocation,
) -> (&'a CellViewRef, &'a DrcLocation) {
    match location {
        DrcLocation::InsideInstance { view, inner, .. } => drawn_place(view, inner),
        _ => (subject, location),
    }
}

/// Whether a pass states the subject's own waivers that cover nothing.
///
/// A live pass checks a few rules, so a waiver of any other rule covers none
/// of its findings without being unmatched: it is left to the checks that
/// state every rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StoredWaivers {
    Stated,
    Left,
}

/// What each finding in `result` is about, in its order: the key its
/// record's identity is minted from.
fn finding_keys(subject: &CellViewRef, result: &DrcResult) -> Vec<FindingKey> {
    let producer: Arc<str> = Arc::from(DESIGN_RULE_PRODUCER);
    let document: Arc<str> = Arc::from(subject.key().as_str());
    result
        .violations()
        .iter()
        .map(|violation| {
            FindingKey::new(
                Arc::clone(&producer),
                violation.violation_type.rule_code(),
                Arc::clone(&document),
                location_key(&violation.location),
            )
        })
        .collect()
}

/// What each finding in `result`, a pass of `subject`, is called, in its
/// order: the identity its record was minted with.
fn finding_identities(subject: &CellViewRef, result: &DrcResult) -> Vec<uuid::Uuid> {
    mint_canonical(&finding_keys(subject, result), 0, "")
        .into_iter()
        .map(|canonical| canonical.diagnostic_id)
        .collect()
}

/// The cellview `subject` names, as the configured design projects it.
fn projected_buffer<'p>(
    projection: &'p rspice_design::projection::DesignProjection,
    subject: &CellViewRef,
) -> Result<&'p rspice_design::projection::ProjectedSchematic, String> {
    projection
        .schematic_buffers()
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&subject.key()))
        .map(|(_, schematic)| schematic)
        .ok_or_else(|| {
            format!(
                "{} is not part of the configured design.",
                subject.display_path()
            )
        })
}

/// Every schematic in the project's own libraries.
///
/// A reference or technology library is attached read-only, which is how the
/// project tells it from its own everywhere else — what a cell may be created
/// in, copied to or edited in — and a cellview in one is never unused: the
/// project borrows what it needs from it and owes nothing for the rest. A
/// testbench is a root of its own, not a cell waiting to be placed.
fn project_schematics(state: &AppState) -> Vec<CellViewRef> {
    state
        .library_manager
        .catalog()
        .libraries_by_key()
        .filter(|(_, library)| !library.read_only)
        .flat_map(|(_, library)| {
            library.cells.values().flat_map(move |cell| {
                cell.views
                    .values()
                    .filter(|view| view.view_type == ViewType::Schematic)
                    .map(move |view| CellViewRef::new(&library.name, &cell.name, &view.name))
            })
        })
        .collect()
}

/// What a pass of `scope` over `subject`, which found `result`, states: the
/// rules it decides, read off the same projected design and walk the check
/// took, and every finding it states.
fn check_coverage(
    state: &AppState,
    subject: &CellViewRef,
    scope: CheckScope,
    result: &DrcResult,
) -> Result<RuleCoverage, String> {
    let mut coverage = RuleCoverage::default();
    coverage.restate(subject, result);
    match scope {
        CheckScope::Cellview => {
            coverage.state(subject, RuleScope::Contents);
            return Ok(coverage);
        }
        CheckScope::Live => {
            coverage.state(subject, RuleScope::Live);
            return Ok(coverage);
        }
        CheckScope::Hierarchy | CheckScope::Design => {}
    }
    let walked = reached_cellviews(state, subject)?;
    let reached = if scope == CheckScope::Design {
        // Whether a cellview is used is decided for every schematic of the
        // project's own: the ones the walk reached by being reached, and the
        // rest by being reported.
        for candidate in project_schematics(state) {
            coverage.state(&candidate, RuleScope::Use);
        }
        RuleScope::Every
    } else {
        RuleScope::Contents
    };
    coverage.state(subject, reached);
    for cell_view in &walked {
        coverage.state(cell_view, reached);
    }
    Ok(coverage)
}

/// The cellviews a check of `subject` descends into, `subject` among them,
/// read off the same projected design and walk the check takes.
fn reached_cellviews(state: &AppState, subject: &CellViewRef) -> Result<Vec<CellViewRef>, String> {
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let checked = projected_buffer(&projection, subject)?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_design_projection(
        state.library_manager.catalog(),
        &projection,
    );
    Ok(rspice_design::drc::check_reads(checked, subject, &hierarchy).reached)
}

/// A stable key of the object a rule finding is located at: the design
/// crate's, which a waiver is keyed by too.
pub(in crate::workbench) use rspice_design::drc::location_key;

fn owner_key(subject: &CellViewRef) -> CanonicalCellViewOwnerKey {
    canonical_cell_view_owner_key(&subject.library, &subject.cell, &subject.view)
}

impl AppState {
    /// The configuration `subject` is checked under in this project.
    ///
    /// The one authority for it. The Check command, the digest that decides
    /// whether its findings are current, preflight, check-and-save, the
    /// connectivity manager, hierarchy extraction and the run gate all take
    /// their configuration from here. Each used to build its own, so the
    /// project's rule deck changed what the Problems page said and not what
    /// the run refused, and the gate checked every root under the shipped
    /// defaults whatever the project had decided.
    ///
    /// It carries the project's rule deck and its connectivity policy — the
    /// width-mismatch rule reads the policy from here, and was reading the
    /// default — the waivers stored with the subject and every schematic it
    /// may instantiate, so a waived error is waived for the run gate as it is
    /// on the page, and asks for a ground where the subject is a design of
    /// its own.
    pub(crate) fn design_check_config(&self, subject: &CellViewRef) -> DrcConfig {
        DrcConfig {
            check_missing_ground: asks_for_ground(self, subject),
            severity_overrides: self.workspace.content.design_rules.severity_overrides(),
            connectivity: self.workspace.content.connectivity.policy.clone(),
            waivers: self.check_waivers(subject),
            ..DrcConfig::default()
        }
    }
}

/// Whether `subject` is a design of its own, which must reach a reference.
///
/// The configured simulation root always is. Any other cellview the root's
/// walk places is a cell, which takes its reference from whatever places it,
/// and so is one that presents an interface: its ground arrives through its
/// ports. One the root does not place, that presents no interface or is a
/// testbench, is a design of its own and is asked.
///
/// Read cheapest first: the common subject, a cell with ports, is decided
/// without resolving the configured hierarchy.
fn asks_for_ground(state: &AppState, subject: &CellViewRef) -> bool {
    let subject_key = owner_key(subject);
    if subject_key == owner_key(&state.workspace.content.simulation_root_reference()) {
        return true;
    }
    let testbench = state
        .library_manager
        .catalog()
        .get_library(&subject.library)
        .and_then(|library| library.get_cell(&subject.cell))
        .and_then(|cell| cell.get_view(&subject.view))
        .is_some_and(|view| view.view_type == ViewType::Testbench);
    if !testbench && subject_has_interface(state, subject) {
        return false;
    }
    // A hierarchy that does not resolve places nothing that can be found, so
    // the subject is asked: the stricter answer, where the looser one would
    // pass a design nobody could tell was grounded.
    let placed = state
        .workspace
        .inspect_design_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .is_ok_and(|projection| {
            projection
                .hierarchy_resolution()
                .bindings
                .iter()
                .any(|binding| {
                    binding.status.is_resolved() && owner_key(&binding.reference) == subject_key
                })
        });
    !placed
}

/// Whether the drawing behind `subject` declares any port, read from the live
/// editor where the subject is the view in front of the reader.
fn subject_has_interface(state: &AppState, subject: &CellViewRef) -> bool {
    let key = subject.key();
    if state
        .workspace
        .content
        .active_view
        .key()
        .eq_ignore_ascii_case(&key)
    {
        return state.schematic.document().has_interface();
    }
    state
        .workspace
        .content
        .schematic_buffers
        .iter()
        .find(|(buffer, _)| buffer.eq_ignore_ascii_case(&key))
        .is_none_or(|(_, schematic)| schematic.document().has_interface())
}

/// The digest of everything one cell view's checks read, as it stands now.
///
/// Exposed because the canonical diagnostic store decides the currency of
/// every producer by comparing input digests, and the check findings must be
/// decided by exactly the digest their receipt is decided by. Two functions
/// computing "the same" digest is how a receipt and the Problems page come to
/// disagree about whether the same findings are current.
pub(in crate::workbench) fn live_input_digest(
    state: &AppState,
    subject: &CellViewRef,
) -> Result<ContentDigest, String> {
    let config = state.design_check_config(subject);
    design_check_input_digest(state, subject, &config)
}

/// The digest of exactly what a check of `subject` reads.
///
/// The cellviews its walk reaches and every cellview an instance in them
/// names, each as the configured design projects it; how the plan bound the
/// instances the walk followed it at; the deck's global nodes; the library,
/// the connectivity contract and the rule deck. Read off the same projection
/// and the same walk the check takes, so an edit to a cellview the check
/// never read leaves its findings current, and an edit to one it read, or to
/// a binding that decided where it went, stales them.
///
/// It used to digest every schematic buffer in the project, so drawing in one
/// cellview staled the findings of every other.
fn design_check_input_digest(
    state: &AppState,
    subject: &CellViewRef,
    config: &DrcConfig,
) -> Result<ContentDigest, String> {
    if let Some(error) = state.workspace.annotation_restoration_error() {
        return Err(format!("reference annotation restoration failed: {error}"));
    }
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let buffer = |key: &str| {
        projection
            .schematic_buffers()
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
            .map(|(_, schematic)| schematic)
    };
    let checked = buffer(&subject.key()).ok_or_else(|| {
        format!(
            "{} is not part of the configured design.",
            subject.display_path()
        )
    })?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_design_projection(
        state.library_manager.catalog(),
        &projection,
    );
    let reads = rspice_design::drc::check_reads(checked, subject, &hierarchy);
    let cellviews = reads
        .cellviews
        .iter()
        .map(|key| {
            let digest = buffer(key)
                .map(|schematic| {
                    schematic
                        .content_digest()
                        .ok_or_else(|| format!("could not digest schematic {key}"))
                })
                .transpose()?;
            Ok((key.as_str(), digest))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let mut overrides = config
        .severity_overrides
        .iter()
        .map(|(kind, severity)| format!("{kind:?}:{severity:?}"))
        .collect::<Vec<_>>();
    overrides.sort();
    let profile = (
        config.check_floating_nodes,
        config.check_unconnected_pins,
        config.check_missing_ground,
        config.check_duplicate_names,
        config.check_component_parameters,
        config.check_unknown_components,
        config.check_shorted_outputs,
        config.min_connections,
        overrides,
    );
    // Each cellview is digested as projected, not as stored: a page
    // assignment or a variant changes what was checked without touching a
    // single schematic, and only the cellviews read carry theirs in. The
    // configuration enters through the bindings it decided.
    let material = serde_json::to_vec(&(
        state.workspace.content.project.id(),
        owner_key(subject).to_string(),
        cellviews,
        &reads.bindings,
        &reads.globals,
        state.library_manager.revision(),
        projection.connectivity(),
        profile,
    ))
    .map_err(|error| format!("could not encode design-check inputs: {error}"))?;
    let mut digest = Sha256::new();
    digest.update(DESIGN_CHECK_DIGEST_DOMAIN);
    digest.update(material);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::drc::DrcLocation;

    fn completed_result() -> DrcResult {
        let mut result = DrcResult::new();
        result.completed = true;
        result
    }

    fn open_pin(component_id: u64, component: &str, pin: &str) -> DrcLocation {
        DrcLocation::Terminal {
            component_id,
            component_name: component.to_owned(),
            pin_name: pin.to_owned(),
            point: crate::state::Point::new(0, 0),
        }
    }

    /// Publish one completed pass of the active view's checks, in the order
    /// given — which is the order the checks would have reported them in.
    fn publish_pass(state: &mut AppState, findings: &[(DrcLocation, &str)]) {
        let mut result = DrcResult::new();
        for (ordinal, (location, message)) in findings.iter().enumerate() {
            result.add_violation(rspice_design::drc::DrcViolation::new(
                ordinal + 1,
                rspice_design::drc::DrcViolationType::UnconnectedPin,
                *message,
                location.clone(),
            ));
        }
        result.completed = true;
        state
            .publish_active_design_check_result(result)
            .expect("the pass publishes");
    }

    /// The identity the published batch gives the finding about `object`.
    fn identity_of(state: &AppState, object: &str) -> uuid::Uuid {
        let subject = state.workspace.content.active_schematic_reference();
        let published = DiagnosticInputs::DesignCheck { subject };
        state
            .diagnostics
            .batches()
            .find(|batch| batch.inputs == published)
            .and_then(|batch| {
                batch
                    .records
                    .iter()
                    .find(|record| record.subject.object == object)
            })
            .map(|record| record.canonical.diagnostic_id)
            .unwrap_or_else(|| panic!("a finding about {object} is published"))
    }

    /// An edit elsewhere moves everything an identity used to be minted
    /// from — the project revision, the finding's ordinal, its wording — and
    /// none of it is what the finding is about.
    #[test]
    fn an_unrelated_edit_and_a_recheck_keep_the_current_finding() {
        let mut state = AppState::default();
        publish_pass(
            &mut state,
            &[
                (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
                (open_pin(7, "R2", "B"), "Pin R2.B is not connected"),
            ],
        );
        let standing = identity_of(&state, "Pin R2.B");
        state.diagnostics.set_current_finding(Some(standing));

        state
            .workspace
            .content
            .project
            .advance_revision()
            .expect("the fixture revision advances");
        publish_pass(
            &mut state,
            &[
                (open_pin(2, "C1", "A"), "Pin C1.A is not connected"),
                (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
                (open_pin(7, "R2", "B"), "Pin R2.B has nothing attached"),
            ],
        );

        assert_eq!(identity_of(&state, "Pin R2.B"), standing);
        assert_eq!(
            state.diagnostics.current_finding(),
            Some(standing),
            "the shared current finding survives the re-check",
        );
    }

    /// A finding re-reported in other words — the rule's message reworded,
    /// or a value it quotes changed — is the same finding. Its identity is
    /// the rule, the cell view and the object, and nothing the message says
    /// is part of it, so the current selection stays on it and so does a
    /// waiver recorded against it. An identity hashed from the message, as
    /// the source editors' own identities are, would have re-keyed it.
    #[test]
    fn rewording_a_finding_keeps_its_identity_its_selection_and_its_waiver() {
        let mut state = AppState::default();
        publish_pass(
            &mut state,
            &[
                (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
                (open_pin(7, "R2", "B"), "Pin R2.B is not connected"),
            ],
        );
        let identity = identity_of(&state, "Pin R2.B");
        state.diagnostics.set_current_finding(Some(identity));
        // A waiver is recorded against a finding by what the finding is.
        let subject = state.workspace.content.active_schematic_reference();
        let waived = |state: &AppState| {
            state
                .diagnostics
                .batch(&DiagnosticInputs::DesignCheck {
                    subject: subject.clone(),
                })
                .and_then(|batch| {
                    batch
                        .records
                        .iter()
                        .find(|record| record.canonical.diagnostic_id == identity)
                })
                .map(|record| (record.key.clone(), record.message.clone()))
        };
        let (waiver_key, _) = waived(&state).expect("the finding is published");

        publish_pass(
            &mut state,
            &[
                (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
                (
                    open_pin(7, "R2", "B"),
                    "Terminal B of R2 has no connection (2 nets within 10 grid units)",
                ),
            ],
        );

        assert_eq!(identity_of(&state, "Pin R2.B"), identity);
        assert_eq!(
            state.diagnostics.current_finding(),
            Some(identity),
            "the current selection stays on the reworded finding",
        );
        let (key, message) = waived(&state).expect("the waived identity still resolves");
        assert_eq!(key, waiver_key, "what a waiver names is unchanged");
        assert_eq!(
            message, "Terminal B of R2 has no connection (2 nets within 10 grid units)",
            "the finding states its new words under its old identity",
        );
    }

    /// Fixing one finding takes its identity away with it. With an ordinal in
    /// the identity, the next finding of the same rule inherited it — and a
    /// selection, or a waiver, on the fixed finding moved to that one.
    #[test]
    fn fixing_one_finding_does_not_hand_its_identity_to_another() {
        let mut state = AppState::default();
        publish_pass(
            &mut state,
            &[
                (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
                (open_pin(7, "R2", "B"), "Pin R2.B is not connected"),
            ],
        );
        let fixed = identity_of(&state, "Pin R1.A");
        let remaining = identity_of(&state, "Pin R2.B");
        state.diagnostics.set_current_finding(Some(fixed));

        publish_pass(
            &mut state,
            &[(open_pin(7, "R2", "B"), "Pin R2.B is not connected")],
        );

        let subject = state.workspace.content.active_schematic_reference();
        assert_eq!(
            state.design_rule_finding_identities(&subject),
            vec![remaining],
            "the remaining finding keeps its identity and the fixed one's is gone",
        );
        assert_ne!(
            state.diagnostics.current_finding(),
            Some(remaining),
            "a selection on the fixed finding does not move to another finding",
        );
    }

    /// Two findings about one object are told apart by the order they were
    /// reported in. When the first goes, the second takes its occurrence —
    /// and a selection on the second follows it there rather than being lost.
    #[test]
    fn a_selection_follows_its_finding_when_its_occurrence_moves() {
        let mut state = AppState::default();
        publish_pass(
            &mut state,
            &[
                (open_pin(4, "R1", "A"), "first report"),
                (open_pin(4, "R1", "A"), "second report"),
            ],
        );
        let subject = state.workspace.content.active_schematic_reference();
        let identities = state.design_rule_finding_identities(&subject);
        assert_eq!(identities.len(), 2);
        assert_ne!(identities[0], identities[1]);
        state.diagnostics.set_current_finding(Some(identities[1]));

        publish_pass(&mut state, &[(open_pin(4, "R1", "A"), "second report")]);

        let remaining = state.design_rule_finding_identities(&subject);
        assert_eq!(remaining, vec![identities[0]]);
        assert_eq!(state.diagnostics.current_finding(), Some(remaining[0]));
    }

    /// The store can refuse a batch, so a receipt is stored only once the
    /// store has taken the findings it describes. Stored first, it put
    /// findings on the canvas and the check pill that the page did not list.
    #[test]
    fn a_refused_publication_stores_no_receipt() {
        let mut state = AppState::default();
        let subject = state.workspace.content.active_schematic_reference();
        let location = open_pin(4, "R1", "A");
        // A record that already holds the identity this pass will mint.
        let key = FindingKey::new(
            DESIGN_RULE_PRODUCER,
            rspice_design::drc::DrcViolationType::UnconnectedPin.rule_code(),
            subject.key(),
            location_key(&location),
        );
        let squatter = DiagnosticRecord::new(
            mint_canonical(&[key], 0, "elsewhere").remove(0),
            crate::workbench::app_state::diagnostic_store::DiagnosticLevel::Error,
            DiagnosticSubject::new(state.workspace.content.project.id(), "elsewhere"),
            "elsewhere",
        );
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, vec![squatter])
            .expect("the squatting record publishes");

        let mut result = DrcResult::new();
        result.add_violation(rspice_design::drc::DrcViolation::new(
            1,
            rspice_design::drc::DrcViolationType::UnconnectedPin,
            "Pin R1.A is not connected",
            location,
        ));
        result.completed = true;
        let error = state
            .publish_active_design_check_result(result)
            .expect_err("the store refuses a second holder of one identity");

        assert!(error.contains("duplicate canonical ID"), "{error}");
        assert!(
            matches!(
                state.active_design_check_status(),
                DesignCheckStatus::NotRun
            ),
            "a refused publication leaves no receipt behind"
        );
        assert!(state.dialogs.drc_results.is_none());
    }

    #[test]
    fn receipts_never_alias_another_active_cell_view() {
        let mut state = AppState::default();
        let root = state.workspace.content.active_schematic_reference();
        let other = CellViewRef::new(&root.library, "aux", &root.view);
        state
            .workspace
            .insert_schematic_editor(other.key(), crate::state::SchematicState::default());
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish root receipt");

        state
            .workspace
            .insert_schematic_editor(root.key(), state.schematic.clone());
        state.workspace.content.active_view = other;
        state.schematic = crate::state::SchematicState::default();
        state.refresh_active_design_check_projection();

        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::NotRun
        ));
        assert!(state.dialogs.drc_results.is_none());
        assert_eq!(state.dialogs.drc_checked_version, 0);
        assert!(matches!(
            state.design_check_status(&root),
            DesignCheckStatus::Current(_)
        ));
    }

    #[test]
    fn publishing_an_active_receipt_refreshes_the_canvas_projection() {
        let mut state = AppState::default();
        let result = completed_result();

        state
            .publish_active_design_check_result(result)
            .expect("publish active receipt");

        assert!(
            state
                .dialogs
                .drc_results
                .as_ref()
                .is_some_and(|projected| projected.completed)
        );
        assert_eq!(
            state.dialogs.drc_checked_version,
            state.schematic.topology_version()
        );
    }

    #[test]
    fn connectivity_changes_stale_previously_current_receipts() {
        let mut state = AppState::default();
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish current receipt");
        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::Current(_)
        ));

        state.workspace.content.connectivity.policy.width_mismatch =
            crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;

        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::Stale(_)
        ));
    }

    #[test]
    fn incomplete_results_can_never_become_current_evidence() {
        let mut state = AppState::default();

        let error = state
            .publish_active_design_check_result(DrcResult::new())
            .expect_err("incomplete result must be rejected");

        assert!(error.contains("incomplete"));
        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::NotRun
        ));
        assert!(state.dialogs.drc_results.is_none());
    }

    #[test]
    fn focusing_a_non_schematic_view_does_not_stale_root_evidence() {
        let mut state = AppState::default();
        let root = state.workspace.content.active_view.clone();
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish root receipt");
        state
            .workspace
            .insert_schematic_editor(root.key(), state.schematic.clone());
        let layout = CellViewRef::new(&root.library, &root.cell, "layout");
        state
            .workspace
            .content
            .open_views
            .push(crate::state::OpenCellView::new(
                layout.clone(),
                ViewType::Layout,
            ));
        state.workspace.content.active_view = layout;
        state.schematic = crate::state::SchematicState::default();

        assert!(matches!(
            state.design_check_status(&root),
            DesignCheckStatus::Current(_)
        ));
    }

    #[test]
    fn symbol_focus_never_aliases_or_clears_its_sibling_schematic_receipt() {
        let mut state = AppState::default();
        let root = state.workspace.content.active_view.clone();
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish root receipt");
        let symbol = CellViewRef::new(&root.library, &root.cell, "symbol");
        state
            .workspace
            .content
            .open_views
            .push(crate::state::OpenCellView::new(
                symbol.clone(),
                ViewType::Symbol,
            ));
        state.workspace.content.active_view = symbol;

        state.refresh_active_design_check_projection();

        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::NotRun
        ));
        assert!(state.dialogs.drc_results.is_none());
        assert!(
            state
                .publish_active_design_check_result(completed_result())
                .is_err()
        );
        state.clear_active_design_check();
        assert!(matches!(
            state.design_check_status(&root),
            DesignCheckStatus::Current(_)
        ));
    }

    /// Two electrically separate conductors drawn at exactly the same
    /// coordinates on two governed sheets.
    ///
    /// The editor buffer holds one coordinate space, so anything read off it
    /// merges the two pages into one net. The projection namespaces the pages
    /// apart before anything electrical is read, which is the whole reason
    /// design checks must not read the buffer.
    fn state_with_two_coincident_sheets() -> AppState {
        const FIRST_WIRE: u64 = 101;
        const SECOND_WIRE: u64 = 102;

        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::segment(
                FIRST_WIRE,
                crate::state::Point::new(0, 0),
                crate::state::Point::new(40, 0),
            ));
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::segment(
                SECOND_WIRE,
                crate::state::Point::new(0, 0),
                crate::state::Point::new(40, 0),
            ));
        state.sync_active_schematic_to_workspace();

        let key = state.workspace.content.active_schematic_reference().key();
        let first = state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Page 1", [FIRST_WIRE])
            .expect("a fresh cell view accepts its first governed sheet");
        let catalog = state
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("the sheet catalog was just created");
        let second = catalog
            .create_sheet(
                crate::state::SheetDefinition {
                    name: "Page 2".to_owned(),
                    template: crate::state::SheetTemplate::AnalogSchematic,
                    port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .expect("a second governed sheet inserts after the first");
        catalog
            .assign_objects(catalog.revision(), second, [SECOND_WIRE])
            .expect("the second conductor belongs to the second page");
        state
    }

    #[test]
    fn design_checks_read_the_projection_so_coincident_pages_stay_two_nets() {
        let mut state = state_with_two_coincident_sheets();
        assert_eq!(
            rspice_design::connectivity::summary::design_nets(&state.schematic).len(),
            1,
            "the editor buffer holds one coordinate space, so its pages overlap"
        );

        let projection = state
            .workspace
            .design_projection(
                &state.library_manager,
                &state.workspace.content.active_view,
                &state.schematic,
            )
            .expect("the fixture configuration resolves");
        assert_eq!(
            rspice_design::connectivity::summary::projection_nets(
                state.library_manager.catalog(),
                &projection,
                &state.workspace.content.active_view.key(),
            )
            .len(),
            2,
            "the projection namespaces the two pages apart"
        );

        let result = state
            .run_active_design_checks()
            .expect("the checks run over the projected design");
        assert!(result.completed);
    }

    #[test]
    fn design_checks_state_an_unresolved_configuration_rather_than_checking_the_buffer() {
        let mut state = AppState::default();
        let root = state.workspace.content.active_view.clone();
        state
            .workspace
            .content
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Unresolvable DUT".to_owned(),
                root,
                dut_path: "/XABSENT".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "projection consumer test".to_owned(),
            })
            .expect("the fixture configuration is well formed");

        let reason = state
            .run_active_design_checks()
            .expect_err("an unresolvable configuration cannot be checked");
        assert!(reason.contains("XABSENT"), "{reason}");
        assert!(
            matches!(
                state.active_design_check_status(),
                DesignCheckStatus::NotRun
            ),
            "a refused check publishes no evidence"
        );
    }
}
