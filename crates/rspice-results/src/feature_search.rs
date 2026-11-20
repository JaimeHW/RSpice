//! Where a cursor or a search marker lands on a sampled curve: its maximum
//! and minimum, its peaks and valleys, the crossings of a level, and its
//! samples.
//!
//! # Conventions
//!
//! - A search reads the retained samples the reader is looking at and lands
//!   on a point of that curve. An extreme, a peak or a valley is a retained
//!   sample; a crossing is interpolated linearly between the two samples that
//!   straddle the level, exactly as the calculator's `cross()` interpolates
//!   it, so a cursor moved to the nth crossing reads the number `cross(w,
//!   level, n)` returns.
//! - X must run forwards. A sweep that turns back has more than one point at
//!   one X, and "the next peak" along it is not a question with one answer,
//!   so it is refused rather than answered along storage order.
//! - A peak is a local maximum the curve rises to and falls from by more than
//!   an excursion, and a valley the same turned over: numerical chatter on a
//!   settled node is not a feature. The first and last samples are never a
//!   peak or a valley, since the curve is not seen on both sides of them. A
//!   flat top is reported at its first sample, as a run of at-level samples
//!   is reported at its first for a crossing.
//! - A non-finite sample breaks the curve: no feature is found across a hole.

/// Which way along X a search runs from where it starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Next,
    Previous,
}

/// A feature of a curve that a cursor can be moved to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Feature {
    /// The largest sample, the first of equals.
    Maximum,
    /// The smallest sample, the first of equals.
    Minimum,
    /// The nearest peak strictly after or before the start.
    Peak(Direction),
    /// The nearest valley strictly after or before the start.
    Valley(Direction),
    /// The nearest crossing of `level`, either edge, strictly after or
    /// before the start.
    Crossing { level: f64, direction: Direction },
    /// The nearest retained sample strictly after or before the start.
    Sample(Direction),
}

/// Where a search landed: the point's X and the curve's value there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FeaturePoint {
    pub x: f64,
    pub y: f64,
}

/// Why a search found nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchRefusal {
    /// The curve holds no finite sample.
    NoSamples,
    /// X runs backwards (or is not a number) at this sample.
    Unordered { sample: usize },
    /// The curve has no such feature in the direction searched.
    NotFound,
}

impl std::fmt::Display for SearchRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSamples => f.write_str("the trace holds no finite sample"),
            Self::Unordered { sample } => write!(
                f,
                "the trace's X turns back at sample {sample}, so it has no single next point"
            ),
            Self::NotFound => f.write_str("there is no such point"),
        }
    }
}

/// The share of a curve's span a peak must rise above its neighbouring
/// valleys by, unless the caller asks for another.
///
/// One part in a thousand of what the curve spans keeps the solver's chatter
/// on a settled node from reading as peaks, and is far below any excursion a
/// reader would call a feature.
pub const PEAK_EXCURSION_SHARE: f64 = 1.0e-3;

/// A curve's samples as a search reads them: by index, mostly in order.
pub trait Curve {
    /// The samples it holds.
    fn len(&self) -> usize;
    /// Sample `index`, which is below [`Self::len`].
    fn at(&self, index: usize) -> f64;
}

impl Curve for [f64] {
    fn len(&self) -> usize {
        <[f64]>::len(self)
    }

    fn at(&self, index: usize) -> f64 {
        self[index]
    }
}

impl Curve for Vec<f64> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn at(&self, index: usize) -> f64 {
        self[index]
    }
}

impl<const N: usize> Curve for [f64; N] {
    fn len(&self) -> usize {
        N
    }

    fn at(&self, index: usize) -> f64 {
        self[index]
    }
}

/// A series read a block at a time as a search walks it, holding one block: in memory it is
/// its own slice; stored, a search over it never holds the whole column.
pub struct WindowedCurve<'a> {
    values: &'a crate::waveform::WaveformValues,
    block: std::cell::RefCell<(usize, Vec<f64>)>,
}

impl<'a> WindowedCurve<'a> {
    pub fn new(values: &'a crate::waveform::WaveformValues) -> Self {
        Self {
            values,
            block: std::cell::RefCell::new((0, Vec::new())),
        }
    }
}

impl Curve for WindowedCurve<'_> {
    fn len(&self) -> usize {
        self.values.len()
    }

    fn at(&self, index: usize) -> f64 {
        if let Some(samples) = self.values.in_memory() {
            return samples[index];
        }
        let mut block = self.block.borrow_mut();
        let (first, samples) = &mut *block;
        if !(index >= *first && index < *first + samples.len()) {
            let start = index - index % crate::waveform::BLOCK_SAMPLES;
            let end = (start + crate::waveform::BLOCK_SAMPLES).min(self.values.len());
            // An unreadable window reads as unavailable samples, which no search lands on.
            *samples = self.values.read_range(start..end).map_or_else(
                |_| vec![f64::NAN; end - start],
                std::borrow::Cow::into_owned,
            );
            *first = start;
        }
        samples[index - *first]
    }
}

/// The first `len` samples of a curve.
struct Prefix<'a, C: Curve + ?Sized>(&'a C, usize);

impl<C: Curve + ?Sized> Curve for Prefix<'_, C> {
    fn len(&self) -> usize {
        self.1
    }

    fn at(&self, index: usize) -> f64 {
        self.0.at(index)
    }
}

/// The excursion a peak needs on `y`: [`PEAK_EXCURSION_SHARE`] of its finite
/// span.
#[must_use]
pub fn default_excursion<Y: Curve + ?Sized>(y: &Y) -> f64 {
    let (low, high) = (0..y.len())
        .map(|index| y.at(index))
        .filter(|value| value.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        });
    excursion_of_span(low, high)
}

/// [`PEAK_EXCURSION_SHARE`] of a curve's finite span from `low` to `high`.
#[must_use]
pub fn excursion_of_span(low: f64, high: f64) -> f64 {
    if high > low {
        (high - low) * PEAK_EXCURSION_SHARE
    } else {
        0.0
    }
}

/// Find `feature` on the curve `(x, y)` starting from `from`.
///
/// `excursion` is how far a peak or valley must stand clear of its
/// neighbours; [`default_excursion`] is the reader's default.
pub fn find<X: Curve + ?Sized, Y: Curve + ?Sized>(
    x: &X,
    y: &Y,
    from: f64,
    feature: Feature,
    excursion: f64,
) -> Result<FeaturePoint, SearchRefusal> {
    let count = ordered_len(x, y)?;
    let (x, y) = (&Prefix(x, count), &Prefix(y, count));
    let point = |index: usize| FeaturePoint {
        x: x.at(index),
        y: y.at(index),
    };
    let found = match feature {
        Feature::Maximum => extreme(y, |candidate, best| candidate > best).map(point),
        Feature::Minimum => extreme(y, |candidate, best| candidate < best).map(point),
        Feature::Peak(direction) => {
            pick(extrema(y, excursion, true), x, from, direction).map(point)
        }
        Feature::Valley(direction) => {
            pick(extrema(y, excursion, false), x, from, direction).map(point)
        }
        Feature::Sample(direction) => {
            let finite = (0..count).filter(|index| y.at(*index).is_finite());
            pick(finite, x, from, direction).map(point)
        }
        Feature::Crossing { level, direction } => {
            let crossings = crossings(x, y, level);
            let found = match direction {
                Direction::Next => crossings.into_iter().find(|at| *at > from),
                Direction::Previous => crossings.into_iter().rev().find(|at| *at < from),
            };
            found.map(|x| FeaturePoint { x, y: level })
        }
    };
    found.ok_or(SearchRefusal::NotFound)
}

/// The feature a search marker rides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    /// The local maximum the curve climbs to from the marker.
    Peak,
    /// The local minimum the curve descends to from the marker.
    Valley,
    /// The crossing of `level` nearest the marker.
    Crossing { level: f64 },
}

/// Where a search marker placed at `seed` sits on the curve as it is now.
///
/// A peak marker climbs from the sample nearest its seed to the top of the
/// hill it is on, and a valley marker descends to the bottom of its basin,
/// so a marker placed by eye on a peak lands on that peak, and when the
/// result under it changes it finds that peak again rather than staying
/// where the old one was. A crossing marker takes the crossing of its level
/// nearest the seed. `None` when the curve has no such feature there: a peak
/// marker on a curve that only rises, or a level the curve never crosses.
pub fn nearest<X: Curve + ?Sized, Y: Curve + ?Sized>(
    x: &X,
    y: &Y,
    seed: f64,
    anchor: Anchor,
) -> Option<FeaturePoint> {
    let count = ordered_len(x, y).ok()?;
    let (x, y) = (&Prefix(x, count), &Prefix(y, count));
    match anchor {
        Anchor::Peak => climb(x, y, seed, |candidate, here| candidate > here),
        Anchor::Valley => climb(x, y, seed, |candidate, here| candidate < here),
        Anchor::Crossing { level } => crossings(x, y, level)
            .into_iter()
            .min_by(|a, b| (a - seed).abs().total_cmp(&(b - seed).abs()))
            .map(|x| FeaturePoint { x, y: level }),
    }
}

/// Every crossing of `level`, in X order.
///
/// The rule is the calculator's own: a sample sitting exactly on the level is
/// not itself a crossing, a run of at-level samples between two opposite
/// signs is reported at the first of them, and a non-finite sample breaks the
/// search. Between two straddling samples the crossing is interpolated
/// linearly, with the same expression `cross()` evaluates, so the two agree to
/// the last bit.
#[must_use]
pub fn crossings<X: Curve + ?Sized, Y: Curve + ?Sized>(x: &X, y: &Y, level: f64) -> Vec<f64> {
    let mut found = Vec::new();
    let mut previous: Option<(usize, f64, f64)> = None;
    for i in 0..x.len().min(y.len()) {
        let value = y.at(i);
        let delta = value - level;
        if !delta.is_finite() {
            previous = None;
            continue;
        }
        if delta == 0.0 {
            continue;
        }
        let sign = if delta > 0.0 { 1.0 } else { -1.0 };
        if let Some((before, before_value, previous_sign)) = previous
            && previous_sign != sign
        {
            found.push(if i == before + 1 {
                let (x0, x1) = (x.at(before), x.at(i));
                x0 + (level - before_value) * (x1 - x0) / (value - before_value)
            } else {
                x.at(before + 1)
            });
        }
        previous = Some((i, value, sign));
    }
    found
}

/// How many leading samples are searchable, or why none is.
fn ordered_len<X: Curve + ?Sized, Y: Curve + ?Sized>(x: &X, y: &Y) -> Result<usize, SearchRefusal> {
    let count = x.len().min(y.len());
    if !(0..count).any(|index| y.at(index).is_finite()) {
        return Err(SearchRefusal::NoSamples);
    }
    let mut previous = f64::NEG_INFINITY;
    for sample in 0..count {
        let here = x.at(sample);
        let ordered = here.is_finite() && (sample == 0 || here >= previous);
        if !ordered {
            return Err(SearchRefusal::Unordered { sample });
        }
        previous = here;
    }
    Ok(count)
}

/// The first finite sample no other finite sample beats.
fn extreme<Y: Curve + ?Sized>(y: &Y, beats: impl Fn(f64, f64) -> bool) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for index in 0..y.len() {
        let value = y.at(index);
        if value.is_finite() && best.is_none_or(|(_, best)| beats(value, best)) {
            best = Some((index, value));
        }
    }
    best.map(|(index, _)| index)
}

/// The candidate sample nearest `from` strictly in `direction`.
fn pick<X: Curve + ?Sized>(
    candidates: impl DoubleEndedIterator<Item = usize>,
    x: &X,
    from: f64,
    direction: Direction,
) -> Option<usize> {
    let mut candidates = candidates;
    match direction {
        Direction::Next => candidates.find(|index| x.at(*index) > from),
        Direction::Previous => candidates.rev().find(|index| x.at(*index) < from),
    }
}

/// The peaks (`peaks`) or valleys of `y`, each confirmed by the curve turning
/// back from it by more than `excursion`.
///
/// One pass with hysteresis: the running top is a peak once the curve falls
/// more than `excursion` below it, and the running bottom a valley once it
/// rises more than `excursion` above it. The running top only moves on a
/// strictly higher sample, so a flat top is held at its first sample.
fn extrema<Y: Curve + ?Sized>(y: &Y, excursion: f64, peaks: bool) -> std::vec::IntoIter<usize> {
    #[derive(Clone, Copy)]
    enum Heading {
        Unknown,
        Rising,
        Falling,
    }
    let excursion = excursion.max(0.0);
    let last = y.len().saturating_sub(1);
    let mut found = Vec::new();
    let mut heading = Heading::Unknown;
    // The running top and bottom, each as (index, value).
    let (mut top, mut bottom): (Option<(usize, f64)>, Option<(usize, f64)>) = (None, None);
    let keep = |index: usize, is_peak: bool, found: &mut Vec<usize>| {
        if is_peak == peaks && index != 0 && index != last {
            found.push(index);
        }
    };
    for index in 0..y.len() {
        let value = y.at(index);
        if !value.is_finite() {
            // A hole ends the curve; what follows is searched afresh.
            heading = Heading::Unknown;
            (top, bottom) = (None, None);
            continue;
        }
        let (Some(mut high), Some(mut low)) = (top, bottom) else {
            (top, bottom) = (Some((index, value)), Some((index, value)));
            continue;
        };
        if value > high.1 {
            high = (index, value);
        }
        if value < low.1 {
            low = (index, value);
        }
        (top, bottom) = (Some(high), Some(low));
        match heading {
            Heading::Unknown | Heading::Rising if high.1 - value > excursion => {
                if matches!(heading, Heading::Rising) || high.0 > low.0 {
                    keep(high.0, true, &mut found);
                }
                heading = Heading::Falling;
                bottom = Some((index, value));
            }
            Heading::Unknown | Heading::Falling if value - low.1 > excursion => {
                if matches!(heading, Heading::Falling) || low.0 > high.0 {
                    keep(low.0, false, &mut found);
                }
                heading = Heading::Rising;
                top = Some((index, value));
            }
            _ => {}
        }
    }
    found.into_iter()
}

/// Walk from the sample nearest `seed` to the neighbour `better` prefers
/// until none is, then back to the first sample of a flat top. `None` when
/// the walk ends on the first or last sample.
fn climb<X: Curve + ?Sized, Y: Curve + ?Sized>(
    x: &X,
    y: &Y,
    seed: f64,
    better: impl Fn(f64, f64) -> bool,
) -> Option<FeaturePoint> {
    let finite = |index: usize| y.at(index).is_finite();
    // The finite sample nearest the seed. The axis ascends, so when both samples around the
    // seed are finite it is one of them; otherwise every finite sample is a candidate.
    let distance =
        |a: &usize, b: &usize| (x.at(*a) - seed).abs().total_cmp(&(x.at(*b) - seed).abs());
    let after = partition(x, |at| at < seed);
    let around = [after.checked_sub(1), (after < x.len()).then_some(after)];
    let mut here = if around.iter().all(|index| index.is_none_or(finite)) {
        around.into_iter().flatten().min_by(distance)
    } else {
        (0..x.len()).filter(|index| finite(*index)).min_by(distance)
    }?;
    let last = x.len() - 1;
    let right = |here: usize| (here < last && finite(here + 1)).then(|| here + 1);
    let left = |here: usize| (here > 0 && finite(here - 1)).then(|| here - 1);
    let step = match (right(here), left(here)) {
        (Some(r), Some(l)) if better(y.at(r), y.at(here)) && better(y.at(l), y.at(here)) => {
            if better(y.at(r), y.at(l)) {
                1
            } else {
                -1
            }
        }
        (Some(r), _) if better(y.at(r), y.at(here)) => 1,
        (_, Some(l)) if better(y.at(l), y.at(here)) => -1,
        _ => 0,
    };
    loop {
        let next = match step {
            1 => right(here),
            -1 => left(here),
            _ => None,
        };
        match next {
            Some(next) if better(y.at(next), y.at(here)) => here = next,
            _ => break,
        }
    }
    // A flat top is held at its first sample.
    while let Some(previous) = left(here)
        && y.at(previous) == y.at(here)
    {
        here = previous;
    }
    // The walk must have come to a top the curve leaves on both sides.
    let rises_after = (here + 1..x.len())
        .take_while(|index| finite(*index))
        .find(|index| y.at(*index) != y.at(here))
        .is_some_and(|index| better(y.at(here), y.at(index)));
    let rises_before = left(here).is_some_and(|previous| better(y.at(here), y.at(previous)));
    (here != 0 && rises_before && rises_after).then(|| FeaturePoint {
        x: x.at(here),
        y: y.at(here),
    })
}

/// The first index whose sample is not `before`, on an axis that ascends.
fn partition<X: Curve + ?Sized>(x: &X, before: impl Fn(f64) -> bool) -> usize {
    let (mut low, mut high) = (0, x.len());
    while low < high {
        let middle = low + (high - low) / 2;
        if before(x.at(middle)) {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low
}

#[cfg(test)]
mod tests;
