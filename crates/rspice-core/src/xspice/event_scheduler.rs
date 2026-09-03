//! Discrete-event scheduler kernel for the digital and mixed-signal substrate.
//!
//! This module owns event ordering and time-slot execution. It schedules and
//! orders events; it does not execute them, because what an event *means*
//! belongs to the code model or the digital process that owns it. Callers
//! supply behaviour through the closure [`EventScheduler::run_time_slot`] or
//! [`EventScheduler::run_due_events`] takes.
//!
//! # Two ways to run a slot
//!
//! [`EventScheduler::run_time_slot`] is for an event world that drives itself:
//! it advances to the next tick that has an event and settles it. That is what
//! a native digital engine wants.
//!
//! [`EventScheduler::run_due_events`] is for an event world an analog loop
//! drives, which is where XSPICE sits: the analog engine names the timepoint
//! being settled and everything dated at or before it is due, including events
//! dated before it. Settling is the outer loop's, marked one iteration at a
//! time with [`EventScheduler::note_delta_cycle`], and that is where a network
//! that will not quiet becomes [`SchedulerError::Oscillation`] rather than a
//! hang. Synchronizing the two directions of time — analog steps stopping on
//! event ticks — is a separate change.
//!
//! # Time base
//!
//! Event time is an unsigned integer tick count. The kernel takes one
//! resolution for the whole run, fixed at construction: a Verilog design
//! declares `timescale`/`timeprecision` per module, and the resolution that
//! serves all of them is the finest one, which [`TimeResolution::finest`]
//! computes. Decimal resolutions make that a minimum of exponents rather than a
//! general GCD.
//!
//! The analog spine stays `f64` seconds and is not retrofitted. The two meet at
//! [`TimeResolution::ticks_to_seconds`], which is what feeds analog breakpoints,
//! so the conversion has to be exactly invertible rather than merely close.
//! It is, up to [`TimeResolution::MAX_EXACT_TICKS`] — see that constant for why
//! the bound is `2^51 - 1` and not `2^53`.
//!
//! A caller whose event times are not on a declared grid supplies its own
//! monotone tick encoding instead; the XSPICE path does, because its times
//! come from code models and the analog step controller and feed transient
//! breakpoints unrounded. See `super::event`.
//!
//! # Ordering
//!
//! Same-time ordering is a total order, not a convention: every event carries a
//! sequence number unique within its scheduler, and events sort by
//!
//! ```text
//! (tick, region, sequence)
//! ```
//!
//! No two events compare equal, so the order does not depend on queue
//! internals, iteration order of a hash container, or how many events happen to
//! share a tick. IEEE 1364-2005 permits a simulator to execute active events in
//! any order; fixing that order is what makes a run reproducible.
//!
//! The sequence counter is per-scheduler. The queue this replaced tie-broke on
//! a process-global atomic, which ordered two schedulers against each other and
//! made a run's ordering depend on what else in the process had scheduled an
//! event first. That is not reproducible, and this kernel does not inherit it.

use super::EventValue;
use std::collections::BTreeMap;
use std::fmt;
use std::mem;

/// Scheduling regions of one time slot, in execution order.
///
/// IEEE 1364-2005 §11 stratifies a time slot so that a nonblocking assignment
/// reads the values its right-hand sides had before any of them updated, and so
/// that `$monitor` observes a settled slot rather than an intermediate one.
///
/// The variants are declared in execution order and the derived [`Ord`] is that
/// order. [`SchedulerRegion::ORDERED`] is the single list the promotion loop
/// walks, so admitting the Verilog-AMS analog-interleave region later is a new
/// variant in the right position plus its entry here — no change to the loop
/// and no change to the key type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SchedulerRegion {
    /// Blocking assignments, evaluation, and `#0`-free process continuation.
    Active,
    /// Explicitly deferred to after the active region drains (`#0`).
    Inactive,
    /// Nonblocking assignment updates. Their right-hand sides were evaluated in
    /// an earlier region; only the update lands here, so it becomes visible in
    /// the next delta cycle rather than the one that computed it.
    NonBlockingAssign,
    /// Passive observation of a settled slot. Runs last and, by convention,
    /// schedules nothing back into the same tick.
    Monitor,
}

impl SchedulerRegion {
    /// Every region, in execution order.
    pub const ORDERED: &[SchedulerRegion] = &[
        SchedulerRegion::Active,
        SchedulerRegion::Inactive,
        SchedulerRegion::NonBlockingAssign,
        SchedulerRegion::Monitor,
    ];

    fn index(self) -> usize {
        match self {
            SchedulerRegion::Active => 0,
            SchedulerRegion::Inactive => 1,
            SchedulerRegion::NonBlockingAssign => 2,
            SchedulerRegion::Monitor => 3,
        }
    }
}

/// Number of regions in one time slot.
const REGION_COUNT: usize = 4;

/// Seconds per tick, indexed by the negated decimal exponent.
const SECONDS_PER_TICK: [f64; 22] = [
    1e0, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13, 1e-14,
    1e-15, 1e-16, 1e-17, 1e-18, 1e-19, 1e-20, 1e-21,
];

/// Tick resolution of a scheduler: one tick is `10^exponent` seconds.
///
/// Verilog spells precision as a decimal power of ten between 1 s and 1 fs, so
/// the exponent is the whole state a resolution needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeResolution {
    exponent: i8,
}

impl Default for TimeResolution {
    /// 1 fs, the finest precision `timescale` can declare.
    fn default() -> Self {
        Self { exponent: -15 }
    }
}

impl TimeResolution {
    /// Largest tick count that survives [`Self::ticks_to_seconds`] followed by
    /// [`Self::seconds_to_ticks`] unchanged.
    ///
    /// Integers are exact in `f64` up to `2^53`, but exactness of the *image*
    /// is not exactness of the *round trip*: no decimal resolution is a binary
    /// power, so both the multiply and the divide round. Each contributes at
    /// most a half-ulp, giving `|round_trip(t) - t| <= t * 2^-52`, which stays
    /// below the half-tick that rounding can absorb exactly when `t < 2^51`.
    ///
    /// Measured behaviour matches: across resolutions from 1 s to 1e-21 s, no
    /// tick below `2^51` fails to round trip, and the first failures appear
    /// near `2^51.8`. At 1 fs this bound is 2.25 s of event time, and a run
    /// needing longer declares a coarser precision.
    pub const MAX_EXACT_TICKS: u64 = (1 << 51) - 1;

    /// Build a resolution of `10^exponent` seconds.
    ///
    /// The range is wider than `timescale` allows so that a kernel resolution
    /// refined below the finest declared precision is still representable.
    pub fn new(exponent: i8) -> Result<Self, SchedulerError> {
        if !(-21..=0).contains(&exponent) {
            return Err(SchedulerError::UnsupportedResolution { exponent });
        }
        Ok(Self { exponent })
    }

    /// The resolution that can represent every tick of both inputs.
    ///
    /// For decimal resolutions the GCD of the two step sizes is the finer of
    /// them, so elaboration folds a design's module precisions through this.
    pub fn finest(self, other: Self) -> Self {
        Self {
            exponent: self.exponent.min(other.exponent),
        }
    }

    /// Seconds in one tick.
    ///
    /// Read from a table of decimal literals rather than computed with
    /// `powi`, which is repeated multiplication and need not land on the
    /// correctly-rounded power of ten. The rest of the crate writes these
    /// scales as literals, and a one-ulp disagreement between the two
    /// spellings would put event ticks and analog breakpoints on different
    /// grids.
    pub fn seconds_per_tick(self) -> f64 {
        SECONDS_PER_TICK[(-self.exponent) as usize]
    }

    /// Convert a tick count to analog seconds.
    ///
    /// This is the conversion analog breakpoints are placed with, so a tick
    /// past [`Self::MAX_EXACT_TICKS`] is refused rather than rounded: a
    /// breakpoint that does not land where the event is scheduled is a
    /// synchronization fault, not an accuracy loss.
    pub fn ticks_to_seconds(self, ticks: u64) -> Result<f64, SchedulerError> {
        if ticks > Self::MAX_EXACT_TICKS {
            return Err(SchedulerError::TickNotExactlyRepresentable { ticks });
        }
        Ok((ticks as f64) * self.seconds_per_tick())
    }

    /// Convert analog seconds to the nearest tick.
    ///
    /// Rejects non-finite and negative inputs, and any value whose tick image
    /// lies past [`Self::MAX_EXACT_TICKS`].
    pub fn seconds_to_ticks(self, seconds: f64) -> Result<u64, SchedulerError> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        let ticks = (seconds / self.seconds_per_tick()).round();
        if !ticks.is_finite() || ticks < 0.0 || ticks > Self::MAX_EXACT_TICKS as f64 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        Ok(ticks as u64)
    }

    /// Convert analog seconds to the tick *at or before* them.
    ///
    /// This is the conversion a mixed-signal interleave *advances the digital
    /// world* with, and it is a separate method rather than a mode of
    /// [`Self::seconds_to_ticks`] because the two answer different questions
    /// and both callers are right. Rounding to nearest is what an event
    /// *scheduled* in seconds wants: the tick closest to the instant asked
    /// for. Flooring is what an analog timepoint being *delivered* to the
    /// digital world wants, and the reason is that a `.tran` step controlled
    /// by local truncation error does not land on the grid at all:
    ///
    /// * flooring is monotone, so a non-decreasing sequence of accepted analog
    ///   times gives a non-decreasing sequence of ticks;
    /// * it never runs the digital world past an instant the integrator has
    ///   accepted, which rounding up to the nearest tick would;
    /// * two analog times inside one tick collapse rather than reorder, which
    ///   is what a declared precision means.
    ///
    /// The exact time is not lost by this — it is carried alongside, in the
    /// unquantized `f64` the analog side already has, which is what keeps a
    /// breakpoint bit-exact. See the module documentation of
    /// [`crate::xspice::verilog`] for the ruling this implements.
    ///
    /// The same interleave is also the caller of [`Self::seconds_to_ticks`],
    /// for its *other* mapping: an A/D transition's own timestamp names the
    /// tick its event lands on, which Verilog-AMS LRM 2.4 section 7.3.6.1
    /// fixes at the nearest tick. Neither call site may be rewritten into the
    /// other — they quantize different quantities for different reasons.
    ///
    /// # Exactness
    ///
    /// The answer is defined as the largest `t` with
    /// `ticks_to_seconds(t) <= seconds`, and it is computed against that same
    /// product rather than against the division alone. A single division is
    /// off by up to one ulp, which for a time that sits exactly on a tick can
    /// land just under the integer and floor to the tick *before* it — the one
    /// error this conversion must not make, because an event time handed back
    /// as a breakpoint would then be delivered a tick late. Correcting against
    /// the multiplication makes `seconds_to_floor_ticks(ticks_to_seconds(t))`
    /// exactly `t` for every representable `t`.
    ///
    /// Crate-visible rather than public: the mixed interleave is the only
    /// caller, and the rest of this type is published because a caller outside
    /// the crate actually reaches for it. This becomes `pub` when one does.
    ///
    /// Gated with that caller too. `xspice::verilog` is a `veriloga` module, so
    /// a build without the feature has no caller at all and `-D warnings` says
    /// so; the gate is what keeps the default build's warning budget honest
    /// rather than an `allow` that would also hide a real orphan later.
    #[cfg(feature = "veriloga")]
    pub(crate) fn seconds_to_floor_ticks(self, seconds: f64) -> Result<u64, SchedulerError> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        let scale = self.seconds_per_tick();
        let estimate = (seconds / scale).floor();
        if !estimate.is_finite() || estimate < 0.0 || estimate > Self::MAX_EXACT_TICKS as f64 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        // The division is accurate to within one ulp of the quotient, so each
        // correction runs at most once; they are loops rather than single
        // steps so the postcondition holds by construction instead of by an
        // argument about rounding.
        let mut ticks = estimate as u64;
        while ticks > 0 && (ticks as f64) * scale > seconds {
            ticks -= 1;
        }
        while ticks < Self::MAX_EXACT_TICKS && ((ticks + 1) as f64) * scale <= seconds {
            ticks += 1;
        }
        Ok(ticks)
    }
}

/// The driver an event updates.
///
/// These are the fields the XSPICE drain moves today — node, port, vector
/// element, and the instance that scheduled it — so a code model's event
/// crosses into this kernel without an adapter. Two events are the same driver
/// exactly when all four agree, which is what lets a later output from one
/// driver supersede an earlier one without disturbing a co-driver on the same
/// node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTarget {
    /// Circuit node identifier the driver writes.
    pub node_id: usize,
    /// Port name on the owning instance.
    pub port_name: String,
    /// Element index within a vector port.
    pub driver_index: usize,
    /// Instance that scheduled the event.
    pub instance: String,
}

/// One scheduled event, carrying its position in the total order.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledEvent {
    /// Tick the event is scheduled for.
    pub tick: u64,
    /// Region of that tick's slot.
    pub region: SchedulerRegion,
    /// Per-scheduler sequence number; unique, and the final tie-break.
    pub sequence: u64,
    /// Driver being updated.
    pub target: EventTarget,
    /// Value written to the driver.
    pub value: EventValue,
}

/// What stopped a tick from settling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OscillationCause {
    /// Regions kept promoting without the tick going quiet.
    DeltaCycleLimit,
    /// The active region kept refilling itself, so no promotion ever happened.
    /// A zero-delay loop inside one delta cycle looks like this.
    EventLimit,
}

/// Why a tick failed to settle, and which drivers were responsible.
///
/// Delta-cycle settling is unbounded in the standard, which means a zero-delay
/// loop is a hang rather than a diagnosis. The limits exist to convert that
/// hang into this: the drivers that fired most often at the offending tick, in
/// descending activation order, which is the evidence needed to name the loop.
#[derive(Debug, Clone, PartialEq)]
pub struct OscillationDiagnostic {
    /// Tick that failed to settle.
    pub tick: u64,
    /// Which limit tripped.
    pub cause: OscillationCause,
    /// Delta cycles completed at `tick` before the abort.
    pub delta_cycles: u32,
    /// Events executed at `tick` before the abort.
    pub events_executed: u64,
    /// The configured delta-cycle ceiling.
    pub delta_cycle_limit: u32,
    /// The configured per-tick event ceiling.
    pub event_limit: u64,
    /// Busiest drivers at `tick`, paired with their activation counts and
    /// ordered by count descending, then by the driver's own order so the
    /// report is reproducible.
    pub entities: Vec<(EventTarget, u64)>,
}

/// Errors the kernel reports instead of panicking or hanging.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerError {
    /// A tick past the exactly-invertible range reached a seconds conversion.
    TickNotExactlyRepresentable {
        /// The offending tick count.
        ticks: u64,
    },
    /// A seconds value was non-finite, negative, or past the exact range.
    SecondsNotRepresentable {
        /// The offending seconds value.
        seconds: f64,
    },
    /// A resolution exponent outside the supported decimal range.
    UnsupportedResolution {
        /// The offending exponent.
        exponent: i8,
    },
    /// An event was scheduled at a tick the scheduler has already left.
    ScheduleInThePast {
        /// Tick the scheduler has reached.
        current_tick: u64,
        /// Tick the caller asked for.
        requested_tick: u64,
    },
    /// A tick did not settle within its limits.
    Oscillation(OscillationDiagnostic),
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchedulerError::TickNotExactlyRepresentable { ticks } => write!(
                f,
                "event tick {ticks} exceeds {} and has no exact seconds image",
                TimeResolution::MAX_EXACT_TICKS
            ),
            SchedulerError::SecondsNotRepresentable { seconds } => {
                write!(f, "time {seconds:e} s has no representable event tick")
            }
            SchedulerError::UnsupportedResolution { exponent } => {
                write!(f, "unsupported time resolution 1e{exponent} s")
            }
            SchedulerError::ScheduleInThePast {
                current_tick,
                requested_tick,
            } => write!(
                f,
                "event scheduled at tick {requested_tick}, but the scheduler has reached tick {current_tick}"
            ),
            SchedulerError::Oscillation(diagnostic) => {
                let cause = match diagnostic.cause {
                    OscillationCause::DeltaCycleLimit => {
                        format!("{} delta cycles", diagnostic.delta_cycle_limit)
                    }
                    OscillationCause::EventLimit => {
                        format!("{} events", diagnostic.event_limit)
                    }
                };
                write!(
                    f,
                    "event network did not settle at tick {} within {cause}",
                    diagnostic.tick
                )?;
                if let Some((target, count)) = diagnostic.entities.first() {
                    write!(
                        f,
                        " (busiest driver {}.{} fired {count} times)",
                        target.instance, target.port_name
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Ceilings that turn a non-settling tick into a diagnosis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulerLimits {
    /// Region promotions allowed at one tick before the tick is declared
    /// oscillating.
    pub max_delta_cycles_per_tick: u32,
    /// Events executed at one tick before the tick is declared oscillating.
    /// This is the guard a self-refilling active region trips, since such a
    /// loop never promotes and so never advances the delta count.
    pub max_events_per_tick: u64,
    /// Cap on the driver list carried by an [`OscillationDiagnostic`].
    pub max_reported_oscillating_entities: usize,
}

impl Default for SchedulerLimits {
    fn default() -> Self {
        Self {
            max_delta_cycles_per_tick: 10_000,
            max_events_per_tick: 1_000_000,
            max_reported_oscillating_entities: 16,
        }
    }
}

/// What one settled tick did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSlotReport {
    /// The tick that ran.
    pub tick: u64,
    /// Delta cycles it took to settle.
    pub delta_cycles: u32,
    /// Events executed at that tick.
    pub events_executed: u64,
}

/// Scheduling state, split out so an executing event can schedule through
/// [`SchedulerContext`] while the driver loop owns the event it popped.
#[derive(Debug, Clone, Default)]
struct EventQueues {
    /// Events at ticks after the current one, keyed by the full total order so
    /// the key type *is* the ordering specification.
    future: BTreeMap<(u64, SchedulerRegion, u64), ScheduledEvent>,
    /// The current tick's slot, one queue per region, each ordered by sequence.
    slot: [BTreeMap<u64, ScheduledEvent>; REGION_COUNT],
    /// Next sequence number. Per-scheduler, never reused.
    next_sequence: u64,
    /// Activation counts at the current tick, for the oscillation report.
    activations: BTreeMap<EventTarget, u64>,
    /// Where each driver's unexecuted events sit, by tick and then in schedule
    /// order. This is the index [`EventScheduler::schedule_superseding_at`]
    /// consults; without it, superseding a driver's own pending output would
    /// mean scanning the whole queue.
    driver_events: BTreeMap<EventTarget, BTreeMap<u64, Vec<(SchedulerRegion, u64)>>>,
}

impl EventQueues {
    /// Place an event, routing it to the current slot or the future tier.
    ///
    /// `open_slot` is the tick whose slot is running, if one is; an event at
    /// that same tick joins it as a delta event. It is an `Option` rather than
    /// a sentinel tick because `u64::MAX` is a schedulable tick, and a sentinel
    /// would route an event scheduled there into whatever slot was open.
    fn insert(
        &mut self,
        open_slot: Option<u64>,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.driver_events
            .entry(target.clone())
            .or_default()
            .entry(tick)
            .or_default()
            .push((region, sequence));
        let event = ScheduledEvent {
            tick,
            region,
            sequence,
            target,
            value,
        };
        if open_slot == Some(tick) {
            self.slot[region.index()].insert(sequence, event);
        } else {
            self.future.insert((tick, region, sequence), event);
        }
        sequence
    }

    fn slot_is_empty(&self) -> bool {
        self.slot.iter().all(BTreeMap::is_empty)
    }

    /// Earliest tick any event still sitting in the slot is dated at.
    ///
    /// The slot is keyed by sequence, not by tick, because within one tick
    /// sequence is the whole tie-break after the region — so the tick has to
    /// be read off the events themselves. That scan is why the empty case
    /// returns first: an empty slot is the steady state between
    /// [`EventScheduler::run_due_events`] calls, which is where the hot
    /// predicates ask, and it must stay as cheap as the region-emptiness
    /// check it already was.
    ///
    /// A non-empty slot is only observable from inside a due-slot run or after
    /// one returned an oscillation, and neither is a per-evaluation path.
    fn slot_min_tick(&self) -> Option<u64> {
        if self.slot_is_empty() {
            return None;
        }
        self.slot
            .iter()
            .flat_map(BTreeMap::values)
            .map(|event| event.tick)
            .min()
    }

    /// Take the lowest-sequence active event.
    fn pop_active(&mut self) -> Option<ScheduledEvent> {
        let event = self.slot[SchedulerRegion::Active.index()]
            .pop_first()
            .map(|(_, event)| event)?;
        self.forget_driver_event(&event.target, event.tick, event.sequence);
        Some(event)
    }

    /// Drop one event from the driver index, pruning the empty levels above it.
    fn forget_driver_event(&mut self, target: &EventTarget, tick: u64, sequence: u64) {
        let Some(ticks) = self.driver_events.get_mut(target) else {
            return;
        };
        if let Some(entries) = ticks.get_mut(&tick) {
            entries.retain(|(_, pending)| *pending != sequence);
            if entries.is_empty() {
                ticks.remove(&tick);
            }
        }
        if ticks.is_empty() {
            self.driver_events.remove(target);
        }
    }

    /// Remove one unexecuted event wherever it is held.
    ///
    /// A promoted event sits in the active queue while its `region` field
    /// still names the region it was scheduled into, so the slot cannot be
    /// indexed by that field; all four queues are checked instead. There are
    /// four of them, so this stays a constant number of lookups.
    fn remove_scheduled(&mut self, tick: u64, region: SchedulerRegion, sequence: u64) {
        if self.future.remove(&(tick, region, sequence)).is_some() {
            return;
        }
        for queue in self.slot.iter_mut() {
            if queue.remove(&sequence).is_some() {
                return;
            }
        }
    }

    /// Cancel every unexecuted event of `target` at or after `tick`, and
    /// report how many were cancelled.
    fn supersede_driver(&mut self, target: &EventTarget, tick: u64) -> usize {
        let Some(ticks) = self.driver_events.get_mut(target) else {
            return 0;
        };
        let superseded = ticks.split_off(&tick);
        if ticks.is_empty() {
            self.driver_events.remove(target);
        }
        let mut cancelled = 0;
        for (superseded_tick, entries) in superseded {
            for (region, sequence) in entries {
                self.remove_scheduled(superseded_tick, region, sequence);
                cancelled += 1;
            }
        }
        cancelled
    }

    /// Promote the first non-empty later region into the active region.
    ///
    /// Wholesale promotion is what makes a nonblocking update visible only in
    /// the next delta: the whole region moves at once, and anything it
    /// schedules back into the active region carries a higher sequence and so
    /// runs after every event promoted with it.
    fn promote(&mut self) -> bool {
        for region in &SchedulerRegion::ORDERED[1..] {
            let index = region.index();
            if self.slot[index].is_empty() {
                continue;
            }
            let promoted = mem::take(&mut self.slot[index]);
            self.slot[SchedulerRegion::Active.index()].extend(promoted);
            return true;
        }
        false
    }

    /// Move the earliest pending tick at or before `bound` into the slot
    /// queues, reporting whether anything moved.
    ///
    /// One tick at a time, never a range: a slot queue is keyed by sequence
    /// alone, because within one tick sequence is the whole tie-break after
    /// the region. Emptying a span of ticks into it at once would order them
    /// by creation instead of by time, so the due-slot mode opens the next
    /// tick only once the current one has gone quiet.
    fn open_next_due_tick(&mut self, bound: u64) -> bool {
        let Some(((tick, _, _), _)) = self.future.first_key_value() else {
            return false;
        };
        let tick = *tick;
        if tick > bound {
            return false;
        }
        let upper = match tick.checked_add(1) {
            Some(next) => self.future.split_off(&(next, SchedulerRegion::Active, 0)),
            None => BTreeMap::new(),
        };
        let due = mem::replace(&mut self.future, upper);
        for (_, event) in due {
            self.slot[event.region.index()].insert(event.sequence, event);
        }
        true
    }
}

/// Two-tier discrete-event scheduler over integer ticks.
///
/// One tier is the current tick's stratified slot, iterated until quiescent;
/// the other is every later tick, keyed by the total order. Advancing is
/// [`Self::run_time_slot`], which jumps straight to the next tick that has an
/// event rather than stepping through empty ones.
#[derive(Debug, Clone)]
pub struct EventScheduler {
    queues: EventQueues,
    resolution: TimeResolution,
    limits: SchedulerLimits,
    /// Tick of the slot currently running or most recently run.
    current_tick: u64,
    /// Whether any slot has run. Until one has, tick 0 is still schedulable.
    started: bool,
    /// Delta cycles counted against the open due slot. Only the due-slot mode
    /// uses this; [`Self::run_time_slot`] settles a tick inside one call and
    /// counts in a local.
    slot_delta_cycles: u32,
    /// Events executed against the open due slot, for the same reason.
    slot_events_executed: u64,
}

impl EventScheduler {
    /// Build a scheduler at the given resolution.
    pub fn new(resolution: TimeResolution, limits: SchedulerLimits) -> Self {
        Self {
            queues: EventQueues::default(),
            resolution,
            limits,
            current_tick: 0,
            started: false,
            slot_delta_cycles: 0,
            slot_events_executed: 0,
        }
    }

    /// The resolution ticks are measured in.
    pub fn resolution(&self) -> TimeResolution {
        self.resolution
    }

    /// Tick of the slot most recently run, or 0 before the first slot.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Earliest tick holding an event, if any.
    ///
    /// Both tiers are consulted and the earlier answer wins. The slot is
    /// asked what its events are *dated*, which is not the same question as
    /// which slot is open: [`Self::open_due_slot`] sets `current_tick` to the
    /// caller's bound, and `open_next_due_tick` then fills the slot from the
    /// earliest pending tick at or before that bound. So a slot observed
    /// part-settled holds events dated before `current_tick`, and answering
    /// with the bound would date them late — as a breakpoint, by however far
    /// the bound overshot.
    ///
    /// A slot is only observable part-settled from inside a due-slot run or
    /// after one returned an oscillation. Every XSPICE reader asks between
    /// runs, where a settled slot is empty in every region, so this reads the
    /// future tier for them exactly as it always did.
    pub fn next_tick(&self) -> Option<u64> {
        let slot = self.queues.slot_min_tick();
        let future = self.queues.future.keys().next().map(|(tick, _, _)| *tick);
        match (slot, future) {
            (Some(slot), Some(future)) => Some(slot.min(future)),
            (slot, future) => slot.or(future),
        }
    }

    /// Number of events not yet executed.
    pub fn pending(&self) -> usize {
        self.queues.future.len() + self.queues.slot.iter().map(BTreeMap::len).sum::<usize>()
    }

    /// Schedule an event from outside a running slot.
    ///
    /// Returns the event's sequence number, which is its place in the total
    /// order. A tick the scheduler has already left is refused: silently
    /// re-dating it would let an event appear to run before something that
    /// already ran.
    pub fn schedule_at(
        &mut self,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        let horizon = if self.started {
            self.current_tick.saturating_add(1)
        } else {
            0
        };
        if tick < horizon {
            return Err(SchedulerError::ScheduleInThePast {
                current_tick: self.current_tick,
                requested_tick: tick,
            });
        }
        // No slot is open outside `run_time_slot`, so everything lands in the
        // future tier and is picked up when its tick opens.
        Ok(self.queues.insert(None, tick, region, target, value))
    }

    /// Schedule an event that replaces this driver's own pending output.
    ///
    /// Every unexecuted event of `target` at a tick at or after `tick` is
    /// cancelled first; the returned count is how many were. A co-driver on
    /// the same node is untouched, because [`EventTarget`] identity includes
    /// the instance and the port, so two drivers of one node are two targets.
    ///
    /// This is inertial-style cancellation: an output that changes its mind
    /// before its earlier output has been delivered replaces it rather than
    /// queueing behind it. It is what the XSPICE queue has always done, and
    /// what a Verilog inertial delay does with a pulse narrower than the
    /// delay. Transport delay, which queues instead, is a separate mode this
    /// method deliberately does not offer.
    ///
    /// Unlike [`Self::schedule_at`] this takes no horizon. A code model dates
    /// its output from an input crossing it interpolated *inside* the accepted
    /// analog step, so the output can be dated before the timepoint being
    /// settled and is still due at it, not late for it. The due-slot mode
    /// ([`Self::run_due_events`]) is what delivers such an event, and refusing
    /// it here would drop output the analog path has always seen.
    ///
    /// Like [`Self::schedule_at`], the event lands in the future tier and is
    /// picked up when its tick opens, because this method is reachable only
    /// from outside a running slot: an event executing inside one schedules
    /// through [`SchedulerContext`], which does not offer supersession. Placing
    /// an event straight into the open slot from out here would put it ahead of
    /// every event the drain has not opened yet, including one dated *earlier*
    /// — the interpolated-crossing case above — and that is the one ordering
    /// [`Self::run_due_events`] promises cannot happen.
    pub fn schedule_superseding_at(
        &mut self,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> usize {
        let cancelled = self.queues.supersede_driver(&target, tick);
        self.queues.insert(None, tick, region, target, value);
        cancelled
    }

    /// Execute every event due at or before `bound_tick`.
    ///
    /// This is the mode for an event world driven by an outer loop rather than
    /// by its own clock: the analog engine names the timepoint being settled,
    /// and everything dated at or before it is due now. Events at several
    /// distinct ticks can therefore run in one call; they still run in
    /// `(tick, region, sequence)` order, so the result is the order they would
    /// have had if each tick had run its own slot.
    ///
    /// Successive calls with the same `bound_tick` continue one slot and
    /// accumulate its accounting, which is what makes the oscillation
    /// diagnostic meaningful when the settling is driven from outside: the
    /// caller marks each iteration with [`Self::note_delta_cycle`]. A
    /// different bound opens a fresh slot. The bound may move backwards — a
    /// rejected analog step retries at a smaller timepoint — and that opens a
    /// fresh slot too.
    pub fn run_due_events<F>(
        &mut self,
        bound_tick: u64,
        mut execute: F,
    ) -> Result<TimeSlotReport, SchedulerError>
    where
        F: FnMut(ScheduledEvent, &mut SchedulerContext<'_>),
    {
        self.open_due_slot(bound_tick);

        loop {
            let Some(event) = self.queues.pop_active() else {
                if self.queues.promote() {
                    self.slot_delta_cycles += 1;
                    if self.slot_delta_cycles > self.limits.max_delta_cycles_per_tick {
                        return Err(self.oscillation(
                            bound_tick,
                            OscillationCause::DeltaCycleLimit,
                            self.slot_delta_cycles,
                            self.slot_events_executed,
                        ));
                    }
                    continue;
                }
                // The current tick has gone quiet in every region, so the next
                // one under the bound may open. This is also what picks up an
                // event `execute` back-dated below the bound, which
                // `SchedulerContext` routes to the future tier.
                if self.queues.open_next_due_tick(bound_tick) {
                    continue;
                }
                break;
            };

            self.slot_events_executed += 1;
            if self.slot_events_executed > self.limits.max_events_per_tick {
                return Err(self.oscillation(
                    bound_tick,
                    OscillationCause::EventLimit,
                    self.slot_delta_cycles,
                    self.slot_events_executed,
                ));
            }
            *self
                .queues
                .activations
                .entry(event.target.clone())
                .or_insert(0) += 1;

            let mut context = SchedulerContext {
                queues: &mut self.queues,
                current_tick: bound_tick,
            };
            execute(event, &mut context);
        }

        Ok(TimeSlotReport {
            tick: bound_tick,
            delta_cycles: self.slot_delta_cycles,
            events_executed: self.slot_events_executed,
        })
    }

    /// Record one delta cycle of the due slot bounded by `bound_tick`.
    ///
    /// An outer settle loop calls this once per iteration. Delta settling is
    /// unbounded in the standard, so a zero-delay loop is a hang rather than a
    /// diagnosis; this is where that hang is converted into
    /// [`SchedulerError::Oscillation`] naming the drivers responsible.
    ///
    /// It is a separate call from [`Self::run_due_events`] because the outer
    /// loop drains the queue several times per iteration — once before it
    /// walks its processes and once after each of them — and counting a delta
    /// per drain would make the ceiling depend on how many processes the
    /// design has rather than on how deep the settling is.
    pub fn note_delta_cycle(&mut self, bound_tick: u64) -> Result<(), SchedulerError> {
        self.open_due_slot(bound_tick);
        self.slot_delta_cycles += 1;
        if self.slot_delta_cycles > self.limits.max_delta_cycles_per_tick {
            return Err(self.oscillation(
                bound_tick,
                OscillationCause::DeltaCycleLimit,
                self.slot_delta_cycles,
                self.slot_events_executed,
            ));
        }
        Ok(())
    }

    /// Open the due slot at `bound_tick`, resetting per-slot accounting when
    /// the bound moves. Continuing the same bound keeps it.
    fn open_due_slot(&mut self, bound_tick: u64) {
        if self.started && self.current_tick == bound_tick {
            return;
        }
        self.current_tick = bound_tick;
        self.started = true;
        self.slot_delta_cycles = 0;
        self.slot_events_executed = 0;
        self.queues.activations.clear();
    }

    /// Run the next tick that has events, to quiescence.
    ///
    /// Returns `Ok(None)` when nothing is pending. Otherwise the tick's slot is
    /// iterated per IEEE 1364-2005: drain the active region, promote the first
    /// non-empty later region when it empties, and stop when every region is
    /// empty. `execute` sees each event in total order and may schedule more
    /// through its [`SchedulerContext`].
    pub fn run_time_slot<F>(
        &mut self,
        mut execute: F,
    ) -> Result<Option<TimeSlotReport>, SchedulerError>
    where
        F: FnMut(ScheduledEvent, &mut SchedulerContext<'_>),
    {
        let Some(tick) = self.next_tick() else {
            return Ok(None);
        };

        self.current_tick = tick;
        self.started = true;
        self.queues.open_next_due_tick(tick);
        self.queues.activations.clear();

        // A tick settles inside this call, so the counters are local. The
        // per-slot fields are cleared so that a scheduler driven both ways
        // does not carry one mode's accounting into the other's.
        self.slot_delta_cycles = 0;
        self.slot_events_executed = 0;
        let mut delta_cycles: u32 = 0;
        let mut events_executed: u64 = 0;

        loop {
            let Some(event) = self.queues.pop_active() else {
                if !self.queues.promote() {
                    break;
                }
                delta_cycles += 1;
                if delta_cycles > self.limits.max_delta_cycles_per_tick {
                    return Err(self.oscillation(
                        tick,
                        OscillationCause::DeltaCycleLimit,
                        delta_cycles,
                        events_executed,
                    ));
                }
                continue;
            };

            events_executed += 1;
            if events_executed > self.limits.max_events_per_tick {
                return Err(self.oscillation(
                    tick,
                    OscillationCause::EventLimit,
                    delta_cycles,
                    events_executed,
                ));
            }
            *self
                .queues
                .activations
                .entry(event.target.clone())
                .or_insert(0) += 1;

            let mut context = SchedulerContext {
                queues: &mut self.queues,
                current_tick: tick,
            };
            execute(event, &mut context);
        }

        Ok(Some(TimeSlotReport {
            tick,
            delta_cycles,
            events_executed,
        }))
    }

    fn oscillation(
        &self,
        tick: u64,
        cause: OscillationCause,
        delta_cycles: u32,
        events_executed: u64,
    ) -> SchedulerError {
        // Ordered by activation count, then by the driver's own order, so two
        // runs of the same oscillation report the same list.
        let mut entities: Vec<(EventTarget, u64)> = self
            .queues
            .activations
            .iter()
            .map(|(target, count)| (target.clone(), *count))
            .collect();
        entities.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        entities.truncate(self.limits.max_reported_oscillating_entities);
        SchedulerError::Oscillation(OscillationDiagnostic {
            tick,
            cause,
            delta_cycles,
            events_executed,
            delta_cycle_limit: self.limits.max_delta_cycles_per_tick,
            event_limit: self.limits.max_events_per_tick,
            entities,
        })
    }
}

/// Scheduling handle handed to an executing event.
///
/// It exposes scheduling and nothing else, so an event cannot re-enter
/// [`EventScheduler::run_time_slot`] from inside a slot.
#[derive(Debug)]
pub struct SchedulerContext<'a> {
    queues: &'a mut EventQueues,
    current_tick: u64,
}

impl SchedulerContext<'_> {
    /// The tick whose slot is running.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Schedule at an absolute tick, which may be the running one.
    ///
    /// A tick before the running one is refused for the same reason as
    /// [`EventScheduler::schedule_at`].
    pub fn schedule_at(
        &mut self,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        if tick < self.current_tick {
            return Err(SchedulerError::ScheduleInThePast {
                current_tick: self.current_tick,
                requested_tick: tick,
            });
        }
        Ok(self
            .queues
            .insert(Some(self.current_tick), tick, region, target, value))
    }

    /// Schedule `delay` ticks from the running tick. A zero delay is a delta
    /// event at the current tick.
    pub fn schedule_after(
        &mut self,
        delay: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        let Some(tick) = self.current_tick.checked_add(delay) else {
            return Err(SchedulerError::TickNotExactlyRepresentable { ticks: u64::MAX });
        };
        self.schedule_at(tick, region, target, value)
    }
}

/// The kernel's conformance tests live in `tests/event_scheduler_kernel.rs`,
/// against the published API. These are here because
/// [`TimeResolution::seconds_to_floor_ticks`] is crate-visible and so has no
/// published API to be tested against.
///
/// Gated on `veriloga` with the method itself, which is gated with its only
/// caller: `xspice::verilog`'s mixed interleave is a `veriloga` module, so a
/// build without the feature has neither the method nor anything to test it.
#[cfg(all(test, feature = "veriloga"))]
mod tests {
    use super::*;

    #[test]
    fn flooring_maps_an_off_grid_analog_time_to_the_tick_at_or_before_it() {
        let resolution = TimeResolution::new(-9).expect("1 ns");

        // The case the round-to-nearest conversion gets wrong for a mixed
        // interleave: a time three quarters of the way through a tick belongs
        // to the tick it is inside, not to the one it is closest to. Rounding
        // here would run the digital world a quarter of a nanosecond past an
        // instant the integrator has accepted.
        assert_eq!(resolution.seconds_to_ticks(2.75e-9), Ok(3));
        assert_eq!(resolution.seconds_to_floor_ticks(2.75e-9), Ok(2));
        assert_eq!(resolution.seconds_to_floor_ticks(2.25e-9), Ok(2));
        assert_eq!(resolution.seconds_to_floor_ticks(0.0), Ok(0));
        assert_eq!(resolution.seconds_to_floor_ticks(0.999e-9), Ok(0));

        // Monotone: a non-decreasing sequence of analog times gives a
        // non-decreasing sequence of ticks, which is what lets `advance_to` be
        // driven straight from accepted timepoints.
        let mut previous = 0u64;
        let mut seconds = 0.0f64;
        while seconds < 5.0e-9 {
            let tick = resolution
                .seconds_to_floor_ticks(seconds)
                .expect("in range");
            assert!(
                tick >= previous,
                "flooring must be monotone: {seconds:e} s gave {tick} after {previous}"
            );
            previous = tick;
            seconds += 3.7e-11;
        }
    }

    /// The two mappings the mixed interleave uses, pinned against each other
    /// at the half tick, which is the only place they can disagree by a whole
    /// tick in the *upward* direction.
    ///
    /// Rounding is what an A/D transition's own timestamp gets — Verilog-AMS
    /// LRM 2.4 section 7.3.6.1 puts an analog event on the nearest digital
    /// tick — and flooring is what the trial timestamp that advances the
    /// digital world gets. A refactor that unified them would have to break
    /// one of these two columns.
    #[test]
    fn the_half_tick_rounds_up_while_the_floor_stays_put() {
        let resolution = TimeResolution::new(-9).expect("1 ns");

        // Below the half: both mappings answer with the tick the time is in.
        assert_eq!(resolution.seconds_to_ticks(2.4e-9), Ok(2));
        assert_eq!(resolution.seconds_to_floor_ticks(2.4e-9), Ok(2));

        // Exactly the half: `f64::round` is half-away-from-zero, so a tie goes
        // to the later tick. Stated rather than discovered, because the
        // direction of the tie is what decides which slot a transition landing
        // dead centre is published into.
        assert_eq!(resolution.seconds_to_ticks(2.5e-9), Ok(3));
        assert_eq!(resolution.seconds_to_floor_ticks(2.5e-9), Ok(2));

        // Above the half: rounding moves on, flooring does not.
        assert_eq!(resolution.seconds_to_ticks(2.6e-9), Ok(3));
        assert_eq!(resolution.seconds_to_floor_ticks(2.6e-9), Ok(2));

        // And the rounding never lands more than one tick from the floor, in
        // either direction, which is what bounds how far forward a transition's
        // publication can move.
        let mut seconds = 0.0f64;
        while seconds < 5.0e-9 {
            let floor = resolution
                .seconds_to_floor_ticks(seconds)
                .expect("in range");
            let nearest = resolution.seconds_to_ticks(seconds).expect("in range");
            assert!(
                nearest == floor || nearest == floor + 1,
                "{seconds:e} s floored to {floor} but rounded to {nearest}"
            );
            seconds += 3.7e-11;
        }
    }

    #[test]
    fn flooring_a_tick_boundary_returns_that_tick_and_not_the_one_before() {
        // The error a bare division would make: an event time handed back as a
        // breakpoint, floored, must be delivered at its own tick. One ulp low
        // and the digital slot runs a whole tick late.
        for exponent in [-9i8, -12, -15] {
            let resolution = TimeResolution::new(exponent).expect("declared precision");
            for tick in [0u64, 1, 2, 3, 7, 999, 1_000, 1_001, 123_456_789] {
                let seconds = resolution.ticks_to_seconds(tick).expect("in range");
                assert_eq!(
                    resolution.seconds_to_floor_ticks(seconds),
                    Ok(tick),
                    "exponent {exponent} tick {tick} round trip"
                );
            }
        }
    }

    #[test]
    fn flooring_refuses_what_rounding_refuses() {
        let resolution = TimeResolution::new(-9).expect("1 ns");
        assert!(resolution.seconds_to_floor_ticks(-1.0e-9).is_err());
        assert!(resolution.seconds_to_floor_ticks(f64::NAN).is_err());
        assert!(resolution.seconds_to_floor_ticks(f64::INFINITY).is_err());
        assert!(
            resolution
                .seconds_to_floor_ticks(TimeResolution::MAX_EXACT_TICKS as f64 * 1.0e-9 * 2.0)
                .is_err()
        );
    }
}
