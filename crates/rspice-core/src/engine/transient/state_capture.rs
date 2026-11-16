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
//! run lands on it, and the accepted point that lands there is captured. A
//! card's own run stops once the last requested time is captured; a `.TRAN`
//! task's run lands on the times its state cards name and continues to its
//! stop time, so one transient serves the task and the cards alike.

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
    /// The sparse solver's accepted state at `time` (its pivot order and
    /// factors), which a segment from this image restores so that it solves
    /// as the run it continues does.
    pub(in crate::engine) solver: Option<rspice_matrix::StaticMatrixSolverCheckpoint>,
}

/// The integration step that reached an accepted point: its length and the
/// companion coefficients the circuit's reactive elements integrated it
/// with. The run's starting point has none.
#[derive(Clone, Copy, Debug)]
pub(in crate::engine) struct AcceptedStep {
    pub(in crate::engine) step: Value,
    pub(in crate::engine) coefficients: crate::numerics::integration::CompanionCoefficients,
    /// How far the step's end moves when its start does. A locked step's
    /// end is a mesh point and stays; the order-one step from a landing short
    /// of a switch's crossing ends a fixed fraction of the way to its target,
    /// so its end follows the crossing by the rest of that fraction.
    pub(in crate::engine) follows: Value,
}

/// Offered every accepted point of a run, with the circuit and solution as
/// accepted there and the step that reached it; it reads and changes nothing
/// of the run.
pub(in crate::engine) type AcceptedPointObserver<'a> =
    dyn Fn(Value, &CircuitData, &[Value], Option<AcceptedStep>) -> Result<(), SimulationError> + 'a;

/// A run's capture request and the states it has captured so far.
pub(in crate::engine) struct TransientStateCapture<'a> {
    /// Requested times, finite, non-negative and strictly increasing.
    times: &'a [Value],
    /// Whether the run stops once the last time is captured.
    stops_at_last: bool,
    captured: RefCell<Vec<CapturedTransientState>>,
    /// The live image a segment starts from, in place of an operating point
    /// or a checkpoint: the run continues the circuit the image holds.
    start: Option<&'a CapturedTransientState>,
    /// The circuit a caller already built for the run, which the run
    /// integrates in place of building its own; taken once.
    built: RefCell<Option<CircuitData>>,
    /// Offered every accepted point before any capture.
    observer: Option<&'a AcceptedPointObserver<'a>>,
}

impl std::fmt::Debug for TransientStateCapture<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TransientStateCapture")
            .field("times", &self.times)
            .field("stops_at_last", &self.stops_at_last)
            .field("captured", &self.captured.borrow().len())
            .field("start", &self.start.map(|image| image.time))
            .field("built", &self.built.borrow().is_some())
            .field("observed", &self.observer.is_some())
            .finish()
    }
}

impl<'a> TransientStateCapture<'a> {
    /// Validate a capture request against the run's stop time. The run stops
    /// once the last requested time is captured.
    pub(in crate::engine) fn new(
        times: &'a [Value],
        tstop: Value,
    ) -> Result<Self, SimulationError> {
        Self::with_end(times, tstop, true)
    }

    /// The same request, for a segment that continues the live `image`.
    pub(in crate::engine) fn from_image(
        times: &'a [Value],
        tstop: Value,
        image: &'a CapturedTransientState,
    ) -> Result<Self, SimulationError> {
        let mut capture = Self::new(times, tstop)?;
        capture.start = Some(image);
        Ok(capture)
    }

    /// The same request, for a run that continues to its stop time.
    fn through_stop(times: &'a [Value], tstop: Value) -> Result<Self, SimulationError> {
        Self::with_end(times, tstop, false)
    }

    fn with_end(
        times: &'a [Value],
        tstop: Value,
        stops_at_last: bool,
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
            stops_at_last,
            captured: RefCell::new(Vec::with_capacity(times.len())),
            start: None,
            built: RefCell::new(None),
            observer: None,
        })
    }

    /// The live image the run starts from, when it is a segment.
    pub(in crate::engine) fn start(&self) -> Option<&'a CapturedTransientState> {
        self.start
    }

    /// The circuit the caller built for the run, the first time it is asked.
    pub(in crate::engine) fn take_built(&self) -> Option<CircuitData> {
        self.built.take()
    }

    /// The requested times, which the run adds as hard breakpoints.
    pub(in crate::engine) fn times(&self) -> &[Value] {
        self.times
    }

    /// Offer an accepted point. Captures it when it lands on the next
    /// requested time, and answers whether the run should stop: every
    /// requested time has now been captured, and the run stops at the last.
    ///
    /// A run that passes a requested time without landing on it is refused:
    /// the requested times are breakpoints, so that is a defect, not a
    /// state the caller could use.
    pub(in crate::engine) fn offer(
        &self,
        time: Value,
        circuit: &CircuitData,
        solution: &[Value],
        matrix: &crate::solver::StaticMatrix,
        step: Option<AcceptedStep>,
    ) -> Result<bool, SimulationError> {
        if let Some(observer) = self.observer {
            observer(time, circuit, solution, step)?;
        }
        let mut captured = self.captured.borrow_mut();
        let Some(&requested) = self.times.get(captured.len()) else {
            return Ok(self.stops_at_last);
        };
        let tolerance = XYCE_BREAKPOINT_TOLERANCE.max(requested.abs() * Value::EPSILON * 4.0);
        if (time - requested).abs() <= tolerance {
            captured.push(CapturedTransientState {
                time,
                circuit: circuit.clone(),
                solution: solution.to_vec(),
                solver: matrix.capture_solver_checkpoint().ok(),
            });
        } else if time > requested {
            return Err(SimulationError::Circuit(format!(
                "the transient accepted t={time:e} s without landing on the requested capture \
                 time {requested:e} s"
            )));
        }
        Ok(self.stops_at_last && captured.len() == self.times.len())
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
        self.run_tran_with_state_capture(netlist, tstop, max_step, startup_mode, capture, abort)
    }

    /// Run a transient card to `tstop`, landing on each of `times` and
    /// capturing the state accepted there, for the small-signal cards that
    /// linearize around them ([`Self::run_small_signal_at_transient_state`]).
    ///
    /// Spectre's `actimes` runs on the transient itself: each requested time
    /// is a breakpoint of the one run, which continues past it. This is
    /// [`Self::run_tran_with_startup_mode_and_abort`]'s run with those
    /// breakpoints added, and up to each time it is the run a card's own state
    /// transient takes.
    ///
    /// Called by the simulation service for a `.TRAN` task whose state cards
    /// are bound to it, and by `rspice run` for a deck's first `.TRAN` card.
    pub fn run_tran_capturing_states_to_stop(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        times: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Vec<crate::engine::TransientState>), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let capture = TransientStateCapture::through_stop(times, tstop)?;
        let (result, captured) = self.run_tran_with_state_capture(
            netlist,
            tstop,
            max_step,
            startup_mode,
            capture,
            abort,
        )?;
        Ok((
            result,
            captured
                .into_iter()
                .map(crate::engine::TransientState)
                .collect(),
        ))
    }

    /// [`Self::run_tran_capturing_states_to_stop`] writing every accepted point to `sink`, as
    /// [`Self::run_tran_into_sink_with_startup_mode_and_abort`] does.
    pub fn run_tran_capturing_states_to_stop_into_sink(
        &self,
        netlist: &Netlist,
        (tstop, max_step): (Value, Value),
        startup_mode: TransientStartupMode,
        times: &[Value],
        abort: &dyn AbortSignal,
        sink: &mut dyn crate::result_sink::ResultSink,
    ) -> Result<
        (
            super::sink_route::SinkTransient,
            Vec<crate::engine::TransientState>,
        ),
        SimulationError,
    > {
        validate_transient_window(tstop, max_step)?;
        let capture = TransientStateCapture::through_stop(times, tstop)?;
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);
        let (run, _, _) = engine.tran_into_sink(
            netlist,
            TransientRunWindow {
                tstop,
                max_step,
                startup_mode,
                dc_seed: None,
                integral_trace: None,
            },
            super::sink_route::SinkRunPlan {
                state_capture: Some(&capture),
                ..super::sink_route::SinkRunPlan::to_stop(None, FinalCheckpointRetention::Discarded)
            },
            abort,
            sink,
        )?;
        let requested = capture.times().len();
        let captured = capture.into_captured();
        if captured.len() != requested {
            return Err(SimulationError::Circuit(format!(
                "the transient captured {} of {requested} requested states",
                captured.len(),
            )));
        }
        Ok((
            run,
            captured
                .into_iter()
                .map(crate::engine::TransientState)
                .collect(),
        ))
    }

    fn run_tran_with_state_capture(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        startup_mode: TransientStartupMode,
        capture: TransientStateCapture<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Vec<CapturedTransientState>), SimulationError> {
        let requested = capture.times().len();
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);
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
                result_sink: None,
            },
        )?;
        let captured = capture.into_captured();
        if captured.len() != requested {
            return Err(SimulationError::Circuit(format!(
                "the transient captured {} of {requested} requested states",
                captured.len(),
            )));
        }
        Ok((result, captured))
    }

    /// Run a transient from its operating point (or `dc_seed`) to `tstop`,
    /// capturing the accepted state at each of `times` and offering every
    /// accepted point to `observer`; the run continues past the last capture
    /// to `tstop`. `built`, when given, is the circuit the transient builds
    /// of `netlist` for `tstop`, and the run integrates it instead of
    /// building it again.
    pub(in crate::engine) fn run_tran_observed(
        &self,
        (netlist, built): (&Netlist, Option<CircuitData>),
        (tstop, max_step): (Value, Value),
        dc_seed: Option<&crate::engine::PeriodicDcOperatingPointSeed>,
        times: &[Value],
        observer: &AcceptedPointObserver<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Vec<CapturedTransientState>), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let mut capture = TransientStateCapture::through_stop(times, tstop)?;
        capture.observer = Some(observer);
        capture.built = RefCell::new(built);
        let requested = times.len();
        let engine = self.resolved_for_netlist(netlist);
        let (result, _, _) = engine.run_tran_resolved_with_resume(
            netlist,
            netlist,
            TransientRunWindow {
                tstop,
                max_step,
                startup_mode: TransientStartupMode::from_uic(false),
                dc_seed,
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
                result_sink: None,
            },
        )?;
        let captured = capture.into_captured();
        if captured.len() != requested {
            return Err(SimulationError::Circuit(format!(
                "the transient captured {} of {requested} requested states",
                captured.len(),
            )));
        }
        Ok((result, captured))
    }

    /// Continue the circuit `image` holds over `mesh`, a locked time grid
    /// after the image's time, and capture the state accepted at the mesh's
    /// last time.
    ///
    /// The segment is the run the image was captured from, restarted at the
    /// image's time the way a breakpoint restarts the integrator: the step
    /// that leaves it is order one and the predictor starts from the image's
    /// solution, so the segment depends on the image's accepted state and not
    /// on the history before it. Every device keeps the state the image
    /// holds, and the reactive histories are taken from the image's
    /// solution, as a run takes them at its starting point. A caller writes
    /// shooting coordinates into the image's solution and device states
    /// before handing it over. The sources keep the context of the run the
    /// image came from.
    ///
    /// `steps`, when given, are the intervals that reach each mesh time
    /// from the one before it (from the image's time, for the first): the
    /// segment takes them as they are rather than the differences of the
    /// mesh's rounded times. Over the accepted times and step sizes of a run
    /// that captured `image` (a capture time is a hard breakpoint of that
    /// run), the segment repeats the run bit for bit: a landing on a source
    /// breakpoint is solved as the run solves it. An ideal switch whose
    /// control crosses its threshold inside a step cuts it at the crossing,
    /// so the change is dated where it happens rather than at the next mesh
    /// time; there the segment departs from a run, which dates the change at
    /// its first accepted point past the crossing.
    ///
    /// `observer`, when given, is offered every accepted point of the
    /// segment, its starting image included.
    pub(in crate::engine) fn run_tran_segment_from_image(
        &self,
        netlist: &Netlist,
        image: &CapturedTransientState,
        (mesh, steps): (&[Value], Option<&[Value]>),
        max_step: Value,
        observer: Option<&AcceptedPointObserver<'_>>,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, CapturedTransientState), SimulationError> {
        let Some(&tstop) = mesh.last() else {
            return Err(SimulationError::Circuit(
                "a transient segment needs at least one mesh time".to_string(),
            ));
        };
        if !image.time.is_finite()
            || mesh.first().is_none_or(|&first| first <= image.time)
            || mesh.windows(2).any(|pair| pair[1] <= pair[0])
            || !tstop.is_finite()
            || steps.is_some_and(|steps| {
                steps.len() != mesh.len()
                    || steps.iter().any(|step| !(step.is_finite() && *step > 0.0))
            })
        {
            return Err(SimulationError::Circuit(format!(
                "a transient segment from t={:e} s needs a finite, strictly increasing mesh \
                 after it",
                image.time
            )));
        }
        if netlist.options.transient_noise.is_some() {
            return Err(SimulationError::Circuit(
                "a transient segment from a live image cannot continue transient noise: the \
                 image does not carry its streams' position"
                    .to_string(),
            ));
        }
        validate_transient_window(tstop, max_step)?;
        let times = [tstop];
        let mut capture = TransientStateCapture::from_image(&times, tstop, image)?;
        capture.observer = observer;
        let mut engine = self.resolved_for_netlist(netlist);
        engine.small_signal_state = None;
        engine.config.locked_time_grid = Some(std::sync::Arc::new(mesh.to_vec()));
        engine.config.locked_time_step_sizes =
            steps.map(|steps| std::sync::Arc::new(steps.to_vec()));
        let (result, _, _) = engine.run_tran_resolved_with_resume(
            netlist,
            netlist,
            TransientRunWindow {
                tstop,
                max_step,
                startup_mode: TransientStartupMode::from_uic(false),
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
                result_sink: None,
            },
        )?;
        let end = capture.into_captured().pop().ok_or_else(|| {
            SimulationError::Circuit(format!(
                "the transient segment from t={:e} s did not reach its stop time {tstop:e} s",
                image.time
            ))
        })?;
        Ok((result, end))
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
        let startup_mode = checkpoint.startup_mode();
        let times = [checkpoint.time];
        let capture = TransientStateCapture::new(&times, tstop)?;
        self.reset_convergence_quality();
        let mut engine = self.resolved_for_netlist(netlist);
        engine.small_signal_state = None;
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
                result_sink: None,
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

    /// A nonlinear deck: a sine drives an RC, a diode with junction charge
    /// clamps the capacitor, and an RL branch carries an inductor current.
    const NONLINEAR_DECK: &str = "segment from a live image\n\
                                 V1 in 0 SIN(0.5 2 1meg)\n\
                                 R0 in a 10\n\
                                 R1 a out 1k\n\
                                 C1 out 0 1n\n\
                                 D1 out 0 dclamp\n\
                                 R2 out m 100\n\
                                 L1 m 0 10u\n\
                                 .model dclamp d is=1e-14 cjo=2p\n\
                                 .TRAN 10n 5u\n\
                                 .END\n";

    /// The nonlinear deck driven by a pulse with ramps, so the segment lands
    /// on corners: each is a breakpoint the run solves at its left limit and
    /// restarts its integrator on leaving.
    const PULSE_DECK: &str = "segment across pulse corners\n\
                              V1 in 0 PULSE(0 2 0 20n 20n 480n 1u)\n\
                              R0 in a 10\n\
                              R1 a out 1k\n\
                              C1 out 0 1n\n\
                              D1 out 0 dclamp\n\
                              R2 out m 100\n\
                              L1 m 0 10u\n\
                              .model dclamp d is=1e-14 cjo=2p\n\
                              .TRAN 10n 5u\n\
                              .END\n";

    /// The nonlinear load behind an ideal hysteretic switch that a pulse
    /// closes above 1.2 V and opens below 0.8 V, so the switch changes four
    /// times inside the segment.
    const SWITCH_DECK: &str = "segment across switch changes\n\
                               V1 in 0 PULSE(0 2 0 20n 20n 480n 1u)\n\
                               S1 in a in 0 sw\n\
                               R1 a out 1k\n\
                               C1 out 0 1n\n\
                               D1 out 0 dclamp\n\
                               R2 out m 100\n\
                               L1 m 0 10u\n\
                               .model sw sw vt=1 vh=0.2 ron=10 roff=1meg\n\
                               .model dclamp d is=1e-14 cjo=2p\n\
                               .TRAN 10n 5u\n\
                               .END\n";

    /// A segment from a live image over a run's own later accepted times and
    /// steps is that run, bit for bit: the image's time is a capture, so a
    /// hard breakpoint the run restarts its integrator and predictor at, and
    /// the segment leaves it the same way, from the same accepted state and
    /// sparse-solver state. A pulse corner inside the segment is landed as
    /// the run lands it: solved at the source's left limit, with the
    /// integrator restarted on leaving it.
    #[test]
    fn a_segment_from_a_live_image_repeats_the_run_bit_for_bit() {
        for deck in [RC_DECK, NONLINEAR_DECK, PULSE_DECK] {
            let (run, segment, first) = segment_of_its_own_run(deck);
            assert_eq!(
                segment.0.time,
                run.0.time[first..],
                "the segment lands on every mesh time and nowhere else"
            );
            for node in ["in", "a", "out", "m"] {
                let Some(from_run) = run.0.try_voltage_waveform_named(node) else {
                    continue;
                };
                let from_segment = segment
                    .0
                    .try_voltage_waveform_named(node)
                    .expect("the segment retains the run's nodes");
                for (index, (left, right)) in from_run[first..].iter().zip(from_segment).enumerate()
                {
                    assert_eq!(
                        left.to_bits(),
                        right.to_bits(),
                        "V({node}) at t={:e} s: the run holds {left:e}, the segment {right:e}",
                        segment.0.time[index]
                    );
                }
            }
            let bits =
                |solution: &[Value]| solution.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
            assert_eq!(
                bits(&segment.1.solution),
                bits(&run.1.solution),
                "the segment ends on the state the run accepts there"
            );
        }
    }

    /// A segment across ideal switch changes cuts its locked step where a
    /// switch's control crosses the threshold its accepted state changes at,
    /// so each change is dated at its crossing, restarts at order one over a
    /// tenth of the mesh step it landed in (half of what is left when that
    /// tenth does not fit twice), and lands on every one of the run's
    /// accepted times. The pulse ramps V(in) linearly, so the switch opens
    /// where it falls through 0.8 V (2.512 us, 3.512 us) and closes where it
    /// rises through 1.2 V (3.012 us).
    ///
    /// The run dates each change at its first accepted point past the
    /// crossing instead, up to 8.2 ps late here (2.5119846 us, then
    /// 2.5120082 us), and the ~0.6-1 mA the switch carries moves the 1 nF
    /// capacitor by 0.6-1 mV/ns, so the segment leaves each change up to
    /// about 5e-6 V from the run (5.0e-6 V measured at the worst point).
    /// Without the cut the segment repeats the run bit for bit.
    #[test]
    fn a_segment_across_switch_changes_dates_them_at_their_crossings() {
        let (run, segment, first) = segment_of_its_own_run(SWITCH_DECK);
        let cuts: Vec<Value> = segment
            .0
            .time
            .iter()
            .copied()
            .filter(|time| !run.0.time[first..].contains(time))
            .collect();
        assert_eq!(
            cuts.len(),
            6,
            "a landing and a restart per switch change: {cuts:?}"
        );
        let mesh = &run.0.time[first..];
        for (pair, crossing) in cuts.chunks(2).zip([2.512e-6, 3.012e-6, 3.512e-6]) {
            let (landing, restart) = (pair[0], pair[1]);
            assert!(
                (landing - crossing).abs() <= 1.0e-15,
                "the change at {crossing:e} s is dated at {landing:e} s"
            );
            let start = mesh
                .iter()
                .copied()
                .filter(|time| *time < landing)
                .fold(Value::NEG_INFINITY, Value::max);
            let target = mesh
                .iter()
                .copied()
                .find(|time| *time > landing)
                .expect("the run steps past the change");
            let tenth = 0.1 * (target - start);
            let expected = if target - landing >= 2.0 * tenth {
                landing + tenth
            } else {
                landing + 0.5 * (target - landing)
            };
            assert_eq!(
                restart.to_bits(),
                expected.to_bits(),
                "the order-one restart after {landing:e} s ends at {expected:e} s, not {restart:e} s"
            );
        }
        for node in ["in", "a", "out", "m"] {
            let from_run = run.0.try_voltage_waveform_named(node).expect("run node");
            let from_segment = segment
                .0
                .try_voltage_waveform_named(node)
                .expect("segment node");
            for (time, left) in run.0.time[first..].iter().zip(&from_run[first..]) {
                let index = segment
                    .0
                    .time
                    .iter()
                    .position(|candidate| candidate == time)
                    .unwrap_or_else(|| panic!("the segment lands on the run's {time:e} s"));
                let right = from_segment[index];
                assert!(
                    (left - right).abs() <= 1.0e-5,
                    "V({node}) at t={time:e} s: the run holds {left:e}, the segment {right:e}"
                );
            }
        }
    }

    /// A run that captures its state at 2.25 us and stops at 4 us, and the
    /// segment from that image over the run's later accepted times and steps:
    /// (the run and its last capture, the segment and its end, the index of
    /// the image's time in the run).
    fn segment_of_its_own_run(
        deck: &str,
    ) -> (
        (TransientResult, CapturedTransientState),
        (TransientResult, CapturedTransientState),
        usize,
    ) {
        {
            let netlist = Netlist::parse(deck).expect("the deck parses");
            let engine = Engine::default();
            let (tstop, max_step) = (4.0e-6, 1.0e-8);
            let times = [2.25e-6, tstop];
            let (run, mut captured) = engine
                .run_tran_capturing_states(
                    &netlist,
                    tstop,
                    max_step,
                    TransientStartupMode::OperatingPoint,
                    &times,
                    &NoAbort,
                )
                .expect("the captured run converges");
            let first = run
                .time
                .iter()
                .position(|time| *time == times[0])
                .expect("the run lands on the image's time");
            let mesh = &run.time[first + 1..];
            let steps = &run.step_sizes[first + 1..];
            let (segment, end) = engine
                .run_tran_segment_from_image(
                    &netlist,
                    &captured[0],
                    (mesh, Some(steps)),
                    max_step,
                    None,
                    &NoAbort,
                )
                .expect("the segment converges");
            assert_eq!(end.time, captured[1].time);
            let last = captured.pop().expect("two captures");
            ((run, last), (segment, end), first)
        }
    }

    /// The shooting coordinates a caller writes into an image are where the
    /// segment starts: an RC whose capacitor is written to 0.3 V with its
    /// source held at 0 decays as 0.3·e^{−t/RC}.
    #[test]
    fn a_segment_starts_from_the_state_written_into_its_image() {
        let netlist = Netlist::parse(RC_DECK).expect("the deck parses");
        let engine = Engine::default();
        // The pulse is low from 10 us to 20 us; C1 has discharged by 15 us.
        let (t0, max_step) = (15.0e-6, 1.0e-9);
        let (_, mut captured) = engine
            .run_tran_capturing_states(
                &netlist,
                t0,
                max_step,
                TransientStartupMode::OperatingPoint,
                &[t0],
                &NoAbort,
            )
            .expect("the captured run converges");
        let mut image = captured.remove(0);
        let out = image.circuit.get_node_by_name("out").expect("out") - 1;
        image.solution[out] = 0.3;
        let mesh: Vec<Value> = (1..=200).map(|k| t0 + Value::from(k) * 1.0e-8).collect();
        let (segment, _) = engine
            .run_tran_segment_from_image(&netlist, &image, (&mesh, None), max_step, None, &NoAbort)
            .expect("the segment converges");
        let values = segment.try_voltage_waveform_named("out").expect("out");
        let tau = 1.0e3 * 1.0e-9;
        for (time, value) in segment.time.iter().zip(values) {
            let expected = 0.3 * (-(time - t0) / tau).exp();
            // Order one leaves the image, then the trapezoid at h = tau/100.
            assert!(
                (value - expected).abs() <= 2.0e-4 * 0.3,
                "at t={time:e} s v(out) = {value}, the decay gives {expected}"
            );
        }
    }

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

    /// A `.TRAN` task's run that captures its cards' states is, up to the last
    /// of them, the run a card's own state transient takes: the same accepted
    /// points and the same states, bit for bit. It then continues to its stop
    /// time, and away from the requested times it is the uncaptured run.
    #[test]
    fn a_run_through_its_stop_captures_what_a_card_run_captures() {
        let netlist = Netlist::parse(RC_DECK).expect("the deck parses");
        let engine = Engine::default();
        let (tstop, max_step) = (5.0e-6, 1.0e-8);
        let times = [1.0e-6, 3.0e-6];
        let (card_run, card_states) = engine
            .run_tran_capturing_states(
                &netlist,
                tstop,
                max_step,
                TransientStartupMode::OperatingPoint,
                &times,
                &NoAbort,
            )
            .expect("the card run converges");
        let (task_run, task_states) = engine
            .run_tran_capturing_states_to_stop(
                &netlist,
                tstop,
                max_step,
                TransientStartupMode::OperatingPoint,
                &times,
                &NoAbort,
            )
            .expect("the task run converges");
        let plain = engine
            .run_tran(&netlist, tstop, max_step)
            .expect("the uncaptured run converges");

        assert_eq!(task_run.time.last().copied(), Some(tstop));
        let shared = card_run.time.len();
        assert_eq!(task_run.time[..shared], card_run.time[..]);
        let out = |result: &TransientResult| {
            result
                .try_voltage_waveform_named("out")
                .expect("out is retained")
                .to_vec()
        };
        assert_eq!(out(&task_run)[..shared], out(&card_run)[..]);
        assert_eq!(task_states.len(), card_states.len());
        for (task, card) in task_states.iter().zip(&card_states) {
            assert_eq!(task.0.time.to_bits(), card.time.to_bits());
            assert_eq!(
                task.0
                    .solution
                    .iter()
                    .map(|v| v.to_bits())
                    .collect::<Vec<_>>(),
                card.solution
                    .iter()
                    .map(|v| v.to_bits())
                    .collect::<Vec<_>>(),
            );
        }
        // Before the first requested time the breakpoints have not yet
        // shaped a step: the run is the uncaptured one.
        let before = plain
            .time
            .iter()
            .take_while(|&&time| time < 0.5 * times[0])
            .count();
        assert!(before > 2, "the comparison spans accepted points");
        assert_eq!(task_run.time[..before], plain.time[..before]);
        assert_eq!(out(&task_run)[..before], out(&plain)[..before]);
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
