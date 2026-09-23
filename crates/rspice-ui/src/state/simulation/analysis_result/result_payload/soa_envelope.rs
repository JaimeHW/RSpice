//! Recompute current limits from the authored curve and retained terminal voltage.
use super::*;
use crate::services::safety::{
    SoARuleVerdict, compare_soa_stress, soa_envelope_limit_waveform_name,
    soa_envelope_voltage_waveform_name, soa_stress_waveform_name,
};

pub(super) fn validate(
    analysis: &AnalysisResult,
    time: &[f64],
    evaluation: &SoaEvaluationEvidence,
) -> Result<usize, String> {
    let envelope = evaluation
        .envelope
        .as_ref()
        .ok_or("Missing SOA curve evidence")?;
    if time.is_empty() {
        return Err("SOA current curve requires accepted samples".into());
    }
    envelope.curve.validate()?;
    if envelope
        .curve
        .pulse_width_s
        .is_some_and(|width| width < time[time.len() - 1] - time[0])
    {
        return Err("SOA pulse rating is shorter than the retained window".into());
    }
    let parameter = evaluation.parameter.runtime_parameter();
    let limits = super::soa_derating::trace(
        analysis,
        &soa_envelope_limit_waveform_name(&evaluation.device_id, parameter),
        time,
        "A",
    )?;
    let volts = super::soa_derating::trace(
        analysis,
        &soa_envelope_voltage_waveform_name(&evaluation.device_id, parameter),
        time,
        "V",
    )?;
    let stress = super::soa_derating::trace(
        analysis,
        &soa_stress_waveform_name(&evaluation.device_id, parameter),
        time,
        "A",
    )?;
    let mut worst = 0;
    let mut events = 0;
    for i in 0..time.len() {
        let limit = envelope
            .maximum_current_a
            .min(envelope.curve.limit(volts[i])?);
        if !limits[i].is_finite()
            || !stress[i].is_finite()
            || stress[i] < 0.0
            || !same_retained_float(limits[i], limit)
        {
            return Err(
                "SOA current/voltage curve contradicts its retained voltage, limit or stress"
                    .into(),
            );
        }
        if compare_soa_stress(stress[i], limit, stress[worst], limits[worst]).is_gt() {
            worst = i;
        }
        if evaluation.thresholds.verdict(stress[i], limit) != SoARuleVerdict::Pass {
            events += 1;
        }
    }
    if evaluation.duration.is_none()
        && (!same_retained_float(evaluation.worst_time_s, time[worst])
            || !same_retained_float(evaluation.worst_actual_value, stress[worst])
            || !same_retained_float(evaluation.limit_value, limits[worst]))
    {
        return Err("SOA current/voltage curve does not retain its exact worst utilization".into());
    }
    Ok(events)
}
