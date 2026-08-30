//! Shared source-to-runtime contracts for Verilog-AMS timing operators.
//!
//! Timing arguments are normalized once so every executable backend receives
//! the same finite, sign-correct representation. In particular, the authored
//! negative slew rate is a signed falling slope, while runtimes consume its
//! positive magnitude.

/// Positive rate magnitudes consumed by the runtime slew implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SlewRateMagnitudes {
    pub(crate) rise: f64,
    pub(crate) fall: f64,
}

/// Normalized behavior for an authored `slew` call.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NormalizedSlewRates {
    /// Neither optional rate was authored, so `slew` is an explicit
    /// passthrough rather than an arbitrarily limited signal.
    Passthrough,
    /// Slew limiting with finite, strictly positive rise and fall magnitudes.
    Limited(SlewRateMagnitudes),
}

/// Invalid authored optional-rate combinations for `slew`.
#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub(crate) enum SlewRateError {
    #[error("slew negative rate requires an authored positive rate")]
    MissingPositive,
    #[error("slew positive rate must be finite, got {rate}")]
    PositiveNonFinite { rate: f64 },
    #[error("slew positive rate must be greater than zero, got {rate}")]
    PositiveNotStrictlyPositive { rate: f64 },
    #[error("slew negative rate must be finite, got {rate}")]
    NegativeNonFinite { rate: f64 },
    #[error("slew negative rate must be less than zero, got {rate}")]
    NegativeNotStrictlyNegative { rate: f64 },
}

/// Normalize the authored optional positive and negative rates for `slew`.
///
/// Omitting both rates is an explicit passthrough. When only the positive
/// rate is present, the negative rate inherits its opposite, yielding equal
/// positive rise and fall magnitudes internally.
pub(crate) fn normalize_slew_rates(
    positive_rate: Option<f64>,
    negative_rate: Option<f64>,
) -> Result<NormalizedSlewRates, SlewRateError> {
    let Some(positive_rate) = positive_rate else {
        return if negative_rate.is_some() {
            Err(SlewRateError::MissingPositive)
        } else {
            Ok(NormalizedSlewRates::Passthrough)
        };
    };

    if !positive_rate.is_finite() {
        return Err(SlewRateError::PositiveNonFinite {
            rate: positive_rate,
        });
    }
    if positive_rate <= 0.0 {
        return Err(SlewRateError::PositiveNotStrictlyPositive {
            rate: positive_rate,
        });
    }

    let fall = match negative_rate {
        None => positive_rate,
        Some(negative_rate) => {
            if !negative_rate.is_finite() {
                return Err(SlewRateError::NegativeNonFinite {
                    rate: negative_rate,
                });
            }
            if negative_rate >= 0.0 {
                return Err(SlewRateError::NegativeNotStrictlyNegative {
                    rate: negative_rate,
                });
            }
            -negative_rate
        }
    };

    Ok(NormalizedSlewRates::Limited(SlewRateMagnitudes {
        rise: positive_rate,
        fall,
    }))
}

#[cfg(test)]
mod tests {
    use super::{NormalizedSlewRates, SlewRateError, SlewRateMagnitudes, normalize_slew_rates};

    #[test]
    fn omitted_rates_are_explicit_passthrough() {
        assert_eq!(
            normalize_slew_rates(None, None),
            Ok(NormalizedSlewRates::Passthrough)
        );
    }

    #[test]
    fn omitted_negative_rate_inherits_positive_magnitude() {
        assert_eq!(
            normalize_slew_rates(Some(4.5), None),
            Ok(NormalizedSlewRates::Limited(SlewRateMagnitudes {
                rise: 4.5,
                fall: 4.5,
            }))
        );
    }

    #[test]
    fn authored_negative_rate_becomes_positive_fall_magnitude() {
        assert_eq!(
            normalize_slew_rates(Some(4.5), Some(-2.25)),
            Ok(NormalizedSlewRates::Limited(SlewRateMagnitudes {
                rise: 4.5,
                fall: 2.25,
            }))
        );
    }

    #[test]
    fn negative_rate_without_positive_rate_is_rejected() {
        assert_eq!(
            normalize_slew_rates(None, Some(-1.0)),
            Err(SlewRateError::MissingPositive)
        );
    }

    #[test]
    fn non_positive_positive_rates_are_rejected() {
        for rate in [0.0, -0.0, -1.0] {
            assert_eq!(
                normalize_slew_rates(Some(rate), None),
                Err(SlewRateError::PositiveNotStrictlyPositive { rate })
            );
        }
    }

    #[test]
    fn non_finite_positive_rates_are_rejected() {
        for rate in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                normalize_slew_rates(Some(rate), None),
                Err(SlewRateError::PositiveNonFinite { rate: actual })
                    if actual.to_bits() == rate.to_bits()
            ));
        }
    }

    #[test]
    fn non_negative_negative_rates_are_rejected() {
        for rate in [0.0, -0.0, 1.0] {
            assert_eq!(
                normalize_slew_rates(Some(1.0), Some(rate)),
                Err(SlewRateError::NegativeNotStrictlyNegative { rate })
            );
        }
    }

    #[test]
    fn non_finite_negative_rates_are_rejected() {
        for rate in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                normalize_slew_rates(Some(1.0), Some(rate)),
                Err(SlewRateError::NegativeNonFinite { rate: actual })
                    if actual.to_bits() == rate.to_bits()
            ));
        }
    }
}
