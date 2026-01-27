//! Window Functions for FFT
//!
//! Commercial-grade windowing functions used in spectral analysis.
//! Each window trades off between frequency resolution and spectral leakage.

use std::f64::consts::PI;

// =============================================================================
// Window Function Types
// =============================================================================

/// Window function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowFunction {
    /// Rectangular window (no windowing)
    Rectangular,
    /// Hanning (Hann) window - good general purpose
    #[default]
    Hanning,
    /// Hamming window - minimizes first sidelobe
    Hamming,
    /// Blackman window - low sidelobes
    Blackman,
    /// Blackman-Harris window - very low sidelobes
    BlackmanHarris,
    /// Flat-top window - accurate amplitude measurement
    FlatTop,
    /// Kaiser window with configurable beta
    Kaiser,
    /// Gaussian window
    Gaussian,
}

impl WindowFunction {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Rectangular => "Rectangular",
            Self::Hanning => "Hanning",
            Self::Hamming => "Hamming",
            Self::Blackman => "Blackman",
            Self::BlackmanHarris => "Blackman-Harris",
            Self::FlatTop => "Flat-Top",
            Self::Kaiser => "Kaiser",
            Self::Gaussian => "Gaussian",
        }
    }

    /// All window functions
    pub fn all() -> &'static [WindowFunction] {
        &[
            Self::Rectangular,
            Self::Hanning,
            Self::Hamming,
            Self::Blackman,
            Self::BlackmanHarris,
            Self::FlatTop,
            Self::Kaiser,
            Self::Gaussian,
        ]
    }

    /// Coherent gain (DC response)
    pub fn coherent_gain(&self) -> f64 {
        match self {
            Self::Rectangular => 1.0,
            Self::Hanning => 0.5,
            Self::Hamming => 0.54,
            Self::Blackman => 0.42,
            Self::BlackmanHarris => 0.35875,
            Self::FlatTop => 0.21557895,
            Self::Kaiser => 0.5,   // Approximate for beta=5
            Self::Gaussian => 0.5, // Approximate
        }
    }

    /// Noise bandwidth factor
    pub fn noise_bandwidth(&self) -> f64 {
        match self {
            Self::Rectangular => 1.0,
            Self::Hanning => 1.5,
            Self::Hamming => 1.36,
            Self::Blackman => 1.73,
            Self::BlackmanHarris => 2.0,
            Self::FlatTop => 3.77,
            Self::Kaiser => 1.5, // Approximate
            Self::Gaussian => 1.57,
        }
    }

    /// Highest sidelobe level in dB (negative)
    pub fn sidelobe_level(&self) -> f64 {
        match self {
            Self::Rectangular => -13.0,
            Self::Hanning => -32.0,
            Self::Hamming => -43.0,
            Self::Blackman => -58.0,
            Self::BlackmanHarris => -92.0,
            Self::FlatTop => -93.0,
            Self::Kaiser => -60.0, // For beta=5
            Self::Gaussian => -55.0,
        }
    }
}

// =============================================================================
// Window Generation
// =============================================================================

/// Generate window coefficients
pub fn generate_window(window_type: WindowFunction, length: usize) -> Vec<f64> {
    match window_type {
        WindowFunction::Rectangular => rectangular_window(length),
        WindowFunction::Hanning => hanning_window(length),
        WindowFunction::Hamming => hamming_window(length),
        WindowFunction::Blackman => blackman_window(length),
        WindowFunction::BlackmanHarris => blackman_harris_window(length),
        WindowFunction::FlatTop => flat_top_window(length),
        WindowFunction::Kaiser => kaiser_window(length, 5.0),
        WindowFunction::Gaussian => gaussian_window(length, 2.5),
    }
}

/// Generate window with custom parameters
pub fn generate_window_with_param(
    window_type: WindowFunction,
    length: usize,
    param: f64,
) -> Vec<f64> {
    match window_type {
        WindowFunction::Kaiser => kaiser_window(length, param),
        WindowFunction::Gaussian => gaussian_window(length, param),
        _ => generate_window(window_type, length),
    }
}

/// Apply window to data in-place
pub fn apply_window(data: &mut [f64], window: &[f64]) {
    let n = data.len().min(window.len());
    for i in 0..n {
        data[i] *= window[i];
    }
}

/// Apply window and return new vector
pub fn apply_window_copy(data: &[f64], window: &[f64]) -> Vec<f64> {
    let n = data.len().min(window.len());
    (0..n).map(|i| data[i] * window[i]).collect()
}

// =============================================================================
// Window Implementations
// =============================================================================

fn rectangular_window(length: usize) -> Vec<f64> {
    vec![1.0; length]
}

fn hanning_window(length: usize) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    (0..length)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1.0)).cos()))
        .collect()
}

fn hamming_window(length: usize) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    (0..length)
        .map(|i| 0.54 - 0.46 * (2.0 * PI * i as f64 / (n - 1.0)).cos())
        .collect()
}

fn blackman_window(length: usize) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    let a0 = 0.42;
    let a1 = 0.5;
    let a2 = 0.08;

    (0..length)
        .map(|i| {
            let x = i as f64 / (n - 1.0);
            a0 - a1 * (2.0 * PI * x).cos() + a2 * (4.0 * PI * x).cos()
        })
        .collect()
}

fn blackman_harris_window(length: usize) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    let a0 = 0.35875;
    let a1 = 0.48829;
    let a2 = 0.14128;
    let a3 = 0.01168;

    (0..length)
        .map(|i| {
            let x = i as f64 / (n - 1.0);
            a0 - a1 * (2.0 * PI * x).cos() + a2 * (4.0 * PI * x).cos() - a3 * (6.0 * PI * x).cos()
        })
        .collect()
}

fn flat_top_window(length: usize) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    let a0 = 0.21557895;
    let a1 = 0.41663158;
    let a2 = 0.277263158;
    let a3 = 0.083578947;
    let a4 = 0.006947368;

    (0..length)
        .map(|i| {
            let x = i as f64 / (n - 1.0);
            a0 - a1 * (2.0 * PI * x).cos() + a2 * (4.0 * PI * x).cos() - a3 * (6.0 * PI * x).cos()
                + a4 * (8.0 * PI * x).cos()
        })
        .collect()
}

fn kaiser_window(length: usize, beta: f64) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    let denom = bessel_i0(beta);

    (0..length)
        .map(|i| {
            let x = 2.0 * i as f64 / (n - 1.0) - 1.0;
            let arg = beta * (1.0 - x * x).sqrt();
            bessel_i0(arg) / denom
        })
        .collect()
}

fn gaussian_window(length: usize, sigma: f64) -> Vec<f64> {
    if length == 0 {
        return Vec::new();
    }
    let n = length as f64;
    let center = (n - 1.0) / 2.0;

    (0..length)
        .map(|i| {
            let x = (i as f64 - center) / (sigma * center);
            (-0.5 * x * x).exp()
        })
        .collect()
}

/// Modified Bessel function of first kind, order 0
fn bessel_i0(x: f64) -> f64 {
    let ax = x.abs();
    if ax < 3.75 {
        let y = (x / 3.75).powi(2);
        1.0 + y
            * (3.5156229
                + y * (3.0899424
                    + y * (1.2067492 + y * (0.2659732 + y * (0.0360768 + y * 0.0045813)))))
    } else {
        let y = 3.75 / ax;
        (ax.exp() / ax.sqrt())
            * (0.39894228
                + y * (0.01328592
                    + y * (0.00225319
                        + y * (-0.00157565
                            + y * (0.00916281
                                + y * (-0.02057706
                                    + y * (0.02635537 + y * (-0.01647633 + y * 0.00392377))))))))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn approx_eq_rel(a: f64, b: f64, rel_tol: f64) -> bool {
        if b.abs() < EPSILON {
            a.abs() < EPSILON
        } else {
            ((a - b) / b).abs() < rel_tol
        }
    }

    // =========================================================================
    // WindowFunction Enum Tests
    // =========================================================================

    #[test]
    fn test_window_function_default() {
        let w = WindowFunction::default();
        assert_eq!(w, WindowFunction::Hanning);
    }

    #[test]
    fn test_window_function_names() {
        assert_eq!(WindowFunction::Hanning.display_name(), "Hanning");
        assert_eq!(
            WindowFunction::BlackmanHarris.display_name(),
            "Blackman-Harris"
        );
    }

    #[test]
    fn test_window_function_all() {
        let all = WindowFunction::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn test_coherent_gain_rectangular() {
        assert_eq!(WindowFunction::Rectangular.coherent_gain(), 1.0);
    }

    #[test]
    fn test_coherent_gain_hanning() {
        assert!(approx_eq(WindowFunction::Hanning.coherent_gain(), 0.5));
    }

    #[test]
    fn test_noise_bandwidth() {
        assert!(WindowFunction::Hanning.noise_bandwidth() > 1.0);
        assert!(
            WindowFunction::Blackman.noise_bandwidth() > WindowFunction::Hanning.noise_bandwidth()
        );
    }

    #[test]
    fn test_sidelobe_level() {
        // Better windows have more negative sidelobe levels
        assert!(
            WindowFunction::BlackmanHarris.sidelobe_level()
                < WindowFunction::Hanning.sidelobe_level()
        );
    }

    // =========================================================================
    // Rectangular Window Tests
    // =========================================================================

    #[test]
    fn test_rectangular_empty() {
        let w = rectangular_window(0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_rectangular_values() {
        let w = rectangular_window(100);
        assert_eq!(w.len(), 100);
        for val in &w {
            assert_eq!(*val, 1.0);
        }
    }

    // =========================================================================
    // Hanning Window Tests
    // =========================================================================

    #[test]
    fn test_hanning_empty() {
        let w = hanning_window(0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_hanning_length() {
        let w = hanning_window(256);
        assert_eq!(w.len(), 256);
    }

    #[test]
    fn test_hanning_symmetry() {
        let w = hanning_window(101);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_hanning_endpoints() {
        let w = hanning_window(64);
        // Endpoints should be near zero
        assert!(w[0] < 0.01);
        assert!(w[63] < 0.01);
    }

    #[test]
    fn test_hanning_center() {
        let w = hanning_window(101);
        // Center should be 1.0
        assert!(approx_eq_rel(w[50], 1.0, 0.01));
    }

    // =========================================================================
    // Hamming Window Tests
    // =========================================================================

    #[test]
    fn test_hamming_empty() {
        let w = hamming_window(0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_hamming_length() {
        let w = hamming_window(128);
        assert_eq!(w.len(), 128);
    }

    #[test]
    fn test_hamming_symmetry() {
        let w = hamming_window(100);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_hamming_endpoints() {
        let w = hamming_window(64);
        // Hamming endpoints are 0.08 (not zero like Hanning)
        assert!(approx_eq_rel(w[0], 0.08, 0.1));
    }

    // =========================================================================
    // Blackman Window Tests
    // =========================================================================

    #[test]
    fn test_blackman_empty() {
        let w = blackman_window(0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_blackman_symmetry() {
        let w = blackman_window(101);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_blackman_endpoints() {
        let w = blackman_window(64);
        assert!(w[0] < 0.01);
    }

    // =========================================================================
    // Blackman-Harris Window Tests
    // =========================================================================

    #[test]
    fn test_blackman_harris_empty() {
        let w = blackman_harris_window(0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_blackman_harris_symmetry() {
        let w = blackman_harris_window(100);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_blackman_harris_range() {
        let w = blackman_harris_window(256);
        for val in &w {
            assert!(*val >= 0.0 && *val <= 1.0);
        }
    }

    // =========================================================================
    // Flat-Top Window Tests
    // =========================================================================

    #[test]
    fn test_flat_top_empty() {
        let w = flat_top_window(0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_flat_top_symmetry() {
        let w = flat_top_window(101);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_flat_top_center() {
        let w = flat_top_window(101);
        // Flat-top should be near 1.0 at center
        assert!(approx_eq_rel(w[50], 1.0, 0.01));
    }

    // =========================================================================
    // Kaiser Window Tests
    // =========================================================================

    #[test]
    fn test_kaiser_empty() {
        let w = kaiser_window(0, 5.0);
        assert!(w.is_empty());
    }

    #[test]
    fn test_kaiser_length() {
        let w = kaiser_window(128, 5.0);
        assert_eq!(w.len(), 128);
    }

    #[test]
    fn test_kaiser_symmetry() {
        let w = kaiser_window(101, 5.0);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_kaiser_beta_effect() {
        let w_low = kaiser_window(64, 2.0);
        let w_high = kaiser_window(64, 10.0);
        // Higher beta = narrower main lobe = steeper edges
        assert!(w_low[0] > w_high[0]); // Higher beta has smaller endpoints
    }

    // =========================================================================
    // Gaussian Window Tests
    // =========================================================================

    #[test]
    fn test_gaussian_empty() {
        let w = gaussian_window(0, 2.5);
        assert!(w.is_empty());
    }

    #[test]
    fn test_gaussian_symmetry() {
        let w = gaussian_window(101, 2.5);
        let n = w.len();
        for i in 0..n / 2 {
            assert!(approx_eq_rel(w[i], w[n - 1 - i], 0.001));
        }
    }

    #[test]
    fn test_gaussian_center() {
        let w = gaussian_window(101, 2.5);
        assert!(approx_eq(w[50], 1.0));
    }

    // =========================================================================
    // Bessel Function Tests
    // =========================================================================

    #[test]
    fn test_bessel_i0_zero() {
        assert!(approx_eq(bessel_i0(0.0), 1.0));
    }

    #[test]
    fn test_bessel_i0_positive() {
        // I0(1) ≈ 1.266
        assert!(approx_eq_rel(bessel_i0(1.0), 1.266, 0.01));
    }

    #[test]
    fn test_bessel_i0_large() {
        // I0 grows exponentially
        assert!(bessel_i0(5.0) > bessel_i0(3.0));
    }

    // =========================================================================
    // Apply Window Tests
    // =========================================================================

    #[test]
    fn test_apply_window_in_place() {
        let mut data = vec![1.0, 1.0, 1.0, 1.0];
        let window = vec![0.5, 1.0, 1.0, 0.5];
        apply_window(&mut data, &window);
        assert!(approx_eq(data[0], 0.5));
        assert!(approx_eq(data[1], 1.0));
    }

    #[test]
    fn test_apply_window_copy() {
        let data = vec![2.0, 2.0, 2.0, 2.0];
        let window = vec![0.5, 1.0, 1.0, 0.5];
        let result = apply_window_copy(&data, &window);
        assert!(approx_eq(result[0], 1.0));
        assert!(approx_eq(result[1], 2.0));
    }

    #[test]
    fn test_apply_window_mismatched_length() {
        let mut data = vec![1.0, 1.0, 1.0];
        let window = vec![0.5, 1.0];
        apply_window(&mut data, &window);
        // Only first 2 elements should be modified
        assert!(approx_eq(data[0], 0.5));
        assert!(approx_eq(data[2], 1.0)); // Unchanged
    }

    // =========================================================================
    // Generate Window Tests
    // =========================================================================

    #[test]
    fn test_generate_window_all_types() {
        for wtype in WindowFunction::all() {
            let w = generate_window(*wtype, 64);
            assert_eq!(w.len(), 64);
            // Most windows should have non-negative values
            // Flat-top window can go negative at edges (by design for amplitude accuracy)
            for val in &w {
                assert!(*val >= -0.1, "Window value {} too negative", val);
            }
        }
    }

    #[test]
    fn test_generate_window_with_param() {
        let w1 = generate_window_with_param(WindowFunction::Kaiser, 64, 3.0);
        let w2 = generate_window_with_param(WindowFunction::Kaiser, 64, 8.0);
        assert_ne!(w1[0], w2[0]);
    }
}
