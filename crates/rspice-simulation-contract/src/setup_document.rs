//! Stable persisted simulation setup document.
//!
//! Only these nine plan fields are written to new projects. Schema-3 singleton
//! drafts are read by the dedicated legacy adapter and live in the workbench
//! session until projected into the stable analysis plan.

pub mod legacy_read;

use crate::legacy_plan_migration::default_global_run_set;
use crate::output_policy::SimulationSavePolicy;
use crate::plan_catalog::StoredSimulationPlan;
use crate::plan_model::SimulationPlan;
use crate::run_set::{ReferencePoint as ReferencePvtPoint, RunSetState};

/// Flat, portable simulation plan state saved with the project.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSetupDocument {
    /// Authoritative reference PVT point used by nominal analyses.
    pub reference_pvt: ReferencePvtPoint,
    /// Global process, supply, and temperature space applied to every
    /// executable analysis in the active plan.
    ///
    /// This is deliberately independent of the legacy Corner analysis draft:
    /// the Run Set page configures where the whole plan executes, while a
    /// Corner analysis remains an analysis instance with its own base mode.
    #[serde(default = "default_global_run_set")]
    pub run_set: RunSetState,
    /// Ordered, content-pinned model libraries consumed by this plan.
    /// Absence is an explicit empty closure; execution never falls back to
    /// every library currently loaded in the project manager.
    #[serde(default)]
    pub model_bindings: Vec<rspice_model_library::SimulationPlanModelBinding>,
    /// Result storage, live delivery, and per-plan history policy.
    #[serde(default)]
    pub save_policy: SimulationSavePolicy,
    /// Validated project-unique name of the active simulation plan.
    #[serde(default)]
    pub active_plan_name: crate::plan_catalog::SimulationPlanName,
    /// Immutable source identity and revision when the active plan is a clone.
    #[serde(default)]
    pub active_plan_lineage: crate::plan_catalog::SimulationPlanLineage,
    /// Complete inactive plans retained in deterministic catalog order.
    #[serde(default)]
    pub inactive_plans: Vec<StoredSimulationPlan>,
    /// Stable, revisioned analysis-instance plan. `None` is accepted only
    /// while reading schema-3 projects/sessions and must be deterministically
    /// migrated before validation, editing, or execution.
    #[serde(default)]
    pub analysis_plan: Option<SimulationPlan>,
    /// Effective engine options (validated).
    pub options: crate::options::SimulationOptions,
}
