//! Decimal literals with a scale factor, rounded once.

/// The double nearest the decimal `number` times `coefficient` times ten to
/// the `decade`.
///
/// `10u` written out is 10e-6. Multiplying the double nearest 10 by the
/// double nearest 1e-6 rounds twice and gives 9.999999999999999e-6, a unit in
/// the last place below the double nearest what was written, so `10u - 10e-6`
/// was not zero. Here the decade moves the decimal exponent of the text and
/// the coefficient multiplies its digits (`mil` is 254e-7), so the value is
/// exact until the one final parse.
///
/// `number` is decimal text: an optional sign, digits with at most one point,
/// and an optional `e`/`E` exponent. Anything else, or an exponent beyond
/// `i64`, returns `None`.
pub fn scaled_decimal(number: &str, coefficient: u32, decade: i32) -> Option<f64> {
    if let Some(value) = exactly_scaled_decimal(number, coefficient, decade) {
        return Some(value);
    }
    let (sign, unsigned) = match number.as_bytes().first() {
        Some(b'-') => ("-", &number[1..]),
        Some(b'+') => ("", &number[1..]),
        _ => ("", number),
    };
    let (mantissa, exponent) = match unsigned.find(['e', 'E']) {
        Some(at) => (&unsigned[..at], unsigned[at + 1..].parse::<i64>().ok()?),
        None => (unsigned, 0),
    };
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut digits = integer
        .bytes()
        .chain(fraction.bytes())
        .map(|byte| byte.is_ascii_digit().then(|| byte - b'0'))
        .collect::<Option<Vec<u8>>>()?;
    if digits.is_empty() {
        return None;
    }
    let mut carry = 0_u64;
    for digit in digits.iter_mut().rev() {
        let product = u64::from(*digit) * u64::from(coefficient) + carry;
        *digit = (product % 10) as u8;
        carry = product / 10;
    }
    while carry > 0 {
        digits.insert(0, (carry % 10) as u8);
        carry /= 10;
    }
    let exponent = exponent
        .checked_sub(i64::try_from(fraction.len()).ok()?)?
        .checked_add(i64::from(decade))?;
    let digits = digits
        .into_iter()
        .map(|digit| char::from(b'0' + digit))
        .collect::<String>();
    format!("{sign}{digits}e{exponent}").parse().ok()
}

/// [`scaled_decimal`] of a number without an exponent whose scaled digits
/// and power of ten are both exact doubles: one correctly rounded product or
/// quotient of the two is then the double nearest the decimal, as the parse
/// would give it (Clinger's fast path). `None` leaves any other number to
/// that parse.
fn exactly_scaled_decimal(number: &str, coefficient: u32, decade: i32) -> Option<f64> {
    /// The powers of ten a double holds exactly.
    const EXACT_POWERS: [f64; 23] = [
        1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
        1e17, 1e18, 1e19, 1e20, 1e21, 1e22,
    ];
    /// The largest integer every smaller one of which a double holds exactly.
    const EXACT_INTEGERS: u64 = 1 << 53;
    let (negative, unsigned) = match number.as_bytes().first() {
        Some(b'-') => (true, &number[1..]),
        Some(b'+') => (false, &number[1..]),
        _ => (false, number),
    };
    let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    // Nineteen digits always fit a u64.
    if !(1..=19).contains(&(integer.len() + fraction.len())) {
        return None;
    }
    let mut digits = 0_u64;
    for byte in integer.bytes().chain(fraction.bytes()) {
        if !byte.is_ascii_digit() {
            return None;
        }
        digits = digits * 10 + u64::from(byte - b'0');
    }
    let scaled = digits.checked_mul(u64::from(coefficient))?;
    let exponent = i64::from(decade) - i64::try_from(fraction.len()).ok()?;
    let magnitude = if exponent >= 0 {
        let product = scaled.checked_mul(10_u64.checked_pow(u32::try_from(exponent).ok()?)?)?;
        if product > EXACT_INTEGERS {
            return None;
        }
        product as f64
    } else {
        let power = EXACT_POWERS.get(usize::try_from(-exponent).ok()?)?;
        if scaled > EXACT_INTEGERS {
            return None;
        }
        scaled as f64 / power
    };
    Some(if negative { -magnitude } else { magnitude })
}

#[cfg(test)]
mod tests {
    use super::{exactly_scaled_decimal, scaled_decimal};

    /// Wherever the exact route answers, it answers what the decimal parse
    /// gives: integers and fractions of every length it takes, both signs,
    /// every scale factor, and the bounds of both exact ranges.
    #[test]
    fn the_exact_route_reads_what_the_parse_reads() {
        let mut texts = vec![
            "0".to_owned(),
            "-0".to_owned(),
            "9007199254740992".to_owned(),
            "9007199254740993".to_owned(),
            "9999999999999999999".to_owned(),
            ".5".to_owned(),
            "5.".to_owned(),
            "+4.7".to_owned(),
        ];
        let mut state = 0x9E37_79B9_7F4A_7C15_u64;
        for length in 1..=19 {
            for _ in 0..40 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let digits = format!("{:019}", state % 10_u64.pow(19));
                let digits = &digits[19 - length..];
                let point = (state >> 40) as usize % (length + 1);
                let sign = ["", "-", "+"][(state >> 50) as usize % 3];
                texts.push(format!("{sign}{}.{}", &digits[..point], &digits[point..]));
                texts.push(format!("{sign}{digits}"));
            }
        }
        let mut answered = 0;
        for text in &texts {
            for (coefficient, decade) in [
                (1, -18),
                (1, -15),
                (1, -12),
                (1, -9),
                (1, -6),
                (1, -3),
                (1, 3),
                (1, 6),
                (1, 9),
                (1, 12),
                (254, -7),
                (1, -22),
                (1, -23),
                (1, 0),
            ] {
                let Some(exact) = exactly_scaled_decimal(text, coefficient, decade) else {
                    continue;
                };
                answered += 1;
                let (sign, unsigned) = match text.as_bytes()[0] {
                    b'-' => ("-", &text[1..]),
                    b'+' => ("", &text[1..]),
                    _ => ("", text.as_str()),
                };
                let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
                let product = u128::from(
                    format!("{integer}{fraction}")
                        .parse::<u64>()
                        .expect("digits"),
                ) * u128::from(coefficient);
                let exponent = i64::from(decade) - fraction.len() as i64;
                let parsed: f64 = format!("{sign}{product}e{exponent}")
                    .parse()
                    .expect("a decimal");
                assert_eq!(
                    exact.to_bits(),
                    parsed.to_bits(),
                    "{text} x {coefficient}e{decade}"
                );
            }
        }
        assert!(answered > texts.len(), "{answered} answered");
    }

    /// Every decade a SPICE or Verilog-A scale factor names, with a mantissa
    /// spread from one to just under ten across the whole exponent range, reads
    /// as the double nearest the written value. The product of two rounded
    /// doubles missed about one value in six.
    #[test]
    fn a_scaled_literal_is_the_double_nearest_the_written_value() {
        let mut misses = Vec::new();
        for exponent in -290..=290 {
            for mantissa in [
                1.0,
                1.234_567_890_123_456_7,
                3.2,
                8.738_147_087_027_245,
                9.999_999_999_999_998,
            ] {
                for sign in [1.0, -1.0] {
                    let value: f64 = sign * mantissa * 10_f64.powi(exponent);
                    let written = format!("{value:e}");
                    let (digits, power) = written.split_once('e').expect("an exponent");
                    let power: i32 = power.parse().expect("an integer exponent");
                    for decade in [-18, -15, -12, -9, -6, -3, 3, 6, 9, 12] {
                        let text = format!("{digits}e{}", power - decade);
                        let scaled = scaled_decimal(&text, 1, decade).expect("a decimal");
                        if scaled.to_bits() != value.to_bits() {
                            misses.push(format!("{text} x 1e{decade} read as {scaled:e}"));
                        }
                    }
                }
            }
        }
        assert!(
            misses.is_empty(),
            "{} misses, the first {:?}",
            misses.len(),
            misses.first()
        );
    }

    #[test]
    fn ten_micro_is_ten_times_ten_to_the_minus_six() {
        let ten_micro = scaled_decimal("10", 1, -6).expect("a decimal");
        assert_eq!(ten_micro.to_bits(), 10e-6_f64.to_bits());
        assert_eq!(ten_micro - 10e-6, 0.0);
        assert_eq!(10.0 * 1e-6, 9.999_999_999_999_999e-6, "the double rounding");
    }

    /// `mil` is 25.4e-6, not a power of ten: the digits are multiplied by 254
    /// exactly and the decade shifted by seven.
    #[test]
    fn a_coefficient_multiplies_the_digits_exactly() {
        for (text, written) in [
            ("1", "254e-7"),
            ("3", "762e-7"),
            ("7", "1778e-7"),
            ("1.5", "381e-7"),
            ("-0.25", "-63.5e-7"),
            ("12.345e2", "313563e-7"),
            ("+9.99", "2537.46e-7"),
        ] {
            let expected: f64 = written.parse().expect("a decimal");
            assert_eq!(
                scaled_decimal(text, 254, -7).map(f64::to_bits),
                Some(expected.to_bits()),
                "{text} mil"
            );
        }
        assert_eq!(
            3.0 * 25.4e-6,
            7.620_000_000_000_001e-5,
            "the double rounding"
        );
    }

    #[test]
    fn zero_sign_and_extremes_follow_the_written_value() {
        assert_eq!(scaled_decimal("0", 1, -6).map(f64::to_bits), Some(0));
        assert_eq!(
            scaled_decimal("-0", 1, -6).map(f64::to_bits),
            Some((-0.0_f64).to_bits())
        );
        assert_eq!(scaled_decimal(".5", 1, 3), Some(500.0));
        assert_eq!(scaled_decimal("5.", 1, 3), Some(5000.0));
        assert_eq!(scaled_decimal("1e308", 1, 3), Some(f64::INFINITY));
        assert_eq!(scaled_decimal("1e-320", 1, -15), Some(0.0));
        for invalid in [
            "",
            ".",
            "-",
            "1e",
            "1x",
            "1.2.3",
            "e5",
            "1e99999999999999999999",
        ] {
            assert_eq!(scaled_decimal(invalid, 1, 3), None, "{invalid:?}");
        }
    }
}
