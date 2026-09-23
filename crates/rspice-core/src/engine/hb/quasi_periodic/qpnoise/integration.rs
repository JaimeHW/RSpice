//! Bounded, cancellable integration of sampled one-sided noise densities.
use super::*;
use crate::analysis::noise::ScaledPositiveSum;

pub(super) fn integrate(
    frequencies: &[Value],
    values: &[QpnoiseValue],
    config: &QpnoiseIntegration,
    abort: &dyn AbortSignal,
) -> Result<Result<ScaledPositiveSum, QpnoiseUnavailable>, SimulationError> {
    use QpnoiseUnavailable::{
        IncompleteIntegrationBand, NegativeIntegrationFrequency, NoFrequencyInterval,
        OutsideNumericRange, UndefinedIntegrationSample,
    };
    check_abort(abort)?;
    if frequencies.len() != values.len() {
        return Err(qpnoise_error(
            "integration shape differs from frequency grid",
        ));
    }
    let Some((&first, &last)) = frequencies.first().zip(frequencies.last()) else {
        return Ok(Err(NoFrequencyInterval));
    };
    if frequencies.len() < 2 {
        return Ok(Err(NoFrequencyInterval));
    }
    let [low, high] = config.band_hz.unwrap_or([first, last]);
    if low < 0.0 {
        return Ok(Err(NegativeIntegrationFrequency));
    }
    if low < first || high > last {
        return Ok(Err(IncompleteIntegrationBand));
    }
    if high <= low {
        return Ok(Err(NoFrequencyInterval));
    }
    let mut total = ScaledPositiveSum::default();
    for i in 1..frequencies.len() {
        if i.is_multiple_of(256) {
            check_abort(abort)?;
        }
        let (f0, f1) = (frequencies[i - 1], frequencies[i]);
        let (a, b) = (f0.max(low), f1.min(high));
        if a >= b {
            continue;
        }
        let (QpnoiseValue::Finite(q0), QpnoiseValue::Finite(q1)) = (values[i - 1], values[i])
        else {
            return Ok(Err(UndefinedIntegrationSample));
        };
        if q0 < 0.0
            || q1 < 0.0
            || !q0.is_finite()
            || !q1.is_finite()
            || !f0.is_finite()
            || !f1.is_finite()
            || f0 >= f1
        {
            return Err(qpnoise_error(
                "integration requires finite nonnegative densities on an increasing grid",
            ));
        }
        // Zero PSD or DC endpoints use linear interpolation. A logarithm at
        // either zero has no defined power-law interpolation.
        let logarithmic =
            config.method == QpnoiseIntegrationMethod::LogLog && f0 > 0.0 && q0 > 0.0 && q1 > 0.0;
        let interpolate = |f: Value| -> Value {
            if f == f0 {
                return q0;
            }
            if f == f1 {
                return q1;
            }
            if logarithmic {
                let t = log_ratio(f, f0) / log_ratio(f1, f0);
                (q0.ln() + t * log_ratio(q1, q0))
                    .exp()
                    .clamp(q0.min(q1), q0.max(q1))
            } else {
                // Scale frequency coordinates if the signed interval spans
                // more than f64::MAX. Interpolate from the smaller density
                // so equal subnormal endpoints cannot both round to zero.
                let scale = f0.abs().max(f1.abs());
                let t = if (f1 - f0).is_finite() {
                    (f - f0) / (f1 - f0)
                } else {
                    (f / scale - f0 / scale) / (f1 / scale - f0 / scale)
                };
                if q1 >= q0 {
                    q0 + (q1 - q0) * t
                } else {
                    q1 + (q0 - q1) * (1.0 - t)
                }
            }
        };
        let (qa, qb) = (interpolate(a), interpolate(b));
        if logarithmic {
            if qa == 0.0 || qb == 0.0 {
                return Ok(Err(OutsideNumericRange));
            }
            let log_width = log_ratio(b, a);
            let delta = log_ratio(qb, qa) + log_width;
            let factor = if delta == 0.0 {
                log_width
            } else {
                log_width * (-(-delta.abs()).exp_m1()) / delta.abs()
            };
            if factor <= 0.0 || !factor.is_finite() {
                return Ok(Err(OutsideNumericRange));
            }
            if delta >= 0.0 {
                total.add_product(qb, b, factor);
            } else {
                total.add_product(qa, a, factor);
            }
        } else {
            total.add_product(qa, b - a, 0.5);
            total.add_product(qb, b - a, 0.5);
        }
    }
    Ok(Ok(total))
}

fn log_ratio(a: Value, b: Value) -> Value {
    if a == b {
        return 0.0;
    }
    // Reversing descending ratios avoids subtracting almost equal numbers
    // from one when a/b is tiny (which corrupts power-law exponents).
    if a < b {
        return -log_ratio(b, a);
    }
    let relative = (a - b) / b;
    if relative.is_finite() && relative > -1.0 {
        relative.ln_1p()
    } else {
        a.ln() - b.ln()
    }
}

pub(super) fn rms(power: Result<ScaledPositiveSum, QpnoiseUnavailable>) -> QpnoiseValue {
    match power {
        Ok(power) => QpnoiseValue::from_nonnegative(power.square_root()),
        Err(reason) => QpnoiseValue::Unavailable(reason),
    }
}
