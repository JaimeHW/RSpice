//! Exact piecewise-linear interval moments with explicit missing coverage.
//! Axis and signal scaling keep finite extreme inputs out of squared overflow.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntervalStatistics {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub rms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementError {
    Empty,
    LengthMismatch,
    NonFiniteAxis,
    NonMonotoneAxis,
    NonFiniteWindow,
    AmbiguousPoint,
    /// Fraction of the requested interval covered by finite linear segments.
    IncompleteCoverage {
        fraction: f64,
    },
}

impl fmt::Display for MeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "no retained samples"),
            Self::LengthMismatch => {
                write!(f, "sample coordinates and values have different lengths")
            }
            Self::NonFiniteAxis => write!(f, "undefined sample coordinates"),
            Self::NonMonotoneAxis => write!(f, "select one monotone sweep branch"),
            Self::NonFiniteWindow => write!(f, "undefined interval endpoints"),
            Self::AmbiguousPoint => write!(f, "multiple values at this zero-width interval"),
            Self::IncompleteCoverage { fraction } => write!(
                f,
                "missing or undefined samples ({:.1}% interval coverage)",
                fraction * 100.0
            ),
        }
    }
}

/// Measure one ascending or descending branch over a closed interval.
///
/// Reversed endpoints are equivalent. Duplicate coordinates represent vertical
/// edges: both values contribute to extrema but the edge has zero area. At a
/// zero-width interval, a unique finite value is required. No extrapolation or
/// integration across holes is allowed. Callers must split sweep branches.
pub fn measure_interval(
    x: &[f64],
    y: &[f64],
    window: Option<(f64, f64)>,
) -> Result<IntervalStatistics, MeasurementError> {
    if x.len() != y.len() {
        return Err(MeasurementError::LengthMismatch);
    }
    let (Some(&first), Some(&last)) = (x.first(), x.last()) else {
        return Err(MeasurementError::Empty);
    };
    if x.iter().any(|v| !v.is_finite()) {
        return Err(MeasurementError::NonFiniteAxis);
    }
    let ascending = last >= first;
    if x.windows(2).any(|pair| {
        if ascending {
            pair[1] < pair[0]
        } else {
            pair[1] > pair[0]
        }
    }) {
        return Err(MeasurementError::NonMonotoneAxis);
    }
    let (a, b) = window.unwrap_or((first, last));
    if !a.is_finite() || !b.is_finite() {
        return Err(MeasurementError::NonFiniteWindow);
    }
    let (lo, hi) = (a.min(b), a.max(b));
    if lo == hi {
        return point_statistics(x, y, lo);
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut complete = lo >= first.min(last) && hi <= first.max(last);
    // Retained samples include both sides of a vertical edge, even when the
    // edge lies exactly on a cursor. Do not silently discard undefined points.
    for (&at, &value) in x.iter().zip(y) {
        if at >= lo && at <= hi {
            if value.is_finite() {
                min = min.min(value);
                max = max.max(value);
            } else {
                complete = false;
            }
        }
    }
    let mut scale: f64 = 0.0;
    let mut coverage = Sum::default();
    for (xs, ys) in x.windows(2).zip(y.windows(2)) {
        let Some((left, right)) = clipped_span(xs, lo, hi) else {
            continue;
        };
        if ys.iter().all(|v| v.is_finite()) {
            let (u, v) = clipped_values(xs, ys, left, right);
            min = min.min(u).min(v);
            max = max.max(u).max(v);
            // Zero-width spikes affect extrema, not the integral's scale.
            scale = scale.max(u.abs()).max(v.abs());
            coverage.add(fraction(left, right, lo, hi));
        } else {
            complete = false;
        }
    }
    if !complete {
        return Err(MeasurementError::IncompleteCoverage {
            fraction: coverage.value().clamp(0.0, 1.0),
        });
    }

    let mut mean = Sum::default();
    // Keep cancellation residuals in physical amplitude units. Normalizing all
    // means by the largest sample would erase tiny signals after large equal
    // positive and negative areas cancel. Scale small signals up to protect
    // subnormal areas, and reserve headroom near the overflow limit.
    let mean_scale = if scale > 0.0 && scale < 1.0 {
        scale
    } else if scale > f64::MAX * 0.5 {
        2.0
    } else {
        1.0
    };
    let mut squares = Sum::default();
    let mut small_rms: f64 = 0.0;
    if scale > 0.0 {
        for (xs, ys) in x.windows(2).zip(y.windows(2)) {
            let Some((left, right)) = clipped_span(xs, lo, hi) else {
                continue;
            };
            let (u, v) = clipped_values(xs, ys, left, right);
            let root_weight = || sqrt_span(left, right) / sqrt_span(lo, hi);
            let weight = fraction(left, right, lo, hi);
            let panel_mean = midpoint(u / mean_scale, v / mean_scale);
            mean.add(if weight > 0.0 {
                panel_mean * weight
            } else {
                (panel_mean * root_weight()) * root_weight()
            });
            let panel_scale = u.abs().max(v.abs());
            if panel_scale == 0.0 {
                continue;
            }
            let (u, v) = (u / panel_scale, v / panel_scale);
            // Integral of the square of a linear segment, not a trapezoid
            // through squared samples. Normalize locally before squaring so a
            // small signal beside a huge short pulse still contributes.
            let panel_square = (u * u + u * v + v * v) / 3.0;
            let amplitude = panel_scale / scale;
            let energy = amplitude * amplitude * panel_square * weight;
            if energy >= f64::MIN_POSITIVE {
                squares.add(energy);
            } else {
                // Tiny energies need not be representable before taking their
                // square root. Keep these separately from the compensated sum
                // so ordinary dense traces do not accumulate hypot roundoff.
                small_rms = small_rms.hypot(amplitude * panel_square.sqrt() * root_weight());
            }
        }
    }
    Ok(IntervalStatistics {
        min,
        max,
        // A mean and RMS cannot exceed the contributing signal's magnitude.
        mean: mean.value().clamp(-scale / mean_scale, scale / mean_scale) * mean_scale,
        rms: squares.value().sqrt().hypot(small_rms).min(1.0) * scale,
    })
}

fn midpoint(a: f64, b: f64) -> f64 {
    if (b - a).is_finite() {
        a + (b - a) * 0.5
    } else {
        a * 0.5 + b * 0.5
    }
}

fn clipped_span(xs: &[f64], lo: f64, hi: f64) -> Option<(f64, f64)> {
    let left = lo.max(xs[0].min(xs[1]));
    let right = hi.min(xs[0].max(xs[1]));
    (left < right).then_some((left, right))
}

fn fraction(a: f64, b: f64, lo: f64, hi: f64) -> f64 {
    if (hi - lo).is_finite() {
        (b - a) / (hi - lo)
    } else {
        (b * 0.5 - a * 0.5) / (hi * 0.5 - lo * 0.5)
    }
}

fn sqrt_span(lo: f64, hi: f64) -> f64 {
    let span = hi - lo;
    if span.is_finite() {
        span.sqrt()
    } else {
        (hi * 0.5 - lo * 0.5).sqrt() * std::f64::consts::SQRT_2
    }
}

fn clipped_values(xs: &[f64], ys: &[f64], left: f64, right: f64) -> (f64, f64) {
    let sample = |at| {
        if at == xs[0] {
            return ys[0];
        }
        if at == xs[1] {
            return ys[1];
        }
        let (x0, x1, y0, y1) = if xs[0] < xs[1] {
            (xs[0], xs[1], ys[0], ys[1])
        } else {
            (xs[1], xs[0], ys[1], ys[0])
        };
        let t = fraction(x0, at, x0, x1);
        (1.0 - t) * y0 + t * y1
    };
    (sample(left), sample(right))
}

fn point_statistics(x: &[f64], y: &[f64], at: f64) -> Result<IntervalStatistics, MeasurementError> {
    let mut value = None;
    for (_, &sample) in x.iter().zip(y).filter(|(position, _)| **position == at) {
        if !sample.is_finite() {
            return Err(MeasurementError::IncompleteCoverage { fraction: 0.0 });
        }
        if value.is_some_and(|previous| previous != sample) {
            return Err(MeasurementError::AmbiguousPoint);
        }
        value = Some(sample);
    }
    if value.is_none() {
        for (xs, ys) in x.windows(2).zip(y.windows(2)) {
            if at > xs[0].min(xs[1]) && at < xs[0].max(xs[1]) && ys.iter().all(|v| v.is_finite()) {
                value = Some(clipped_values(xs, ys, at, at).0);
                break;
            }
        }
    }
    let value = value.ok_or(MeasurementError::IncompleteCoverage { fraction: 0.0 })?;
    Ok(IntervalStatistics {
        min: value,
        max: value,
        mean: value,
        rms: value.abs(),
    })
}

#[derive(Default)]
struct Sum {
    sum: f64,
    correction: f64,
}

impl Sum {
    fn add(&mut self, value: f64) {
        let next = self.sum + value;
        self.correction += if self.sum.abs() >= value.abs() {
            (self.sum - next) + value
        } else {
            (value - next) + self.sum
        };
        self.sum = next;
    }

    fn value(self) -> f64 {
        self.sum + self.correction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-14, "{actual} != {expected}");
    }

    #[test]
    fn cursor_endpoints_and_resampling_preserve_linear_statistics() {
        for (x, y) in [
            (vec![0.0, 1.0], vec![0.0, 1.0]),
            (vec![0.0, 0.1, 0.5, 1.0], vec![0.0, 0.1, 0.5, 1.0]),
            (vec![1.0, 0.5, 0.0], vec![1.0, 0.5, 0.0]),
        ] {
            for window in [(0.25, 0.75), (0.75, 0.25)] {
                let stats = measure_interval(&x, &y, Some(window)).unwrap();
                close(stats.min, 0.25);
                close(stats.max, 0.75);
                close(stats.mean, 0.5);
                close(stats.rms, (13.0_f64 / 48.0).sqrt());
            }
        }
    }

    #[test]
    fn missing_coverage_is_not_a_complete_measurement() {
        let x = [0.0, 1.0, 2.0, 3.0, 4.0];
        let y = [1.0, 1.0, f64::NAN, 1.0, 1.0];
        assert_eq!(
            measure_interval(&x, &y, None),
            Err(MeasurementError::IncompleteCoverage { fraction: 0.5 })
        );
        assert_eq!(
            measure_interval(&[0.0, 1.0], &[1.0; 2], Some((-1.0, 1.0))),
            Err(MeasurementError::IncompleteCoverage { fraction: 0.5 })
        );
        assert_eq!(
            measure_interval(&x, &y, Some((0.25, 0.75))).unwrap().rms,
            1.0
        );
        assert_eq!(
            measure_interval(&x, &y, Some((1.0, 1.0))).unwrap().mean,
            1.0
        );
    }

    #[test]
    fn vertical_edges_have_no_area_and_zero_width_requires_a_unique_value() {
        let x = [0.0, 1.0, 1.0, 2.0];
        let y = [2.0, 2.0, -1.0, -1.0];
        let stats = measure_interval(&x, &y, None).unwrap();
        close(stats.mean, 0.5);
        close(stats.rms, 2.5_f64.sqrt());
        assert_eq!(
            measure_interval(&x, &y, Some((1.0, 1.0))),
            Err(MeasurementError::AmbiguousPoint)
        );
        assert_eq!(
            measure_interval(&[1.0, 1.0], &[2.0, 3.0], None),
            Err(MeasurementError::AmbiguousPoint)
        );
        assert_eq!(measure_interval(&[1.0], &[-2.0], None).unwrap().rms, 2.0);
        assert_eq!(
            measure_interval(&[0.0, 1.0], &[0.0, 1.0], Some((0.5, 0.5)))
                .unwrap()
                .mean,
            0.5
        );
    }

    #[test]
    fn invalid_domains_are_explained() {
        for (x, y, error) in [
            (vec![], vec![], MeasurementError::Empty),
            (vec![0.0], vec![], MeasurementError::LengthMismatch),
            (
                vec![0.0, f64::NAN],
                vec![1.0; 2],
                MeasurementError::NonFiniteAxis,
            ),
            (
                vec![0.0, 1.0, 0.0],
                vec![1.0; 3],
                MeasurementError::NonMonotoneAxis,
            ),
        ] {
            assert_eq!(measure_interval(&x, &y, None), Err(error));
        }
        assert_eq!(
            measure_interval(&[0.0, 1.0], &[1.0; 2], Some((0.0, f64::NAN))),
            Err(MeasurementError::NonFiniteWindow)
        );
    }

    #[test]
    fn mean_retains_small_residuals_after_large_areas_cancel() {
        let stats = measure_interval(
            &[0.0, 1.0, 1.0, 2.0, 2.0, 3.0],
            &[1e300, 1e300, -1e300, -1e300, 1e-300, 1e-300],
            None,
        )
        .unwrap();
        close(stats.mean / 1e-300, 1.0 / 3.0);
    }

    #[test]
    fn finite_moments_survive_an_underflowing_duration_fraction() {
        let stats = measure_interval(
            &[0.0, 1e-300, 1e-300, 1e300],
            &[1e300, 1e300, 0.0, 0.0],
            None,
        )
        .unwrap();
        close(stats.mean / 1e-300, 1.0);
        close(stats.rms, 1.0);
    }

    #[test]
    fn subnormal_mean_is_invariant_under_equivalent_resampling() {
        let tiny = f64::from_bits(1);
        for x in [&[0.0, 1.0][..], &[0.0, 0.5, 1.0][..]] {
            let constant = vec![tiny; x.len()];
            let ramp: Vec<_> = x.iter().map(|value| value * (2.0 * tiny)).collect();
            for y in [constant, ramp] {
                let stats = measure_interval(x, &y, None).unwrap();
                assert_eq!(stats.mean, tiny);
                assert_eq!(stats.rms, tiny);
            }
        }
    }

    #[test]
    fn a_tiny_duty_cycle_pulse_does_not_erase_the_background_rms() {
        let stats = measure_interval(
            &[0.0, 1e-300, 1e-300, 1e300],
            &[1e300, 1e300, 1.0, 1.0],
            None,
        )
        .unwrap();
        // The pulse and the background each contribute one unit of mean square.
        close(stats.rms, 2.0_f64.sqrt());
    }

    #[test]
    fn dense_resampling_preserves_the_linear_rms() {
        let x: Vec<_> = (0..=100_000)
            .map(|index| f64::from(index) / 100_000.0)
            .collect();
        let stats = measure_interval(&x, &x, None).unwrap();
        close(stats.rms, (1.0_f64 / 3.0).sqrt());
    }
}
