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
mod manager;
pub use manager::SoAManager;

/// Find the retained active interval containing `worst_index` using each sample's limit.
pub fn dynamic_active_interval_indices(
    values: &[f64],
    worst_index: usize,
    threshold: impl Fn(usize) -> f64,
) -> Option<(usize, usize)> {
    if worst_index >= values.len() || values[worst_index] <= threshold(worst_index) {
        return None;
    }
    let mut start = worst_index;
    while start > 0 && values[start - 1] > threshold(start - 1) {
        start -= 1;
    }
    let mut end = worst_index;
    while end + 1 < values.len() && values[end + 1] > threshold(end + 1) {
        end += 1;
    }
    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn active_interval_indices(
        values: &[f64],
        worst_index: usize,
        threshold: f64,
    ) -> Option<(usize, usize)> {
        dynamic_active_interval_indices(values, worst_index, |_| threshold)
    }

    #[test]
    fn worst_interval_expands_only_across_contiguous_active_samples() {
        let values = [0.1, 1.1, 1.3, 0.8, 1.4, 0.7];
        assert_eq!(active_interval_indices(&values, 2, 1.0), Some((1, 2)));
        assert_eq!(active_interval_indices(&values, 4, 1.0), Some((4, 4)));
        assert_eq!(active_interval_indices(&values, 3, 1.0), None);
        let stress = [0.8, 0.5, 0.2];
        let limits = [1.0, 0.4, 0.0];
        assert_eq!(
            dynamic_active_interval_indices(&stress, 2, |i| limits[i]),
            Some((1, 2))
        );
        assert_eq!(
            dynamic_active_interval_indices(&stress, 0, |i| limits[i]),
            None
        );
    }
}
