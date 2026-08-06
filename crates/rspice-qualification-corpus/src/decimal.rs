//! Exact-arithmetic mirror of the cloud admission algorithm
//! `rspice-decimal-v1`: release admission recomputes every oracle
//! comparison with arbitrary-precision decimals, so corpus generation must
//! accept a case with exactly the same arithmetic — a value that squeaks
//! through a float comparison here but fails the exact one there would
//! poison a signed corpus.

use num_bigint::BigInt;
use num_traits_shim::{abs, is_multiple_of_ten, pow10};

/// An exact decimal: `numerator / 10^scale`, normalized so `scale >= 0`
/// and the numerator carries no trailing factor of ten while scaled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decimal {
    numerator: BigInt,
    scale: u32,
}

/// The outcome of one tolerance comparison, in admission's own terms.
pub struct Comparison {
    pub passed: bool,
    /// `|observed - expected|` rendered exactly as admission renders it;
    /// the qualification evaluator must reproduce this string.
    pub absolute_error_decimal: String,
}

/// Parses a decimal in the corpus grammar
/// `^-?(0|[1-9][0-9]{0,30})(\.[0-9]{1,30})?([eE][+-]?(0|[1-9][0-9]{0,2}))?$`,
/// with admission's additional exponent bound of ±100. `None` mirrors the
/// verifier throwing: such a value can never enter a corpus or a report.
pub fn parse_decimal(value: &str) -> Option<Decimal> {
    let rest = value.strip_prefix('-');
    let negative = rest.is_some();
    let rest = rest.unwrap_or(value);
    let (mantissa, exponent) = match rest.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, exponent),
        None => (rest, "0"),
    };
    let (whole, fraction) = match mantissa.split_once('.') {
        Some((whole, fraction)) => (whole, fraction),
        None => (mantissa, ""),
    };
    let whole_ok = whole == "0"
        || ((1..=31).contains(&whole.len())
            && whole.as_bytes()[0] != b'0'
            && whole.bytes().all(|byte| byte.is_ascii_digit()));
    let fraction_ok = mantissa.split_once('.').is_none()
        || ((1..=30).contains(&fraction.len())
            && fraction.bytes().all(|byte| byte.is_ascii_digit()));
    let exponent_digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
    let exponent_ok = !exponent_digits.is_empty()
        && exponent_digits.len() <= 3
        && (exponent_digits == "0" || exponent_digits.as_bytes()[0] != b'0')
        && exponent_digits.bytes().all(|byte| byte.is_ascii_digit());
    if !(whole_ok && fraction_ok && exponent_ok) {
        return None;
    }
    let exponent: i32 = exponent.parse().ok()?;
    if !(-100..=100).contains(&exponent) {
        return None;
    }

    let mut numerator: BigInt = format!("{whole}{fraction}").parse().ok()?;
    if negative {
        numerator = -numerator;
    }
    let mut scale = i64::from(u32::try_from(fraction.len()).ok()?) - i64::from(exponent);
    if scale < 0 {
        numerator *= pow10(u32::try_from(-scale).ok()?);
        scale = 0;
    }
    let mut scale = u32::try_from(scale).ok()?;
    while scale > 0 && is_multiple_of_ten(&numerator) {
        numerator /= 10;
        scale -= 1;
    }
    Some(Decimal { numerator, scale })
}

fn rescaled(value: &Decimal, scale: u32) -> BigInt {
    &value.numerator * pow10(scale - value.scale)
}

fn absolute_difference(left: &Decimal, right: &Decimal) -> Decimal {
    let scale = left.scale.max(right.scale);
    Decimal {
        numerator: abs(rescaled(left, scale) - rescaled(right, scale)),
        scale,
    }
}

fn add(left: &Decimal, right: &Decimal) -> Decimal {
    let scale = left.scale.max(right.scale);
    Decimal {
        numerator: rescaled(left, scale) + rescaled(right, scale),
        scale,
    }
}

fn multiply(left: &Decimal, right: &Decimal) -> Decimal {
    Decimal {
        numerator: &left.numerator * &right.numerator,
        scale: left.scale + right.scale,
    }
}

fn less_than_or_equal(left: &Decimal, right: &Decimal) -> bool {
    let scale = left.scale.max(right.scale);
    rescaled(left, scale) <= rescaled(right, scale)
}

/// Renders a decimal exactly as admission's `decimalToString`: trailing
/// zeros dropped from the fraction, plain notation, no exponent.
pub fn decimal_to_string(value: &Decimal) -> String {
    let mut numerator = value.numerator.clone();
    let mut scale = value.scale;
    while scale > 0 && is_multiple_of_ten(&numerator) {
        numerator /= 10;
        scale -= 1;
    }
    let negative = numerator < BigInt::ZERO;
    let mut digits = abs(numerator).to_string();
    let sign = if negative { "-" } else { "" };
    if scale == 0 {
        return format!("{sign}{digits}");
    }
    let scale = scale as usize;
    if digits.len() < scale + 1 {
        digits = format!("{}{digits}", "0".repeat(scale + 1 - digits.len()));
    }
    let split = digits.len() - scale;
    format!("{sign}{}.{}", &digits[..split], &digits[split..])
}

/// `|observed - expected| <= absolute + relative * |expected|`, all exact.
/// `None` when any operand fails the grammar or a tolerance is negative —
/// admission rejects both, so generation must too.
pub fn within_tolerance(
    observed: &str,
    expected: &str,
    absolute_tolerance: &str,
    relative_tolerance: &str,
) -> Option<Comparison> {
    let observed = parse_decimal(observed)?;
    let expected = parse_decimal(expected)?;
    let absolute_tolerance = parse_decimal(absolute_tolerance)?;
    let relative_tolerance = parse_decimal(relative_tolerance)?;
    if absolute_tolerance.numerator < BigInt::ZERO || relative_tolerance.numerator < BigInt::ZERO {
        return None;
    }
    let error = absolute_difference(&observed, &expected);
    let absolute_expected = Decimal {
        numerator: abs(expected.numerator.clone()),
        scale: expected.scale,
    };
    let limit = add(
        &absolute_tolerance,
        &multiply(&relative_tolerance, &absolute_expected),
    );
    Some(Comparison {
        passed: less_than_or_equal(&error, &limit),
        absolute_error_decimal: decimal_to_string(&error),
    })
}

/// The few big-integer helpers this module needs, kept local so the crate
/// only depends on `num-bigint` itself.
mod num_traits_shim {
    use num_bigint::BigInt;

    pub fn pow10(exponent: u32) -> BigInt {
        BigInt::from(10).pow(exponent)
    }

    pub fn abs(value: BigInt) -> BigInt {
        if value < BigInt::ZERO { -value } else { value }
    }

    pub fn is_multiple_of_ten(value: &BigInt) -> bool {
        value % 10 == BigInt::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_matches_the_admission_grammar_and_bounds() {
        for valid in ["0", "-1.5", "2.5e-3", "1e100", "-1e-100", "0e0", "12.340"] {
            assert!(parse_decimal(valid).is_some(), "{valid} must parse");
        }
        for invalid in [
            "", "-", "01", "1.", ".5", "+1", "1e", "1e101", "1e-101", "1e004", "1E+1000", "NaN",
            "1_0", "0x1",
        ] {
            assert!(
                parse_decimal(invalid).is_none(),
                "{invalid} must be rejected"
            );
        }
    }

    #[test]
    fn normalization_matches_admission() {
        // 12.340 => numerator 1234, scale 2 (trailing zero stripped).
        assert_eq!(parse_decimal("12.340"), parse_decimal("12.34"));
        // 2.5e3 => scale -1 clamps to 0 by multiplying out: 2500.
        assert_eq!(parse_decimal("2.5e3"), parse_decimal("2500"));
        assert_eq!(parse_decimal("-0"), parse_decimal("0"));
    }

    #[test]
    fn absolute_error_strings_match_admission_rendering() {
        let comparison = within_tolerance("1.5", "1.25", "1", "0").expect("comparison");
        assert_eq!(comparison.absolute_error_decimal, "0.25");
        let comparison = within_tolerance("2e-3", "1e-3", "1", "0").expect("comparison");
        assert_eq!(comparison.absolute_error_decimal, "0.001");
        let comparison = within_tolerance("5", "5", "0", "0").expect("comparison");
        assert_eq!(comparison.absolute_error_decimal, "0");
        assert!(comparison.passed);
    }

    #[test]
    fn tolerance_composition_is_exact() {
        // limit = 1e-6 + 1e-3 * 2 = 0.002001; error = 0.002 => passes.
        let passing = within_tolerance("2.002", "2", "1e-6", "1e-3").expect("comparison");
        assert!(passing.passed);
        // error = 0.0020011 exceeds the limit by 1e-7 => fails, exactly.
        let failing = within_tolerance("2.0020011", "2", "1e-6", "1e-3").expect("comparison");
        assert!(!failing.passed);
        // Floating-point arithmetic would blur this boundary; the exact
        // mirror must sit exactly on it.
        let boundary = within_tolerance("2.002001", "2", "1e-6", "1e-3").expect("comparison");
        assert!(boundary.passed);
    }

    #[test]
    fn negative_tolerances_are_rejected() {
        assert!(within_tolerance("1", "1", "-1", "0").is_none());
        assert!(within_tolerance("1", "1", "0", "-1e-3").is_none());
    }

    #[test]
    fn extreme_exponents_compose_without_overflow() {
        let comparison =
            within_tolerance("1e100", "1e100", "1e-100", "1e-100").expect("comparison");
        assert!(comparison.passed);
        let comparison =
            within_tolerance("1.000001e100", "1e100", "0", "1e-9").expect("comparison");
        assert!(!comparison.passed);
    }
}
