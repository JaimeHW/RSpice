//! Bounded QPXF reconstruction from two packed complex buffers and authenticated metadata.
use super::*;
use rspice_core::engine::{QpxfAnalysisResult, QpxfResultMetadata};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerQpxfResultTransport {
    metadata: QpxfResultMetadata,
    real: WorkerF64Series,
    imaginary: WorkerF64Series,
}
impl WorkerQpxfResultTransport {
    pub(super) fn from_response(response: QpxfAnalysisResult, buffers: &mut Vec<Vec<f64>>) -> Self {
        let (metadata, rows) = response.into_transfer_parts();
        let (real, imaginary) = rows.into_iter().flatten().map(|v| (v.re, v.im)).unzip();
        Self {
            metadata,
            real: WorkerF64Series::from_vec(real, buffers),
            imaginary: WorkerF64Series::from_vec(imaginary, buffers),
        }
    }
    pub(super) fn into_response(self, buffers: &[Vec<f64>]) -> Result<QpxfAnalysisResult, String> {
        if !matches!(self.real, WorkerF64Series::Buffer { .. })
            || !matches!(self.imaginary, WorkerF64Series::Buffer { .. })
            || self.real.len() != self.imaginary.len()
        {
            return Err("QPXF requires equal dedicated real/imaginary buffers".into());
        }
        let points = self.metadata.request.frequencies_hz.len();
        let coordinates = self
            .metadata
            .node_names
            .len()
            .checked_add(self.metadata.branch_names.len())
            .ok_or("QPXF coordinate count overflow")?;
        let rows = points
            .checked_mul(coordinates)
            .ok_or("QPXF sweep shape overflow")?;
        let paths = self
            .metadata
            .input_sources
            .len()
            .checked_mul(self.metadata.input_lattices.len())
            .ok_or("QPXF path count overflow")?;
        let limits = rspice_core::ResourceLimits::default();
        if points == 0
            || points > limits.max_analysis_points
            || coordinates == 0
            || coordinates > 65_536
            || paths == 0
            || rows > MAX_WORKER_F64_VALUES / 2
            || paths > MAX_WORKER_F64_VALUES / 2
        {
            return Err("QPXF retained dimensions exceed worker limits".into());
        }
        let grid = rspice_core::analysis::quasi_periodic::QuasiPeriodicGrid::new_with_abort(
            self.metadata.grid.clone(),
            &limits,
            &rspice_core::NoAbort,
        )
        .map_err(|e| e.to_string())?;
        let expected = rows
            .checked_mul(grid.len())
            .and_then(|n| paths.checked_mul(points).and_then(|p| n.checked_add(p)))
            .ok_or("QPXF payload shape overflow")?;
        if expected > MAX_WORKER_F64_VALUES / 2 || self.real.len() != expected {
            return Err("QPXF packed data differs from its bounded adjoint/transfer layout".into());
        }
        let mut lengths = vec![grid.len(); rows];
        lengths.extend(std::iter::repeat_n(points, paths));
        self.metadata
            .validate_transfer_layout_with_abort(&lengths, &limits, &rspice_core::NoAbort)
            .map_err(|e| e.to_string())?;
        let mut values = worker_join_complex(
            "QPXF complete transfer",
            self.real.into_vec(buffers)?,
            self.imaginary.into_vec(buffers)?,
        )?
        .into_iter();
        let rows = lengths
            .into_iter()
            .map(|n| values.by_ref().take(n).collect())
            .collect();
        QpxfAnalysisResult::from_transfer_parts_with_abort(
            self.metadata,
            rows,
            &limits,
            &rspice_core::NoAbort,
        )
        .map_err(|e| e.to_string())
    }
}
#[cfg(test)]
mod tests;
