//! Each batch's live digest, reused while nothing it read has moved.
//!
//! A batch's standing is decided by digesting its inputs as they stand, and
//! four surfaces ask every frame. The digest is exact and is not free: a
//! design check's walks the configured hierarchy and digests every cellview
//! the check read. So each digest is remembered with the key of what it was
//! taken from — the [stamps](rspice_design::stamped) of the documents and
//! authorities, the library and plan revisions, and the few small values a
//! digest reads directly — and a frame whose key is the one remembered reads
//! the remembered digest. Every input a digest reads is in its key, so a
//! remembered digest is the one a recomputation would give; an input moving
//! changes the key, and the next reader recomputes.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::product::{ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};
use crate::state::workspace::DesignProjectionKey;

use super::{AppState, DiagnosticInputs};

/// What one batch's digest was taken from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DigestKey {
    /// The configured design, which covers every cellview, authority and
    /// binding the check's walk may read, and the deck it was checked under.
    DesignCheck {
        design: DesignProjectionKey,
        deck: u64,
        project: ProjectId,
    },
    Dispatch {
        design: DesignProjectionKey,
        root_topology: Option<u64>,
        plan: Option<(SimulationPlanId, ObjectRevision)>,
        preflight: bool,
        project: ProjectId,
    },
    /// A run or a refused campaign member. The model catalogue has no stamp,
    /// so its key — a hash, not a digest — is read each time, as the run's
    /// own digest always read it.
    Run {
        design: DesignProjectionKey,
        retained: bool,
        plan: Option<(SimulationPlanId, ObjectRevision)>,
        options: String,
        process: String,
        temperature: u64,
        models: u64,
        project: ProjectId,
    },
    RuleDeck {
        deck: u64,
        project: ProjectId,
        label: String,
    },
}

impl DigestKey {
    /// The key of what `inputs` reads, or `None` where it cannot be stated
    /// cheaply and exactly, which recomputes the digest every time: a design
    /// whose authorities cannot be serialized has no projection key, and a
    /// symbol's pin contract is read from documents with no stamp.
    pub(super) fn of(state: &AppState, inputs: &DiagnosticInputs) -> Option<Self> {
        let project = state.workspace.content.project.id();
        let design = || {
            state.workspace.design_projection_key(
                &state.library_manager,
                &state.workspace.content.active_view,
                &state.schematic,
            )
        };
        match inputs {
            DiagnosticInputs::DesignCheck { .. } => Some(Self::DesignCheck {
                design: design()?,
                deck: state.workspace.content.design_rules.stamp(),
                project,
            }),
            DiagnosticInputs::SymbolPins { .. } => None,
            DiagnosticInputs::Dispatch => Some(Self::Dispatch {
                design: design()?,
                root_topology: state
                    .workspace
                    .simulation_root_schematic(
                        &state.workspace.content.active_view,
                        &state.schematic,
                    )
                    .map(|source| source.design.topology_version()),
                plan: state.active_plan_revision(),
                preflight: state.workbench.preflight.report.is_some(),
                project,
            }),
            DiagnosticInputs::Run { run_sequence } => Some(Self::Run {
                design: design()?,
                retained: state
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .any(|run| run.id == *run_sequence),
                plan: state.active_plan_revision(),
                options: state.sim_setup.options.to_spice_options(),
                process: format!("{:?}", state.sim_setup.reference_pvt.process),
                temperature: state.sim_setup.reference_pvt.temperature_celsius.to_bits(),
                models: state.model_library_manager.design_inspection_catalog_key(),
                project,
            }),
            DiagnosticInputs::CampaignMember { .. } => Some(Self::Run {
                design: design()?,
                retained: false,
                plan: state.active_plan_revision(),
                options: state.sim_setup.options.to_spice_options(),
                process: format!("{:?}", state.sim_setup.reference_pvt.process),
                temperature: state.sim_setup.reference_pvt.temperature_celsius.to_bits(),
                models: state.model_library_manager.design_inspection_catalog_key(),
                project,
            }),
            DiagnosticInputs::RuleDeck => Some(Self::RuleDeck {
                deck: state.workspace.content.design_rules.stamp(),
                project,
                label: super::super::rule_deck::project_file_label(state),
            }),
        }
    }
}

/// The remembered digests, by the store key of the batch inputs they are of.
///
/// Behind a lock because the readers hold the session immutably. A clone
/// starts empty, as the pass memo does.
#[derive(Default)]
pub(super) struct DigestMemo(Mutex<HashMap<String, Remembered>>);

/// One remembered digest, and the key of what it was taken from.
type Remembered = (DigestKey, Result<ContentDigest, String>);

impl Clone for DigestMemo {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl std::fmt::Debug for DigestMemo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("DigestMemo")
    }
}

impl DigestMemo {
    pub(super) fn get(&self, slot: &str, key: &DigestKey) -> Option<Result<ContentDigest, String>> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(slot)
            .filter(|(remembered, _)| remembered == key)
            .map(|(_, digest)| digest.clone())
    }

    pub(super) fn remember(
        &self,
        slot: String,
        key: DigestKey,
        digest: Result<ContentDigest, String>,
    ) {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(slot, (key, digest));
    }

    pub(super) fn clear(&self) {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }
}

// Count of batch digests computed rather than remembered, so a test can assert
// that a frame with no edit computes none.
#[cfg(test)]
thread_local! {
    static COMPUTED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(super) fn count_computed() {
    COMPUTED.with(|count| count.set(count.get().wrapping_add(1)));
}

/// Batch digests computed on this thread since the last reset.
#[cfg(test)]
pub(in crate::workbench) fn computed_digests() -> u64 {
    COMPUTED.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(in crate::workbench) fn reset_computed_digests() {
    COMPUTED.with(|count| count.set(0));
}
