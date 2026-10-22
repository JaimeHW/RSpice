//! Interpolation behind `$table_model`.
//!
//! Binary search into sorted x data, then the control string's interpolation
//! (closest point, linear, C1 quadratic spline or natural cubic spline) and
//! per-end extrapolation (clamp, linear along the end slope, or a refusal),
//! plus the matching slope so the Jacobian stays consistent with the value.
//! Degenerate tables — empty, single point, duplicate abscissae — are handled
//! rather than left to produce infinities.

use super::*;
use rspice_veriloga_runtime::arithmetic::sum_products_ratio;

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}

/// An input outside the data of a table whose control string forbids
/// extrapolation at that end (`E`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableRangeError {
    pub input: f64,
    pub first: f64,
    pub last: f64,
}

impl std::fmt::Display for TableRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "$table_model input {} is outside the table data [{}, {}] and the control string \
             forbids extrapolation there (E)",
            self.input, self.first, self.last
        )
    }
}

/// Where an input falls relative to the data.
#[derive(Clone, Copy)]
enum Region {
    Below,
    Above,
    /// Inside segment `i`, `x_data[i] <= x <= x_data[i + 1]`.
    Inside(usize),
}

impl LookupTable {
    /// Create a new empty lookup table
    pub fn new() -> Self {
        Self {
            x_data: Vec::new(),
            y_data: Vec::new(),
            name: None,
            control: TableControl::default(),
            knots: Vec::new(),
        }
    }

    /// Apply a control string's interpolation and extrapolation, computing the
    /// spline knot data the degree needs.
    pub fn with_control(mut self, control: TableControl) -> Self {
        self.control = control;
        self.knots = match control.interpolation {
            TableInterpolation::Quadratic => quadratic_knot_slopes(&self.x_data, &self.y_data),
            TableInterpolation::Cubic => natural_cubic_curvatures(&self.x_data, &self.y_data),
            TableInterpolation::Discrete | TableInterpolation::Linear => Vec::new(),
        };
        self
    }

    /// Create a lookup table from data vectors
    ///
    /// # Panics
    /// Panics if x_data and y_data have different lengths
    pub fn from_data(x_data: Vec<f64>, y_data: Vec<f64>) -> Self {
        assert_eq!(
            x_data.len(),
            y_data.len(),
            "LookupTable: x_data and y_data must have the same length"
        );
        Self {
            x_data,
            y_data,
            name: None,
            control: TableControl::default(),
            knots: Vec::new(),
        }
    }

    /// Check if the table is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x_data.is_empty()
    }

    /// Get the number of data points
    #[inline]
    pub fn len(&self) -> usize {
        self.x_data.len()
    }

    /// The table's value at `x`, NaN where the control string forbids
    /// extrapolating to `x` (see [`Self::evaluate`] for the refusal itself).
    ///
    /// Behavior:
    /// - Empty table: returns 0.0
    /// - Single point: returns that y value (constant)
    /// - x outside the data: the end's extrapolation (linear by default)
    /// - x within range: the control string's interpolation (linear by default)
    pub fn interpolate(&self, x: f64) -> f64 {
        self.evaluate(x).unwrap_or(f64::NAN)
    }

    /// The table's value at `x`, or the refusal an `E` end makes of an input
    /// beyond it.
    pub fn evaluate(&self, x: f64) -> Result<f64, TableRangeError> {
        let n = self.x_data.len();
        if n == 0 {
            return Ok(0.0);
        }
        if n == 1 {
            return Ok(self.y_data[0]);
        }
        let linear = self.control.interpolation == TableInterpolation::Linear;
        let value = match self.region(x) {
            Region::Below => match self.control.below {
                TableExtrapolation::Error => return Err(self.range_error(x)),
                TableExtrapolation::Clamp => self.y_data[0],
                TableExtrapolation::Linear if linear => self.linear_extrapolate(0, 1, x),
                TableExtrapolation::Linear => {
                    self.y_data[0] + self.end_slope(false) * (x - self.x_data[0])
                }
            },
            Region::Above => match self.control.above {
                TableExtrapolation::Error => return Err(self.range_error(x)),
                TableExtrapolation::Clamp => self.y_data[n - 1],
                TableExtrapolation::Linear if linear => self.linear_extrapolate(n - 2, n - 1, x),
                TableExtrapolation::Linear => {
                    self.y_data[n - 1] + self.end_slope(true) * (x - self.x_data[n - 1])
                }
            },
            Region::Inside(i) => match self.control.interpolation {
                TableInterpolation::Linear => self.linear_interpolate(i, i + 1, x),
                TableInterpolation::Discrete => {
                    if x.is_nan() {
                        x
                    } else if x - self.x_data[i] < self.x_data[i + 1] - x {
                        self.y_data[i]
                    } else {
                        self.y_data[i + 1]
                    }
                }
                TableInterpolation::Quadratic => {
                    let (d, a) = self.quadratic_segment(i);
                    let t = x - self.x_data[i];
                    self.y_data[i] + t * (d + a * t)
                }
                TableInterpolation::Cubic => {
                    let h = self.x_data[i + 1] - self.x_data[i];
                    let a = (self.x_data[i + 1] - x) / h;
                    let b = (x - self.x_data[i]) / h;
                    a * self.y_data[i]
                        + b * self.y_data[i + 1]
                        + ((a * a * a - a) * self.knots[i] + (b * b * b - b) * self.knots[i + 1])
                            * (h * h)
                            / 6.0
                }
            },
        };
        Ok(value)
    }

    /// Compute the derivative (slope) at the given x value
    ///
    /// This is useful for Jacobian computation: the slope of the interpolant
    /// the value comes from, the end slope beyond a linear end and zero beyond
    /// a clamped one. A refused (`E`) end reports the slope a linear end
    /// would, since the value evaluation is what refuses.
    pub fn derivative(&self, x: f64) -> f64 {
        let n = self.x_data.len();

        if n < 2 {
            return 0.0;
        }
        let linear = self.control.interpolation == TableInterpolation::Linear;
        match self.region(x) {
            Region::Below if self.control.below == TableExtrapolation::Clamp => 0.0,
            Region::Above if self.control.above == TableExtrapolation::Clamp => 0.0,
            Region::Below if linear => self.slope(0, 1),
            Region::Above if linear => self.slope(n - 2, n - 1),
            Region::Below => self.end_slope(false),
            Region::Above => self.end_slope(true),
            Region::Inside(i) => self.segment_slope(i, x),
        }
    }

    /// Slope of the interpolant inside segment `i`.
    fn segment_slope(&self, i: usize, x: f64) -> f64 {
        match self.control.interpolation {
            TableInterpolation::Discrete => 0.0,
            TableInterpolation::Linear => self.slope(i, i + 1),
            TableInterpolation::Quadratic => {
                let (d, a) = self.quadratic_segment(i);
                d + 2.0 * a * (x - self.x_data[i])
            }
            TableInterpolation::Cubic => {
                let h = self.x_data[i + 1] - self.x_data[i];
                let a = (self.x_data[i + 1] - x) / h;
                let b = (x - self.x_data[i]) / h;
                self.slope(i, i + 1) - (3.0 * a * a - 1.0) / 6.0 * h * self.knots[i]
                    + (3.0 * b * b - 1.0) / 6.0 * h * self.knots[i + 1]
            }
        }
    }

    /// The interpolant's slope at the last (`upper`) or first data point,
    /// which linear extrapolation continues.
    fn end_slope(&self, upper: bool) -> f64 {
        let n = self.x_data.len();
        let (segment, x) = if upper {
            (n - 2, self.x_data[n - 1])
        } else {
            (0, self.x_data[0])
        };
        self.segment_slope(segment, x)
    }

    /// Start slope and curvature coefficient of quadratic segment `i`:
    /// `y_i + d t + a t^2` with `t = x - x_i`.
    fn quadratic_segment(&self, i: usize) -> (f64, f64) {
        let d = self.knots[i];
        let a = (self.slope(i, i + 1) - d) / (self.x_data[i + 1] - self.x_data[i]);
        (d, a)
    }

    fn region(&self, x: f64) -> Region {
        let n = self.x_data.len();
        if x < self.x_data[0] {
            Region::Below
        } else if x > self.x_data[n - 1] {
            Region::Above
        } else if x == self.x_data[n - 1] {
            Region::Inside(n - 2)
        } else {
            Region::Inside(self.find_interval(x))
        }
    }

    fn range_error(&self, input: f64) -> TableRangeError {
        TableRangeError {
            input,
            first: self.x_data[0],
            last: self.x_data[self.x_data.len() - 1],
        }
    }

    /// Apply the active slope without rounding it before its input derivative.
    pub fn apply_derivative(&self, x: f64, input_derivative: f64) -> f64 {
        if !x.is_finite() || !input_derivative.is_finite() {
            return f64::NAN;
        }
        let Some([x0, x1, y0, y1]) = self.derivative_segment(x) else {
            return 0.0;
        };
        if x0 == x1 || y0 == y1 || input_derivative == 0.0 {
            return 0.0;
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let slope = dy / dx;
        let result = slope * input_derivative;
        if dx.is_finite() && dy.is_finite() && slope.is_normal() && result.is_normal() {
            return result;
        }
        sum_products_ratio(
            [(y1, input_derivative), (-y0, input_derivative)].into_iter(),
            [(x1, 1.0), (-x0, 1.0)].into_iter(),
        )
        .unwrap_or(f64::NAN)
    }

    /// The active slope as a rise over a run, `[x0, x1, y0, y1]`. A linear
    /// segment returns its own endpoints so the slope is rounded once, after
    /// it meets the input derivative; any other slope is `[0, 1, 0, slope]`.
    pub(crate) fn derivative_segment(&self, x: f64) -> Option<[f64; 4]> {
        let n = self.x_data.len();
        if n < 2 {
            return None;
        }
        let region = self.region(x);
        let clamped = match region {
            Region::Below => self.control.below == TableExtrapolation::Clamp,
            Region::Above => self.control.above == TableExtrapolation::Clamp,
            Region::Inside(_) => false,
        };
        if clamped {
            return Some([0.0, 1.0, 0.0, 0.0]);
        }
        if self.control.interpolation != TableInterpolation::Linear {
            return Some([0.0, 1.0, 0.0, self.derivative(x)]);
        }
        let i = match region {
            Region::Below => 0,
            Region::Above => n - 2,
            Region::Inside(i) => i,
        };
        Some([
            self.x_data[i],
            self.x_data[i + 1],
            self.y_data[i],
            self.y_data[i + 1],
        ])
    }

    /// Binary search to find the interval containing x
    /// Returns index `i` such that `x_data[i] <= x < x_data[i+1]`
    #[inline]
    fn find_interval(&self, x: f64) -> usize {
        // Binary search for the insertion point
        match self
            .x_data
            .binary_search_by(|probe| probe.partial_cmp(&x).unwrap_or(std::cmp::Ordering::Equal))
        {
            Ok(idx) => {
                // Exact match - use this as the lower bound
                // But clamp to ensure idx+1 is valid
                if idx >= self.x_data.len() - 1 {
                    self.x_data.len() - 2
                } else {
                    idx
                }
            }
            Err(idx) => {
                // Not found - idx is where it would be inserted
                // So the interval is [idx-1, idx]
                if idx == 0 { 0 } else { idx - 1 }
            }
        }
    }

    /// Linear interpolation between points at indices i and j
    #[inline]
    fn linear_interpolate(&self, i: usize, j: usize, x: f64) -> f64 {
        let x0 = self.x_data[i];
        let x1 = self.x_data[j];
        let y0 = self.y_data[i];
        let y1 = self.y_data[j];

        // Distinct binary64 abscissae define an interval at every scale.
        if x1 == x0 || x == x0 {
            return y0;
        }
        if x == x1 {
            return y1;
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let t = (x - x0) / dx;
        let step = t * dy;
        let result = y0 + step;
        let cancellation =
            y0 != 0.0 && step != 0.0 && y0.is_sign_negative() != step.is_sign_negative();
        // Preserve the inexpensive ordinary path. Range loss and cancellation
        // require rounding the complete affine ratio, before forming a tiny
        // weight or an overflowing difference/product.
        if dx.is_finite()
            && dy.is_finite()
            && t.is_normal()
            && (step.is_normal() || dy == 0.0)
            && result.is_finite()
            && !cancellation
        {
            return result;
        }
        sum_products_ratio(
            [(y0, x1), (y1, x), (-y0, x), (-y1, x0)].into_iter(),
            [(x1, 1.0), (-x0, 1.0)].into_iter(),
        )
        .unwrap_or(f64::NAN)
    }

    /// Linear extrapolation using points at indices i and j
    #[inline]
    fn linear_extrapolate(&self, i: usize, j: usize, x: f64) -> f64 {
        // Same as interpolation, just allows x outside [x_i, x_j]
        self.linear_interpolate(i, j, x)
    }

    /// Compute the slope between points at indices i and j
    #[inline]
    fn slope(&self, i: usize, j: usize) -> f64 {
        let x0 = self.x_data[i];
        let x1 = self.x_data[j];
        let y0 = self.y_data[i];
        let y1 = self.y_data[j];

        if x1 == x0 {
            return 0.0;
        }
        let dy = y1 - y0;
        let dx = x1 - x0;
        if dy.is_finite() && dx.is_finite() {
            return dy / dx;
        }
        sum_products_ratio(
            [(y1, 1.0), (-y0, 1.0)].into_iter(),
            [(x1, 1.0), (-x0, 1.0)].into_iter(),
        )
        .unwrap_or(f64::NAN)
    }

    /// Validate the table data (sorted, no NaN, etc.)
    pub fn validate(&self) -> Result<(), String> {
        if self.x_data.len() != self.y_data.len() {
            return Err("x_data and y_data must have the same length".to_string());
        }

        // Check for NaN/Inf
        for (i, (&x, &y)) in self.x_data.iter().zip(self.y_data.iter()).enumerate() {
            if !x.is_finite() {
                return Err(format!("x_data[{}] = {} is not finite", i, x));
            }
            if !y.is_finite() {
                return Err(format!("y_data[{}] = {} is not finite", i, y));
            }
        }

        // Check sorted order
        for i in 1..self.x_data.len() {
            if self.x_data[i] < self.x_data[i - 1] {
                return Err(format!(
                    "x_data is not sorted: x[{}] = {} < x[{}] = {}",
                    i,
                    self.x_data[i],
                    i - 1,
                    self.x_data[i - 1]
                ));
            }
        }

        Ok(())
    }
}

/// Knot slopes of the C1 quadratic spline through the data.
///
/// A quadratic spline with knots at the data points leaves one slope free.
/// Started from the parabola through the first three points and propagated
/// forward (`d[i+1] = 2 s[i] - d[i]`), it reproduces any quadratic exactly;
/// started from the last three points and propagated backward it does too.
/// Their mean is again an interpolating C1 quadratic spline, and the only
/// one of the family that does not depend on which end the data is read from.
fn quadratic_knot_slopes(x: &[f64], y: &[f64]) -> Vec<f64> {
    let n = x.len();
    if n < 2 {
        return vec![0.0; n];
    }
    let h: Vec<f64> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let s: Vec<f64> = (0..n - 1).map(|i| (y[i + 1] - y[i]) / h[i]).collect();
    if n == 2 {
        return vec![s[0], s[0]];
    }
    let mut forward = vec![0.0; n];
    forward[0] = s[0] - h[0] * (s[1] - s[0]) / (h[0] + h[1]);
    for i in 0..n - 1 {
        forward[i + 1] = 2.0 * s[i] - forward[i];
    }
    let mut backward = vec![0.0; n];
    backward[n - 1] = s[n - 2] + h[n - 2] * (s[n - 2] - s[n - 3]) / (h[n - 3] + h[n - 2]);
    for i in (0..n - 1).rev() {
        backward[i] = 2.0 * s[i] - backward[i + 1];
    }
    forward
        .iter()
        .zip(&backward)
        .map(|(f, b)| 0.5 * (f + b))
        .collect()
}

/// Knot second derivatives of the natural cubic spline through the data
/// (zero curvature at both ends), by the tridiagonal (Thomas) solve.
fn natural_cubic_curvatures(x: &[f64], y: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut m = vec![0.0; n];
    if n < 3 {
        return m;
    }
    let h: Vec<f64> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let s: Vec<f64> = (0..n - 1).map(|i| (y[i + 1] - y[i]) / h[i]).collect();
    // Row i (1..n-1): h[i-1] m[i-1] + 2 (h[i-1] + h[i]) m[i] + h[i] m[i+1]
    //                 = 6 (s[i] - s[i-1]).
    let mut diagonal = vec![0.0; n];
    let mut rhs = vec![0.0; n];
    for i in 1..n - 1 {
        diagonal[i] = 2.0 * (h[i - 1] + h[i]);
        rhs[i] = 6.0 * (s[i] - s[i - 1]);
        if i > 1 {
            let factor = h[i - 1] / diagonal[i - 1];
            diagonal[i] -= factor * h[i - 1];
            rhs[i] -= factor * rhs[i - 1];
        }
    }
    for i in (1..n - 1).rev() {
        m[i] = (rhs[i] - h[i] * m[i + 1]) / diagonal[i];
    }
    m
}

#[cfg(test)]
mod control_tests {
    use super::*;

    fn squares(control: &str) -> LookupTable {
        let (interpolation, below, above) = match control {
            "D" => (
                TableInterpolation::Discrete,
                TableExtrapolation::Linear,
                TableExtrapolation::Linear,
            ),
            "1C" => (
                TableInterpolation::Linear,
                TableExtrapolation::Clamp,
                TableExtrapolation::Clamp,
            ),
            "1E" => (
                TableInterpolation::Linear,
                TableExtrapolation::Error,
                TableExtrapolation::Error,
            ),
            "2" => (
                TableInterpolation::Quadratic,
                TableExtrapolation::Linear,
                TableExtrapolation::Linear,
            ),
            "3" => (
                TableInterpolation::Cubic,
                TableExtrapolation::Linear,
                TableExtrapolation::Linear,
            ),
            "3CL" => (
                TableInterpolation::Cubic,
                TableExtrapolation::Clamp,
                TableExtrapolation::Linear,
            ),
            _ => unreachable!(),
        };
        LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]).with_control(
            TableControl {
                interpolation,
                below,
                above,
            },
        )
    }

    #[test]
    fn control_string_selects_the_interpolant_and_each_end() {
        // Clamp holds the end value; linear (the default) continues the slope.
        assert_eq!(squares("1C").interpolate(3.0), 4.0);
        assert_eq!(squares("1C").derivative(3.0), 0.0);
        assert_eq!(squares("1C").interpolate(-1.0), 0.0);
        let refused = squares("1E").evaluate(3.0).unwrap_err();
        assert_eq!(
            (refused.input, refused.first, refused.last),
            (3.0, 0.0, 2.0)
        );
        assert_eq!(squares("1E").evaluate(1.5), Ok(2.5));
        // Closest point.
        assert_eq!(squares("D").interpolate(0.4), 0.0);
        assert_eq!(squares("D").interpolate(1.6), 4.0);
        assert_eq!(squares("D").derivative(1.6), 0.0);
        // The quadratic spline reproduces y = x^2 exactly.
        assert_eq!(squares("2").interpolate(0.5), 0.25);
        assert_eq!(squares("2").interpolate(1.5), 2.25);
        assert_eq!(squares("2").derivative(1.5), 3.0);
        assert_eq!(squares("2").interpolate(3.0), 4.0 + 4.0);
        // Natural cubic spline through (0,0) (1,1) (2,4): curvature 3 at the
        // middle knot, so S(0.5) = 0.5 - 3 * 0.375 / 6 = 0.3125 and
        // S'(0) = 1 - 3 / 6 = 0.5, S'(2) = 3 + 3 / 6 = 3.5.
        assert_eq!(squares("3").interpolate(0.5), 0.3125);
        assert_eq!(squares("3").interpolate(1.5), 2.3125);
        assert_eq!(squares("3").derivative(0.0), 0.5);
        assert_eq!(squares("3").interpolate(3.0), 4.0 + 3.5);
        assert_eq!(squares("3").derivative(3.0), 3.5);
        assert_eq!(squares("3CL").interpolate(-1.0), 0.0);
        assert_eq!(squares("3CL").derivative(-1.0), 0.0);
    }
}
