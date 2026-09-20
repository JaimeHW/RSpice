//! Worker lifetime plots must match the retained mission evidence exactly.
use super::*;
pub(super) fn validate_worker_reliability_result(
    result: &WorkerSimulationResult,
) -> Result<(), String> {
    let WorkerSimulationResult::ReliabilityMission {
        years,
        waveforms,
        response,
    } = result
    else {
        return Ok(());
    };
    if *years != response.stress.request.target_years
        || *waveforms != worker_waveforms(SimulationResult::reliability_waveforms(response)?)
    {
        return Err("Reliability worker display differs from retained mission evidence".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reliability_mission_worker_project_and_plot_evidence_roundtrip() {
        let packet =
            WorkerResponseTransport::from_response(WorkerResponse::from_result_for_transfer(
                91,
                Ok(SimulationResult::reliability_mission_test_fixture()),
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
        let SimulationResult::ReliabilityMission {
            response,
            waveforms,
            ..
        } = &restored
        else {
            panic!("wrong family");
        };
        assert_eq!(response.aged.len(), 4);
        assert!(
            waveforms
                .values()
                .any(|w| w.name.starts_with("Shift(") && w.y_unit == "V")
        );
        let expected = response.clone();
        let retained = crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                restored,
                crate::state::AnalysisType::Reliability,
                "Reliability",
            );
        assert!(retained.success, "{:?}", retained.error_message);
        retained.validate_retained_evidence().unwrap();
        let state = crate::simulation::engine_bridge::nested_dc_tests::history(retained.clone());
        let saved = crate::io::project_io::ProjectSimulationResults::from_state(&state);
        let decoded: crate::io::project_io::ProjectSimulationResults =
            serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
        let loaded = decoded.into_simulation_state().unwrap();
        let Some(crate::state::AnalysisResultPayload::ReliabilityMission { response }) =
            &loaded.runs[0].analyses[0].result_payload
        else {
            panic!("missing mission");
        };
        assert_eq!(response, &expected);
        let mut tampered = retained.clone();
        Arc::make_mut(&mut tampered.waveforms[0].y)[0] += 1.0;
        assert!(tampered.validate_retained_evidence().is_err());
        let mut response = packet.into_response().unwrap();
        let WorkerOutcome::Success(result) = &mut response.outcome else {
            panic!("failed");
        };
        let WorkerSimulationResult::ReliabilityMission { waveforms, .. } = result.as_mut() else {
            panic!("wrong family");
        };
        waveforms[0].y_values[0] += 1.0;
        assert!(response.into_result().is_err());
    }
}
