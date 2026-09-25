//! Portable catalog transaction failures.

use std::fmt;

use rspice_app_types::product::SimulationPlanId;

use crate::plan_model::AnalysisPlanError;

use super::SimulationPlanNameError;

/// Atomic named-plan catalog operation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationPlanCatalogError {
    InvalidName(String),
    DuplicateName(String),
    ActivePlanUnavailable,
    PlanNotFound(SimulationPlanId),
    PlanExecuting(SimulationPlanId),
    ActivePlanCannotBeArchived(SimulationPlanId),
    PlanArchived(SimulationPlanId),
    PlanAlreadyArchived(SimulationPlanId),
    PlanNotArchived(SimulationPlanId),
    InvalidLineage(SimulationPlanId),
    DuplicatePlanIdentity(SimulationPlanId),
    InvalidModelBindings(String),
    InvalidSavePolicy(String),
    InvalidPlan(AnalysisPlanError),
}

impl fmt::Display for SimulationPlanCatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(reason) => formatter.write_str(reason),
            Self::DuplicateName(name) => {
                write!(
                    formatter,
                    "A simulation plan named '{name}' already exists."
                )
            }
            Self::ActivePlanUnavailable => formatter
                .write_str("The active simulation plan has not been migrated to stable identity."),
            Self::PlanNotFound(id) => write!(formatter, "Simulation plan {id} does not exist."),
            Self::PlanExecuting(id) => write!(
                formatter,
                "Simulation plan {id} owns queued or executing work and cannot be replaced."
            ),
            Self::ActivePlanCannotBeArchived(id) => write!(
                formatter,
                "Active simulation plan {id} cannot be archived; activate another plan first."
            ),
            Self::PlanArchived(id) => write!(
                formatter,
                "Simulation plan {id} is archived and must be restored before activation."
            ),
            Self::PlanAlreadyArchived(id) => {
                write!(formatter, "Simulation plan {id} is already archived.")
            }
            Self::PlanNotArchived(id) => {
                write!(formatter, "Simulation plan {id} is not archived.")
            }
            Self::InvalidLineage(id) => write!(
                formatter,
                "Simulation plan {id} has incomplete clone-lineage metadata."
            ),
            Self::DuplicatePlanIdentity(id) => write!(
                formatter,
                "Simulation plan identity {id} appears more than once in the project."
            ),
            Self::InvalidModelBindings(error) => formatter.write_str(error),
            Self::InvalidSavePolicy(error) => formatter.write_str(error),
            Self::InvalidPlan(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SimulationPlanCatalogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidPlan(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SimulationPlanNameError> for SimulationPlanCatalogError {
    fn from(error: SimulationPlanNameError) -> Self {
        Self::InvalidName(error.to_string())
    }
}

impl From<AnalysisPlanError> for SimulationPlanCatalogError {
    fn from(error: AnalysisPlanError) -> Self {
        Self::InvalidPlan(error)
    }
}
