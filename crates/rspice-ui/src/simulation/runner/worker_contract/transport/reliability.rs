//! Dedicated bounded numeric transport for complete reliability missions.
use super::*;
use rspice_core::engine::{ReliabilityRunResult, ReliabilityTransferMetadata};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerReliabilityMissionTransport {
    metadata: ReliabilityTransferMetadata,
    values: WorkerF64Series,
}
impl WorkerReliabilityMissionTransport {
    pub(super) fn from_response(
        response: ReliabilityRunResult,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Result<Self, String> {
        let (metadata, values) = response
            .into_transfer_parts_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        if values.len() > MAX_WORKER_F64_VALUES {
            return Err("Reliability evidence exceeds worker limit".into());
        }
        Ok(Self {
            metadata,
            values: WorkerF64Series::from_vec(values, buffers),
        })
    }
    pub(super) fn into_response(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<ReliabilityRunResult, String> {
        if !matches!(self.values, WorkerF64Series::Buffer { .. })
            || self.values.len() > MAX_WORKER_F64_VALUES
        {
            return Err("Reliability requires a bounded dedicated numeric buffer".into());
        }
        let limits = rspice_core::ResourceLimits::default();
        self.metadata
            .validate_transfer_layout_with_abort(self.values.len(), &limits, &rspice_core::NoAbort)
            .map_err(|e| e.to_string())?;
        ReliabilityRunResult::from_transfer_parts_with_abort(
            self.metadata,
            self.values.into_vec(buffers)?,
            &limits,
            &rspice_core::NoAbort,
        )
        .map_err(|e| e.to_string())
    }
}
