//! Smooth transition functions for C1 continuous MOSFET region transitions
//!
//! These functions provide mathematically smooth (C1 continuous) approximations
//! of discontinuous operations like max, min, and step. This is critical for
//! Newton-Raphson convergence in circuit simulation as it ensures continuous
//! first derivatives (conductances).
//!
//! # Usage
//! Instead of hard region transitions like:
//! ```text
//! if vgs > vth { ... } else { ... }
//! ```
//! Use smooth transitions:
//! ```text
//! let vgt = smooth_positive(vgs - vth, SMOOTH_VOLTAGE);
//! ```

use crate::Value;

/// Smoothing voltage range for region transitions (in Volts)
///
/// This controls the width of smooth region transitions.
/// - Smaller values give sharper but more accurate transitions
/// - Larger values give smoother but less accurate transitions
/// - 50mV is a good balance for most simulations
pub const SMOOTH_VOLTAGE: Value = 0.05;

/// Smooth step function using tanh for C1 continuous transitions
///
/// Returns a value smoothly transitioning from 0 to 1 as x goes from -∞ to +∞.
/// The transition width is controlled by the `width` parameter.
///
/// # Mathematical Definition
/// ```text
/// smooth_step(x, w) = 0.5 * (1 + tanh(x/w))
/// ```
///
/// # Properties
/// - `smooth_step(x, w) ≈ 0` when `x << -w`
/// - `smooth_step(0, w) = 0.5`
/// - `smooth_step(x, w) ≈ 1` when `x >> w`
/// - First derivative is continuous everywhere
#[inline]
pub fn smooth_step(x: Value, width: Value) -> Value {
    0.5 * (1.0 + (x / width.max(1e-12)).tanh())
}

/// Smooth maximum function: returns approximately max(a, b) but C1 continuous
///
/// Uses a weighted blend based on smooth_step, with a small correction term
/// to ensure the result is slightly above the true maximum near the transition.
///
/// # Arguments
/// * `a` - First value
/// * `b` - Second value  
/// * `smoothing` - Width of transition region
///
/// # Returns
/// - When `a >> b + smoothing`, returns approximately `a`
/// - When `b >> a + smoothing`, returns approximately `b`
/// - When `a ≈ b`, returns smooth blend slightly above both
#[inline]
pub fn smooth_max(a: Value, b: Value, smoothing: Value) -> Value {
    let diff = a - b;
    let s = smooth_step(diff, smoothing);
    s * a + (1.0 - s) * b + smoothing * 0.5 * (1.0 - (diff / smoothing.max(1e-12)).tanh().powi(2))
}

/// Smooth minimum function: returns approximately min(a, b) but C1 continuous
///
/// Implemented as the dual of smooth_max: `smooth_min(a, b) = -smooth_max(-a, -b)`
#[inline]
pub fn smooth_min(a: Value, b: Value, smoothing: Value) -> Value {
    -smooth_max(-a, -b, smoothing)
}

/// Smooth positive part: returns approximately max(x, 0) but C1 continuous
///
/// This is crucial for the cutoff→on transition in MOSFETs where the
/// gate overdrive `Vgs - Vth` transitions from negative to positive.
///
/// # Properties
/// - `smooth_positive(x) ≈ 0` when `x << 0`
/// - `smooth_positive(x) ≈ x` when `x >> 0`
/// - Provides smooth, differentiable transition at x = 0
#[inline]
pub fn smooth_positive(x: Value, smoothing: Value) -> Value {
    smooth_max(x, 0.0, smoothing)
}
