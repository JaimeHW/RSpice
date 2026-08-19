//! Runtime design-check evidence keyed by exact project cell/view identity.
//!
//! Interactive checks are derived evidence: they do not dirty project bytes or
//! masquerade as a validated save. Each receipt binds the checker profile and
//! the complete live schematic working set, so switching documents can never
//! relabel another cell's findings as current.

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::product::{ContentDigest, ObjectRevision, ProjectId};
use crate::services::drc::{DrcConfig, DrcResult};
use crate::state::{
    CanonicalCellViewOwnerKey, CellViewRef, ViewType, canonical_cell_view_owner_key,
};

use super::AppState;

const DESIGN_CHECK_DIGEST_DOMAIN: &[u8] = b"rspice-interactive-design-check/v1\0";

#[derive(Debug, Clone)]
pub(crate) struct CellViewCheckReceipt {
    pub(crate) project_id: ProjectId,
    pub(crate) subject: CellViewRef,
    pub(crate) checked_project_revision: ObjectRevision,
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
        let subject = self.workspace.active_schematic_reference();
        let config = design_check_config(self, &subject);
        // Checks run over the design as configured, not over the editor
        // buffer. That is what makes two sheets with coincident authored
        // coordinates two nets: the projection namespaces the pages apart
        // before anything electrical is read off them.
        let projection = self
            .workspace
            .design_projection(
                &self.library_manager,
                &self.workspace.active_view,
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
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_design_projection(
            &self.library_manager,
            &projection,
        );
        let result = crate::services::drc::run_drc_check_with_hierarchy_and_config(
            checked,
            &hierarchy,
            config.clone(),
        );
        self.publish_design_check_result(subject, config, result.clone())?;
        Ok(result)
    }

    pub(crate) fn publish_active_design_check_result(
        &mut self,
        result: DrcResult,
    ) -> Result<(), String> {
        if !matches!(
            self.workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return Err(
                "schematic design-check evidence can only be published from a schematic or testbench"
                    .to_owned(),
            );
        }
        let subject = self.workspace.active_schematic_reference();
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
            owner_key(&subject) == owner_key(&self.workspace.active_schematic_reference());
        let input_digest = design_check_input_digest(self, &subject, &config)?;
        self.design_checks.insert(CellViewCheckReceipt {
            project_id: self.workspace.project.id(),
            subject,
            checked_project_revision: self.workspace.project.revision(),
            input_digest,
            result,
        });
        if is_active_subject {
            self.refresh_active_design_check_projection();
        }
        Ok(())
    }

    pub(crate) fn design_check_status(&self, subject: &CellViewRef) -> DesignCheckStatus<'_> {
        let last = self.design_checks.receipt(subject);
        let Some(receipt) = last else {
            return DesignCheckStatus::NotRun;
        };
        if receipt.project_id != self.workspace.project.id() {
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
            self.workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return DesignCheckStatus::NotRun;
        }
        self.design_check_status(&self.workspace.active_schematic_reference())
    }

    pub(crate) fn project_root_design_check_status(&self) -> DesignCheckStatus<'_> {
        self.design_check_status(&project_root_reference(self))
    }

    pub(crate) fn clear_active_design_check(&mut self) {
        if !matches!(
            self.workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            self.refresh_active_design_check_projection();
            return;
        }
        let subject = self.workspace.active_schematic_reference();
        self.design_checks.clear_subject(&subject);
    }

    pub(crate) fn clear_all_design_checks(&mut self) {
        self.design_checks.clear();
    }

    /// Keep the legacy interactive DRC projection scoped to the exact active
    /// schematic. Remaining canvas/status consumers can therefore never show
    /// a different cell/view's findings while they are migrated to receipts.
    pub(crate) fn refresh_active_design_check_projection(&mut self) {
        if !matches!(
            self.workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            self.dialogs.drc_checked_version = 0;
            self.dialogs.drc_results = None;
            self.dialogs.drc_cycle = None;
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
        self.dialogs.drc_cycle = None;
    }
}

fn owner_key(subject: &CellViewRef) -> CanonicalCellViewOwnerKey {
    canonical_cell_view_owner_key(&subject.library, &subject.cell, &subject.view)
}

fn project_root_reference(state: &AppState) -> CellViewRef {
    CellViewRef::new(
        &state.workspace.project.root_library,
        &state.workspace.project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    )
}

fn design_check_config(state: &AppState, subject: &CellViewRef) -> DrcConfig {
    DrcConfig {
        check_missing_ground: owner_key(subject) == owner_key(&project_root_reference(state)),
        ..DrcConfig::default()
    }
}

fn design_check_input_digest(
    state: &AppState,
    subject: &CellViewRef,
    config: &DrcConfig,
) -> Result<ContentDigest, String> {
    let mut live_buffers = state.workspace.schematic_buffers.clone();
    if matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        live_buffers.insert(state.workspace.active_view.key(), state.schematic.clone());
    }
    let mut schematic_digests = live_buffers
        .iter()
        .map(|(key, schematic)| {
            let content_digest = schematic
                .validated_design_content_digest()
                .map_err(|error| format!("could not digest schematic {key}: {error}"))?;
            let mut net_mapping = schematic
                .net_mapping
                .iter()
                .map(|(point, net)| (point.x, point.y, net.as_str()))
                .collect::<Vec<_>>();
            net_mapping.sort_unstable();
            Ok((key.clone(), content_digest, net_mapping))
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
        state.workspace.project.id(),
        owner_key(subject).to_string(),
        schematic_digests,
        state.library_manager.revision(),
        &state.workspace.connectivity,
        &state.workspace.design_management,
        &state.workspace.configuration_sets,
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

    #[test]
    fn receipts_never_alias_another_active_cell_view() {
        let mut state = AppState::default();
        let root = state.workspace.active_schematic_reference();
        let other = CellViewRef::new(&root.library, "aux", &root.view);
        state
            .workspace
            .schematic_buffers
            .insert(other.key(), crate::state::SchematicState::default());
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish root receipt");

        state
            .workspace
            .schematic_buffers
            .insert(root.key(), state.schematic.clone());
        state.workspace.active_view = other;
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

        state.workspace.connectivity.policy.width_mismatch =
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
        let root = state.workspace.active_view.clone();
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish root receipt");
        state
            .workspace
            .schematic_buffers
            .insert(root.key(), state.schematic.clone());
        let layout = CellViewRef::new(&root.library, &root.cell, "layout");
        state
            .workspace
            .open_views
            .push(crate::state::OpenCellView::new(
                layout.clone(),
                ViewType::Layout,
            ));
        state.workspace.active_view = layout;
        state.schematic = crate::state::SchematicState::default();

        assert!(matches!(
            state.design_check_status(&root),
            DesignCheckStatus::Current(_)
        ));
    }

    #[test]
    fn symbol_focus_never_aliases_or_clears_its_sibling_schematic_receipt() {
        let mut state = AppState::default();
        let root = state.workspace.active_view.clone();
        state
            .publish_active_design_check_result(completed_result())
            .expect("publish root receipt");
        let symbol = CellViewRef::new(&root.library, &root.cell, "symbol");
        state
            .workspace
            .open_views
            .push(crate::state::OpenCellView::new(
                symbol.clone(),
                ViewType::Symbol,
            ));
        state.workspace.active_view = symbol;

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
        state.schematic.wires.push(crate::state::Wire::segment(
            FIRST_WIRE,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(40, 0),
        ));
        state.schematic.wires.push(crate::state::Wire::segment(
            SECOND_WIRE,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(40, 0),
        ));
        state.sync_active_schematic_to_workspace();

        let key = state.workspace.active_schematic_reference().key();
        let first = state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Page 1", [FIRST_WIRE])
            .expect("a fresh cell view accepts its first governed sheet");
        let catalog = state
            .workspace
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
            crate::simulation::netlist_gen::design_nets(&state.schematic).len(),
            1,
            "the editor buffer holds one coordinate space, so its pages overlap"
        );

        let projection = state
            .workspace
            .design_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            )
            .expect("the fixture configuration resolves");
        assert_eq!(
            crate::simulation::netlist_gen::projection_nets(
                &state.library_manager,
                &projection,
                &state.workspace.active_view.key(),
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
        let root = state.workspace.active_view.clone();
        state
            .workspace
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
