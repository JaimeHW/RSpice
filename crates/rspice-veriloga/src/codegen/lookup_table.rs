//! Interpolation behind `$table_model`.
//!
//! Binary search into sorted x data, linear interpolation between points and
//! linear extrapolation beyond the ends, plus the matching slope so the
//! Jacobian stays consistent with the value. Degenerate tables — empty, single
//! point, duplicate abscissae — are handled rather than left to produce
//! infinities.

use super::*;
use rspice_veriloga_runtime::arithmetic::sum_products_ratio;

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}

impl LookupTable {
    /// Create a new empty lookup table
    pub fn new() -> Self {
        Self {
            x_data: Vec::new(),
            y_data: Vec::new(),
            name: None,
        }
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
        }
    }

    /// Create a lookup table with a name for debugging
    pub fn from_data_named(x_data: Vec<f64>, y_data: Vec<f64>, name: impl Into<SmolStr>) -> Self {
        assert_eq!(
            x_data.len(),
            y_data.len(),
            "LookupTable: x_data and y_data must have the same length"
        );
        Self {
            x_data,
            y_data,
            name: Some(name.into()),
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

    /// Perform linear interpolation at the given x value
    ///
    /// Behavior:
    /// - Empty table: returns 0.0
    /// - Single point: returns that y value (constant)
    /// - x below range: linear extrapolation from first two points
    /// - x above range: linear extrapolation from last two points
    /// - x within range: linear interpolation between bracketing points
    pub fn interpolate(&self, x: f64) -> f64 {
        let n = self.x_data.len();

        // Handle edge cases
        if n == 0 {
            return 0.0;
        }
        if n == 1 {
            return self.y_data[0];
        }

        // Handle out-of-bounds with linear extrapolation
        if x <= self.x_data[0] {
            // Extrapolate below range
            return self.linear_extrapolate(0, 1, x);
        }
        if x >= self.x_data[n - 1] {
            // Extrapolate above range
            return self.linear_extrapolate(n - 2, n - 1, x);
        }

        // Binary search for the bracketing interval
        let idx = self.find_interval(x);
        self.linear_interpolate(idx, idx + 1, x)
    }

    /// Compute the derivative (slope) at the given x value
    ///
    /// This is useful for Jacobian computation. Uses the local slope
    /// of the linear interpolation segment.
    pub fn derivative(&self, x: f64) -> f64 {
        let n = self.x_data.len();

        if n < 2 {
            return 0.0;
        }

        // Find the interval and return its slope
        if x <= self.x_data[0] || x >= self.x_data[n - 1] {
            // Use endpoint slopes for extrapolation
            if x <= self.x_data[0] {
                return self.slope(0, 1);
            } else {
                return self.slope(n - 2, n - 1);
            }
        }

        let idx = self.find_interval(x);
        self.slope(idx, idx + 1)
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

    pub(crate) fn derivative_segment(&self, x: f64) -> Option<[f64; 4]> {
        let n = self.x_data.len();
        if n < 2 {
            return None;
        }
        let i = if x <= self.x_data[0] {
            0
        } else if x >= self.x_data[n - 1] {
            n - 2
        } else {
            self.find_interval(x)
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
