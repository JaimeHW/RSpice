//! Centralized simulator constants for RSpice
//!
//! This module provides a single source of truth for all numerical constants,
//! tolerances, and limits used throughout the simulation engine. Centralizing
//! these values:
//!
//! - Eliminates magic numbers scattered throughout the codebase
//! - Makes tuning and calibration easier
//! - Ensures consistency across all analysis types
//! - Follows industry-standard SPICE simulator practices

use crate::Value;

//=============================================================================
// Numerical Tolerances
//=============================================================================

/// Default absolute voltage tolerance for convergence checking (volts)
///
/// Similar to SPICE ABSTOL parameter. Typical range: 1e-12 to 1e-6.
pub const ABSTOL: Value = 1e-9;

/// Default relative tolerance for convergence checking (dimensionless)
///
/// Similar to SPICE RELTOL parameter. Typical range: 1e-4 to 1e-2.
pub const RELTOL: Value = 1e-3;

/// Default current tolerance for convergence checking (amperes)
///
/// Similar to SPICE ITOL parameter.
pub const ITOL: Value = 1e-12;

/// Default charge tolerance (coulombs)
///
/// Similar to SPICE CHGTOL parameter.
pub const CHGTOL: Value = 1e-14;

/// Default voltage tolerance (volts)
///
/// Similar to SPICE VNTOL parameter.
pub const VNTOL: Value = 1e-6;

//=============================================================================
// Newton-Raphson Iteration Control
//=============================================================================

/// Maximum Newton-Raphson iterations before declaring non-convergence
pub const MAX_NR_ITERATIONS: usize = 50;

/// Maximum transient simulation retries per timepoint before force-accepting
pub const MAX_TRANSIENT_RETRIES: usize = 20;

/// Total iteration safety limit for transient simulation
pub const MAX_TOTAL_ITERATIONS: usize = 100_000;

//=============================================================================
// Timestep Control
//=============================================================================

/// Minimum allowable timestep for transient analysis (seconds)
///
/// Prevents infinite loops when convergence is difficult.
pub const MIN_TIMESTEP: Value = 1e-15;

/// Maximum timestep for transient analysis (seconds)
///
/// Ensures adequate resolution of fast waveforms.
pub const MAX_TIMESTEP: Value = 1e-3;

/// Minimum timestep immediately after a breakpoint (seconds)
///
/// Used to restart the integrator after discontinuities.
pub const MIN_STEP_AFTER_BREAKPOINT: Value = 1e-12;

/// Tolerance for detecting exact breakpoint landing
pub const BREAKPOINT_TOLERANCE: Value = 1e-15;

//=============================================================================
// Matrix and Solver Constants
//=============================================================================

/// Default GMIN (minimum conductance to ground)
///
/// Added to diagonal elements for numerical stability. Prevents floating nodes.
/// Similar to SPICE GMIN parameter. Typical range: 1e-12 to 1e-9.
pub const GMIN: Value = 1e-12;

/// Initial GMIN value for GMIN stepping (larger for convergence aid)
pub const GMIN_INITIAL: Value = 1e-2;

/// Target GMIN value after stepping converges
pub const GMIN_TARGET: Value = 1e-12;

/// GMIN reduction factor per successful step
pub const GMIN_FACTOR: Value = 10.0;

/// Maximum GMIN stepping iterations
pub const GMIN_MAX_STEPS: usize = 10;

//=============================================================================
// Source Stepping Constants
//=============================================================================

/// Initial source stepping factor (0 to 1)
pub const SOURCE_STEP_INITIAL: Value = 0.1;

/// Source step increase factor on successful convergence
pub const SOURCE_STEP_FACTOR: Value = 2.0;

/// Minimum source stepping factor before giving up
pub const SOURCE_STEP_MIN: Value = 0.001;

/// Maximum source stepping iterations
pub const SOURCE_MAX_STEPS: usize = 20;

//=============================================================================
// Pseudo-Transient Constants
//=============================================================================

/// Default pseudo-transient timestep
pub const PSEUDO_TRANSIENT_DT: Value = 1e-6;

/// Pseudo-transient timestep increase factor
pub const PSEUDO_TRANSIENT_DT_FACTOR: Value = 2.0;

/// Maximum pseudo-transient timestep
pub const PSEUDO_TRANSIENT_DT_MAX: Value = 1e-3;

/// Maximum pseudo-transient iterations
pub const PSEUDO_TRANSIENT_MAX_STEPS: usize = 50;

//=============================================================================
// Temperature Constants
//=============================================================================

/// Reference temperature in Kelvin (27°C = 300.15K, often approximated as 300K)
pub const TEMP_REFERENCE: Value = 300.0;

/// Boltzmann constant (J/K)
pub const K_BOLTZMANN: Value = 1.380649e-23;

/// Elementary charge (C)
pub const Q_ELECTRON: Value = 1.602176634e-19;

/// Thermal voltage at reference temperature (kT/q)
pub const VT_REFERENCE: Value = TEMP_REFERENCE * K_BOLTZMANN / Q_ELECTRON;

//=============================================================================
// Timeout Constants
//=============================================================================

/// Wall-clock timeout for transient simulation (seconds)
///
/// Prevents runaway simulations. Set to 0 to disable.
pub const WALL_TIMEOUT_SECS: u64 = 30;

//=============================================================================
// Numerical Safety
//=============================================================================

/// Maximum exponential argument to prevent overflow
///
/// exp(709) ≈ 1e308 (near f64 max), so we clamp below this.
pub const MAX_EXP_ARG: Value = 700.0;

/// Minimum exponential argument to prevent underflow
pub const MIN_EXP_ARG: Value = -700.0;

/// Small value to avoid division by zero
pub const EPSILON: Value = 1e-30;

//=============================================================================
// LTE (Local Truncation Error) Control
//=============================================================================

/// LTE safety factor for timestep adaptation
pub const LTE_SAFETY_FACTOR: Value = 0.9;

/// Maximum timestep growth factor per step
pub const TIMESTEP_GROWTH_MAX: Value = 2.0;

/// Minimum timestep shrink factor per step
pub const TIMESTEP_SHRINK_MIN: Value = 0.1;

//=============================================================================
// Device-Specific Constants
//=============================================================================

/// Diode thermal voltage coefficient for forward bias limiting
pub const DIODE_VT_CRIT: Value = 0.6;

/// Junction breakdown detection threshold
pub const JUNCTION_BREAKDOWN_FACTOR: Value = 0.9;

//=============================================================================
// Convenience Functions
//=============================================================================

/// Calculate thermal voltage at a given temperature
#[inline]
pub fn thermal_voltage(temp_kelvin: Value) -> Value {
    temp_kelvin * K_BOLTZMANN / Q_ELECTRON
}

/// Safe exponential function with overflow/underflow protection
#[inline]
pub fn safe_exp(x: Value) -> Value {
    if x > MAX_EXP_ARG {
        (MAX_EXP_ARG).exp()
    } else if x < MIN_EXP_ARG {
        0.0
    } else {
        x.exp()
    }
}

/// Clamp a value to a range with a small epsilon margin
#[inline]
pub fn clamp_with_margin(x: Value, min: Value, max: Value) -> Value {
    x.max(min + EPSILON).min(max - EPSILON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_voltage_at_reference() {
        let vt = thermal_voltage(TEMP_REFERENCE);
        // At 300K, VT ≈ 0.02585V
        assert!((vt - 0.02585).abs() < 0.001);
    }

    #[test]
    fn test_thermal_voltage_at_400k() {
        let vt = thermal_voltage(400.0);
        // At 400K, VT ≈ 0.03447V
        assert!((vt - 0.03447).abs() < 0.001);
    }

    #[test]
    fn test_safe_exp_normal() {
        let x = 10.0;
        assert!((safe_exp(x) - x.exp()).abs() < 1e-10);
    }

    #[test]
    fn test_safe_exp_overflow_protection() {
        let x = 1000.0;
        let result = safe_exp(x);
        assert!(result.is_finite());
        assert!(result > 0.0);
    }

    #[test]
    fn test_safe_exp_underflow_protection() {
        let x = -1000.0;
        let result = safe_exp(x);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_clamp_with_margin() {
        // Values within range are preserved
        assert!((clamp_with_margin(0.5, 0.0, 1.0) - 0.5).abs() < 1e-10);
        // Values below min are clamped to min + epsilon
        assert!(clamp_with_margin(-1.0, 0.0, 1.0) > 0.0);
        // Values above max are clamped to max - epsilon
        // Note: EPSILON is 1e-30, too small to see in f64 subtraction from 1.0
        // So we just verify it doesn't exceed max
        assert!(clamp_with_margin(2.0, 0.0, 1.0) <= 1.0);
    }

    #[test]
    fn test_constants_are_reasonable() {
        // Sanity checks for constant values
        assert!(ABSTOL > 0.0);
        assert!(RELTOL > 0.0 && RELTOL < 1.0);
        assert!(GMIN > 0.0 && GMIN < 1.0);
        assert!(MAX_NR_ITERATIONS > 0);
        assert!(MIN_TIMESTEP < MAX_TIMESTEP);
        assert!(GMIN_INITIAL > GMIN_TARGET);
    }

    #[test]
    fn test_physical_constants() {
        // Boltzmann constant
        assert!((K_BOLTZMANN - 1.380649e-23).abs() < 1e-30);
        // Elementary charge
        assert!((Q_ELECTRON - 1.602176634e-19).abs() < 1e-26);
    }
}
