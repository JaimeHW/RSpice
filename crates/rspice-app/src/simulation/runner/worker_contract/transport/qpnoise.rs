//! One bounded scalar buffer carries the complete authenticated noise evidence.
use super::*;
use rspice_core::engine::{QpnoiseAnalysisResult, QpnoiseTransferMetadata};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerQpnoiseResultTransport {
    metadata: QpnoiseTransferMetadata,
    values: WorkerF64Series,
}
impl WorkerQpnoiseResultTransport {
    pub(super) fn from_response(
        response: QpnoiseAnalysisResult,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Result<Self, String> {
        let (metadata, values) = response
            .into_transfer_parts_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        if values.len() > MAX_WORKER_F64_VALUES {
            return Err("QPNOISE evidence exceeds worker limit".into());
        }
        Ok(Self {
            metadata,
            values: WorkerF64Series::from_vec(values, buffers),
        })
    }
    pub(super) fn into_response(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<QpnoiseAnalysisResult, String> {
        if !matches!(self.values, WorkerF64Series::Buffer { .. })
            || self.values.len() > MAX_WORKER_F64_VALUES
        {
            return Err("QPNOISE requires a bounded dedicated numeric buffer".into());
        }
        let limits = rspice_core::ResourceLimits::default();
        self.metadata
            .validate_transfer_layout_with_abort(self.values.len(), &limits, &rspice_core::NoAbort)
            .map_err(|e| e.to_string())?;
        QpnoiseAnalysisResult::from_transfer_parts_with_abort(
            self.metadata,
            self.values.into_vec(buffers)?,
            &limits,
            &rspice_core::NoAbort,
        )
        .map_err(|e| e.to_string())
    }
}
#[cfg(test)]
mod tests;
