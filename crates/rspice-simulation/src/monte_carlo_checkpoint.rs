//! Immutable checkpoint requests and bounded delivery of the latest snapshot.

pub mod preparation;

use crate::error::SimulationError;
use rspice_app_types::product::ContentDigest;
use rspice_core::{NoAbort, ResourceKind, ResourceLimits, abort_signal::AbortSignal};
use rspice_results::monte_carlo_checkpoint::{StudyMonteCarloCheckpoint, checkpoint_digest};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// The numerical payload is detached before serializing a worker request.
/// Deserialization alone never makes a usable checkpoint input: the receiver
/// must restore its transferable bytes and verify the declared content identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonteCarloCheckpointInput {
    digest: ContentDigest,
    #[serde(skip)]
    bytes: Vec<u8>,
}

impl MonteCarloCheckpointInput {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, SimulationError> {
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
    pub fn digest(&self) -> ContentDigest {
        self.digest
    }
    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn decode(&self) -> Result<StudyMonteCarloCheckpoint, SimulationError> {
        self.decode_with_limits(ResourceLimits::default(), &NoAbort)
    }

    pub fn decode_with_limits(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<StudyMonteCarloCheckpoint, SimulationError> {
        crate::error::ensure_not_aborted(abort)?;
        if self.bytes.len() > limits.max_external_data_bytes {
            return Err(SimulationError::ResourceLimit {
                resource: ResourceKind::ExternalDataBytes.as_str().into(),
                requested: self.bytes.len(),
                limit: limits.max_external_data_bytes,
            });
        }
        if checkpoint_digest(&self.bytes) != self.digest {
            return Err(SimulationError::InvalidConfig(
                "Monte Carlo checkpoint input does not match its frozen content identity".into(),
            ));
        }
        let checkpoint =
            StudyMonteCarloCheckpoint::from_bytes_with_limits(&self.bytes, limits, abort)?;
        checkpoint.validate_for_resume()?;
        Ok(checkpoint)
    }
    pub fn take_bytes(&mut self) -> Result<Vec<u8>, SimulationError> {
        self.decode()?;
        Ok(std::mem::take(&mut self.bytes))
    }
    pub fn restore_bytes(&mut self, bytes: Vec<u8>) -> Result<(), SimulationError> {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonteCarloCheckpointRequest {
    pub publish_every: std::num::NonZeroUsize,
    /// Original trial coordinates against the same frozen source.
    pub trial_range: Option<std::ops::Range<usize>>,
    pub resume: Option<MonteCarloCheckpointInput>,
}

impl MonteCarloCheckpointRequest {
    pub fn validate(&self) -> Result<(), SimulationError> {
        self.validate_with_limits(ResourceLimits::default(), &NoAbort)
    }

    pub fn validate_with_limits(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        crate::error::ensure_not_aborted(abort)?;
        if let Some(range) = &self.trial_range {
            if range.is_empty() {
                return Err(SimulationError::InvalidConfig(
                    "Monte Carlo checkpoint range must be nonempty".into(),
                ));
            }
            let requested = range.end - range.start;
            if requested > limits.max_batch_runs {
                return Err(SimulationError::ResourceLimit {
                    resource: ResourceKind::BatchRuns.as_str().into(),
                    requested,
                    limit: limits.max_batch_runs,
                });
            }
        }
        if let Some(input) = &self.resume {
            input.decode_with_limits(limits, abort)?;
        }
        Ok(())
    }
}

pub type CheckpointQueue = Arc<Mutex<Option<Arc<[u8]>>>>;
pub type CheckpointObserver = Arc<dyn Fn(&[u8]) -> Result<(), SimulationError> + Send + Sync>;

/// Coalescing is lossless: every snapshot includes all previously accepted rows.
/// Holding only the latest snapshot bounds a suspended UI's retained queue.
pub fn replace_checkpoint(queue: &CheckpointQueue, bytes: Arc<[u8]>) {
    *queue
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(bytes);
}

pub fn validate_checkpoint_bytes_size(length: usize) -> Result<(), String> {
    let limit = ResourceLimits::default().max_external_data_bytes;
    if length == 0 || length > limit {
        Err(format!(
            "Checkpoint has {length} bytes; expected 1..={limit}"
        ))
    } else {
        Ok(())
    }
}

/// Bound the combined numerical and checkpoint payload before copying from JS.
pub fn validate_worker_request_checkpoint_lengths(
    numeric_values: usize,
    lengths: &[usize],
) -> Result<(), String> {
    if lengths.len() > 1 {
        return Err("Worker request carries more than one checkpoint".into());
    }
    let mut bytes = numeric_values.saturating_mul(8);
    for length in lengths {
        validate_checkpoint_bytes_size(*length)?;
        bytes = bytes.saturating_add(*length);
    }
    let limit = rspice_core::ResourceLimits::default().max_external_data_bytes;
    if bytes > limit {
        return Err(format!(
            "Worker request numerical/checkpoint payload has {bytes} bytes, exceeding {limit}"
        ));
    }
    Ok(())
}
