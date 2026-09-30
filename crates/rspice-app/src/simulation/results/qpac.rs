//! Real QPAC fixtures shared by application integration tests.
use rspice_simulation::results::SimulationResult;
use std::sync::Arc;

pub(crate) fn qpac_retained_test_fixture() -> crate::state::AnalysisResult {
    crate::simulation::controller::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(
            qpac_test_fixture(),
            crate::state::AnalysisType::Qpac,
            "QPAC",
        )
}

pub(crate) fn qpac_test_fixture() -> SimulationResult {
    let deck = "QPAC retained response\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n.end\n";
    let netlist = rspice_core::Netlist::parse(deck).unwrap();
    let engine = rspice_core::engine::Engine::new(Default::default());
    let point = engine
        .run_qpss(
            &netlist,
            rspice_core::engine::QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
        )
        .unwrap();
    let request = rspice_core::engine::QpacRequest {
        offsets_hz: vec![-37.0, 0.0, 127.0],
        input_source: "Iprobe".into(),
        input_lattice: vec![1, -1],
        output_node: "out".into(),
        output_ref: "0".into(),
        output_lattice: vec![1, -1],
        magnitude: 0.002,
        phase_degrees: 73.0,
        solver: Default::default(),
    };
    let result = engine
        .run_qpac_from_qpss(&netlist, request, &point)
        .unwrap();
    SimulationResult::from_qpac_response(Arc::new(result)).unwrap()
}
