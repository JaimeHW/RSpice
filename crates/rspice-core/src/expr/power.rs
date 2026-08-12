//! Dialect-specific real projections of the power operator.
//!
//! Xyce evaluates expressions with a complex scalar and projects a behavioral
//! source back onto the real system.  Consequently, a negative real base and
//! a fractional real exponent follows the principal complex branch rather
//! than producing `NaN`.  Keeping the value and its Newton derivative here
//! prevents the bytecode and analytic evaluators from drifting apart.

use crate::Value;
use crate::config::ExpressionDialect;

const XYCE_NONFINITE_REPLACEMENT: Value = 1.0e50;

/// Apply Xyce's public expression-boundary `fixNan`/`fixInf` policy.
///
/// This must be called only after the complete expression (or derivative) has
/// been evaluated.  Normalizing bytecode intermediates would change IEEE
/// expressions such as `0*exp(1000)` from NaN to zero.
#[inline]
pub(crate) fn normalize_expression_boundary(value: Value, dialect: ExpressionDialect) -> Value {
    if dialect == ExpressionDialect::Xyce && !value.is_finite() {
        XYCE_NONFINITE_REPLACEMENT.copysign(value)
    } else {
        value
    }
}

/// Evaluate the real-valued power operation for one expression dialect.
#[inline]
pub(crate) fn real_pow(base: Value, exponent: Value, dialect: ExpressionDialect) -> Value {
    if dialect != ExpressionDialect::Xyce || base >= 0.0 || exponent.fract() == 0.0 {
        return base.powf(exponent);
    }

    let (_, cos_pi_exponent) = sin_cos_pi(exponent);
    base.abs().powf(exponent) * cos_pi_exponent
}

/// Evaluate the named `POW` function. Xyce aliases `POW` and `PWR` to its
/// complex-projected power operation, while ngspice's behavioral `POW` uses
/// the magnitude of a negative base.
#[inline]
pub(crate) fn real_function_pow(base: Value, exponent: Value, dialect: ExpressionDialect) -> Value {
    if dialect == ExpressionDialect::Xyce {
        real_pow(base, exponent, dialect)
    } else {
        base.abs().powf(exponent)
    }
}

/// Evaluate the named `PWR` function. It is an alias of `POW` in Xyce and a
/// sign-preserving magnitude power in ngspice behavioral expressions.
#[inline]
pub(crate) fn real_function_pwr(base: Value, exponent: Value, dialect: ExpressionDialect) -> Value {
    if dialect == ExpressionDialect::Xyce {
        real_pow(base, exponent, dialect)
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
pub(crate) fn ordered_sign(value: Value) -> Value {
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
pub(crate) fn ordered_limit(
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

/// Evaluate a power operation and its directional derivative.
///
/// `d_base` and `d_exponent` are derivatives with respect to the same Newton
/// variable.  `None` denotes a genuine real-domain singularity so the caller
/// can use its established numerical fallback rather than stamping a bogus
/// finite slope.
pub(crate) fn real_pow_with_derivative(
    base: Value,
    d_base: Value,
    exponent: Value,
    d_exponent: Value,
    dialect: ExpressionDialect,
) -> Option<(Value, Value)> {
    if dialect != ExpressionDialect::Xyce {
        return legacy_real_pow_with_derivative(base, d_base, exponent, d_exponent);
    }

    let value = real_pow(base, exponent, dialect);
    if !base.is_finite() || !exponent.is_finite() {
        return Some((value, Value::NAN));
    }

    if base > 0.0 {
        let mut derivative = 0.0;
        if d_base != 0.0 {
            derivative += exponent * base.powf(exponent - 1.0) * d_base;
        }
        if d_exponent != 0.0 {
            derivative += value * base.ln() * d_exponent;
        }
        return Some((value, derivative));
    }

    if base < 0.0 {
        let magnitude = base.abs().powf(exponent);
        let (sin_pi_exponent, cos_pi_exponent) = sin_cos_pi(exponent);
        let base_partial = exponent * magnitude * cos_pi_exponent / base;
        let exponent_partial = magnitude
            * (base.abs().ln() * cos_pi_exponent - std::f64::consts::PI * sin_pi_exponent);
        // Do not manufacture NaN from an absent direction (`inf * 0`).  This
        // matters for high-magnitude constant bases/exponents where one
        // partial is singular but the requested Newton direction is exactly
        // independent of it.
        let mut derivative = 0.0;
        if d_base != 0.0 {
            derivative += base_partial * d_base;
        }
        if d_exponent != 0.0 {
            derivative += exponent_partial * d_exponent;
        }
        return Some((value, derivative));
    }

    // Xyce 7.10's powOp::dx2 leaves every derivative slot at exactly zero
    // whenever the base is zero, independent of exponent and direction.  The
    // value retains ordinary pow semantics and is normalized only at the
    // completed expression boundary.
    Some((value, 0.0))
}

pub(crate) fn real_function_pow_with_derivative(
    base: Value,
    d_base: Value,
    exponent: Value,
    d_exponent: Value,
    dialect: ExpressionDialect,
) -> Option<(Value, Value)> {
    if dialect == ExpressionDialect::Xyce {
        return real_pow_with_derivative(base, d_base, exponent, d_exponent, dialect);
    }
    magnitude_power_with_derivative(base, d_base, exponent, d_exponent, false)
}

pub(crate) fn real_function_pwr_with_derivative(
    base: Value,
    d_base: Value,
    exponent: Value,
    d_exponent: Value,
    dialect: ExpressionDialect,
) -> Option<(Value, Value)> {
    if dialect == ExpressionDialect::Xyce {
        return real_pow_with_derivative(base, d_base, exponent, d_exponent, dialect);
    }
    magnitude_power_with_derivative(base, d_base, exponent, d_exponent, true)
}

pub(crate) fn real_function_pwrs_with_derivative(
    base: Value,
    d_base: Value,
    exponent: Value,
    d_exponent: Value,
    dialect: ExpressionDialect,
) -> Option<(Value, Value)> {
    if dialect == ExpressionDialect::Xyce && base == 0.0 {
        return Some((real_function_pwrs(base, exponent), 0.0));
    }
    magnitude_power_with_derivative(base, d_base, exponent, d_exponent, true)
}

fn magnitude_power_with_derivative(
    base: Value,
    d_base: Value,
    exponent: Value,
    d_exponent: Value,
    preserve_sign: bool,
) -> Option<(Value, Value)> {
    let magnitude = base.abs();
    let branch_sign = if base < 0.0 { -1.0 } else { 1.0 };
    let polarity = if preserve_sign { branch_sign } else { 1.0 };
    let value = polarity * magnitude.powf(exponent);
    let mut derivative = 0.0;
    if d_base != 0.0 {
        let base_polarity = if preserve_sign { 1.0 } else { branch_sign };
        derivative += base_polarity * exponent * magnitude.powf(exponent - 1.0) * d_base;
    }
    if d_exponent != 0.0 {
        derivative += value * magnitude.ln() * d_exponent;
    }
    Some((value, derivative))
}

fn legacy_real_pow_with_derivative(
    base: Value,
    d_base: Value,
    exponent: Value,
    d_exponent: Value,
) -> Option<(Value, Value)> {
    let value = base.powf(exponent);
    if d_exponent == 0.0 {
        return Some((value, exponent * base.powf(exponent - 1.0) * d_base));
    }
    if base > 0.0 {
        Some((
            value,
            value * (d_exponent * base.ln() + exponent * d_base / base),
        ))
    } else {
        None
    }
}

/// Compute sin(pi*x) and cos(pi*x), preserving exact integer parity.
///
/// The exact integer path is important for negative integer powers: otherwise
/// a tiny library `sin(n*pi)` residue leaks into the exponent derivative and
/// the cosine can perturb an otherwise exact signed power value.
fn sin_cos_pi(value: Value) -> (Value, Value) {
    if value.is_finite() && value.fract() == 0.0 {
        let cosine = if value.rem_euclid(2.0) == 0.0 {
            1.0
        } else {
            -1.0
        };
        (0.0, cosine)
    } else {
        (std::f64::consts::PI * value).sin_cos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xyce_negative_fractional_power_is_principal_real_projection() {
        let expected = 2.5_f64.powf(3.1) * (3.1 * std::f64::consts::PI).cos();
        assert_eq!(real_pow(-2.5, 3.1, ExpressionDialect::Xyce), expected);
        assert!(real_pow(-2.5, 3.1, ExpressionDialect::Ngspice).is_nan());
    }

    #[test]
    fn xyce_integer_and_zero_power_edges_are_deliberate() {
        assert_eq!(real_pow(-2.0, 3.0, ExpressionDialect::Xyce), -8.0);
        assert_eq!(real_pow(-2.0, -2.0, ExpressionDialect::Xyce), 0.25);
        assert_eq!(real_pow(0.0, 0.0, ExpressionDialect::Xyce), 1.0);
        assert_eq!(real_pow(0.0, 2.0, ExpressionDialect::Xyce), 0.0);
        assert!(real_pow(0.0, -1.0, ExpressionDialect::Xyce).is_infinite());

        assert_eq!(
            real_pow_with_derivative(0.0, 1.0, 2.0, 0.0, ExpressionDialect::Xyce),
            Some((0.0, 0.0))
        );
        assert_eq!(
            real_pow_with_derivative(0.0, 1.0, 1.0, 0.0, ExpressionDialect::Xyce),
            Some((0.0, 0.0))
        );
        assert_eq!(
            real_pow_with_derivative(0.0, 1.0, 0.5, 0.0, ExpressionDialect::Xyce),
            Some((0.0, 0.0))
        );
        let (negative_power, derivative) =
            real_pow_with_derivative(0.0, 1.0, -1.0, 0.0, ExpressionDialect::Xyce)
                .expect("Xyce zero-base pow derivative is defined as zero");
        assert!(negative_power.is_infinite());
        assert_eq!(derivative, 0.0);
    }

    #[test]
    fn inactive_derivative_directions_do_not_create_nan() {
        let (value, derivative) =
            real_pow_with_derivative(-1.0e308, 0.0, 2.0, 0.0, ExpressionDialect::Xyce)
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
            magnitude_power_with_derivative(0.0, 0.0, 0.0, 0.0, false),
            Some((1.0, 0.0))
        );
        assert_eq!(
            magnitude_power_with_derivative(Value::MAX, 0.0, 2.0, 0.0, false),
            Some((Value::INFINITY, 0.0))
        );
    }
}
