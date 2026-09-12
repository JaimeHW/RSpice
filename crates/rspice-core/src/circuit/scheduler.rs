//! One owner for the circuit's discrete-event lanes.
//!
//! Two kernels queue activations the analog stepper has to land points for:
//! the HDL process wheel every enrolled mixed instance schedules into, and the
//! XSPICE code models' event queue. Both are driven by the same clock — the
//! analog solver's — and both answer the same two questions to it: *what is
//! next*, and *what is the smallest interval you can advance by*. Until this
//! type they answered them from different places, and each caller reached
//! through [`CircuitData`](super::CircuitData) into whichever sub-object it
//! happened to know about, folding the lanes again on its own rule.
//!
//! So the state that answers those questions lives here, and the questions are
//! methods on it. This is the façade step of the scheduler migration: the
//! fields are the same fields, moved rather than copied, and nothing about the
//! trial protocol, the bridges or the per-host wheel moves with them.

use crate::Value;
use crate::xspice::event_scheduler::Instant;

use super::external_models::ScheduleOwner;

/// The instant a queued time is, or `None` when it is no instant at all.
///
/// Negative, infinite and NaN times name no point the analog stepper can land
/// on, so they are not activations. Every lane's time reaches here from a key
/// that was already an [`Instant`] — the XSPICE queue's bit pattern, an HDL
/// tick through `ticks_to_seconds` — so the conversion is exact and its
/// refusal is unreachable; it is written out rather than asserted because a
/// silently-taken NaN would otherwise win every fold.
fn activation_at(seconds: Value) -> Option<Instant> {
    Instant::from_seconds(seconds)
}

/// One queued activation, with the owner a diagnostic names.
///
/// Borrowed, not rendered, for the reason [`ScheduleOwner`] gives: this is
/// produced once per accepted transient point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Activation<'a> {
    /// The exact instant the activation is due at.
    pub(crate) at: Instant,
    /// Whose schedule it is, when the lane can say whose.
    ///
    /// The HDL wheel also holds wakeups belonging to no module of the design —
    /// the host's own nonblocking-update wakeup, external bit drivers — and
    /// `None` is the honest answer for one of those.
    pub(crate) owner: Option<ScheduleOwner<'a>>,
}

impl Activation<'_> {
    /// The absolute time the stepper lands on.
    pub(crate) fn seconds(self) -> Value {
        self.at.seconds()
    }
}

/// Which of the HDL wheel's activations a folded question is about.
///
/// The two callers that fold both lanes do not ask quite the same thing, and
/// the difference is physics rather than taste:
///
/// - The landing target ([`Self::landing`]) is the set of activations the
///   stepper *must* put an accepted point on. An HDL activation closer to the
///   accepted point than the solver's floor is left to the module that is
///   already coalescing it, because there is no analog instant between the two
///   to land on, and a target for it would only spend accepted points marching
///   at the floor.
/// - The sub-minimum schedule bound ([`Self::scheduled`]) asks whose schedule
///   is pacing the accepted points, which is the same pair of lanes without
///   that filter: a run of floor landings is exactly what it is looking for.
///
/// Both take the code-model queue only when it is coupled to a mixed module.
/// Coupling is what makes those events landed rather than merely coalescible,
/// so an uncoupled queue paces no accepted point — it reaches the stepper
/// through the breakpoint list, which asks for its lane by itself
/// ([`CircuitScheduler::next_xspice_activation`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum ActivationLanes {
    /// Every activation the wheel has queued, beside the coupled queue.
    Scheduled,
    /// The same, restricted to what the stepper can still land on: an HDL
    /// activation counts only when `due - accepted_time >= hard_min_dt`, the
    /// same test the coordinator's own missed-breakpoint guard applies.
    Landing {
        accepted_time: Value,
        hard_min_dt: Value,
    },
}

impl ActivationLanes {
    /// Everything the sub-minimum schedule bound calls a scheduled activation.
    pub(crate) const fn scheduled() -> Self {
        Self::Scheduled
    }

    /// The lanes `accepted_veriloga_event_time` must land a point on.
    pub(crate) const fn landing(accepted_time: Value, hard_min_dt: Value) -> Self {
        Self::Landing {
            accepted_time,
            hard_min_dt,
        }
    }

    /// Whether an HDL activation at `seconds` is one this question counts.
    fn admits_hdl(self, seconds: Value) -> bool {
        match self {
            Self::Scheduled => true,
            Self::Landing {
                accepted_time,
                hard_min_dt,
            } => seconds - accepted_time >= hard_min_dt,
        }
    }
}

/// Every discrete-event lane the analog stepper shares its clock with.
///
/// Owned by [`CircuitData`](super::CircuitData), one per circuit. A circuit
/// with no code model and no mixed module has one of these too; every lane in
/// it is then empty and every question answers `None`.
#[derive(Debug, Clone)]
pub(crate) struct CircuitScheduler {
    /// Resolved event-node values, per-driver state, and event times, shared
    /// with rollback snapshots until an event writes through them.
    pub(crate) xspice_event_values: crate::xspice::SharedXspiceEventValues,
    /// Circuit-level XSPICE digital event queue, shared with rollback
    /// snapshots until an event is scheduled or drained.
    pub(crate) xspice_event_queue: crate::xspice::SharedXspiceEventQueue,
    /// Net-to-instance sensitivity for the settle loop, built on first use.
    /// Derived from port directions and connections, so it is not part of any
    /// rollback snapshot.
    pub(crate) xspice_event_dispatch: Option<super::xspice_dispatch::XspiceEventDispatch>,
    /// One process, driver and event-queue authority for every mixed HDL
    /// instance.
    #[cfg(feature = "veriloga")]
    pub(crate) mixed_digital_coordinator: Option<crate::xspice::verilog::MixedDigitalCoordinator>,
    /// Original XSPICE drivers attached to the coordinator's event-bit groups.
    ///
    /// Present exactly when a code model shares an event net with a mixed
    /// module, which is what makes the whole XSPICE queue a lane the stepper
    /// lands points for rather than one the breakpoint manager may coalesce.
    #[cfg(feature = "veriloga")]
    pub(super) mixed_xspice_bindings:
        Option<std::sync::Arc<super::external_models::XspiceDigitalBindings>>,
    /// Smallest interval the analog solver may advance by, or zero when
    /// nothing has declared one. See [`Self::set_analog_step_floor`].
    #[cfg(feature = "veriloga")]
    analog_step_floor: Value,
}

impl CircuitScheduler {
    /// An empty scheduler: no code-model queue, no HDL wheel, no floor.
    pub(crate) fn new() -> Self {
        Self {
            xspice_event_values: crate::xspice::SharedXspiceEventValues::default(),
            xspice_event_queue: crate::xspice::SharedXspiceEventQueue::new(),
            xspice_event_dispatch: None,
            #[cfg(feature = "veriloga")]
            mixed_digital_coordinator: None,
            #[cfg(feature = "veriloga")]
            mixed_xspice_bindings: None,
            #[cfg(feature = "veriloga")]
            analog_step_floor: 0.0,
        }
    }

    /// Whether a code model shares an event net with a mixed module.
    ///
    /// What makes the code-model queue a lane the stepper lands points for
    /// rather than one the breakpoint manager may coalesce, and therefore what
    /// [`Self::next_activation`] tests before folding it.
    pub(crate) fn has_coupled_event_nets(&self) -> bool {
        #[cfg(feature = "veriloga")]
        {
            self.mixed_xspice_bindings.is_some()
        }
        #[cfg(not(feature = "veriloga"))]
        {
            false
        }
    }

    /// The earliest activation across both lanes, and its owner.
    ///
    /// One fold, one rule. Ties go to the HDL wheel, which is the landed
    /// same-instant order across the two kernels: HDL events run, then the
    /// code models' wave. Nothing here filters on the accepted point — the
    /// callers that need "strictly after" apply it themselves, because the
    /// landing fold and the schedule bound disagree about what to do with an
    /// event at or before it.
    pub(crate) fn next_activation(
        &self,
        lanes: ActivationLanes,
    ) -> Result<Option<Activation<'_>>, crate::SimulationError> {
        let mut earliest = self
            .next_hdl_activation()?
            .filter(|activation| lanes.admits_hdl(activation.seconds()));
        if self.has_coupled_event_nets()
            && let Some(coupled) = self.next_xspice_activation()
            && earliest.is_none_or(|held| coupled.at < held.at)
        {
            earliest = Some(coupled);
        }
        Ok(earliest)
    }

    /// The HDL wheel's earliest activation across every enrolled instance,
    /// with the instance that owns the process it belongs to.
    ///
    /// One of the two entries in the runtime breakpoint list, which carries
    /// each lane's own next activation rather than the earliest of them.
    pub(crate) fn next_hdl_activation(
        &self,
    ) -> Result<Option<Activation<'_>>, crate::SimulationError> {
        let Some((instance, seconds)) = self.next_hdl_event()? else {
            return Ok(None);
        };
        Ok(activation_at(seconds).map(|at| Activation {
            at,
            owner: instance.map(ScheduleOwner::veriloga),
        }))
    }

    /// The code-model queue's earliest activation, coupled or not.
    ///
    /// Infallible: the queue's key already *is* an instant, so there is no
    /// tick arithmetic here to refuse. Every driver in it is a code model's
    /// output port, so the answer is never anonymous.
    pub(crate) fn next_xspice_activation(&self) -> Option<Activation<'_>> {
        let (instance, seconds) = self.xspice_event_queue.next_event_instance()?;
        Some(Activation {
            at: activation_at(seconds)?,
            owner: Some(ScheduleOwner::code_model(instance)),
        })
    }

    /// The wheel's earliest activation as the coordinator reports it.
    ///
    /// A build without the Verilog-A/AMS route has no wheel, so this is where
    /// the feature gate stops and every question above it is one body under
    /// every feature set.
    #[cfg(feature = "veriloga")]
    fn next_hdl_event(&self) -> Result<Option<(Option<&str>, Value)>, crate::SimulationError> {
        self.mixed_digital_coordinator
            .as_ref()
            .map(|coordinator| coordinator.next_event_time().map_err(shared_error))
            .transpose()
            .map(Option::flatten)
    }

    #[cfg(not(feature = "veriloga"))]
    fn next_hdl_event(&self) -> Result<Option<(Option<&str>, Value)>, crate::SimulationError> {
        Ok(None)
    }

    /// Declare the smallest interval the analog solver may advance by.
    ///
    /// A module's discrete half schedules on its own declared precision, which
    /// can be far finer than this. Telling every lane what the analog side can
    /// resolve is what makes an activation inside that window land instead of
    /// being reported as a breakpoint the stepper skipped: without it the two
    /// halves disagree about what "stepped past an activation" means, and a
    /// schedule the analog side merely cannot resolve is reported as a lost
    /// breakpoint.
    ///
    /// One floor, every lane. A code model sharing an event net with a mixed
    /// module schedules on the same picosecond-and-finer grid — ngspice clamps
    /// a gate delay at 1 ps, which is a tenth of the minimum a one-second
    /// maximum timestep leaves the solver — and its events are landed by the
    /// same contract as an HDL tick, so its guard needs the same interval to
    /// measure against rather than a second knob. The value is normalized once
    /// here and stored once here; the coordinator holds a published copy of
    /// this one value so that its own missed-breakpoint guard can read it
    /// without the circuit, and [`Self::analog_step_floor`] is what everything
    /// else reads.
    #[cfg(feature = "veriloga")]
    pub(crate) fn set_analog_step_floor(&mut self, floor: Value) {
        self.analog_step_floor = if floor.is_finite() && floor > 0.0 {
            floor
        } else {
            0.0
        };
        if let Some(coordinator) = self.mixed_digital_coordinator.as_mut() {
            coordinator.set_analog_step_floor(self.analog_step_floor);
        }
    }

    /// The floor every lane measures a stepped-past activation against, or
    /// zero when no solver has declared one.
    #[cfg(feature = "veriloga")]
    pub(crate) fn analog_step_floor(&self) -> Value {
        self.analog_step_floor
    }
}

impl super::CircuitData {
    /// [`CircuitScheduler::next_activation`], for the stepper that holds the
    /// circuit rather than the scheduler.
    pub(crate) fn next_activation(
        &self,
        lanes: ActivationLanes,
    ) -> Result<Option<Activation<'_>>, crate::SimulationError> {
        self.scheduler.next_activation(lanes)
    }

    /// [`CircuitScheduler::next_hdl_activation`].
    pub(crate) fn next_hdl_activation(
        &self,
    ) -> Result<Option<Activation<'_>>, crate::SimulationError> {
        self.scheduler.next_hdl_activation()
    }

    /// [`CircuitScheduler::next_xspice_activation`].
    pub(crate) fn next_xspice_activation(&self) -> Option<Activation<'_>> {
        self.scheduler.next_xspice_activation()
    }

    /// Publish the transient stepper's hard minimum timestep to every lane.
    ///
    /// The scheduler stores and normalizes it once; every mixed module holds a
    /// published copy of that one value, because a host's own
    /// missed-breakpoint guard runs deep inside a trial where the circuit is
    /// split apart and the scheduler is not reachable.
    #[cfg(feature = "veriloga")]
    pub(crate) fn set_analog_step_floor(&mut self, floor: Value) {
        self.scheduler.set_analog_step_floor(floor);
        let floor = self.scheduler.analog_step_floor();
        for host in &mut self.mixed_signal_hosts {
            host.set_analog_step_floor(floor);
        }
    }
}

/// A digital-execution failure, as the circuit reports it.
#[cfg(feature = "veriloga")]
pub(super) fn shared_error(
    error: crate::xspice::verilog::MixedSignalError,
) -> crate::SimulationError {
    crate::SimulationError::Circuit(format!("mixed circuit digital execution: {error}"))
}
