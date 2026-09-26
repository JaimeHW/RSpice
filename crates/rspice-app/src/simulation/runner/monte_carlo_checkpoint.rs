//! Immutable checkpoint requests and bounded delivery of the latest snapshot.

use super::SimulationError;
use crate::product::ContentDigest;
use rspice_core::{NoAbort, ResourceLimits};
use rspice_results::monte_carlo_checkpoint::{
    CheckpointError, StudyMonteCarloCheckpoint, checkpoint_digest,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// The numerical payload is detached before serializing a worker request.
/// Deserialization alone never makes a usable checkpoint input: the receiver
/// must restore its transferable bytes and verify the declared content identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MonteCarloCheckpointInput {
    digest: ContentDigest,
    #[serde(skip)]
    bytes: Vec<u8>,
}

impl MonteCarloCheckpointInput {
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Result<Self, SimulationError> {
        StudyMonteCarloCheckpoint::from_bytes_with_limits(
            &bytes,
            ResourceLimits::default(),
            &NoAbort,
        )?
        .validate_for_resume()?;
        Ok(Self {
            digest: checkpoint_digest(&bytes),
            bytes,
        })
    }
    pub(crate) fn digest(&self) -> ContentDigest {
        self.digest
    }
    pub(crate) fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn decode(&self) -> Result<StudyMonteCarloCheckpoint, SimulationError> {
        if checkpoint_digest(&self.bytes) != self.digest {
            return Err(SimulationError::InvalidConfig(
                "Monte Carlo checkpoint input does not match its frozen content identity".into(),
            ));
        }
        let checkpoint = StudyMonteCarloCheckpoint::from_bytes_with_limits(
            &self.bytes,
            ResourceLimits::default(),
            &NoAbort,
        )?;
        checkpoint.validate_for_resume()?;
        Ok(checkpoint)
    }
    #[allow(
        dead_code,
        reason = "browser worker checkpoint transport uses this method"
    )]
    pub(super) fn take_bytes(&mut self) -> Result<Vec<u8>, SimulationError> {
        self.decode()?;
        Ok(std::mem::take(&mut self.bytes))
    }
    #[allow(
        dead_code,
        reason = "browser worker checkpoint transport uses this method"
    )]
    pub(super) fn restore_bytes(&mut self, bytes: Vec<u8>) -> Result<(), SimulationError> {
        if !self.bytes.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Duplicate inline Monte Carlo checkpoint input".into(),
            ));
        }
        let candidate = Self {
            digest: self.digest,
            bytes,
        };
        candidate.decode()?;
        *self = candidate;
        Ok(())
    }
}

impl From<CheckpointError> for SimulationError {
    fn from(error: CheckpointError) -> Self {
        match error {
            CheckpointError::InvalidConfig(message) => Self::InvalidConfig(message),
            CheckpointError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource: resource.as_str().into(),
                requested,
                limit,
            },
            CheckpointError::Aborted => Self::Aborted,
            CheckpointError::Numerical(error) => {
                crate::simulation::engine_bridge::EngineBridge::new().translate_error(*error)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MonteCarloCheckpointRequest {
    pub publish_every: std::num::NonZeroUsize,
    /// Original trial coordinates against the same frozen source.
    pub trial_range: Option<std::ops::Range<usize>>,
    pub resume: Option<MonteCarloCheckpointInput>,
}

impl MonteCarloCheckpointRequest {
    pub(crate) fn validate(&self) -> Result<(), SimulationError> {
        let limits = ResourceLimits::default();
        if let Some(range) = &self.trial_range
            && (range.is_empty() || range.end - range.start > limits.max_batch_runs)
        {
            return Err(SimulationError::InvalidConfig(
                "Monte Carlo checkpoint range must be nonempty and within the batch limit".into(),
            ));
        }
        if let Some(input) = &self.resume {
            input.decode()?;
        }
        Ok(())
    }
}

pub(in crate::simulation::runner) type CheckpointQueue = Arc<Mutex<Option<Arc<[u8]>>>>;
pub(in crate::simulation::runner) type CheckpointObserver =
    Arc<dyn Fn(&[u8]) -> Result<(), SimulationError> + Send + Sync>;

/// Coalescing is lossless: every snapshot includes all previously accepted rows.
/// Holding only the latest snapshot bounds a suspended UI's retained queue.
pub(super) fn replace_checkpoint(queue: &CheckpointQueue, bytes: Arc<[u8]>) {
    *queue
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(bytes);
}

#[allow(
    dead_code,
    reason = "browser worker checkpoint transport uses this validator"
)]
pub(super) fn validate_checkpoint_bytes_size(length: usize) -> Result<(), String> {
    let limit = ResourceLimits::default().max_external_data_bytes;
    if length == 0 || length > limit {
        Err(format!(
            "Monte Carlo checkpoint has {length} bytes; expected 1..={limit}"
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests;
