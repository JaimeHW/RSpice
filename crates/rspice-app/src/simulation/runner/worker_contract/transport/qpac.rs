//! Packed real/imaginary sweep buffers with bounded, validated QPAC reconstruction.
use super::*;
use rspice_core::engine::{QpacAnalysisResult, QpacResultMetadata};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerQpacResultTransport {
    metadata: QpacResultMetadata,
    real: WorkerF64Series,
    imaginary: WorkerF64Series,
}

impl WorkerQpacResultTransport {
    pub(super) fn from_response(point: QpacAnalysisResult, buffers: &mut Vec<Vec<f64>>) -> Self {
        let (metadata, rows) = point.into_transfer_parts();
        // Two buffers for the whole sweep; one pair per offset/coordinate
        // would exhaust the worker buffer-count limit on ordinary long sweeps.
        let (real, imaginary) = rows.into_iter().flatten().map(|v| (v.re, v.im)).unzip();
        Self {
            metadata,
            real: WorkerF64Series::from_vec(real, buffers),
            imaginary: WorkerF64Series::from_vec(imaginary, buffers),
        }
    }

    pub(super) fn into_response(self, buffers: &[Vec<f64>]) -> Result<QpacAnalysisResult, String> {
        if !matches!(self.real, WorkerF64Series::Buffer { .. })
            || !matches!(self.imaginary, WorkerF64Series::Buffer { .. })
            || self.real.len() != self.imaginary.len()
        {
            return Err("QPAC requires equal dedicated real/imaginary buffers".into());
        }
        let points = self.metadata.request.offsets_hz.len();
        let coordinates = self
            .metadata
            .node_names
            .len()
            .checked_add(self.metadata.branch_names.len())
            .ok_or("QPAC MNA shape overflow")?;
        let rows = points
            .checked_mul(coordinates)
            .ok_or("QPAC sweep shape overflow")?;
        let tuples = self.metadata.tuples.len();
        let expected = rows
            .checked_mul(tuples)
            .and_then(|n| points.checked_mul(2).and_then(|p| n.checked_add(p)))
            .ok_or("QPAC payload shape overflow")?;
        if tuples == 0
            || rows > MAX_WORKER_F64_VALUES / 2
            || expected > MAX_WORKER_F64_VALUES / 2
            || self.real.len() != expected
        {
            return Err("QPAC packed buffers differ from the bounded sweep layout".into());
        }
        let mut lengths = vec![tuples; rows];
        lengths.extend([points, points]);
        let limits = rspice_core::ResourceLimits::default();
        self.metadata
            .validate_transfer_layout_with_abort(&lengths, &limits, &rspice_core::NoAbort)
            .map_err(|e| e.to_string())?;
        let values = worker_join_complex(
            "QPAC complete response",
            self.real.into_vec(buffers)?,
            self.imaginary.into_vec(buffers)?,
        )?;
        let mut values = values.into_iter();
        let rows = lengths
            .into_iter()
            .map(|len| values.by_ref().take(len).collect())
            .collect();
        QpacAnalysisResult::from_transfer_parts_with_abort(
            self.metadata,
            rows,
            &limits,
            &rspice_core::NoAbort,
        )
        .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qpac_transport_rejects_short_inline_and_altered_complex_buffers() {
        let SimulationResult::Qpac { response, .. } = SimulationResult::qpac_test_fixture() else {
            unreachable!()
        };
        let mut buffers = Vec::new();
        let transport =
            WorkerQpacResultTransport::from_response(Arc::unwrap_or_clone(response), &mut buffers);
        assert_eq!(
            buffers.len(),
            2,
            "buffer count is independent of sweep length"
        );
        let mut short = transport.clone();
        short.imaginary = WorkerF64Series::from_vec(vec![0.0], &mut buffers);
        assert!(short.into_response(&buffers).is_err());
        let mut inline = transport.clone();
        inline.real = WorkerF64Series::Inline(vec![0.0; inline.imaginary.len()]);
        assert!(inline.into_response(&buffers).is_err());
        let mut json = serde_json::to_value(&transport).unwrap();
        // Alter scalar metadata without recomputing its complete-result identity.
        json["metadata"]["request"]["magnitude"] = serde_json::json!(0.003);
        let altered: WorkerQpacResultTransport = serde_json::from_value(json).unwrap();
        assert!(altered.into_response(&buffers).is_err());
        let WorkerF64Series::Buffer { buffer, .. } = transport.real else {
            panic!("complex spectrum was not transferred")
        };
        buffers[buffer][0] += 0.1;
        assert!(transport.into_response(&buffers).is_err());
    }
}
