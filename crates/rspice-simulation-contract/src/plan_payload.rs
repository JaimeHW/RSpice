//! Persisted, per-plan authored payload independent of the project workspace.

use rspice_app_types::product::{RunId, SimulationPlanId};
use serde::{Deserialize, Serialize};

use crate::capture_group::CaptureGroup;
use crate::design_variable::DesignVariable;
use crate::regression_policy::RegressionToleranceRule;
use crate::saved_output::SavedOutput;
use crate::specification::{SpecEntry, SpecificationDefinition, SpecificationPolicy};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanPayload {
    #[serde(default)]
    pub design_variables: Vec<DesignVariable>,
    #[serde(default)]
    pub saved_outputs: Vec<SavedOutput>,
    /// Named capture policies over the saved outputs. Empty is the state every
    /// project written before this model loads in, and it means exactly one
    /// thing: every output belongs to the synthesized fallback group, which
    /// overrides nothing. So an old project's forecast and its execution are
    /// unchanged by the field's arrival.
    #[serde(default)]
    pub capture_groups: Vec<CaptureGroup>,
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    /// Governed specification records. Empty means the project predates this
    /// model and is deterministically migrated from `specs` on first access.
    #[serde(default)]
    pub specification_definitions: Vec<SpecificationDefinition>,
    #[serde(default)]
    pub specification_policy: SpecificationPolicy,
    #[serde(default)]
    pub regression_baseline_run: Option<RunId>,
    #[serde(default)]
    pub regression_tolerances: Vec<RegressionToleranceRule>,
}

/// Vec-backed because product UUID wrappers intentionally do not define an
/// ordering. Validation guarantees unique owners; lifecycle hashing sorts a
/// canonical projection by UUID bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanPayloadRecord {
    pub plan_id: SimulationPlanId,
    pub payload: SimulationPlanPayload,
}
