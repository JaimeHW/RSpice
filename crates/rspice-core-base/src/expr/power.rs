//! The real power operator, the named power functions and their Newton
//! derivatives.
//!
//! The power operator `^` (`**`) and the named `POW` follow QSPICE in every
//! dialect: a negative base takes its real odd root when the exponent is the
//! reciprocal of an odd integer, and the power is zero at every other
//! fractional exponent. Keeping the value and its Newton derivative here
//! prevents the bytecode and analytic evaluators from drifting apart.

use super::{Derivative, derivative_pair};
use crate::Value;
use crate::config::ExpressionDialect;

const XYCE_NONFINITE_REPLACEMENT: Value = 1.0e50;

/// How far an exponent's reciprocal may lie from an odd integer and still
/// select the real odd root of a negative base. QSPICE takes the root within
/// 9.8e-15 of the reciprocals 3, 7 and 49 and refuses it from 1.07e-14.
const ODD_ROOT_RECIPROCAL_TOLERANCE: Value = 1.0e-14;

/// Apply Xyce's public expression-boundary `fixNan`/`fixInf` policy.
///
/// This must be called only after the complete expression (or derivative) has
/// been evaluated.  Normalizing bytecode intermediates would change IEEE
/// expressions such as `0*exp(1000)` from NaN to zero.
#[inline]
#[doc(hidden)]
pub fn normalize_expression_boundary(value: Value, dialect: ExpressionDialect) -> Value {
    if dialect == ExpressionDialect::Xyce && !value.is_finite() {
        XYCE_NONFINITE_REPLACEMENT.copysign(value)
    } else {
        value
    }
}

/// Evaluate the power operator `^` (`**`) and the named `POW`, as QSPICE
/// does. A negative base keeps the ordinary signed power at an integer
/// exponent, takes its real odd root where the exponent is the reciprocal of
/// an odd integer, so `(-8)^(1/3)` is -2, and is zero at every other
/// fractional exponent: `(-8)^(2/3)` and `(-2)^2.5` are 0.
#[inline]
pub(crate) fn real_pow(base: Value, exponent: Value) -> Value {
    if base.is_nan() || base >= 0.0 || !exponent.is_finite() || exponent.fract() == 0.0 {
        return base.powf(exponent);
    }
    let sign = negative_base_sign(exponent);
    if sign == 0.0 {
        0.0
    } else {
        sign * base.abs().powf(exponent)
    }
}

/// The signed unit a negative base's power multiplies `|x|^y` by: the parity
/// of an integer exponent, -1 at the reciprocal of an odd integer (the real
/// odd root), and 0 at every other exponent.
#[inline]
pub(super) fn negative_base_sign(exponent: Value) -> Value {
    if exponent.fract() == 0.0 {
        return if exponent.rem_euclid(2.0) == 0.0 {
            1.0
        } else {
            -1.0
        };
    }
    let reciprocal = exponent.recip();
    let index = reciprocal.round();
    if (reciprocal - index).abs() < ODD_ROOT_RECIPROCAL_TOLERANCE && index.rem_euclid(2.0) == 1.0 {
        -1.0
    } else {
        0.0
    }
}

/// Evaluate the named `PWR` function. It is an alias of `POW` in Xyce and a
/// sign-preserving magnitude power in ngspice behavioral expressions.
#[inline]
pub(crate) fn real_function_pwr(base: Value, exponent: Value, dialect: ExpressionDialect) -> Value {
    if dialect == ExpressionDialect::Xyce {
        real_pow(base, exponent)
    } else {
        real_signed_magnitude_power(base, exponent)
    }
}

/// Evaluate the signed-magnitude `PWRS` operation. Both supported SPICE
/// dialects select the ordinary power branch for zero and positive bases.
#[inline]
pub(crate) fn real_function_pwrs(base: Value, exponent: Value) -> Value {
    real_signed_magnitude_power(base, exponent)
}

#[inline]
fn real_signed_magnitude_power(base: Value, exponent: Value) -> Value {
    if base < 0.0 {
        -(-base).powf(exponent)
    } else {
        base.powf(exponent)
    }
}

/// Exact SPICE sign function. Ordered comparisons deliberately map both
/// signed zero and NaN to zero, matching Xyce and ngspice.
#[inline]
#[doc(hidden)]
pub fn ordered_sign(value: Value) -> Value {
    if value > 0.0 {
        1.0
    } else if value < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Non-panicking three-argument LIMIT with Xyce's specified branch order.
/// The boolean is true exactly when the input branch is selected and its
/// directional derivative must be retained.
#[inline]
#[doc(hidden)]
pub fn ordered_limit(
    value: Value,
    lower: Value,
    upper: Value,
    dialect: ExpressionDialect,
) -> (Value, bool) {
    let normalize_operand = |operand: Value| {
        if dialect != ExpressionDialect::Xyce || operand.is_finite() {
            operand
        } else {
            XYCE_NONFINITE_REPLACEMENT.copysign(operand)
        }
    };
    let value = normalize_operand(value);
    let lower = normalize_operand(lower);
    let upper = normalize_operand(upper);
    if value < lower {
        (lower, false)
    } else if value > upper {
        (upper, false)
    } else {
        (value, true)
    }
}

/// Evaluate the power operator and its directional derivative, the exact
/// derivative along both arguments of the branch [`real_pow`] is on.
///
/// `d_base` and `d_exponent` are derivatives with respect to the same Newton
/// variable.
pub(crate) fn real_pow_with_derivative(
    base: Value,
    d_base: Derivative,
    exponent: Value,
    d_exponent: Derivative,
    dialect: ExpressionDialect,
) -> Option<(Value, Derivative)> {
    let value = real_pow(base, exponent);
    if !base.is_finite() || !exponent.is_finite() {
        return derivative_pair(value, Value::NAN);
    }

    if base > 0.0 {
        let mut derivative = Derivative::from(0.0);
        if d_base != 0.0 {
            derivative += power_base_direction(base, exponent, value, d_base);
        }
        if d_exponent != 0.0 {
            derivative += d_exponent * value * base.ln();
        }
        return derivative_pair(value, derivative);
    }

    if base < 0.0 {
        // The branch in force is `sign * |x|^y`, or zero off the roots.
        let sign = negative_base_sign(exponent);
        if sign == 0.0 {
            return derivative_pair(value, 0.0);
        }
        let magnitude = base.abs().powf(exponent);
        // Do not manufacture NaN from an absent direction (`inf * 0`).  This
        // matters for high-magnitude constant bases/exponents where one
        // partial is singular but the requested Newton direction is exactly
        // independent of it.
        let mut derivative = Derivative::from(0.0);
        if d_base != 0.0 {
            derivative += power_base_direction(base.abs(), exponent, magnitude, d_base) * -sign;
        }
        if d_exponent != 0.0 {
            derivative += d_exponent * value * base.abs().ln();
        }
        return derivative_pair(value, derivative);
    }

    // Xyce 7.10's powOp::dx2 leaves every derivative slot at exactly zero
    // whenever the base is zero, independent of exponent and direction.  The
    // value retains ordinary pow semantics and is normalized only at the
    // completed expression boundary.
    if dialect == ExpressionDialect::Xyce {
        return derivative_pair(value, 0.0);
    }
    // Elsewhere a zero base keeps the ordinary slope along the base. Along the
    // exponent `0^y` is 0 for every positive `y`, so that direction adds
    // nothing.
    derivative_pair(value, power_base_direction(base, exponent, value, d_base))
}

pub(crate) fn real_function_pwr_with_derivative(
    base: Value,
    d_base: Derivative,
    exponent: Value,
    d_exponent: Derivative,
    dialect: ExpressionDialect,
) -> Option<(Value, Derivative)> {
    if dialect == ExpressionDialect::Xyce {
        return real_pow_with_derivative(base, d_base, exponent, d_exponent, dialect);
    }
    signed_magnitude_power_with_derivative(base, d_base, exponent, d_exponent)
}

pub(crate) fn real_function_pwrs_with_derivative(
    base: Value,
    d_base: Derivative,
    exponent: Value,
    d_exponent: Derivative,
    dialect: ExpressionDialect,
) -> Option<(Value, Derivative)> {
    if dialect == ExpressionDialect::Xyce && base == 0.0 {
        return derivative_pair(real_function_pwrs(base, exponent), 0.0);
    }
    signed_magnitude_power_with_derivative(base, d_base, exponent, d_exponent)
}

/// The sign-preserving magnitude power `sign(x) |x|^y` and its derivative.
fn signed_magnitude_power_with_derivative(
    base: Value,
    d_base: Derivative,
    exponent: Value,
    d_exponent: Derivative,
) -> Option<(Value, Derivative)> {
    let magnitude = base.abs();
    let polarity = if base < 0.0 { -1.0 } else { 1.0 };
    let value = polarity * magnitude.powf(exponent);
    let mut derivative = Derivative::from(0.0);
    if d_base != 0.0 {
        derivative += power_base_direction(magnitude, exponent, value.abs(), d_base);
    }
    if d_exponent != 0.0 {
        derivative += d_exponent * value * magnitude.ln();
    }
    derivative_pair(value, derivative)
}

/// Reuse a normal power value before dividing by the base. Besides avoiding
/// an overflowing shifted power, this avoids rounding `exponent - 1` in the
/// common case. If the primal underflows or overflows, the shifted power may
/// still give a usable slope (for example the derivative of a tiny square).
fn power_base_direction(
    base: Value,
    exponent: Value,
    power: Value,
    direction: Derivative,
) -> Derivative {
    if direction == 0.0 || exponent == 0.0 {
        return 0.0.into();
    }
    if base != 0.0 && power.is_normal() {
        direction * exponent * power / base
    } else {
        direction * exponent * base.powf(exponent - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// QSPICE's power at a negative base: `.op` B-sources, the same through
    /// `**`, `^` and `pow()`.
    #[test]
    fn negative_base_power_is_the_real_odd_root_or_zero() {
        for (base, exponent, qspice) in [
            (-8.0, 1.0 / 3.0, -2.0),
            (-2.0, 1.0 / 5.0, -1.148698354997035),
            (-0.5, -1.0 / 3.0, -1.259921049894873),
            (-8.0, 1.0 / 49.0, -1.043350930967019),
            (-8.0, 3.0, -512.0),
            (-8.0, -3.0, -0.001953125),
            (-8.0, 2.0 / 3.0, 0.0),
            (-8.0, 3.0 / 5.0, 0.0),
            (-8.0, 0.333333, 0.0),
            (-8.0, 1.0 / 99.0, 0.0),
            (-2.0, 0.5, 0.0),
            (-2.0, 2.5, 0.0),
        ] {
            let value = real_pow(base, exponent);
            assert!(
                (value - qspice).abs() <= 1.0e-15 * qspice.abs(),
                "({base})^{exponent} = {value}, QSPICE {qspice}"
            );
        }
    }

    /// The derivative is the slope of the branch the power is on: the
    /// ordinary slope along the base, `value ln|x|` along the exponent, and
    /// nothing off the real roots, in every dialect.
    #[test]
    fn power_derivative_is_the_slope_of_its_branch() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for (base, exponent) in [
                (-8.0, 1.0 / 3.0),
                (-0.5, -1.0 / 3.0),
                (-2.0, 3.0),
                (-2.0, 2.5),
                (3.0, 0.7),
            ] {
                let value = real_pow(base, exponent);
                let (_, along_base) =
                    real_pow_with_derivative(base, 1.0.into(), exponent, 0.0.into(), dialect)
                        .expect("a nonzero finite base has a derivative");
                let h = 1.0e-6 * base.abs();
                let difference =
                    (real_pow(base + h, exponent) - real_pow(base - h, exponent)) / (2.0 * h);
                let along_base = along_base.binary64();
                assert!(
                    (along_base - difference).abs() <= 1.0e-6 * difference.abs().max(1.0),
                    "{dialect:?} ({base})^{exponent} along the base: {along_base} against {difference}"
                );
                let (_, along_exponent) =
                    real_pow_with_derivative(base, 0.0.into(), exponent, 1.0.into(), dialect)
                        .expect("a nonzero finite base has a derivative");
                let expected = value * base.abs().ln();
                let along_exponent = along_exponent.binary64();
                assert!(
                    (along_exponent - expected).abs() <= 1.0e-15 * expected.abs(),
                    "{dialect:?} ({base})^{exponent} along the exponent: {along_exponent} against {expected}"
                );
            }
            let (_, cube) =
                real_pow_with_derivative(-2.0, 1.0.into(), 3.0, 0.0.into(), dialect).unwrap();
            assert_eq!(cube.binary64(), 12.0);
        }
        // A zero base keeps the ordinary slope along the base outside Xyce,
        // and none along the exponent.
        let (value, derivative) =
            real_pow_with_derivative(0.0, 1.0.into(), 1.0, 1.0.into(), ExpressionDialect::Ngspice)
                .unwrap();
        assert_eq!((value, derivative.binary64()), (0.0, 1.0));
        let (value, derivative) =
            real_pow_with_derivative(0.0, 1.0.into(), 0.0, 1.0.into(), ExpressionDialect::Ngspice)
                .unwrap();
        assert_eq!((value, derivative.binary64()), (1.0, 0.0));
    }

    #[test]
    fn xyce_integer_and_zero_power_edges_are_deliberate() {
        assert_eq!(real_pow(-2.0, 3.0), -8.0);
        assert_eq!(real_pow(-2.0, -2.0), 0.25);
        assert_eq!(real_pow(0.0, 0.0), 1.0);
        assert_eq!(real_pow(0.0, 2.0), 0.0);
        assert!(real_pow(0.0, -1.0).is_infinite());

        assert_eq!(
            real_pow_with_derivative(0.0, 1.0.into(), 2.0, 0.0.into(), ExpressionDialect::Xyce),
            derivative_pair(0.0, 0.0)
        );
        assert_eq!(
            real_pow_with_derivative(0.0, 1.0.into(), 1.0, 0.0.into(), ExpressionDialect::Xyce),
            derivative_pair(0.0, 0.0)
        );
        assert_eq!(
            real_pow_with_derivative(0.0, 1.0.into(), 0.5, 0.0.into(), ExpressionDialect::Xyce),
            derivative_pair(0.0, 0.0)
        );
        let (negative_power, derivative) =
            real_pow_with_derivative(0.0, 1.0.into(), -1.0, 0.0.into(), ExpressionDialect::Xyce)
                .expect("Xyce zero-base pow derivative is defined as zero");
        assert!(negative_power.is_infinite());
        assert_eq!(derivative, 0.0);
    }

    #[test]
    fn inactive_derivative_directions_do_not_create_nan() {
        let (value, derivative) = real_pow_with_derivative(
            -1.0e308,
            0.0.into(),
            2.0,
            0.0.into(),
            ExpressionDialect::Xyce,
        )
        .expect("a constant overflowing power still has zero directional derivative");
        assert!(value.is_infinite());
        assert_eq!(derivative, 0.0);
    }

    #[test]
    fn xyce_boundary_preserves_nonfinite_sign_bits() {
        assert_eq!(
            normalize_expression_boundary(Value::INFINITY, ExpressionDialect::Xyce),
            1.0e50
        );
        assert_eq!(
            normalize_expression_boundary(Value::NEG_INFINITY, ExpressionDialect::Xyce),
            -1.0e50
        );
        assert_eq!(
            normalize_expression_boundary(Value::NAN, ExpressionDialect::Xyce),
            1.0e50
        );
        assert_eq!(
            normalize_expression_boundary(-Value::NAN, ExpressionDialect::Xyce),
            -1.0e50
        );
        assert!(
            normalize_expression_boundary(Value::NEG_INFINITY, ExpressionDialect::Ngspice)
                .is_infinite()
        );
    }

    #[test]
    fn sign_and_limit_helpers_follow_ordered_spice_branches() {
        assert_eq!(ordered_sign(-1.0e-300), -1.0);
        assert_eq!(ordered_sign(-0.0), 0.0);
        assert_eq!(ordered_sign(0.0), 0.0);
        assert_eq!(ordered_sign(1.0e-300), 1.0);
        assert_eq!(ordered_sign(Value::NAN), 0.0);

        assert_eq!(
            ordered_limit(1.0, 2.0, 0.0, ExpressionDialect::Ngspice),
            (2.0, false)
        );
        assert_eq!(
            ordered_limit(3.0, 2.0, 0.0, ExpressionDialect::Ngspice),
            (0.0, false)
        );
        assert!(
            ordered_limit(Value::NAN, 0.0, 2.0, ExpressionDialect::Ngspice)
                .0
                .is_nan()
        );
        assert_eq!(
            ordered_limit(Value::NAN, 0.0, 2.0, ExpressionDialect::Xyce),
            (2.0, false)
        );
        assert_eq!(
            ordered_limit(1.0, Value::NAN, 2.0, ExpressionDialect::Xyce),
            (XYCE_NONFINITE_REPLACEMENT, false)
        );
        assert_eq!(
            ordered_limit(1.0, 0.0, Value::NAN, ExpressionDialect::Xyce),
            (1.0, true)
        );
    }

    #[test]
    fn signed_magnitude_power_uses_the_nonnegative_zero_branch() {
        assert_eq!(real_function_pwrs(0.0, 0.0), 1.0);
        assert_eq!(real_function_pwrs(-0.0, 0.0), 1.0);
        assert_eq!(real_function_pwr(0.0, 0.0, ExpressionDialect::Ngspice), 1.0);
        assert!(real_function_pwr(0.0, -1.0, ExpressionDialect::Ngspice).is_infinite());

        assert_eq!(
            signed_magnitude_power_with_derivative(0.0, 0.0.into(), 0.0, 0.0.into()),
            derivative_pair(1.0, 0.0)
        );
        assert_eq!(
            signed_magnitude_power_with_derivative(Value::MAX, 0.0.into(), 2.0, 0.0.into()),
            derivative_pair(Value::INFINITY, 0.0)
        );
    }
}
