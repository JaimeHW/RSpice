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
    /// Whether it holds no sample.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Sample `index`, which is below [`Self::len`].
    fn at(&self, index: usize) -> f64;
    /// Whether its samples are known to ascend strictly, so a search need not read them all to
    /// check their order.
    fn ascending(&self) -> bool {
        false
    }
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

    fn ascending(&self) -> bool {
        self.values.is_ascending_axis()
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

    fn ascending(&self) -> bool {
        self.0.ascending()
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
        Feature::Sample(direction) => step(x, y, from, direction).map(point),
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
    /// The top of the hill the marker stands on, which stands at least
    /// `size` clear of the curve either side of it. A size of zero takes
    /// any top the curve falls away from.
    Peak { size: f64 },
    /// The bottom of the basin the marker stands in, `size` deep at least.
    Valley { size: f64 },
    /// The crossing of `level` nearest the marker.
    Crossing { level: f64 },
}

/// Where a search marker placed at `seed` sits on the curve as it is now.
///
/// A peak marker climbs from the sample nearest its seed to the top of the
/// hill it is on, and a valley marker descends to the bottom of its basin,
/// so a marker placed by eye on a peak lands on that peak, and when the
/// result under it changes it finds that peak again rather than staying
/// where the old one was. It climbs past a dip no deeper than its size, so
/// noise on a flank is not the top, and the top must fall away by more than
/// that on both sides before the record ends. A crossing marker takes the crossing of its level
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
        Anchor::Peak { size } => summit(x, y, seed, size.max(0.0), |a, b| a > b),
        Anchor::Valley { size } => summit(x, y, seed, size.max(0.0), |a, b| a < b),
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
    // A stored run's time axis was checked as its run ended; walking it again would read it
    // whole.
    if x.ascending() {
        return Ok(count);
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

/// The finite sample nearest `from` strictly after or before it on an ordered axis: found by
/// bisection, so a step reads the few blocks around the cursor however long the curve.
fn step<X: Curve + ?Sized, Y: Curve + ?Sized>(
    x: &X,
    y: &Y,
    from: f64,
    direction: Direction,
) -> Option<usize> {
    if from.is_nan() {
        return None;
    }
    // The first sample whose X `beyond` holds of; every one after it holds too.
    let first = |beyond: &dyn Fn(f64) -> bool| {
        let (mut low, mut high) = (0, x.len());
        while low < high {
            let middle = low + (high - low) / 2;
            if beyond(x.at(middle)) {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        low
    };
    match direction {
        Direction::Next => (first(&|at| at > from)..x.len()).find(|&index| y.at(index).is_finite()),
        Direction::Previous => (0..first(&|at| at >= from))
            .rev()
            .find(|&index| y.at(index).is_finite()),
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
    let mut top: Option<(usize, f64)> = None;
    let mut bottom: Option<(usize, f64)> = None;
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

/// Walk from the sample nearest `seed` to the top `better` prefers: up the
/// hill it stands on and past any dip of `size` or less on the way, back
/// and forth until neither side holds a better sample before the curve
/// falls more than `size` from the best, then back to the first sample of a
/// flat top. `None` when on either side the record ends or breaks before
/// the curve falls that far.
fn summit<X: Curve + ?Sized, Y: Curve + ?Sized>(
    x: &X,
    y: &Y,
    seed: f64,
    size: f64,
    better: impl Fn(f64, f64) -> bool,
) -> Option<FeaturePoint> {
    let finite = |index: usize| y.at(index).is_finite();
    // The finite sample nearest the seed. The axis ascends, so when both samples around the
    // seed are finite it is one of them; otherwise every finite sample is a candidate.
    let distance =
        |a: &usize, b: &usize| (x.at(*a) - seed).abs().total_cmp(&(x.at(*b) - seed).abs());
    let after = partition(x, |at| at < seed);
    let around = [after.checked_sub(1), (after < x.len()).then_some(after)];
    let here = if around.iter().all(|index| index.is_none_or(finite)) {
        around.into_iter().flatten().min_by(distance)
    } else {
        (0..x.len()).filter(|index| finite(*index)).min_by(distance)
    }?;
    let last = x.len() - 1;
    let step = |index: usize, rightward: bool| {
        if rightward {
            (index < last && finite(index + 1)).then(|| index + 1)
        } else {
            (index > 0 && finite(index - 1)).then(|| index - 1)
        }
    };
    // The best sample one way from `start` before the curve falls more than
    // `size` from the best so far, and whether it fell that far.
    let walk = |start: usize, rightward: bool| {
        let mut best = start;
        let mut at = start;
        while let Some(next) = step(at, rightward) {
            at = next;
            if better(y.at(at), y.at(best)) {
                best = at;
            } else if (y.at(best) - y.at(at)).abs() > size {
                return (best, true);
            }
        }
        (best, false)
    };
    let mut top = here;
    let (top, falls_both_ways) = loop {
        let (right, falls_right) = walk(top, true);
        let (left, falls_left) = walk(right, false);
        if left == right {
            break (right, falls_right && falls_left);
        }
        top = left;
    };
    let mut top = top;
    // A flat top is held at its first sample.
    while let Some(previous) = step(top, false)
        && y.at(previous) == y.at(top)
    {
        top = previous;
    }
    falls_both_ways.then(|| FeaturePoint {
        x: x.at(top),
        y: y.at(top),
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
