//! Apply shared output controls to envelope projection times.
use super::*;
use rspice_core::analysis::transient::TransientOutputProjection;

pub(super) fn configured_times(
    options: &rspice_core::netlist::SimulationOptions,
    source_times: &[Value],
    carriers: &[Value],
    max_points: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Vec<Value>>> {
    ensure_not_aborted(abort)?;
    let explicit = &options.output_time_points;
    let schedule = options.output_interval_schedule.as_ref();
    if explicit.is_empty() && schedule.is_none() {
        return Ok(None);
    }
    if !explicit.is_empty() && schedule.is_some() {
        return Err(ServiceRunError::Failure(
            "Envelope output cannot combine a strobe interval and explicit reporting times".into(),
        ));
    }
    let (&first, &last) = source_times
        .first()
        .zip(source_times.last())
        .ok_or_else(|| {
            ServiceRunError::Failure("Envelope reporting requires a solved time grid".into())
        })?;
    if let Some(schedule) = schedule {
        // Reuse the core scheduler on the real accepted grid: cadence changes
        // can depend on accepted steps. Only its times are needed; envelopes
        // are projected directly from the full voltage/current histories.
        let projection = TransientOutputProjection::from_accepted_times(
            source_times,
            &[],
            Some(schedule),
            first,
            last,
            max_points,
        )
        .map_err(ServiceRunError::Failure)?;
        let times =
            centered_projection_output_times(projection.times(), first, last, carriers, abort)?;
        return Ok(Some(times));
    }
    let requested = explicit.len().saturating_add(1);
    if requested > max_points {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::AnalysisPoints,
            requested,
            max_points,
        ));
    }
    let centers = centered_projection_output_times(&[first, last], first, last, carriers, abort)?;
    let minimum = centers[0];
    let maximum = *centers.last().expect("centered projection is nonempty");
    let mut times = Vec::with_capacity(requested);
    for (index, &time) in explicit.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !time.is_finite() || time < 0.0 {
            return Err(ServiceRunError::Failure(
                "Envelope reporting times must be finite and nonnegative".into(),
            ));
        }
        if time > last {
            // Match transient reporting: times beyond this run are unused.
            continue;
        }
        if time < minimum || time > maximum {
            return Err(ServiceRunError::Failure(format!(
                "Envelope reporting time {time:.12e}s requires a complete carrier window; choose a time between {minimum:.12e}s and {maximum:.12e}s"
            )));
        }
        times.push(time);
    }
    // Report the final valid center, analogous to transient's final sample.
    // Unlike an automatic schedule, explicit times are never shifted inward.
    times.push(maximum);
    times.sort_by(Value::total_cmp);
    times.dedup();
    ensure_not_aborted(abort)?;
    Ok(Some(times))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::{
        AnalysisKind, AnalysisNumericOverride, NumericOverrideOption, SolverOwnership,
    };

    #[test]
    fn envelope_reporting_controls_reach_complex_voltage_and_current_results() {
        let config = EnvelopeRunConfig {
            multirate: None,
            initialization: Default::default(),
            fundamental_freq: 1e6,
            additional_carrier_tones: Vec::new(),
            stop_time: 4e-6,
            num_harmonics: 1,
            envelope_step: Some(0.5e-6),
            modulation_sources: Vec::new(),
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
            adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
            extraction_path: EnvelopeExtractionPath::Projection,
        };
        for (option, value, expected) in [
            (
                NumericOverrideOption::StrobeInterval,
                "0.8u",
                vec![0.5, 0.8, 1.6, 2.4, 3.2, 3.5],
            ),
            (
                NumericOverrideOption::OutputTimePoints,
                "0.733u, 2.117u, 3.311u",
                vec![0.733, 2.117, 3.311, 3.5],
            ),
        ] {
            let mut options = AnalysisNumericOverride::default();
            options
                .set_for_instance(AnalysisKind::Envelope, SolverOwnership::NONE, option, value)
                .unwrap();
            let deck = format!(
                "Envelope reporting\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nR2 out 0 1k\n.save V(out) I(R1)\n{}\n.end\n",
                options.to_spice_options()
            );
            let data =
                run_envelope_analysis_with_source_path_and_abort(&deck, &config, None, &NoAbort)
                    .unwrap();
            assert_eq!(data.time.len(), expected.len());
            for (&actual, expected) in data.time.iter().zip(expected) {
                assert!((actual - expected * 1e-6).abs() < 1e-18);
            }
            assert_eq!(data.waveforms.len(), 2);
            for waveform in data.waveforms {
                let amplitude = if waveform.unit == "A" { 0.5e-3 } else { 0.5 };
                for value in waveform.values {
                    assert!(value.re.abs() < amplitude * 2e-4);
                    assert!((value.im + amplitude).abs() < amplitude * 2e-4);
                }
            }
        }
    }

    #[test]
    fn envelope_reporting_rejects_uncenterable_times_and_bounds_schedules() {
        let time = [0.0, 0.25, 0.5, 0.75, 1.0];
        let options = rspice_core::netlist::SimulationOptions {
            output_time_points: vec![0.25],
            ..Default::default()
        };
        let error = configured_times(&options, &time, &[1.0], 20, &NoAbort).unwrap_err();
        assert!(error.to_string().contains("complete carrier window"));
        let options = rspice_core::netlist::SimulationOptions {
            output_interval_schedule: Some(rspice_core::netlist::XyceOutputIntervalSchedule {
                initial_interval: 0.01,
                intervals: Vec::new(),
            }),
            ..Default::default()
        };
        assert!(configured_times(&options, &time, &[4.0], 5, &NoAbort).is_err());
        assert!(matches!(
            configured_times(
                &options,
                &time,
                &[4.0],
                200,
                &rspice_core::abort_signal::CountingAbort::new(0)
            ),
            Err(ServiceRunError::Aborted)
        ));
    }
}
