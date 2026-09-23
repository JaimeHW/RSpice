use super::*;

#[test]
fn reliability_recovery_studio_draft_dispatch_worker_plots_and_saved_project() {
    use crate::simulation::dialog::{ReliabilityDialogState, reliability::ReliabilityConfig};
    use crate::simulation::reliability_engine::tests::{RECOVERY_DECK, recovery_fixture};
    use crate::simulation::runner::worker_contract::{
        WorkerAnalysisSpec, round_trip_response_for_test,
    };
    use rspice_core::analysis::reliability::SECONDS_PER_AGING_YEAR;
    let config = ReliabilityConfig {
        study: Some(recovery_fixture()),
        target_years: vec![1.0 / SECONDS_PER_AGING_YEAR, 2.0 / SECONDS_PER_AGING_YEAR],
        enable_hci: false,
        enable_nbti: true,
        enable_em: false,
        min_stress_voltage: 0.5,
    };
    let draft = ReliabilityDialogState::from_config(&config);
    let restored: ReliabilityDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
    let r = restored.to_config().unwrap();
    assert_eq!(r, config);
    let spec = AnalysisSpec::Reliability {
        study: r.study,
        target_years: r.target_years,
        enable_hci: r.enable_hci,
        enable_nbti: r.enable_nbti,
        enable_em: r.enable_em,
        min_stress_voltage: r.min_stress_voltage,
    };
    let wire = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: WorkerAnalysisSpec =
        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
    let restored = AnalysisSpec::from(restored);
    assert_eq!(restored, spec);
    let result = run_spec_request(
        &EngineBridge::new(),
        restored,
        Default::default(),
        RECOVERY_DECK,
        None,
        &ResolvedExecutionDependencies::default(),
        &rspice_core::NoAbort,
    )
    .unwrap();
    let result = round_trip_response_for_test(result);
    let SimulationResult::ReliabilityMission {
        response,
        waveforms,
        ..
    } = &result
    else {
        panic!("wrong result");
    };
    let occupancies = &waveforms["Occupancy(M1 / nbti / interface)"];
    let expected = [
        1.0 - (-2.0f64).exp(),
        (1.0 - (-2.0f64).exp()) * (-3.0f64).exp(),
    ];
    for (i, expected) in expected.into_iter().enumerate() {
        assert!((occupancies.y_values[i] - expected).abs() < 1e-12);
        assert!((response.aged[i].parameters[0].shift + 0.1 * expected).abs() < 1e-13);
    }
    assert_eq!(occupancies.y_unit, "1");
    assert!(waveforms.contains_key("Elapsed history(M1 / nbti)"));
    assert!(!waveforms.contains_key("Equivalent age(M1 / nbti)"));
    let expected = response.clone();
    let retained = crate::simulation::controller::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(
            result,
            crate::state::AnalysisType::Reliability,
            "NBTI recovery",
        );
    retained.validate_retained_evidence().unwrap();
    let state = crate::simulation::engine_bridge::nested_dc_tests::history(retained);
    let saved = crate::io::project_io::ProjectSimulationResults::from_state(&state);
    let decoded: crate::io::project_io::ProjectSimulationResults =
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    let loaded = decoded.into_simulation_state().unwrap();
    let Some(crate::state::AnalysisResultPayload::ReliabilityMission { response }) =
        &loaded.runs[0].analyses[0].result_payload
    else {
        panic!("missing saved evidence");
    };
    assert_eq!(response, &expected);
}
