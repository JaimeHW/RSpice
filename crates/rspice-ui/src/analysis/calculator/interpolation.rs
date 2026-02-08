//! Waveform Interpolation Module
//!
//! Professional-grade interpolation for waveform operations following Spectre conventions.
//! Handles mismatched time bases through automatic resampling.
//!
//! # Features
//!
//! - Linear interpolation (default, fast)
//! - Cubic spline interpolation (accurate, smooth derivatives)
//! - Automatic resampling for binary operations
//! - Extrapolation control (flat, linear, error)
//!
//! # Architecture
//!
//! ```text
//! calc(V(out) + V(in))
//!         │
//!         ▼
//! ┌─────────────────────┐
//! │ detect time base    │
//! │ mismatch            │
//! └─────────────────────┘
//!         │
//!         ▼
//! ┌─────────────────────┐
//! │ resample second     │
//! │ waveform to first   │
//! └─────────────────────┘
//!         │
//!         ▼
//! ┌─────────────────────┐
//! │ apply operation     │
//! └─────────────────────┘
//! ```

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
    /// Zero-order hold (step function, for digital signals)
    ZeroOrderHold,
}

// =============================================================================
// Extrapolation Mode
// =============================================================================

/// Behavior when interpolating outside the data range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExtrapolationMode {
    /// Hold the first/last value constant
    #[default]
    Flat,
    /// Linear extrapolation from the endpoint (can diverge)
    Linear,
    /// Return NaN for out-of-range queries
    NaN,
    /// Return an error for out-of-range queries
    Error,
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
    /// Interpolation method
    method: InterpolationMethod,
    /// Extrapolation behavior
    extrap: ExtrapolationMode,
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
    /// Create a new interpolator with default settings (linear, flat extrapolation)
    pub fn new(x: &'a [f64], y: &'a [f64]) -> Self {
        assert_eq!(x.len(), y.len(), "x and y must have same length");
        Self {
            x,
            y,
            method: InterpolationMethod::Linear,
            extrap: ExtrapolationMode::Flat,
            spline_coeffs: None,
        }
    }

    /// Set interpolation method
    pub fn with_method(mut self, method: InterpolationMethod) -> Self {
        self.method = method;
        if method == InterpolationMethod::CubicSpline && self.x.len() > 2 {
            self.spline_coeffs = Some(compute_spline_coeffs(self.x, self.y));
        }
        self
    }

    /// Set extrapolation mode
    pub fn with_extrapolation(mut self, mode: ExtrapolationMode) -> Self {
        self.extrap = mode;
        self
    }

    /// Get waveform length
    pub fn len(&self) -> usize {
        self.x.len()
    }

    /// Check if waveform is empty
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    /// Get x range
    pub fn x_range(&self) -> Option<(f64, f64)> {
        if self.x.is_empty() {
            None
        } else {
            Some((self.x[0], self.x[self.x.len() - 1]))
        }
    }

    /// Interpolate at a single point
    pub fn interpolate_at(&self, target_x: f64) -> Result<f64, InterpolationError> {
        if self.x.is_empty() {
            return Err(InterpolationError::EmptyWaveform);
        }

        if self.x.len() == 1 {
            // Single point - just return the value
            return Ok(self.y[0]);
        }

        let x_min = self.x[0];
        let x_max = self.x[self.x.len() - 1];

        // Handle extrapolation
        if target_x < x_min {
            return self.extrapolate_left(target_x, x_min);
        }
        if target_x > x_max {
            return self.extrapolate_right(target_x, x_max);
        }

        // Find bracketing interval using binary search
        let idx = self.find_interval(target_x);

        // Interpolate within interval
        match self.method {
            InterpolationMethod::Linear => Ok(self.interpolate_linear(idx, target_x)),
            InterpolationMethod::CubicSpline => Ok(self.interpolate_cubic(idx, target_x)),
            InterpolationMethod::ZeroOrderHold => Ok(self.y[idx]),
        }
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
        // Binary search to find i such that x[i] <= target_x < x[i+1]
        let mut low = 0;
        let mut high = self.x.len() - 1;

        while high - low > 1 {
            let mid = (low + high) / 2;
            if self.x[mid] <= target_x {
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

        if x1 == x0 {
            return y0; // Avoid division by zero
        }

        let t = (target_x - x0) / (x1 - x0);
        y0 + t * (y1 - y0)
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

    /// Extrapolate to the left
    fn extrapolate_left(&self, target_x: f64, x_min: f64) -> Result<f64, InterpolationError> {
        match self.extrap {
            ExtrapolationMode::Flat => Ok(self.y[0]),
            ExtrapolationMode::Linear => {
                if self.x.len() < 2 {
                    Ok(self.y[0])
                } else {
                    // Linear extrapolation from first two points
                    let slope = (self.y[1] - self.y[0]) / (self.x[1] - self.x[0]);
                    Ok(self.y[0] + slope * (target_x - x_min))
                }
            }
            ExtrapolationMode::NaN => Ok(f64::NAN),
            ExtrapolationMode::Error => Err(InterpolationError::OutOfRange(target_x)),
        }
    }

    /// Extrapolate to the right
    fn extrapolate_right(&self, target_x: f64, x_max: f64) -> Result<f64, InterpolationError> {
        let n = self.x.len();
        match self.extrap {
            ExtrapolationMode::Flat => Ok(self.y[n - 1]),
            ExtrapolationMode::Linear => {
                if n < 2 {
                    Ok(self.y[n - 1])
                } else {
                    let slope = (self.y[n - 1] - self.y[n - 2]) / (self.x[n - 1] - self.x[n - 2]);
                    Ok(self.y[n - 1] + slope * (target_x - x_max))
                }
            }
            ExtrapolationMode::NaN => Ok(f64::NAN),
            ExtrapolationMode::Error => Err(InterpolationError::OutOfRange(target_x)),
        }
    }
}

// =============================================================================
// Spline Coefficient Computation
// =============================================================================

/// Compute natural cubic spline second derivatives
fn compute_spline_coeffs(x: &[f64], y: &[f64]) -> SplineCoefficients {
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
    /// Target x is out of range
    OutOfRange(f64),
    /// Waveforms cannot be aligned
    AlignmentFailed,
}

impl std::fmt::Display for InterpolationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyWaveform => write!(f, "Empty waveform"),
            Self::OutOfRange(x) => write!(f, "Value {} is out of interpolation range", x),
            Self::AlignmentFailed => write!(f, "Waveform alignment failed"),
        }
    }
}

impl std::error::Error for InterpolationError {}

// =============================================================================
// Waveform Alignment Utility
// =============================================================================

/// Align two waveforms to a common time base for binary operations
///
/// Resamples the second waveform onto the first waveform's time base.
pub fn align_waveforms(
    x1: &[f64],
    y1: &[f64],
    x2: &[f64],
    y2: &[f64],
    method: InterpolationMethod,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), InterpolationError> {
    if x1.is_empty() || x2.is_empty() {
        return Err(InterpolationError::EmptyWaveform);
    }

    // Use first waveform's x-axis as reference
    let interp = WaveformInterpolator::new(x2, y2)
        .with_method(method)
        .with_extrapolation(ExtrapolationMode::Flat);

    let y2_resampled = interp.resample(x1)?;

    Ok((x1.to_vec(), y1.to_vec(), y2_resampled))
}

/// Align waveforms to a union of both time bases
///
/// Creates a merged time base containing all unique points from both waveforms.
pub fn align_waveforms_union(
    x1: &[f64],
    y1: &[f64],
    x2: &[f64],
    y2: &[f64],
    method: InterpolationMethod,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), InterpolationError> {
    if x1.is_empty() || x2.is_empty() {
        return Err(InterpolationError::EmptyWaveform);
    }

    // Merge time bases
    let mut merged_x: Vec<f64> = Vec::with_capacity(x1.len() + x2.len());
    merged_x.extend_from_slice(x1);
    merged_x.extend_from_slice(x2);
    merged_x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    merged_x.dedup_by(|a, b| (*a - *b).abs() < 1e-15);

    // Resample both waveforms onto merged grid
    let interp1 = WaveformInterpolator::new(x1, y1)
        .with_method(method)
        .with_extrapolation(ExtrapolationMode::Flat);
    let interp2 = WaveformInterpolator::new(x2, y2)
        .with_method(method)
        .with_extrapolation(ExtrapolationMode::Flat);

    let y1_resampled = interp1.resample(&merged_x)?;
    let y2_resampled = interp2.resample(&merged_x)?;

    Ok((merged_x, y1_resampled, y2_resampled))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Interpolation Method and Mode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_default_method_is_linear() {
        assert_eq!(InterpolationMethod::default(), InterpolationMethod::Linear);
    }

    #[test]
    fn test_default_extrapolation_is_flat() {
        assert_eq!(ExtrapolationMode::default(), ExtrapolationMode::Flat);
    }

    // -------------------------------------------------------------------------
    // WaveformInterpolator Construction Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_interpolator_creation() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 1.0, 4.0];
        let interp = WaveformInterpolator::new(&x, &y);
        assert_eq!(interp.len(), 3);
        assert!(!interp.is_empty());
    }

    #[test]
    fn test_interpolator_x_range() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![0.0; 4];
        let interp = WaveformInterpolator::new(&x, &y);
        assert_eq!(interp.x_range(), Some((1.0, 4.0)));
    }

    #[test]
    fn test_interpolator_empty_waveform() {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let interp = WaveformInterpolator::new(&x, &y);
        assert!(interp.is_empty());
        assert_eq!(interp.x_range(), None);
    }

    #[test]
    fn test_interpolator_single_point() {
        let x = vec![1.0];
        let y = vec![5.0];
        let interp = WaveformInterpolator::new(&x, &y);
        assert_eq!(interp.interpolate_at(1.0).unwrap(), 5.0);
        assert_eq!(interp.interpolate_at(0.0).unwrap(), 5.0); // Flat extrap
        assert_eq!(interp.interpolate_at(10.0).unwrap(), 5.0); // Flat extrap
    }

    // -------------------------------------------------------------------------
    // Linear Interpolation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_linear_interpolation_exact_points() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 2.0, 4.0];
        let interp = WaveformInterpolator::new(&x, &y);

        assert_eq!(interp.interpolate_at(0.0).unwrap(), 0.0);
        assert_eq!(interp.interpolate_at(1.0).unwrap(), 2.0);
        assert_eq!(interp.interpolate_at(2.0).unwrap(), 4.0);
    }

    #[test]
    fn test_linear_interpolation_midpoints() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 2.0, 4.0];
        let interp = WaveformInterpolator::new(&x, &y);

        assert!((interp.interpolate_at(0.5).unwrap() - 1.0).abs() < 1e-10);
        assert!((interp.interpolate_at(1.5).unwrap() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_interpolation_nonlinear_function() {
        // y = x² -> not exactly linear between points
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let y = vec![0.0, 1.0, 4.0, 9.0];
        let interp = WaveformInterpolator::new(&x, &y);

        // At 0.5: linear interp gives 0.5, actual x² = 0.25
        let val = interp.interpolate_at(0.5).unwrap();
        assert!((val - 0.5).abs() < 1e-10);

        // At 2.5: linear interp gives 6.5, actual x² = 6.25
        let val = interp.interpolate_at(2.5).unwrap();
        assert!((val - 6.5).abs() < 1e-10);
    }

    // -------------------------------------------------------------------------
    // Zero-Order Hold Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_zero_order_hold() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![1.0, 3.0, 2.0];
        let interp =
            WaveformInterpolator::new(&x, &y).with_method(InterpolationMethod::ZeroOrderHold);

        assert_eq!(interp.interpolate_at(0.0).unwrap(), 1.0);
        assert_eq!(interp.interpolate_at(0.5).unwrap(), 1.0); // Holds value
        assert_eq!(interp.interpolate_at(0.99).unwrap(), 1.0);
        assert_eq!(interp.interpolate_at(1.0).unwrap(), 3.0);
        assert_eq!(interp.interpolate_at(1.5).unwrap(), 3.0);
    }

    // -------------------------------------------------------------------------
    // Cubic Spline Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_cubic_spline_exact_points() {
        let x = vec![0.0, 1.0, 2.0, 3.0];
        let y = vec![0.0, 1.0, 0.0, 1.0];
        let interp =
            WaveformInterpolator::new(&x, &y).with_method(InterpolationMethod::CubicSpline);

        // Should pass through data points exactly
        assert!((interp.interpolate_at(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((interp.interpolate_at(1.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((interp.interpolate_at(2.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((interp.interpolate_at(3.0).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cubic_spline_smooth() {
        // Quadratic function: y = x²
        let x: Vec<f64> = (0..11).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi * xi).collect();

        let interp =
            WaveformInterpolator::new(&x, &y).with_method(InterpolationMethod::CubicSpline);

        // Should be reasonably close to actual x² for interior points
        let val = interp.interpolate_at(2.5).unwrap();
        // x² at 2.5 = 6.25
        assert!((val - 6.25).abs() < 0.5);
    }

    // -------------------------------------------------------------------------
    // Extrapolation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_flat_extrapolation() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let interp = WaveformInterpolator::new(&x, &y).with_extrapolation(ExtrapolationMode::Flat);

        assert_eq!(interp.interpolate_at(0.0).unwrap(), 2.0); // Left flat
        assert_eq!(interp.interpolate_at(5.0).unwrap(), 6.0); // Right flat
    }

    #[test]
    fn test_linear_extrapolation() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0]; // slope = 2
        let interp =
            WaveformInterpolator::new(&x, &y).with_extrapolation(ExtrapolationMode::Linear);

        // Left: y = 2 + 2*(x-1)
        assert!((interp.interpolate_at(0.0).unwrap() - 0.0).abs() < 1e-10);

        // Right: y = 6 + 2*(x-3)
        assert!((interp.interpolate_at(4.0).unwrap() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_nan_extrapolation() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0, 2.0];
        let interp = WaveformInterpolator::new(&x, &y).with_extrapolation(ExtrapolationMode::NaN);

        assert!(interp.interpolate_at(0.0).unwrap().is_nan());
        assert!(interp.interpolate_at(3.0).unwrap().is_nan());
    }

    #[test]
    fn test_error_extrapolation() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0, 2.0];
        let interp = WaveformInterpolator::new(&x, &y).with_extrapolation(ExtrapolationMode::Error);

        assert!(matches!(
            interp.interpolate_at(0.0),
            Err(InterpolationError::OutOfRange(_))
        ));
        assert!(matches!(
            interp.interpolate_at(3.0),
            Err(InterpolationError::OutOfRange(_))
        ));
    }

    // -------------------------------------------------------------------------
    // Resample Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_resample_same_grid() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 5.0, 10.0];
        let interp = WaveformInterpolator::new(&x, &y);

        let resampled = interp.resample(&x).unwrap();
        assert_eq!(resampled.len(), 3);
        for (orig, resamp) in y.iter().zip(resampled.iter()) {
            assert!((orig - resamp).abs() < 1e-10);
        }
    }

    #[test]
    fn test_resample_finer_grid() {
        let x = vec![0.0, 2.0];
        let y = vec![0.0, 4.0];
        let interp = WaveformInterpolator::new(&x, &y);

        let fine_x = vec![0.0, 0.5, 1.0, 1.5, 2.0];
        let resampled = interp.resample(&fine_x).unwrap();

        assert_eq!(resampled.len(), 5);
        assert!((resampled[0] - 0.0).abs() < 1e-10);
        assert!((resampled[1] - 1.0).abs() < 1e-10);
        assert!((resampled[2] - 2.0).abs() < 1e-10);
        assert!((resampled[3] - 3.0).abs() < 1e-10);
        assert!((resampled[4] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_resample_coarser_grid() {
        let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y = vec![0.0, 1.0, 4.0, 9.0, 16.0];
        let interp = WaveformInterpolator::new(&x, &y);

        let coarse_x = vec![0.0, 2.0, 4.0];
        let resampled = interp.resample(&coarse_x).unwrap();

        assert_eq!(resampled.len(), 3);
        assert!((resampled[0] - 0.0).abs() < 1e-10);
        assert!((resampled[1] - 4.0).abs() < 1e-10);
        assert!((resampled[2] - 16.0).abs() < 1e-10);
    }

    // -------------------------------------------------------------------------
    // Waveform Alignment Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_align_waveforms_same_grid() {
        let x1 = vec![0.0, 1.0, 2.0];
        let y1 = vec![1.0, 2.0, 3.0];
        let x2 = vec![0.0, 1.0, 2.0];
        let y2 = vec![2.0, 4.0, 6.0];

        let (out_x, out_y1, out_y2) =
            align_waveforms(&x1, &y1, &x2, &y2, InterpolationMethod::Linear).unwrap();

        assert_eq!(out_x.len(), 3);
        assert_eq!(out_y1.len(), 3);
        assert_eq!(out_y2.len(), 3);

        for i in 0..3 {
            assert!((out_y1[i] - y1[i]).abs() < 1e-10);
            assert!((out_y2[i] - y2[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_align_waveforms_different_grids() {
        let x1 = vec![0.0, 1.0, 2.0, 3.0];
        let y1 = vec![0.0, 1.0, 2.0, 3.0];
        let x2 = vec![0.0, 3.0]; // Coarser
        let y2 = vec![0.0, 6.0]; // Linear: y = 2x

        let (_out_x, out_y1, out_y2) =
            align_waveforms(&x1, &y1, &x2, &y2, InterpolationMethod::Linear).unwrap();

        // y2 should be resampled: [0, 2, 4, 6]
        assert!((out_y2[0] - 0.0).abs() < 1e-10);
        assert!((out_y2[1] - 2.0).abs() < 1e-10);
        assert!((out_y2[2] - 4.0).abs() < 1e-10);
        assert!((out_y2[3] - 6.0).abs() < 1e-10);

        // y1 unchanged
        for i in 0..4 {
            assert!((out_y1[i] - y1[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_align_waveforms_union() {
        let x1 = vec![0.0, 2.0, 4.0];
        let y1 = vec![0.0, 2.0, 4.0]; // y = x
        let x2 = vec![1.0, 3.0, 5.0];
        let y2 = vec![2.0, 6.0, 10.0]; // y = 2x

        let (merged_x, out_y1, out_y2) =
            align_waveforms_union(&x1, &y1, &x2, &y2, InterpolationMethod::Linear).unwrap();

        // Merged x should have 6 unique points
        assert_eq!(merged_x.len(), 6);
        assert!((merged_x[0] - 0.0).abs() < 1e-10);

        // Both outputs should be same length
        assert_eq!(out_y1.len(), 6);
        assert_eq!(out_y2.len(), 6);
    }

    #[test]
    fn test_align_empty_waveform_error() {
        let x1: Vec<f64> = vec![];
        let y1: Vec<f64> = vec![];
        let x2 = vec![0.0, 1.0];
        let y2 = vec![0.0, 1.0];

        let result = align_waveforms(&x1, &y1, &x2, &y2, InterpolationMethod::Linear);
        assert!(matches!(result, Err(InterpolationError::EmptyWaveform)));
    }

    // -------------------------------------------------------------------------
    // Edge Case Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_two_point_waveform() {
        let x = vec![0.0, 1.0];
        let y = vec![10.0, 20.0];
        let interp = WaveformInterpolator::new(&x, &y);

        assert!((interp.interpolate_at(0.5).unwrap() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_non_uniform_spacing() {
        let x = vec![0.0, 0.1, 0.5, 2.0];
        let y = vec![0.0, 1.0, 5.0, 20.0];
        let interp = WaveformInterpolator::new(&x, &y);

        // Test exact points
        assert_eq!(interp.interpolate_at(0.0).unwrap(), 0.0);
        assert_eq!(interp.interpolate_at(0.1).unwrap(), 1.0);
        assert_eq!(interp.interpolate_at(0.5).unwrap(), 5.0);
        assert_eq!(interp.interpolate_at(2.0).unwrap(), 20.0);
    }

    #[test]
    fn test_negative_values() {
        let x = vec![-2.0, -1.0, 0.0, 1.0];
        let y = vec![-4.0, -2.0, 0.0, 2.0];
        let interp = WaveformInterpolator::new(&x, &y);

        assert!((interp.interpolate_at(-1.5).unwrap() - (-3.0)).abs() < 1e-10);
        assert!((interp.interpolate_at(0.5).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_interpolator_empty_error() {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let interp = WaveformInterpolator::new(&x, &y);

        assert!(matches!(
            interp.interpolate_at(0.0),
            Err(InterpolationError::EmptyWaveform)
        ));
    }

    // -------------------------------------------------------------------------
    // Error Display Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_error_display() {
        let err = InterpolationError::EmptyWaveform;
        assert_eq!(format!("{}", err), "Empty waveform");

        let err = InterpolationError::OutOfRange(1.5);
        assert!(format!("{}", err).contains("1.5"));
    }
}
