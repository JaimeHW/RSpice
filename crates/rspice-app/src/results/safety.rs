//! Safe operating area evidence and deterministic limit evaluation.
//!
//! Owns Safe Operating Area (SOA) records used by saved results, workers, and
//! viewers. Evaluation has no application-state or host-storage dependency.

pub(crate) mod soa_manager;

pub use soa_manager::{
    SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter, SoARuleVerdict, SoAViolation,
    SoaVoltageBasis, ViolationSeverity, soa_stress_waveform_name,
};

mod thresholds;
pub use thresholds::SoaThresholds;

mod current_envelope;
mod power_derating;
pub use current_envelope::{
    SoaCurrentEnvelope, SoaCurrentEnvelopeEvidence, SoaEnvelopeSamples, SoaPulseCurve,
    SoaPulseInterpolation, SoaVoltageInterpolation, soa_envelope_limit_waveform_name,
    soa_envelope_voltage_waveform_name,
};
pub use power_derating::{
    SoaDeratingSamples, SoaPowerDerating, SoaPowerDeratingEvidence, compare_soa_stress,
    soa_derating_temperature_waveform_name, soa_power_limit_waveform_name,
};

mod duration;
#[cfg(test)]
pub use duration::SoaCumulativeDurationEvidence;
pub use duration::{
    SoaDurationEvidence, SoaDurationMode, SoaLimitTrace, qualify_soa_duration_with_mode,
    soa_duration_verdict,
};
