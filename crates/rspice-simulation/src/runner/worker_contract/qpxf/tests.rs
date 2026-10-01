//! QPXF worker transport and retained evidence checks.
use super::*;

#[test]
fn qpxf_worker_and_retained_display_preserve_complex_response_and_units() {
    let original = crate::results::test_fixtures::qpxf_test_fixture();
    let packet = WorkerResponseTransport::from_response(WorkerResponse::from_result_for_transfer(
        72,
        Ok(original),
    ))
    .unwrap();
    assert!(!packet.buffers.is_empty());
    let metadata = serde_json::to_string(&packet.response).unwrap();
    let restored = WorkerResponseTransport {
        response: serde_json::from_str(&metadata).unwrap(),
        ..packet.clone()
    }
    .into_response()
    .unwrap()
    .into_result()
    .unwrap();
    let SimulationResult::Qpxf {
        response,
        waveforms,
        frequencies,
    } = &restored
    else {
        panic!("wrong family")
    };
    assert_eq!(frequencies, &[-37.0, 0.0, 127.0]);
    assert_eq!(response.transfers.len(), 4);
    assert!(
        waveforms
            .iter()
            .any(|(name, trace)| name.starts_with("H(") && trace.y_unit == "Ω")
    );
    assert!(
        waveforms
            .iter()
            .any(|(name, trace)| name.starts_with("H(") && trace.y_unit == "1")
    );
    assert!(
        waveforms
            .iter()
            .any(|(name, trace)| name.starts_with("group_delay(")
                && trace.y_unit == "s"
                && trace.y_imag.is_none())
    );
    assert!(
        !waveforms
            .keys()
            .any(|name| name.starts_with("V(") || name.starts_with("I("))
    );
    let expected = Arc::clone(response);
    let retained = crate::result_conversion::convert(
        restored,
        rspice_results::analysis_type::AnalysisType::Qpxf,
        "QPXF",
        || 0.0,
    );
    assert!(retained.success, "{:?}", retained.error_message);
    retained.validate_retained_evidence().unwrap();
    let Some(rspice_results::analysis_payload::AnalysisResultPayload::Qpxf { response }) =
        &retained.result_payload
    else {
        panic!("missing QPXF payload")
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
        panic!("no worker response")
    };
    let WorkerSimulationResult::Qpxf { waveforms, .. } = result.as_mut() else {
        panic!("wrong family")
    };
    waveforms[0].y_values[0] += 1.0;
    assert!(worker.into_result().is_err());
}
