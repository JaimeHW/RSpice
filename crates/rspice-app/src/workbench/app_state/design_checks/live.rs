//! Live checking: the rules a drawing alone decides, run on the cellview an
//! author is editing once each edit settles.
//!
//! A live pass is published beside the explicit checks with the coverage of
//! those rules in that cellview, so it takes over their statements of the
//! same findings and they take them back when they run: the newest look at a
//! finding is the one listed, under the one identity its rule and object
//! give it. Waivers apply to it as to any pass.

use std::time::Duration;

use sha2::{Digest, Sha256};

use super::{
    AppState, CellViewCheckReceipt, CheckScope, DESIGN_CHECK_DIGEST_DOMAIN, DiagnosticInputs,
    RuleCoverage, StoredWaivers, check_coverage, design_rule_records, owner_key, projected_buffer,
};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::product::ContentDigest;
use crate::state::{CanonicalCellViewOwnerKey, CellViewRef, ViewType};
use crate::time_compat::Instant;
use crate::workbench::TogglePreference;
use rspice_design::drc::{DrcConfig, DrcResult};

/// How long a drawing stands still before it is checked, so a drag is
/// checked where it ends rather than at every step of it.
const SETTLE: Duration = Duration::from_millis(250);

/// Where the drawing in front of the reader stands against its live check.
#[derive(Debug, Clone, Default)]
pub(super) struct DrawingState {
    /// The drawing seen on the last frame, by cellview and stamp.
    seen: Option<(CanonicalCellViewOwnerKey, u64)>,
    /// An edit not yet checked: the cellview it was made in, and when the
    /// drawing last moved.
    pending: Option<(CellViewRef, Instant)>,
    /// Why the last live check could not run, until one runs.
    failed: Option<String>,
}

impl AppState {
    /// Keep the live findings of the drawing in front of the reader current.
    ///
    /// Asked every frame. A frame with no edit costs one comparison of
    /// stamps; an edit is checked once the drawing has stood still for
    /// [`SETTLE`], and while it waits this says how long until it should be
    /// asked again. Opening a cellview is not an edit and checks nothing.
    /// Leaving one with an edit still settling checks it on the way out.
    pub(in crate::workbench) fn refresh_live_checks(&mut self, now: Instant) -> Option<Duration> {
        if !self
            .ui
            .preferences
            .toggle(TogglePreference::IncrementalConnectivityChecks)
        {
            self.withdraw_live_checks();
            return None;
        }
        let drawing = (self.project_lifecycle.is_open()
            && matches!(
                self.workspace.content.active_view_type(),
                ViewType::Schematic | ViewType::Testbench
            ))
        .then(|| {
            let subject = self.workspace.content.active_schematic_reference();
            (owner_key(&subject), self.schematic.content_stamp(), subject)
        });
        let state = &mut self.design_checks.drawing;
        let seen = drawing
            .as_ref()
            .map(|(key, stamp, _)| (key.clone(), *stamp));
        let previous = std::mem::replace(&mut state.seen, seen);
        if let Some((key, stamp, subject)) = &drawing
            && previous
                .as_ref()
                .is_some_and(|(was, then)| was == key && then != stamp)
        {
            state.pending = Some((subject.clone(), now));
        }
        let (subject, since) = state.pending.clone()?;
        let still_in_front = drawing
            .as_ref()
            .is_some_and(|(key, _, _)| *key == owner_key(&subject));
        let waited = now.saturating_duration_since(since);
        if still_in_front && waited < SETTLE {
            return Some(SETTLE - waited);
        }
        self.design_checks.drawing.pending = None;
        // A failed background job is an Error, and the Console lifts it into
        // a notice. A drawing that cannot be checked — a configuration that
        // does not resolve under it — fails the same way at every edit, so
        // it is recorded when the reason changes, not each time.
        match self.run_live_check(subject) {
            Ok(_) => self.design_checks.drawing.failed = None,
            Err(error) if self.design_checks.drawing.failed.as_ref() == Some(&error) => {}
            Err(error) => {
                self.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!("Live checks could not run: {error}"),
                );
                self.design_checks.drawing.failed = Some(error);
            }
        }
        None
    }

    /// Check one cellview's drawing against the live rules, and publish it.
    pub(in crate::workbench) fn run_live_check(
        &mut self,
        subject: CellViewRef,
    ) -> Result<DrcResult, String> {
        let config = self.live_check_config(&subject);
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
        let result = rspice_design::drc::run_live_check(checked, &subject, &hierarchy, &config);
        self.publish_live_check_result(subject, &config, result.clone())?;
        Ok(result)
    }

    /// What the live rules are checked under: the deck, the connectivity
    /// policy and the waivers a check of the cellview uses. Whether it must
    /// reach a ground is not asked, because no live rule asks it.
    fn live_check_config(&self, subject: &CellViewRef) -> DrcConfig {
        DrcConfig {
            severity_overrides: self.workspace.content.design_rules.severity_overrides(),
            connectivity: self.workspace.content.connectivity.policy.clone(),
            waivers: self.check_waivers(subject),
            ..DrcConfig::default()
        }
    }

    fn publish_live_check_result(
        &mut self,
        subject: CellViewRef,
        config: &DrcConfig,
        mut result: DrcResult,
    ) -> Result<(), String> {
        config.waivers.apply(&mut result);
        let inputs = DiagnosticInputs::LiveCheck {
            subject: subject.clone(),
        };
        let input_digest = self.diagnostic_live_digest(&inputs)?;
        let records = design_rule_records(self, &subject, &result, StoredWaivers::Left);
        let coverage: RuleCoverage = check_coverage(self, &subject, CheckScope::Live)?;
        self.publish_superseding(inputs, records, &coverage)?;
        let is_active_subject =
            owner_key(&subject) == owner_key(&self.workspace.content.active_schematic_reference());
        self.design_checks.insert_live(CellViewCheckReceipt {
            project_id: self.workspace.content.project.id(),
            subject,
            input_digest,
            result,
            scope: CheckScope::Live,
            sequence: 0,
        });
        if is_active_subject {
            self.refresh_active_design_check_projection();
        }
        Ok(())
    }

    /// The live pass of `subject`, while it still describes the drawing.
    pub(super) fn current_live_receipt(
        &self,
        subject: &CellViewRef,
    ) -> Option<&CellViewCheckReceipt> {
        let receipt = self.design_checks.live.get(&owner_key(subject))?;
        let current = receipt.project_id == self.workspace.content.project.id()
            && self
                .diagnostic_live_digest(&DiagnosticInputs::LiveCheck {
                    subject: subject.clone(),
                })
                .is_ok_and(|digest| digest == receipt.input_digest);
        current.then_some(receipt)
    }

    /// State a live pass again against the waivers as they now stand, where
    /// it still describes the drawing.
    pub(super) fn restate_live_check(&mut self, subject: CellViewRef) {
        let Some(receipt) = self.current_live_receipt(&subject) else {
            return;
        };
        let result = receipt.result.clone();
        let config = self.live_check_config(&subject);
        if let Err(error) = self.publish_live_check_result(subject, &config, result) {
            self.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("Findings could not be stated again against the waivers: {error}"),
            );
        }
    }

    /// Withdraw every live pass, for live checking turned off: its findings
    /// would otherwise stand on the list with nothing keeping them current.
    fn withdraw_live_checks(&mut self) {
        if self.design_checks.live.is_empty() {
            return;
        }
        let subjects = std::mem::take(&mut self.design_checks.live)
            .into_values()
            .map(|receipt| receipt.subject)
            .collect::<Vec<_>>();
        self.design_checks.drawing = DrawingState::default();
        for subject in subjects {
            self.withdraw_diagnostics(&DiagnosticInputs::LiveCheck { subject });
        }
        self.refresh_active_design_check_projection();
    }
}

/// The digest of what a live check of `subject` reads: its drawing as the
/// configured design projects it, the cellview every instance in it names —
/// whose interface decides what its pins are — the deck's global nodes, the
/// library, the connectivity contract and the rule deck.
///
/// Narrower than a check of the cellview's, which reads the whole walk below
/// it: an edit two levels down changes nothing a live finding says.
pub(in crate::workbench) fn live_check_digest(
    state: &AppState,
    subject: &CellViewRef,
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
    let digest_of = |key: &str| -> Result<Option<ContentDigest>, String> {
        projection
            .schematic_buffers()
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
            .map(|(_, schematic)| {
                schematic
                    .content_digest()
                    .ok_or_else(|| format!("could not digest schematic {key}"))
            })
            .transpose()
    };
    let checked = projected_buffer(&projection, subject)?;
    let drawing = digest_of(&subject.key())?;
    let masters = rspice_design::drc::live_check_reads(checked)
        .into_iter()
        .map(|key| Ok((digest_of(&key)?, key)))
        .collect::<Result<Vec<_>, String>>()?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_design_projection(
        state.library_manager.catalog(),
        &projection,
    );
    let mut globals = hierarchy
        .global_net_names()
        .into_iter()
        .map(|name| name.to_ascii_lowercase())
        .collect::<Vec<_>>();
    globals.sort();
    let mut overrides = state
        .workspace
        .content
        .design_rules
        .severity_overrides()
        .iter()
        .map(|(kind, severity)| format!("{kind:?}:{severity:?}"))
        .collect::<Vec<_>>();
    overrides.sort();
    let material = serde_json::to_vec(&(
        "live",
        state.workspace.content.project.id(),
        owner_key(subject).to_string(),
        drawing,
        masters,
        globals,
        state.library_manager.revision(),
        projection.connectivity(),
        overrides,
    ))
    .map_err(|error| format!("could not encode live-check inputs: {error}"))?;
    let mut digest = Sha256::new();
    digest.update(DESIGN_CHECK_DIGEST_DOMAIN);
    digest.update(material);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[cfg(test)]
mod tests;
