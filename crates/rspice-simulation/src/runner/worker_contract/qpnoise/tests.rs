//! Worker transport, plot and retained evidence validation.
use super::*;
#[test]
fn qpnoise_worker_retained_evidence_and_typed_plots_roundtrip() {
    let packet = WorkerResponseTransport::from_response(WorkerResponse::from_result_for_transfer(
        73,
        Ok(crate::results::test_fixtures::qpnoise_test_fixture()),
    ))
    .unwrap();
    let metadata = serde_json::to_string(&packet.response).unwrap();
    let restored = WorkerResponseTransport {
        response: serde_json::from_str(&metadata).unwrap(),
        ..packet.clone()
    }
    .into_response()
    .unwrap()
    .into_result()
    .unwrap();
    let SimulationResult::Qpnoise {
        response,
        waveforms,
        frequencies,
    } = &restored
    else {
        panic!("wrong family")
    };
    assert_eq!(frequencies, &[100.0, 300.0, 700.0]);
    assert_eq!(response.outputs.len(), 3);
    assert!(response.outputs[2].frequencies_hz[0] < 0.0);
    for unit in ["V²/Hz", "A²/Hz", "V/√Hz", "A/√Hz", "dB", "V·A/Hz"] {
        assert!(
            waveforms.values().any(|w| w.y_unit == unit),
            "missing {unit}"
        );
    }
    assert!(
        waveforms
            .values()
            .all(|w| w.y_values.iter().all(|v| v.is_finite()))
    );
    assert!(
        response.outputs[2]
            .input_noise
            .as_ref()
            .unwrap()
            .iter()
            .any(|v| matches!(v, rspice_core::engine::QpnoiseValue::Unavailable(_)))
    );
    let expected = response.clone();
    let retained = crate::result_conversion::convert(
        restored,
        rspice_results::analysis_type::AnalysisType::Qpnoise,
        "QPNOISE",
        || 0.0,
    );
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Qpnoise { response }) =
        &retained.result_payload
    else {
        panic!("missing QPNOISE payload")
    };
    assert_eq!(response, &expected);
    let mut changed = retained.clone();
    Arc::make_mut(&mut changed.waveforms[0].x)[0] += 1.0;
    assert!(changed.validate_retained_evidence().is_err());
    let mut changed = retained.clone();
    changed.waveforms[0].unit = Some("wrong".into());
    assert!(changed.validate_retained_evidence().is_err());
    let mut worker = packet.into_response().unwrap();
    let WorkerOutcome::Success(result) = &mut worker.outcome else {
        panic!("failed")
    };
    let WorkerSimulationResult::Qpnoise { waveforms, .. } = result.as_mut() else {
        panic!("wrong family")
    };
    waveforms[0].y_values[0] += 1.0;
    assert!(worker.into_result().is_err());
}
