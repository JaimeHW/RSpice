//! Persisted typed dependency between analysis instances in a plan.

mod validation;

pub use validation::{
    DependencyConfigurationIssue, dependency_configuration_issue, fourier_requirement,
    periodic_state_requirement,
};

use rspice_app_types::product::AnalysisInstanceId;
use serde::{Deserialize, Serialize};

use crate::analysis_kind::AnalysisKind;

/// Explicit, typed dependency from an analysis to one prerequisite instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisDependency {
    prerequisite: AnalysisKind,
    target: AnalysisInstanceId,
}

impl AnalysisDependency {
    #[must_use]
    pub const fn new(prerequisite: AnalysisKind, target: AnalysisInstanceId) -> Self {
        Self {
            prerequisite,
            target,
        }
    }

    #[must_use]
    pub const fn prerequisite(self) -> AnalysisKind {
        self.prerequisite
    }

    #[must_use]
    pub const fn target(self) -> AnalysisInstanceId {
        self.target
    }
}
