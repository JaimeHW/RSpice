//! Typed checkpoint failures, independent of a host runner.

use rspice_core::{ResourceKind, SimulationError};

#[derive(Debug, thiserror::Error)]
pub enum CheckpointError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Resource limit exceeded for {resource}: requested {requested}, limit {limit}")]
    ResourceLimit {
        resource: ResourceKind,
        requested: usize,
        limit: usize,
    },
    #[error("Simulation aborted")]
    Aborted,
    #[error(transparent)]
    Numerical(Box<SimulationError>),
}

impl CheckpointError {
    pub(super) fn from_core(error: SimulationError) -> Self {
        match error {
            SimulationError::ResourceLimit(error)
            | SimulationError::Configuration(rspice_core::SimulationConfigError::ResourceLimit(
                error,
            )) => Self::ResourceLimit {
                resource: error.resource,
                requested: error.requested,
                limit: error.limit,
            },
            SimulationError::Aborted | SimulationError::TimeLimitExceeded => Self::Aborted,
            other => Self::Numerical(Box::new(other)),
        }
    }
}
