//! Monotone cursor sampling on retained source arrays.

/// Evaluation method used for cursor readouts between accepted source points.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SampleInterpolation {
    #[default]
    MonotoneCubic,
    Linear,
    Nearest,
}

/// A series read in ascending-X order whichever way it was swept.
///
/// A reverse sweep carries exactly the same curve as its mirror; only the
/// storage order differs. Every kernel below indexes through this, so the
/// direction is handled once, at the boundary, instead of being an assumption
/// spread across a binary search, an interpolation and three slope estimates.
#[derive(Debug, Clone, Copy)]
pub struct LogicalView<'a> {
    x: &'a [f64],
    y: &'a [f64],
    n: usize,
    reversed: bool,
}

impl<'a> LogicalView<'a> {
    pub fn new(x: &'a [f64], y: &'a [f64], reversed: bool) -> Self {
        Self {
            x,
            y,
            n: x.len().min(y.len()),
            reversed,
        }
    }

    /// Number of paired samples available through this view.
    pub fn paired_len(self) -> usize {
        self.n
    }

    /// Logical index → source index.
    fn source(self, index: usize) -> usize {
        if self.reversed {
            self.n - 1 - index
        } else {
            index
        }
    }

    pub fn x(self, index: usize) -> f64 {
        self.x[self.source(index)]
    }

    pub fn y(self, index: usize) -> f64 {
        self.y[self.source(index)]
    }

    /// How many samples sit strictly below `value` — the logical index of the
    /// first sample at or above it.
    pub fn below(self, value: f64) -> usize {
        if self.reversed {
            self.n - self.x[..self.n].partition_point(|&v| v >= value)
        } else {
            self.x[..self.n].partition_point(|&v| v < value)
        }
    }

    /// How many samples sit at or below `value`.
    pub fn at_or_below(self, value: f64) -> usize {
        if self.reversed {
            self.n - self.x[..self.n].partition_point(|&v| v > value)
        } else {
            self.x[..self.n].partition_point(|&v| v <= value)
        }
    }
}

/// Linearly interpolated sample of `(x, y)` at `xq` (x monotone, either
/// direction). Clamps outside the data range.
pub fn sample_at(x: &[f64], y: &[f64], xq: f64) -> f64 {
    sample_at_with(x, y, xq, SampleInterpolation::Linear)
}

/// Evaluate a source series at `xq` using the selected cursor policy. Values
/// outside the accepted range clamp to the first/last accepted sample.
///
/// Correct for monotone `x` of either orientation: a reverse sweep reads as
/// the mirror of its ascending twin, to the bit. X that turns around — a
/// hysteresis loop, a locus — has no single answer here and belongs in
/// [`super::sample_branches_into`], which reports one value per
/// branch. A series whose first and last abscissae are equal (every closed
/// loop) is read in source order.
pub fn sample_at_with(x: &[f64], y: &[f64], xq: f64, interpolation: SampleInterpolation) -> f64 {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0.0;
    }
    let view = LogicalView::new(x, y, x[n - 1] < x[0]);
    if xq <= view.x(0) {
        return view.y(0);
    }
    if xq >= view.x(n - 1) {
        return view.y(n - 1);
    }
    let hi = view.below(xq).max(1).min(n - 1);
    let lo = hi - 1;
    let span = view.x(hi) - view.x(lo);
    if span <= 0.0 {
        return view.y(lo);
    }
    // An unrepresentable abscissa at one end of the bracket — `-inf`, which
    // `log_frequency_axis` produces for `f <= 0` and an imported AC vector
    // opening at `f = 0` reaches — makes the span infinite and the fraction
    // `inf / inf`. Every finite query in that interval sits infinitely close
    // to the finite endpoint, so that is what it reads; interpolating would
    // return NaN and carry it into a margin or a readout unannounced.
    if !span.is_finite() {
        return view.y(hi);
    }
    if matches!(interpolation, SampleInterpolation::Nearest) {
        return if xq - view.x(lo) <= view.x(hi) - xq {
            view.y(lo)
        } else {
            view.y(hi)
        };
    }
    let t = (xq - view.x(lo)) / span;
    let linear = view.y(lo) + t * (view.y(hi) - view.y(lo));
    if !matches!(interpolation, SampleInterpolation::MonotoneCubic) || n < 3 {
        return linear;
    }
    let Some(m0) = monotone_slope(view, lo) else {
        return linear;
    };
    let Some(m1) = monotone_slope(view, hi) else {
        return linear;
    };
    let t2 = t * t;
    let t3 = t2 * t;
    let value = (2.0 * t3 - 3.0 * t2 + 1.0) * view.y(lo)
        + (t3 - 2.0 * t2 + t) * span * m0
        + (-2.0 * t3 + 3.0 * t2) * view.y(hi)
        + (t3 - t2) * span * m1;
    if value.is_finite() { value } else { linear }
}

fn monotone_slope(view: LogicalView<'_>, index: usize) -> Option<f64> {
    let n = view.n;
    let secant = |left: usize, right: usize| {
        let h = view.x(right) - view.x(left);
        (h.is_finite() && h > 0.0 && view.y(left).is_finite() && view.y(right).is_finite())
            .then(|| (h, (view.y(right) - view.y(left)) / h))
    };
    if index == 0 {
        let (h0, d0) = secant(0, 1)?;
        let (h1, d1) = secant(1, 2)?;
        return Some(endpoint_slope(h0, h1, d0, d1));
    }
    if index + 1 == n {
        let (h0, d0) = secant(n - 2, n - 1)?;
        let (h1, d1) = secant(n - 3, n - 2)?;
        return Some(endpoint_slope(h0, h1, d0, d1));
    }
    let (h_previous, d_previous) = secant(index - 1, index)?;
    let (h_next, d_next) = secant(index, index + 1)?;
    if d_previous == 0.0 || d_next == 0.0 || d_previous.signum() != d_next.signum() {
        return Some(0.0);
    }
    let w1 = 2.0 * h_next + h_previous;
    let w2 = h_next + 2.0 * h_previous;
    Some((w1 + w2) / (w1 / d_previous + w2 / d_next))
}

fn endpoint_slope(h0: f64, h1: f64, d0: f64, d1: f64) -> f64 {
    let mut slope = ((2.0 * h0 + h1) * d0 - h0 * d1) / (h0 + h1);
    if slope.signum() != d0.signum() {
        slope = 0.0;
    } else if d0.signum() != d1.signum() && slope.abs() > 3.0 * d0.abs() {
        slope = 3.0 * d0;
    }
    slope
}
