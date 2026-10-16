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
use super::waivers;
use super::diagnostic_store::{
    DiagnosticInputs, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject, FindingKey,
    mint_canonical,
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
        let config = self.design_check_config(&subject);
        self.publish_design_check_result(subject, config, result)
    }

    /// The configuration the design rules are checked under for the
    /// configured simulation root — the profile the Check command uses for
    /// that cell view, so a pass run by either is the same evidence, and the
    /// one the run gate judges the root under.
    pub(crate) fn configured_root_design_check_config(&self) -> DrcConfig {
        self.design_check_config(&self.workspace.content.simulation_root_reference())
    }

    /// Publish the design-rule findings of the configured simulation root, as
    /// the dispatch checks found them.
    ///
    /// Through the one producer the Check command publishes through, with the
    /// root as the subject: the same records, the same identities, the same
    /// digest and the same receipt. The dispatch checks used to re-file every
    /// error as a blocker and every warning as an advisory beside the Check
    /// command's own findings, so the page listed each one twice.
    pub(in crate::workbench) fn publish_configured_root_design_check_result(
        &mut self,
        result: DrcResult,
    ) -> Result<(), String> {
        let subject = self.workspace.content.simulation_root_reference();
        let config = self.design_check_config(&subject);
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
        // Canonical records are derived before anything is stored, because
        // one of them needs to ask the live drawing where its object is and
        // both insertions take the state mutably.
        let records = design_rule_records(self, &subject, &result);
        // Published before the receipt is stored, because the store can
        // refuse a batch and the receipt cannot: a receipt stored first would
        // put findings on the canvas and the check pill that the Problems
        // page, refusing them, does not list.
        self.publish_diagnostics(
            DiagnosticInputs::DesignCheck {
                subject: subject.clone(),
            },
            records,
        )?;
        self.design_checks.insert(CellViewCheckReceipt {
            project_id: self.workspace.content.project.id(),
            subject,
            input_digest,
            result,
        });
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
    /// by [`DiagnosticInputs::SymbolPins`] and named there. The records about
    /// the subject's stored waivers follow the findings, so an ordinal below
    /// the result's length still names its finding.
    pub(crate) fn design_rule_finding_identities(&self, subject: &CellViewRef) -> Vec<uuid::Uuid> {
        let published = DiagnosticInputs::DesignCheck {
            subject: subject.clone(),
        };
        self.diagnostics
            .batch(&published)
            .map(|batch| {
                batch
                    .records
                    .iter()
                    .map(|record| record.canonical.diagnostic_id)
                    .collect()
            })
            .unwrap_or_default()
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
        let config = self.design_check_config(subject);
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

    /// State every current design-rule finding again against the waivers as
    /// they now stand: after a waiver is recorded or removed, and after an
    /// undo or redo that may have done either.
    ///
    /// Only a current pass is re-stated. Its findings still describe the
    /// design, and a waiver changes whether each counts, not what was found,
    /// so the pass keeps its digest and stays current. A stale pass is left
    /// as it is: re-stating it would publish findings about a design that has
    /// moved under the digest of the one in front of the reader.
    pub(in crate::workbench) fn refresh_waived_findings(&mut self) {
        let subjects = self
            .design_checks
            .receipts
            .values()
            .map(|receipt| receipt.subject.clone())
            .collect::<Vec<_>>();
        for subject in subjects {
            let DesignCheckStatus::Current(receipt) = self.design_check_status(&subject) else {
                continue;
            };
            let mut result = receipt.result.clone();
            let config = self.design_check_config(&subject);
            config.waivers.apply(&mut result);
            if let Err(error) = self.publish_design_check_result(subject, config, result) {
                debug_assert!(false, "a current pass was refused on re-statement: {error}");
            }
        }
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
/// Each route is resolved in the subject's own drawing — the live editor when
/// the subject is the view in front of the reader, the workspace's copy of it
/// otherwise — and names that view, so following it opens the view before it
/// selects anything. A finding about a view that is not open used to be
/// published with no route at all, and one resolved against whichever view
/// was open would have pointed into the wrong coordinate space.
fn design_rule_records(
    state: &AppState,
    subject: &CellViewRef,
    result: &DrcResult,
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
                violation.violation_type.rule_code(),
                Arc::clone(&document),
                location_key(&violation.location),
            )
        })
        .collect();
    let canonicals = mint_canonical(&keys, revision, &format!("design-check:{document_id}"));
    let mut records = result
        .violations()
        .iter()
        .zip(keys)
        .zip(canonicals)
        .map(|((violation, key), mut canonical)| {
            let route = crate::schematic::view::violations::location_anchor_in(
                state,
                subject,
                &violation.location,
            )
            .map(DiagnosticRoute::Anchor);
            canonical.suppression = violation.waiver.as_ref().map(waivers::suppression_of);
            DiagnosticRecord::new(
                canonical,
                violation.severity.into(),
                DiagnosticSubject::new(project, violation.location.display())
                    .in_cell_view(subject.clone()),
                violation.message.clone(),
            )
            .keyed(key)
            .with_details(violation.violation_type.suggested_fix())
            .with_rule(violation.violation_type.rule().explanation)
            .with_route(route)
            .with_waiver_action(waivers::finding_waiver_action(subject, violation))
        })
        .collect::<Vec<_>>();
    // The subject's own waivers that do not do what they say are stated with
    // the pass that found that out.
    if let Some(stored) = state.schematic_waivers(subject) {
        records.extend(waivers::stored_waiver_records(
            state, subject, stored, result,
        ));
    }
    records
}

/// A stable key of the object a rule finding is located at: the design
/// crate's, which a waiver is keyed by too.
pub(in crate::workbench) use rspice_design::drc::location_key;

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
