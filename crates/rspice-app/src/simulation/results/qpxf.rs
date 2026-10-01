//! Checked QPXF fixtures shared by application integration tests.
use rspice_core::engine::*;

pub(crate) fn qpxf_retained_test_fixture() -> crate::state::AnalysisResult {
    let deck = "QPXF response\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n";
    let carrier = QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]);
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
    let source = format!(
        "{deck}{}\n{}\n.end\n",
        carrier.to_spice().unwrap(),
        request.to_spice().unwrap()
    );
    let mut retained = super::retained_manual_fixture(&source, crate::state::AnalysisType::Qpxf);
    retained.label = "QPXF".into();
    retained
}
