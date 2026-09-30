//! Real QPNOISE fixtures shared by application integration tests.
use rspice_simulation::results::SimulationResult;
use std::sync::Arc;

pub(crate) fn qpnoise_retained_test_fixture() -> crate::state::AnalysisResult {
    crate::simulation::controller::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(
            qpnoise_test_fixture(),
            crate::state::AnalysisType::Qpnoise,
            "QPNOISE",
        )
}
pub(crate) fn qpnoise_test_fixture() -> SimulationResult {
    let netlist=rspice_core::Netlist::parse("Noise outputs\nV1 in 0 DC 1\nRs in out 1k\nRl out 0 2k\nL1 out sense 1m\nR3 sense 0 100\nC1 out 0 100n\n.end\n").unwrap();
    let engine = rspice_core::engine::Engine::default();
    let point = engine
        .run_qpss(
            &netlist,
            rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
        )
        .unwrap();
    let request = rspice_core::engine::QpnoiseRequest::from_qpnoise_card(
        &crate::simulation::plan::QuasiPeriodicNoiseDraft {
            explicit_frequencies: "100,300,700".into(),
            additional_outputs: vec![
                crate::simulation::plan::QpnoiseOutputDraft {
                    current: true,
                    branch: "L1".into(),
                    ..Default::default()
                },
                crate::simulation::plan::QpnoiseOutputDraft {
                    lattice: "1,-1".into(),
                    ..Default::default()
                },
            ],
            noise_figure: true,
            source_resistor: "Rs".into(),
            integration_method: rspice_core::engine::QpnoiseIntegrationMethod::LogLog,
            ..Default::default()
        }
        .to_spec()
        .unwrap()
        .qpnoise_card()
        .unwrap(),
    )
    .unwrap();
    SimulationResult::from_qpnoise_response(Arc::new(
        engine
            .run_qpnoise_from_qpss(&netlist, request, &point)
            .unwrap(),
    ))
    .unwrap()
}
