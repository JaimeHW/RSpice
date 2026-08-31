//! Discrete-event scheduler kernel for the digital and mixed-signal substrate.
//!
//! The XSPICE event path drains its queue inside analog Newton assembly: event
//! time never advances on its own, and the relaxation that stands in for delta
//! cycles is bounded by a pass count whose overrun is fatal. This module is the
//! substrate that replaces it. It schedules and orders events; it does not
//! execute them, because what an event *means* belongs to the code model or the
//! digital process that owns it. Callers supply behaviour through the closure
//! [`EventScheduler::run_time_slot`] takes.
//!
//! Nothing in the analog engine calls this module yet. Rehosting XSPICE on it
//! and synchronizing it against analog time are separate changes.
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
//! The sequence counter is per-scheduler. The XSPICE queue's tie-break is a
//! process-global atomic, which orders two schedulers against each other and
//! makes a run's ordering depend on what else in the process scheduled an event
//! first. That is not reproducible, and this kernel does not inherit it.

use crate::xspice::EventValue;
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
#[derive(Debug, Default)]
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
}

impl EventQueues {
    /// Place an event, routing it to the current slot or the future tier.
    ///
    /// `current_tick` is the tick whose slot is open; equal ticks land in the
    /// slot so a delta event joins the tick already running.
    fn insert(
        &mut self,
        current_tick: u64,
        tick: u64,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        let event = ScheduledEvent {
            tick,
            region,
            sequence,
            target,
            value,
        };
        if tick == current_tick {
            self.slot[region.index()].insert(sequence, event);
        } else {
            self.future.insert((tick, region, sequence), event);
        }
        sequence
    }

    fn slot_is_empty(&self) -> bool {
        self.slot.iter().all(BTreeMap::is_empty)
    }

    /// Take the lowest-sequence active event.
    fn pop_active(&mut self) -> Option<ScheduledEvent> {
        self.slot[SchedulerRegion::Active.index()]
            .pop_first()
            .map(|(_, event)| event)
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

    /// Move every future event at `tick` into the slot queues.
    fn open_slot(&mut self, tick: u64) {
        let upper = match tick.checked_add(1) {
            Some(next) => self.future.split_off(&(next, SchedulerRegion::Active, 0)),
            None => BTreeMap::new(),
        };
        let due = mem::replace(&mut self.future, upper);
        for (_, event) in due {
            self.slot[event.region.index()].insert(event.sequence, event);
        }
    }
}

/// Two-tier discrete-event scheduler over integer ticks.
///
/// One tier is the current tick's stratified slot, iterated until quiescent;
/// the other is every later tick, keyed by the total order. Advancing is
/// [`Self::run_time_slot`], which jumps straight to the next tick that has an
/// event rather than stepping through empty ones.
#[derive(Debug)]
pub struct EventScheduler {
    queues: EventQueues,
    resolution: TimeResolution,
    limits: SchedulerLimits,
    /// Tick of the slot currently running or most recently run.
    current_tick: u64,
    /// Whether any slot has run. Until one has, tick 0 is still schedulable.
    started: bool,
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
    pub fn next_tick(&self) -> Option<u64> {
        if !self.queues.slot_is_empty() {
            return Some(self.current_tick);
        }
        self.queues.future.keys().next().map(|(tick, _, _)| *tick)
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
        // Outside a slot nothing is open, so everything lands in the future
        // tier; `u64::MAX` can never equal a real current tick here because a
        // slot at that tick would have to be running.
        Ok(self.queues.insert(u64::MAX, tick, region, target, value))
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
        F: FnMut(&ScheduledEvent, &mut SchedulerContext<'_>),
    {
        let Some(tick) = self.next_tick() else {
            return Ok(None);
        };

        self.current_tick = tick;
        self.started = true;
        self.queues.open_slot(tick);
        self.queues.activations.clear();

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
            execute(&event, &mut context);
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
            .insert(self.current_tick, tick, region, target, value))
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
