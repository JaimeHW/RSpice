//! Safety Services Module
//!
//! Provides Safe Operating Area (SOA) and design rule checking.

pub(crate) mod soa_manager;

pub use soa_manager::{
    SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter, SoARuleVerdict, SoAViolation,
    SoaVoltageBasis, ViolationSeverity, soa_stress_waveform_name,
};

mod thresholds;
pub use thresholds::SoaThresholds;

mod power_derating;
pub use power_derating::{
    SoaDeratingSamples, SoaPowerDerating, SoaPowerDeratingEvidence, compare_soa_stress,
    soa_derating_temperature_waveform_name, soa_power_limit_waveform_name,
};

mod duration;
pub use duration::{
    SoaCumulativeDurationEvidence, SoaDurationEvidence, SoaDurationMode, SoaLimitTrace,
    qualify_soa_duration_with_mode, soa_duration_verdict,
};
