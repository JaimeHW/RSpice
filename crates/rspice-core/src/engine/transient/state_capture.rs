//! Live capture of accepted transient states.
//!
//! A small-signal analysis at a transient state (Spectre's `actimes`)
//! linearizes around the circuit as it stands at an accepted time `T`: every
//! accepted analog state — charges, operator states, `idt` values — the
//! source values at `T`, and the discrete state of every mixed host. A
//! checkpoint image can carry that state only for devices whose state it
//! knows how to serialize. A live capture carries it for every device,
//! because it is a clone of the circuit itself, taken at an accepted point,
//! where a mixed circuit's clone is valid.
//!
//! The request is a list of times. Each becomes a hard breakpoint, so the
//! run lands on it; the accepted point that lands there is captured; and the
//! run stops once the last requested time is captured.

use std::cell::RefCell;

use super::{
    FinalCheckpointRetention, ResumeValidation, TransientResumePlan, TransientRunWindow,
    TransientStartupMode, validate_transient_window,
};
use crate::abort_signal::AbortSignal;
use crate::engine::{Engine, SimulationError, TransientResult};
use crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE;
use crate::{CircuitData, Netlist, Value};

/// One accepted transient state, captured live.
#[derive(Clone, Debug)]
pub(in crate::engine) struct CapturedTransientState {
    /// The accepted time the state belongs to.
    pub(in crate::engine) time: Value,
    /// The whole circuit as accepted at `time`.
    pub(in crate::engine) circuit: CircuitData,
    /// The accepted solution at `time`.
    pub(in crate::engine) solution: Vec<Value>,
}

/// A run's capture request and the states it has captured so far.
#[derive(Debug)]
pub(in crate::engine) struct TransientStateCapture<'a> {
    /// Requested times, finite, non-negative and strictly increasing.
    times: &'a [Value],
    captured: RefCell<Vec<CapturedTransientState>>,
}

impl<'a> TransientStateCapture<'a> {
    /// Validate a capture request against the run's stop time.
    pub(in crate::engine) fn new(
        times: &'a [Value],
        tstop: Value,
    ) -> Result<Self, SimulationError> {
        if times.is_empty() {
            return Err(SimulationError::Circuit(
                "a transient state capture needs at least one time".to_string(),
            ));
        }
        let mut previous = None;
        for &time in times {
            if !time.is_finite() || time < 0.0 || time > tstop {
                return Err(SimulationError::Circuit(format!(
                    "transient state capture time {time:e} s is outside the run's [0, {tstop:e}] s"
                )));
            }
            if previous.is_some_and(|previous| time <= previous) {
                return Err(SimulationError::Circuit(format!(
                    "transient state capture times must be strictly increasing; {time:e} s \
                     follows {:e} s",
                    previous.unwrap_or_default()
                )));
            }
            previous = Some(time);
        }
        Ok(Self {
            times,
            captured: RefCell::new(Vec::with_capacity(times.len())),
        })
    }

    /// The requested times, which the run adds as hard breakpoints.
    pub(in crate::engine) fn times(&self) -> &[Value] {
        self.times
    }

    /// Offer an accepted point. Captures it when it lands on the next
    /// requested time, and answers whether every requested time has now been
    /// captured, which is the run's signal to stop.
    ///
    /// A run that passes a requested time without landing on it is refused:
    /// the requested times are breakpoints, so that is a defect, not a
    /// state the caller could use.
    pub(in crate::engine) fn offer(
        &self,
        time: Value,
        circuit: &CircuitData,
        solution: &[Value],
    ) -> Result<bool, SimulationError> {
        let mut captured = self.captured.borrow_mut();
        let Some(&requested) = self.times.get(captured.len()) else {
            return Ok(true);
        };
        let tolerance = XYCE_BREAKPOINT_TOLERANCE.max(requested.abs() * Value::EPSILON * 4.0);
        if (time - requested).abs() <= tolerance {
            captured.push(CapturedTransientState {
                time,
                circuit: circuit.clone(),
                solution: solution.to_vec(),
            });
        } else if time > requested {
            return Err(SimulationError::Circuit(format!(
                "the transient accepted t={time:e} s without landing on the requested capture \
                 time {requested:e} s"
            )));
        }
        Ok(captured.len() == self.times.len())
    }

    /// The states captured, in request order.
    pub(in crate::engine) fn into_captured(self) -> Vec<CapturedTransientState> {
        self.captured.into_inner()
    }
}

impl Engine {
    /// Run a transient card and capture its accepted state at each of
    /// `times`, stopping once the last is captured.
    ///
    /// The card's whole window resolves exactly as an uncaptured run's —
    /// `tstop` still sets the default lowest noise frequency and the sample
    /// count of a transient-noise run — so the trajectory up to the last
    /// capture is the uncaptured run's, landed on the requested times.
    pub(in crate::engine) fn run_tran_capturing_states(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        times: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Vec<CapturedTransientState>), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let capture = TransientStateCapture::new(times, tstop)?;
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_transient_request_floor(tstop, max_step)?;
        let (result, _, _) = engine.run_tran_resolved_with_resume(
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
                final_checkpoint_retention: FinalCheckpointRetention::Discarded,
                scheduled_checkpoint_times: &[],
                checkpoint_observer: None,
                state_capture: Some(&capture),
            },
        )?;
        let captured = capture.into_captured();
        if captured.len() != times.len() {
            return Err(SimulationError::Circuit(format!(
                "the transient captured {} of {} requested states",
                captured.len(),
                times.len()
            )));
        }
        Ok((result, captured))
    }

    /// Resume the transient `checkpoint` holds on `netlist`, as an authored
    /// restart of it, and capture the state at the checkpoint's own time.
    ///
    /// That time is the resumed run's first accepted point, so the run
    /// validates the checkpoint against the deck, restores it, captures the
    /// restored state and stops without taking a step. `tstop` must exceed
    /// the checkpoint's time, as any resumed run's does.
    pub(in crate::engine) fn capture_checkpoint_state(
        &self,
        netlist: &Netlist,
        checkpoint: &super::TransientCheckpoint,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<CapturedTransientState, SimulationError> {
        validate_transient_window(tstop, max_step)?;
        if !tstop.is_finite() || tstop <= checkpoint.time {
            return Err(SimulationError::Circuit(format!(
                "resume stop time {tstop:e} must exceed the checkpoint time {:e}",
                checkpoint.time
            )));
        }
        let startup_mode = checkpoint.startup_mode().ok_or_else(|| {
            SimulationError::Circuit(
                "legacy transient checkpoint does not record its startup mode; refusing to \
                 guess whether the captured trajectory used UIC"
                    .to_string(),
            )
        })?;
        let times = [checkpoint.time];
        let capture = TransientStateCapture::new(&times, tstop)?;
        self.reset_convergence_quality();
        let mut engine = self.resolved_for_netlist(netlist);
        engine.small_signal_state = None;
        engine.ensure_transient_request_floor(tstop - checkpoint.time, max_step)?;
        engine.run_tran_resolved_with_resume(
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
                resume: Some(checkpoint),
                resume_validation: ResumeValidation::AuthoredRestart,
                final_checkpoint_retention: FinalCheckpointRetention::Discarded,
                scheduled_checkpoint_times: &[],
                checkpoint_observer: None,
                state_capture: Some(&capture),
            },
        )?;
        let captured = capture.into_captured().pop().ok_or_else(|| {
            SimulationError::Circuit(format!(
                "the resumed transient captured no state at the checkpoint time {:e} s",
                checkpoint.time
            ))
        })?;
        // The analysis reads each mixed instance's discrete inputs from its
        // analog slots as the image restored them; they must be the values
        // the digital state restored beside them gives, wherever that state
        // determines them.
        #[cfg(feature = "veriloga")]
        if let Some(refusal) = captured.circuit.restored_discrete_input_refusal() {
            return Err(refusal.into());
        }
        Ok(captured)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    const RC_DECK: &str = "captured RC charge\n\
                           V1 in 0 PULSE(0 1 0 1n 1n 10u 20u)\n\
                           R1 in out 1k\n\
                           C1 out 0 1n\n\
                           .TRAN 10n 5u\n\
                           .END\n";

    /// Linear interpolation of a waveform at `time`.
    fn interpolate(times: &[Value], values: &[Value], time: Value) -> Value {
        let upper = times.partition_point(|candidate| *candidate < time);
        if upper == 0 {
            return values[0];
        }
        if upper >= times.len() {
            return values[values.len() - 1];
        }
        let (t0, t1) = (times[upper - 1], times[upper]);
        let (v0, v1) = (values[upper - 1], values[upper]);
        if t1 == t0 {
            return v1;
        }
        v0 + (v1 - v0) * (time - t0) / (t1 - t0)
    }

    /// Capturing lands on every requested time exactly, stops at the last,
    /// and captures the state the uncaptured run passes through there.
    #[test]
    fn captured_states_land_on_the_requested_times_and_match_the_run() {
        let netlist = Netlist::parse(RC_DECK).expect("the deck parses");
        let engine = Engine::default();
        let (tstop, max_step) = (5.0e-6, 1.0e-8);
        let times = [1.0e-6, 3.0e-6];
        let (captured_run, captured) = engine
            .run_tran_capturing_states(
                &netlist,
                tstop,
                max_step,
                TransientStartupMode::OperatingPoint,
                &times,
                &NoAbort,
            )
            .expect("the captured run converges");
        let reference = engine
            .run_tran(&netlist, tstop, max_step)
            .expect("the uncaptured run converges");

        assert_eq!(captured.len(), times.len());
        let last = *captured_run.time.last().expect("the run accepted points");
        assert_eq!(last, times[1], "the run stops at the last capture");
        let reference_out = reference
            .try_voltage_waveform_named("out")
            .expect("out is retained");
        for (state, requested) in captured.iter().zip(times) {
            assert_eq!(
                state.time, requested,
                "the capture lands on the requested time"
            );
            assert!(
                captured_run.time.contains(&requested),
                "the requested time is an accepted point of the run"
            );
            let node = state
                .circuit
                .get_node_by_name("out")
                .expect("the captured circuit names out");
            let captured_value = state.solution[node - 1];
            let expected = interpolate(&reference.time, reference_out, requested);
            // RELTOL is the deck's default 1e-3; the uncaptured run's own
            // accuracy at an interpolated time is what the comparison rests
            // on, so the bound is RELTOL of the value plus VNTOL.
            let bound = 1.0e-3 * expected.abs() + 1.0e-6;
            assert!(
                (captured_value - expected).abs() <= bound,
                "at t={requested:e} s the captured {captured_value} differs from the run's \
                 {expected} by more than {bound:e}"
            );
        }
    }

    /// A request must be ordered and inside the run.
    #[test]
    fn a_malformed_capture_request_is_refused() {
        assert!(TransientStateCapture::new(&[], 1.0).is_err());
        assert!(TransientStateCapture::new(&[0.5, 0.5], 1.0).is_err());
        assert!(TransientStateCapture::new(&[0.5, 0.25], 1.0).is_err());
        assert!(TransientStateCapture::new(&[2.0], 1.0).is_err());
        assert!(TransientStateCapture::new(&[-1.0], 1.0).is_err());
        assert!(TransientStateCapture::new(&[0.0, 1.0], 1.0).is_ok());
    }
}
