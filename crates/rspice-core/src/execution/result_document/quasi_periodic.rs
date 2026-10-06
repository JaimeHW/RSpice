//! Quasiperiodic documents retain the core's authenticated numerical evidence.
//!
//! Primary series give generic result clients their usual windowed view. The
//! payload preserves independent-tone coordinates, complete solves, unavailable
//! measurements and provenance in the engine's existing versioned types.
use super::*;
use crate::engine::{
    QpacAnalysisResult, QpnoiseAnalysisResult, QpssOperatingPoint, QpxfAnalysisResult,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QpssPayload {
    pub operating_point: QpssOperatingPoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QpacPayload {
    pub result: QpacAnalysisResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QpxfPayload {
    pub result: QpxfAnalysisResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QpnoisePayload {
    pub result: QpnoiseAnalysisResult,
}

pub(super) fn evidence_error(error: crate::SimulationError) -> ResultDocumentError {
    match error {
        crate::SimulationError::Aborted => ResultDocumentError::Aborted,
        crate::SimulationError::ResourceLimit(error) => ResultDocumentError::ResourceLimit(error),
        error => ResultDocumentError::Malformed {
            location: "quasiperiodic evidence",
            detail: error.to_string(),
        },
    }
}

macro_rules! payload {
    ($ty:ident, $field:ident) => {
        impl $ty {
            pub(super) fn value_count(&self) -> usize {
                numeric_count::count(self)
            }
            pub(super) fn validate(
                &self,
                limits: &crate::ResourceLimits,
                abort: &dyn AbortSignal,
            ) -> Result<(), ResultDocumentError> {
                self.$field
                    .validate_retained_payload_with_abort(limits, abort)
                    .map(|_| ())
                    .map_err(evidence_error)
            }
        }
    };
}
payload!(QpssPayload, operating_point);
payload!(QpacPayload, result);
payload!(QpxfPayload, result);
payload!(QpnoisePayload, result);

pub(super) fn validate_primary(
    document: &AnalysisResultDocument,
    limits: &crate::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    use super::builders::quasi_periodic::*;
    let projection = match document.payload() {
        ResultPayload::Qpss(payload) => {
            let grid = crate::analysis::quasi_periodic::QuasiPeriodicGrid::new_with_abort(
                payload
                    .operating_point
                    .resolved_grid_config()
                    .map_err(evidence_error)?,
                limits,
                abort,
            )
            .map_err(|error| match error {
                crate::analysis::quasi_periodic::QuasiPeriodicError::Aborted => {
                    ResultDocumentError::Aborted
                }
                crate::analysis::quasi_periodic::QuasiPeriodicError::ResourceLimit(error) => {
                    ResultDocumentError::ResourceLimit(error)
                }
                error => ResultDocumentError::Malformed {
                    location: "QPSS tone grid",
                    detail: error.to_string(),
                },
            })?;
            qpss_projection(&payload.operating_point, &grid)?
        }
        ResultPayload::Qpac(payload) => qpac_projection(&payload.result)?,
        ResultPayload::Qpxf(payload) => qpxf_projection(&payload.result)?,
        ResultPayload::Qpnoise(payload) => qpnoise_projection(&payload.result)?,
        _ => return Ok(()),
    };
    projection.validate(document)
}
