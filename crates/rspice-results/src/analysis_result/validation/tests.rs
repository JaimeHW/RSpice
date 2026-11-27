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

/// A running transient's live plot breaks its trace where the live queue let points go, and
/// is held as an unfinished analysis; the same gap in a finished result is a solver NaN.
#[test]
fn only_an_unfinished_analysis_holds_the_live_plots_gaps() {
    let gapped = || {
        crate::waveform::RetainedWaveform::new(
            "out",
            vec![0.0, 1.0, 2.0, 9.0],
            vec![0.0, 0.5, f64::NAN, 0.9],
        )
    };
    let mut running =
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![gapped()]);
    running.success = false;
    running.error_message = Some(crate::analysis_result::LIVE_TRANSIENT_PARTIAL_MESSAGE.to_owned());
    running
        .validate_retained_evidence()
        .expect("a live plot's gap is not a damaged result");

    let finished =
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![gapped()]);
    assert!(finished.validate_retained_evidence().is_err());
}

/// An SOA analysis whose reporting view is its whole source history: a warning at the second of
/// two samples.
#[cfg(feature = "test-support")]
fn soa_with_history() -> AnalysisResult {
    use crate::analysis_payload::AnalysisResultPayload;
    use crate::family_metadata::AnalysisResultFamilyMetadata;
    use crate::soa_evidence::{
        SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
        SoaViolationSeverityEvidence,
    };
    use crate::soa_source::{SoaSourceHistory, SoaSourceWaveform};
    use crate::waveform::RetainedWaveform;
    let time = vec![0.0, 1.0e-9];
    let stress = crate::safety::soa_stress_waveform_name("M1", crate::safety::SoAParameter::Vds);
    let traces = [
        (stress.as_str(), "V", vec![2.0, 3.0]),
        ("SOA_VIOLATION_COUNT", "count", vec![0.0, 1.0]),
    ];
    let source = SoaSourceHistory {
        time: time.clone(),
        waveforms: traces
            .iter()
            .map(|(name, unit, values)| SoaSourceWaveform {
                name: (*name).to_owned(),
                unit: (*unit).to_owned(),
                values: values.clone(),
            })
            .collect(),
    };
    AnalysisResult::new(1, AnalysisType::Soa, "SOA", 0.0)
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: time.clone() })
        .with_waveforms(
            traces
                .iter()
                .map(|(name, unit, values)| {
                    RetainedWaveform::new(*name, time.clone(), values.clone()).with_unit(*unit)
                })
                .collect(),
        )
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: Some(std::sync::Arc::new(source)),
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        })
}

/// A stored SOA reporting view is checked against its source history through the reader
/// service: on the thread that draws the window the check waits for the read, as a pending read
/// that a later frame finds, and reads nothing of the store itself; once read it passes as the
/// same view in memory does.
#[cfg(feature = "test-support")]
#[test]
fn a_stored_soa_view_is_checked_once_the_reader_service_has_read_it() {
    use crate::waveform::gated_reads::{Gate, stored};
    use std::sync::atomic::Ordering;
    static GATE: Gate = Gate::closed();
    let memory = soa_with_history();
    memory
        .validate_retained_evidence()
        .expect("the history and its view agree");
    let mut kept = memory.clone();
    let mut columns = Vec::new();
    for wave in &mut kept.waveforms {
        for values in [&mut wave.x, &mut wave.y] {
            let (column, held) = stored(&values.samples(..).collect::<Vec<_>>());
            columns.push(column);
            *values = held;
        }
    }
    crate::waveform::never_wait_for_reads_here();
    crate::waveform::read_through(&GATE, || {
        let pending = kept
            .validate_retained_evidence()
            .expect_err("the view is still being read");
        assert!(crate::waveform::is_pending_read(&pending), "{pending}");
        assert!(
            columns
                .iter()
                .all(|column| column.rows_read.load(Ordering::SeqCst) == 0),
            "the asker reads nothing of the store"
        );
        GATE.open();
        kept.validate_retained_evidence()
            .expect("the view passes once it is read");
    });
}
