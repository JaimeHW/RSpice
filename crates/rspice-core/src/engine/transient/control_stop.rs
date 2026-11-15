//! A control script's `stop when time = T` and `resume`.
//!
//! ngspice lands the transient on the stop time (frontend/breakp.c:192 sets
//! the breakpoint), interrupts the run at the accepted point there, and
//! `resume` (frontend/runcoms2.c:63 com_resume) continues the same run from
//! that point, with whatever `alter` changed in the meantime.
//!
//! Here the run ends at the stop time and keeps its state there twice: live,
//! as a clone of the circuit, which holds every device's state, an XSPICE
//! model's events included; and as a checkpoint, when every device's state
//! serializes. A resume after `alter` changed only independent sources
//! continues the live image with those sources driven anew, as ngspice's
//! sources read their altered parameters from the next time point on. A
//! resume after `alter` changed any other value builds the circuit from the
//! altered netlist and restores the checkpoint into it, as a breakpoint
//! restarts the integrator.

use super::state_capture::{CapturedTransientState, TransientStateCapture};
use super::{
    FinalCheckpointRetention, ResumeValidation, TransientCheckpoint, TransientEnd,
    TransientResumePlan, TransientRunWindow, TransientStartupMode, netlist_checkpoint_identity,
    validate_transient_window,
};
use crate::abort_signal::AbortSignal;
use crate::engine::{Engine, SimulationError, TransientResult, UnsupportedCapabilityError};
use crate::identity::AnalysisKind;
use crate::netlist::{Element, ElementKind};
use crate::{Netlist, Value};

/// A transient a control script's `stop` interrupted, as `resume` continues
/// it.
pub(in crate::engine) struct TransientStop {
    /// The netlist the run began on, whose identity every checkpoint of the
    /// run carries.
    origin: Netlist,
    /// The netlist the interrupted segment ran: the origin with what `alter`
    /// had changed by then.
    ran: Netlist,
    /// The run's state at the stop, live.
    image: CapturedTransientState,
    /// The same state as a checkpoint, or why the circuit's state cannot be
    /// one.
    checkpoint: Result<TransientCheckpoint, SimulationError>,
}

impl TransientStop {
    /// The time the run was interrupted at.
    pub(in crate::engine) fn time(&self) -> Value {
        self.image.time
    }
}

/// Where a segment of a control transient starts.
#[derive(Clone, Copy)]
enum SegmentStart<'a> {
    /// The run's start: its operating point, or its UIC state.
    Fresh(TransientStartupMode),
    /// A live image whose sources the segment's netlist already drives.
    Image(&'a CapturedTransientState),
    /// A checkpoint, restored into the circuit the segment's netlist builds.
    Checkpoint(&'a TransientCheckpoint),
}

impl SegmentStart<'_> {
    fn time(self) -> Value {
        match self {
            Self::Fresh(_) => 0.0,
            Self::Image(image) => image.time,
            Self::Checkpoint(checkpoint) => checkpoint.time,
        }
    }

    /// How the segment starts. A segment from a live image starts from an
    /// accepted state, as one from a checkpoint does, and skips every
    /// startup step; it starts no UIC run, and a checkpoint it keeps says
    /// so (a UIC checkpoint claims the linearized startup a UIC run takes).
    fn startup_mode(self) -> TransientStartupMode {
        match self {
            Self::Fresh(mode) => mode,
            Self::Image(_) => TransientStartupMode::from_uic(false),
            Self::Checkpoint(checkpoint) => checkpoint.startup_mode(),
        }
    }
}

/// The state a segment kept at its stop: live, and as a checkpoint when it
/// was asked to keep one.
type StopState = (CapturedTransientState, Option<TransientCheckpoint>);

impl Engine {
    /// Run the transient of `netlist` from its start and interrupt it at
    /// `stop`, keeping its state there for [`Self::resume_tran_from_control_stop`].
    /// A model that finishes the run first completes it, and no state is
    /// kept.
    pub(in crate::engine) fn run_tran_to_control_stop(
        &self,
        netlist: &Netlist,
        window: (Value, Value),
        startup_mode: TransientStartupMode,
        stop: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Option<TransientStop>), SimulationError> {
        // Neither a live image nor a checkpoint carries the noise streams'
        // position, so a resumed run would replay a different realization.
        if netlist.options.transient_noise.is_some() {
            return Err(Self::transient_noise_checkpoint_refusal());
        }
        let capability = self.preflight_transient_checkpoint_with_abort(netlist, abort)?;
        let refusal = capability.require_resumable().err().map(|detail| {
            let refusal = UnsupportedCapabilityError::new(
                "analysis.tran.checkpoint_capability",
                format!(
                    "resume after `alter` changed a value other than an independent source's \
                     builds the circuit anew and restores the interrupted run's state into it, \
                     which this circuit's state cannot be: {detail}"
                ),
            )
            .for_analysis_kind(AnalysisKind::Tran);
            SimulationError::from(match capability.blocking_instance() {
                Some(instance) => refusal.for_instance(instance),
                None => refusal,
            })
        });
        let (result, reached) = self.control_segment(
            (netlist, netlist),
            SegmentStart::Fresh(startup_mode),
            window,
            (Some(stop), refusal.is_none()),
            abort,
        )?;
        Ok((
            result,
            reached.map(|(image, checkpoint)| TransientStop {
                origin: netlist.clone(),
                ran: netlist.clone(),
                image,
                checkpoint: kept(checkpoint, refusal),
            }),
        ))
    }

    /// Continue the run `from` holds on `netlist`, the netlist it ran with
    /// what `alter` has changed since, to `stop`, or to its end without one.
    /// Every changed value takes effect from the stop's time on.
    pub(in crate::engine) fn resume_tran_from_control_stop(
        &self,
        netlist: &Netlist,
        from: TransientStop,
        window: (Value, Value),
        stop: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Option<TransientStop>), SimulationError> {
        let TransientStop {
            origin,
            ran,
            image,
            checkpoint,
        } = from;
        let (result, reached, refusal) =
            match self.image_with_altered_sources(netlist, &ran, image)? {
                Some(image) => {
                    let (result, reached) = self.control_segment(
                        (netlist, &origin),
                        SegmentStart::Image(&image),
                        window,
                        (stop, checkpoint.is_ok()),
                        abort,
                    )?;
                    (result, reached, checkpoint.err())
                }
                None => {
                    let checkpoint = checkpoint?;
                    let (result, reached) = self.control_segment(
                        (netlist, &origin),
                        SegmentStart::Checkpoint(&checkpoint),
                        window,
                        (stop, true),
                        abort,
                    )?;
                    (result, reached, None)
                }
            };
        Ok((
            result,
            reached.map(|(image, checkpoint)| TransientStop {
                origin,
                ran: netlist.clone(),
                image,
                checkpoint: kept(checkpoint, refusal),
            }),
        ))
    }

    /// `image`, its independent sources driven as `netlist` drives them,
    /// when that is all `netlist` changes from `ran`, the netlist the image's
    /// circuit was built and driven from; `None` when it changes anything
    /// else.
    fn image_with_altered_sources(
        &self,
        netlist: &Netlist,
        ran: &Netlist,
        mut image: CapturedTransientState,
    ) -> Result<Option<CapturedTransientState>, SimulationError> {
        let Some(altered) = altered_sources(ran, netlist) else {
            return Ok(None);
        };
        let engine = self.resolved_for_netlist(netlist);
        for element in altered {
            let (ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)) =
                &element.kind
            else {
                return Ok(None);
            };
            let (excitation, pwl_waveform) =
                engine.independent_source_excitation(netlist, &element.name, spec)?;
            let circuit = &mut image.circuit;
            let replaced = if matches!(element.kind, ElementKind::VoltageSource(_)) {
                circuit
                    .voltage_sources
                    .replace_excitation(&element.name, excitation, pwl_waveform)
            } else {
                circuit
                    .current_sources
                    .replace_excitation(&element.name, excitation, pwl_waveform)
            };
            if !replaced {
                return Ok(None);
            }
        }
        Ok(Some(image))
    }

    /// Run one segment of a control transient on `netlist` from `start` to
    /// `stop`, or to the run's end without one; every checkpoint it keeps
    /// carries `origin`'s identity. At a stop it keeps its live state, and
    /// its checkpoint when `checkpointed`; a run a model finishes before its
    /// stop keeps neither.
    fn control_segment(
        &self,
        (netlist, origin): (&Netlist, &Netlist),
        start: SegmentStart<'_>,
        (tstop, max_step): (Value, Value),
        (stop, checkpointed): (Option<Value>, bool),
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Option<StopState>), SimulationError> {
        validate_transient_window(tstop, max_step)?;
        let begins = start.time();
        if tstop <= begins {
            return Err(SimulationError::Circuit(format!(
                "resume stop time {tstop:e} s must exceed the interrupted run's time {begins:e} s"
            )));
        }
        if let Some(stop) = stop
            && !(stop.is_finite() && stop > begins && stop < tstop)
        {
            return Err(SimulationError::Circuit(format!(
                "a transient stop at {stop:e} s must lie inside the run's ({begins:e}, {tstop:e}) s"
            )));
        }
        let times = [stop.unwrap_or(tstop)];
        let capture = match start {
            SegmentStart::Image(image) => TransientStateCapture::from_image(&times, tstop, image)?,
            SegmentStart::Fresh(_) | SegmentStart::Checkpoint(_) => {
                TransientStateCapture::new(&times, tstop)?
            }
        };
        self.reset_convergence_quality();
        let engine = self.resolved_for_netlist(netlist);
        let (result, end, _) = engine.run_tran_resolved_with_resume(
            netlist,
            origin,
            TransientRunWindow {
                tstop,
                max_step,
                startup_mode: start.startup_mode(),
                dc_seed: None,
                integral_trace: None,
            },
            abort,
            TransientResumePlan {
                resume: match start {
                    SegmentStart::Checkpoint(checkpoint) => Some(checkpoint),
                    SegmentStart::Fresh(_) | SegmentStart::Image(_) => None,
                },
                resume_validation: ResumeValidation::ExactNetlist,
                final_checkpoint_retention: if stop.is_some() && checkpointed {
                    FinalCheckpointRetention::Retained
                } else {
                    FinalCheckpointRetention::Discarded
                },
                scheduled_checkpoint_times: &[],
                checkpoint_observer: None,
                state_capture: Some(&capture),
            },
        )?;
        if stop.is_none() {
            return Ok((result, None));
        }
        let reached = capture.into_captured().pop().map(|image| {
            let checkpoint = match end {
                Some(TransientEnd::Checkpoint(checkpoint)) => Some(checkpoint),
                _ => None,
            };
            (image, checkpoint)
        });
        Ok((result, reached))
    }
}

/// The checkpoint a stop keeps, or why it keeps none.
fn kept(
    checkpoint: Option<TransientCheckpoint>,
    refusal: Option<SimulationError>,
) -> Result<TransientCheckpoint, SimulationError> {
    match (checkpoint, refusal) {
        (Some(checkpoint), _) => Ok(checkpoint),
        (None, Some(refusal)) => Err(refusal),
        (None, None) => Err(SimulationError::Circuit(
            "the interrupted transient kept no checkpoint at its stop".to_string(),
        )),
    }
}

/// The independent sources `altered` drives differently from `ran`, when
/// that is all it changes; `None` when it changes anything else.
fn altered_sources<'a>(ran: &Netlist, altered: &'a Netlist) -> Option<Vec<&'a Element>> {
    if ran.elements.len() != altered.elements.len() {
        return None;
    }
    let mut probe = altered.clone();
    let mut changed = Vec::new();
    for ((element, before), restored) in altered
        .elements
        .iter()
        .zip(&ran.elements)
        .zip(&mut probe.elements)
    {
        let source = matches!(
            (&element.kind, &before.kind),
            (ElementKind::VoltageSource(_), ElementKind::VoltageSource(_))
                | (ElementKind::CurrentSource(_), ElementKind::CurrentSource(_))
        );
        // A source specification has no equality of its own; its debug form
        // spells every value exactly.
        if source
            && element.name == before.name
            && format!("{:?}", element.kind) != format!("{:?}", before.kind)
        {
            restored.kind = before.kind.clone();
            changed.push(element);
        }
    }
    (netlist_checkpoint_identity(&probe) == netlist_checkpoint_identity(ran)).then_some(changed)
}
