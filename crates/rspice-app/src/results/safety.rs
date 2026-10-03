//! Shared SOA fixtures and imports for application integration tests.

pub use rspice_results::safety::SoARuleVerdict;
pub use rspice_results::safety::SoaCumulativeDurationEvidence;
pub use rspice_results::safety::soa_stress_waveform_name;
pub use rspice_results::safety::{SoAEvaluation, SoAParameter, SoaCurrentEnvelope};
pub use rspice_results::safety::{
    SoaDurationEvidence, SoaPowerDerating, SoaPulseCurve, SoaPulseInterpolation, SoaThresholds,
    SoaVoltageInterpolation,
};

pub(crate) fn soa_current_envelope_test_fixture() -> SoaCurrentEnvelope {
    SoaCurrentEnvelope {
        source: "Synthetic SOA fixture".into(),
        conditions: "Synthetic fixed case temperature; single window".into(),
        voltages_v: vec![0.0, 1.0, 10.0],
        dc_currents_a: Some(vec![0.0, 0.01, 0.001]),
        pulses: vec![
            SoaPulseCurve {
                duration_s: 1e-9,
                currents_a: vec![0.0, 0.04, 0.004],
            },
            SoaPulseCurve {
                duration_s: 100e-9,
                currents_a: vec![0.0, 0.02, 0.002],
            },
        ],
        pulse_width_s: Some(10e-9),
        voltage_interpolation: SoaVoltageInterpolation::Logarithmic,
        pulse_interpolation: SoaPulseInterpolation::Logarithmic,
    }
}
