//! Event scheduling for XSPICE event-driven simulation.
//!
//! [`EventValue`] is what a digital or real code-model output carries.
//! [`XspiceEventScheduler`] is the analog-seconds face of the discrete-event
//! kernel next door: the XSPICE path speaks in seconds because the analog
//! spine does, and the kernel speaks in integer ticks because an event order
//! has to be exact.

use super::digital::DigitalValue;
use super::event_scheduler::{
    EventScheduler, EventTarget, SchedulerError, SchedulerLimits, SchedulerRegion, TimeResolution,
};
use crate::{NodeId, Value};
use std::collections::HashMap;
use std::sync::Arc;

//=============================================================================
// Event Types
//=============================================================================

/// Payload carried by a scheduled XSPICE event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventValue {
    /// 12-state digital event value.
    Digital(DigitalValue),
    /// Real-valued event-node value.
    Real(Value),
}

/// One executed event, in the terms the analog side works in.
///
/// The kernel hands out a `ScheduledEvent` keyed by tick and target; this is
/// the same thing with the tick converted back to seconds and the target
/// flattened, so the drain can move the driver's names straight into its
/// driver key without copying them.
#[derive(Debug, Clone)]
pub(crate) struct XspiceEvent {
    /// Absolute time the event occurs, in analog seconds.
    pub(crate) time: Value,
    /// Target node identifier (internal representation).
    pub(crate) node_id: usize,
    /// Port name on the driving instance.
    pub(crate) port_name: String,
    /// Vector element index distinguishing separate drivers on one port.
    pub(crate) driver_index: usize,
    /// Instance that scheduled the event.
    pub(crate) instance: String,
    /// New event value.
    pub(crate) value: EventValue,
}

//=============================================================================
// Seconds <-> ticks
//=============================================================================

/// Analog seconds as a scheduler tick.
///
/// The kernel's decimal [`TimeResolution`] is not what this path uses, and the
/// reason is worth stating: XSPICE event times are seconds chosen by code
/// models and by the analog step controller, not points on a declared grid.
/// `next_event_time` hands them straight to the transient breakpoint manager,
/// so quantizing them would move the times events fire at. A decimal grid also
/// bounds event time — 1 fs ticks stop being exactly invertible past 2.25 s,
/// which is far inside the range a transient run uses.
///
/// For a non-negative finite `f64` the IEEE-754 bit pattern read as a `u64` is
/// strictly monotone in the value and exactly invertible, which is everything
/// the kernel asks of a tick: it orders, and it converts back. `None` is the
/// answer for a time that cannot be scheduled at all, which is the same set
/// the queue has always dropped: negative, infinite, or NaN.
fn event_tick(seconds: Value) -> Option<u64> {
    if !seconds.is_finite() || seconds < 0.0 {
        return None;
    }
    // Positive and negative zero are one instant with two bit patterns, and
    // the negative one does not compare below the positive one as a `u64`.
    Some(if seconds == 0.0 { 0 } else { seconds.to_bits() })
}

/// The inverse of [`event_tick`] over the ticks it produces.
fn tick_seconds(tick: u64) -> Value {
    Value::from_bits(tick)
}

/// Absolute output time for a delay measured from `current_time`.
///
/// A negative delay is legitimate: a code model interpolates an input crossing
/// inside the accepted analog step and dates its output from that crossing,
/// which is behind the timepoint being evaluated. The result is still refused
/// if it lands before the start of the analysis.
fn scheduled_output_time(current_time: Value, delay: Value) -> Option<Value> {
    if !current_time.is_finite() || !delay.is_finite() {
        return None;
    }
    let event_time = current_time + delay;
    if !event_time.is_finite() || event_time < 0.0 {
        return None;
    }
    Some(event_time)
}

//=============================================================================
// Scheduler
//=============================================================================

/// The event queue of one circuit.
///
/// Ordering, supersession and the settling diagnostics all belong to the
/// kernel; what lives here is the seconds/tick conversion and the shape the
/// XSPICE call sites expect.
#[derive(Debug, Clone)]
pub(crate) struct XspiceEventScheduler {
    inner: EventScheduler,
}

impl Default for XspiceEventScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl XspiceEventScheduler {
    /// Create a new empty scheduler.
    pub(crate) fn new() -> Self {
        Self {
            // The resolution is inert on this path: ticks come from
            // `event_tick`, not from a decimal grid, and nothing here calls
            // the kernel's seconds conversions. It is passed because the
            // kernel takes one resolution for a whole run.
            inner: EventScheduler::new(TimeResolution::default(), SchedulerLimits::default()),
        }
    }

    /// Schedule an event at an absolute time, superseding this driver's own
    /// pending output at or after that time.
    pub(crate) fn schedule(
        &mut self,
        time: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        driver_index: usize,
        value: EventValue,
    ) {
        let Some(tick) = event_tick(time) else {
            return;
        };
        self.inner.schedule_superseding_at(
            tick,
            // Every XSPICE output is a blocking assignment: the value is
            // computed and written in the same pass. None of the deferred
            // regions has a spelling in a code model.
            SchedulerRegion::Active,
            EventTarget {
                node_id,
                port_name: port_name.into(),
                driver_index,
                instance: instance.into(),
            },
            value,
        );
    }

    /// Schedule a digital event with delay from current time.
    pub(crate) fn schedule_delayed(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        value: DigitalValue,
    ) {
        self.schedule_delayed_with_driver_index(
            current_time,
            delay,
            node_id,
            port_name,
            instance,
            0,
            value,
        );
    }

    /// Schedule a digital event with delay from current time for one driver
    /// element.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn schedule_delayed_with_driver_index(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        driver_index: usize,
        value: DigitalValue,
    ) {
        let Some(event_time) = scheduled_output_time(current_time, delay) else {
            return;
        };
        self.schedule(
            event_time,
            node_id,
            port_name,
            instance,
            driver_index,
            EventValue::Digital(value),
        );
    }

    /// Schedule a real-valued event with delay from current time.
    pub(crate) fn schedule_real_delayed(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        value: Value,
    ) {
        self.schedule_real_delayed_with_driver_index(
            current_time,
            delay,
            node_id,
            port_name,
            instance,
            0,
            value,
        );
    }

    /// Schedule a real-valued event with delay from current time for one
    /// driver element.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn schedule_real_delayed_with_driver_index(
        &mut self,
        current_time: Value,
        delay: Value,
        node_id: usize,
        port_name: impl Into<String>,
        instance: impl Into<String>,
        driver_index: usize,
        value: Value,
    ) {
        let Some(event_time) = scheduled_output_time(current_time, delay) else {
            return;
        };
        self.schedule(
            event_time,
            node_id,
            port_name,
            instance,
            driver_index,
            EventValue::Real(value),
        );
    }

    /// Time of the next pending event.
    pub(crate) fn next_event_time(&self) -> Option<Value> {
        self.inner.next_tick().map(tick_seconds)
    }

    /// Whether an event is pending at or before the given time.
    pub(crate) fn has_event_at_or_before(&self, time: Value) -> bool {
        self.inner
            .next_tick()
            .is_some_and(|tick| tick_seconds(tick) <= time)
    }

    /// Execute every event due at or before `time`, in event order.
    ///
    /// Failure means the tick did not settle; see
    /// [`XspiceEventScheduler::note_delta_cycle`].
    pub(crate) fn run_due_events<F>(
        &mut self,
        time: Value,
        mut sink: F,
    ) -> Result<(), SchedulerError>
    where
        F: FnMut(XspiceEvent),
    {
        let Some(bound) = event_tick(time) else {
            return Ok(());
        };
        self.inner.run_due_events(bound, |event, _| {
            sink(XspiceEvent {
                time: tick_seconds(event.tick),
                node_id: event.target.node_id,
                port_name: event.target.port_name,
                driver_index: event.target.driver_index,
                instance: event.target.instance,
                value: event.value,
            })
        })?;
        Ok(())
    }

    /// Mark one iteration of the settle loop at `time`.
    ///
    /// Delta settling is unbounded, so this is what turns a zero-delay loop
    /// into an [`SchedulerError::Oscillation`] naming its busiest drivers
    /// rather than a hang.
    pub(crate) fn note_delta_cycle(&mut self, time: Value) -> Result<(), SchedulerError> {
        let Some(bound) = event_tick(time) else {
            return Ok(());
        };
        self.inner.note_delta_cycle(bound)
    }

    /// Number of pending events.
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.inner.pending()
    }

    /// Whether any event is pending.
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.pending() == 0
    }
}

//=============================================================================
// The event world, shared with rollback snapshots until something writes
//=============================================================================

/// Identity of one output driver: instance, port, and vector element.
pub(crate) type XspiceDriverId = (String, String, usize);
/// Per-node digital drive state, one entry per driver of the node.
pub(crate) type XspiceDigitalDrivers = HashMap<NodeId, HashMap<XspiceDriverId, DigitalValue>>;
/// Per-node real-valued drive state, one entry per driver of the node.
pub(crate) type XspiceRealDrivers = HashMap<NodeId, HashMap<XspiceDriverId, Value>>;

/// What every event-driven net of one circuit currently carries.
///
/// Six maps rather than six fields on `CircuitData` because they are written
/// as a unit and rolled back as a unit: the drain resolves a node's drivers
/// into its value and stamps the event time in the same step, so a snapshot
/// holding any one of them without the others would describe a state the
/// drain never produces. Grouping them is also what lets the whole set ride
/// behind a single [`Arc`] — see [`SharedXspiceEventValues`].
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct XspiceEventValues {
    /// Resolved digital value of each event-driven node.
    pub(crate) digital_values: HashMap<NodeId, DigitalValue>,
    /// Per-output digital driver values, resolved onto digital nodes.
    pub(crate) digital_drivers: XspiceDigitalDrivers,
    /// Last event time per digital node.
    pub(crate) digital_event_times: HashMap<NodeId, Value>,
    /// Resolved real-valued value of each event-driven node.
    pub(crate) real_values: HashMap<NodeId, Value>,
    /// Per-output real-valued drivers, summed onto real nodes.
    pub(crate) real_drivers: XspiceRealDrivers,
    /// Last event time per real-valued node.
    pub(crate) real_event_times: HashMap<NodeId, Value>,
}

/// A circuit's handle on its resolved event-node state, shared with rollback
/// snapshots until something writes through it.
///
/// Rollback capture used to deep-copy all six maps at every attempted timestep
/// and merit checkpoint. On a gate-level design the driver maps are the
/// expensive half — a `HashMap` per node of `(instance, port, element)` keys,
/// so two `String`s per driver — and a run copied them whether or not any
/// event had fired. Sharing them behind an [`Arc`] turns that capture into a
/// reference-count bump and defers the copy to the first write after it.
///
/// The rollback image this produces is the image the deep copy produced.
/// [`Arc::make_mut`] copies whenever the pointer is shared, so a snapshot that
/// aliases the maps observes every subsequent write on a fresh allocation and
/// never on its own; and writing is the only way to reach that path, because
/// [`Self::make_mut`] is the only mutable view. `DerefMut` is deliberately not
/// implemented, so every mutation site is spelled out.
#[derive(Clone, Default)]
pub(crate) struct SharedXspiceEventValues(Arc<XspiceEventValues>);

impl SharedXspiceEventValues {
    /// Take a mutable view, copying the maps first if a snapshot still shares
    /// them.
    ///
    /// Callers on a per-step path must ask a cheap shared-borrow predicate
    /// whether the write would change anything before calling this — a write
    /// that stores what is already there still costs a copy. The drain's
    /// predicate is [`XspiceEventScheduler::has_event_at_or_before`].
    ///
    /// Each copy this actually takes is counted, so
    /// `engine::xspice_settle_ratchet` can fail a change that puts the deep
    /// copies back.
    #[inline]
    pub(crate) fn make_mut(&mut self) -> &mut XspiceEventValues {
        let (values, copied) = crate::xspice::settle_cost::make_mut_reporting_copy(&mut self.0);
        if copied {
            crate::xspice::settle_cost::note_event_values_deep_copy();
        }
        values
    }
}

impl std::ops::Deref for SharedXspiceEventValues {
    type Target = XspiceEventValues;

    #[inline]
    fn deref(&self) -> &XspiceEventValues {
        &self.0
    }
}

impl std::fmt::Debug for SharedXspiceEventValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&*self.0, f)
    }
}

/// A circuit's handle on its event queue, shared with rollback snapshots until
/// something writes through it.
///
/// The scheduler is the mechanism D5 clause 1 is enforced by: a rejected step
/// rolls the event world back completely because the queue rides inside
/// `NonlinearDeviceStateSnapshot`. Sharing it changes nothing about that
/// image. `EventScheduler`'s derived `Clone` is a deep copy of a future tier,
/// four slot queues, a per-driver supersede index and the activation counts,
/// and the settling of one accepted timepoint left all of it unchanged for
/// every rejected trial that followed; the copy now happens on the first
/// write instead of at every capture.
///
/// As with [`SharedXspiceEventValues`], [`Self::make_mut`] is the only mutable
/// view and there is no `DerefMut`, so a snapshot can never observe a write
/// through a handle it aliases.
#[derive(Clone)]
pub(crate) struct SharedXspiceEventQueue(Arc<XspiceEventScheduler>);

impl SharedXspiceEventQueue {
    /// Create a circuit's empty event queue.
    pub(crate) fn new() -> Self {
        Self(Arc::new(XspiceEventScheduler::new()))
    }

    /// Take a mutable view, copying the scheduler first if a snapshot still
    /// shares it.
    ///
    /// Per-step callers must gate this on a shared-borrow predicate:
    /// [`XspiceEventScheduler::has_event_at_or_before`] for the drain, and
    /// `XspiceInstance::has_pending_events` for the scheduling sweep. Both
    /// answer no on a quiet analog step, which is when the sharing pays.
    ///
    /// Each copy this actually takes is counted, so
    /// `engine::xspice_settle_ratchet` can fail a change that puts the deep
    /// copies back.
    #[inline]
    pub(crate) fn make_mut(&mut self) -> &mut XspiceEventScheduler {
        let (queue, copied) = crate::xspice::settle_cost::make_mut_reporting_copy(&mut self.0);
        if copied {
            crate::xspice::settle_cost::note_event_queue_deep_copy();
        }
        queue
    }
}

impl Default for SharedXspiceEventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for SharedXspiceEventQueue {
    type Target = XspiceEventScheduler;

    #[inline]
    fn deref(&self) -> &XspiceEventScheduler {
        &self.0
    }
}

impl std::fmt::Debug for SharedXspiceEventQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&*self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain(scheduler: &mut XspiceEventScheduler, time: Value) -> Vec<XspiceEvent> {
        let mut drained = Vec::new();
        scheduler
            .run_due_events(time, |event| drained.push(event))
            .expect("a queue nothing feeds back into settles");
        drained
    }

    #[test]
    fn scheduler_ignores_negative_absolute_output_times() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule(
            -1.0e-12,
            1,
            "out",
            "a_negative_absolute_event_time",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        scheduler.schedule_delayed(
            1.0e-12,
            -1.0e-9,
            1,
            "out",
            "a_negative_digital_event_time",
            DigitalValue::one(),
        );
        scheduler.schedule_real_delayed(
            1.0e-12,
            -1.0e-9,
            2,
            "out",
            "a_negative_real_event_time",
            1.0,
        );

        assert!(
            scheduler.is_empty(),
            "event outputs before the start of transient analysis must be ignored"
        );
    }

    #[test]
    fn scheduler_preserves_valid_rebased_delayed_output_times() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule_delayed(
            10.0e-9,
            -2.0e-9,
            1,
            "out",
            "a_late_digital_output",
            DigitalValue::one(),
        );
        scheduler.schedule_real_delayed(
            10.0e-9,
            -1.0e-9,
            2,
            "real_out",
            "a_late_real_output",
            1.25,
        );

        assert_eq!(
            scheduler.next_event_time(),
            Some(8.0e-9),
            "valid rebased output events should keep their absolute target time"
        );
        let events = drain(&mut scheduler, 10.0e-9);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].time, 8.0e-9);
        assert_eq!(events[1].time, 9.0e-9);
    }

    #[test]
    fn event_times_survive_the_tick_round_trip_exactly() {
        // The times a transient run actually produces are not on any decimal
        // grid; quantizing them would move the breakpoints they generate.
        let mut scheduler = XspiceEventScheduler::new();
        let awkward = [
            0.0,
            1.0e-18,
            1.0 / 3.0e9,
            2.718_281_828_459_045e-9,
            7.234_567_890_123_456e-4,
            123.456_789,
        ];
        for (index, time) in awkward.iter().enumerate() {
            scheduler.schedule(
                *time,
                index + 1,
                "out",
                "driver",
                index,
                EventValue::Real(*time),
            );
        }

        let times: Vec<Value> = drain(&mut scheduler, 1.0e9)
            .into_iter()
            .map(|event| event.time)
            .collect();
        assert_eq!(times, awkward.to_vec());
    }

    #[test]
    fn a_later_output_replaces_this_drivers_pending_output() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule(
            0.5e-9,
            1,
            "out",
            "a_driver",
            0,
            EventValue::Digital(DigitalValue::zero()),
        );
        scheduler.schedule(
            2.0e-9,
            1,
            "out",
            "a_driver",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        scheduler.schedule(
            2.0e-9,
            1,
            "other",
            "a_driver",
            0,
            EventValue::Digital(DigitalValue::unknown()),
        );
        scheduler.schedule(
            1.0e-9,
            1,
            "out",
            "a_driver",
            0,
            EventValue::Digital(DigitalValue::unknown()),
        );

        assert_eq!(scheduler.len(), 3);
        let values: Vec<_> = drain(&mut scheduler, 2.0e-9)
            .iter()
            .map(|event| event.value)
            .collect();
        assert!(values.contains(&EventValue::Digital(DigitalValue::zero())));
        assert!(values.contains(&EventValue::Digital(DigitalValue::unknown())));
        assert!(!values.contains(&EventValue::Digital(DigitalValue::one())));
    }

    #[test]
    fn distinct_vector_driver_elements_on_one_node_are_distinct_drivers() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule(
            3.0e-9,
            1,
            "out",
            "vector_driver",
            0,
            EventValue::Digital(DigitalValue::zero()),
        );
        scheduler.schedule(
            2.0e-9,
            1,
            "out",
            "vector_driver",
            1,
            EventValue::Digital(DigitalValue::one()),
        );
        scheduler.schedule(
            1.0e-9,
            1,
            "out",
            "vector_driver",
            0,
            EventValue::Digital(DigitalValue::unknown()),
        );

        assert_eq!(scheduler.len(), 2);
        let driver_values: Vec<_> = drain(&mut scheduler, 3.0e-9)
            .iter()
            .map(|event| (event.driver_index, event.value))
            .collect();
        assert!(driver_values.contains(&(0, EventValue::Digital(DigitalValue::unknown()))));
        assert!(driver_values.contains(&(1, EventValue::Digital(DigitalValue::one()))));
        assert!(!driver_values.contains(&(0, EventValue::Digital(DigitalValue::zero()))));
    }

    #[test]
    fn delayed_digital_event_can_target_earlier_time_in_current_step() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule_delayed(4.0e-9, -1.5e-9, 1, "out", "driver", DigitalValue::one());

        let events = drain(&mut scheduler, 4.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].time, 2.5e-9);
    }

    #[test]
    fn delayed_real_event_can_target_earlier_time_in_current_step() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule_real_delayed(4.0e-9, -1.5e-9, 1, "out", "driver", 2.0);

        let events = drain(&mut scheduler, 4.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].time, 2.5e-9);
        assert_eq!(events[0].value, EventValue::Real(2.0));
    }

    #[test]
    fn an_output_dated_before_a_settled_timepoint_is_still_delivered() {
        // The interpolated-crossing case: after the drain at 4 ns, a model
        // schedules an output dated 3 ns. The old queue had no horizon and
        // delivered it; the kernel's due slot has to as well.
        let mut scheduler = XspiceEventScheduler::new();
        scheduler.schedule(
            4.0e-9,
            1,
            "out",
            "driver",
            0,
            EventValue::Digital(DigitalValue::one()),
        );
        assert_eq!(drain(&mut scheduler, 4.0e-9).len(), 1);

        scheduler.schedule(
            3.0e-9,
            2,
            "out",
            "late_driver",
            0,
            EventValue::Digital(DigitalValue::zero()),
        );
        let events = drain(&mut scheduler, 4.0e-9);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].time, 3.0e-9);
    }

    #[test]
    fn only_due_events_drain_and_the_rest_stay_pending() {
        let mut scheduler = XspiceEventScheduler::new();

        scheduler.schedule(
            0.5e-9,
            1,
            "out",
            "a_driver",
            0,
            EventValue::Digital(DigitalValue::zero()),
        );
        scheduler.schedule(2.0e-9, 2, "out", "a_driver", 0, EventValue::Real(1.25));

        let drained: Vec<_> = drain(&mut scheduler, 1.0e-9)
            .into_iter()
            .map(|event| event.value)
            .collect();

        assert_eq!(drained, vec![EventValue::Digital(DigitalValue::zero())]);
        assert_eq!(scheduler.len(), 1);
        assert_eq!(scheduler.next_event_time(), Some(2.0e-9));
    }

    #[test]
    fn a_settle_loop_that_never_quiets_is_diagnosed() {
        let mut scheduler = XspiceEventScheduler::new();
        let mut reported = None;
        for cycle in 0..20_000u32 {
            scheduler.schedule(
                1.0e-9,
                1,
                "q",
                "an_oscillator",
                0,
                EventValue::Digital(if cycle % 2 == 0 {
                    DigitalValue::zero()
                } else {
                    DigitalValue::one()
                }),
            );
            scheduler
                .run_due_events(1.0e-9, |_| {})
                .expect("the drain itself settles");
            if let Err(error) = scheduler.note_delta_cycle(1.0e-9) {
                reported = Some(error);
                break;
            }
        }

        let Some(SchedulerError::Oscillation(diagnostic)) = reported else {
            panic!("a network that never quiets must be diagnosed, got {reported:?}");
        };
        assert_eq!(
            diagnostic.tick,
            event_tick(1.0e-9).expect("a schedulable time")
        );
        assert_eq!(diagnostic.entities[0].0.instance, "an_oscillator");
    }
}
