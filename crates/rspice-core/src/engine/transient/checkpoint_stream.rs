//! Immediate checkpoint delivery from the same accepted boundaries as retention.
use super::*;

/// Initial state for a streamed transient run. Resume uses the checkpoint's
/// original startup policy and requires the same source/configuration identity.
#[derive(Clone, Copy, Debug)]
pub enum TransientCheckpointStart<'a> {
    Fresh(TransientStartupMode),
    Resume(&'a TransientCheckpoint),
    /// Permit only the horizon/file-option changes of an authored restart deck.
    Restart(&'a TransientCheckpoint),
}

/// A synchronous sink for an accepted snapshot. Returning an error stops the
/// run without retracting previously delivered snapshots. The sink may encode
/// or copy a snapshot; the engine releases its copy when the call returns.
pub type TransientCheckpointObserver<'a> =
    dyn Fn(&ScheduledTransientCheckpoint) -> Result<(), SimulationError> + Sync + 'a;

/// A checkpoint schedule delivered during execution instead of accumulated in
/// the result. Times must increase strictly and lie within the run interval.
/// Include the stop time to request a final scheduled snapshot. Due times may
/// coalesce under the restart schedule tolerance. A callback does not add solver
/// breakpoints: nominal and accepted times remain distinct.
pub struct TransientCheckpointStream<'a> {
    pub start: TransientCheckpointStart<'a>,
    pub times: &'a [Value],
    pub observer: &'a TransientCheckpointObserver<'a>,
}

impl Engine {
    /// Run or resume a transient while publishing accepted checkpoints.
    ///
    /// No scheduled snapshots are retained by the engine after delivery. The
    /// usual result budget includes the snapshot while it is being delivered;
    /// storage retained by the callback is the caller's responsibility. Both
    /// cancellation and callback errors stop subsequent delivery. Previously
    /// delivered snapshots remain independently usable for continuation.
    pub fn run_tran_checkpoint_stream_with_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        stream: TransientCheckpointStream<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let resume_validation = match stream.start {
            TransientCheckpointStart::Restart(_) => ResumeValidation::AuthoredRestart,
            _ => ResumeValidation::ExactNetlist,
        };
        let (resume, startup_mode, start) = match stream.start {
            TransientCheckpointStart::Fresh(mode) => (None, mode, 0.0),
            TransientCheckpointStart::Resume(checkpoint)
            | TransientCheckpointStart::Restart(checkpoint) => {
                if !checkpoint.time.is_finite() || checkpoint.time < 0.0 || tstop <= checkpoint.time
                {
                    return Err(SimulationError::Circuit(
                        "streamed resume requires a finite accepted time before the stop time"
                            .into(),
                    ));
                }
                let mode = checkpoint.startup_mode().ok_or_else(|| {
                    SimulationError::Circuit(
                        "legacy transient checkpoint does not record its startup mode".into(),
                    )
                })?;
                (Some(checkpoint), mode, checkpoint.time)
            }
        };
        self.validate_transient_checkpoint_schedule(netlist, start, tstop, stream.times)?;
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop - start, max_step)?;
        let observe = |point: &ScheduledTransientCheckpoint| {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            (stream.observer)(point)?;
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            Ok(())
        };
        engine
            .run_tran_resolved_with_resume(
                netlist,
                netlist,
                TransientRunWindow {
                    tstop,
                    max_step,
                    startup_mode,
                    dc_seed: None,
                    integral_trace: None,
                },
                abort,
                TransientResumePlan {
                    resume,
                    resume_validation,
                    final_checkpoint_retention: FinalCheckpointRetention::Discarded,
                    scheduled_checkpoint_times: stream.times,
                    checkpoint_observer: Some(&observe),
                },
            )
            .map(|(result, _, _)| result)
    }

    pub(super) fn validate_transient_checkpoint_schedule(
        &self,
        netlist: &Netlist,
        start: Value,
        stop: Value,
        times: &[Value],
    ) -> Result<(), SimulationError> {
        let mut previous = None;
        for (index, &time) in times.iter().enumerate() {
            if !time.is_finite() || time < start || time > stop {
                return Err(SimulationError::Circuit(format!(
                    "scheduled checkpoint time {index} must be finite and within [{start:.17e}, {stop:.17e}], found {time:.17e}"
                )));
            }
            if previous.is_some_and(|previous| time <= previous) {
                return Err(SimulationError::Circuit(format!(
                    "scheduled checkpoint times must be strictly increasing; found {time:.17e} after {:.17e}",
                    previous.expect("checked above")
                )));
            }
            previous = Some(time);
        }
        let retained_schedules = netlist
            .options
            .output_time_points
            .len()
            .saturating_add(
                netlist
                    .options
                    .output_interval_schedule
                    .as_ref()
                    .map_or(0, |schedule| schedule.intervals.len()),
            )
            .saturating_add(netlist.options.timeint_breakpoints.len())
            .saturating_add(
                netlist
                    .options
                    .restart
                    .as_ref()
                    .map_or(0, |restart| restart.intervals.len()),
            )
            .saturating_add(times.len());
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            retained_schedules,
            self.config.resource_limits.max_analysis_points,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimulationConfig;
    use std::sync::Mutex;

    #[test]
    fn streamed_checkpoints_release_retention_and_propagate_sink_failure() {
        let netlist =
            Netlist::parse("streamed RC\nV1 in 0 SIN(0 1 1k)\nR1 in out 1k\nC1 out 0 1n\n.end\n")
                .unwrap();
        let engine = Engine::new(SimulationConfig::default());
        let times: Vec<_> = (1..=1024).map(|i| f64::from(i) * 1e-6).collect();
        let (baseline, retained) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                1025e-6,
                1e-6,
                TransientStartupMode::OperatingPoint,
                &times,
            )
            .unwrap();
        assert_eq!(retained.len(), times.len());
        let result_values = Engine::transient_result_value_count(&baseline);
        let peak_snapshot = retained
            .iter()
            .map(|p| p.checkpoint.retained_value_count() + 1)
            .max()
            .unwrap();
        // Leave ample room for the shared solver workspace and waveform.
        // The complete checkpoint collection must still exceed this ceiling,
        // so successful streaming cannot be explained by retaining it all.
        let all_snapshots = retained
            .iter()
            .map(|p| p.checkpoint.retained_value_count() + 1)
            .sum::<usize>();
        let mut config = SimulationConfig::default();
        config.resource_limits.max_result_values = 64 * 1024;
        assert!(result_values + peak_snapshot < config.resource_limits.max_result_values);
        assert!(all_snapshots > config.resource_limits.max_result_values);
        let bounded = Engine::new(config);
        let count = Mutex::new(0usize);
        let observe = |point: &ScheduledTransientCheckpoint| {
            let mut count = count.lock().unwrap();
            assert_eq!(point, &retained[*count]);
            *count += 1;
            Ok(())
        };
        let result = bounded
            .run_tran_checkpoint_stream_with_abort(
                &netlist,
                1025e-6,
                1e-6,
                TransientCheckpointStream {
                    start: TransientCheckpointStart::Fresh(TransientStartupMode::OperatingPoint),
                    times: &times,
                    observer: &observe,
                },
                &NoAbort,
            )
            .unwrap();
        assert_eq!(*count.lock().unwrap(), times.len());
        assert_eq!(result.time, baseline.time);
        assert_eq!(result.voltages, baseline.voltages);
        assert!(
            bounded
                .run_tran_checkpoint_schedule_with_startup_mode(
                    &netlist,
                    1025e-6,
                    1e-6,
                    TransientStartupMode::OperatingPoint,
                    &times,
                )
                .is_err()
        );

        let failed_calls = Mutex::new(0);
        let fail = |_: &ScheduledTransientCheckpoint| {
            *failed_calls.lock().unwrap() += 1;
            Err(SimulationError::Circuit(
                "snapshot destination unavailable".into(),
            ))
        };
        let error = engine
            .run_tran_checkpoint_stream_with_abort(
                &netlist,
                1025e-6,
                1e-6,
                TransientCheckpointStream {
                    start: TransientCheckpointStart::Fresh(TransientStartupMode::OperatingPoint),
                    times: &times,
                    observer: &fail,
                },
                &NoAbort,
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("snapshot destination unavailable")
        );
        assert_eq!(*failed_calls.lock().unwrap(), 1);
    }

    #[test]
    fn streamed_checkpoint_origin_and_invalid_schedules_obey_delivery_contract() {
        let engine = Engine::new(SimulationConfig::default());
        let netlist = Netlist::parse("empty\n.end\n").unwrap();
        let count = Mutex::new(0);
        let observe = |point: &ScheduledTransientCheckpoint| {
            assert_eq!(point.nominal_time, 0.0);
            assert_eq!(point.checkpoint.time, 0.0);
            *count.lock().unwrap() += 1;
            Ok(())
        };
        engine
            .run_tran_checkpoint_stream_with_abort(
                &netlist,
                1e-6,
                1e-9,
                TransientCheckpointStream {
                    start: TransientCheckpointStart::Fresh(TransientStartupMode::Uic),
                    times: &[0.0],
                    observer: &observe,
                },
                &NoAbort,
            )
            .unwrap();
        for times in [vec![-1.0], vec![f64::NAN], vec![2e-6], vec![0.0, 0.0]] {
            assert!(
                engine
                    .run_tran_checkpoint_stream_with_abort(
                        &netlist,
                        1e-6,
                        1e-9,
                        TransientCheckpointStream {
                            start: TransientCheckpointStart::Fresh(TransientStartupMode::Uic),
                            times: &times,
                            observer: &observe,
                        },
                        &NoAbort,
                    )
                    .is_err()
            );
        }
        assert_eq!(*count.lock().unwrap(), 1);
    }
}
