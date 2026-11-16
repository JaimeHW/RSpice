//! A checkpointed transient interrupted inside its own window.
//!
//! A checkpoint taken mid-run, as Spectre's is, keeps the run's own window: its stop time, its
//! maximum step and its breakpoint schedule, all of which shape the steps it takes from its first.
//! The run lands on the interruption as on a breakpoint and stops there once it has proposed the
//! step after it, so the checkpoint holds the integrator as it stood: that step, its order and its
//! histories. A resume to the run's stop then takes every step the run takes with a breakpoint at
//! the interruption, from the first to the last.

use super::state_capture::TransientStateCapture;
use super::{
    FinalCheckpointRetention, ResumeValidation, TransientEnd, TransientResumePlan,
    TransientRunWindow, TransientStartupMode, validate_transient_window,
};
use crate::analysis::transient::TransientResult;
use crate::{AbortSignal, Engine, Netlist, SimulationError, Value};

impl Engine {
    /// Run the transient of `netlist` over its window to `tstop` and interrupt it at `stop`,
    /// returning the rows to `stop` and the checkpoint a resume continues from there.
    ///
    /// A run its model finishes before `stop` ends finished, with nothing to resume. A circuit
    /// whose integrator state does not continue across a checkpoint keeps the one that restarts
    /// it at `stop`, as [`Engine::run_tran_checkpointed_with_startup_mode_and_abort`] does at
    /// its stop.
    pub fn run_tran_checkpointed_at_with_startup_mode_and_abort(
        &self,
        netlist: &Netlist,
        (tstop, max_step): (Value, Value),
        stop: Value,
        startup_mode: TransientStartupMode,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientEnd), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        if !(stop.is_finite() && stop > 0.0 && stop < tstop) {
            return Err(SimulationError::Circuit(format!(
                "a transient checkpoint at {stop:e} s must lie inside the run's (0, {tstop:e}) s"
            )));
        }
        // The interruption is a hard breakpoint of the run, and the capture there ends it once
        // the checkpoint scheduled at the same time has been taken after the next step's
        // proposal.
        let times = [stop];
        let capture = TransientStateCapture::new(&times, tstop)?;
        let engine = self.resolved_for_netlist(netlist);
        let (result, end, mut continued) = engine.run_tran_resolved_with_resume(
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
                resume: None,
                resume_validation: ResumeValidation::ExactNetlist,
                final_checkpoint_retention: FinalCheckpointRetention::Retained,
                scheduled_checkpoint_times: &times,
                checkpoint_observer: None,
                state_capture: Some(&capture),
                result_sink: None,
            },
        )?;
        let end = match (end, continued.pop()) {
            (Some(TransientEnd::Checkpoint(_)), Some(scheduled)) => {
                TransientEnd::Checkpoint(scheduled.checkpoint)
            }
            (Some(end), _) => end,
            (None, _) => {
                return Err(SimulationError::Circuit(
                    "internal transient checkpoint retention contract was not fulfilled"
                        .to_string(),
                ));
            }
        };
        Ok((result, end))
    }
}
