//! Checked QPNOISE fixtures shared by application integration tests.
use rspice_core::engine::*;

pub(crate) fn qpnoise_retained_test_fixture() -> crate::state::AnalysisResult {
    let deck = "Noise outputs\nV1 in 0 DC 1\nRs in out 1k\nRl out 0 2k\nL1 out sense 1m\nR3 sense 0 100\nC1 out 0 100n\n";
    let carrier = QpssConfig::new(vec![1000.0, 1414.213562373095], vec![1, 1]);
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
    let source = format!(
        "{deck}{}\n{}\n.end\n",
        carrier.to_spice().unwrap(),
        request.to_spice().unwrap()
    );
    let mut retained = super::retained_manual_fixture(&source, crate::state::AnalysisType::Qpnoise);
    retained.label = "QPNOISE".into();
    retained
}
