//! Authenticated transient continuation and bounded snapshot publication.
use crate::error::SimulationError;
use rspice_app_types::product::ContentDigest;
use rspice_core::engine::TransientCheckpoint;
use rspice_core::{AbortSignal, NoAbort, ResourceLimits};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub(crate) fn check_limit(
    resource: rspice_core::ResourceKind,
    requested: usize,
    limit: usize,
) -> Result<(), rspice_core::ResourceLimitError> {
    if requested <= limit {
        Ok(())
    } else {
        Err(rspice_core::ResourceLimitError {
            resource,
            requested,
            limit,
        })
    }
}

pub fn checkpoint_digest(bytes: &[u8]) -> ContentDigest {
    rspice_app_types::canonical::content_digest("rspice.transient-checkpoint/v1", bytes)
}

pub fn validate_bytes_size(length: usize, limits: ResourceLimits) -> Result<(), SimulationError> {
    if length == 0 {
        return Err(SimulationError::InvalidConfig(
            "Transient checkpoint is empty".into(),
        ));
    }
    check_limit(
        rspice_core::ResourceKind::ExternalDataBytes,
        length,
        limits.max_external_data_bytes,
    )
    .map_err(|error| SimulationError::from(rspice_core::SimulationError::from(error)))
}

pub fn decode_bytes(
    bytes: &[u8],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<TransientCheckpoint, SimulationError> {
    crate::error::ensure_not_aborted(abort)?;
    validate_bytes_size(bytes.len(), limits)?;
    let checkpoint = TransientCheckpoint::from_bytes_with_limit_and_abort(
        bytes,
        limits.max_external_data_bytes,
        abort,
    )
    .map_err(SimulationError::from)?;
    checkpoint
        .capability()
        .require_resumable()
        .map_err(SimulationError::InvalidConfig)?;
    check_limit(
        rspice_core::ResourceKind::ResultValues,
        checkpoint.retained_value_count(),
        limits.max_result_values,
    )
    .map_err(|error| SimulationError::from(rspice_core::SimulationError::from(error)))?;
    Ok(checkpoint)
}

/// Bytes are detached from worker JSON and authenticated again after transfer.
/// Deserializing the metadata alone does not create a usable resume input.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientCheckpointInput {
    digest: ContentDigest,
    #[serde(skip)]
    bytes: Vec<u8>,
}

impl TransientCheckpointInput {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, SimulationError> {
        Self::from_bytes_with_limits(bytes, ResourceLimits::default(), &NoAbort)
    }
    pub fn from_bytes_with_limits(
        bytes: Vec<u8>,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        decode_bytes(&bytes, limits, abort)?;
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
    pub fn decode(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<TransientCheckpoint, SimulationError> {
        crate::error::ensure_not_aborted(abort)?;
        validate_bytes_size(self.bytes.len(), limits)?;
        if checkpoint_digest(&self.bytes) != self.digest {
            return Err(SimulationError::InvalidConfig(
                "Transient checkpoint content identity differs".into(),
            ));
        }
        decode_bytes(&self.bytes, limits, abort)
    }
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn take_bytes(
        &mut self,
        limits: ResourceLimits,
    ) -> Result<Vec<u8>, SimulationError> {
        self.decode(limits, &NoAbort)?;
        Ok(std::mem::take(&mut self.bytes))
    }
    #[cfg(any(test, all(target_arch = "wasm32", feature = "browser-worker")))]
    pub(crate) fn restore_bytes(
        &mut self,
        bytes: Vec<u8>,
        limits: ResourceLimits,
    ) -> Result<(), SimulationError> {
        if !self.bytes.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Duplicate inline transient checkpoint".into(),
            ));
        }
        let candidate = Self {
            digest: self.digest,
            bytes,
        };
        candidate.decode(limits, &NoAbort)?;
        *self = candidate;
        Ok(())
    }
}

/// Nominal absolute times, with the engine's accepted-point coalescing rules.
/// Resume uses the authored-restart identity contract: the stop horizon and
/// restart/output controls may change; circuit and source timing stay bound.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientCheckpointRequest {
    pub times: Vec<f64>,
    pub resume: Option<TransientCheckpointInput>,
}
impl TransientCheckpointRequest {
    pub fn validate_schedule(
        &self,
        stop: f64,
        limits: ResourceLimits,
    ) -> Result<(), SimulationError> {
        check_limit(
            rspice_core::ResourceKind::AnalysisPoints,
            self.times.len(),
            limits.max_analysis_points,
        )
        .map_err(|error| SimulationError::from(rspice_core::SimulationError::from(error)))?;
        let mut previous = None;
        for &time in &self.times {
            if !time.is_finite() || time < 0.0 || time > stop || previous.is_some_and(|p| time <= p)
            {
                return Err(SimulationError::InvalidConfig(
                    "Transient checkpoint times must increase strictly within the run interval"
                        .into(),
                ));
            }
            previous = Some(time);
        }
        if self.times.is_empty() && self.resume.is_none() {
            return Err(SimulationError::InvalidConfig(
                "Transient checkpoint request has no schedule or resume input".into(),
            ));
        }
        Ok(())
    }
}

pub type CheckpointQueue = Arc<Mutex<Option<Arc<[u8]>>>>;
pub type CheckpointObserver = Arc<dyn Fn(&[u8]) -> Result<(), SimulationError> + Send + Sync>;
pub(crate) type BorrowedObserver<'a> = dyn Fn(&[u8]) -> Result<(), SimulationError> + Sync + 'a;
pub(crate) struct CheckpointExecution<'a> {
    pub request: &'a TransientCheckpointRequest,
    pub observer: &'a BorrowedObserver<'a>,
}

pub fn replace_checkpoint(queue: &CheckpointQueue, bytes: Arc<[u8]>) {
    *queue
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(bytes);
}
