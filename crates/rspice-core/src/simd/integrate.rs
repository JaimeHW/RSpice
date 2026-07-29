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
