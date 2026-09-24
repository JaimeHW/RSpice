//! Portable SOA rule definitions, exact curve policy, and retained evidence.

mod model;
pub use model::{
    SoADefinition, SoAEvaluation, SoALimit, SoAParameter, SoARuleVerdict, SoAViolation,
    SoaVoltageBasis, ViolationSeverity, soa_stress_waveform_name,
};
mod thresholds;
pub use thresholds::SoaThresholds;
mod current_envelope;
pub use current_envelope::{
    SoaCurrentEnvelope, SoaCurrentEnvelopeEvidence, SoaEnvelopeSamples, SoaPulseCurve,
    SoaPulseInterpolation, SoaVoltageInterpolation, soa_envelope_limit_waveform_name,
    soa_envelope_voltage_waveform_name,
};
mod power_derating;
pub use power_derating::{
    SoaDeratingSamples, SoaPowerDerating, SoaPowerDeratingEvidence, compare_soa_stress,
    soa_derating_temperature_waveform_name, soa_power_limit_waveform_name,
};
mod duration;
pub use duration::{
    SoaCumulativeDurationEvidence, SoaDurationEvidence, SoaDurationMode, SoaDurationScan,
    SoaDurationScanError, SoaExcursion, SoaLimitTrace, scan_soa_duration_with_mode,
    soa_duration_verdict,
};
