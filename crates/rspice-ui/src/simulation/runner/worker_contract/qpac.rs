//! Worker QPAC plots must be exact projections of their retained response.
use super::*;

pub(super) fn validate_worker_qpac_result(result: &WorkerSimulationResult) -> Result<(), String> {
    let WorkerSimulationResult::Qpac {
        frequencies,
        waveforms,
        response,
    } = result
    else {
        return Ok(());
    };
    let expected_waveforms = SimulationResult::qpac_waveforms(response)?;
    if *frequencies != response.metadata.request.offsets_hz
        || *waveforms != worker_waveforms(expected_waveforms)
    {
        return Err("QPAC worker display differs from the retained complex response".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qpac_worker_saved_result_and_display_preserve_complex_response_and_units() {
        let original = SimulationResult::qpac_test_fixture();
        let packet = WorkerResponseTransport::from_response(
            WorkerResponse::from_result_for_transfer(72, Ok(original)),
        )
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
        let SimulationResult::Qpac {
            response,
            waveforms,
            frequencies,
        } = &restored
        else {
            panic!("wrong family")
        };
        assert_eq!(frequencies, &[-37.0, 0.0, 127.0]);
        assert!(
            response
                .metadata
                .output_frequencies_hz
                .iter()
                .all(|f| *f < 0.0)
        );
        assert!(
            waveforms
                .iter()
                .any(|(name, trace)| name.starts_with("H(") && trace.y_unit == "Ω")
        );
        assert!(
            waveforms
                .iter()
                .any(|(name, trace)| name.starts_with("I(") && trace.y_unit == "A")
        );
        let expected = Arc::clone(response);
        let controller = crate::simulation::controller::SimulationController::new();
        let retained = controller.convert_to_analysis_result_with_metadata_owned(
            restored,
            crate::state::AnalysisType::Qpac,
            "QPAC",
        );
        assert!(retained.success, "{:?}", retained.error_message);
        retained.validate_retained_evidence().unwrap();
        let saved = crate::io::project_io::ProjectAnalysisResult::from(&retained);
        let decoded: crate::io::project_io::ProjectAnalysisResult =
            serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
        assert_eq!(decoded, saved);
        let crate::io::project_io::PersistedField::Value(
            crate::state::AnalysisResultPayload::Qpac { response },
        ) = decoded.result_payload
        else {
            panic!("missing QPAC payload")
        };
        assert_eq!(response, expected);
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
        let WorkerSimulationResult::Qpac { waveforms, .. } = result.as_mut() else {
            panic!("wrong family")
        };
        waveforms[0].y_values[0] += 1.0;
        assert!(worker.into_result().is_err());
    }
}
