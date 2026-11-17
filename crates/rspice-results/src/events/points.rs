//! An event history's points: in memory, or a signal a results store holds.
//!
//! A transient written to a results store keeps its digital and real event histories there. A
//! stored history holds the signal instead of its points and reads them all the first time
//! something asks for them; a view reads only the changes over its window, the one held into
//! it and the one after it ([`EventPoints::around`]), and a reader that asks how many there are
//! reads nothing.

use std::fmt;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

/// What a store's event signal holds, as an [`EventHistorySource`] hands it out.
pub use rspice_results_store::{EventChange, EventSpan, EventValue, Logic};

use crate::waveform::ColumnReadFailure;

/// One point of an event history, as a store's change of its signal states it.
pub trait EventPoint: Copy + Send + Sync + 'static {
    /// When the change was committed.
    fn time(&self) -> f64;

    /// The point a store's change is, `None` when the change is not a point of this kind of
    /// history.
    fn from_change(change: &EventChange) -> Option<Self>;
}

/// An event signal kept outside memory and read on demand: one digital or real event history
/// of a run in a results store.
pub trait EventHistorySource: Send + Sync {
    /// The history's node, as a failure to read it names it.
    fn name(&self) -> &str;

    /// The store that holds it, as its host names it (a segment's path).
    fn store(&self) -> String;

    /// How many changes the signal holds.
    fn count(&self) -> Result<u64, String>;

    /// How many changes come before time `t`.
    fn before(&self, t: f64) -> Result<u64, String>;

    /// The changes with time in `t0 ..= t1`, with the last change before `t0` and the first
    /// after `t1`.
    fn span(&self, t0: f64, t1: f64) -> Result<EventSpan, String>;

    /// The logical digest of the store that holds the signal and the signal's index in it: a
    /// signal with the same identity holds the same changes. `None` when the store states no
    /// digest.
    fn identity(&self) -> Option<([u8; 32], u64)> {
        None
    }
}

/// An event history's points.
///
/// It dereferences to the points, reading a stored history the first time; a history that
/// cannot be read shows no points and its failure goes to
/// [`crate::waveform::take_column_read_failures`]. [`Self::len`] never reads.
pub struct EventPoints<P>(Repr<P>);

enum Repr<P> {
    Memory(Arc<Vec<P>>),
    Stored(Arc<Stored<P>>),
}

struct Stored<P> {
    source: Arc<dyn EventHistorySource>,
    /// The history keeps the signal's changes at or after this time: the run's output window.
    from: f64,
    len: usize,
    points: OnceLock<Vec<P>>,
    failure: OnceLock<ColumnReadFailure>,
}

impl<P: EventPoint> Stored<P> {
    fn failed(&self, cause: String) -> ColumnReadFailure {
        ColumnReadFailure {
            column: self.source.name().to_owned(),
            store: self.source.store(),
            cause,
        }
    }

    /// Keep the history's first failure, and hand it to diagnostics once.
    fn fail(&self, failure: ColumnReadFailure) -> ColumnReadFailure {
        if self.failure.set(failure.clone()).is_ok() {
            crate::waveform::report_column_read_failure(failure.clone());
        }
        failure
    }

    fn points_of(&self, changes: impl IntoIterator<Item = EventChange>) -> Result<Vec<P>, String> {
        changes
            .into_iter()
            .map(|change| {
                P::from_change(&change).ok_or_else(|| {
                    format!(
                        "the store holds a change at {:e} s this history cannot hold",
                        change.time
                    )
                })
            })
            .collect()
    }

    fn read(&self) -> Result<Vec<P>, ColumnReadFailure> {
        let span = self
            .source
            .span(self.from, f64::INFINITY)
            .map_err(|cause| self.failed(cause))?;
        self.points_of(span.within)
            .map_err(|cause| self.failed(cause))
    }

    fn points(&self) -> &Vec<P> {
        self.points.get_or_init(|| {
            if self.failure.get().is_some() {
                return Vec::new();
            }
            self.read().unwrap_or_else(|failure| {
                self.fail(failure);
                Vec::new()
            })
        })
    }

    fn around(
        &self,
        t0: f64,
        t1: f64,
        same: &dyn Fn(&P, &P) -> bool,
    ) -> Result<Vec<P>, ColumnReadFailure> {
        if let Some(points) = self.points.get() {
            return Ok(around(points, t0, t1, same));
        }
        if let Some(failure) = self.failure.get() {
            return Err(failure.clone());
        }
        self.read_around(t0, t1, same)
            .map_err(|cause| self.fail(self.failed(cause)))
    }

    /// [`around`] read from the source: the window's frames, then back from the change before
    /// it and on from the change after it, over spans that double until each side finds where
    /// the value changes.
    fn read_around(
        &self,
        t0: f64,
        t1: f64,
        same: &dyn Fn(&P, &P) -> bool,
    ) -> Result<Vec<P>, String> {
        let lo = t0.max(self.from);
        let (before, mut out, after) = if t1 < lo {
            // The window ends before the history starts: what comes after it starts there.
            let span = self.source.span(self.from, self.from)?;
            let first = span.within.into_iter().chain(span.after).next();
            (None, Vec::new(), first)
        } else {
            let span = self.source.span(lo, t1)?;
            let before = span.before.filter(|change| change.time >= self.from);
            (before, self.points_of(span.within)?, span.after)
        };
        let width = |from: f64| {
            (t1 - t0)
                .max(from)
                .max(f64::EPSILON * t0.abs())
                .max(f64::MIN_POSITIVE)
        };
        let mut held = out.last().copied();
        if let Some(before) = before {
            let before = self.points_of([before])?[0];
            held = held.or(Some(before));
            // Back over the points that hold the value held into the window, to its change.
            let mut buffer = vec![before];
            let mut span_width = width(t0 - before.time());
            let start = loop {
                let (at, found) = run_start(&buffer, buffer.len() - 1, same);
                if found {
                    break buffer[at];
                }
                let edge = buffer[0].time();
                let span = self.source.span((edge - span_width).max(self.from), edge)?;
                let earlier = span
                    .before
                    .filter(|change| change.time >= self.from)
                    .into_iter()
                    .chain(span.within.into_iter().filter(|change| change.time < edge));
                let mut earlier = self.points_of(earlier)?;
                if earlier.is_empty() {
                    break buffer[at];
                }
                earlier.append(&mut buffer);
                buffer = earlier;
                span_width *= 2.0;
            };
            out.insert(0, start);
        }
        if let Some(after) = after {
            // On to the first instant whose value is not the one held at the window's end.
            let mut from = after.time;
            let mut span_width = width(from - t1);
            loop {
                let span = self.source.span(from, from + span_width)?;
                let instants = self.points_of(span.within)?;
                let change = next_change(&instants, 0, held, same);
                if !change.is_empty() {
                    out.extend_from_slice(change);
                    break;
                }
                let Some(next) = span.after else {
                    break;
                };
                from = next.time;
                span_width *= 2.0;
            }
        }
        Ok(out)
    }
}

/// The points of `points` (in time order) over `t0 ..= t1`, with what a lane needs on each side
/// to read as the whole history does: the point where the value held into the window was set
/// (the change before it, `same` telling which points hold one value) and the points of the
/// first instant after `t1` whose value is not the one held at `t1`.
fn around<P: EventPoint>(points: &[P], t0: f64, t1: f64, same: &dyn Fn(&P, &P) -> bool) -> Vec<P> {
    let start = points.partition_point(|point| point.time() < t0);
    let end = points
        .partition_point(|point| point.time() <= t1)
        .max(start);
    let mut out = Vec::with_capacity(end - start + 2);
    if let Some(held) = start.checked_sub(1) {
        out.push(points[run_start(points, held, same).0]);
    }
    out.extend_from_slice(&points[start..end]);
    let held = end.checked_sub(1).map(|at| points[at]);
    out.extend_from_slice(next_change(points, end, held, same));
    out
}

/// Where the run of instants holding `points[held]`'s value starts: the last point of its first
/// instant, and whether the instant before it is in `points` (else the run may reach further
/// back than `points` do).
fn run_start<P: EventPoint>(
    points: &[P],
    held: usize,
    same: &dyn Fn(&P, &P) -> bool,
) -> (usize, bool) {
    let mut at = held;
    loop {
        let instant = points[at].time();
        let first = points[..at]
            .iter()
            .rposition(|point| point.time() != instant)
            .map_or(0, |before| before + 1);
        let Some(previous) = first.checked_sub(1) else {
            return (at, false);
        };
        if !same(&points[previous], &points[held]) {
            return (at, true);
        }
        at = previous;
    }
}

/// The points of the first instant at or after `points[from]` whose last point does not hold
/// `held`'s value (every instant when nothing is held); empty when there is none.
fn next_change<'a, P: EventPoint>(
    points: &'a [P],
    from: usize,
    held: Option<P>,
    same: &dyn Fn(&P, &P) -> bool,
) -> &'a [P] {
    let mut at = from;
    while at < points.len() {
        let instant = points[at].time();
        let last = at
            + points[at..]
                .iter()
                .take_while(|point| point.time() == instant)
                .count()
            - 1;
        if held.is_none_or(|held| !same(&points[last], &held)) {
            return &points[at..=last];
        }
        at = last + 1;
    }
    &[]
}

impl<P: EventPoint> EventPoints<P> {
    /// The changes of `source` at or after `from`. Counting them reads at most the one frame of
    /// the signal that spans `from`.
    pub fn stored(source: Arc<dyn EventHistorySource>, from: f64) -> Result<Self, String> {
        let count = source.count()?;
        let before = if from > f64::NEG_INFINITY {
            source.before(from)?
        } else {
            0
        };
        let len = usize::try_from(count.saturating_sub(before))
            .map_err(|_| "the history holds more changes than this host can address".to_owned())?;
        Ok(Self(Repr::Stored(Arc::new(Stored {
            source,
            from,
            len,
            points: OnceLock::new(),
            failure: OnceLock::new(),
        }))))
    }

    /// How many points there are, without reading a stored history.
    pub fn len(&self) -> usize {
        match &self.0 {
            Repr::Memory(points) => points.len(),
            Repr::Stored(stored) => stored.len,
        }
    }

    /// Whether there are no points, without reading a stored history.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the points are a stored signal's.
    pub fn is_stored(&self) -> bool {
        matches!(self.0, Repr::Stored(_))
    }

    /// The identity of the stored signal the points are and the time they start from, when they
    /// are one whose store states it: what a digest of the points can hash in place of reading
    /// them.
    pub fn stored_identity(&self) -> Option<(([u8; 32], u64), f64)> {
        match &self.0 {
            Repr::Memory(_) => None,
            Repr::Stored(stored) => Some((stored.source.identity()?, stored.from)),
        }
    }

    /// The points, read from a stored history the first time, or why it could not be read.
    pub fn try_points(&self) -> Result<&Vec<P>, ColumnReadFailure> {
        match &self.0 {
            Repr::Memory(points) => Ok(points),
            Repr::Stored(stored) => {
                let points = stored.points();
                match stored.failure.get() {
                    Some(failure) => Err(failure.clone()),
                    None => Ok(points),
                }
            }
        }
    }

    /// The points over `t0 ..= t1` in time order, with what a lane drawn from them needs on
    /// each side to read there as the whole history does: the point where the value held into
    /// the window was set, and the points of the first instant after `t1` whose value is not
    /// the one held at `t1`. `same` says which points hold one value: a lane's, not every
    /// change's (a digital change of strength alone keeps its level).
    ///
    /// A stored history reads the frames over the window and on each side only as far as the
    /// value changes, and keeps nothing.
    pub fn around(
        &self,
        t0: f64,
        t1: f64,
        same: impl Fn(&P, &P) -> bool,
    ) -> Result<Vec<P>, ColumnReadFailure> {
        let (t0, t1) = (t0.min(t1), t0.max(t1));
        match &self.0 {
            Repr::Memory(points) => Ok(around(points, t0, t1, &same)),
            Repr::Stored(stored) => stored.around(t0, t1, &same),
        }
    }

    /// The same history as points of another kind: each point mapped by `map` in memory, the
    /// same stored signal read as `Q`.
    pub fn map<Q: EventPoint>(self, map: impl Fn(&P) -> Q) -> EventPoints<Q> {
        match self.0 {
            Repr::Memory(points) => points.iter().map(map).collect(),
            Repr::Stored(stored) => EventPoints(Repr::Stored(Arc::new(Stored {
                source: Arc::clone(&stored.source),
                from: stored.from,
                len: stored.len,
                points: OnceLock::new(),
                failure: stored
                    .failure
                    .get()
                    .cloned()
                    .map_or_else(OnceLock::new, |failure| {
                        let held = OnceLock::new();
                        let _ = held.set(failure);
                        held
                    }),
            }))),
        }
    }

    /// The points to change, held in memory from now on.
    pub fn make_mut(&mut self) -> &mut Vec<P> {
        if let Repr::Stored(stored) = &self.0 {
            self.0 = Repr::Memory(Arc::new(stored.points().clone()));
        }
        match &mut self.0 {
            Repr::Memory(points) => Arc::make_mut(points),
            Repr::Stored(_) => unreachable!("a stored history was just read into memory"),
        }
    }

    /// Whether both are the same points without comparing them: one allocation, or one stored
    /// history.
    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (&left.0, &right.0) {
            (Repr::Memory(left), Repr::Memory(right)) => Arc::ptr_eq(left, right),
            (Repr::Stored(left), Repr::Stored(right)) => Arc::ptr_eq(left, right),
            _ => false,
        }
    }
}

impl<P: EventPoint> Deref for EventPoints<P> {
    type Target = Vec<P>;

    fn deref(&self) -> &Vec<P> {
        match &self.0 {
            Repr::Memory(points) => points,
            Repr::Stored(stored) => stored.points(),
        }
    }
}

impl<P> Clone for EventPoints<P> {
    fn clone(&self) -> Self {
        Self(match &self.0 {
            Repr::Memory(points) => Repr::Memory(Arc::clone(points)),
            Repr::Stored(stored) => Repr::Stored(Arc::clone(stored)),
        })
    }
}

impl<P> Default for EventPoints<P> {
    fn default() -> Self {
        Self(Repr::Memory(Arc::new(Vec::new())))
    }
}

impl<P> From<Vec<P>> for EventPoints<P> {
    fn from(points: Vec<P>) -> Self {
        Self(Repr::Memory(Arc::new(points)))
    }
}

impl<P> FromIterator<P> for EventPoints<P> {
    fn from_iter<T: IntoIterator<Item = P>>(points: T) -> Self {
        Self::from(points.into_iter().collect::<Vec<_>>())
    }
}

impl<'a, P: EventPoint> IntoIterator for &'a EventPoints<P> {
    type Item = &'a P;
    type IntoIter = std::slice::Iter<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<P: EventPoint + PartialEq> PartialEq for EventPoints<P> {
    fn eq(&self, other: &Self) -> bool {
        // Lengths first: a stored history of another length is unequal without being read.
        Self::ptr_eq(self, other) || (self.len() == other.len() && **self == **other)
    }
}

impl<P: EventPoint + fmt::Debug> fmt::Debug for EventPoints<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Repr::Memory(points) => points.fmt(f),
            Repr::Stored(stored) => write!(
                f,
                "EventPoints({} changes of '{}' in {} from {:e} s)",
                stored.len,
                stored.source.name(),
                stored.source.store(),
                stored.from
            ),
        }
    }
}

impl<P: EventPoint + serde::Serialize> serde::Serialize for EventPoints<P> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_slice().serialize(serializer)
    }
}

impl<'de, P: serde::Deserialize<'de>> serde::Deserialize<'de> for EventPoints<P> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<P>::deserialize(deserializer).map(Self::from)
    }
}

macro_rules! digital_point {
    ($point:ty) => {
        impl EventPoint for $point {
            fn time(&self) -> f64 {
                self.time_s
            }

            fn from_change(change: &EventChange) -> Option<Self> {
                let rspice_results_store::EventValue::Logic(_) = change.value else {
                    return None;
                };
                let value =
                    rspice_core::xspice::DigitalValue::from_store_strength(change.strength)?;
                if !change.time.is_finite() {
                    return None;
                }
                Some(Self {
                    time_s: change.time,
                    value_code: value.event_code(),
                })
            }
        }
    };
}

macro_rules! real_point {
    ($point:ty) => {
        impl EventPoint for $point {
            fn time(&self) -> f64 {
                self.time_s
            }

            fn from_change(change: &EventChange) -> Option<Self> {
                match change.value {
                    rspice_results_store::EventValue::Real(value)
                        if change.time.is_finite() && value.is_finite() =>
                    {
                        Some(Self {
                            time_s: change.time,
                            value,
                        })
                    }
                    _ => None,
                }
            }
        }
    };
}

digital_point!(crate::simulation_values::DigitalEventPoint);
digital_point!(super::DigitalEventPointEvidence);
real_point!(crate::simulation_values::RealEventPoint);
real_point!(super::RealEventPointEvidence);

#[cfg(test)]
mod tests;
