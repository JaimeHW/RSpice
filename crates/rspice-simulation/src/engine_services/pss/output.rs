//! Report a shooting orbit without changing its authenticated carrier state.

use super::*;
use rspice_core::analysis::transient::TransientOutputProjection;

pub(super) fn projection(
    source_times: &[Value],
    options: &rspice_core::netlist::SimulationOptions,
    limits: rspice_core::ResourceLimits,
    channels: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TransientOutputProjection> {
    ensure_not_aborted(abort)?;
    let period = source_times
        .last()
        .copied()
        .ok_or_else(|| ServiceRunError::Failure("PSS output has no solved period".into()))?;
    let projection = if options.output_time_points.is_empty() {
        let scheduled = TransientOutputProjection::from_accepted_times(
            source_times,
            &[],
            options.output_interval_schedule.as_ref(),
            0.0,
            period,
            limits.max_analysis_points,
        )
        .map_err(ServiceRunError::Failure)?;
        if options.output_interval_schedule.is_some() {
            // Xyce's writer can emit a repeated terminal event, with a held
            // previous state on the first copy. A plotted periodic waveform
            // needs one value at its actual phase, including the endpoint.
            let mut times = scheduled.times().to_vec();
            times.dedup();
            TransientOutputProjection::interpolate_times(
                source_times,
                &times,
                limits.max_analysis_points,
            )
        } else {
            Ok(scheduled)
        }
    } else {
        if options.output_interval_schedule.is_some() {
            return Err(ServiceRunError::Failure(
                "PSS output cannot combine a strobe interval and explicit reporting times".into(),
            ));
        }
        let mut times = Vec::new();
        for (index, &time) in options.output_time_points.iter().enumerate() {
            poll_periodically(abort, index)?;
            if !time.is_finite() || time < 0.0 {
                return Err(ServiceRunError::Failure(
                    "PSS reporting times must be finite and nonnegative".into(),
                ));
            }
            if time <= period {
                times.push(time);
            }
        }
        // Match transient reporting's inclusive final sample. Output-only
        // times are interpolated; the retained shooting mesh stays intact.
        times.push(period);
        times.sort_by(Value::total_cmp);
        times.dedup();
        TransientOutputProjection::interpolate_times(
            source_times,
            &times,
            limits.max_analysis_points,
        )
    }
    .map_err(ServiceRunError::Failure)?;
    let requested = projection
        .times()
        .len()
        .saturating_mul(channels.saturating_add(1));
    if requested > limits.max_result_values {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            requested,
            limits.max_result_values,
        ));
    }
    ensure_not_aborted(abort)?;
    Ok(projection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pss_reporting_schedule_respects_output_budgets_and_cancellation() {
        let coarse = rspice_core::Netlist::parse(
            "output endpoint\n.options OUTPUT INITIAL_INTERVAL=.3\n.end\n",
        )
        .unwrap();
        let view = projection(
            &[0.0, 0.95, 1.0],
            &coarse.options,
            Default::default(),
            1,
            &NoAbort,
        )
        .unwrap();
        assert_eq!(view.times().len(), 5);
        assert!(view.times().windows(2).all(|pair| pair[1] > pair[0]));
        assert_eq!(view.project(&[0.0, 1.9, 2.0]).unwrap().last(), Some(&2.0));
        let netlist = rspice_core::Netlist::parse(
            "output budget\n.options OUTPUT INITIAL_INTERVAL=1u\n.end\n",
        )
        .unwrap();
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_analysis_points = 10;
        assert!(projection(&[0.0, 1e-3], &netlist.options, limits, 2, &NoAbort).is_err());
        limits.max_analysis_points = 10_000;
        limits.max_result_values = 2;
        assert!(matches!(
            projection(&[0.0, 1e-3], &netlist.options, limits, 2, &NoAbort),
            Err(ServiceRunError::ResourceLimit(_))
        ));
        let cancelled = rspice_core::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            projection(&[0.0, 1e-3], &netlist.options, limits, 2, &cancelled),
            Err(ServiceRunError::Aborted)
        ));
    }
}
