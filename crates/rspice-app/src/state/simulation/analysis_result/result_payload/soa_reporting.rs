//! Reporting views must retain every constant-limit event and cumulative count.
use super::*;
use crate::results::safety::{SoAParameter, soa_stress_waveform_name};

pub(super) fn validate(
    analysis: &AnalysisResult,
    time: &[f64],
    evaluations: &[SoaEvaluationEvidence],
    violations: &[SoaViolationEvidence],
) -> Result<(), String> {
    let mut cumulative = vec![0.0; time.len()];
    let mut events = BTreeMap::<_, Vec<_>>::new();
    for event in violations {
        let index = time
            .binary_search_by(|time| time.total_cmp(&event.time_s))
            .map_err(|_| "SOA event is missing from its complete observation history")?;
        cumulative[index] += 1.0;
        events
            .entry((event.device_id.as_str(), event.parameter))
            .or_default()
            .push(event);
    }
    for index in 1..cumulative.len() {
        cumulative[index] += cumulative[index - 1];
    }
    if soa_derating::trace(analysis, "SOA_VIOLATION_COUNT", time, "count")? != cumulative {
        return Err("SOA cumulative count contradicts its complete event history".into());
    }
    for evaluation in evaluations
        .iter()
        .filter(|e| e.duration.is_none() && e.derating.is_none() && e.envelope.is_none())
    {
        let parameter = evaluation.parameter.runtime_parameter();
        let unit = match parameter {
            SoAParameter::Temp => "K",
            SoAParameter::Pdiss => "W",
            parameter if parameter.is_current() => "A",
            _ => "V",
        };
        if evaluation.unit != unit {
            return Err("SOA constant-limit rule has the wrong physical unit".into());
        }
        let stress = soa_derating::trace(
            analysis,
            &soa_stress_waveform_name(&evaluation.device_id, parameter),
            time,
            unit,
        )?;
        let worst = (1..time.len()).fold(0, |worst, index| {
            if crate::results::safety::compare_soa_stress(
                stress[index],
                evaluation.limit_value,
                stress[worst],
                evaluation.limit_value,
            )
            .is_gt()
            {
                index
            } else {
                worst
            }
        });
        if !same_retained_float(time[worst], evaluation.worst_time_s)
            || !same_retained_float(stress[worst], evaluation.worst_actual_value)
        {
            return Err(
                "SOA constant-limit worst point contradicts its complete observations".into(),
            );
        }
        let retained_events = events
            .remove(&(evaluation.device_id.as_str(), evaluation.parameter))
            .unwrap_or_default();
        let mut retained_events = retained_events.into_iter();
        for (index, &value) in stress.iter().enumerate() {
            if let Some(severity) =
                soa_duration::severity(evaluation.thresholds.verdict(value, evaluation.limit_value))
            {
                let event = retained_events
                    .next()
                    .ok_or("SOA constant-limit observations contain an unreported event")?;
                if event.severity != severity
                    || !same_retained_float(event.time_s, time[index])
                    || !same_retained_float(event.actual_value, value)
                {
                    return Err(
                        "SOA constant-limit event contradicts its complete observations".into(),
                    );
                }
            }
        }
        if retained_events.next().is_some() {
            return Err("SOA constant-limit event has no matching observation".into());
        }
    }
    Ok(())
}
