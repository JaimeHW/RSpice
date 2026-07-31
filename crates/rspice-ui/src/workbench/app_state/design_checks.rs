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
use crate::state::{CanonicalCellViewOwnerKey, CellViewRef, canonical_cell_view_owner_key};

use super::AppState;

const DESIGN_CHECK_DIGEST_DOMAIN: &[u8] = b"rspice-interactive-design-check/v1\0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesignCheckOrigin {
    Manual,
    Incremental,
    ConnectivityCommit,
}

#[derive(Debug, Clone)]
pub(crate) struct CellViewCheckReceipt {
    pub(crate) project_id: ProjectId,
    pub(crate) subject: CellViewRef,
    pub(crate) checked_project_revision: ObjectRevision,
    pub(crate) input_digest: ContentDigest,
    pub(crate) checked_unix_ms: u64,
    pub(crate) origin: DesignCheckOrigin,
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
    pub(crate) fn run_active_design_checks(
        &mut self,
        origin: DesignCheckOrigin,
    ) -> Result<DrcResult, String> {
        let subject = self.workspace.active_schematic_reference();
        let config = design_check_config(self, &subject);
        let mut live_buffers = self.workspace.schematic_buffers.clone();
        live_buffers.insert(subject.key(), self.schematic.clone());
        let hierarchy =
            crate::simulation::netlist_gen::HierarchySource::from_workspace_with_connectivity(
                &self.library_manager,
                &live_buffers,
                &self.workspace.connectivity,
            );
        let result = crate::services::drc::run_drc_check_with_hierarchy_and_config(
            &self.schematic,
            &hierarchy,
            config.clone(),
        );
        self.publish_design_check_result(subject, config, result.clone(), origin)?;
        Ok(result)
    }

    pub(crate) fn publish_active_design_check_result(
        &mut self,
        result: DrcResult,
        origin: DesignCheckOrigin,
    ) -> Result<(), String> {
        let subject = self.workspace.active_schematic_reference();
        let config = design_check_config(self, &subject);
        self.publish_design_check_result(subject, config, result, origin)
    }

    fn publish_design_check_result(
        &mut self,
        subject: CellViewRef,
        config: DrcConfig,
        result: DrcResult,
        origin: DesignCheckOrigin,
    ) -> Result<(), String> {
        let input_digest = design_check_input_digest(self, &subject, &config)?;
        self.design_checks.insert(CellViewCheckReceipt {
            project_id: self.workspace.project.id(),
            subject,
            checked_project_revision: self.workspace.project.revision(),
            input_digest,
            checked_unix_ms: crate::time_compat::unix_epoch().as_millis() as u64,
            origin,
            result,
        });
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
        self.design_check_status(&self.workspace.active_schematic_reference())
    }

    pub(crate) fn project_root_design_check_status(&self) -> DesignCheckStatus<'_> {
        self.design_check_status(&project_root_reference(self))
    }

    pub(crate) fn clear_active_design_check(&mut self) {
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
    let active = state.workspace.active_schematic_reference();
    let active_key = active.key();
    let mut schematic_digests = state
        .workspace
        .schematic_buffers
        .iter()
        .filter(|(key, _)| key.as_str() != active_key)
        .map(|(key, schematic)| {
            schematic
                .validated_design_content_digest()
                .map(|digest| (key.clone(), digest))
                .map_err(|error| format!("could not digest schematic {key}: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    schematic_digests.push((
        active_key,
        state
            .schematic
            .validated_design_content_digest()
            .map_err(|error| format!("could not digest active schematic: {error}"))?,
    ));
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
    let material = serde_json::to_vec(&(
        state.workspace.project.id(),
        owner_key(subject).to_string(),
        schematic_digests,
        &state.workspace.connectivity,
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
            .publish_active_design_check_result(DrcResult::new(), DesignCheckOrigin::Manual)
            .expect("publish root receipt");

        state
            .workspace
            .schematic_buffers
            .insert(root.key(), state.schematic.clone());
        state.workspace.active_view = other;
        state.schematic = crate::state::SchematicState::default();

        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::NotRun
        ));
        assert!(matches!(
            state.design_check_status(&root),
            DesignCheckStatus::Current(_)
        ));
    }

    #[test]
    fn connectivity_changes_stale_previously_current_receipts() {
        let mut state = AppState::default();
        state
            .publish_active_design_check_result(DrcResult::new(), DesignCheckOrigin::Manual)
            .expect("publish current receipt");
        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::Current(_)
        ));

        state.workspace.connectivity.next_identity =
            state.workspace.connectivity.next_identity.saturating_add(1);

        assert!(matches!(
            state.active_design_check_status(),
            DesignCheckStatus::Stale(_)
        ));
    }
}
