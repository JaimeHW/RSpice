//! Canonical transient-directive execution semantics shared by every frontend.

use thiserror::Error;

use crate::Value;

/// Why a `.TRAN` directive cannot produce a solver maximum-step ceiling.
///
/// The variants intentionally identify the authored field that is invalid.
/// Frontends may translate the error into their native exception/diagnostic
/// type without parsing a display string.
#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum TransientMaximumStepError {
    #[error("transient TSTEP must be finite and positive, got {step}")]
    InvalidPrintStep { step: Value },
    #[error("transient TSTOP must be finite and positive, got {stop}")]
    InvalidStopTime { stop: Value },
    #[error("transient TSTART must be finite and non-negative, got {start}")]
    InvalidStartTime { start: Value },
    #[error("transient TSTOP must be greater than TSTART, got TSTART={start}, TSTOP={stop}")]
    InvalidWindow { start: Value, stop: Value },
    #[error("transient TMAX must be finite and positive, got {maximum_step}")]
    InvalidExplicitMaximumStep { maximum_step: Value },
    #[error("transient default TMAX underflowed for TSTEP={step}, TSTART={start}, TSTOP={stop}")]
    UnrepresentableDefault {
        step: Value,
        start: Value,
        stop: Value,
    },
}

/// Resolve the solver ceiling for a SPICE `.TRAN` request.
///
/// An omitted `TSTART` is zero. An explicit positive finite `TMAX` wins.
/// Otherwise ngspice-compatible semantics use
/// `min(TSTEP, (TSTOP - TSTART) / 50)`. Every authored field is validated even
/// when `TMAX` is explicit, so an otherwise malformed directive cannot be
/// made to look valid by supplying a ceiling.
pub fn resolve_transient_maximum_step(
    step: Value,
    stop: Value,
    start: Option<Value>,
    explicit_maximum_step: Option<Value>,
) -> Result<Value, TransientMaximumStepError> {
    if !step.is_finite() || step <= 0.0 {
        return Err(TransientMaximumStepError::InvalidPrintStep { step });
    }
    if !stop.is_finite() || stop <= 0.0 {
        return Err(TransientMaximumStepError::InvalidStopTime { stop });
    }
    let start = start.unwrap_or(0.0);
    if !start.is_finite() || start < 0.0 {
        return Err(TransientMaximumStepError::InvalidStartTime { start });
    }
    if start >= stop {
        return Err(TransientMaximumStepError::InvalidWindow { start, stop });
    }
    if let Some(maximum_step) = explicit_maximum_step {
        if !maximum_step.is_finite() || maximum_step <= 0.0 {
            return Err(TransientMaximumStepError::InvalidExplicitMaximumStep { maximum_step });
        }
        return Ok(maximum_step);
    }

    let maximum_step = step.min((stop - start) / 50.0);
    if !maximum_step.is_finite() || maximum_step <= 0.0 {
        return Err(TransientMaximumStepError::UnrepresentableDefault { step, start, stop });
    }
    Ok(maximum_step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omitted_maximum_uses_smaller_print_step_or_fiftieth_window() {
        assert_eq!(
            resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, None),
            Ok(1.0e-6)
        );
        assert!(
            matches!(
                resolve_transient_maximum_step(10.0e-3, 1.0, Some(0.9), None),
                Ok(value) if (value - 2.0e-3).abs() < 1.0e-15
            ),
            "the nonzero-start analysis window determines the default ceiling"
        );
        assert_eq!(
            resolve_transient_maximum_step(10.0e-3, 1.0, None, None),
            Ok(10.0e-3)
        );
    }

    #[test]
    fn explicit_positive_finite_maximum_wins_without_being_clamped() {
        assert_eq!(
            resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, Some(7.0e-6)),
            Ok(7.0e-6)
        );
    }

    #[test]
    fn every_invalid_authored_field_has_a_typed_error() {
        for step in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                resolve_transient_maximum_step(step, 1.0, None, None),
                Err(TransientMaximumStepError::InvalidPrintStep { .. })
            ));
        }
        for stop in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                resolve_transient_maximum_step(0.1, stop, None, None),
                Err(TransientMaximumStepError::InvalidStopTime { .. })
            ));
        }
        for start in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                resolve_transient_maximum_step(0.1, 1.0, Some(start), None),
                Err(TransientMaximumStepError::InvalidStartTime { .. })
            ));
        }
        for start in [1.0, 2.0] {
            assert!(matches!(
                resolve_transient_maximum_step(0.1, 1.0, Some(start), None),
                Err(TransientMaximumStepError::InvalidWindow { .. })
            ));
        }
        for maximum_step in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                resolve_transient_maximum_step(0.1, 1.0, None, Some(maximum_step)),
                Err(TransientMaximumStepError::InvalidExplicitMaximumStep { .. })
            ));
        }
    }

    #[test]
    fn explicit_maximum_does_not_hide_an_invalid_window() {
        assert!(matches!(
            resolve_transient_maximum_step(0.1, 1.0, Some(1.0), Some(0.01)),
            Err(TransientMaximumStepError::InvalidWindow { .. })
        ));
    }

    #[test]
    fn an_unrepresentably_small_default_fails_instead_of_returning_zero() {
        let smallest_positive = f64::from_bits(1);
        assert!(matches!(
            resolve_transient_maximum_step(smallest_positive, smallest_positive, None, None),
            Err(TransientMaximumStepError::UnrepresentableDefault { .. })
        ));
    }
}
