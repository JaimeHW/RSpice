//! Checked QPAC fixtures shared by application integration tests.
use rspice_core::engine::*;

pub(crate) fn qpac_retained_test_fixture() -> crate::state::AnalysisResult {
    let deck =
        "QPAC retained response\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n";
    let carrier = QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]);
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
    let source = format!(
        "{deck}{}\n{}\n.end\n",
        carrier.to_spice().unwrap(),
        request.to_spice().unwrap()
    );
    let mut retained = super::retained_manual_fixture(&source, crate::state::AnalysisType::Qpac);
    retained.label = "QPAC".into();
    retained
}
