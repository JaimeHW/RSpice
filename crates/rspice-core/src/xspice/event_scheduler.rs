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
//! Event time is an [`Instant`]: one exact physical instant, held as the
//! IEEE-754 bit pattern of a non-negative finite `f64` number of seconds. It
//! is the only time base the kernel has, and it quantizes nothing — the analog
//! spine is `f64` seconds and every event the kernel orders is at a time the
//! analog spine can name bit for bit.
//!
//! A Verilog design still declares `timescale`/`timeprecision` per module, and
//! the resolution that serves all of them is the finest one, which
//! [`TimeResolution::finest`] computes. That grid is a *sublattice* of the
//! instants rather than a second time base: an HDL tick `T` embeds exactly,
//! as [`Instant::from_tick`], at the seconds [`TimeResolution::ticks_to_seconds`]
//! names, and reads back as itself through [`Instant::tick`] — for every
//! 64-bit tick, because an instant also carries a tick's rank among the
//! ticks that share one double of seconds. Reading an *analog* time as an
//! HDL-visible tick is [`Instant::floor_tick`], [`Instant::nearest_tick`] or
//! [`Instant::ceil_tick`], and which of the three a caller wants is a property
//! of the question it is asking, not of the instant; the three are stated
//! together on that type. All four conversions are exact integer arithmetic.
//!
//! A caller whose event times are not on any declared grid needs no encoding of
//! its own. The XSPICE path is that caller: its times come from code models and
//! the analog step controller and feed transient breakpoints unrounded, and it
//! schedules them as the instants they are. See `super::event`.
//!
//! # Ordering
//!
//! Same-time ordering is a total order, not a convention: every event carries a
//! sequence number unique within its scheduler, and events sort by
//!
//! ```text
//! (instant, region, sequence)
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
use rspice_veriloga_runtime::loop_abort;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::mem;
use std::sync::Arc;

mod checkpoint;
pub use checkpoint::{SchedulerCheckpoint, SchedulerCheckpointError, SchedulerCheckpointLimits};

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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
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

/// Ticks per second, indexed by the negated decimal exponent. Every entry is
/// an integer below `2^53`, so each is exact, unlike its reciprocal above.
const TICKS_PER_SECOND: [f64; 22] = [
    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
    1e17, 1e18, 1e19, 1e20, 1e21,
];

/// Where the exact value of a time sits between two consecutive ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TickFraction {
    /// On a tick.
    Zero,
    BelowHalf,
    Half,
    AboveHalf,
}

impl TickFraction {
    /// Where a remainder sits against `half`, half a tick on its scale.
    fn of(remainder: u128, half: u128) -> Self {
        match remainder.cmp(&half) {
            _ if remainder == 0 => Self::Zero,
            std::cmp::Ordering::Less => Self::BelowHalf,
            std::cmp::Ordering::Equal => Self::Half,
            std::cmp::Ordering::Greater => Self::AboveHalf,
        }
    }
}

/// The exact value of a non-negative finite double as `mantissa * 2^shift`.
fn dyadic(seconds: f64) -> (u64, i32) {
    let bits = seconds.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1 << 52) - 1);
    if exponent == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1 << 52), exponent - 1075)
    }
}

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
    /// The least tick count at which two ticks of some resolution can share
    /// one seconds image.
    ///
    /// The images of `t` and `t - 1` differ by one tick less two roundings of
    /// at most half an ulp each, and the ulp of `t` ticks is at most
    /// `t * 2^-52` ticks, so below `2^52` every tick of every resolution has a
    /// seconds image of its own. Above it a fine resolution runs out of
    /// doubles — 1 fs from 8 s on — and consecutive ticks share an image,
    /// which [`Instant`] then tells apart by rank. Only
    /// [`Instant::from_tick`] reads it, and that only with HDL time.
    #[cfg_attr(not(feature = "veriloga"), allow(dead_code))]
    const SHARED_IMAGE_FLOOR: u64 = 1 << 52;

    /// Build a resolution of `10^exponent` seconds.
    ///
    /// The range is wider than `timescale` allows so that a kernel resolution
    /// refined below the finest declared precision is still representable.
    pub fn new(exponent: i8) -> Result<Self, SchedulerError> {
        if !(-21..=2).contains(&exponent) {
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
        match self.exponent {
            1 => 10.0,
            2 => 100.0,
            _ => SECONDS_PER_TICK[(-self.exponent) as usize],
        }
    }

    /// Convert a tick count to analog seconds: the tick's *image*.
    ///
    /// The image is the double nearest `ticks * 10^exponent`, ties to even,
    /// which is what the netlist reader makes of the same decimal: `#15` at
    /// 1 ns and a deck's `15n` are one instant. The product with
    /// [`Self::seconds_per_tick`] rounds twice and lands one ulp past `15n`,
    /// where a `.tran` stop at `15n` would drop the event. This is the
    /// conversion analog breakpoints are placed with, and every tick count
    /// has one: the `Result` refuses nothing.
    ///
    /// Images are monotone in the tick and, below
    /// [`Self::SHARED_IMAGE_FLOOR`], distinct. Above it consecutive ticks of a
    /// fine resolution can share one: the analog time they land at is one
    /// double, and their order is kept by [`Instant`]'s rank.
    pub fn ticks_to_seconds(self, ticks: u64) -> Result<f64, SchedulerError> {
        Ok(self.tick_instant(ticks))
    }

    /// The image of a tick, correctly rounded.
    ///
    /// Below `2^53` the tick count and the scale are both exact doubles, so
    /// one operation rounds once. Above it the count itself rounds, and the
    /// candidate that gives is moved, a double at a time, to the one whose
    /// rounding interval holds the exact value — a step or two at most.
    fn tick_instant(self, ticks: u64) -> f64 {
        let mut image = self.one_operation_image(ticks);
        if ticks < 1 << 53 {
            return image;
        }
        loop {
            if self
                .first_tick_at_or_above(image)
                .is_none_or(|first| first > ticks)
            {
                image = image.next_down();
            } else if self
                .last_tick_at_or_below(image)
                .is_some_and(|last| last < ticks)
            {
                image = image.next_up();
            } else {
                return image;
            }
        }
    }

    /// The tick count over the scale in one floating-point operation.
    fn one_operation_image(self, ticks: u64) -> f64 {
        let count = ticks as f64;
        match self.exponent {
            1 => count * 10.0,
            2 => count * 100.0,
            exponent => count / TICKS_PER_SECOND[(-exponent) as usize],
        }
    }

    /// The exact value of `mantissa * 2^shift` seconds counted in ticks: its
    /// whole part, and where the remainder sits against half a tick.
    ///
    /// Integer arithmetic throughout, so the answer is exact for every
    /// double and every resolution: the mantissa times a power of ten up to
    /// `10^21` stays below `2^125`, and a shift either lands inside `u128` or
    /// proves the value is past every tick count. `None` is a value of at
    /// least `2^128` ticks.
    fn split(self, mantissa: u64, shift: i32) -> Option<(u128, TickFraction)> {
        let mantissa = u128::from(mantissa);
        if mantissa == 0 {
            return Some((0, TickFraction::Zero));
        }
        let scale = 10u128.pow(u32::from(self.exponent.unsigned_abs()));
        if self.exponent <= 0 {
            // value = mantissa * 10^-exponent * 2^shift
            let scaled = mantissa * scale;
            let shift_count = shift.unsigned_abs();
            if shift >= 0 {
                if shift_count >= scaled.leading_zeros() {
                    return None;
                }
                return Some((scaled << shift_count, TickFraction::Zero));
            }
            if shift_count >= 128 {
                return Some((0, TickFraction::BelowHalf));
            }
            let remainder = scaled & ((1 << shift_count) - 1);
            let fraction = TickFraction::of(remainder, 1 << (shift_count - 1));
            return Some((scaled >> shift_count, fraction));
        }
        // value = numerator / denominator, with the power of two on whichever
        // side keeps both integers.
        let shift_count = shift.unsigned_abs();
        let (numerator, denominator) = if shift >= 0 {
            if shift_count >= mantissa.leading_zeros() {
                return None;
            }
            (mantissa << shift_count, scale)
        } else {
            // A denominator of 2^127 or more is over 2^72 times the mantissa:
            // far short of half a tick.
            if shift_count + 1 >= scale.leading_zeros() {
                return Some((0, TickFraction::BelowHalf));
            }
            (mantissa, scale << shift_count)
        };
        // The denominator is below 2^127, so twice the remainder fits.
        let fraction = TickFraction::of(numerator % denominator * 2, denominator);
        Some((numerator / denominator, fraction))
    }

    /// The largest tick whose image is at or below `seconds`, or `None` when
    /// `seconds` itself is past every tick.
    ///
    /// A tick's image is at or below `seconds` exactly when its value is below
    /// the upper edge of `seconds`' rounding interval, half an ulp above it —
    /// or on that edge, when ties to even round the edge down onto `seconds`.
    fn last_tick_at_or_below(self, seconds: f64) -> Option<u64> {
        let (mantissa, shift) = dyadic(seconds);
        if self.split(mantissa, shift)?.0 > u128::from(u64::MAX) {
            return None;
        }
        let last = match self.split(2 * mantissa + 1, shift - 1) {
            // On the edge, which rounds up and away from `seconds`.
            Some((whole, TickFraction::Zero)) if mantissa % 2 == 1 => whole - 1,
            Some((whole, _)) => whole,
            None => u128::MAX,
        };
        Some(u64::try_from(last).unwrap_or(u64::MAX))
    }

    /// The least tick whose image is at or above `seconds`, or `None` when no
    /// tick's image is.
    ///
    /// The mirror of [`Self::last_tick_at_or_below`], against the lower edge
    /// of `seconds`' rounding interval: the upper edge of the double below it.
    fn first_tick_at_or_above(self, seconds: f64) -> Option<u64> {
        if seconds == 0.0 {
            return Some(0);
        }
        let (mantissa, shift) = dyadic(seconds.next_down());
        let first = match self.split(2 * mantissa + 1, shift - 1)? {
            // On the edge, which ties to even round up onto `seconds`.
            (whole, TickFraction::Zero) if mantissa % 2 == 1 => whole,
            (whole, _) => whole + 1,
        };
        u64::try_from(first).ok()
    }

    /// The tick nearest `seconds`, read the way a tick is: the double nearest
    /// a half tick *is* that half tick, and a half tick goes to the later one.
    ///
    /// A deck's `2.5n` is a double just below 2.5 ns, and an author who writes
    /// it means the half tick of a 1 ns grid. Reading it exactly would put it
    /// on tick 2, against the tie rule, and a double one ulp below it on tick
    /// 3 by an accident of rounding the product, as the one-multiply reading
    /// did. A tick's own image is that tick, even where it is also the double
    /// nearest the half tick after it, as it can be once a double's spacing
    /// passes half a tick.
    fn tick_nearest(self, seconds: f64) -> Option<u64> {
        let (mantissa, shift) = dyadic(seconds);
        let (whole, fraction) = self.split(mantissa, shift)?;
        let later = match fraction {
            TickFraction::Zero => false,
            TickFraction::Half | TickFraction::AboveHalf => true,
            // Short of the half tick, but its nearest double when the half
            // lies inside this double's rounding interval, unless `whole`'s
            // own image is this double too.
            TickFraction::BelowHalf => {
                let holds_the_half = match self.split(2 * mantissa + 1, shift - 1) {
                    None => true,
                    Some((edge, _)) if edge > whole => true,
                    Some((_, TickFraction::AboveHalf)) => true,
                    Some((_, TickFraction::Half)) => mantissa % 2 == 0,
                    Some(_) => false,
                };
                holds_the_half
                    && self
                        .first_tick_at_or_above(seconds)
                        .is_none_or(|first| u128::from(first) > whole)
            }
        };
        u64::try_from(whole + u128::from(later)).ok()
    }

    /// Convert analog seconds to the nearest tick.
    ///
    /// Rejects non-finite and negative inputs, and any value whose nearest
    /// tick is past `u64`. Exact for every double: see the reading of a half
    /// tick on the method this calls.
    pub fn seconds_to_ticks(self, seconds: f64) -> Result<u64, SchedulerError> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        self.tick_nearest(seconds)
            .ok_or(SchedulerError::SecondsNotRepresentable { seconds })
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
    /// `ticks_to_seconds(t) <= seconds` — against the tick's image, not its
    /// exact value. A time that is a tick's image converts back to that tick
    /// and never to the one before it, which is the one error this conversion
    /// must not make: an event time handed back as a breakpoint would be
    /// delivered a tick late. It is computed in integers from the edge of
    /// `seconds`' rounding interval, so it is exact for every double and every
    /// tick count; where consecutive ticks share an image (past
    /// [`Self::SHARED_IMAGE_FLOOR`]) it is the last of them, every one of
    /// which the analog time has reached.
    ///
    /// Crate-visible rather than public, and reached through
    /// [`Instant::floor_tick`] rather than directly: an analog time is an
    /// [`Instant`] before anything asks which tick it is in, so the seconds
    /// this takes are always an instant's own seconds. Gated with that method,
    /// whose callers are all in the `veriloga` mixed interleave.
    #[cfg(feature = "veriloga")]
    pub(crate) fn seconds_to_floor_ticks(self, seconds: f64) -> Result<u64, SchedulerError> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        self.last_tick_at_or_below(seconds)
            .ok_or(SchedulerError::SecondsNotRepresentable { seconds })
    }

    /// Convert analog seconds to the tick *at or after* them.
    ///
    /// This is the conversion an event crossing from *another event kernel*
    /// into the HDL time wheel is dated with, and it is the third of the three
    /// mappings this type carries — see the module documentation of
    /// [`crate::xspice::verilog`], which states all three in one place.
    ///
    /// The rule it implements is "not before". An XSPICE code model names its
    /// event times in seconds and the HDL names them in ticks, so the two
    /// directions of that boundary are the two halves of one map: an HDL tick
    /// `T` reaches XSPICE as exactly [`Self::ticks_to_seconds`]`(T)`, and an
    /// off-grid XSPICE instant reaches the HDL at the least tick whose own
    /// instant is not before it — the HDL never labels a foreign-kernel event
    /// earlier than it happened.
    ///
    /// Rounding to nearest is a defensible alternative rather than a forbidden
    /// one: a `#1` from a process dated at `T` still fires at `T + 1`, one
    /// whole unit later on the wheel. What it costs is that `$realtime` names
    /// an instant before the event that caused the wake, and that the delay,
    /// measured against the far kernel's clock, comes out short of a unit —
    /// which is what the nearest-tick rule for *analog* events accepts on
    /// purpose and what a two-discrete-kernel boundary need not.
    ///
    /// Neither of the other two mappings may be rewritten into this one.
    /// [`Self::seconds_to_floor_ticks`] bounds how far the digital world may
    /// be advanced; [`Self::seconds_to_ticks`] places an *analog* event, which
    /// Verilog-AMS LRM 2.4 section 7.3.6.1 fixes at the nearest tick rather
    /// than at the next one.
    ///
    /// # Exactness
    ///
    /// Defined as the smallest `t` with `ticks_to_seconds(t) >= seconds`, the
    /// mirror of [`Self::seconds_to_floor_ticks`] and computed the same way,
    /// so the two agree: an instant that is a tick's image converts to that
    /// tick in both directions, and only a strictly interior one moves up.
    ///
    /// Crate-visible and reached through [`Instant::ceil_tick`], and gated
    /// with it: that method's only caller is the mixed interleave, which is a
    /// `veriloga` module, so a build without the feature would carry an orphan
    /// that `-D warnings` reports.
    #[cfg(feature = "veriloga")]
    pub(crate) fn seconds_to_ceil_ticks(self, seconds: f64) -> Result<u64, SchedulerError> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(SchedulerError::SecondsNotRepresentable { seconds });
        }
        self.first_tick_at_or_above(seconds)
            .ok_or(SchedulerError::SecondsNotRepresentable { seconds })
    }

    /// The tick `seconds` is that tick's instant computed another way: the
    /// nearest tick, when `seconds` lies strictly less than `window` from its
    /// exact image.
    ///
    /// `None` when no tick is that close, when the window is not a finite
    /// positive interval, and for a time that has no tick at all. The window an
    /// [`EventLattice`] hands this is never wider than a thousandth of a tick,
    /// so at most one tick can qualify and the nearest one is it.
    #[cfg(feature = "veriloga")]
    pub(crate) fn coincident_tick(self, seconds: f64, window: f64) -> Option<u64> {
        if !window.is_finite() || window <= 0.0 {
            return None;
        }
        let tick = self.seconds_to_ticks(seconds).ok()?;
        ((seconds - self.tick_instant(tick)).abs() < window).then_some(tick)
    }
}

/// The HDL tick lattice a coupled run's discrete instants are canonical on,
/// with the solver floor that says how close to a tick image an instant has to
/// be to be that tick.
///
/// Present only where code models share event nets with a mixed module: an
/// uncoupled XSPICE run has no lattice to be canonical on. See
/// [`Instant`]'s "one instant, one key" for the rule this carries.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct EventLattice {
    resolution: TimeResolution,
    /// The transient stepper's hard minimum at its latest accepted clock, or
    /// zero before a solver has declared one.
    floor: f64,
}

#[cfg(feature = "veriloga")]
impl EventLattice {
    /// The widest coincidence window, as a fraction of one tick.
    ///
    /// The window absorbs two roundings of one instant and nothing else. A
    /// floor is a solver resolution rather than a rounding, and a deck can
    /// declare a large one — a `DELMIN` near the precision — which uncapped
    /// would quantize code-model timing onto the HDL grid. At a 1 ps precision
    /// the cap is a femtosecond: thousands of ulps of a millisecond clock, and
    /// four of a one-second one.
    const MAX_WINDOW_PER_TICK: f64 = 1.0e-3;

    /// The lattice of `resolution`'s ticks under the solver floor `floor`. A
    /// floor that is not a finite positive interval declares none.
    #[doc(hidden)]
    pub fn new(resolution: TimeResolution, floor: f64) -> Self {
        Self {
            resolution,
            floor: if floor.is_finite() && floor > 0.0 {
                floor
            } else {
                0.0
            },
        }
    }

    /// How close to a tick image an instant at `seconds` has to be to be that
    /// tick: the solver's floor at that clock, never wider than
    /// [`Self::MAX_WINDOW_PER_TICK`] of a tick.
    ///
    /// The floor at a later clock is the declared one or the clock's own
    /// resolution, whichever is larger
    /// ([`crate::numerics::integration::clock_resolution`]) — the transient
    /// stepper's `hard_min_at_clock`, read at the instant in question rather
    /// than at the point that scheduled it.
    pub(crate) fn window(self, seconds: f64) -> f64 {
        let floor = self
            .floor
            .max(crate::numerics::integration::clock_resolution(seconds));
        floor.min(self.resolution.seconds_per_tick() * Self::MAX_WINDOW_PER_TICK)
    }
}

/// One exact physical instant: the IEEE-754 bit pattern of a non-negative
/// finite `f64` number of seconds, and the rank of an HDL tick among the
/// ticks that share those seconds.
///
/// This is the kernel's whole time key, and its seconds are the *analog*
/// spine's time with nothing done to it. For a non-negative finite `f64` the
/// bit pattern read as a `u64` is strictly monotone in the value and exactly
/// invertible, which is everything an event key has to be: it orders, and it
/// converts back. A time that cannot be scheduled at all — negative,
/// infinite, or NaN — has no instant, which is the same set the queue has
/// always dropped.
///
/// The seconds are the high half of one `u128` and the rank the low half, so
/// the kernel orders keys with one integer comparison and never looks at the
/// rank on its own. A time named in seconds has rank zero. So does every tick
/// whose image no other tick shares — every tick below
/// [`TimeResolution::SHARED_IMAGE_FLOOR`], which is every tick of every run
/// short of 8 s at 1 fs — and its key is then exactly its image's.
///
/// # Why not a tick count
///
/// The two worlds that schedule here name their times differently and both are
/// right. An HDL process names a tick on its module's declared grid; an XSPICE
/// code model and the analog step controller name a `f64` second chosen by
/// interpolation, a delay, or a step controller, on no grid at all. A shared
/// decimal grid would have to quantize the second kind — a 100.4 ps output on
/// a 1 ns grid would fire at 1 ns.
///
/// The exact key refuses that loss, and the HDL grid does not disappear: it
/// embeds. A tick `T` is keyed at its image, [`TimeResolution::ticks_to_seconds`],
/// which is where the analog side lands its event, and [`Self::tick`] reads
/// `T` back exactly. Doubles do not resolve every tick of a fine grid for
/// long — at 1 fs consecutive ticks share an image from 8 s on — so a tick
/// carries its rank among the ticks sharing its image, and two of them are
/// two instants, in tick order, at one analog time. The HDL keeps the whole
/// 64-bit tick range Verilog gives it, as Icarus Verilog does.
///
/// # Reading an instant as a tick
///
/// Three rules, and a caller may not substitute one for another — they answer
/// different questions and each has its own landed ruling:
///
/// * [`Self::floor_tick`] — how far the digital world may be advanced for an
///   analog timepoint. Monotone, and never past an instant the integrator has
///   accepted.
/// * [`Self::nearest_tick`] — where an A/D transition's own timestamp lands,
///   which Verilog-AMS LRM 2.4 section 7.3.6.1 fixes at the nearest tick.
/// * [`Self::ceil_tick`] — where an event crossing in from another event
///   kernel lands, so the HDL never labels a foreign event earlier than it
///   happened. A wake read off a coupled event uses its coincidence-aware
///   form, [`Self::coincident_ceil_tick`]; see below.
///
/// # One instant, one key
///
/// Where code models share event nets with an HDL module, one instant can be
/// computed two ways: as a tick's exact image, and as a code model's float sum
/// — `crossing + 1e-9`, `TIME + delay` — or a PULSE corner that names the same
/// nanosecond a few ulps away. Two keys for one instant cost the run twice. The
/// copy keyed later is landed one solver floor after the other, the code model
/// it causes is evaluated again there and puts a second, late copy of its own
/// output on the queue, and a wake read off the late landing reaches the HDL a
/// whole tick late. The two copies' ulp order also decides which side of an
/// HDL edge a flip-flop samples, which no declared ordering rule does.
///
/// So in a coupled run **a discrete instant less than one coincidence window
/// from a tick image is that tick image**, and is keyed at its exact bits
/// ([`Self::canonical`]). The window is the solver's floor at that clock,
/// capped at a thousandth of a tick ([`EventLattice::window`]): it absorbs a
/// rounding, never a delay a model meant, however large a minimum step the
/// deck declares. The snap is onto the lattice, never onto a neighbouring
/// event, so it cannot chain; it is idempotent; and it never moves a key
/// across the instant that scheduled it, so a zero-delay output at an off-grid
/// "now" stays at "now". Two bit-equal keys are then ordered by the declared
/// same-instant rule — HDL events first, then the code models' wave — rather
/// than by rounding. An uncoupled run has no lattice, and its keys are the sums
/// its code models computed.
///
/// The ceiling reads the rule back. A wake from the other kernel lands on the
/// least tick not before the event *after coincidence*: an instant less than
/// one window past a tick image is that tick ([`Self::coincident_ceil_tick`]).
/// Floor and nearest are unchanged, and the landing contract the analog
/// stepper keeps with every lane is the same rule again: instants less than one
/// floor apart are one instant, landed at the latest of them
/// (`engine::transient`'s `accepted_veriloga_event_time`).
///
/// Two words, compared image first: a pair of `u64`s keeps the eight-byte
/// alignment of everything that holds a key, where a `u128` would double it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// The bit pattern of the image's seconds.
    bits: u64,
    /// The rank among the ticks sharing that image.
    rank: u64,
}

impl Instant {
    /// The start of an analysis.
    ///
    /// Crate-visible: nothing outside names it, because an out-of-crate caller
    /// builds an instant from the time it has rather than from the origin.
    pub(crate) const ZERO: Instant = Instant { bits: 0, rank: 0 };

    /// The key of `seconds`' bit pattern and a rank.
    const fn from_parts(bits: u64, rank: u64) -> Self {
        Self { bits, rank }
    }

    /// The instant a number of seconds names, or `None` if it names none.
    ///
    /// Negative, infinite and NaN seconds have no instant. Positive and
    /// negative zero are one instant with two bit patterns, and the negative
    /// one does not compare below the positive one as a `u64`, so zero is
    /// normalized here rather than left to compare wrongly later.
    pub fn from_seconds(seconds: f64) -> Option<Self> {
        if !seconds.is_finite() || seconds < 0.0 {
            return None;
        }
        Some(Self::from_parts(
            if seconds == 0.0 { 0 } else { seconds.to_bits() },
            0,
        ))
    }

    /// The seconds this instant is: the exact inverse of
    /// [`Self::from_seconds`], and for a tick the analog time it lands at.
    pub fn seconds(self) -> f64 {
        f64::from_bits(self.bits())
    }

    /// The bit pattern of [`Self::seconds`].
    fn bits(self) -> u64 {
        self.bits
    }

    /// The rank of a tick among the ticks sharing its image; zero for a time
    /// named in seconds.
    fn rank(self) -> u64 {
        self.rank
    }

    /// The instant an HDL tick names, exactly: its image, and its rank among
    /// the ticks sharing that image.
    ///
    /// Every tick count has one, and [`Self::tick`] reads it back. The
    /// seconds are the image a breakpoint is placed at, so the analog side
    /// lands where the event is scheduled.
    ///
    /// Crate-visible and gated with the only world that has ticks: a caller
    /// outside the crate composes [`TimeResolution::ticks_to_seconds`] with
    /// [`Self::from_seconds`], which is the same map written out for every
    /// tick whose image is its own.
    #[cfg(feature = "veriloga")]
    pub(crate) fn from_tick(tick: u64, resolution: TimeResolution) -> Self {
        let image = resolution.tick_instant(tick);
        let rank = if tick < TimeResolution::SHARED_IMAGE_FLOOR {
            0
        } else {
            tick - resolution.first_tick_at_or_above(image).unwrap_or(tick)
        };
        Self::from_parts(image.to_bits(), rank)
    }

    /// The HDL tick an instant made by [`Self::from_tick`] names: the inverse
    /// of that map, exact for every tick.
    ///
    /// This is how a host reads its own keys back. It is not one of the three
    /// readings of an analog time below, which take the seconds alone: past
    /// [`TimeResolution::SHARED_IMAGE_FLOOR`] a tick's seconds are shared, and
    /// only its rank says which of them it is.
    #[cfg(feature = "veriloga")]
    pub(crate) fn tick(self, resolution: TimeResolution) -> Result<u64, SchedulerError> {
        let seconds = self.seconds();
        resolution
            .first_tick_at_or_above(seconds)
            .and_then(|first| first.checked_add(self.rank()))
            .ok_or(SchedulerError::SecondsNotRepresentable { seconds })
    }

    /// The HDL tick at or before this instant.
    ///
    /// Crate-visible and gated with its callers, as [`Self::nearest_tick`] and
    /// [`Self::ceil_tick`] are.
    #[cfg(feature = "veriloga")]
    pub(crate) fn floor_tick(self, resolution: TimeResolution) -> Result<u64, SchedulerError> {
        resolution.seconds_to_floor_ticks(self.seconds())
    }

    /// The HDL tick nearest this instant, ties going to the later one.
    ///
    /// Crate-visible and gated with its callers, the way the conversion behind
    /// it is: an A/D crossing is dated by `xspice::verilog`'s mixed
    /// interleave, which is a `veriloga` module, so a build without the
    /// feature has no caller and `-D warnings` would report an orphan.
    #[cfg(feature = "veriloga")]
    pub(crate) fn nearest_tick(self, resolution: TimeResolution) -> Result<u64, SchedulerError> {
        resolution.seconds_to_ticks(self.seconds())
    }

    /// The HDL tick at or after this instant.
    ///
    /// Crate-visible and gated for the same reason [`Self::nearest_tick`] is:
    /// the only caller is the wake the mixed interleave dates when an event
    /// crosses in from the other event kernel.
    #[cfg(feature = "veriloga")]
    pub(crate) fn ceil_tick(self, resolution: TimeResolution) -> Result<u64, SchedulerError> {
        resolution.seconds_to_ceil_ticks(self.seconds())
    }

    /// This instant as a coupled run keys it: the exact image of the tick it
    /// is within one coincidence window of, or itself when it is near none.
    ///
    /// `scheduled_from` is the instant whose evaluation scheduled this one,
    /// and the snap never moves a key across it, in either direction: an image
    /// earlier than both this instant and `scheduled_from` is refused, and so
    /// is an image after `scheduled_from` for an instant at or before it. The
    /// instant then keeps its own bits. A zero-delay output at an off-lattice
    /// "now" therefore stays at "now", on whichever side of a tick image "now"
    /// fell: moved back, it would land in a slot the run has already left;
    /// moved on, onto an image a rounding after the point that produced it, it
    /// would become a future instant less than one solver floor away, which
    /// the stepper can only reach a whole floor later. An accepted point a few
    /// ulps short of a tick image then took a 1e-20 s step and restarted its
    /// integration from there. An instant a negative delay — an output dated
    /// from a crossing inside the step — already put before `scheduled_from`
    /// may still move later onto its image, up to `scheduled_from`.
    ///
    /// A pure function of the instant, the lattice and `scheduled_from`, and
    /// idempotent: a tick image is its own nearest tick at distance zero. See
    /// "one instant, one key" on this type.
    #[cfg(feature = "veriloga")]
    #[doc(hidden)]
    pub fn canonical(self, lattice: EventLattice, scheduled_from: Instant) -> Self {
        let seconds = self.seconds();
        let Some(image) = lattice
            .resolution
            .coincident_tick(seconds, lattice.window(seconds))
            .and_then(|tick| Self::from_seconds(lattice.resolution.tick_instant(tick)))
        else {
            return self;
        };
        if image < self.min(scheduled_from) || (self <= scheduled_from && image > scheduled_from) {
            self
        } else {
            image
        }
    }

    /// The least HDL tick not before this instant, after coincidence: the
    /// tick at or before it when it lies less than one coincidence window past
    /// that tick's image, and otherwise [`Self::ceil_tick`].
    ///
    /// This is the wake a coupled code-model event dates in the HDL. The plain
    /// ceiling of an instant one rounding past a tick image is the *next*
    /// tick, so a `#d` that wake schedules would fire a whole tick late; the
    /// two are one instant, and the wake is that tick. See "one instant, one
    /// key" on this type.
    #[cfg(feature = "veriloga")]
    pub(crate) fn coincident_ceil_tick(self, lattice: EventLattice) -> Result<u64, SchedulerError> {
        let seconds = self.seconds();
        let resolution = lattice.resolution;
        let floor = resolution.seconds_to_floor_ticks(seconds)?;
        if seconds - resolution.tick_instant(floor) < lattice.window(seconds) {
            return Ok(floor);
        }
        self.ceil_tick(resolution)
    }
}

/// The seconds, and a tick's rank after a plus sign when the tick shares its
/// seconds with earlier ones.
impl fmt::Display for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.seconds())?;
        match self.rank() {
            0 => Ok(()),
            rank => write!(f, "+{rank}"),
        }
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
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct EventTarget {
    /// Circuit node identifier the driver writes.
    #[serde(with = "checkpoint::target_index")]
    pub node_id: usize,
    /// Port name on the owning instance.
    pub port_name: String,
    /// Element index within a vector port.
    #[serde(with = "checkpoint::target_index")]
    pub driver_index: usize,
    /// Instance that scheduled the event.
    pub instance: String,
}

/// A driver's identity, interned to a dense index.
///
/// [`EventTarget`] is the identity a caller names a driver by and the one a
/// diagnostic prints, and it carries two `String`s. Nothing on the per-event
/// path may compare or clone those: an event is queued, superseded, popped and
/// counted against its driver, and each of those was a `String`-keyed
/// `BTreeMap` probe. Interning happens once per distinct driver — at host
/// construction for a digital plan, at the first schedule for an XSPICE code
/// model — and every hot structure is keyed by the resulting index instead.
///
/// Ids are handed out in interning order, so which id a driver gets is a
/// function of the order the design schedules in and never of a hash seed. The
/// interning table is a lookup only: it is never iterated, and nothing
/// observable is ordered by it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(hidden)]
pub struct TargetId(u32);

impl TargetId {
    /// The dense index this id is.
    fn index(self) -> usize {
        self.0 as usize
    }
}

impl From<TargetId> for usize {
    fn from(id: TargetId) -> Self {
        id.index()
    }
}

/// One scheduled event, carrying its position in the total order.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledEvent {
    /// Instant the event is scheduled at.
    pub at: Instant,
    /// Region of that instant's slot.
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
    /// A delta cycle returned the whole design to a state it had already
    /// been in at this instant, `period` delta cycles before: the instant
    /// can never settle. Proven by the host that owns the state, not counted.
    RepeatedState {
        /// Delta cycles between two visits of the repeated state.
        period: u32,
    },
}

/// Why a tick failed to settle, and which drivers were responsible.
///
/// Delta-cycle settling is unbounded in the standard, which means a zero-delay
/// loop is a hang rather than a diagnosis. The limits exist to convert that
/// hang into this: the drivers that fired most often at the offending tick, in
/// descending activation order, which is the evidence needed to name the loop.
#[derive(Debug, Clone, PartialEq)]
pub struct OscillationDiagnostic {
    /// Instant that failed to settle.
    pub at: Instant,
    /// That instant as an HDL-visible tick of the resolution the reporting
    /// caller named, when it has one.
    ///
    /// A design's author names a time by its tick, so that is what the
    /// sentence this renders into prints. `None` is the honest answer for an
    /// instant with no tick on that grid — past the last 64-bit tick — and
    /// the sentence then names the instant's seconds instead.
    pub tick: Option<u64>,
    /// Which limit tripped.
    pub cause: OscillationCause,
    /// Delta cycles completed at `at` before the abort.
    pub delta_cycles: u32,
    /// Events executed at `at` before the abort.
    pub events_executed: u64,
    /// The configured delta-cycle ceiling.
    pub delta_cycle_limit: u32,
    /// The configured per-tick event ceiling.
    pub event_limit: u64,
    /// Busiest drivers at `tick`, paired with their activation counts and
    /// ordered by count descending, then by the driver's own order so the
    /// report is reproducible.
    pub entities: Vec<(EventTarget, u64)>,
    /// The nets of a loop among `entities`, in loop order, when the host that
    /// owns the drivers could close one through their static sensitivity.
    ///
    /// The kernel knows who fired, not why; this is the why, and only a host
    /// that knows what each driver reads and writes can fill it in. Boxed
    /// because it is rare and every refusal that can carry it is moved by
    /// value.
    #[cfg(feature = "veriloga")]
    pub(crate) zero_delay_loop: Option<Box<ZeroDelayLoop>>,
}

/// A loop among an oscillation's drivers; see
/// [`OscillationDiagnostic::zero_delay_loop`].
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ZeroDelayLoop {
    /// The nets the loop runs through, in order; the last wakes the first.
    pub(crate) nets: Vec<String>,
}

/// Errors the kernel reports instead of panicking or hanging.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulerError {
    /// No unused sequence number remains. The rejected operation changes no
    /// pending event, and subsequent execution reports the same failure.
    SequenceExhausted,
    /// A per-slot execution counter cannot represent another operation.
    AccountingExhausted,
    /// A seconds value was non-finite, negative, or past the last 64-bit tick.
    SecondsNotRepresentable {
        /// The offending seconds value.
        seconds: f64,
    },
    /// A resolution exponent outside the supported decimal range.
    UnsupportedResolution {
        /// The offending exponent.
        exponent: i8,
    },
    /// An event was scheduled at an instant the scheduler has already left.
    ScheduleInThePast {
        /// Instant the scheduler has reached.
        reached: Instant,
        /// Instant the caller asked for.
        requested: Instant,
    },
    /// An instant did not settle within its limits.
    Oscillation(OscillationDiagnostic),
    /// The run's abort hook answered while an instant was settling.
    Aborted,
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchedulerError::SequenceExhausted => write!(f, "event sequence numbers are exhausted"),
            SchedulerError::AccountingExhausted => write!(f, "event slot accounting is exhausted"),
            SchedulerError::SecondsNotRepresentable { seconds } => {
                write!(f, "time {seconds:e} s has no representable event tick")
            }
            SchedulerError::UnsupportedResolution { exponent } => {
                write!(f, "unsupported time resolution 1e{exponent} s")
            }
            SchedulerError::ScheduleInThePast { reached, requested } => write!(
                f,
                "event scheduled at {requested} s, but the scheduler has reached {reached} s"
            ),
            SchedulerError::Aborted => write!(f, "stopped while an event time step was settling"),
            SchedulerError::Oscillation(diagnostic) => {
                let cause = match diagnostic.cause {
                    OscillationCause::DeltaCycleLimit => {
                        format!(" within {} delta cycles", diagnostic.delta_cycle_limit)
                    }
                    OscillationCause::EventLimit => {
                        format!(" within {} events", diagnostic.event_limit)
                    }
                    OscillationCause::RepeatedState { period } => format!(
                        ": after {} delta cycles its state repeats every {period}",
                        diagnostic.delta_cycles
                    ),
                };
                // A design's author names a time by its tick, so the tick is
                // what this prints when the reporting caller named a grid to
                // read the instant on. Without one — the XSPICE lane, whose
                // times sit on no grid — the instant's own seconds are the
                // only honest answer, and that lane renders its own sentence
                // anyway.
                match diagnostic.tick {
                    Some(tick) => write!(
                        f,
                        "event network did not settle at tick {tick} ({:e} s){cause}",
                        diagnostic.at.seconds()
                    )?,
                    None => write!(
                        f,
                        "event network did not settle at {} s{cause}",
                        diagnostic.at
                    )?,
                }
                // Every driver collected, not the first: a loop has at least
                // two members unless it feeds itself, and the one that fired
                // most is only one of them.
                for (position, (target, count)) in diagnostic.entities.iter().enumerate() {
                    f.write_str(if position == 0 {
                        "; busiest drivers: "
                    } else {
                        "; "
                    })?;
                    f.write_str(&target.instance)?;
                    if !target.port_name.is_empty() {
                        write!(f, " driving {}", target.port_name)?;
                    }
                    let times = if *count == 1 { "time" } else { "times" };
                    write!(f, " fired {count} {times}")?;
                }
                #[cfg(feature = "veriloga")]
                if let Some(nets) = diagnostic
                    .zero_delay_loop
                    .as_deref()
                    .map(|cycle| &cycle.nets)
                    && let Some(first) = nets.first()
                {
                    write!(f, "; zero-delay loop: {} -> {first}", nets.join(" -> "))?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SchedulerError {}

/// Ceilings that turn a non-settling tick into a diagnosis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

impl SchedulerLimits {
    /// The limits for an HDL design, alone or inside a circuit: none.
    ///
    /// IEEE 1364-2005 section 11.4 leaves the settling of one time step
    /// unbounded, and Icarus Verilog bounds none, so no count may stop a legal
    /// program. A step that cannot settle is proven by its host, which refuses
    /// a delta cycle that returns the design to a state it was already in
    /// ([`OscillationCause::RepeatedState`]); any other runaway is ended by the
    /// run's Stop or time limit, which the kernel polls inside the step
    /// ([`SchedulerError::Aborted`]).
    #[cfg(feature = "veriloga")]
    #[doc(hidden)]
    pub const HDL: Self = Self {
        max_delta_cycles_per_tick: u32::MAX,
        max_events_per_tick: u64::MAX,
        max_reported_oscillating_entities: 16,
    };
}

/// One event due in an open slot, as [`EventScheduler::due_events_into`]
/// reports it: everything but its sequence number.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DueEvent {
    at: Instant,
    region: SchedulerRegion,
    target: TargetId,
    value: EventValue,
}

#[cfg(feature = "veriloga")]
impl DueEvent {
    /// The target the event activates.
    pub(crate) fn target(&self) -> TargetId {
        self.target
    }
}

/// What one settled instant did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSlotReport {
    /// The instant that ran.
    pub at: Instant,
    /// Delta cycles it took to settle.
    pub delta_cycles: u32,
    /// Events executed at that instant.
    pub events_executed: u64,
}

/// Scheduling state, split out so an executing event can schedule through
/// [`SchedulerContext`] while the driver loop owns the event it popped.
/// A queued event, as the queues hold it.
///
/// The same thing a [`ScheduledEvent`] is, with the driver as its interned id
/// rather than its two-`String` identity. This is what moves between the
/// tiers, so queueing, superseding and popping cost no string traffic at all;
/// the public `ScheduledEvent` is materialized from the interning table only
/// where a caller's closure is about to see one.
#[derive(Debug, Clone, PartialEq)]
struct PendingEvent {
    at: Instant,
    region: SchedulerRegion,
    sequence: u64,
    target: TargetId,
    value: EventValue,
}

/// One event in the future tier, ordered by the total order and by nothing
/// else.
///
/// The comparison *is* the ordering specification: `(instant, region, sequence)`,
/// with the sequence unique across the scheduler, so no two entries compare
/// equal and the heap's shape can never decide which of two events runs first.
/// Deliberately not derived — a derived `Ord` would order by whatever fields
/// [`PendingEvent`] happens to declare, and adding a field to that struct
/// would silently change the order of a run.
#[derive(Debug, Clone)]
struct Queued(PendingEvent);

impl Queued {
    fn key(&self) -> (Instant, SchedulerRegion, u64) {
        (self.0.at, self.0.region, self.0.sequence)
    }
}

impl Ord for Queued {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

impl PartialOrd for Queued {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Queued {
    fn eq(&self, other: &Self) -> bool {
        self.0.sequence == other.0.sequence
    }
}

impl Eq for Queued {}

/// The drivers a kernel has named, in interning order, and the index back from
/// a driver to its id.
///
/// Append-only: an id, once handed out, names the same driver for the life of
/// the kernel and of every copy taken from it. That is what lets a copy share
/// the table rather than duplicate it. A rollback image copies the kernel at
/// every attempted timestep and every Newton probe that drains an event, and
/// the table holds two `String`s per driver twice over -- on a gate-level
/// design, thousands of allocations per copy for a table that stops growing
/// once every driver has fired. A copy that interns a new driver copies the
/// table then, through [`Arc::make_mut`], and the ids both copies agree on
/// stay the same ids.
#[derive(Debug, Clone, Default)]
struct TargetTable {
    /// Every driver ever scheduled, indexed by its [`TargetId`]. This is what
    /// a `ScheduledEvent` and an oscillation report are rendered from.
    targets: Vec<EventTarget>,
    /// Interning index. A lookup only: never iterated, so no observable order
    /// depends on it.
    ids: HashMap<EventTarget, TargetId>,
}

#[derive(Debug, Default)]
struct EventQueues {
    /// Events at ticks after the current one, as a min-heap on the total
    /// order.
    ///
    /// A heap rather than an ordered map because nothing reads this tier in
    /// order except one event at a time from its front: the drain wants the
    /// earliest tick and then every event at that tick, which is a run of
    /// pops. A map paid a node allocation per event to keep an ordering
    /// nothing else consults — W5.2a measured it at a third of an allocation
    /// per event across both tiers — and the heap pays none, because its
    /// backing vector keeps the capacity a tick's traffic gave it.
    ///
    /// Its top is always live: a cancellation that lands on the top prunes it
    /// (see [`Self::cancelled`]), so [`Self::future_min_tick`] and
    /// [`Self::open_next_due_tick`] can read it without a scan.
    future: BinaryHeap<Reverse<Queued>>,
    /// The current tick's slot, one FIFO per region.
    ///
    /// In sequence order by construction rather than by comparison: an event
    /// filed into the open slot carries a sequence higher than everything
    /// already in it — it was drawn later — and the events a tick opens with
    /// arrive from `future` already ascending. [`Self::push_slot`] states that
    /// as a check rather than assuming it, so the one path that can break it
    /// (a slot left part-settled by an oscillation, then refilled from an
    /// earlier tick) still files in order.
    slot: [VecDeque<PendingEvent>; REGION_COUNT],
    /// Sequences cancelled out of `future` and not yet skipped.
    ///
    /// Supersession cannot reach into a heap, so a cancelled future event is
    /// tombstoned here and dropped when it surfaces. A set of sequences rather
    /// than a flag in the event or a bitmap over a moving base, because the
    /// set is *empty* in the steady state — a driver supersedes an output it
    /// has already delivered, which cancels nothing — so every reader can skip
    /// the lookup behind one `is_empty` test, and nothing is paid per event
    /// for a case that is not happening. It is exact, so [`Self::pending`]
    /// subtracts it rather than estimating.
    ///
    /// The slot is never tombstoned: [`Self::remove_scheduled`] takes an event
    /// out of a region queue directly. It can afford to, because supersession
    /// is only reachable from outside a running slot, so a non-empty slot to
    /// scan means a previous drain stopped on an oscillation ceiling.
    cancelled: HashSet<u64>,
    /// Next sequence number. Per-scheduler, never reused.
    next_sequence: u64,
    /// Infallible legacy supersession reports allocation failure at the next
    /// execution boundary; it must never cancel an output it cannot replace.
    sequence_exhausted: bool,
    /// Every driver ever scheduled and its interning index. Shared with every
    /// copy of the kernel until one of them interns a new driver; see
    /// [`TargetTable`].
    interned: Arc<TargetTable>,
    /// Activation counts at the current tick, for the oscillation report,
    /// indexed by [`TargetId`].
    activations: Vec<u64>,
    /// Which ids `activations` holds a non-zero count for at this tick, so
    /// opening a slot clears what fired rather than every driver in the
    /// design.
    activated: Vec<TargetId>,
    /// Where each driver's unexecuted events sit, indexed by [`TargetId`].
    /// This is the index [`EventScheduler::schedule_superseding_at`] consults;
    /// without it, superseding a driver's own pending output would mean
    /// scanning the whole queue.
    ///
    /// A flat list per driver, not a map by tick: a driver has one pending
    /// event almost always — that is what supersession *means* — so the map
    /// bought an ordering nothing reads and charged a node and a `Vec` per
    /// activation for it. Nothing here is ordered, because both readers cancel
    /// by unique sequence and neither outcome depends on the order the
    /// cancellations happen in.
    driver_events: Vec<Vec<PendingRef>>,
}

/// Written out so that `clone_from` refills every queue in place: a mixed
/// circuit's digital host images this kernel before each Newton probe and
/// refills a spent image rather than allocating one (`DigitalHost`'s
/// `host/image.rs`). Both methods destructure, so a new field must be placed.
impl Clone for EventQueues {
    fn clone(&self) -> Self {
        let Self {
            future,
            slot,
            cancelled,
            next_sequence,
            sequence_exhausted,
            interned,
            activations,
            activated,
            driver_events,
        } = self;
        Self {
            future: future.clone(),
            slot: slot.clone(),
            cancelled: cancelled.clone(),
            next_sequence: *next_sequence,
            sequence_exhausted: *sequence_exhausted,
            interned: Arc::clone(interned),
            activations: activations.clone(),
            activated: activated.clone(),
            driver_events: driver_events.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let Self {
            future,
            slot,
            cancelled,
            next_sequence,
            sequence_exhausted,
            interned,
            activations,
            activated,
            driver_events,
        } = self;
        let Self {
            future: source_future,
            slot: source_slot,
            cancelled: source_cancelled,
            next_sequence: source_next_sequence,
            sequence_exhausted: source_sequence_exhausted,
            interned: source_interned,
            activations: source_activations,
            activated: source_activated,
            driver_events: source_driver_events,
        } = source;
        future.clone_from(source_future);
        slot.clone_from(source_slot);
        cancelled.clone_from(source_cancelled);
        *next_sequence = *source_next_sequence;
        *sequence_exhausted = *source_sequence_exhausted;
        interned.clone_from(source_interned);
        activations.clone_from(source_activations);
        activated.clone_from(source_activated);
        driver_events.clone_from(source_driver_events);
    }
}

/// One of a driver's unexecuted events: the instant that decides whether a
/// supersession reaches it, and the sequence that identifies it.
///
/// The region is not carried. Cancellation names an event by its sequence,
/// which is unique across the scheduler, and the region a promoted event was
/// scheduled into no longer says which queue it is sitting in.
type PendingRef = (Instant, u64);

impl EventQueues {
    /// Place an event, routing it to the current slot or the future tier.
    ///
    /// `open_slot` is the instant whose slot is running, if one is; an event at
    /// that same instant joins it as a delta event. It is an `Option` rather
    /// than a sentinel instant because every instant is schedulable, and a
    /// sentinel would route an event scheduled there into whatever slot was
    /// open.
    fn insert(
        &mut self,
        open_slot: Option<Instant>,
        at: Instant,
        region: SchedulerRegion,
        target: TargetId,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        let sequence = self.next_sequence;
        self.require_sequence()?;
        self.next_sequence += 1;
        self.driver_events[target.index()].push((at, sequence));
        let event = PendingEvent {
            at,
            region,
            sequence,
            target,
            value,
        };
        if open_slot == Some(at) {
            self.push_slot(event);
        } else {
            self.future.push(Reverse(Queued(event)));
        }
        Ok(sequence)
    }

    fn require_sequence(&mut self) -> Result<(), SchedulerError> {
        if self.sequence_exhausted || self.next_sequence == u64::MAX {
            self.sequence_exhausted = true;
            return Err(SchedulerError::SequenceExhausted);
        }
        Ok(())
    }

    /// File an event into its region's queue, in sequence order.
    ///
    /// The append is the whole operation in every reachable case: both callers
    /// hand over events whose sequences ascend, so the queue's last entry is
    /// already lower. The ordered branch exists for the one state that breaks
    /// that — a region still holding events from a slot a ceiling stopped
    /// mid-settle, refilled when a later call opens an earlier instant — where
    /// an append would run a lower sequence after a higher one.
    fn push_slot(&mut self, event: PendingEvent) {
        let queue = &mut self.slot[event.region.index()];
        match queue.back() {
            Some(last) if last.sequence > event.sequence => {
                let at = queue.partition_point(|held| held.sequence < event.sequence);
                queue.insert(at, event);
            }
            _ => queue.push_back(event),
        }
    }

    /// Drop cancelled entries from the top of the future tier, so that the top
    /// is a live event or the tier is empty.
    ///
    /// Every reader of the tier's earliest event depends on this, and only a
    /// cancellation can break it: nothing else puts a dead entry where a live
    /// one was.
    fn prune_future(&mut self) {
        while !self.cancelled.is_empty() {
            let Some(Reverse(top)) = self.future.peek() else {
                return;
            };
            let sequence = top.0.sequence;
            if !self.cancelled.remove(&sequence) {
                return;
            }
            self.future.pop();
        }
    }

    /// The id of a driver, interning it if this is the first time it is named.
    ///
    /// Off the per-event path by construction: a caller that schedules for the
    /// same driver repeatedly — every digital process does — interns once and
    /// then schedules by id.
    fn intern(&mut self, target: EventTarget) -> TargetId {
        if let Some(id) = self.interned.ids.get(&target) {
            return *id;
        }
        let id = TargetId(
            u32::try_from(self.interned.targets.len())
                .expect("an event kernel does not host 2^32 distinct drivers"),
        );
        let interned = Arc::make_mut(&mut self.interned);
        interned.targets.push(target.clone());
        interned.ids.insert(target, id);
        self.driver_events.push(Vec::new());
        self.activations.push(0);
        id
    }

    /// The identity behind an id.
    fn target(&self, id: TargetId) -> &EventTarget {
        &self.interned.targets[id.index()]
    }

    /// Every driver interned so far, in id order.
    fn targets(&self) -> &[EventTarget] {
        &self.interned.targets
    }

    /// Render one queued event as the public [`ScheduledEvent`] a caller's
    /// closure sees. The two `String`s are cloned exactly here and nowhere
    /// else on the event path.
    fn render(&self, event: PendingEvent) -> ScheduledEvent {
        ScheduledEvent {
            at: event.at,
            region: event.region,
            sequence: event.sequence,
            target: self.target(event.target).clone(),
            value: event.value,
        }
    }

    /// Count one activation of `target` against the open slot.
    fn note_activation(&mut self, target: TargetId) -> Result<(), SchedulerError> {
        let count = &mut self.activations[target.index()];
        let next = count
            .checked_add(1)
            .ok_or(SchedulerError::AccountingExhausted)?;
        if *count == 0 {
            self.activated.push(target);
        }
        *count = next;
        Ok(())
    }

    /// Forget the open slot's activation counts.
    fn clear_activations(&mut self) {
        for id in self.activated.drain(..) {
            self.activations[id.index()] = 0;
        }
    }

    fn slot_is_empty(&self) -> bool {
        self.slot.iter().all(VecDeque::is_empty)
    }

    /// Earliest instant any event still sitting in the slot is dated at.
    ///
    /// The slot is ordered by sequence, not by instant, because within one
    /// instant sequence is the whole tie-break after the region — so the
    /// instant has to be read off the events themselves. That scan is why the
    /// empty case returns first: an empty slot is the steady state between
    /// [`EventScheduler::run_due_events`] calls, which is where the hot
    /// predicates ask, and it must stay as cheap as the region-emptiness
    /// check it already was.
    ///
    /// A non-empty slot is only observable from inside a due-slot run or after
    /// one returned an oscillation, and neither is a per-evaluation path.
    fn slot_min_instant(&self) -> Option<Instant> {
        if self.slot_is_empty() {
            return None;
        }
        self.slot.iter().flatten().map(|event| event.at).min()
    }

    /// Earliest instant the future tier holds, which is the instant of its
    /// top: the top is the least entry by `(instant, region, sequence)` and is
    /// live.
    fn future_min_instant(&self) -> Option<Instant> {
        self.future.peek().map(|Reverse(top)| top.0.at)
    }

    /// [`Self::slot_min_instant`] with the driver that event names, for a
    /// diagnostic that has to say whose activation it is. Ties on the instant
    /// go to the lower sequence, which is the one the slot would run first.
    fn slot_min_event(&self) -> Option<(Instant, TargetId)> {
        if self.slot_is_empty() {
            return None;
        }
        self.slot
            .iter()
            .flatten()
            .min_by_key(|event| (event.at, event.sequence))
            .map(|event| (event.at, event.target))
    }

    /// The driver of every live event dated at `at`, in neither tier's
    /// order. A scan of both tiers, so it is for a question asked once per
    /// accepted point rather than once per event.
    fn targets_at(&self, at: Instant) -> impl Iterator<Item = TargetId> + '_ {
        let slot = self
            .slot
            .iter()
            .flatten()
            .filter(move |event| event.at == at);
        let future = self
            .future
            .iter()
            .map(|Reverse(queued)| &queued.0)
            .filter(move |event| {
                event.at == at
                    && (self.cancelled.is_empty() || !self.cancelled.contains(&event.sequence))
            });
        slot.chain(future).map(|event| event.target)
    }

    /// [`Self::future_min_instant`] with the driver that event names.
    fn future_min_event(&self) -> Option<(Instant, TargetId)> {
        self.future
            .peek()
            .map(|Reverse(top)| (top.0.at, top.0.target))
    }

    /// Number of events not yet executed.
    ///
    /// Exact, not an estimate: `cancelled` holds one sequence per tombstoned
    /// entry still sitting in `future`, and the slot holds no tombstones at
    /// all.
    fn pending(&self) -> usize {
        self.future.len() - self.cancelled.len()
            + self.slot.iter().map(VecDeque::len).sum::<usize>()
    }

    /// Take the lowest-sequence active event.
    ///
    /// The queue is in sequence order by construction, so this is its front.
    /// Returning `None` leaves the active queue *empty* rather than merely
    /// exhausted, which is what lets [`Self::promote`] be a swap.
    fn pop_active(&mut self) -> Option<PendingEvent> {
        let event = self.slot[SchedulerRegion::Active.index()].pop_front()?;
        self.forget_driver_event(event.target, event.sequence);
        Some(event)
    }

    /// Drop one event from the driver index.
    ///
    /// Matched on the sequence alone, which is unique across the scheduler,
    /// and removed by swapping the last entry down: the list is unordered by
    /// construction.
    fn forget_driver_event(&mut self, target: TargetId, sequence: u64) {
        let pending = &mut self.driver_events[target.index()];
        if let Some(position) = pending.iter().position(|(_, filed)| *filed == sequence) {
            pending.swap_remove(position);
        }
    }

    /// Remove one unexecuted event wherever it is held.
    ///
    /// Two different removals, because the two tiers answer differently. A
    /// region queue is a sequence of events and can give one up directly. A
    /// heap cannot, so a future event is tombstoned instead and dropped when
    /// it surfaces — and the top is pruned afterwards, because every reader of
    /// the tier's earliest event reads the top and expects it to be live.
    ///
    /// The slot is checked first only when it is not empty, and it is empty
    /// except after a ceiling stopped a drain mid-settle: this is reached from
    /// [`Self::supersede_driver`] alone, which is reached from outside a
    /// running slot alone. So the common case is one `is_empty` test and an
    /// insertion into a set that is about to be emptied again.
    fn remove_scheduled(&mut self, sequence: u64) {
        if !self.slot_is_empty() {
            for queue in self.slot.iter_mut() {
                if let Some(at) = queue.iter().position(|held| held.sequence == sequence) {
                    queue.remove(at);
                    return;
                }
            }
        }
        self.cancelled.insert(sequence);
        self.prune_future();
    }

    /// Cancel every unexecuted event of `target` at or after `at`, and
    /// report how many were cancelled.
    fn supersede_driver(&mut self, target: TargetId, at: Instant) -> usize {
        if self.driver_events[target.index()].is_empty() {
            return 0;
        }
        // Lifted out so that cancelling an entry can reach the queues, and put
        // back afterwards so the driver keeps the capacity it has already paid
        // for. Nothing cancelled here can re-file against this driver:
        // `remove_scheduled` only takes events out of the tiers.
        let mut pending = mem::take(&mut self.driver_events[target.index()]);
        let mut cancelled = 0;
        let mut position = 0;
        while position < pending.len() {
            if pending[position].0 < at {
                position += 1;
                continue;
            }
            let (_, sequence) = pending.swap_remove(position);
            self.remove_scheduled(sequence);
            cancelled += 1;
        }
        self.driver_events[target.index()] = pending;
        cancelled
    }

    /// Promote the first non-empty later region into the active region.
    ///
    /// Wholesale promotion is what makes a nonblocking update visible only in
    /// the next delta: the whole region moves at once, and anything it
    /// schedules back into the active region carries a higher sequence and so
    /// runs after every event promoted with it.
    ///
    /// A swap of two queues, not a re-insertion of every event. It is allowed
    /// to be, because this runs only when [`Self::pop_active`] has emptied the
    /// active queue, so the promoted region arrives in front of nothing and
    /// keeps the order it already had; and the active queue's buffer goes back
    /// to the region it came from rather than being dropped, so a region that
    /// fills every delta cycle allocates once for the whole run.
    fn promote(&mut self) -> bool {
        for region in &SchedulerRegion::ORDERED[1..] {
            let index = region.index();
            if self.slot[index].is_empty() {
                continue;
            }
            self.slot.swap(SchedulerRegion::Active.index(), index);
            return true;
        }
        false
    }

    /// Move the earliest pending instant at or before `bound` into the slot
    /// queues, reporting whether anything moved.
    ///
    /// One instant at a time, never a range: a slot queue is ordered by
    /// sequence alone, because within one instant sequence is the whole
    /// tie-break after the region. Emptying a span of instants into it at once
    /// would order them by creation instead of by time, so the due-slot mode
    /// opens the next instant only once the current one has gone quiet.
    ///
    /// The tier's top is live and least, so it names the due instant; the
    /// events at that instant are exactly the run of pops that follows, and
    /// they arrive ascending in `(region, sequence)`, which is the order the
    /// region queues want them in.
    fn open_next_due_instant(&mut self, bound: Instant) -> bool {
        let Some(due) = self.future_min_instant() else {
            return false;
        };
        if due > bound {
            return false;
        }
        while self.future_min_instant() == Some(due) {
            let Some(Reverse(entry)) = self.future.pop() else {
                break;
            };
            if !self.cancelled.is_empty() && self.cancelled.remove(&entry.0.sequence) {
                continue;
            }
            self.push_slot(entry.0);
        }
        self.prune_future();
        true
    }
}

/// Two-tier discrete-event scheduler over exact instants.
///
/// One tier is the current instant's stratified slot, iterated until
/// quiescent; the other is every later instant, ordered by the total order.
/// Advancing is [`Self::run_time_slot`], which jumps straight to the next
/// instant that has an event rather than stepping through empty ones.
///
/// No grid is declared here. An [`Instant`] is an exact time, and a caller
/// that thinks in HDL ticks converts at its own seam — which is what lets one
/// kernel order a module's `#5` and a code model's 100.4 ps output against
/// each other without either one being rounded onto the other's grid.
#[derive(Debug)]
pub struct EventScheduler {
    queues: EventQueues,
    limits: SchedulerLimits,
    /// Instant of the slot currently running or most recently run.
    current: Instant,
    /// Whether any slot has run. Until one has, [`Instant::ZERO`] is still
    /// schedulable.
    started: bool,
    /// Delta cycles counted against the open due slot. Only the due-slot mode
    /// uses this; [`Self::run_time_slot`] settles an instant inside one call
    /// and counts in a local.
    slot_delta_cycles: u32,
    /// Zero-delay resumptions of the open due slot ([`Self::note_zero_delay`]).
    slot_zero_delays: u32,
    /// Events executed against the open due slot, for the same reason.
    slot_events_executed: u64,
    /// A failed slot is not an accepted checkpoint boundary.
    failed_slot: bool,
}

/// Written out so that `clone_from` reaches [`EventQueues`]' own; see there.
impl Clone for EventScheduler {
    fn clone(&self) -> Self {
        let Self {
            queues,
            limits,
            current,
            started,
            slot_delta_cycles,
            slot_zero_delays,
            slot_events_executed,
            failed_slot,
        } = self;
        Self {
            queues: queues.clone(),
            limits: *limits,
            current: *current,
            started: *started,
            slot_delta_cycles: *slot_delta_cycles,
            slot_zero_delays: *slot_zero_delays,
            slot_events_executed: *slot_events_executed,
            failed_slot: *failed_slot,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let Self {
            queues,
            limits,
            current,
            started,
            slot_delta_cycles,
            slot_zero_delays,
            slot_events_executed,
            failed_slot,
        } = self;
        let Self {
            queues: source_queues,
            limits: source_limits,
            current: source_current,
            started: source_started,
            slot_delta_cycles: source_slot_delta_cycles,
            slot_zero_delays: source_slot_zero_delays,
            slot_events_executed: source_slot_events_executed,
            failed_slot: source_failed_slot,
        } = source;
        queues.clone_from(source_queues);
        *limits = *source_limits;
        *current = *source_current;
        *started = *source_started;
        *slot_delta_cycles = *source_slot_delta_cycles;
        *slot_zero_delays = *source_slot_zero_delays;
        *slot_events_executed = *source_slot_events_executed;
        *failed_slot = *source_failed_slot;
    }
}

impl EventScheduler {
    /// Build a scheduler under the given ceilings.
    pub fn new(limits: SchedulerLimits) -> Self {
        Self {
            queues: EventQueues::default(),
            limits,
            current: Instant::ZERO,
            started: false,
            slot_delta_cycles: 0,
            slot_zero_delays: 0,
            slot_events_executed: 0,
            failed_slot: false,
        }
    }

    #[cfg(any(feature = "veriloga", test))]
    #[doc(hidden)]
    pub fn limits(&self) -> SchedulerLimits {
        self.limits
    }

    /// Instant of the slot most recently run, or [`Instant::ZERO`] before the
    /// first slot.
    pub fn current_instant(&self) -> Instant {
        self.current
    }

    /// Earliest instant holding an event, if any.
    ///
    /// Both tiers are consulted and the earlier answer wins. The slot is
    /// asked what its events are *dated*, which is not the same question as
    /// which slot is open: `Self::open_due_slot` sets `current` to the
    /// caller's bound, and `open_next_due_instant` then fills the slot from
    /// the earliest pending instant at or before that bound. So a slot
    /// observed part-settled holds events dated before `current`, and
    /// answering with the bound would date them late — as a breakpoint, by
    /// however far the bound overshot.
    ///
    /// A slot is only observable part-settled from inside a due-slot run or
    /// after one returned an oscillation. Every XSPICE reader asks between
    /// runs, where a settled slot is empty in every region, so this reads the
    /// future tier for them exactly as it always did.
    pub fn next_instant(&self) -> Option<Instant> {
        let slot = self.queues.slot_min_instant();
        let future = self.queues.future_min_instant();
        match (slot, future) {
            (Some(slot), Some(future)) => Some(slot.min(future)),
            (slot, future) => slot.or(future),
        }
    }

    /// [`Self::next_instant`], with the driver the earliest event belongs to.
    ///
    /// The instant is the same one [`Self::next_instant`] answers with — both
    /// tiers consulted, the earlier winning — and the driver is the one a
    /// diagnostic has to name to say *whose* schedule is holding a run. Ties
    /// between the tiers go to the slot, which is where the earlier sequence
    /// sits.
    pub(crate) fn next_instant_target(&self) -> Option<(Instant, TargetId)> {
        let slot = self.queues.slot_min_event();
        let future = self.queues.future_min_event();
        match (slot, future) {
            (Some(slot), Some(future)) => Some(if future.0 < slot.0 { future } else { slot }),
            (slot, future) => slot.or(future),
        }
    }

    /// The driver of every event at [`Self::next_instant`], however many
    /// share it.
    pub(crate) fn next_instant_targets(&self) -> impl Iterator<Item = TargetId> + '_ {
        self.next_instant()
            .into_iter()
            .flat_map(|at| self.queues.targets_at(at))
    }

    /// [`Self::next_instant_target`], resolved to the driver's own identity.
    ///
    /// The interned id is an index into this scheduler's table and means
    /// nothing outside it; the instance name is what a diagnostic prints. A
    /// caller that has no target table of its own — the XSPICE queue, whose
    /// drivers *are* code-model instances — asks for the name directly.
    pub(crate) fn next_instant_instance(&self) -> Option<(Instant, &str)> {
        let (at, target) = self.next_instant_target()?;
        Some((at, self.queues.target(target).instance.as_str()))
    }

    /// Number of events not yet executed.
    pub fn pending(&self) -> usize {
        self.queues.pending()
    }

    /// Schedule an event from outside a running slot.
    ///
    /// Returns the event's sequence number, which is its place in the total
    /// order. An instant the scheduler has already left is refused: silently
    /// re-dating it would let an event appear to run before something that
    /// already ran.
    pub fn schedule_at(
        &mut self,
        at: Instant,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        let target = self.queues.intern(target);
        self.schedule_id_at(at, region, target, value)
    }

    /// The identity of a driver, interned once so that everything after can
    /// name it by index.
    ///
    /// A caller that schedules for the same driver over and over — every
    /// digital process, every code-model output — takes its id here at
    /// construction and then never touches a `String` again.
    ///
    /// Gated with its caller, the same way [`TimeResolution::
    /// seconds_to_floor_ticks`] is: `xspice::verilog` is a `veriloga` module,
    /// so a build without the feature has no caller and `-D warnings` would
    /// say so.
    #[cfg(feature = "veriloga")]
    pub(crate) fn intern_target(&mut self, target: EventTarget) -> TargetId {
        self.queues.intern(target)
    }

    /// The identity behind an interned id, for a host naming its own drivers
    /// in a diagnostic, or the transient asking which net the next event
    /// writes.
    pub(crate) fn target(&self, id: TargetId) -> &EventTarget {
        self.queues.target(id)
    }

    /// [`Self::schedule_at`] for a driver already interned.
    pub(crate) fn schedule_id_at(
        &mut self,
        at: Instant,
        region: SchedulerRegion,
        target: TargetId,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        if self.started && at <= self.current {
            return Err(SchedulerError::ScheduleInThePast {
                reached: self.current,
                requested: at,
            });
        }
        // No slot is open outside `run_time_slot`, so everything lands in the
        // future tier and is picked up when its instant opens.
        self.queues.insert(None, at, region, target, value)
    }

    /// Schedule an event that replaces this driver's own pending output.
    ///
    /// Every unexecuted event of `target` at an instant at or after `at` is
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
    /// picked up when its instant opens, because this method is reachable only
    /// from outside a running slot: an event executing inside one schedules
    /// through [`SchedulerContext`], which does not offer supersession. Placing
    /// an event straight into the open slot from out here would put it ahead of
    /// every event the drain has not opened yet, including one dated *earlier*
    /// — the interpolated-crossing case above — and that is the one ordering
    /// [`Self::run_due_events`] promises cannot happen.
    /// If the sequence space is exhausted, no pending event is changed and
    /// execution/checkpoint calls report the latched `SequenceExhausted` error.
    pub fn schedule_superseding_at(
        &mut self,
        at: Instant,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> usize {
        let target = self.queues.intern(target);
        self.schedule_id_superseding_at(at, region, target, value)
    }

    /// Cancel every pending event of a driver already interned, and return
    /// how many there were.
    #[cfg(feature = "veriloga")]
    pub(crate) fn cancel_target(&mut self, target: TargetId) -> usize {
        self.queues.supersede_driver(target, Instant::ZERO)
    }

    /// [`Self::schedule_superseding_at`] for a driver already interned.
    pub(crate) fn schedule_id_superseding_at(
        &mut self,
        at: Instant,
        region: SchedulerRegion,
        target: TargetId,
        value: EventValue,
    ) -> usize {
        if self.queues.require_sequence().is_err() {
            return 0;
        }
        let cancelled = self.queues.supersede_driver(target, at);
        // No callback or intervening allocation can consume this reservation.
        let _ = self.queues.insert(None, at, region, target, value);
        cancelled
    }

    /// Execute every event due at or before `bound`.
    ///
    /// This is the mode for an event world driven by an outer loop rather than
    /// by its own clock: the analog engine names the timepoint being settled,
    /// and everything dated at or before it is due now. Events at several
    /// distinct instants can therefore run in one call; they still run in
    /// `(instant, region, sequence)` order, so the result is the order they
    /// would have had if each instant had run its own slot.
    ///
    /// Successive calls with the same `bound` continue one slot and
    /// accumulate its accounting, which is what makes the oscillation
    /// diagnostic meaningful when the settling is driven from outside: the
    /// caller marks each iteration with [`Self::note_delta_cycle`]. A
    /// different bound opens a fresh slot. The bound may move backwards — a
    /// rejected analog step retries at a smaller timepoint — and that opens a
    /// fresh slot too.
    pub fn run_due_events<F>(
        &mut self,
        bound: Instant,
        execute: F,
    ) -> Result<TimeSlotReport, SchedulerError>
    where
        F: FnMut(ScheduledEvent, &mut SchedulerContext<'_>),
    {
        self.run_due_events_through(bound, bound, execute)
    }

    /// Drain part of an outer acceptance interval without resetting its
    /// settling budget. Timestamp-grouped publication uses this between groups.
    pub(crate) fn run_due_events_through<F>(
        &mut self,
        bound: Instant,
        through: Instant,
        mut execute: F,
    ) -> Result<TimeSlotReport, SchedulerError>
    where
        F: FnMut(ScheduledEvent, &mut SchedulerContext<'_>),
    {
        self.drain_due_through(bound, through, |queues, event, at| {
            // Rendered before the context takes the queues, which is what
            // keeps the two `String` clones off every other event path.
            let event = queues.render(event);
            let mut context = SchedulerContext {
                queues,
                current: at,
            };
            execute(event, &mut context);
        })
    }

    /// [`Self::run_due_events`] for a caller that only needs to know *which
    /// drivers* fired, in order.
    ///
    /// The digital host is that caller: it collects the slot's activations and
    /// then runs each one's process, and the identity it acts on is the id.
    /// Draining this way materializes no [`ScheduledEvent`] and so allocates
    /// nothing per event. `fired` is appended to, not cleared, so the caller
    /// owns the buffer and can reuse it across delta cycles.
    ///
    /// Nothing may be scheduled while this drains, which is exactly the
    /// difference from [`Self::run_due_events`]: this caller schedules from
    /// *outside*, after the slot has gone quiet.
    ///
    /// Gated with its caller; see [`Self::intern_target`].
    #[cfg(feature = "veriloga")]
    pub(crate) fn run_due_event_targets(
        &mut self,
        bound: Instant,
        fired: &mut Vec<TargetId>,
    ) -> Result<TimeSlotReport, SchedulerError> {
        self.drain_due(bound, |_, event, _| fired.push(event.target))
    }

    /// Account for an immediate analog-caused activation without draining any
    /// timer at the instant it is reported on.
    ///
    /// The two lanes that reach here date that instant differently, and
    /// neither answer authorizes running a timer that is merely due at it: an
    /// A/D crossing is rounded to the nearest HDL tick and then clamped
    /// forward onto the trial's high-water mark, a wake from the other event
    /// kernel takes the least tick not before it.
    #[cfg(feature = "veriloga")]
    pub(crate) fn note_external_activation(
        &mut self,
        at: Instant,
        target: TargetId,
    ) -> Result<(), SchedulerError> {
        self.check_sequence_state()?;
        self.open_due_slot(at);
        self.slot_events_executed = self
            .slot_events_executed
            .checked_add(1)
            .ok_or_else(|| self.accounting_failure())?;
        self.queues
            .note_activation(target)
            .inspect_err(|_| self.failed_slot = true)?;
        self.poll_abort(self.slot_events_executed)?;
        if self.slot_events_executed > self.limits.max_events_per_tick {
            return Err(self.oscillation(
                at,
                OscillationCause::EventLimit,
                self.slot_delta_cycles,
                self.slot_events_executed,
            ));
        }
        Ok(())
    }

    #[cfg(feature = "veriloga")]
    fn drain_due<F>(&mut self, bound: Instant, execute: F) -> Result<TimeSlotReport, SchedulerError>
    where
        F: FnMut(&mut EventQueues, PendingEvent, Instant),
    {
        self.drain_due_through(bound, bound, execute)
    }

    /// The one drain loop both due-slot modes run.
    fn drain_due_through<F>(
        &mut self,
        bound: Instant,
        through: Instant,
        mut execute: F,
    ) -> Result<TimeSlotReport, SchedulerError>
    where
        F: FnMut(&mut EventQueues, PendingEvent, Instant),
    {
        debug_assert!(through <= bound);
        self.check_sequence_state()?;
        self.open_due_slot(bound);

        loop {
            self.check_sequence_state()?;
            let Some(event) = self.queues.pop_active() else {
                if self.queues.promote() {
                    let limit = self.limits.max_delta_cycles_per_tick;
                    self.count_delta_cycle(bound, limit, limit)?;
                    continue;
                }
                // The current instant has gone quiet in every region, so the
                // next one under the bound may open. This is also what picks
                // up an event `execute` back-dated below the bound, which
                // `SchedulerContext` routes to the future tier.
                if self.queues.open_next_due_instant(through) {
                    continue;
                }
                break;
            };

            self.slot_events_executed = self
                .slot_events_executed
                .checked_add(1)
                .ok_or_else(|| self.accounting_failure())?;
            self.poll_abort(self.slot_events_executed)?;
            if self.slot_events_executed > self.limits.max_events_per_tick {
                return Err(self.oscillation(
                    bound,
                    OscillationCause::EventLimit,
                    self.slot_delta_cycles,
                    self.slot_events_executed,
                ));
            }
            self.queues
                .note_activation(event.target)
                .inspect_err(|_| self.failed_slot = true)?;
            execute(&mut self.queues, event, bound);
        }

        Ok(TimeSlotReport {
            at: bound,
            delta_cycles: self.slot_delta_cycles,
            events_executed: self.slot_events_executed,
        })
    }

    /// Record one delta cycle of the due slot bounded by `bound`.
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
    pub fn note_delta_cycle(&mut self, bound: Instant) -> Result<(), SchedulerError> {
        self.check_sequence_state()?;
        self.open_due_slot(bound);
        let limit = self.limits.max_delta_cycles_per_tick;
        self.count_delta_cycle(bound, limit, limit)
    }

    /// Count one pass of a code-model settle at `bound` that left outputs
    /// changing, refusing the settle once `passes` of them have.
    ///
    /// This is ngspice's `max_event_passes` (xspice/evt/evtiter.c), and it
    /// is counted as ngspice counts it. EVTiter runs `passes < max` and
    /// returns at the first pass that leaves no output changing, so of its
    /// `max` passes the last has to be quiet: a settle whose `max`-th pass
    /// still changed outputs is refused (evtiter.c:146-294). It is counted
    /// per settle, which `first` opens, not
    /// per instant, since one instant is settled again by every Newton
    /// iteration and every retried step that lands on it. What fired is
    /// counted again from the end of the settle's first pass, so a refusal
    /// names what went round in it rather than what earlier settles of the
    /// same instant fired.
    ///
    /// `passes` bounds the settle alone. The kernel's own limits, which bound
    /// the delta cycles of one drain and which a checkpoint records, stay as
    /// they were configured.
    pub(crate) fn note_settle_pass(
        &mut self,
        bound: Instant,
        first: bool,
        passes: u32,
    ) -> Result<(), SchedulerError> {
        self.check_sequence_state()?;
        self.open_due_slot(bound);
        if first {
            self.slot_delta_cycles = 0;
            self.slot_events_executed = 0;
            self.queues.clear_activations();
        }
        self.count_delta_cycle(bound, passes.saturating_sub(1), passes)
    }

    /// Record that the open slot resumed work deferred by a zero delay
    /// (`#0`). Returns how many such resumptions the slot has had, this one
    /// included.
    #[cfg(feature = "veriloga")]
    pub(crate) fn note_zero_delay(&mut self) -> u32 {
        self.slot_zero_delays = self.slot_zero_delays.saturating_add(1);
        self.slot_zero_delays
    }

    /// Delta cycles the open due slot has counted.
    #[cfg(feature = "veriloga")]
    pub(crate) fn slot_delta_cycles(&self) -> u32 {
        self.slot_delta_cycles
    }

    /// Zero-delay resumptions the open due slot has counted.
    #[cfg(feature = "veriloga")]
    pub(crate) fn slot_zero_delays(&self) -> u32 {
        self.slot_zero_delays
    }

    /// Refuse the open due slot at `bound` as proven never to settle: its
    /// host found the design back in a state it was in `period` delta cycles
    /// before. The report names the slot's busiest drivers, as a ceiling's
    /// does.
    #[cfg(feature = "veriloga")]
    pub(crate) fn repeated_state(&mut self, bound: Instant, period: u32) -> SchedulerError {
        self.oscillation(
            bound,
            OscillationCause::RepeatedState { period },
            self.slot_delta_cycles,
            self.slot_events_executed,
        )
    }

    /// The events due at or before `bound`, in the order they will run, each
    /// without its sequence number, written over `into`.
    ///
    /// Sequence numbers only grow, so two states of one slot never share
    /// them; what decides how the slot goes on is which events wait and in
    /// what order, and every event scheduled later runs after all of these.
    /// That is what a host compares to prove the slot repeats.
    #[cfg(feature = "veriloga")]
    pub(crate) fn due_events_into(&self, bound: Instant, into: &mut Vec<DueEvent>) {
        into.clear();
        let due = |event: &PendingEvent| DueEvent {
            at: event.at,
            region: event.region,
            target: event.target,
            value: event.value,
        };
        for region in &self.queues.slot {
            into.extend(region.iter().map(due));
        }
        // An event back-dated below the bound waits in the future tier, whose
        // slice is in heap order: sorted by the total order, it is in running
        // order. Collecting none allocates nothing, and the heap's least
        // event says when there are none without visiting the rest.
        if self
            .queues
            .future
            .peek()
            .is_none_or(|Reverse(queued)| queued.0.at > bound)
        {
            return;
        }
        let mut back_dated: Vec<&PendingEvent> = self
            .queues
            .future
            .iter()
            .map(|Reverse(queued)| &queued.0)
            .filter(|event| event.at <= bound && !self.queues.cancelled.contains(&event.sequence))
            .collect();
        back_dated.sort_by_key(|event| (event.at, event.region, event.sequence));
        into.extend(back_dated.into_iter().map(due));
    }

    /// Count one delta cycle of the open slot against its ceiling.
    /// Count one delta cycle of the open slot, refusing it past `limit` and
    /// naming `named` as the bound that tripped.
    fn count_delta_cycle(
        &mut self,
        bound: Instant,
        limit: u32,
        named: u32,
    ) -> Result<(), SchedulerError> {
        self.slot_delta_cycles = self
            .slot_delta_cycles
            .checked_add(1)
            .ok_or_else(|| self.accounting_failure())?;
        self.poll_abort(u64::from(self.slot_delta_cycles))?;
        if self.slot_delta_cycles > limit {
            let mut error = self.oscillation(
                bound,
                OscillationCause::DeltaCycleLimit,
                self.slot_delta_cycles,
                self.slot_events_executed,
            );
            if let SchedulerError::Oscillation(diagnostic) = &mut error {
                diagnostic.delta_cycle_limit = named;
            }
            return Err(error);
        }
        Ok(())
    }

    /// Ask the run's abort hook whether to stop, once every
    /// [`loop_abort::POLL_INTERVAL`] events or delta cycles of one slot.
    ///
    /// A time step that never settles holds the run inside one call, out of
    /// reach of the engine's polls between steps, as a Verilog-A loop holds a
    /// device evaluation; the same hook ends both. Between polls a slot pays
    /// one mask test per event.
    fn poll_abort(&mut self, count: u64) -> Result<(), SchedulerError> {
        if count & (u64::from(loop_abort::POLL_INTERVAL) - 1) == 0 && loop_abort::aborted() {
            self.failed_slot = true;
            return Err(SchedulerError::Aborted);
        }
        Ok(())
    }

    /// Open the due slot at `bound`, resetting per-slot accounting when
    /// the bound moves. Continuing the same bound keeps it.
    fn open_due_slot(&mut self, bound: Instant) {
        if self.started && self.current == bound {
            return;
        }
        self.current = bound;
        self.started = true;
        self.slot_delta_cycles = 0;
        self.slot_zero_delays = 0;
        self.slot_events_executed = 0;
        self.queues.clear_activations();
    }

    /// Run the next instant that has events, to quiescence.
    ///
    /// Returns `Ok(None)` when nothing is pending. Otherwise that instant's
    /// slot is iterated per IEEE 1364-2005: drain the active region, promote
    /// the first non-empty later region when it empties, and stop when every
    /// region is empty. `execute` sees each event in total order and may
    /// schedule more through its [`SchedulerContext`].
    pub fn run_time_slot<F>(
        &mut self,
        mut execute: F,
    ) -> Result<Option<TimeSlotReport>, SchedulerError>
    where
        F: FnMut(ScheduledEvent, &mut SchedulerContext<'_>),
    {
        self.check_sequence_state()?;
        let Some(at) = self.next_instant() else {
            return Ok(None);
        };

        self.current = at;
        self.started = true;
        self.queues.open_next_due_instant(at);
        self.queues.clear_activations();

        // An instant settles inside this call, so the counters are local. The
        // per-slot fields are cleared so that a scheduler driven both ways
        // does not carry one mode's accounting into the other's.
        self.slot_delta_cycles = 0;
        self.slot_zero_delays = 0;
        self.slot_events_executed = 0;
        let mut delta_cycles: u32 = 0;
        let mut events_executed: u64 = 0;

        loop {
            self.check_sequence_state()?;
            let Some(event) = self.queues.pop_active() else {
                if !self.queues.promote() {
                    break;
                }
                delta_cycles = delta_cycles
                    .checked_add(1)
                    .ok_or_else(|| self.accounting_failure())?;
                self.poll_abort(u64::from(delta_cycles))?;
                if delta_cycles > self.limits.max_delta_cycles_per_tick {
                    return Err(self.oscillation(
                        at,
                        OscillationCause::DeltaCycleLimit,
                        delta_cycles,
                        events_executed,
                    ));
                }
                continue;
            };

            events_executed = events_executed
                .checked_add(1)
                .ok_or_else(|| self.accounting_failure())?;
            self.poll_abort(events_executed)?;
            if events_executed > self.limits.max_events_per_tick {
                return Err(self.oscillation(
                    at,
                    OscillationCause::EventLimit,
                    delta_cycles,
                    events_executed,
                ));
            }
            self.queues
                .note_activation(event.target)
                .inspect_err(|_| self.failed_slot = true)?;
            let event = self.queues.render(event);
            let mut context = SchedulerContext {
                queues: &mut self.queues,
                current: at,
            };
            execute(event, &mut context);
        }

        Ok(Some(TimeSlotReport {
            at,
            delta_cycles,
            events_executed,
        }))
    }

    fn accounting_failure(&mut self) -> SchedulerError {
        self.failed_slot = true;
        SchedulerError::AccountingExhausted
    }

    fn check_sequence_state(&self) -> Result<(), SchedulerError> {
        if self.queues.sequence_exhausted {
            Err(SchedulerError::SequenceExhausted)
        } else {
            Ok(())
        }
    }

    fn oscillation(
        &mut self,
        at: Instant,
        cause: OscillationCause,
        delta_cycles: u32,
        events_executed: u64,
    ) -> SchedulerError {
        self.failed_slot = true;
        // Ordered by activation count, then by the driver's own order, so two
        // runs of the same oscillation report the same list.
        let mut entities: Vec<(EventTarget, u64)> = self
            .queues
            .activated
            .iter()
            .map(|id| {
                (
                    self.queues.target(*id).clone(),
                    self.queues.activations[id.index()],
                )
            })
            .collect();
        entities.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        entities.truncate(self.limits.max_reported_oscillating_entities);
        SchedulerError::Oscillation(OscillationDiagnostic {
            at,
            // The kernel declares no grid, so it names no tick. Whoever does
            // declare one fills this in on the way out — see
            // `DigitalHost::name_oscillating_tick`.
            tick: None,
            cause,
            delta_cycles,
            events_executed,
            delta_cycle_limit: self.limits.max_delta_cycles_per_tick,
            event_limit: self.limits.max_events_per_tick,
            entities,
            #[cfg(feature = "veriloga")]
            zero_delay_loop: None,
        })
    }
}

/// Scheduling handle handed to an executing event.
///
/// It exposes scheduling and nothing else, so an event cannot re-enter
/// [`EventScheduler::run_time_slot`] from inside a slot.
///
/// It offers one absolute schedule and no relative one. A "delay" is a number
/// of ticks on some declared grid, and the kernel declares none — so the
/// caller that owns the grid does the arithmetic and names the instant it
/// arrived at. A zero delay is [`Self::schedule_at`] at
/// [`Self::current_instant`], which is a delta event at the running instant.
#[derive(Debug)]
pub struct SchedulerContext<'a> {
    queues: &'a mut EventQueues,
    current: Instant,
}

impl SchedulerContext<'_> {
    /// The instant whose slot is running.
    pub fn current_instant(&self) -> Instant {
        self.current
    }

    /// Schedule at an absolute instant, which may be the running one.
    ///
    /// An instant before the running one is refused for the same reason as
    /// [`EventScheduler::schedule_at`].
    pub fn schedule_at(
        &mut self,
        at: Instant,
        region: SchedulerRegion,
        target: EventTarget,
        value: EventValue,
    ) -> Result<u64, SchedulerError> {
        if at < self.current {
            return Err(SchedulerError::ScheduleInThePast {
                reached: self.current,
                requested: at,
            });
        }
        let target = self.queues.intern(target);
        self.queues
            .insert(Some(self.current), at, region, target, value)
    }
}

/// The kernel's conformance tests live in `tests/xspice/event_scheduler_kernel.rs`,
/// against the published API. These are here because they pin the three
/// rounding rules of [`Instant`] against the crate-visible conversions they
/// are implemented by, which no published API reaches.
///
/// Gated on `veriloga` with two of those three rules, which are gated with
/// their only callers: `xspice::verilog`'s mixed interleave is a `veriloga`
/// module, so a build without the feature has neither the methods nor
/// anything to test them with.
#[cfg(all(test, feature = "veriloga"))]
mod tests {
    use super::*;

    /// The instant a time names, for a test that only passes times an instant
    /// exists for.
    fn instant(seconds: f64) -> Instant {
        Instant::from_seconds(seconds).expect("a schedulable time")
    }

    #[test]
    fn flooring_maps_an_off_grid_analog_time_to_the_tick_at_or_before_it() {
        let resolution = TimeResolution::new(-9).expect("1 ns");

        // The case the round-to-nearest conversion gets wrong for a mixed
        // interleave: a time three quarters of the way through a tick belongs
        // to the tick it is inside, not to the one it is closest to. Rounding
        // here would run the digital world a quarter of a nanosecond past an
        // instant the integrator has accepted.
        assert_eq!(instant(2.75e-9).nearest_tick(resolution), Ok(3));
        assert_eq!(instant(2.75e-9).floor_tick(resolution), Ok(2));
        assert_eq!(instant(2.25e-9).floor_tick(resolution), Ok(2));
        assert_eq!(Instant::ZERO.floor_tick(resolution), Ok(0));
        assert_eq!(instant(0.999e-9).floor_tick(resolution), Ok(0));

        // Monotone: a non-decreasing sequence of analog times gives a
        // non-decreasing sequence of ticks, which is what lets `advance_to` be
        // driven straight from accepted timepoints.
        let mut previous = 0u64;
        let mut seconds = 0.0f64;
        while seconds < 5.0e-9 {
            let tick = instant(seconds).floor_tick(resolution).expect("in range");
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
        assert_eq!(instant(2.4e-9).nearest_tick(resolution), Ok(2));
        assert_eq!(instant(2.4e-9).floor_tick(resolution), Ok(2));

        // Exactly the half: `f64::round` is half-away-from-zero, so a tie goes
        // to the later tick. Stated rather than discovered, because the
        // direction of the tie is what decides which slot a transition landing
        // dead centre is published into.
        assert_eq!(instant(2.5e-9).nearest_tick(resolution), Ok(3));
        assert_eq!(instant(2.5e-9).floor_tick(resolution), Ok(2));

        // Above the half: rounding moves on, flooring does not.
        assert_eq!(instant(2.6e-9).nearest_tick(resolution), Ok(3));
        assert_eq!(instant(2.6e-9).floor_tick(resolution), Ok(2));

        // And the rounding never lands more than one tick from the floor, in
        // either direction, which is what bounds how far forward a transition's
        // publication can move.
        let mut seconds = 0.0f64;
        while seconds < 5.0e-9 {
            let floor = instant(seconds).floor_tick(resolution).expect("in range");
            let nearest = instant(seconds).nearest_tick(resolution).expect("in range");
            assert!(
                nearest == floor || nearest == floor + 1,
                "{seconds:e} s floored to {floor} but rounded to {nearest}"
            );
            seconds += 3.7e-11;
        }
    }

    /// The third rule, against the other two: an instant crossing in from
    /// another event kernel is dated at the least tick not before it, so the
    /// HDL never labels a foreign event earlier than it happened.
    ///
    /// The three agree exactly on a tick boundary and differ nowhere else by
    /// more than one tick, which is what makes them substitutable nowhere and
    /// comparable everywhere.
    #[test]
    fn the_three_rounding_rules_agree_on_a_boundary_and_part_inside_a_tick() {
        let resolution = TimeResolution::new(-9).expect("1 ns");

        for tick in [0u64, 1, 2, 7, 1_000] {
            let on_grid = Instant::from_tick(tick, resolution);
            assert_eq!(on_grid.floor_tick(resolution), Ok(tick));
            assert_eq!(on_grid.nearest_tick(resolution), Ok(tick));
            assert_eq!(on_grid.ceil_tick(resolution), Ok(tick));
        }

        // Strictly inside a tick: the floor stays, the ceiling moves on, and
        // the nearest one picks whichever half the instant is in.
        assert_eq!(instant(2.4e-9).ceil_tick(resolution), Ok(3));
        assert_eq!(instant(2.5e-9).ceil_tick(resolution), Ok(3));
        assert_eq!(instant(2.6e-9).ceil_tick(resolution), Ok(3));

        let mut seconds = 0.0f64;
        while seconds < 5.0e-9 {
            let at = instant(seconds);
            let floor = at.floor_tick(resolution).expect("in range");
            let ceil = at.ceil_tick(resolution).expect("in range");
            assert!(
                ceil == floor || ceil == floor + 1,
                "{seconds:e} s floored to {floor} but ceiled to {ceil}"
            );
            seconds += 3.7e-11;
        }
    }

    /// Every HDL tick embeds in an instant exactly, and reads back as itself.
    ///
    /// This is the property the whole one-key design rests on: a module's
    /// `#5` and a code model's 100.4 ps output can share one heap because the
    /// tick is not approximated by the instant it becomes. It holds for every
    /// 64-bit tick: where consecutive ticks share one double of seconds, the
    /// rank keeps them apart and in order. Below that range a tick's instant
    /// is its image's, and each of the three readings of an analog time names
    /// the tick again.
    #[test]
    fn an_hdl_tick_embeds_in_an_instant_and_reads_back_as_itself() {
        for exponent in [-21i8, -15, -12, -9, 0, 2] {
            let resolution = TimeResolution::new(exponent).expect("declared precision");
            let mut ticks = vec![0u64, 1, 2, 3, 7, 999, 1_000, 1_001, 123_456_789];
            for centre in [1u64 << 51, 1 << 52, 1 << 53, 1 << 63, u64::MAX - 4] {
                ticks.extend(centre - 4..=centre + 4);
            }
            let mut previous = None;
            for tick in ticks {
                let at = Instant::from_tick(tick, resolution);
                assert_eq!(
                    at.seconds(),
                    resolution
                        .ticks_to_seconds(tick)
                        .expect("every tick has seconds"),
                    "1e{exponent} s tick {tick} is the seconds the grid names"
                );
                assert_eq!(at.tick(resolution), Ok(tick), "1e{exponent} s tick {tick}");
                if let Some((before, key)) = previous
                    && before < tick
                {
                    assert!(
                        key < at,
                        "1e{exponent} s ticks {before} and {tick} keep order"
                    );
                }
                previous = Some((tick, at));
                if tick < TimeResolution::SHARED_IMAGE_FLOOR {
                    assert_eq!(at, Instant::from_seconds(at.seconds()).expect("an instant"));
                    for (rule, read) in [
                        ("floor", at.floor_tick(resolution)),
                        ("nearest", at.nearest_tick(resolution)),
                        ("ceil", at.ceil_tick(resolution)),
                    ] {
                        assert_eq!(read, Ok(tick), "1e{exponent} s tick {tick} {rule}");
                    }
                }
            }
        }
    }

    /// Where the exact value of `seconds` sits in ticks of `resolution`, read
    /// off its exact decimal expansion: `{:.1100}` prints every digit a double
    /// has, so this is exact rational arithmetic done in base ten, by a route
    /// that shares nothing with [`TimeResolution::split`].
    fn decimal_split(resolution: TimeResolution, seconds: f64) -> (u128, TickFraction) {
        let text = format!("{seconds:.1100}");
        let (whole, fraction) = text.split_once('.').expect("a decimal point");
        let mut digits = format!("{whole}{fraction}");
        let point = whole.len() as i64 - i64::from(resolution.exponent);
        while point > digits.len() as i64 {
            digits.push('0');
        }
        let (whole, rest) = if point <= 0 {
            (
                String::new(),
                "0".repeat(point.unsigned_abs() as usize) + &digits,
            )
        } else {
            let point = point as usize;
            (digits[..point].to_owned(), digits[point..].to_owned())
        };
        let whole = whole.trim_start_matches('0').parse::<u128>().unwrap_or(0);
        let rest = rest.trim_end_matches('0');
        let fraction = match rest.as_bytes().first() {
            None => TickFraction::Zero,
            Some(digit) if *digit < b'5' => TickFraction::BelowHalf,
            Some(b'5') if rest.len() == 1 => TickFraction::Half,
            Some(_) => TickFraction::AboveHalf,
        };
        (whole, fraction)
    }

    /// The double nearest a decimal number of ticks, by the standard
    /// library's correctly rounded reader.
    fn decimal_image(resolution: TimeResolution, ticks: &str) -> f64 {
        format!("{ticks}e{}", resolution.exponent)
            .parse()
            .expect("a decimal")
    }

    /// The four conversions against an exact reference, over every
    /// resolution: ticks around `2^51` (the bound the one-multiply reading was
    /// proven to), `2^52` (where images start to be shared), `2^53` (where the
    /// tick count stops being an exact double), `2^63` and the top of `u64`,
    /// with each image, its neighbouring doubles, and the doubles at and
    /// around each half tick.
    #[test]
    fn tick_conversions_are_exact_against_decimal_arithmetic() {
        let mut state = 0x9E37_79B9_7F4A_7C15_u64;
        let mut random = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for exponent in -21..=2i8 {
            let resolution = TimeResolution::new(exponent).expect("a declarable resolution");
            let mut ticks = vec![0u64, 1, 2, 3, 7, 10, 999, 1_000, 12_345_678_901];
            for centre in [1u64 << 51, 1 << 52, 1 << 53, 1 << 63, u64::MAX - 3] {
                ticks.extend(centre - 3..=centre + 3);
                ticks.push(centre.saturating_add(random() % 100_000));
            }
            for _ in 0..40 {
                ticks.push(random() >> (random() % 64));
            }
            for tick in ticks {
                let image = resolution.tick_instant(tick);
                assert_eq!(
                    image.to_bits(),
                    decimal_image(resolution, &tick.to_string()).to_bits(),
                    "image of {tick} ticks of 1e{exponent} s"
                );
                let half = decimal_image(resolution, &format!("{tick}.5"));
                for seconds in [image, half] {
                    for at in [seconds.next_down(), seconds, seconds.next_up()] {
                        if at < 0.0 {
                            continue;
                        }
                        check_against_decimal(resolution, at);
                    }
                }
            }
        }
    }

    fn check_against_decimal(resolution: TimeResolution, seconds: f64) {
        let exponent = resolution.exponent;
        let (whole, fraction) = decimal_split(resolution, seconds);
        let Ok(whole) = u64::try_from(whole) else {
            assert!(resolution.seconds_to_ticks(seconds).is_err());
            assert!(resolution.seconds_to_floor_ticks(seconds).is_err());
            return;
        };
        let image = |tick: u64| decimal_image(resolution, &tick.to_string());
        // A double exactly on a tick is that tick; otherwise the half tick's
        // nearest double is the tie, and goes later, unless it is short of
        // the half and the earlier tick's own image.
        let half = decimal_image(resolution, &format!("{whole}.5"));
        let later = match fraction {
            TickFraction::Zero => false,
            TickFraction::BelowHalf => seconds >= half && image(whole) != seconds,
            TickFraction::Half | TickFraction::AboveHalf => true,
        };
        let nearest = u128::from(whole) + u128::from(later);
        assert_eq!(
            resolution.seconds_to_ticks(seconds).ok().map(u128::from),
            (nearest <= u128::from(u64::MAX)).then_some(nearest),
            "nearest tick of {seconds:e} s at 1e{exponent} s"
        );
        // The last tick whose image is at or below: the exact floor's image
        // is, and so is every later tick sharing a double with it.
        let mut floor = whole;
        while floor < u64::MAX && image(floor + 1) <= seconds {
            floor += 1;
        }
        assert_eq!(
            resolution.seconds_to_floor_ticks(seconds),
            Ok(floor),
            "floor tick of {seconds:e} s at 1e{exponent} s"
        );
        let mut ceil = u128::from(whole) + u128::from(fraction != TickFraction::Zero);
        if let Ok(mut tick) = u64::try_from(ceil) {
            while tick > 0 && image(tick - 1) >= seconds {
                tick -= 1;
            }
            ceil = u128::from(tick);
        }
        assert_eq!(
            resolution
                .seconds_to_ceil_ticks(seconds)
                .ok()
                .map(u128::from),
            (ceil <= u128::from(u64::MAX)).then_some(ceil),
            "ceil tick of {seconds:e} s at 1e{exponent} s"
        );
    }

    /// Below `2^51`, the range the one-multiply readings were proven over,
    /// the exact ones give the same ticks — except the nearest tick of a
    /// double within one ulp of a half tick, which the product's rounding
    /// used to decide. Those are the double nearest the half tick, which now
    /// always goes later, and the double just below it, which now always
    /// goes earlier; nothing else moves.
    #[test]
    fn below_two_to_the_fifty_one_only_the_half_tick_doubles_move() {
        let previous_estimate = |resolution: TimeResolution, seconds: f64| match resolution.exponent
        {
            1 => seconds / 10.0,
            2 => seconds / 100.0,
            exponent => seconds * TICKS_PER_SECOND[exponent.unsigned_abs() as usize],
        };
        let mut state = 0x2545_F491_4F6C_DD1D_u64;
        let mut moved = 0;
        for exponent in -21..=2i8 {
            let resolution = TimeResolution::new(exponent).expect("a declarable resolution");
            for _ in 0..2_000 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let tick = (state >> 13) % (1 << 51);
                let image = resolution.tick_instant(tick);
                let half = decimal_image(resolution, &format!("{tick}.5"));
                for seconds in [
                    image.next_down(),
                    image,
                    image.next_up(),
                    half.next_down(),
                    half,
                    half.next_up(),
                ] {
                    if seconds < 0.0 {
                        continue;
                    }
                    let estimate = previous_estimate(resolution, seconds);
                    let mut floor = estimate.floor() as u64;
                    while floor > 0 && resolution.one_operation_image(floor) > seconds {
                        floor -= 1;
                    }
                    while resolution.one_operation_image(floor + 1) <= seconds {
                        floor += 1;
                    }
                    assert_eq!(resolution.seconds_to_floor_ticks(seconds), Ok(floor));
                    let ceil = if resolution.one_operation_image(floor) >= seconds {
                        floor
                    } else {
                        floor + 1
                    };
                    assert_eq!(resolution.seconds_to_ceil_ticks(seconds), Ok(ceil));
                    let previous = estimate.round() as u64;
                    let nearest = resolution.seconds_to_ticks(seconds).expect("in range");
                    if nearest != previous {
                        moved += 1;
                        // The half tick this double is near, which need not
                        // be the sample's: a tick's neighbour can be the
                        // double nearest the half tick before it.
                        let half = decimal_image(resolution, &format!("{floor}.5"));
                        let expected = if seconds == half {
                            (floor + 1, floor)
                        } else {
                            assert_eq!(seconds, half.next_down(), "only half-tick doubles move");
                            (floor, floor + 1)
                        };
                        assert_eq!(
                            (nearest, previous),
                            expected,
                            "{seconds:e} s at 1e{exponent}"
                        );
                    }
                }
            }
        }
        assert!(moved > 0, "the sweep reaches the half-tick doubles");
    }

    /// An instant that is on no grid is kept as the time it is.
    ///
    /// This is the XSPICE lane's whole requirement of the key (design decision
    /// D1): a 100.4 ps code-model output on a 1 ns design is not quantized on
    /// the way in, because its instant is handed to the analog breakpoint
    /// manager unrounded.
    #[test]
    fn an_instant_off_every_grid_is_stored_bit_for_bit() {
        for seconds in [
            0.0,
            1.0e-18,
            1.0 / 3.0e9,
            2.718_281_828_459_045e-9,
            7.234_567_890_123_456e-4,
            123.456_789,
        ] {
            let at = Instant::from_seconds(seconds).expect("a schedulable time");
            assert_eq!(at.seconds().to_bits(), seconds.to_bits(), "{seconds:e} s");
        }

        // Ordering is the numeric ordering, which is what lets the heap use
        // the bits directly.
        assert!(instant(100.4e-12) < instant(1.0e-9));
        assert!(Instant::ZERO < instant(f64::MIN_POSITIVE));

        // Negative zero is the same instant as positive zero, and a time that
        // cannot be scheduled has no instant at all.
        assert_eq!(Instant::from_seconds(-0.0), Some(Instant::ZERO));
        assert_eq!(Instant::from_seconds(-1.0e-12), None);
        assert_eq!(Instant::from_seconds(f64::NAN), None);
        assert_eq!(Instant::from_seconds(f64::INFINITY), None);
    }

    #[test]
    fn flooring_a_tick_boundary_returns_that_tick_and_not_the_one_before() {
        // The error a bare division would make: an event time handed back as a
        // breakpoint, floored, must be delivered at its own tick. One ulp low
        // and the digital slot runs a whole tick late.
        for exponent in [-9i8, -12, -15] {
            let resolution = TimeResolution::new(exponent).expect("declared precision");
            for tick in [0u64, 1, 2, 3, 7, 999, 1_000, 1_001, 123_456_789] {
                let at = Instant::from_tick(tick, resolution);
                assert_eq!(
                    at.floor_tick(resolution),
                    Ok(tick),
                    "exponent {exponent} tick {tick} round trip"
                );
            }
        }
    }

    #[test]
    fn flooring_refuses_what_rounding_refuses() {
        let resolution = TimeResolution::new(-9).expect("1 ns");
        // Negative, NaN and infinite times have no instant at all, which is
        // where the refusal now sits.
        assert_eq!(Instant::from_seconds(-1.0e-9), None);
        assert_eq!(Instant::from_seconds(f64::NAN), None);
        assert_eq!(Instant::from_seconds(f64::INFINITY), None);
        assert!(
            instant(u64::MAX as f64 * 1.0e-9 * 2.0)
                .floor_tick(resolution)
                .is_err()
        );
    }

    /// `n` ulps above `at`.
    fn ulps_above(at: Instant, n: usize) -> Instant {
        let mut seconds = at.seconds();
        for _ in 0..n {
            seconds = seconds.next_up();
        }
        instant(seconds)
    }

    /// The picosecond lattice of a sigma-delta testbench at 1.000104 ms, under
    /// the floor its clock resolves there: eight ulps, 1.7e-18 s.
    fn picosecond_lattice_at_a_millisecond() -> (EventLattice, u64, Instant) {
        let resolution = TimeResolution::new(-12).expect("1 ps");
        let tick = 1_000_104_000;
        let image = Instant::from_tick(tick, resolution);
        let floor = crate::numerics::integration::clock_resolution(image.seconds());
        (EventLattice::new(resolution, floor), tick, image)
    }

    /// The coincident ceiling of an instant less than one floor past a tick
    /// image is that tick; one floor and more past it is the next one. The
    /// plain ceiling moves on at the first ulp, which is the whole tick a
    /// floor-landed wake used to lose.
    #[test]
    fn the_coincident_ceiling_keeps_an_instant_within_a_floor_of_its_tick() {
        let (lattice, tick, image) = picosecond_lattice_at_a_millisecond();
        let window = lattice.window(image.seconds());
        assert_eq!(window, lattice.floor, "a floor under the cap is the window");

        let mut k = 0;
        while ulps_above(image, k).seconds() - image.seconds() < window {
            let at = ulps_above(image, k);
            assert_eq!(at.coincident_ceil_tick(lattice), Ok(tick), "{k} ulps past");
            if k > 0 {
                assert_eq!(at.ceil_tick(lattice.resolution), Ok(tick + 1));
            }
            k += 1;
        }
        assert_eq!(k, 8, "the floor at a millisecond is eight ulps");
        assert_eq!(
            ulps_above(image, k).coincident_ceil_tick(lattice),
            Ok(tick + 1),
            "one whole floor past the image is another instant"
        );
        assert_eq!(
            instant(image.seconds() + 2.0 * window).coincident_ceil_tick(lattice),
            Ok(tick + 1)
        );
        // Below the image the ceiling already is the tick.
        let below = instant(image.seconds().next_down());
        assert_eq!(below.coincident_ceil_tick(lattice), Ok(tick));
        assert_eq!(image.coincident_ceil_tick(lattice), Ok(tick));
    }

    /// The snap is onto the lattice: idempotent, the identity on a tick image,
    /// and the identity on an instant at least one window from every image.
    #[test]
    fn the_snap_is_idempotent_and_moves_only_an_instant_within_a_floor_of_a_tick() {
        let (lattice, _, image) = picosecond_lattice_at_a_millisecond();
        let window = lattice.window(image.seconds());
        let long_ago = Instant::ZERO;

        assert_eq!(image.canonical(lattice, long_ago), image);
        for k in 1..8 {
            for at in [
                ulps_above(image, k),
                instant({
                    let mut seconds = image.seconds();
                    for _ in 0..k {
                        seconds = seconds.next_down();
                    }
                    seconds
                }),
            ] {
                let snapped = at.canonical(lattice, long_ago);
                assert_eq!(snapped, image, "{at} is {k} ulps from {image}");
                assert_eq!(snapped.canonical(lattice, long_ago), snapped);
            }
        }
        for at in [
            ulps_above(image, 8),
            instant(image.seconds() + 2.0 * window),
            instant(image.seconds() - 2.0 * window),
            instant(image.seconds() + 0.5e-12),
        ] {
            assert_eq!(at.canonical(lattice, long_ago), at, "{at} is near no tick");
            assert_eq!(
                at.canonical(lattice, long_ago).canonical(lattice, long_ago),
                at
            );
        }
    }

    /// The snap never moves a key across the instant that scheduled it. A
    /// zero-delay output at an off-lattice "now" keeps "now" on either side
    /// of the image; an instant scheduled from long before it still moves onto
    /// it; and an output a positive delay puts a rounding past a later image
    /// takes that image.
    #[test]
    fn the_snap_never_moves_a_key_before_the_instant_that_scheduled_it() {
        let (lattice, tick, image) = picosecond_lattice_at_a_millisecond();
        let resolution = lattice.resolution;

        let now = ulps_above(image, 3);
        assert_eq!(now.canonical(lattice, now), now, "zero delay keeps now");
        let later_output = ulps_above(image, 5);
        assert_eq!(
            later_output.canonical(lattice, now),
            later_output,
            "the image is before the instant that scheduled it"
        );

        let just_before = instant(image.seconds().next_down().next_down());
        assert_eq!(
            just_before.canonical(lattice, just_before),
            just_before,
            "zero delay keeps a now just short of the image too"
        );
        assert_eq!(just_before.canonical(lattice, Instant::ZERO), image);
        let negative_delay = instant(image.seconds().next_down().next_down().next_down());
        assert_eq!(
            negative_delay.canonical(lattice, just_before),
            negative_delay,
            "the image is after the instant that scheduled it"
        );

        let next = Instant::from_tick(tick + 1_000, resolution);
        let sum = instant(now.seconds() + 1.0e-9);
        assert!(
            sum != next && (sum.seconds() - next.seconds()).abs() < lattice.window(sum.seconds())
        );
        assert_eq!(sum.canonical(lattice, now), next);
    }

    /// The window is rounding-sized, never semantic: a solver floor near the
    /// precision itself — a large `DELMIN` — would otherwise quantize
    /// code-model timing onto the HDL grid. Capped at a thousandth of a tick,
    /// an output half a tick from its nearest image keeps its time, and only
    /// one within the cap moves.
    #[test]
    fn a_floor_near_the_precision_moves_no_instant_by_more_than_the_cap() {
        let resolution = TimeResolution::new(-12).expect("1 ps");
        let lattice = EventLattice::new(resolution, 0.9e-12);
        let tick = 5_000;
        let image = Instant::from_tick(tick, resolution);
        let cap = resolution.seconds_per_tick() * EventLattice::MAX_WINDOW_PER_TICK;
        assert_eq!(lattice.window(image.seconds()), cap);
        assert!((cap - 1.0e-15).abs() < 1.0e-30);

        for offset in [0.5e-12, 0.3e-12, -0.4e-12, 2.0e-15, -2.0e-15] {
            let at = instant(image.seconds() + offset);
            assert_eq!(
                at.canonical(lattice, Instant::ZERO),
                at,
                "offset {offset:e}"
            );
        }
        let near = instant(image.seconds() + 0.5e-15);
        assert_eq!(near.canonical(lattice, Instant::ZERO), image);
        assert_eq!(
            instant(image.seconds() + 0.5e-12).coincident_ceil_tick(lattice),
            Ok(tick + 1),
            "a wake half a tick late is a tick late"
        );
        assert_eq!(near.coincident_ceil_tick(lattice), Ok(tick));
    }

    /// No declared floor still leaves the clock's own resolution as the
    /// window, which is what a coupled run schedules with before its first
    /// transient step.
    #[test]
    fn a_lattice_with_no_declared_floor_resolves_the_clock() {
        let resolution = TimeResolution::new(-12).expect("1 ps");
        let image = Instant::from_tick(1_000, resolution);
        for floor in [0.0, -1.0, f64::NAN] {
            let lattice = EventLattice::new(resolution, floor);
            assert_eq!(
                lattice.window(image.seconds()),
                crate::numerics::integration::clock_resolution(image.seconds())
            );
            assert_eq!(
                ulps_above(image, 2).canonical(lattice, Instant::ZERO),
                image
            );
        }
    }
}
