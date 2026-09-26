//! Retained measurement admission uses the exact original FAILVALUE evidence.

use crate::analysis_type::AnalysisType;
type AnalysisResult = crate::analysis_result::AnalysisResult;

fn projected_failvalue_measurement() -> rspice_core::MeasureResult {
    rspice_core::MeasureResult {
        name: "peak_at".to_owned(),
        value: Some(20.0),
        raw_value: Some(3.0),
        error: None,
        passed: true,
        expected: None,
        tolerance: None,
        failure_limit: Some(4.0),
        failure_limit_exceeded: false,
        event_axis: Some(20.0),
        units: None,
    }
}

#[test]
fn retained_measurements_require_exact_failvalue_evidence() {
    let valid = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
        .with_measurements(vec![projected_failvalue_measurement()]);
    valid
        .validate_retained_evidence()
        .expect("a projected value may differ from its exact raw FAILVALUE evidence");

    let mut unevaluated = projected_failvalue_measurement();
    unevaluated.value = None;
    unevaluated.raw_value = None;
    unevaluated.event_axis = None;
    unevaluated.passed = false;
    unevaluated.error = Some("signal was unavailable".to_owned());
    AnalysisResult::new(2, AnalysisType::Transient, "TRAN", 0.0)
        .with_measurements(vec![unevaluated])
        .validate_retained_evidence()
        .expect("an early failure retains the authored limit without raw evidence");

    let mut missing_raw = valid.clone();
    missing_raw.measurements[0].raw_value = None;
    assert!(missing_raw.validate_retained_evidence().is_err());

    let mut missing_published = valid.clone();
    missing_published.measurements[0].value = None;
    assert!(missing_published.validate_retained_evidence().is_err());

    let mut nonfinite_raw = valid.clone();
    nonfinite_raw.measurements[0].raw_value = Some(f64::NAN);
    assert!(nonfinite_raw.validate_retained_evidence().is_err());

    let mut nonfinite_limit = valid.clone();
    nonfinite_limit.measurements[0].failure_limit = Some(f64::INFINITY);
    assert!(nonfinite_limit.validate_retained_evidence().is_err());

    let mut false_positive = valid.clone();
    false_positive.measurements[0].failure_limit_exceeded = true;
    false_positive.measurements[0].passed = false;
    assert!(false_positive.validate_retained_evidence().is_err());

    let mut false_negative = valid.clone();
    false_negative.measurements[0].raw_value = Some(-4.0);
    false_negative.measurements[0].passed = false;
    assert!(false_negative.validate_retained_evidence().is_err());

    let mut passed_after_exceeded = valid;
    passed_after_exceeded.measurements[0].raw_value = Some(4.0);
    passed_after_exceeded.measurements[0].failure_limit_exceeded = true;
    assert!(passed_after_exceeded.validate_retained_evidence().is_err());
}
