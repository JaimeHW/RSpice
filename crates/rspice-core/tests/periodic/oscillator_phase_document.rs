//! Oscillator phase-error units survive public result-document serialization.

use rspice_core::engine::{OscPnoiseResult, PeriodicNoiseResult};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, SignalUnit,
};

#[test]
fn oscillator_phase_error_document_retains_physical_units() {
    let mut result = OscPnoiseResult {
        frequencies: vec![1.0, 2.0],
        phase_noise_dbc: vec![-30.0, -36.0],
        phase_error_psd: vec![200.0, 50.0],
        diffusion_constant: 1e-4,
        period: 1e-3,
        corner_hz: std::f64::consts::PI * 100.0,
        integrated_phase_noise: None,
        phase_noise_contributors: Vec::new(),
    };
    result.integrate_band().unwrap();
    let document = AnalysisResultDocument::from_pnoise(
        AnalysisInstanceId::new(AnalysisKind::PNoise, 0),
        &PeriodicNoiseResult::Oscillator {
            output: "V(osc)".into(),
            result,
        },
    )
    .unwrap()
    .build()
    .unwrap();
    let signal = document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "phase_error_psd")
        .unwrap();
    assert_eq!(
        signal.descriptor().unit(),
        &SignalUnit::Custom("rad^2/Hz".into())
    );
    assert_eq!(
        AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
        document
    );
}
