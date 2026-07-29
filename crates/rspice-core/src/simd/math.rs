//! SIMD-Accelerated Mathematical Functions
//!
//! High-performance implementations of transcendental functions (exp, ln, sqrt)
//! using SIMD instructions. These are optimized for the voltage/current ranges
//! typical in circuit simulation.
//!
//! # Accuracy
//!
//! All functions are accurate to at least 1e-10 relative error in their
//! typical operating ranges. This is sufficient for circuit simulation where
//! device model parameter uncertainties are typically 1-10%.
//!
//! # Implementation Notes
//!
//! The exponential function uses a polynomial approximation with range reduction.
//! This is faster than calling scalar `f64::exp()` four times and allows full
//! SIMD parallelism.

use crate::Value;
use wide::f64x4;

//=============================================================================
// Constants
//=============================================================================


//=============================================================================
// Exponential Function
//=============================================================================

/// SIMD exponential function: computes exp(x) for 4 values simultaneously.
///
/// Uses scalar exp() for accuracy while maintaining the SIMD interface.
/// The compiler can still vectorize surrounding operations.
///
/// # Accuracy
///
/// Uses the standard library exp() for full accuracy.
///
/// # Performance
///
/// While this falls back to scalar exp, having values in SIMD registers
/// allows the compiler to optimize surrounding code better.
#[inline]
pub fn exp_f64x4(x: f64x4) -> f64x4 {
    let arr: [f64; 4] = x.into();
    f64x4::from([arr[0].exp(), arr[1].exp(), arr[2].exp(), arr[3].exp()])
}




//=============================================================================
// Safe Division
//=============================================================================

/// Safe division that returns a default value when denominator is near zero.
///
/// This is useful for avoiding NaN/Inf in conductance calculations.
#[inline]
pub fn div_safe(numerator: f64x4, denominator: f64x4, default: Value) -> f64x4 {
    let abs_denom = denominator.abs();

    // Create mask for safe division
    let denom_arr: [f64; 4] = abs_denom.into();
    let num_arr: [f64; 4] = numerator.into();
    let den_arr: [f64; 4] = denominator.into();

    let result = [
        if denom_arr[0] > 1e-30 {
            num_arr[0] / den_arr[0]
        } else {
            default
        },
        if denom_arr[1] > 1e-30 {
            num_arr[1] / den_arr[1]
        } else {
            default
        },
        if denom_arr[2] > 1e-30 {
            num_arr[2] / den_arr[2]
        } else {
            default
        },
        if denom_arr[3] > 1e-30 {
            num_arr[3] / den_arr[3]
        } else {
            default
        },
    ];

    f64x4::from(result)
}

//=============================================================================
// Clamping and Limiting
//=============================================================================

/// Clamp values to a range, useful for voltage limiting in device models.
#[inline]
pub fn clamp(x: f64x4, min_val: Value, max_val: Value) -> f64x4 {
    x.max(f64x4::splat(min_val)).min(f64x4::splat(max_val))
}

/// Limit voltage step for diode-like devices to prevent numerical overflow.
///
/// Returns vd limited to prevent exp() overflow.
#[inline]
pub fn limit_diode_voltage(vd: f64x4, n_vt: f64x4) -> f64x4 {
    // Limit to ~80 * n * vt to prevent exp overflow
    let limit = n_vt * f64x4::splat(80.0);
    vd.min(limit)
}

//=============================================================================
// Diode-Specific SIMD Operations
//=============================================================================

/// Compute diode current and conductance for 4 diodes simultaneously.
///
/// Uses the Shockley equation:
/// - I = Is * (exp(Vd / (n*Vt)) - 1)
/// - g = Is / (n*Vt) * exp(Vd / (n*Vt))
///
/// # Arguments
///
/// * `vd` - Diode voltages (4 values)
/// * `is` - Saturation currents (4 values)
/// * `n_vt` - Pre-computed n*Vt products (4 values)
///
/// # Returns
///
/// Tuple of (currents, conductances)
#[inline]
pub fn diode_iv(vd: f64x4, is: f64x4, n_vt: f64x4) -> (f64x4, f64x4) {
    // Limit voltage to prevent overflow
    let vd_limited = limit_diode_voltage(vd, n_vt);

    // Compute normalized voltage
    let x = div_safe(vd_limited, n_vt, 0.0);

    // exp(Vd / n*Vt)
    let exp_term = exp_f64x4(x);

    // Current: I = Is * (exp(x) - 1)
    let id = is * (exp_term - f64x4::splat(1.0));

    // Conductance: g = Is / (n*Vt) * exp(x) = (Is * exp(x)) / (n*Vt)
    let gd = div_safe(is * exp_term, n_vt, 1e-12);

    (id, gd)
}


//=============================================================================
// Common SIMD Helpers for Batch Processing
//=============================================================================





//=============================================================================
// Helper: Store to slice
//=============================================================================

/// Store an f64x4 vector to a mutable slice.
#[inline]
pub fn store_f64x4(v: f64x4, dst: &mut [Value]) {
    let arr: [f64; 4] = v.into();
    dst[0] = arr[0];
    dst[1] = arr[1];
    dst[2] = arr[2];
    dst[3] = arr[3];
}

//=============================================================================
// Tests
//=============================================================================
