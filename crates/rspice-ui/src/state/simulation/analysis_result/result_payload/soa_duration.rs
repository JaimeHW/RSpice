//! Recompute excursion qualifications from the complete retained samples.
use super::*;
use crate::services::safety::{
    SoARuleVerdict, SoaLimitTrace, compare_soa_stress, qualify_soa_duration_with_mode,
    soa_duration_verdict, soa_power_limit_waveform_name, soa_stress_waveform_name,
};

pub(super) struct DurationSamples {
    pub verdicts: Vec<SoARuleVerdict>,
    pub events: usize,
}

pub(super) fn severity(verdict: SoARuleVerdict) -> Option<SoaViolationSeverityEvidence> {
    match verdict {
        SoARuleVerdict::Pass => None,
        SoARuleVerdict::Warning => Some(SoaViolationSeverityEvidence::Warning),
        SoARuleVerdict::Violation => Some(SoaViolationSeverityEvidence::Violation),
        SoARuleVerdict::Critical => Some(SoaViolationSeverityEvidence::Critical),
    }
}

pub(super) fn validate(
    analysis: &AnalysisResult,
    time: &[f64],
    evaluation: &SoaEvaluationEvidence,
) -> Result<DurationSamples, String> {
    let duration = evaluation.duration.ok_or("Missing SOA duration policy")?;
    let parameter = evaluation.parameter.runtime_parameter();
    let unit = match parameter {
        crate::services::safety::SoAParameter::Temp => "K",
        crate::services::safety::SoAParameter::Pdiss => "W",
        parameter if parameter.is_current() => "A",
        _ => "V",
    };
    if evaluation.unit != unit {
        return Err("SOA duration rule has the wrong physical unit".into());
    }
    let stress = super::soa_derating::trace(
        analysis,
        &soa_stress_waveform_name(&evaluation.device_id, parameter),
        time,
        unit,
    )?;
    let limits = if evaluation.envelope.is_some() {
        SoaLimitTrace::Samples(super::soa_derating::trace(
            analysis,
            &crate::services::safety::soa_envelope_limit_waveform_name(
                &evaluation.device_id,
                parameter,
            ),
            time,
            "A",
        )?)
    } else if evaluation.derating.is_some() {
        SoaLimitTrace::Samples(super::soa_derating::trace(
            analysis,
            &soa_power_limit_waveform_name(&evaluation.device_id),
            time,
            "W",
        )?)
    } else {
        SoaLimitTrace::Constant(evaluation.limit_value)
    };
    let scan = qualify_soa_duration_with_mode(
        time,
        stress,
        limits,
        duration.minimum_duration_s,
        duration.mode(),
        &rspice_core::abort_signal::NoAbort,
    )
    .map_err(|error| error.to_string())?;
    if scan.evidence != duration {
        return Err("SOA duration summary contradicts its retained samples".into());
    }
    let verdicts: Vec<_> = (0..time.len())
        .map(|i| {
            soa_duration_verdict(
                evaluation.thresholds,
                stress[i],
                limits.at(i),
                scan.qualified_samples[i],
            )
        })
        .collect();
    let mut worst = 0;
    for i in 1..time.len() {
        if verdicts[i]
            .cmp(&verdicts[worst])
            .then_with(|| {
                compare_soa_stress(stress[i], limits.at(i), stress[worst], limits.at(worst))
            })
            .is_gt()
        {
            worst = i;
        }
    }
    let verdict = match verdicts[worst] {
        SoARuleVerdict::Pass => SoaRuleVerdictEvidence::Pass,
        SoARuleVerdict::Warning => SoaRuleVerdictEvidence::Warning,
        SoARuleVerdict::Violation => SoaRuleVerdictEvidence::Violation,
        SoARuleVerdict::Critical => SoaRuleVerdictEvidence::Critical,
    };
    if verdict != evaluation.verdict
        || !same_retained_float(stress[worst], evaluation.worst_actual_value)
        || !same_retained_float(time[worst], evaluation.worst_time_s)
        || !same_retained_float(limits.at(worst), evaluation.limit_value)
    {
        return Err("SOA duration-qualified worst point contradicts its retained samples".into());
    }
    let events = verdicts
        .iter()
        .filter(|verdict| **verdict != SoARuleVerdict::Pass)
        .count();
    Ok(DurationSamples { verdicts, events })
}
