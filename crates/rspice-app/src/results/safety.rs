//! App-facing SOA compatibility imports and engine error/cancellation adapters.
//!
//! Rule policy, sampled evaluation, and retained evidence live in
//! `rspice_results::safety`.

// Existing app callers migrate to the portable owner as their modules move.
#[cfg(test)]
pub use rspice_results::safety::SoaCumulativeDurationEvidence;
pub use rspice_results::safety::{
    SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter, SoARuleVerdict, SoAViolation,
    SoaCurrentEnvelope, SoaDeratingSamples, SoaDurationEvidence, SoaDurationMode,
    SoaEnvelopeSamples, SoaLimitTrace, SoaPulseInterpolation, SoaThresholds, SoaVoltageBasis,
    SoaVoltageInterpolation, ViolationSeverity, compare_soa_stress,
    soa_derating_temperature_waveform_name, soa_duration_verdict, soa_envelope_limit_waveform_name,
    soa_envelope_voltage_waveform_name, soa_power_limit_waveform_name, soa_stress_waveform_name,
};
#[cfg(test)]
pub use rspice_results::safety::{SoaPowerDerating, SoaPulseCurve};
mod duration;
pub use duration::{finalize_soa_durations, qualify_soa_duration_with_mode};

#[cfg(test)]
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
