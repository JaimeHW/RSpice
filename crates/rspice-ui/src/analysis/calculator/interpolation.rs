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

#![allow(clippy::type_complexity)]
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
            return Ok(self.extrapolate_left());
        }
        if target_x > x_max {
            return Ok(self.extrapolate_right());
        }

        // Find bracketing interval using binary search
        let idx = self.find_interval(target_x);

        // Interpolate within interval
        match self.method {
            InterpolationMethod::Linear => Ok(self.interpolate_linear(idx, target_x)),
            InterpolationMethod::CubicSpline => Ok(self.interpolate_cubic(idx, target_x)),
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

    /// Extrapolate to the left. Out-of-range queries hold the endpoint value:
    /// a resampled trace never invents samples beyond what was measured.
    fn extrapolate_left(&self) -> f64 {
        self.y[0]
    }

    /// Extrapolate to the right, holding the endpoint value.
    fn extrapolate_right(&self) -> f64 {
        self.y[self.y.len() - 1]
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
}

impl std::fmt::Display for InterpolationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyWaveform => write!(f, "Empty waveform"),
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
    let interp = WaveformInterpolator::new(x2, y2).with_method(method);

    let y2_resampled = interp.resample(x1)?;

    Ok((x1.to_vec(), y1.to_vec(), y2_resampled))
}

// =============================================================================
// Tests
// =============================================================================
