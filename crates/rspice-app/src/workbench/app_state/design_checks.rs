//! Runtime design-check evidence keyed by exact project cell/view identity.
//!
//! Interactive checks are derived evidence: they do not dirty project bytes or
//! masquerade as a validated save. Each receipt binds the checker profile and
//! the complete live schematic working set, so switching documents can never
//! relabel another cell's findings as current.

use std::collections::BTreeMap;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::product::{ContentDigest, ProjectId};
use crate::state::{
    CanonicalCellViewOwnerKey, CellViewRef, ViewType, canonical_cell_view_owner_key,
};
use rspice_design::drc::{DrcConfig, DrcLocation, DrcResult};

use super::AppState;
use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject, FindingKey,
    design_rule_code, mint_canonical,
};

/// Producer identity every schematic and electrical rule finding carries.
const DESIGN_RULE_PRODUCER: &str = "rspice.design.rules";

const DESIGN_CHECK_DIGEST_DOMAIN: &[u8] = b"rspice-interactive-design-check/v1\0";

#[derive(Debug, Clone)]
pub(crate) struct CellViewCheckReceipt {
    pub(crate) project_id: ProjectId,
    pub(crate) subject: CellViewRef,
    pub(crate) input_digest: ContentDigest,
    pub(crate) result: DrcResult,
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
}

impl DesignCheckRuntime {
    fn receipt(&self, subject: &CellViewRef) -> Option<&CellViewCheckReceipt> {
        self.receipts.get(&owner_key(subject))
    }

    fn insert(&mut self, receipt: CellViewCheckReceipt) {
        self.receipts.insert(owner_key(&receipt.subject), receipt);
    }

    pub(crate) fn clear(&mut self) {
        self.receipts.clear();
    }

    fn clear_subject(&mut self, subject: &CellViewRef) {
        self.receipts.remove(&owner_key(subject));
    }
}

impl AppState {
    pub(crate) fn run_active_design_checks(&mut self) -> Result<DrcResult, String> {
        if self.workspace.annotation_restoration_error().is_some() {
            self.sync_active_schematic_to_workspace();
        }
        let subject = self.workspace.content.active_schematic_reference();
        let config = design_check_config(self, &subject);
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
        let checked = projection
            .schematic_buffers()
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&subject.key()))
            .map(|(_, schematic)| schematic)
            .ok_or_else(|| {
                format!(
                    "{} is not part of the configured design.",
                    subject.display_path()
                )
            })?;
        let hierarchy = rspice_design::hierarchy::HierarchySource::from_design_projection(
            self.library_manager.catalog(),
            &projection,
        );
        let result =
            rspice_design::drc::run_check_with_hierarchy(checked, &hierarchy, config.clone());
        self.publish_design_check_result(subject, config, result.clone())?;
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
        let config = design_check_config(self, &subject);
        self.publish_design_check_result(subject, config, result)
    }

    fn publish_design_check_result(
        &mut self,
        subject: CellViewRef,
        config: DrcConfig,
        result: DrcResult,
    ) -> Result<(), String> {
        if !result.completed {
            return Err(
                "an incomplete design-check result cannot become current evidence".to_owned(),
            );
        }
        let is_active_subject =
            owner_key(&subject) == owner_key(&self.workspace.content.active_schematic_reference());
        let input_digest = design_check_input_digest(self, &subject, &config)?;
        // Canonical records are derived before the receipt is stored, because
        // one of them needs to ask the live drawing where its object is and
        // the receipt's own insertion takes the state mutably.
        let records = design_rule_records(self, &subject, &result, is_active_subject);
        self.design_checks.insert(CellViewCheckReceipt {
            project_id: self.workspace.content.project.id(),
            subject: subject.clone(),
            input_digest,
            result,
        });
        self.publish_diagnostics(
            DiagnosticInputs::DesignCheck {
                subject: subject.clone(),
            },
            records,
        )?;
        if is_active_subject {
            self.refresh_active_design_check_projection();
        }
        Ok(())
    }

    /// What the published rule findings for `subject` are called, in the order
    /// the checks reported them.
    ///
    /// The order is [`DrcResult::violations`]' own, because the records are
    /// derived from it one for one — which is what lets a surface holding the
    /// result name the finding at an ordinal without recomputing an identity.
    /// Each identity is structural: it names the object the finding is about,
    /// so an unrelated edit and a re-check name the same finding the same
    /// way.
    ///
    /// Empty where these checks published no records for `subject`. A symbol
    /// view's findings are never among them: they are the pin checks', owned
    /// by [`DiagnosticInputs::SymbolPins`] and named there.
    pub(crate) fn design_rule_finding_identities(&self, subject: &CellViewRef) -> Vec<uuid::Uuid> {
        let published = DiagnosticInputs::DesignCheck {
            subject: subject.clone(),
        };
        self.diagnostics
            .batches()
            .find(|batch| batch.inputs == published)
            .map(|batch| {
                batch
                    .records
                    .iter()
                    .map(|record| record.canonical.diagnostic_id)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub(crate) fn design_check_status(&self, subject: &CellViewRef) -> DesignCheckStatus<'_> {
        let last = self.design_checks.receipt(subject);
        let Some(receipt) = last else {
            return DesignCheckStatus::NotRun;
        };
        if receipt.project_id != self.workspace.content.project.id() {
            return DesignCheckStatus::Stale(receipt);
        }
        let config = design_check_config(self, subject);
        match design_check_input_digest(self, subject, &config) {
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

    pub(crate) fn project_root_design_check_status(&self) -> DesignCheckStatus<'_> {
        self.design_check_status(&project_root_reference(self))
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
        self.withdraw_diagnostics(&DiagnosticInputs::DesignCheck { subject });
    }

    pub(crate) fn clear_all_design_checks(&mut self) {
        let subjects: Vec<CellViewRef> = self
            .design_checks
            .receipts
            .values()
            .map(|receipt| receipt.subject.clone())
            .collect();
        self.design_checks.clear();
        for subject in subjects {
            self.withdraw_diagnostics(&DiagnosticInputs::DesignCheck { subject });
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
        let current = match self.active_design_check_status() {
            DesignCheckStatus::Current(receipt) => Some(receipt.result.clone()),
            DesignCheckStatus::NotRun
            | DesignCheckStatus::Stale(_)
            | DesignCheckStatus::Unavailable { .. } => None,
        };
        self.dialogs.drc_checked_version = current
            .as_ref()
            .map_or(0, |_| self.schematic.topology_version());
        self.dialogs.drc_results = current;
    }
}

/// One canonical record per rule finding.
///
/// `drawn` says whether the subject is the drawing in front of the reader.
/// Only that drawing can be asked where an object is, so a finding in another
/// cell view is published without a route rather than with one resolved
/// against the wrong coordinate space — which is what a shared canvas
/// projection would have produced.
fn design_rule_records(
    state: &AppState,
    subject: &CellViewRef,
    result: &DrcResult,
    drawn: bool,
) -> Vec<DiagnosticRecord> {
    let project = state.workspace.content.project.id();
    let revision = state.workspace.content.project.revision().get();
    let document_id = subject.key();
    let producer: Arc<str> = Arc::from(DESIGN_RULE_PRODUCER);
    let document: Arc<str> = Arc::from(document_id.as_str());
    let keys: Vec<FindingKey> = result
        .violations()
        .iter()
        .map(|violation| {
            FindingKey::new(
                Arc::clone(&producer),
                design_rule_code(violation.violation_type),
                Arc::clone(&document),
                location_key(&violation.location),
            )
        })
        .collect();
    let canonicals = mint_canonical(&keys, revision, &format!("design-check:{document_id}"));
    result
        .violations()
        .iter()
        .zip(keys)
        .zip(canonicals)
        .map(|((violation, key), canonical)| {
            let route = drawn
                .then(|| {
                    crate::schematic::view::violations::location_anchor(state, &violation.location)
                })
                .flatten()
                .map(DiagnosticRoute::Anchor);
            DiagnosticRecord::new(
                canonical,
                violation.severity.into(),
                DiagnosticSubject::new(project, violation.location.display())
                    .in_cell_view(subject.clone()),
                violation.message.clone(),
            )
            .keyed(key)
            .with_details(violation.violation_type.suggested_fix())
            .with_rule(violation.violation_type.description())
            .with_route(route)
        })
        .collect()
}

/// A stable key of the object a rule finding is located at.
///
/// What survives an unrelated edit is the object's own identity, so that is
/// what the key is: a component or wire by its id, a terminal by its
/// component and pin, a net or a label by its name. Where the object is drawn
/// and what the message says about it are not part of it. A finding inside an
/// instantiated cell view is keyed by the occurrence path down to that view
/// and its own key inside it, which is what tells `X1/M3` from `X2/M3`. A
/// point is the last resort, for a finding that names no object at all.
pub(in crate::workbench) fn location_key(location: &DrcLocation) -> String {
    match location {
        DrcLocation::Terminal {
            component_id,
            pin_name,
            ..
        } => format!("terminal:{component_id}:{pin_name}"),
        DrcLocation::Component { id, .. } => format!("component:{id}"),
        DrcLocation::Node { net_name } => format!("net:{net_name}"),
        DrcLocation::Wire { id } => format!("wire:{id}"),
        DrcLocation::NetLabel { name } => format!("label:{name}"),
        DrcLocation::Bus { id } => format!("bus:{id}"),
        DrcLocation::BusTap { id } => format!("bus-tap:{id}"),
        DrcLocation::InsideInstance { path, view, inner } => {
            // Length-prefixed, because an instance name or a net name inside
            // may contain any separator chosen here.
            let path = path.to_string();
            let view = view.key();
            let inner = location_key(inner);
            format!(
                "instance:{}:{path}{}:{view}{}:{inner}",
                path.len(),
                view.len(),
                inner.len()
            )
        }
        DrcLocation::Global => "global".to_owned(),
        DrcLocation::SymbolPin {
            reference,
            pin_name,
            ..
        } => format!("symbol-pin:{}:{pin_name}", reference.key()),
        DrcLocation::Point { x, y } => format!("point:{x}:{y}"),
    }
}

fn owner_key(subject: &CellViewRef) -> CanonicalCellViewOwnerKey {
    canonical_cell_view_owner_key(&subject.library, &subject.cell, &subject.view)
}

fn project_root_reference(state: &AppState) -> CellViewRef {
    CellViewRef::new(
        &state.workspace.content.project.root_library,
        &state.workspace.content.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    )
}

fn design_check_config(state: &AppState, subject: &CellViewRef) -> DrcConfig {
    DrcConfig {
        check_missing_ground: owner_key(subject) == owner_key(&project_root_reference(state)),
        ..DrcConfig::default()
    }
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
    let config = design_check_config(state, subject);
    design_check_input_digest(state, subject, &config)
}

fn design_check_input_digest(
    state: &AppState,
    subject: &CellViewRef,
    config: &DrcConfig,
) -> Result<ContentDigest, String> {
    if let Some(error) = state.workspace.annotation_restoration_error() {
        return Err(format!("reference annotation restoration failed: {error}"));
    }
    let mut live_buffers = state
        .workspace
        .content
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key.clone(), schematic))
        .collect::<std::collections::HashMap<_, _>>();
    if matches!(
        state.workspace.content.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        live_buffers.insert(
            state.workspace.content.active_view.key(),
            state.schematic.editor_ref().design,
        );
    }
    let mut schematic_digests = live_buffers
        .iter()
        .map(|(key, schematic)| {
            let content_digest = schematic
                .validated_design_content_digest()
                .map_err(|error| format!("could not digest schematic {key}: {error}"))?;
            Ok((key.clone(), content_digest))
        })
        .collect::<Result<Vec<_>, String>>()?;
    schematic_digests.sort_by(|left, right| left.0.cmp(&right.0));

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
    // Design management and the active configuration are inputs because the
    // checks read the projection, not the source documents: a page assignment
    // or a scoped override changes what was checked without touching a single
    // schematic. A receipt that ignored them would read "current" for a design
    // nobody checked.
    let material = serde_json::to_vec(&(
        state.workspace.content.project.id(),
        owner_key(subject).to_string(),
        schematic_digests,
        state.library_manager.revision(),
        &state.workspace.content.connectivity,
        &state.workspace.content.design_management,
        &state.workspace.content.configuration_sets,
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
