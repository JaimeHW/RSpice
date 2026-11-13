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

/// The excursion a peak needs on `y`: [`PEAK_EXCURSION_SHARE`] of its finite
/// span.
#[must_use]
pub fn default_excursion(y: &[f64]) -> f64 {
    let (low, high) = y
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        });
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
pub fn find(
    x: &[f64],
    y: &[f64],
    from: f64,
    feature: Feature,
    excursion: f64,
) -> Result<FeaturePoint, SearchRefusal> {
    let count = ordered_len(x, y)?;
    let (x, y) = (&x[..count], &y[..count]);
    let point = |index: usize| FeaturePoint {
        x: x[index],
        y: y[index],
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
            let finite = (0..count).filter(|index| y[*index].is_finite());
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
pub fn nearest(x: &[f64], y: &[f64], seed: f64, anchor: Anchor) -> Option<FeaturePoint> {
    let count = ordered_len(x, y).ok()?;
    let (x, y) = (&x[..count], &y[..count]);
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
pub fn crossings(x: &[f64], y: &[f64], level: f64) -> Vec<f64> {
    let mut found = Vec::new();
    let mut previous: Option<(usize, f64)> = None;
    for i in 0..x.len().min(y.len()) {
        let delta = y[i] - level;
        if !delta.is_finite() {
            previous = None;
            continue;
        }
        if delta == 0.0 {
            continue;
        }
        let sign = if delta > 0.0 { 1.0 } else { -1.0 };
        if let Some((before, previous_sign)) = previous
            && previous_sign != sign
        {
            found.push(if i == before + 1 {
                x[before] + (level - y[before]) * (x[i] - x[before]) / (y[i] - y[before])
            } else {
                x[before + 1]
            });
        }
        previous = Some((i, sign));
    }
    found
}

/// How many leading samples are searchable, or why none is.
fn ordered_len(x: &[f64], y: &[f64]) -> Result<usize, SearchRefusal> {
    let count = x.len().min(y.len());
    if !y[..count].iter().any(|value| value.is_finite()) {
        return Err(SearchRefusal::NoSamples);
    }
    for sample in 0..count {
        let ordered = x[sample].is_finite() && (sample == 0 || x[sample] >= x[sample - 1]);
        if !ordered {
            return Err(SearchRefusal::Unordered { sample });
        }
    }
    Ok(count)
}

/// The first finite sample no other finite sample beats.
fn extreme(y: &[f64], beats: impl Fn(f64, f64) -> bool) -> Option<usize> {
    let mut best: Option<usize> = None;
    for (index, value) in y.iter().enumerate() {
        if value.is_finite() && best.is_none_or(|best| beats(*value, y[best])) {
            best = Some(index);
        }
    }
    best
}

/// The candidate sample nearest `from` strictly in `direction`.
fn pick(
    candidates: impl DoubleEndedIterator<Item = usize>,
    x: &[f64],
    from: f64,
    direction: Direction,
) -> Option<usize> {
    let mut candidates = candidates;
    match direction {
        Direction::Next => candidates.find(|index| x[*index] > from),
        Direction::Previous => candidates.rev().find(|index| x[*index] < from),
    }
}

/// The peaks (`peaks`) or valleys of `y`, each confirmed by the curve turning
/// back from it by more than `excursion`.
///
/// One pass with hysteresis: the running top is a peak once the curve falls
/// more than `excursion` below it, and the running bottom a valley once it
/// rises more than `excursion` above it. The running top only moves on a
/// strictly higher sample, so a flat top is held at its first sample.
fn extrema(y: &[f64], excursion: f64, peaks: bool) -> std::vec::IntoIter<usize> {
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
    let (mut top, mut bottom): (Option<usize>, Option<usize>) = (None, None);
    let keep = |index: usize, is_peak: bool, found: &mut Vec<usize>| {
        if is_peak == peaks && index != 0 && index != last {
            found.push(index);
        }
    };
    for (index, value) in y.iter().copied().enumerate() {
        if !value.is_finite() {
            // A hole ends the curve; what follows is searched afresh.
            heading = Heading::Unknown;
            (top, bottom) = (None, None);
            continue;
        }
        let (Some(high), Some(low)) = (top, bottom) else {
            (top, bottom) = (Some(index), Some(index));
            continue;
        };
        if value > y[high] {
            top = Some(index);
        }
        if value < y[low] {
            bottom = Some(index);
        }
        match heading {
            Heading::Unknown | Heading::Rising if y[high] - value > excursion => {
                if matches!(heading, Heading::Rising) || high > low {
                    keep(high, true, &mut found);
                }
                heading = Heading::Falling;
                bottom = Some(index);
            }
            Heading::Unknown | Heading::Falling if value - y[low] > excursion => {
                if matches!(heading, Heading::Falling) || low > high {
                    keep(low, false, &mut found);
                }
                heading = Heading::Rising;
                top = Some(index);
            }
            _ => {}
        }
    }
    found.into_iter()
}

/// Walk from the sample nearest `seed` to the neighbour `better` prefers
/// until none is, then back to the first sample of a flat top. `None` when
/// the walk ends on the first or last sample.
fn climb(
    x: &[f64],
    y: &[f64],
    seed: f64,
    better: impl Fn(f64, f64) -> bool,
) -> Option<FeaturePoint> {
    let finite = |index: usize| y[index].is_finite();
    let mut here = (0..x.len())
        .filter(|index| finite(*index))
        .min_by(|a, b| (x[*a] - seed).abs().total_cmp(&(x[*b] - seed).abs()))?;
    let last = x.len() - 1;
    let right = |here: usize| (here < last && finite(here + 1)).then(|| here + 1);
    let left = |here: usize| (here > 0 && finite(here - 1)).then(|| here - 1);
    let step = match (right(here), left(here)) {
        (Some(r), Some(l)) if better(y[r], y[here]) && better(y[l], y[here]) => {
            if better(y[r], y[l]) { 1 } else { -1 }
        }
        (Some(r), _) if better(y[r], y[here]) => 1,
        (_, Some(l)) if better(y[l], y[here]) => -1,
        _ => 0,
    };
    loop {
        let next = match step {
            1 => right(here),
            -1 => left(here),
            _ => None,
        };
        match next {
            Some(next) if better(y[next], y[here]) => here = next,
            _ => break,
        }
    }
    // A flat top is held at its first sample.
    while let Some(previous) = left(here)
        && y[previous] == y[here]
    {
        here = previous;
    }
    // The walk must have come to a top the curve leaves on both sides.
    let rises_after = (here + 1..x.len())
        .take_while(|index| finite(*index))
        .find(|index| y[*index] != y[here])
        .is_some_and(|index| better(y[here], y[index]));
    let rises_before = left(here).is_some_and(|previous| better(y[here], y[previous]));
    (here != 0 && rises_before && rises_after).then(|| FeaturePoint {
        x: x[here],
        y: y[here],
    })
}

#[cfg(test)]
mod tests;
