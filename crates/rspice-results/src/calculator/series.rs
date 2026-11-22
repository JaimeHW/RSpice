//! The samples a reduction reads, in as many passes as it needs: a trace
//! already in memory, or a stored column read a block at a time, so a pass
//! over a run of any length holds one block and the column is never
//! collected. A window cut by `clip` is applied as the samples pass, with
//! its ends interpolated exactly as `clip` makes them.

use std::collections::VecDeque;
use std::ops::Range;

use super::evaluator::EvaluationError;
use crate::waveform::WaveformValues;

/// Where the samples come from.
#[derive(Clone, Copy)]
enum Source<'a> {
    Memory {
        x: &'a [f64],
        y: &'a [f64],
    },
    Stored {
        x: &'a WaveformValues,
        y: &'a WaveformValues,
    },
}

/// A real trace for a reduction to read in passes.
#[derive(Clone)]
pub(super) struct Series<'a> {
    source: Source<'a>,
    /// The samples a pass reads: all of them, or the run that covers a
    /// window and the sample on each side of it.
    range: Range<usize>,
    /// The window `clip` cut, low end first.
    window: Option<(f64, f64)>,
}

impl<'a> Series<'a> {
    pub(super) fn memory(x: &'a [f64], y: &'a [f64]) -> Self {
        Self {
            source: Source::Memory { x, y },
            range: 0..x.len().min(y.len()),
            window: None,
        }
    }

    pub(super) fn stored(x: &'a WaveformValues, y: &'a WaveformValues) -> Self {
        Self {
            source: Source::Stored { x, y },
            range: 0..x.len().min(y.len()),
            window: None,
        }
    }

    fn len(&self) -> usize {
        match self.source {
            Source::Memory { x, .. } => x.len(),
            Source::Stored { x, .. } => x.len(),
        }
    }

    /// What `x` a pass interpolates in between two samples: X itself, or
    /// log X on a sweep spaced by a constant ratio, as the trace is drawn,
    /// judged on the whole axis' ends as `yval` judges a wave's.
    fn along(&self) -> fn(f64) -> f64 {
        let n = self.len();
        let ends = n.checked_sub(2).filter(|_| n >= 3).and_then(|penultimate| {
            Some((
                [self.x_at(0)?, self.x_at(1)?],
                [self.x_at(penultimate)?, self.x_at(penultimate + 1)?],
            ))
        });
        match ends {
            Some((first, last)) if crate::plot_axis::spaced_by_ratio(first, last) => f64::ln,
            _ => |x| x,
        }
    }

    fn x_at(&self, index: usize) -> Option<f64> {
        match self.source {
            Source::Memory { x, .. } => x.get(index).copied(),
            Source::Stored { x, .. } => x.sample(index),
        }
    }

    /// The first index whose x `before` does not hold for, the axis being
    /// partitioned by it, found by bisection.
    fn partition_point(&self, before: impl FnMut(f64) -> bool) -> usize {
        match self.source {
            Source::Memory { x, .. } => {
                let mut before = before;
                x.partition_point(|value| before(*value))
            }
            Source::Stored { x, .. } => x.partition_point(before),
        }
    }

    /// The part of the trace between `from` and `to` on the x-axis, as
    /// `clip(x, from, to)` cuts it: refused when an end is not a number or
    /// the window lies outside the trace. Only the samples that cover the
    /// window are read.
    pub(super) fn clipped(mut self, from: f64, to: f64) -> Result<Self, EvaluationError> {
        if !from.is_finite() || !to.is_finite() {
            return Err(EvaluationError::MathError(
                "clip start and end must be finite numbers".to_owned(),
            ));
        }
        let n = self.len();
        let (Some(first), Some(last)) = (self.x_at(0), n.checked_sub(1).and_then(|i| self.x_at(i)))
        else {
            return Err(EvaluationError::MathError(
                "clip needs a waveform with samples".to_owned(),
            ));
        };
        let (low, high) = (from.min(to), from.max(to));
        if high < first.min(last) || low > first.max(last) {
            return Err(EvaluationError::MathError(format!(
                "clip from {from} to {to} lies outside the trace, which runs from {first} to {last}"
            )));
        }
        let (start, end) = if first <= last {
            (
                self.partition_point(|at| at < low),
                self.partition_point(|at| at <= high),
            )
        } else {
            (
                self.partition_point(|at| at > high),
                self.partition_point(|at| at >= low),
            )
        };
        self.range = start.saturating_sub(1)..(end + 1).min(n);
        self.window = Some((low, high));
        Ok(self)
    }

    /// The first and last x a pass reads: a window's ends as `clip` cuts them, else the trace's
    /// own. `None` for a trace with no samples.
    pub(super) fn ends(&self) -> Option<(f64, f64)> {
        if self.window.is_some() {
            let mut pairs = self.pairs();
            let first = pairs.next()?.0;
            return Some((first, pairs.last().map_or(first, |(x, _)| x)));
        }
        let last = self.range.end.checked_sub(1)?;
        (self.range.start <= last).then_some(())?;
        Some((self.x_at(self.range.start)?, self.x_at(last)?))
    }

    /// Refuse an x-axis that runs backwards, as [`super::functions::forward_domain`] refuses
    /// one in memory. A stored time axis was checked as its run ended, so it is not read again.
    pub(super) fn forward(&self, name: &str) -> Result<(), EvaluationError> {
        match self.source {
            Source::Memory { x, .. } if self.window.is_none() => {
                return super::functions::forward_domain(name, &x[self.range.clone()]);
            }
            Source::Stored { x, .. } if self.window.is_none() && x.is_ascending_axis() => {
                return Ok(());
            }
            _ => {}
        }
        let mut previous: Option<f64> = None;
        for (x, _) in self.pairs() {
            if let Some(before) = previous
                && x < before
            {
                return super::functions::forward_domain(name, &[before, x]);
            }
            previous = Some(x);
        }
        Ok(())
    }

    /// The trace's value where its x-axis reads `at`, interpolated linearly between the samples
    /// either side, as [`super::functions::interpolate`] reads a trace in memory: the first
    /// sample at or past `at` and the one before it. A stored time axis is searched by
    /// bisection, so only the samples around `at` are read.
    pub(super) fn value_at(&self, at: f64) -> Option<f64> {
        let along = if at > 0.0 { self.along() } else { |x| x };
        if let Source::Stored { x, y } = self.source
            && self.window.is_none()
            && x.is_ascending_axis()
        {
            let range = self.range.clone();
            let after = x
                .partition_point(|value| value < at)
                .clamp(range.start, range.end);
            if after == range.end {
                return y.sample(range.end.checked_sub(1)?);
            }
            let (x1, y1) = (x.sample(after)?, y.sample(after)?);
            if after == range.start {
                return Some(y1);
            }
            let (x0, y0) = (x.sample(after - 1)?, y.sample(after - 1)?);
            let dx = along(x1) - along(x0);
            return Some(if dx == 0.0 {
                y1
            } else {
                y0 + (y1 - y0) * (along(at) - along(x0)) / dx
            });
        }
        let (first, last) = self.ends()?;
        let descending = first > last;
        let before = |a: f64, b: f64| if descending { a >= b } else { a <= b };
        let mut previous: Option<(f64, f64)> = None;
        let mut final_value = None;
        for (x1, y1) in self.pairs() {
            match previous {
                None if before(at, x1) => return Some(y1),
                Some((x0, y0)) if before(at, x1) => {
                    let dx = along(x1) - along(x0);
                    return Some(if dx == 0.0 {
                        y1
                    } else {
                        y0 + (y1 - y0) * (along(at) - along(x0)) / dx
                    });
                }
                _ => {}
            }
            previous = Some((x1, y1));
            final_value = Some(y1);
        }
        final_value
    }

    /// One pass over the samples as (x, y), a window's ends interpolated.
    /// A stored sample that cannot be read is NaN, an undefined sample, and
    /// its failure is kept for the run's diagnostics.
    pub(super) fn pairs(&self) -> Box<dyn Iterator<Item = (f64, f64)> + 'a> {
        let range = self.range.clone();
        let samples: Box<dyn Iterator<Item = (f64, f64)> + 'a> = match self.source {
            Source::Memory { x, y } => Box::new(
                x[range.clone()]
                    .iter()
                    .copied()
                    .zip(y[range].iter().copied()),
            ),
            Source::Stored { x, y } => Box::new(x.samples(range.clone()).zip(y.samples(range))),
        };
        match self.window {
            None => samples,
            Some((low, high)) => Box::new(Clipped::new(samples, low, high)),
        }
    }
}

/// The samples of a pass between two x points, with an end that falls
/// strictly between two samples made a sample of its own, interpolated
/// linearly, in sweep order: what `clip` keeps.
pub(super) struct Clipped<I> {
    samples: I,
    low: f64,
    high: f64,
    previous: Option<(f64, f64)>,
    ready: VecDeque<(f64, f64)>,
}

impl<I> Clipped<I> {
    pub(super) fn new(samples: I, low: f64, high: f64) -> Self {
        Self {
            samples,
            low,
            high,
            previous: None,
            ready: VecDeque::with_capacity(3),
        }
    }
}

impl<I: Iterator<Item = (f64, f64)>> Iterator for Clipped<I> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<(f64, f64)> {
        loop {
            if let Some(sample) = self.ready.pop_front() {
                return Some(sample);
            }
            let (x1, y1) = self.samples.next()?;
            if let Some((x0, y0)) = self.previous {
                let mut ends = [self.low, self.high];
                if x1 < x0 {
                    ends.reverse();
                }
                let ends = if self.low == self.high {
                    &ends[..1]
                } else {
                    &ends[..]
                };
                for &end in ends {
                    if (x0 < end && end < x1) || (x1 < end && end < x0) {
                        self.ready
                            .push_back((end, y0 + (y1 - y0) * (end - x0) / (x1 - x0)));
                    }
                }
            }
            if x1 >= self.low && x1 <= self.high {
                self.ready.push_back((x1, y1));
            }
            self.previous = Some((x1, y1));
        }
    }
}
