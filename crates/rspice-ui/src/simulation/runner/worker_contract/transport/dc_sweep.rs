//! DC coordinates cross as a dedicated numerical buffer; identities remain metadata.

use super::*;
use crate::state::{
    DcCurveSelection, DcSweepDirection, DcSweepEvidence, DcSweepFamily, DcSweepQuantity,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerDcSweepEvidence {
    source: String,
    direction: DcSweepDirection,
    quantities: Vec<DcSweepQuantity>,
    family: WorkerDcSweepFamily,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WorkerDcSweepFamily {
    Single,
    Nested {
        source: String,
        values: WorkerF64Series,
    },
    Retraced,
}

impl WorkerDcSweepEvidence {
    pub(super) fn from_evidence(evidence: DcSweepEvidence, buffers: &mut Vec<Vec<f64>>) -> Self {
        Self {
            source: evidence.source,
            direction: evidence.direction,
            quantities: evidence.quantities,
            family: match evidence.family {
                DcSweepFamily::Single => WorkerDcSweepFamily::Single,
                DcSweepFamily::Nested { source, values } => WorkerDcSweepFamily::Nested {
                    source,
                    values: WorkerF64Series::from_vec(values, buffers),
                },
                DcSweepFamily::Retraced => WorkerDcSweepFamily::Retraced,
            },
        }
    }

    pub(super) fn into_evidence(self, buffers: &[Vec<f64>]) -> Result<DcSweepEvidence, String> {
        let evidence = DcSweepEvidence {
            source: self.source,
            direction: self.direction,
            quantities: self.quantities,
            family: match self.family {
                WorkerDcSweepFamily::Single => DcSweepFamily::Single,
                WorkerDcSweepFamily::Nested { source, values } => {
                    if matches!(values, WorkerF64Series::Inline(_)) {
                        return Err(
                            "Nested DC coordinates must use a dedicated transfer buffer".to_owned()
                        );
                    }
                    DcSweepFamily::Nested {
                        source,
                        values: values.into_vec(buffers)?,
                    }
                }
                WorkerDcSweepFamily::Retraced => DcSweepFamily::Retraced,
            },
            selection: DcCurveSelection::All,
        };
        evidence.validate()?;
        Ok(evidence)
    }
}

pub(super) fn validate_dc_transport_size(
    axis: &WorkerF64Series,
    waveforms: &[WorkerWaveformTransport],
    evidence: Option<&WorkerDcSweepEvidence>,
) -> Result<(), String> {
    let evidence =
        evidence.ok_or_else(|| "Worker DC transport is missing exact curve evidence".to_owned())?;
    let (members, coordinates) = match &evidence.family {
        WorkerDcSweepFamily::Single => (1, 0),
        WorkerDcSweepFamily::Nested { values, .. } => (values.len(), values.len()),
        WorkerDcSweepFamily::Retraced => (2, 0),
    };
    if evidence.quantities.len().checked_mul(members) != Some(waveforms.len()) {
        return Err("Worker DC curves do not cover the declared sweep space".to_owned());
    }
    let values = waveforms
        .iter()
        .fold(axis.len().saturating_add(coordinates), |total, trace| {
            total
                .saturating_add(trace.x_values.len())
                .saturating_add(trace.y_values.len())
                .saturating_add(trace.y_imag.as_ref().map_or(0, WorkerF64Series::len))
        });
    if values > MAX_WORKER_F64_VALUES {
        return Err("DC curves and coordinate evidence exceed the worker numeric limit".to_owned());
    }
    Ok(())
}
