//! What a transient written to a result sink leaves to its caller.

use super::state_capture::TransientStateCapture;
use super::{
    FinalCheckpointRetention, ResumeValidation, ScheduledTransientCheckpoint, TransientCheckpoint,
    TransientEnd, TransientResumePlan, TransientRunWindow, TransientStartupMode,
    validate_transient_window,
};
use super::{fft, lead_power};
use crate::analysis::transient::TransientResult;
use crate::result_sink::{
    EventChange, ResultSchema, ResultSink, RunMeta, RunStatus, SinkReceipt, SinkStop,
};
use crate::{AbortSignal, Engine, Netlist, SimulationError, Value};

/// A transient whose rows a result sink kept.
#[derive(Debug)]
pub struct SinkTransient {
    /// Everything the run returns but its time and waveform columns, which are empty.
    pub result: TransientResult,
    /// What the sink kept.
    pub receipt: SinkReceipt,
    /// The sink ran out of disk or memory and the run ended at the accepted point whose row it
    /// refused, before its stop time. Every row the sink took before is kept, and the sink was
    /// ended as stopped.
    pub stopped: Option<SinkStop>,
}

/// The caller's sink, remembering whether it began and the first refusal of a row: the stop
/// that ends the run.
pub(super) struct StopLatch<'s> {
    pub(super) sink: &'s mut dyn ResultSink,
    pub(super) begun: bool,
    pub(super) stop: Option<SinkStop>,
}

impl<'s> StopLatch<'s> {
    pub(super) fn new(sink: &'s mut dyn ResultSink) -> Self {
        Self {
            sink,
            begun: false,
            stop: None,
        }
    }
}

/// What a run into a result sink starts from, keeps of its end and captures on its way.
#[derive(Clone, Copy)]
pub(super) struct SinkRunPlan<'a> {
    pub(super) resume: Option<&'a TransientCheckpoint>,
    pub(super) resume_validation: ResumeValidation,
    pub(super) retention: FinalCheckpointRetention,
    pub(super) scheduled_checkpoint_times: &'a [Value],
    pub(super) state_capture: Option<&'a TransientStateCapture<'a>>,
}

impl<'a> SinkRunPlan<'a> {
    /// A run from its operating point (or the checkpoint `resume`) to its stop, keeping its final
    /// checkpoint as `retention` says.
    pub(super) fn to_stop(
        resume: Option<&'a TransientCheckpoint>,
        retention: FinalCheckpointRetention,
    ) -> Self {
        Self {
            resume,
            resume_validation: ResumeValidation::ExactNetlist,
            retention,
            scheduled_checkpoint_times: &[],
            state_capture: None,
        }
    }
}

/// The checkpoint a run asked to keep its end ended with.
fn retained(end: Option<TransientEnd>) -> Result<TransientEnd, SimulationError> {
    end.ok_or_else(|| {
        SimulationError::Circuit(
            "internal transient checkpoint retention contract was not fulfilled".to_string(),
        )
    })
}

impl ResultSink for StopLatch<'_> {
    fn begin(&mut self, schema: &ResultSchema, meta: &RunMeta) -> Result<(), SinkStop> {
        let begun = self.sink.begin(schema, meta);
        self.begun = begun.is_ok();
        begun
    }

    #[inline]
    fn row(&mut self, x: f64, values: &[f64]) -> Result<(), SinkStop> {
        let written = self.sink.row(x, values);
        if let Err(stop) = &written
            && self.stop.is_none()
        {
            self.stop = Some(stop.clone());
        }
        written
    }

    fn amend_open_row(&mut self, row: u64, column: u32, value: f64) -> Result<(), SinkStop> {
        self.sink.amend_open_row(row, column, value)
    }

    fn events(&mut self, changes: &[EventChange]) -> Result<(), SinkStop> {
        self.sink.events(changes)
    }

    fn end(&mut self, status: RunStatus) -> Result<SinkReceipt, SinkStop> {
        self.sink.end(status)
    }
}

impl Engine {
    /// [`Engine::run_tran_checkpointed_with_startup_mode_and_abort`] writing every accepted point
    /// to `sink`, as [`Engine::run_tran_into_sink_with_startup_mode_and_abort`] does: the run
    /// returns the checkpoint it ends with, which a later resume continues from its last row.
    pub fn run_tran_checkpointed_into_sink_with_startup_mode_and_abort(
        &self,
        netlist: &Netlist,
        (tstop, max_step): (Value, Value),
        startup_mode: TransientStartupMode,
        abort: &dyn AbortSignal,
        sink: &mut dyn ResultSink,
    ) -> Result<(SinkTransient, TransientEnd), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let engine = self.resolved_for_netlist(netlist);
        let window = TransientRunWindow {
            tstop,
            max_step,
            startup_mode,
            dc_seed: None,
            integral_trace: None,
        };
        let (run, end, _) = engine.tran_into_sink(
            netlist,
            window,
            SinkRunPlan::to_stop(None, FinalCheckpointRetention::Retained),
            abort,
            sink,
        )?;
        Ok((run, retained(end)?))
    }

    /// [`Engine::run_tran_resume_with_abort`] writing every accepted point to `sink`, from the
    /// checkpoint's own row on. A sink that already holds the run the checkpoint continues, such
    /// as a results store reopened after that row, takes the resumed rows after its own: the
    /// resumed run's first row, and any event change at the checkpoint's time, repeat what it
    /// holds.
    pub fn run_tran_resume_into_sink_with_abort(
        &self,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        (tstop, max_step): (Value, Value),
        abort: &dyn AbortSignal,
        sink: &mut dyn ResultSink,
    ) -> Result<(SinkTransient, TransientEnd), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        if !tstop.is_finite() || tstop <= checkpoint.time {
            return Err(SimulationError::Circuit(format!(
                "resume stop time {tstop:e} must exceed the checkpoint time {:e}",
                checkpoint.time
            )));
        }
        let engine = self.resolved_for_netlist(netlist);
        let window = TransientRunWindow {
            tstop,
            max_step,
            startup_mode: checkpoint.startup_mode(),
            dc_seed: None,
            integral_trace: None,
        };
        let (run, end, _) = engine.tran_into_sink(
            netlist,
            window,
            SinkRunPlan::to_stop(Some(checkpoint), FinalCheckpointRetention::Retained),
            abort,
            sink,
        )?;
        Ok((run, retained(end)?))
    }

    /// Run a transient into `sink` as `plan` says. The sink is ended with how the run ended.
    /// Returns the run, its end when it kept one, and the checkpoints scheduled on its way.
    pub(super) fn tran_into_sink(
        &self,
        netlist: &Netlist,
        window: TransientRunWindow<'_>,
        plan: SinkRunPlan<'_>,
        abort: &dyn AbortSignal,
        sink: &mut dyn ResultSink,
    ) -> Result<
        (
            SinkTransient,
            Option<TransientEnd>,
            Vec<ScheduledTransientCheckpoint>,
        ),
        SimulationError,
    > {
        use crate::result_sink::StopReason;
        let cell = std::cell::RefCell::new(StopLatch::new(sink));
        let shared: &std::cell::RefCell<dyn ResultSink + '_> = &cell;
        let outcome = self.run_tran_resolved_with_resume(
            netlist,
            netlist,
            window,
            abort,
            TransientResumePlan {
                resume: plan.resume,
                resume_validation: plan.resume_validation,
                final_checkpoint_retention: plan.retention,
                scheduled_checkpoint_times: plan.scheduled_checkpoint_times,
                checkpoint_observer: None,
                state_capture: plan.state_capture,
                result_sink: Some(shared),
            },
        );
        let StopLatch { sink, begun, stop } = cell.into_inner();
        match outcome {
            Ok((result, end, scheduled)) => {
                let status = match &stop {
                    None => RunStatus::Completed,
                    Some(SinkStop::MemoryFull { .. }) => RunStatus::Stopped(StopReason::MemoryFull),
                    Some(_) => RunStatus::Stopped(StopReason::DiskFull),
                };
                let receipt = sink.end(status).map_err(SimulationError::ResultSink)?;
                Ok((
                    SinkTransient {
                        result,
                        receipt,
                        stopped: stop,
                    },
                    end,
                    scheduled,
                ))
            }
            Err(error) => {
                let reason = match &error {
                    SimulationError::Aborted => StopReason::Cancelled,
                    SimulationError::TimeLimitExceeded => StopReason::Timeout,
                    SimulationError::ResultSink(SinkStop::DiskFull { .. }) => StopReason::DiskFull,
                    SimulationError::ResultSink(SinkStop::MemoryFull { .. }) => {
                        StopReason::MemoryFull
                    }
                    other => StopReason::Failed(other.to_string()),
                };
                // The run's own error is the one to report; a sink that also fails to close has
                // already stopped writing. A sink the run never began has nothing to end.
                if begun {
                    let _ = sink.end(RunStatus::Stopped(reason));
                }
                Err(error)
            }
        }
    }

    /// Complete the outputs of a transient whose rows a result sink kept, once the caller has
    /// read them back into `result`: the `.FFT` spectra and the lead-power outputs, which read
    /// the whole waveforms after the run. `tstop` is the stop time the run was given.
    ///
    /// The run itself wrote its instance parameters as constant columns of its rows, which
    /// [`TransientResult::adopt_instance_parameter`] takes back.
    pub fn finish_transient_from_rows(
        &self,
        netlist: &Netlist,
        result: &mut TransientResult,
        tstop: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        result.fft_results = fft::evaluate(&engine, netlist, result, tstop, abort)?;
        let requests = lead_power::requested_lead_power(netlist)?;
        lead_power::record_lead_power(&requests, result);
        Ok(())
    }
}
