//! Output-only sampling; every accepted observation still participates in SOA.

use super::*;
use rspice_core::analysis::transient::TransientOutputProjection;

pub(super) fn projection(
    time: &[f64],
    options: &rspice_core::netlist::SimulationOptions,
    start: f64,
    limits: rspice_core::ResourceLimits,
    channels: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<TransientOutputProjection>> {
    if options.output_interval_schedule.is_none() && options.output_time_points.is_empty() {
        return Ok(None);
    }
    ensure_not_aborted(abort)?;
    let stop = *time
        .last()
        .ok_or_else(|| ServiceRunError::Failure("SOA has no observations".into()))?;
    let mut requested = if options.output_time_points.is_empty() {
        TransientOutputProjection::from_accepted_times(
            time,
            &[],
            options.output_interval_schedule.as_ref(),
            start,
            stop,
            limits.max_analysis_points,
        )
        .map_err(ServiceRunError::Failure)?
        .times()
        .to_vec()
    } else {
        if options.output_interval_schedule.is_some() {
            return Err(ServiceRunError::Failure(
                "SOA output cannot combine a strobe interval and explicit reporting times".into(),
            ));
        }
        let mut requested = Vec::new();
        for (index, &value) in options.output_time_points.iter().enumerate() {
            poll_periodically(abort, index)?;
            if !value.is_finite() || value < 0.0 {
                return Err(ServiceRunError::Failure(
                    "SOA reporting times must be finite and nonnegative".into(),
                ));
            }
            if value >= start && value <= stop {
                requested.push(value);
            }
        }
        requested
    };
    requested.push(stop);
    requested.sort_by(f64::total_cmp);
    requested.dedup();
    let projection =
        TransientOutputProjection::interpolate_times(time, &requested, limits.max_analysis_points)
            .map_err(ServiceRunError::Failure)?;
    // Both the authoritative observations and the reporting view travel with
    // the result. Charge both to the output budget before allocating columns.
    let values = time
        .len()
        .saturating_mul(channels.saturating_add(1))
        .saturating_add(
            requested
                .len()
                .saturating_mul(channels.saturating_mul(2).saturating_add(1)),
        );
    if values > limits.max_result_values {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            values,
            limits.max_result_values,
        ));
    }
    ensure_not_aborted(abort)?;
    Ok(Some(projection))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soa_reporting_window_budget_and_abort_are_checked() {
        let options = rspice_core::Netlist::parse(
            "reporting\n.options OUTPUT OUTPUTTIMEPOINTS=.1,.25,.9\n.end\n",
        )
        .unwrap()
        .options;
        let time = [0.2, 0.3, 0.55, 1.0];
        let view = projection(&time, &options, 0.2, Default::default(), 2, &NoAbort)
            .unwrap()
            .unwrap();
        assert_eq!(view.times(), &[0.25, 0.9, 1.0]);
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_result_values = 20;
        assert!(matches!(
            projection(&time, &options, 0.2, limits, 2, &NoAbort),
            Err(ServiceRunError::ResourceLimit(_))
        ));
        let abort = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        assert!(matches!(
            projection(&time, &options, 0.2, Default::default(), 2, &abort),
            Err(ServiceRunError::Aborted)
        ));
        let mut invalid = options.clone();
        invalid.output_time_points = vec![0.19];
        assert!(projection(&time, &invalid, 0.15, Default::default(), 2, &NoAbort).is_err());
        invalid.output_time_points = vec![f64::NAN];
        assert!(projection(&time, &invalid, 0.2, Default::default(), 2, &NoAbort).is_err());
    }
}
