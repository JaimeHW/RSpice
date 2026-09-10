//! Rectangular complex arithmetic and alignment on the shared sample domain.

use num_complex::Complex64;

use super::ast::BinaryOp;
use super::evaluator::EvaluationError;
use super::interpolation::{
    InterpolationMethod, WaveformInterpolator, align_waveforms, validate_samples,
};
use super::value::{ComplexValue, finite, hole};

pub(super) fn binary(
    op: BinaryOp,
    left: ComplexValue,
    right: ComplexValue,
) -> Result<ComplexValue, EvaluationError> {
    for value in [&left, &right] {
        if let ComplexValue::Waveform(x, y) = value {
            validate_samples(x, y)
                .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
        }
    }
    let apply = |left, right| {
        if !finite(left) || !finite(right) {
            return Complex64::new(f64::NAN, f64::NAN);
        }
        match op {
            BinaryOp::Add => left + right,
            BinaryOp::Sub => left - right,
            BinaryOp::Mul => left * right,
            BinaryOp::Div => divide(left, right),
            BinaryOp::Pow => power(left, right),
        }
    };
    Ok(match (left, right) {
        (ComplexValue::Scalar(left), ComplexValue::Scalar(right)) => {
            ComplexValue::Scalar(apply(left, right))
        }
        (ComplexValue::Scalar(left), ComplexValue::Waveform(x, y)) => ComplexValue::Waveform(
            x,
            y.into_iter()
                .map(|right| hole(apply(left, right)))
                .collect(),
        ),
        (ComplexValue::Waveform(x, y), ComplexValue::Scalar(right)) => ComplexValue::Waveform(
            x,
            y.into_iter().map(|left| hole(apply(left, right))).collect(),
        ),
        (ComplexValue::Waveform(lx, ly), ComplexValue::Waveform(rx, ry)) => {
            let (x, left, right) = if lx == rx {
                (lx, ly, ry)
            } else {
                align(&lx, &ly, &rx, &ry)?
            };
            ComplexValue::Waveform(
                x,
                left.into_iter()
                    .zip(right)
                    .map(|(left, right)| hole(apply(left, right)))
                    .collect(),
            )
        }
    })
}

type AlignedComplex = (Vec<f64>, Vec<Complex64>, Vec<Complex64>);

fn align(
    lx: &[f64],
    ly: &[Complex64],
    rx: &[f64],
    ry: &[Complex64],
) -> Result<AlignedComplex, EvaluationError> {
    let real = |values: &[Complex64]| values.iter().map(|v| v.re).collect::<Vec<_>>();
    let imag = |values: &[Complex64]| values.iter().map(|v| v.im).collect::<Vec<_>>();
    let failure = |error: super::interpolation::InterpolationError| {
        EvaluationError::WaveformMismatch(error.to_string())
    };
    let (x, left_real, right_real) =
        align_waveforms(lx, &real(ly), rx, &real(ry), InterpolationMethod::Linear)
            .map_err(failure)?;
    let (left_imag, right_imag) = (imag(ly), imag(ry));
    let left_imag = WaveformInterpolator::new(lx, &left_imag)
        .map_err(failure)?
        .resample(&x)
        .map_err(failure)?;
    let right_imag = WaveformInterpolator::new(rx, &right_imag)
        .map_err(failure)?
        .resample(&x)
        .map_err(failure)?;
    let combine = |re: Vec<f64>, im: Vec<f64>| {
        re.into_iter()
            .zip(im)
            .map(|(re, im)| hole(Complex64::new(re, im)))
            .collect()
    };
    Ok((
        x,
        combine(left_real, left_imag),
        combine(right_real, right_imag),
    ))
}

/// Scale the divisor before squaring. Choose the division order so both tiny
/// divisors and large cancelling numerators can still produce finite results.
pub(super) fn divide(a: Complex64, b: Complex64) -> Complex64 {
    let scale = b.re.abs().max(b.im.abs());
    if scale == 0.0 {
        return Complex64::new(f64::NAN, f64::NAN);
    }
    let (c, d) = (b.re / scale, b.im / scale);
    let denominator = c * c + d * d;
    let (c, d) = (c / denominator, d / denominator);
    let scaled = a / scale;
    if scaled.re.is_finite() && scaled.im.is_finite() {
        Complex64::new(scaled.re * c + scaled.im * d, scaled.im * c - scaled.re * d)
    } else {
        Complex64::new((a.re * c + a.im * d) / scale, (a.im * c - a.re * d) / scale)
    }
}

/// Principal complex logarithm, with phase in radians in [-pi, pi].
pub(super) fn logarithm(value: Complex64) -> Complex64 {
    let scale = value.re.abs().max(value.im.abs());
    if scale == 0.0 {
        return Complex64::new(f64::NEG_INFINITY, value.arg());
    }
    Complex64::new(
        scale.ln() + (value.re / scale).hypot(value.im / scale).ln(),
        value.arg(),
    )
}

pub(super) fn exponential(value: Complex64) -> Complex64 {
    let amplitude = value.re.exp();
    let (sin, cos) = value.im.sin_cos();
    if amplitude.is_finite() {
        return Complex64::new(amplitude * cos, amplitude * sin);
    }
    let component = |factor: f64| {
        if factor == 0.0 {
            factor
        } else {
            (value.re + factor.abs().ln()).exp().copysign(factor)
        }
    };
    Complex64::new(component(cos), component(sin))
}

/// Principal square root; normalize before hypot and addition to avoid
/// overflowing when the square root itself is representable.
pub(super) fn square_root(value: Complex64) -> Complex64 {
    let scale = value.re.abs().max(value.im.abs());
    if scale == 0.0 {
        return Complex64::new(0.0, value.im);
    }
    let (a, b) = (value.re / scale, value.im / scale);
    let radius = a.hypot(b);
    let large = scale.sqrt() * ((radius + a.abs()) * 0.5).sqrt();
    if a >= 0.0 {
        Complex64::new(large, value.im / large * 0.5)
    } else {
        Complex64::new(value.im.abs() / large * 0.5, large.copysign(value.im))
    }
}

fn power(base: Complex64, exponent: Complex64) -> Complex64 {
    if exponent == Complex64::new(0.0, 0.0) {
        return Complex64::new(1.0, 0.0);
    }
    if base == Complex64::new(0.0, 0.0) && exponent.im == 0.0 && exponent.re > 0.0 {
        return base;
    }
    exponential(exponent * logarithm(base))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: Complex64, expected: Complex64) {
        assert!(
            (actual.re - expected.re).abs() < 1e-14 && (actual.im - expected.im).abs() < 1e-14,
            "{actual:?} != {expected:?}"
        );
    }

    #[test]
    fn complex_arithmetic_matches_rectangular_identities_at_extreme_scales() {
        for scale in [f64::from_bits(1), 1e-300, 1.0, 1e300, f64::MAX] {
            close(
                divide(Complex64::new(scale, scale), Complex64::new(scale, scale)),
                Complex64::new(1.0, 0.0),
            );
        }
        close(
            divide(Complex64::new(f64::MAX, 0.0), Complex64::new(0.5, 0.5)) / f64::MAX,
            Complex64::new(1.0, -1.0),
        );
        close(
            divide(Complex64::new(3.0, 4.0), Complex64::new(1.0, -2.0)),
            Complex64::new(-1.0, 2.0),
        );
    }

    #[test]
    fn principal_functions_keep_finite_results_at_the_range_limits() {
        close(
            square_root(Complex64::new(-4.0, 0.0)),
            Complex64::new(0.0, 2.0),
        );
        let root = square_root(Complex64::new(f64::MAX, f64::MAX));
        assert!(root.re.is_finite() && root.im.is_finite());
        close(
            logarithm(Complex64::new(0.0, 1.0)),
            Complex64::new(0.0, std::f64::consts::FRAC_PI_2),
        );
        let log = logarithm(Complex64::new(f64::MAX, f64::MAX));
        assert!(log.re.is_finite());
        let value = exponential(Complex64::new(
            f64::MAX.ln() + 0.1,
            std::f64::consts::FRAC_PI_4,
        ));
        assert!(value.re.is_finite() && value.im.is_finite());
    }

    #[test]
    fn rectangular_alignment_preserves_opposing_and_rotated_phasors() {
        let left = ComplexValue::Waveform(
            vec![0.0, 1.0, 3.0],
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 2.0),
                Complex64::new(3.0, 6.0),
            ],
        );
        let right = ComplexValue::Waveform(
            vec![0.0, 2.0, 3.0],
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(2.0, 4.0),
                Complex64::new(3.0, 6.0),
            ],
        );
        let actual = binary(BinaryOp::Sub, left, right).unwrap();
        assert_eq!(
            actual,
            ComplexValue::Waveform(vec![0.0, 1.0, 2.0, 3.0], vec![Complex64::new(0.0, 0.0); 4])
        );
    }
}
