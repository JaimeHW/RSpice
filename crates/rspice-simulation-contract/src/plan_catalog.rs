//! Persisted names and clone lineage for the simulation-plan catalog.

use std::fmt;

use rspice_app_types::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
use serde::{Deserialize, Deserializer, Serialize};

const DEFAULT_PLAN_NAME: &str = "Lab characterization";
const MAX_PLAN_NAME_CHARACTERS: usize = 96;

/// Failure to normalize a user-visible simulation-plan name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPlanNameError {
    Empty,
    TooLong,
    ControlCharacter,
}

impl fmt::Display for SimulationPlanNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Plan name cannot be empty."),
            Self::TooLong => write!(
                formatter,
                "Plan name cannot exceed {MAX_PLAN_NAME_CHARACTERS} characters."
            ),
            Self::ControlCharacter => {
                formatter.write_str("Plan name cannot contain control characters.")
            }
        }
    }
}

impl std::error::Error for SimulationPlanNameError {}

/// Validated, user-visible simulation-plan name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct SimulationPlanName(String);

impl SimulationPlanName {
    /// Validate and normalize a user-entered plan name.
    pub fn new(value: impl Into<String>) -> Result<Self, SimulationPlanNameError> {
        let value = value.into();
        let normalized = value.trim();
        if normalized.is_empty() {
            return Err(SimulationPlanNameError::Empty);
        }
        if normalized.chars().count() > MAX_PLAN_NAME_CHARACTERS {
            return Err(SimulationPlanNameError::TooLong);
        }
        if normalized.chars().any(char::is_control) {
            return Err(SimulationPlanNameError::ControlCharacter);
        }
        Ok(Self(normalized.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Case-insensitive key used by the project catalog's uniqueness check.
    #[must_use]
    pub fn uniqueness_key(&self) -> String {
        self.0.to_lowercase()
    }
}

impl Default for SimulationPlanName {
    fn default() -> Self {
        Self(DEFAULT_PLAN_NAME.to_owned())
    }
}

impl fmt::Display for SimulationPlanName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SimulationPlanName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Durable source lineage for one working simulation plan.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanLineage {
    #[serde(default)]
    source_plan_id: Option<SimulationPlanId>,
    #[serde(default)]
    source_revision: Option<ObjectRevision>,
    #[serde(default)]
    clone_contents: Option<SimulationPlanCloneOptions>,
}

impl SimulationPlanLineage {
    #[must_use]
    pub const fn cloned_from_with_contents(
        source_plan_id: SimulationPlanId,
        source_revision: ObjectRevision,
        clone_contents: SimulationPlanCloneOptions,
    ) -> Self {
        Self {
            source_plan_id: Some(source_plan_id),
            source_revision: Some(source_revision),
            clone_contents: Some(clone_contents),
        }
    }

    #[must_use]
    pub const fn source_plan_id(self) -> Option<SimulationPlanId> {
        self.source_plan_id
    }

    #[must_use]
    pub const fn source_revision(self) -> Option<ObjectRevision> {
        self.source_revision
    }

    #[must_use]
    pub const fn clone_contents(self) -> Option<SimulationPlanCloneOptions> {
        self.clone_contents
    }

    /// Whether the three clone-lineage fields are jointly present or absent.
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.source_plan_id.is_some() == self.source_revision.is_some()
            && self.source_plan_id.is_some() == self.clone_contents.is_some()
    }
}

/// Content domains copied into a newly cloned plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanCloneOptions {
    pub copy_analyses: bool,
    pub copy_advanced_options: bool,
    pub copy_variables_outputs_and_specifications: bool,
    pub copy_pvt_and_model_bindings: bool,
    pub copy_regression_baseline_ownership: bool,
}

impl Default for SimulationPlanCloneOptions {
    fn default() -> Self {
        Self::ALL_PLAN_CONTENTS
    }
}

impl SimulationPlanCloneOptions {
    /// Initial clone workflow selection.
    pub const ALL_PLAN_CONTENTS: Self = Self {
        copy_analyses: true,
        copy_advanced_options: true,
        copy_variables_outputs_and_specifications: true,
        copy_pvt_and_model_bindings: true,
        copy_regression_baseline_ownership: false,
    };
}

/// Committed clone identity mapping used by adjacent project-owned payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPlanCloneOutcome {
    pub source_plan_id: SimulationPlanId,
    pub source_revision: ObjectRevision,
    pub cloned_plan_id: SimulationPlanId,
    pub contents: SimulationPlanCloneOptions,
    pub analysis_identity_map: Vec<(AnalysisInstanceId, AnalysisInstanceId)>,
}
