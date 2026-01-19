//! SIMD Numerical Integration
//!
//! Accelerated numerical integration routines commonly used in
//! circuit simulation for waveform analysis and time-stepping.

use super::{SIMD_WIDTH, should_use_simd};
use crate::Value;
use wide::f64x4;

//=============================================================================
// Trapezoidal Integration
//=============================================================================

/// Compute the trapezoidal integral of values over time points.
///
/// Computes: `sum( 0.5 * (y[i] + y[i-1]) * (t[i] - t[i-1]) )`
///
/// This is the workhorse for Fourier analysis and waveform measurements.
///
/// # Arguments
///
/// * `time` - Time points (must be sorted in ascending order)
/// * `values` - Function values at each time point
///
/// # Returns
///
/// The approximate integral using the trapezoidal rule.
///
/// # Panics
///
/// Panics if `time.len() != values.len()`.
#[inline]
pub fn trapezoidal_integrate(time: &[Value], values: &[Value]) -> Value {
    assert_eq!(
        time.len(),
        values.len(),
        "time and values must have same length"
    );

    if time.len() < 2 {
        return 0.0;
    }

    let n = time.len() - 1;

    // For SIMD, we process groups of 4 integration steps
    // Each step needs: t[i], t[i+1], v[i], v[i+1]
    if !should_use_simd(n) {
        return trapezoidal_integrate_scalar(time, values);
    }

    let half = f64x4::splat(0.5);
    let aligned_steps = n - (n % SIMD_WIDTH);
    let mut sum_vec = f64x4::ZERO;

    let mut i = 0;
    while i < aligned_steps {
        // Load overlapping pairs
        // We need t[i..i+4], t[i+1..i+5], v[i..i+4], v[i+1..i+5]
        let t0 = f64x4::from(&time[i..i + SIMD_WIDTH]);
        let t1 = f64x4::from(&time[i + 1..i + 1 + SIMD_WIDTH]);
        let v0 = f64x4::from(&values[i..i + SIMD_WIDTH]);
        let v1 = f64x4::from(&values[i + 1..i + 1 + SIMD_WIDTH]);

        let dt = t1 - t0;
        let avg = v0 + v1;
        sum_vec += half * avg * dt;

        i += SIMD_WIDTH;
    }

    // Horizontal sum
    let arr: [f64; 4] = sum_vec.into();
    let mut result = arr[0] + arr[1] + arr[2] + arr[3];

    // Process remainder
    for j in aligned_steps..n {
        let dt = time[j + 1] - time[j];
        result += 0.5 * (values[j] + values[j + 1]) * dt;
    }

    result
}

/// Scalar fallback for trapezoidal integration.
#[inline]
fn trapezoidal_integrate_scalar(time: &[Value], values: &[Value]) -> Value {
    let mut integral = 0.0;
    for i in 1..time.len() {
        let dt = time[i] - time[i - 1];
        integral += 0.5 * (values[i] + values[i - 1]) * dt;
    }
    integral
}

//=============================================================================
// Fourier-Specific Integration
//=============================================================================

/// Compute the cosine Fourier coefficient: (2/T) * integral(f(t) * cos(omega*t) dt).
///
/// This is a core operation for Fourier analysis.
///
/// # Arguments
///
/// * `time` - Time points
/// * `values` - Signal values
/// * `omega` - Angular frequency (2*pi*f)
/// * `t_start` - Phase reference time (typically the window start)
///
/// # Returns
///
/// The cosine coefficient `a_n` for the Fourier series.
#[inline]
pub fn fourier_cosine_coefficient(
    time: &[Value],
    values: &[Value],
    omega: Value,
    t_start: Value,
) -> Value {
    assert_eq!(
        time.len(),
        values.len(),
        "time and values must have same length"
    );

    if time.len() < 2 {
        return 0.0;
    }

    let duration = time[time.len() - 1] - time[0];
    if duration <= 0.0 {
        return 0.0;
    }

    // Compute f(t) * cos(omega * (t - t_start)) and integrate
    // For SIMD, we'd need SIMD sin/cos which is complex
    // Use scalar for now, but structure for future SIMD sin/cos
    let mut integral = 0.0;

    for i in 1..time.len() {
        let dt = time[i] - time[i - 1];
        let t_mid = 0.5 * (time[i] + time[i - 1]);
        let v_mid = 0.5 * (values[i] + values[i - 1]);
        let cos_val = (omega * (t_mid - t_start)).cos();
        integral += v_mid * cos_val * dt;
    }

    2.0 / duration * integral
}

/// Compute the sine Fourier coefficient: (2/T) * integral(f(t) * sin(omega*t) dt).
#[inline]
pub fn fourier_sine_coefficient(
    time: &[Value],
    values: &[Value],
    omega: Value,
    t_start: Value,
) -> Value {
    assert_eq!(
        time.len(),
        values.len(),
        "time and values must have same length"
    );

    if time.len() < 2 {
        return 0.0;
    }

    let duration = time[time.len() - 1] - time[0];
    if duration <= 0.0 {
        return 0.0;
    }

    let mut integral = 0.0;

    for i in 1..time.len() {
        let dt = time[i] - time[i - 1];
        let t_mid = 0.5 * (time[i] + time[i - 1]);
        let v_mid = 0.5 * (values[i] + values[i - 1]);
        let sin_val = (omega * (t_mid - t_start)).sin();
        integral += v_mid * sin_val * dt;
    }

    2.0 / duration * integral
}

/// Compute both sine and cosine coefficients together.
///
/// This is more efficient than calling both functions separately
/// since they share most of the setup computation.
///
/// # Returns
///
/// Tuple of (a_n, b_n) - cosine and sine coefficients.
#[inline]
pub fn fourier_coefficients(
    time: &[Value],
    values: &[Value],
    omega: Value,
    t_start: Value,
) -> (Value, Value) {
    assert_eq!(
        time.len(),
        values.len(),
        "time and values must have same length"
    );

    if time.len() < 2 {
        return (0.0, 0.0);
    }

    let duration = time[time.len() - 1] - time[0];
    if duration <= 0.0 {
        return (0.0, 0.0);
    }

    let mut cos_integral = 0.0;
    let mut sin_integral = 0.0;

    for i in 1..time.len() {
        let dt = time[i] - time[i - 1];
        let t_mid = 0.5 * (time[i] + time[i - 1]);
        let v_mid = 0.5 * (values[i] + values[i - 1]);
        let phase = omega * (t_mid - t_start);
        let (sin_val, cos_val) = phase.sin_cos();

        cos_integral += v_mid * cos_val * dt;
        sin_integral += v_mid * sin_val * dt;
    }

    let scale = 2.0 / duration;
    (scale * cos_integral, scale * sin_integral)
}

//=============================================================================
// RMS and Average Calculations
//=============================================================================

/// Compute the average (DC) value of a waveform.
#[inline]
pub fn average(time: &[Value], values: &[Value]) -> Value {
    if time.len() < 2 {
        return values.first().copied().unwrap_or(0.0);
    }

    let duration = time[time.len() - 1] - time[0];
    if duration <= 0.0 {
        return values.first().copied().unwrap_or(0.0);
    }

    trapezoidal_integrate(time, values) / duration
}

/// Compute the RMS (Root Mean Square) value of a waveform.
#[inline]
pub fn rms(time: &[Value], values: &[Value]) -> Value {
    if time.len() < 2 {
        return values.first().copied().unwrap_or(0.0).abs();
    }

    let duration = time[time.len() - 1] - time[0];
    if duration <= 0.0 {
        return values.first().copied().unwrap_or(0.0).abs();
    }

    // Compute integral of values^2
    let squared_values: Vec<_> = values.iter().map(|v| v * v).collect();
    let mean_square = trapezoidal_integrate(time, &squared_values) / duration;

    mean_square.sqrt()
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_trapezoidal_constant() {
        // Integral of constant 2.0 from 0 to 1 = 2.0
        let time: Vec<_> = (0..=100).map(|i| i as f64 / 100.0).collect();
        let values = vec![2.0; time.len()];
        let result = trapezoidal_integrate(&time, &values);
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_trapezoidal_linear() {
        // Integral of x from 0 to 1 = 0.5
        let n = 100;
        let time: Vec<_> = (0..=n).map(|i| i as f64 / n as f64).collect();
        let values: Vec<_> = time.clone();
        let result = trapezoidal_integrate(&time, &values);
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_trapezoidal_sine() {
        // Integral of sin(x) from 0 to pi = 2.0
        let n = 1000;
        let time: Vec<_> = (0..=n).map(|i| i as f64 / n as f64 * PI).collect();
        let values: Vec<_> = time.iter().map(|t| t.sin()).collect();
        let result = trapezoidal_integrate(&time, &values);
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_average() {
        // Average of sin over one period = 0
        let n = 1000;
        let time: Vec<_> = (0..=n).map(|i| i as f64 / n as f64 * 2.0 * PI).collect();
        let values: Vec<_> = time.iter().map(|t| t.sin()).collect();
        let result = average(&time, &values);
        assert!(result.abs() < 0.01);
    }

    #[test]
    fn test_rms_sine() {
        // RMS of sin = 1/sqrt(2) ≈ 0.707
        let n = 1000;
        let time: Vec<_> = (0..=n).map(|i| i as f64 / n as f64 * 2.0 * PI).collect();
        let values: Vec<_> = time.iter().map(|t| t.sin()).collect();
        let result = rms(&time, &values);
        let expected = 1.0 / 2.0_f64.sqrt();
        assert!((result - expected).abs() < 0.01);
    }

    #[test]
    fn test_fourier_pure_sine() {
        // For f(t) = sin(omega*t), a_1 = 0, b_1 = 1
        let omega = 2.0 * PI;
        let n = 1000;
        let time: Vec<_> = (0..=n).map(|i| i as f64 / n as f64).collect();
        let values: Vec<_> = time.iter().map(|t| (omega * t).sin()).collect();

        let (a, b) = fourier_coefficients(&time, &values, omega, 0.0);

        // a_1 should be ~0, b_1 should be ~1
        assert!(a.abs() < 0.05, "a = {}", a);
        assert!((b - 1.0).abs() < 0.05, "b = {}", b);
    }

    #[test]
    fn test_fourier_pure_cosine() {
        // For f(t) = cos(omega*t), a_1 = 1, b_1 = 0
        let omega = 2.0 * PI;
        let n = 1000;
        let time: Vec<_> = (0..=n).map(|i| i as f64 / n as f64).collect();
        let values: Vec<_> = time.iter().map(|t| (omega * t).cos()).collect();

        let (a, b) = fourier_coefficients(&time, &values, omega, 0.0);

        assert!((a - 1.0).abs() < 0.05, "a = {}", a);
        assert!(b.abs() < 0.05, "b = {}", b);
    }
}
