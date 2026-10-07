//! Lossless persistence of continuous measurements evaluated before compression.

use pyo3::PyResult;
use rspice_core::analysis::{
    ContinuousMeasureFailureMetadata, ContinuousMeasureRecord, ContinuousMeasureResult,
    ContinuousMeasureVerificationFailure,
};

type RecordState = (
    f64,
    f64,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    bool,
    bool,
    Option<String>,
);

pub(crate) type ContinuousMeasurementState = (
    String,
    Vec<RecordState>,
    Option<String>,
    Option<(Option<f64>, Option<f64>)>,
);

pub(crate) fn continuous_measurement_state(
    result: &ContinuousMeasureResult,
) -> ContinuousMeasurementState {
    (
        result.name.clone(),
        result
            .records
            .iter()
            .map(|record| {
                let failure = record.verification_failure.map(|failure| {
                    match failure {
                        ContinuousMeasureVerificationFailure::NonFiniteRawValue => {
                            "non_finite_raw_value"
                        }
                        ContinuousMeasureVerificationFailure::NonFiniteFailureLimit => {
                            "non_finite_failure_limit"
                        }
                        ContinuousMeasureVerificationFailure::FailureLimitExceeded => {
                            "failure_limit_exceeded"
                        }
                    }
                    .to_string()
                });
                (
                    record.value,
                    record.raw_value,
                    record.event_axis,
                    record.trigger_axis,
                    record.target_axis,
                    record.failure_limit,
                    record.failure_limit_exceeded,
                    record.passed,
                    failure,
                )
            })
            .collect(),
        result.failure.clone(),
        result
            .failure_metadata
            .map(|metadata| (metadata.trigger_axis, metadata.target_axis)),
    )
}

pub(crate) fn rebuild_continuous_measurement(
    state: ContinuousMeasurementState,
) -> PyResult<ContinuousMeasureResult> {
    let (name, records, failure, metadata) = state;
    let records = records
        .into_iter()
        .map(
            |(
                value,
                raw_value,
                event_axis,
                trigger_axis,
                target_axis,
                failure_limit,
                failure_limit_exceeded,
                passed,
                verification_failure,
            )| {
                let verification_failure = verification_failure
                    .map(|tag| match tag.as_str() {
                        "non_finite_raw_value" => {
                            Ok(ContinuousMeasureVerificationFailure::NonFiniteRawValue)
                        }
                        "non_finite_failure_limit" => {
                            Ok(ContinuousMeasureVerificationFailure::NonFiniteFailureLimit)
                        }
                        "failure_limit_exceeded" => {
                            Ok(ContinuousMeasureVerificationFailure::FailureLimitExceeded)
                        }
                        _ => Err(crate::errors::value_error(format!(
                            "unsupported continuous measurement verification failure '{tag}'"
                        ))),
                    })
                    .transpose()?;
                Ok(ContinuousMeasureRecord {
                    value,
                    raw_value,
                    event_axis,
                    trigger_axis,
                    target_axis,
                    failure_limit,
                    failure_limit_exceeded,
                    passed,
                    verification_failure,
                })
            },
        )
        .collect::<PyResult<_>>()?;
    Ok(ContinuousMeasureResult {
        name,
        records,
        failure,
        failure_metadata: metadata.map(|(trigger_axis, target_axis)| {
            ContinuousMeasureFailureMetadata {
                trigger_axis,
                target_axis,
            }
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{IntoPyObject, Python, types::PyAnyMethods};

    #[test]
    fn continuous_measurement_pickle_preserves_records_verdicts_and_partial_events() {
        Python::initialize();
        Python::attach(|py| {
            let mut result = ContinuousMeasureResult {
                name: "crossings".into(),
                records: vec![ContinuousMeasureRecord {
                    value: 2.0,
                    raw_value: 2.0,
                    event_axis: Some(0.5),
                    trigger_axis: None,
                    target_axis: None,
                    failure_limit: Some(1.0),
                    failure_limit_exceeded: true,
                    passed: false,
                    verification_failure: Some(
                        ContinuousMeasureVerificationFailure::FailureLimitExceeded,
                    ),
                }],
                failure: None,
                failure_metadata: None,
            };
            for failure in [
                None,
                Some(ContinuousMeasureVerificationFailure::FailureLimitExceeded),
                Some(ContinuousMeasureVerificationFailure::NonFiniteRawValue),
                Some(ContinuousMeasureVerificationFailure::NonFiniteFailureLimit),
            ] {
                result.records[0].verification_failure = failure;
                let object = continuous_measurement_state(&result)
                    .into_pyobject(py)
                    .unwrap();
                let restored = rebuild_continuous_measurement(object.extract().unwrap()).unwrap();
                assert_eq!(restored, result);
            }
            result.records.clear();
            result.failure = Some("target event not found".into());
            result.failure_metadata = Some(ContinuousMeasureFailureMetadata {
                trigger_axis: Some(0.5),
                target_axis: None,
            });
            let object = continuous_measurement_state(&result)
                .into_pyobject(py)
                .unwrap();
            assert_eq!(
                rebuild_continuous_measurement(object.extract().unwrap()).unwrap(),
                result
            );
            let mut state = continuous_measurement_state(&result);
            state.1.push((
                1.0,
                1.0,
                Some(0.5),
                None,
                None,
                None,
                false,
                false,
                Some("future".into()),
            ));
            assert!(rebuild_continuous_measurement(state).is_err());
        });
    }
}
