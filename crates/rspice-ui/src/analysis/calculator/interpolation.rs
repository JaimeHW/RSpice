//! Interpolation on a strictly monotone, finite axis. Binary arithmetic uses
//! both operands' breakpoints in their shared domain, without extrapolation.
//! Nonfinite ordinates represent gaps; interpolation does not bridge them.

#![allow(clippy::type_complexity)]

// =============================================================================
// Interpolation Method
// =============================================================================

/// Interpolation method for resampling waveforms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InterpolationMethod {
    /// Linear interpolation between adjacent points (fast)
    #[default]
    Linear,
    /// Cubic spline interpolation (smoother, preserves derivatives)
    CubicSpline,
}

// =============================================================================
// Waveform Interpolator
// =============================================================================

/// Interpolates waveform data at arbitrary points
#[derive(Debug, Clone)]
pub struct WaveformInterpolator<'a> {
    /// X values (time/frequency points)
    x: &'a [f64],
    /// Y values (signal values)
    y: &'a [f64],
    descending: bool,
    /// Interpolation method
    method: InterpolationMethod,
    /// Pre-computed spline coefficients (for CubicSpline)
    spline_coeffs: Option<SplineCoefficients>,
}

/// Pre-computed cubic spline coefficients
#[derive(Debug, Clone)]
struct SplineCoefficients {
    /// Second derivatives at each point
    y2: Vec<f64>,
}

impl<'a> WaveformInterpolator<'a> {
    /// Validate an unambiguous interpolation domain once before sampling it.
    pub fn new(x: &'a [f64], y: &'a [f64]) -> Result<Self, InterpolationError> {
        validate_samples(x, y)?;
        let descending = x[0] > x[x.len() - 1];
        if x.windows(2).any(|pair| {
            if descending {
                pair[0] <= pair[1]
            } else {
                pair[0] >= pair[1]
            }
        }) {
            return Err(InterpolationError::AmbiguousAxis);
        }
        Ok(Self {
            x,
            y,
            descending,
            method: InterpolationMethod::Linear,
            spline_coeffs: None,
        })
    }

    /// Set interpolation method
    pub fn with_method(mut self, method: InterpolationMethod) -> Self {
        self.method = method;
        if method == InterpolationMethod::CubicSpline && self.x.len() > 2 {
            self.spline_coeffs = Some(compute_spline_coeffs(self.x, self.y));
        }
        self
    }

    /// Interpolate at a single point
    pub fn interpolate_at(&self, target_x: f64) -> Result<f64, InterpolationError> {
        if !target_x.is_finite() {
            return Err(InterpolationError::NonfiniteCoordinate);
        }
        let last = self.x.len() - 1;
        if target_x < self.x[0].min(self.x[last]) || target_x > self.x[0].max(self.x[last]) {
            return Err(InterpolationError::OutsideDomain);
        }
        if target_x == self.x[0] {
            return Ok(self.sample(0));
        }
        if target_x == self.x[last] {
            return Ok(self.sample(last));
        }
        let idx = self.find_interval(target_x);
        if target_x == self.x[idx] {
            return Ok(self.sample(idx));
        }
        if !self.y[idx].is_finite() || !self.y[idx + 1].is_finite() {
            return Ok(f64::NAN);
        }
        let value = match self.method {
            InterpolationMethod::Linear => self.interpolate_linear(idx, target_x),
            InterpolationMethod::CubicSpline => self.interpolate_cubic(idx, target_x),
        };
        value
            .is_finite()
            .then_some(value)
            .ok_or(InterpolationError::UnrepresentableValue)
    }

    /// Resample onto a new x grid
    pub fn resample(&self, new_x: &[f64]) -> Result<Vec<f64>, InterpolationError> {
        new_x.iter().map(|&x| self.interpolate_at(x)).collect()
    }

    // -------------------------------------------------------------------------
    // Private Helpers
    // -------------------------------------------------------------------------

    /// Binary search for interval containing target_x
    fn find_interval(&self, target_x: f64) -> usize {
        let mut low = 0;
        let mut high = self.x.len() - 1;

        while high - low > 1 {
            let mid = (low + high) / 2;
            if if self.descending {
                self.x[mid] >= target_x
            } else {
                self.x[mid] <= target_x
            } {
                low = mid;
            } else {
                high = mid;
            }
        }

        low
    }

    /// Linear interpolation within an interval
    fn interpolate_linear(&self, idx: usize, target_x: f64) -> f64 {
        let x0 = self.x[idx];
        let x1 = self.x[idx + 1];
        let y0 = self.y[idx];
        let y1 = self.y[idx + 1];

        let width = x1 - x0;
        let t = if width.is_finite() {
            (target_x - x0) / width
        } else {
            (target_x * 0.5 - x0 * 0.5) / (x1 * 0.5 - x0 * 0.5)
        };
        // A convex combination avoids overflowing the difference between
        // opposite-sign finite ordinates.
        (1.0 - t) * y0 + t * y1
    }

    /// Cubic spline interpolation within an interval
    fn interpolate_cubic(&self, idx: usize, target_x: f64) -> f64 {
        let Some(coeffs) = &self.spline_coeffs else {
            // Fall back to linear if no coefficients
            return self.interpolate_linear(idx, target_x);
        };

        let x0 = self.x[idx];
        let x1 = self.x[idx + 1];
        let y0 = self.y[idx];
        let y1 = self.y[idx + 1];

        let h = x1 - x0;
        if h == 0.0 {
            return y0;
        }

        let a = (x1 - target_x) / h;
        let b = (target_x - x0) / h;

        let y2_0 = coeffs.y2[idx];
        let y2_1 = coeffs.y2[idx + 1];

        // Natural cubic spline formula
        a * y0 + b * y1 + ((a * a * a - a) * y2_0 + (b * b * b - b) * y2_1) * (h * h) / 6.0
    }

    fn sample(&self, index: usize) -> f64 {
        if self.y[index].is_finite() {
            self.y[index]
        } else {
            f64::NAN
        }
    }

    fn ascending_x(&self, index: usize) -> f64 {
        self.x[if self.descending {
            self.x.len() - 1 - index
        } else {
            index
        }]
    }
}

// =============================================================================
// Spline Coefficient Computation
// =============================================================================

/// Compute natural cubic spline second derivatives
fn compute_spline_coeffs(x: &[f64], y: &[f64]) -> SplineCoefficients {
    let mut y2 = vec![f64::NAN; x.len()];
    let mut start = 0;
    while start < y.len() {
        if !y[start].is_finite() {
            start += 1;
            continue;
        }
        let end = start
            + y[start..]
                .iter()
                .take_while(|value| value.is_finite())
                .count();
        let segment = compute_finite_spline(&x[start..end], &y[start..end]);
        y2[start..end].copy_from_slice(&segment.y2);
        start = end;
    }
    SplineCoefficients { y2 }
}

fn compute_finite_spline(x: &[f64], y: &[f64]) -> SplineCoefficients {
    let n = x.len();
    if n < 3 {
        return SplineCoefficients { y2: vec![0.0; n] };
    }

    // Tridiagonal solve for natural cubic spline
    let mut y2 = vec![0.0; n];
    let mut u = vec![0.0; n - 1];

    // Natural spline: y2[0] = y2[n-1] = 0
    for i in 1..n - 1 {
        let h_i = x[i] - x[i - 1];
        let h_i1 = x[i + 1] - x[i];

        if h_i == 0.0 || h_i1 == 0.0 {
            continue;
        }

        let sig = h_i / (h_i + h_i1);
        let p = sig * y2[i - 1] + 2.0;
        y2[i] = (sig - 1.0) / p;
        u[i] = (6.0 * ((y[i + 1] - y[i]) / h_i1 - (y[i] - y[i - 1]) / h_i) / (h_i + h_i1)
            - sig * u[i - 1])
            / p;
    }

    // Back substitution
    for i in (0..n - 2).rev() {
        y2[i + 1] = y2[i + 1] * y2[i + 2] + u[i + 1];
    }

    SplineCoefficients { y2 }
}

// =============================================================================
// Error Types
// =============================================================================

/// Interpolation error
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationError {
    /// Empty waveform provided
    EmptyWaveform,
    LengthMismatch,
    NonfiniteCoordinate,
    AmbiguousAxis,
    OutsideDomain,
    NoSharedDomain,
    UnrepresentableValue,
}

impl std::fmt::Display for InterpolationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyWaveform => write!(f, "Empty waveform"),
            Self::LengthMismatch => write!(f, "X and Y sample counts differ"),
            Self::NonfiniteCoordinate => write!(f, "X coordinates must be finite"),
            Self::AmbiguousAxis => write!(
                f,
                "unequal axes with repeated coordinates or multiple sweep branches need an explicit branch selection"
            ),
            Self::OutsideDomain => {
                write!(f, "the requested coordinate is outside the waveform domain")
            }
            Self::NoSharedDomain => write!(f, "waveform domains do not overlap"),
            Self::UnrepresentableValue => write!(
                f,
                "the interpolated value cannot be represented as a finite number"
            ),
        }
    }
}

impl std::error::Error for InterpolationError {}

/// Pointwise operations can keep a repeated or branching axis, but cannot
/// accept malformed storage or nonfinite coordinates.
pub(super) fn validate_samples(x: &[f64], y: &[f64]) -> Result<(), InterpolationError> {
    if x.len() != y.len() {
        return Err(InterpolationError::LengthMismatch);
    }
    if x.is_empty() {
        return Err(InterpolationError::EmptyWaveform);
    }
    if x.iter().any(|value| !value.is_finite()) {
        return Err(InterpolationError::NonfiniteCoordinate);
    }
    Ok(())
}

// =============================================================================
// Waveform Alignment Utility
// =============================================================================

/// Align two waveforms to a common time base for binary operations
///
/// Keep every breakpoint in the intersection, ordered like the first operand.
/// This preserves narrow features regardless of which operand is on the left.
pub fn align_waveforms(
    x1: &[f64],
    y1: &[f64],
    x2: &[f64],
    y2: &[f64],
    method: InterpolationMethod,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), InterpolationError> {
    let left = WaveformInterpolator::new(x1, y1)?.with_method(method);
    let right = WaveformInterpolator::new(x2, y2)?.with_method(method);
    let low = left.ascending_x(0).max(right.ascending_x(0));
    let high = left
        .ascending_x(x1.len() - 1)
        .min(right.ascending_x(x2.len() - 1));
    if low > high {
        return Err(InterpolationError::NoSharedDomain);
    }
    let mut x = Vec::with_capacity(x1.len() + x2.len());
    let (mut i, mut j) = (0, 0);
    while i < x1.len() || j < x2.len() {
        let next = if j == x2.len() || (i < x1.len() && left.ascending_x(i) < right.ascending_x(j))
        {
            let value = left.ascending_x(i);
            i += 1;
            value
        } else {
            let value = right.ascending_x(j);
            j += 1;
            if i < x1.len() && left.ascending_x(i) == value {
                i += 1;
            }
            value
        };
        if next >= low && next <= high {
            x.push(next);
        }
    }
    if left.descending {
        x.reverse();
    }
    let left_y = left.resample(&x)?;
    let right_y = right.resample(&x)?;
    Ok((x, left_y, right_y))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending_interpolation_matches_ascending_for_both_methods() {
        for method in [
            InterpolationMethod::Linear,
            InterpolationMethod::CubicSpline,
        ] {
            let ascending = WaveformInterpolator::new(&[0.0, 1.0, 3.0], &[0.0, 2.0, 1.0])
                .unwrap()
                .with_method(method);
            let descending = WaveformInterpolator::new(&[3.0, 1.0, 0.0], &[1.0, 2.0, 0.0])
                .unwrap()
                .with_method(method);
            for at in [0.0, 0.25, 1.0, 1.5, 2.5, 3.0] {
                assert!(
                    (ascending.interpolate_at(at).unwrap()
                        - descending.interpolate_at(at).unwrap())
                    .abs()
                        < 1e-14
                );
            }
            assert!(descending.interpolate_at(-0.1).is_err());
            assert!(descending.interpolate_at(3.1).is_err());
        }
        assert_eq!(
            WaveformInterpolator::new(&[3.0, 2.0, 0.0], &[3.0, 2.0, 0.0])
                .unwrap()
                .interpolate_at(1.5)
                .unwrap(),
            1.5
        );
    }

    #[test]
    fn interpolation_does_not_bridge_holes_or_poison_other_segments() {
        let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y = [0.0, 1.0, 2.0, f64::NAN, 4.0, 5.0, 6.0];
        for method in [
            InterpolationMethod::Linear,
            InterpolationMethod::CubicSpline,
        ] {
            let interpolator = WaveformInterpolator::new(&x, &y)
                .unwrap()
                .with_method(method);
            for at in [0.0, 0.5, 1.0, 1.5, 2.0, 4.0, 4.5, 5.0, 5.5, 6.0] {
                assert_eq!(interpolator.interpolate_at(at).unwrap(), at);
            }
            for at in [2.5, 3.0, 3.5] {
                assert!(interpolator.interpolate_at(at).unwrap().is_nan());
            }
        }
    }

    #[test]
    fn interpolation_validates_storage_and_ambiguous_domains() {
        assert!(matches!(
            WaveformInterpolator::new(&[0.0, 1.0], &[0.0]),
            Err(InterpolationError::LengthMismatch)
        ));
        assert!(matches!(
            WaveformInterpolator::new(&[], &[]),
            Err(InterpolationError::EmptyWaveform)
        ));
        for x in [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0]] {
            assert!(matches!(
                WaveformInterpolator::new(&x, &[0.0; 3]),
                Err(InterpolationError::AmbiguousAxis)
            ));
        }
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                WaveformInterpolator::new(&[0.0, invalid], &[0.0, 1.0]),
                Err(InterpolationError::NonfiniteCoordinate)
            ));
            assert!(
                WaveformInterpolator::new(&[0.0], &[1.0])
                    .unwrap()
                    .interpolate_at(invalid)
                    .is_err()
            );
        }
    }

    #[test]
    fn finite_extreme_linear_samples_do_not_overflow_the_intermediate_difference() {
        let x = [-f64::MAX, f64::MAX];
        let interpolator = WaveformInterpolator::new(&x, &x).unwrap();
        assert_eq!(interpolator.interpolate_at(0.0).unwrap(), 0.0);
        assert_eq!(interpolator.interpolate_at(-f64::MAX).unwrap(), -f64::MAX);
        assert_eq!(interpolator.interpolate_at(f64::MAX).unwrap(), f64::MAX);
        let small = [0.0, f64::MIN_POSITIVE];
        assert_eq!(
            WaveformInterpolator::new(&small, &[0.0, 1.0])
                .unwrap()
                .interpolate_at(f64::MIN_POSITIVE / 2.0)
                .unwrap(),
            0.5
        );
    }
}
