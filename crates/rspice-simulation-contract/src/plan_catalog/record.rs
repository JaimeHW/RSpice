//! Complete inactive-plan records and portable import documents.

use std::collections::HashSet;

use rspice_app_types::product::{ObjectRevision, SimulationPlanId};
use rspice_model_library::SimulationPlanModelBinding;
use serde::{Deserialize, Serialize};

use crate::options::SimulationOptions;
use crate::output_policy::SimulationSavePolicy;
use crate::plan_model::SimulationPlan;
use crate::run_set::{ReferencePoint, RunSetState};

use super::{SimulationPlanLineage, SimulationPlanName};

/// Complete persisted state for an inactive named plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoredSimulationPlan {
    name: SimulationPlanName,
    lineage: SimulationPlanLineage,
    analysis_plan: SimulationPlan,
    reference_pvt: ReferencePoint,
    /// Complete run-space declaration owned by this plan. Older inactive-plan
    /// records did not persist one; migration restores a review-required
    /// reference-only declaration rather than copying the active plan's axes.
    #[serde(default = "legacy_inactive_run_set")]
    run_set: RunSetState,
    /// Ordered model closure and per-library nominal section owned by this
    /// inactive plan.
    #[serde(default)]
    model_bindings: Vec<SimulationPlanModelBinding>,
    #[serde(default)]
    save_policy: SimulationSavePolicy,
    /// Recoverable retirement state. Archived plans retain their complete
    /// configuration and all result references.
    #[serde(default)]
    archived: bool,
    options: SimulationOptions,
}

fn legacy_inactive_run_set() -> RunSetState {
    RunSetState::reference_only()
}

impl StoredSimulationPlan {
    /// Retain a plan document as an unarchived catalog entry. Its export-source
    /// fields are envelope provenance; the stored identity and revision come
    /// from `analysis_plan`. The committing transaction checks catalog-wide
    /// identity, name and graph invariants.
    #[must_use]
    pub fn from_document(
        document: SimulationPlanImportDocument,
        lineage: SimulationPlanLineage,
    ) -> Self {
        Self {
            name: document.name,
            lineage,
            analysis_plan: document.analysis_plan,
            reference_pvt: document.reference_pvt,
            run_set: document.run_set,
            model_bindings: document.model_bindings,
            save_policy: document.save_policy,
            archived: false,
            options: document.options,
        }
    }

    /// Snapshot the persisted fields for a portable export.
    #[must_use]
    pub fn to_document(&self) -> SimulationPlanImportDocument {
        SimulationPlanImportDocument {
            source_plan_id: self.id(),
            source_revision: self.revision(),
            name: self.name.clone(),
            analysis_plan: self.analysis_plan.clone(),
            reference_pvt: self.reference_pvt,
            run_set: self.run_set.clone(),
            model_bindings: self.model_bindings.clone(),
            save_policy: self.save_policy,
            options: self.options.clone(),
        }
    }

    /// Consume a selected inactive record when its plan becomes active.
    #[must_use]
    pub fn into_document(self) -> SimulationPlanImportDocument {
        SimulationPlanImportDocument {
            source_plan_id: self.analysis_plan.id(),
            source_revision: self.analysis_plan.revision(),
            name: self.name,
            analysis_plan: self.analysis_plan,
            reference_pvt: self.reference_pvt,
            run_set: self.run_set,
            model_bindings: self.model_bindings,
            save_policy: self.save_policy,
            options: self.options,
        }
    }

    pub fn rename(&mut self, name: SimulationPlanName) {
        self.name = name;
    }

    pub fn set_archived(&mut self, archived: bool) {
        self.archived = archived;
    }

    pub fn replace_model_bindings(&mut self, bindings: Vec<SimulationPlanModelBinding>) {
        self.model_bindings = bindings;
    }

    pub fn prepare_after_restore(&mut self) {
        self.analysis_plan.prepare_after_restore();
    }

    #[must_use]
    pub fn name(&self) -> &SimulationPlanName {
        &self.name
    }

    #[must_use]
    pub const fn id(&self) -> SimulationPlanId {
        self.analysis_plan.id()
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.analysis_plan.revision()
    }

    #[must_use]
    pub const fn lineage(&self) -> SimulationPlanLineage {
        self.lineage
    }

    #[must_use]
    pub const fn analysis_plan(&self) -> &SimulationPlan {
        &self.analysis_plan
    }

    /// The corner and temperature this plan resolves an undeclared axis to.
    ///
    /// Every stored plan carries one, so a catalog projection that could not
    /// read it reported the corner as unknown for every inactive plan — a fact
    /// the record holds, rendered as though it did not exist.
    #[must_use]
    pub const fn reference_pvt(&self) -> ReferencePoint {
        self.reference_pvt
    }

    #[must_use]
    pub const fn run_set(&self) -> &RunSetState {
        &self.run_set
    }

    #[must_use]
    pub fn model_bindings(&self) -> &[SimulationPlanModelBinding] {
        &self.model_bindings
    }

    #[must_use]
    pub const fn save_policy(&self) -> SimulationSavePolicy {
        self.save_policy
    }

    #[must_use]
    pub const fn archived(&self) -> bool {
        self.archived
    }
}

/// Portable, payload-independent simulation-plan document. The workspace
/// payload travels beside this record so import can remap every analysis
/// reference to the fresh destination identities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanImportDocument {
    pub source_plan_id: SimulationPlanId,
    pub source_revision: ObjectRevision,
    pub name: SimulationPlanName,
    pub analysis_plan: SimulationPlan,
    pub reference_pvt: ReferencePoint,
    pub run_set: RunSetState,
    pub model_bindings: Vec<SimulationPlanModelBinding>,
    #[serde(default)]
    pub save_policy: SimulationSavePolicy,
    pub options: SimulationOptions,
}

pub fn validate_model_binding_list(bindings: &[SimulationPlanModelBinding]) -> Result<(), String> {
    let mut names = HashSet::with_capacity(bindings.len());
    for binding in bindings {
        let name = binding.library_name.as_str();
        if name.is_empty() || name != name.trim() || name.chars().any(char::is_control) {
            return Err(
                "model binding library names must be nonempty, trimmed, and control-free"
                    .to_owned(),
            );
        }
        if !names.insert(name.to_ascii_lowercase()) {
            return Err(format!("model library '{name}' is bound more than once"));
        }
        if let Some(section) = binding.selected_corner.as_deref()
            && (section.is_empty()
                || section != section.trim()
                || section.chars().any(char::is_control))
        {
            return Err(format!(
                "model binding '{name}' has an invalid corner section"
            ));
        }
    }
    Ok(())
}
