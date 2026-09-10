//! Frequency substitution for generated, coherently routed noise tangents.

use super::arithmetic::ScaledValue;
use super::{GeneratedEvalContext, GeneratedNoiseComplex};

impl GeneratedNoiseComplex {
    /// Sum `coefficient * (j*2*pi*f)^ddt / (j*2*pi*f)^idt`, then apply
    /// multiplicity and equation orientation. The slice is emitted on the stack.
    /// Retained exponents protect intermediate products and coherent cancellation;
    /// the shared arithmetic layer bounds its exceptional exact-sum recovery.
    pub fn frequency_transfer(
        ctx: &GeneratedEvalContext<'_>,
        coefficients: &[(f64, u32, u32)],
        frequency_hz: f64,
        scale: f64,
    ) -> Self {
        Self::try_frequency_transfer(coefficients, frequency_hz, scale).unwrap_or_else(|reason| {
            ctx.report_small_signal_error(reason);
            Self {
                re: f64::NAN,
                im: f64::NAN,
            }
        })
    }

    fn try_frequency_transfer(
        coefficients: &[(f64, u32, u32)],
        frequency_hz: f64,
        scale: f64,
    ) -> Result<Self, &'static str> {
        Self::sum_frequency_transfer(coefficients.iter().copied(), frequency_hz, scale)
    }

    /// Compose only the terms present on the executed analog path. Presence
    /// is separate from gain so active zero-gain integrals remain singular at DC.
    pub fn frequency_transfer_with_activation(
        ctx: &GeneratedEvalContext<'_>,
        coefficients: &[(f64, u32, u32, bool)],
        frequency_hz: f64,
        scale: f64,
    ) -> Self {
        Self::sum_frequency_transfer(
            coefficients.iter().map(|&(value, ddt, idt, active)| {
                if active {
                    (value, ddt, idt)
                } else {
                    (0.0, 0, 0)
                }
            }),
            frequency_hz,
            scale,
        )
        .unwrap_or_else(|reason| {
            ctx.report_small_signal_error(reason);
            Self {
                re: f64::NAN,
                im: f64::NAN,
            }
        })
    }

    fn sum_frequency_transfer(
        coefficients: impl ExactSizeIterator<Item = (f64, u32, u32)> + Clone,
        frequency_hz: f64,
        scale: f64,
    ) -> Result<Self, &'static str> {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return Err("noise frequency must be finite and nonnegative");
        }
        if !scale.is_finite() || coefficients.clone().any(|(value, _, _)| !value.is_finite()) {
            return Err("non-finite noise frequency coefficient or scale");
        }
        // Keep the integral order even when D and I have equal powers.
        if frequency_hz == 0.0 && coefficients.clone().any(|(_, _, idt)| idt > 0) {
            return Err("idt small-signal transfer is singular at zero frequency");
        }
        let omega =
            ScaledValue::new(frequency_hz).multiply(ScaledValue::new(core::f64::consts::TAU));
        let one = ScaledValue::new(1.0);
        let component = |imaginary: bool| {
            let terms = coefficients.clone().map(|(coefficient, ddt, idt)| {
                let exponent = i64::from(ddt) - i64::from(idt);
                let phase = exponent.rem_euclid(4);
                let value = if (phase % 2 == 1) != imaginary {
                    ScaledValue::new(0.0)
                } else {
                    let power = omega.powu(exponent.unsigned_abs() as u32);
                    let value = ScaledValue::new(coefficient);
                    let value = if exponent < 0 {
                        value.divide(power)
                    } else {
                        value.multiply(power)
                    };
                    if phase >= 2 { value.negated() } else { value }
                };
                [value, one]
            });
            ScaledValue::sum_products_div(terms, one)
                .map(|value| value.multiply_binary64(scale))
                .map_err(|_| "noise frequency summation exceeds arithmetic precision limits")
        };
        let result = Self {
            re: component(false)?,
            im: component(true)?,
        };
        if !result.is_finite() {
            return Err("noise frequency response overflows");
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_frequency_activity_preserves_errors_only_for_executed_terms() {
        for (coefficient, active, ddt, idt, frequency, fails) in [
            (f64::NAN, false, 0, 1, 0.0, false),
            (0.0, true, 0, 1, 0.0, true),
            (0.0, true, 1, 1, 0.0, true),
            (1.0, false, 4, 0, 1e300, false),
            (f64::NAN, true, 0, 0, 1.0, true),
            (0.0, true, 0, 1, 1.0, false),
        ] {
            let ctx = GeneratedEvalContext::with_analysis(
                &[0.0],
                300.15,
                1,
                crate::GeneratedAnalysisKind::Noise,
            );
            let result = GeneratedNoiseComplex::frequency_transfer_with_activation(
                &ctx,
                &[(3.0, 0, 0, true), (coefficient, ddt, idt, active)],
                frequency,
                -2.0,
            );
            assert_eq!(ctx.evaluation_failed(), fails);
            if fails {
                assert!(!result.is_finite());
            } else {
                assert_eq!((result.re, result.im), (-6.0, 0.0));
            }
        }
    }

    #[test]
    fn noise_frequency_powers_have_the_correct_phase_and_singularity() {
        for ddt in 0..9 {
            for idt in 0..9 {
                let exponent = ddt as i32 - idt as i32;
                let actual = GeneratedNoiseComplex::try_frequency_transfer(
                    &[(3.0, ddt, idt)],
                    2.0 / core::f64::consts::TAU,
                    -2.0,
                )
                .unwrap();
                let magnitude = -6.0 * 2.0_f64.powi(exponent);
                let expected = match exponent.rem_euclid(4) {
                    0 => (magnitude, 0.0),
                    1 => (0.0, magnitude),
                    2 => (-magnitude, 0.0),
                    _ => (0.0, -magnitude),
                };
                assert_eq!((actual.re, actual.im), expected);
                assert_eq!(
                    GeneratedNoiseComplex::try_frequency_transfer(&[(3.0, ddt, idt)], 0.0, 1.0)
                        .is_err(),
                    idt > 0
                );
            }
        }
    }

    #[test]
    fn noise_frequency_sums_recover_range_and_coherent_cancellation() {
        for (coefficient, frequency, scale, expected) in [
            (1e200, 1e200, 1e-100, core::f64::consts::TAU * 1e300),
            (1e-200, 1e-200, 1e100, core::f64::consts::TAU * 1e-300),
            (0.1, 1e308, 1.0, core::f64::consts::TAU * 1e307),
        ] {
            let value = GeneratedNoiseComplex::try_frequency_transfer(
                &[(coefficient, 1, 0)],
                frequency,
                scale,
            )
            .unwrap();
            assert!((value.im / expected - 1.0).abs() < 8.0 * f64::EPSILON);
            assert_eq!(value.re, 0.0);
        }
        // Each fourth-order term is outside binary64, but their sum is finite.
        let value = GeneratedNoiseComplex::try_frequency_transfer(
            &[(1.0, 4, 0), (3.0, 0, 0), (-1.0, 8, 4)],
            1e100,
            2.0,
        )
        .unwrap();
        assert_eq!((value.re, value.im), (6.0, 0.0));
        let value = GeneratedNoiseComplex::try_frequency_transfer(
            &[(1.0, 1, 0), (1.0, 3, 0)],
            1.0 / core::f64::consts::TAU,
            1.0,
        )
        .unwrap();
        assert_eq!((value.re, value.im), (0.0, 0.0));
        assert!(
            GeneratedNoiseComplex::try_frequency_transfer(&[(1.0, u32::MAX, 0)], 2.0, 1.0).is_err()
        );
        for frequency in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(
                GeneratedNoiseComplex::try_frequency_transfer(&[(1.0, 1, 0)], frequency, 1.0)
                    .is_err()
            );
        }
    }
}
