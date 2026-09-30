//! Real QPXF fixtures shared by application integration tests.
use rspice_simulation::results::SimulationResult;
use std::sync::Arc;

pub(crate) fn qpxf_retained_test_fixture() -> crate::state::AnalysisResult {
    crate::simulation::controller::SimulationController::new()
        .convert_to_analysis_result_with_metadata_owned(
            qpxf_test_fixture(),
            crate::state::AnalysisType::Qpxf,
            "QPXF",
        )
}
pub(crate) fn qpxf_test_fixture() -> SimulationResult {
    use rspice_core::engine::*;
    let netlist = rspice_core::Netlist::parse(
        "QPXF response\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n.end\n",
    )
    .unwrap();
    let engine = Engine::new(Default::default());
    let point = engine
        .run_qpss(
            &netlist,
            QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]),
        )
        .unwrap();
    let request = QpxfRequest {
        frequencies_hz: vec![-37.0, 0.0, 127.0],
        frequency_axis: QpxfFrequencyAxis::Output,
        input_sources: QpxfSources::AllIndependent,
        input_lattices: QpxfInputLattices::Explicit(vec![vec![1, -1], vec![0, 0]]),
        output: QpxfOutput::Voltage {
            positive: "out".into(),
            negative: "0".into(),
        },
        output_lattice: vec![1, -1],
        linear: Default::default(),
        group_delay: true,
        group_delay_magnitude_floor: 1e-8,
    };
    SimulationResult::from_qpxf_response(Arc::new(
        engine
            .run_qpxf_from_qpss(&netlist, request, &point)
            .unwrap(),
    ))
    .unwrap()
}
