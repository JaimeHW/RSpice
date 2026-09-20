//! Cross-check derating metadata against the complete retained waveforms.
use super::*;
use crate::services::safety::{
    SoAParameter, compare_soa_stress, soa_derating_temperature_waveform_name,
    soa_power_limit_waveform_name, soa_stress_waveform_name,
};

pub(super) fn trace<'a>(
    analysis: &'a AnalysisResult,
    name: &str,
    time: &[f64],
    unit: &str,
) -> Result<&'a [f64], String> {
    let wave = analysis
        .waveforms
        .iter()
        .find(|wave| wave.name == name)
        .ok_or_else(|| format!("SOA derating is missing retained trace '{name}'"))?;
    if wave.y.len() != time.len()
        || wave.x.as_slice() != time
        || wave.unit.as_deref() != Some(unit)
        || wave.complex.is_some()
    {
        return Err(format!(
            "SOA derating trace '{name}' has an invalid axis, unit or sample coverage"
        ));
    }
    Ok(&wave.y)
}

pub(super) fn validate(
    analysis: &AnalysisResult,
    time: &[f64],
    evaluation: &SoaEvaluationEvidence,
) -> Result<usize, String> {
    let Some(derating) = evaluation.derating else {
        return Ok(0);
    };
    derating.validate()?;
    let limits = trace(
        analysis,
        &soa_power_limit_waveform_name(&evaluation.device_id),
        time,
        "W",
    )?;
    let temperatures = trace(
        analysis,
        &soa_derating_temperature_waveform_name(&evaluation.device_id),
        time,
        "K",
    )?;
    let stress = trace(
        analysis,
        &soa_stress_waveform_name(&evaluation.device_id, SoAParameter::Pdiss),
        time,
        "W",
    )?;
    if time.is_empty() {
        return Err("SOA derating requires accepted samples".into());
    }
    let mut worst = 0;
    let mut events = 0;
    for i in 0..time.len() {
        if !temperatures[i].is_finite()
            || temperatures[i] <= 0.0
            || !stress[i].is_finite()
            || stress[i] < 0.0
            || !limits[i].is_finite()
            || limits[i] < 0.0
            || !same_retained_float(
                limits[i],
                derating
                    .curve
                    .limit(derating.rated_power_w, temperatures[i]),
            )
        {
            return Err(format!(
                "SOA derating for '{}' has contradictory temperature, stress or allowed power at sample {i}",
                evaluation.device_id
            ));
        }
        if compare_soa_stress(stress[i], limits[i], stress[worst], limits[worst]).is_gt() {
            worst = i;
        }
        if evaluation.thresholds.verdict(stress[i], limits[i])
            != crate::services::safety::SoARuleVerdict::Pass
        {
            events += 1;
        }
    }
    if !same_retained_float(evaluation.worst_time_s, time[worst])
        || !same_retained_float(evaluation.worst_actual_value, stress[worst])
        || !same_retained_float(evaluation.limit_value, limits[worst])
    {
        return Err(format!(
            "SOA derating for '{}' does not retain the exact point of highest utilization",
            evaluation.device_id
        ));
    }
    Ok(events)
}
