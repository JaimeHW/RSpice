//! Constructor argument checks shared by every configuration type.
//!
//! A configuration is validated where it is built, not where it is used, so an
//! out-of-range tolerance raises at the call that set it instead of surfacing
//! as a solver failure much later with no trace back to its cause.

use super::*;

/// Exact float comparison for configuration equality.
///
/// Bit equality rather than `==` so a configuration compares equal to itself
/// even when a field carries a sentinel such as NaN, and so `inf` ceilings
/// compare as the same policy.
pub(super) fn same_float(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits()
}

pub(super) fn validate_positive(name: &str, value: f64) -> PyResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(crate::errors::value_error(format!(
            "{name} must be a positive finite number, got {value}"
        )))
    }
}

pub(super) fn validate_nonnegative(name: &str, value: f64) -> PyResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(crate::errors::value_error(format!(
            "{name} must be a non-negative finite number, got {value}"
        )))
    }
}

/// Accept a positive finite value or `+inf`, which means "no hard cap".
///
/// Mirrors core's `validate_positive_or_unbounded` so a ceiling that reads
/// back as `inf` can also be written back.
pub(super) fn validate_positive_or_unbounded(name: &str, value: f64) -> PyResult<f64> {
    if value == f64::INFINITY || (value.is_finite() && value > 0.0) {
        Ok(value)
    } else {
        Err(crate::errors::value_error(format!(
            "{name} must be a positive finite number, or float('inf') for no cap, got {value}"
        )))
    }
}

pub(super) fn validate_positive_usize(name: &str, value: usize) -> PyResult<usize> {
    if value > 0 {
        Ok(value)
    } else {
        Err(crate::errors::value_error(format!(
            "{name} must be at least 1, got {value}"
        )))
    }
}

pub(super) fn validate_timestep_window(min_timestep: f64, max_timestep: f64) -> PyResult<()> {
    if min_timestep <= max_timestep {
        Ok(())
    } else {
        Err(crate::errors::value_error(format!(
            "min_timestep ({min_timestep}) must be <= max_timestep ({max_timestep})"
        )))
    }
}

pub(super) fn validate_gmin_window(gmin_initial: f64, gmin_target: f64) -> PyResult<()> {
    if gmin_initial >= gmin_target {
        Ok(())
    } else {
        Err(crate::errors::value_error(format!(
            "gmin_initial ({gmin_initial}) must be >= gmin_target ({gmin_target})"
        )))
    }
}
