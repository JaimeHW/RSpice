//! Sampled power-wave passivity and reciprocity diagnostics.
//!
//! Passivity is an operator-norm condition, not an elementwise magnitude
//! check. These diagnostics apply to the complete matrix at one frequency;
//! they do not certify interpolation between samples or a truncated periodic
//! sideband block.

use super::{Complex64, NetworkError, Value};
use crate::abort_signal::AbortSignal;

/// Bound the dense diagnostic so an interactive request has bounded work.
/// Circuit solving and retained matrix inspection have independent limits.
pub const MAX_NETWORK_DIAGNOSTIC_PORTS: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampledPassivity {
    Passive,
    Active,
    /// The largest singular value lies within the declared tolerance of one.
    Boundary,
}

#[derive(Debug, Clone, Copy)]
pub struct NetworkQuality {
    pub largest_singular_value: Value,
    /// max |Sij - Sji| / max(1, max |Re Sij|, max |Im Sij|).
    pub reciprocity_residual: Value,
    pub tolerance: Value,
    pub passivity: SampledPassivity,
}

pub fn network_quality_with_abort(
    matrix: &[Vec<Complex64>],
    tolerance: Value,
    abort: &dyn AbortSignal,
) -> Result<NetworkQuality, NetworkError> {
    if abort.is_aborted() {
        return Err(NetworkError::Aborted);
    }
    let n = matrix.len();
    if n > MAX_NETWORK_DIAGNOSTIC_PORTS {
        return Err(NetworkError::NumericalFailure(format!(
            "dense network diagnostics support at most {MAX_NETWORK_DIAGNOSTIC_PORTS} ports; requested {n}"
        )));
    }
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return Err(NetworkError::MalformedAdmittance {
            rows: n,
            impedances: n,
        });
    }
    if !tolerance.is_finite() || tolerance <= 0.0 || tolerance >= 1.0 {
        return Err(NetworkError::NumericalFailure(
            "network quality tolerance must be finite and between zero and one".to_owned(),
        ));
    }
    let mut scale = 0.0_f64;
    for (row, values) in matrix.iter().enumerate() {
        if abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        for (column, value) in values.iter().enumerate() {
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(NetworkError::NonFiniteMatrixEntry { row, column });
            }
            scale = scale.max(value.re.abs()).max(value.im.abs());
        }
    }
    if scale == 0.0 {
        return Ok(NetworkQuality {
            largest_singular_value: 0.0,
            reciprocity_residual: 0.0,
            tolerance,
            passivity: SampledPassivity::Passive,
        });
    }
    // Normalize before decomposition to protect large and subnormal matrices.
    let normalized = faer::Mat::from_fn(n, n, |row, column| {
        let value = matrix[row][column];
        faer::c64::new(value.re / scale, value.im / scale)
    });
    let singular_values = normalized.singular_values().map_err(|error| {
        NetworkError::NumericalFailure(format!(
            "network singular-value decomposition failed: {error:?}"
        ))
    })?;
    if abort.is_aborted() {
        return Err(NetworkError::Aborted);
    }
    if singular_values.len() != n
        || singular_values
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(NetworkError::NumericalFailure(
            "network singular-value decomposition returned invalid values".to_owned(),
        ));
    }
    let largest = singular_values.into_iter().fold(0.0_f64, f64::max) * scale;
    if !largest.is_finite() {
        return Err(NetworkError::NumericalFailure(
            "network singular value exceeds the finite result range".to_owned(),
        ));
    }
    let reference = scale.max(1.0);
    let mut residual = 0.0_f64;
    for (row, values) in matrix.iter().enumerate() {
        if abort.is_aborted() {
            return Err(NetworkError::Aborted);
        }
        for (column, left) in values.iter().enumerate().take(row) {
            let right = matrix[column][row];
            residual = residual.max(
                (left.re / reference - right.re / reference)
                    .hypot(left.im / reference - right.im / reference),
            );
        }
    }
    // Do not classify numerical roundoff around a lossless boundary as gain.
    let tolerance = tolerance.max(64.0 * n as Value * Value::EPSILON);
    Ok(NetworkQuality {
        largest_singular_value: largest,
        reciprocity_residual: residual,
        tolerance,
        passivity: if largest < 1.0 - tolerance {
            SampledPassivity::Passive
        } else if largest > 1.0 + tolerance {
            SampledPassivity::Active
        } else {
            SampledPassivity::Boundary
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    #[test]
    fn collective_gain_is_detected_even_when_each_coefficient_is_below_one() {
        let matrix = vec![vec![Complex64::new(0.6, 0.0); 2]; 2];
        let quality = network_quality_with_abort(&matrix, 1e-10, &NoAbort).unwrap();
        assert!((quality.largest_singular_value - 1.2).abs() < 1e-14);
        assert_eq!(quality.passivity, SampledPassivity::Active);
        assert_eq!(quality.reciprocity_residual, 0.0);
    }

    #[test]
    fn complex_lossless_through_and_attenuation_have_distinct_verdicts() {
        for (amplitude, expected) in [
            (1.0, SampledPassivity::Boundary),
            (0.5, SampledPassivity::Passive),
        ] {
            let phase = Complex64::new(0.0, amplitude);
            let matrix = vec![vec![Complex64::ZERO, phase], vec![phase, Complex64::ZERO]];
            let quality = network_quality_with_abort(&matrix, 1e-10, &NoAbort).unwrap();
            assert!((quality.largest_singular_value - amplitude).abs() < 1e-14);
            assert_eq!(quality.passivity, expected);
        }
    }

    #[test]
    fn scaling_preserves_subnormal_and_large_finite_networks() {
        for scale in [1e-310, 1e250] {
            let matrix = vec![vec![Complex64::new(scale, 0.0)]];
            let quality = network_quality_with_abort(&matrix, 1e-10, &NoAbort).unwrap();
            assert_eq!(quality.largest_singular_value, scale);
        }
    }

    #[test]
    fn invalid_input_never_becomes_a_passivity_verdict() {
        assert!(network_quality_with_abort(&[], 1e-10, &NoAbort).is_err());
        assert!(
            network_quality_with_abort(&[vec![Complex64::new(f64::NAN, 0.0)]], 1e-10, &NoAbort)
                .is_err()
        );
        assert!(network_quality_with_abort(&[vec![Complex64::ZERO]], 0.0, &NoAbort).is_err());
    }

    #[test]
    fn a_nonreciprocal_network_can_still_be_passive() {
        let matrix = vec![
            vec![Complex64::ZERO, Complex64::ZERO],
            vec![Complex64::new(0.0, 0.5), Complex64::ZERO],
        ];
        let quality = network_quality_with_abort(&matrix, 1e-10, &NoAbort).unwrap();
        assert_eq!(quality.passivity, SampledPassivity::Passive);
        assert!((quality.reciprocity_residual - 0.5).abs() < 1e-14);
    }

    #[test]
    fn overflow_and_cancellation_cannot_publish_a_diagnostic() {
        let huge = vec![vec![Complex64::new(1e308, 0.0); 2]; 2];
        assert!(network_quality_with_abort(&huge, 1e-10, &NoAbort).is_err());
        struct Stop;
        impl AbortSignal for Stop {
            fn is_aborted(&self) -> bool {
                true
            }
        }
        assert!(matches!(
            network_quality_with_abort(&huge, 1e-10, &Stop),
            Err(NetworkError::Aborted)
        ));
    }
}
