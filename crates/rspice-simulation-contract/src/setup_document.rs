//! Persisted simulation setup document and schema-3 read compatibility.
//!
//! Nine plan fields serialize. The 27 singleton analysis fields marked
//! `skip_serializing` are accepted only when reading older projects and are
//! projected into the stable analysis plan during migration. Workbench editor
//! state is neither serialized nor stored in this contract.

use std::collections::HashSet;

use crate::drafts::{AcSetup, DcSetup, TranSetup};
use crate::legacy_plan_migration::{NoiseSetup, default_global_run_set, deserialize_analysis_set};
use crate::output_policy::SimulationSavePolicy;
use crate::plan_catalog::StoredSimulationPlan;
use crate::plan_model::SimulationPlan;
use crate::run_set::{ReferencePoint as ReferencePvtPoint, RunSetState};

/// Flat persisted setup document, including schema-3 read-only migration fields.
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
    /// Enabled analysis indices.
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "deserialize_analysis_set"
    )]
    pub enabled: HashSet<usize>,
    /// Stable execution order. Enabled analyses absent from this vector are
    /// appended deterministically; disabled entries are ignored and removed
    /// from the persisted normalized plan.
    #[serde(default, skip_serializing)]
    pub analysis_order: Vec<usize>,
    /// Transient sweep.
    #[serde(default, skip_serializing)]
    pub tran: TranSetup,
    /// AC sweep.
    #[serde(default, skip_serializing)]
    pub ac: AcSetup,
    /// DISTO secondary tone ratio f2/f1 (empty = single-tone HD).
    #[serde(default, skip_serializing)]
    pub disto_f2_over_f1: String,
    /// DC transfer sweep.
    #[serde(default, skip_serializing)]
    pub dc: DcSetup,
    /// Noise analysis.
    #[serde(default, skip_serializing)]
    pub noise: NoiseSetup,
    /// DC operating point.
    #[serde(default, skip_serializing)]
    pub op: crate::op_draft::OpDialogState,
    /// Pole-zero extraction.
    #[serde(default, skip_serializing)]
    pub pz: crate::pz_draft::PzDialogState,
    /// Sensitivity.
    #[serde(default, skip_serializing)]
    pub sens: crate::sens_draft::SensDialogState,
    /// Monte Carlo.
    #[serde(default, skip_serializing)]
    pub mc: crate::mc_draft::McDialogState,
    /// Periodic steady state.
    #[serde(default, skip_serializing)]
    pub pss: crate::pss_draft::PssDialogState,
    /// Loop stability.
    #[serde(default, skip_serializing)]
    pub stb: crate::stb_draft::StbDialogState,
    /// Temperature sweep.
    #[serde(default, skip_serializing)]
    pub temp: crate::temp_draft::TempDialogState,
    /// Harmonic balance.
    #[serde(default, skip_serializing)]
    pub hb: crate::hb_draft::HbDialogState,
    /// S-parameters.
    #[serde(default, skip_serializing)]
    pub sp: crate::sp_draft::SpDialogState,
    /// Periodic AC.
    #[serde(default, skip_serializing)]
    pub pac: crate::pac_draft::PacDialogState,
    /// Periodic noise.
    #[serde(default, skip_serializing)]
    pub pnoise: crate::pnoise_draft::PnoiseDialogState,
    /// Periodic transfer.
    #[serde(default, skip_serializing)]
    pub pxf: crate::pxf_draft::PxfDialogState,
    /// Periodic stability.
    #[serde(default, skip_serializing)]
    pub pstb: crate::pstb_draft::PstbDialogState,
    /// Transfer function.
    #[serde(default, skip_serializing)]
    pub xf: crate::xf_draft::XfDialogState,
    /// Process corners.
    #[serde(default, skip_serializing)]
    pub corner: crate::corner_draft::CornerDialogState,
    /// Envelope transient.
    #[serde(default, skip_serializing)]
    pub envelope: crate::envelope_draft::EnvelopeDialogState,
    /// Fourier.
    #[serde(default, skip_serializing)]
    pub fourier: crate::fourier_draft::FourierDialogState,
    /// Optimization.
    #[serde(default, skip_serializing)]
    pub optimization: crate::optimization_draft::OptimizationDialogState,
    /// Safe operating area.
    #[serde(default, skip_serializing)]
    pub soa: crate::soa_draft::SoaDialogState,
    /// Effective engine options (validated).
    pub options: crate::options::SimulationOptions,
    /// Analyses listed in the run-set card beyond the always-listed core —
    /// exotics stay listed (dimmed) when unticked, until removed.
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "deserialize_analysis_set"
    )]
    pub listed: HashSet<usize>,
}
