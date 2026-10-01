//! Private numerical fixtures for result and worker transport tests.
use super::SimulationResult;
use std::sync::Arc;

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
        &rspice_simulation_contract::quasi_periodic_draft::QuasiPeriodicNoiseDraft {
            explicit_frequencies: "100,300,700".into(),
            additional_outputs: vec![
                rspice_simulation_contract::quasi_periodic_draft::QpnoiseOutputDraft {
                    current: true,
                    branch: "L1".into(),
                    ..Default::default()
                },
                rspice_simulation_contract::quasi_periodic_draft::QpnoiseOutputDraft {
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
