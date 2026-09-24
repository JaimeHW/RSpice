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
    fn pss_reporting_schedules_preserve_the_authenticated_orbit() {
        let deck =
            "PSS reporting\nV1 in 0 SIN(0 1 1k)\nR1 in out 1k\nC1 out 0 159.15494309189535n\n";
        let mut config = PssRunConfig::new(1e3, vec!["V1".into()], 4, 1e-8);
        config.tstab_periods = 0;
        config.points_per_period = 512;
        let run = |options: &str| {
            run_pss_analysis_with_config_and_source_path_and_abort(
                &format!("{deck}{options}\n.end\n"),
                &config,
                None,
                &NoAbort,
            )
            .unwrap()
        };
        let full = run("");
        for (options, expected) in [
            (
                ".options OUTPUT INITIAL_INTERVAL=250u",
                vec![0.0, 250e-6, 500e-6, 750e-6, 1e-3],
            ),
            (
                ".options OUTPUT OUTPUTTIMEPOINTS=173u,333u",
                vec![173e-6, 333e-6, 1e-3],
            ),
        ] {
            let reported = run(options);
            assert_eq!(reported.time.len(), expected.len());
            for (actual, expected) in reported.time.iter().zip(expected) {
                assert!((actual - expected).abs() < 1e-15);
            }
            let retained = &reported.operating_point.analysis().result;
            let baseline = &full.operating_point.analysis().result;
            assert_eq!(retained.time, baseline.time);
            assert_eq!(retained.node_names, baseline.node_names);
            assert_eq!(retained.branch_names, baseline.branch_names);
            for (actual, expected) in retained
                .waveforms
                .iter()
                .chain(&retained.branch_waveforms)
                .zip(baseline.waveforms.iter().chain(&baseline.branch_waveforms))
            {
                assert_eq!(actual.values, expected.values);
            }
            assert_eq!(
                reported.operating_point.shooting_state(),
                full.operating_point.shooting_state()
            );
            assert!(retained.time.len() > reported.time.len());
            let values = &reported
                .waveforms
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
                .unwrap()
                .1;
            for (&time, &voltage) in reported.time.iter().zip(values) {
                let phase = std::f64::consts::TAU * 1e3 * time;
                assert!((voltage - 0.5 * (phase.sin() - phase.cos())).abs() < 5e-5);
            }
            // Exercise the same serialization boundary used by Studio, including
            // reconstruction from the retained orbit and reporting-time buffer.
            use crate::simulation::results::{SimulationResult, WaveformData};
            let time = reported.time;
            let waveforms: std::collections::HashMap<_, _> = reported
                .waveforms
                .into_iter()
                .map(|(name, values)| {
                    let unit = if name.starts_with("I(") { "A" } else { "V" };
                    let waveform = WaveformData::new_time_domain_in_unit(
                        name.clone(),
                        time.clone(),
                        values,
                        unit,
                    );
                    (name, waveform)
                })
                .collect();
            let expected_time = time.clone();
            let expected_waveforms = waveforms.clone();
            let display = SimulationResult::Transient {
                spectra: Vec::new(),
                time,
                waveforms,
                measurements: Vec::new(),
                periodic_state: Some(reported.operating_point.clone()),
                convergence: None,
                events: Default::default(),
            };
            let restored =
                crate::simulation::runner::worker_contract::round_trip_response_for_test(display);
            let SimulationResult::Transient {
                time,
                waveforms,
                periodic_state: Some(point),
                ..
            } = restored
            else {
                panic!("expected PSS display");
            };
            assert_eq!(time, expected_time);
            assert_eq!(waveforms.len(), expected_waveforms.len());
            for (name, expected) in expected_waveforms {
                let actual = &waveforms[&name];
                assert_eq!(actual.name, expected.name);
                assert_eq!(actual.x_values, expected.x_values);
                assert_eq!(actual.y_values, expected.y_values);
                assert_eq!(actual.y_unit, expected.y_unit);
            }
            assert_eq!(point, reported.operating_point);
        }
    }

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
