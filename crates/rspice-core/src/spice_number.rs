//! Shared SPICE numeric-token semantics.
//!
//! This leaf owns engineering suffixes so the deck lexer, expression parser,
//! and typed-input surfaces cannot drift into incompatible numeric dialects.

use crate::Value;

/// Resolve the SPICE engineering suffix at the head of `text`.
///
/// Returns the multiplier the suffix names together with how many characters
/// of `text` it consumed. Unknown text consumes nothing and scales by one, so
/// callers that require a complete token can reject an unconsumed tail.
pub fn spice_suffix_scale(text: &str) -> (Value, usize) {
    let chars = text.chars().collect::<Vec<_>>();
    parse_spice_suffix(&chars)
}

pub(crate) fn parse_spice_suffix(chars: &[char]) -> (Value, usize) {
    let (scale, consumed) = parse_suffix(chars);
    (scale.parts().0, consumed)
}

fn parse_suffix(chars: &[char]) -> (SuffixScale, usize) {
    if chars.is_empty() {
        return (SuffixScale::Unit, 0);
    }

    if chars.len() >= 3 {
        let three = chars[..3].iter().collect::<String>().to_ascii_uppercase();
        match three.as_str() {
            "MEG" => return (SuffixScale::Mega, 3),
            "MIL" => return (SuffixScale::Mil, 3),
            "GHZ" => return (SuffixScale::Giga, 3),
            "MHZ" => return (SuffixScale::Mega, 3),
            "KHZ" => return (SuffixScale::Kilo, 3),
            "UHZ" => return (SuffixScale::Micro, 3),
            "NHZ" => return (SuffixScale::Nano, 3),
            "PHZ" => return (SuffixScale::Pico, 3),
            "FHZ" => return (SuffixScale::Femto, 3),
            "THZ" => return (SuffixScale::Tera, 3),
            _ => {}
        }
    }

    if chars.len() >= 2 {
        let prefix = chars[0].to_ascii_uppercase();
        let unit = chars[1].to_ascii_uppercase();
        let scale = match unit {
            'S' | 'F' | 'H' => match prefix {
                'N' => Some(SuffixScale::Nano),
                'P' => Some(SuffixScale::Pico),
                'U' => Some(SuffixScale::Micro),
                'M' => Some(SuffixScale::Milli),
                'F' if unit == 'S' => Some(SuffixScale::Femto),
                _ => None,
            },
            'V' | 'A' | 'M' => match prefix {
                'T' => Some(SuffixScale::Tera),
                'G' => Some(SuffixScale::Giga),
                'K' => Some(SuffixScale::Kilo),
                'M' => Some(SuffixScale::Milli),
                'U' => Some(SuffixScale::Micro),
                'N' => Some(SuffixScale::Nano),
                'P' => Some(SuffixScale::Pico),
                'F' => Some(SuffixScale::Femto),
                _ => None,
            },
            _ => None,
        };
        if let Some(scale) = scale {
            return (scale, 2);
        }
    }

    match chars[0].to_ascii_uppercase() {
        'T' => (SuffixScale::Tera, 1),
        'G' => (SuffixScale::Giga, 1),
        'K' => (SuffixScale::Kilo, 1),
        'M' => (SuffixScale::Milli, 1),
        'U' => (SuffixScale::Micro, 1),
        'N' => (SuffixScale::Nano, 1),
        'P' => (SuffixScale::Pico, 1),
        'F' => (SuffixScale::Femto, 1),
        'X' if chars.len() == 1 || !chars[1].is_ascii_alphabetic() => (SuffixScale::Mega, 1),
        'V' | 'A' | 'S' => (SuffixScale::Unit, 1),
        _ => (SuffixScale::Unit, 0),
    }
}

/// Read an entire non-negative integer token without passing through `f64`.
/// Decimal points, exponents and the shared engineering suffix vocabulary
/// are accepted only when their exact value is an integer in the u64 range.
pub(crate) fn parse_spice_u64_complete(text: &str) -> Option<u64> {
    let text = text.trim();
    let negative = text.starts_with('-');
    let text = text.strip_prefix(['+', '-']).unwrap_or(text);
    let bytes = text.as_bytes();
    let mut cursor = 0;
    while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
        cursor += 1;
    }
    let integer_digits = cursor;
    let mut fractional_digits = 0;
    if bytes.get(cursor) == Some(&b'.') {
        cursor += 1;
        let start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        fractional_digits = cursor - start;
    }
    if integer_digits + fractional_digits == 0 {
        return None;
    }
    let mantissa = &text[..cursor];
    let mut exponent = 0_i64;
    if matches!(bytes.get(cursor), Some(b'e' | b'E')) {
        cursor += 1;
        let exponent_start = cursor;
        if matches!(bytes.get(cursor), Some(b'+' | b'-')) {
            cursor += 1;
        }
        let digits_start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        if cursor == digits_start {
            return None;
        }
        let literal = &text[exponent_start..cursor];
        // Magnitudes beyond i64 cannot bring a nonzero u64 into range. Keep
        // the sign so zero and overflow/fraction rejection remain exact.
        exponent = literal.parse().unwrap_or_else(|_| {
            if literal.starts_with('-') {
                i64::MIN
            } else {
                i64::MAX
            }
        });
    }
    let suffix = text[cursor..].chars().collect::<Vec<_>>();
    let (scale, consumed) = parse_suffix(&suffix);
    if consumed != suffix.len() {
        return None;
    }
    let (_, scale_coefficient, scale_exponent) = scale.parts();
    let digits = mantissa
        .bytes()
        .filter(|byte| *byte != b'.')
        .map(char::from)
        .collect::<String>();
    let significant = digits.trim_start_matches('0');
    if significant.is_empty() {
        return Some(0);
    }
    if negative {
        return None;
    }
    let trimmed = significant.trim_end_matches('0');
    let trailing_zeros = significant.len() - trimmed.len();
    let coefficient = trimmed
        .parse::<u128>()
        .ok()?
        .checked_mul(scale_coefficient)?;
    let exponent = exponent
        .saturating_sub(i64::try_from(fractional_digits).ok()?)
        .saturating_add(i64::try_from(trailing_zeros).ok()?)
        .saturating_add(scale_exponent);
    let power = 10_u128.checked_pow(u32::try_from(exponent.unsigned_abs()).ok()?)?;
    let value = if exponent >= 0 {
        coefficient.checked_mul(power)?
    } else {
        if !coefficient.is_multiple_of(power) {
            return None;
        }
        coefficient / power
    };
    u64::try_from(value).ok()
}

#[derive(Clone, Copy)]
enum SuffixScale {
    Unit,
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Kilo,
    Mega,
    Giga,
    Tera,
    Mil,
}

impl SuffixScale {
    /// The existing floating-point multiplier and its exact decimal ratio.
    /// Keep the literal multipliers: computing powers at runtime can change
    /// their bits and would alter ordinary SPICE numeric-token behavior.
    fn parts(self) -> (Value, u128, i64) {
        match self {
            Self::Unit => (1.0, 1, 0),
            Self::Femto => (1e-15, 1, -15),
            Self::Pico => (1e-12, 1, -12),
            Self::Nano => (1e-9, 1, -9),
            Self::Micro => (1e-6, 1, -6),
            Self::Milli => (1e-3, 1, -3),
            Self::Kilo => (1e3, 1, 3),
            Self::Mega => (1e6, 1, 6),
            Self::Giga => (1e9, 1, 9),
            Self::Tera => (1e12, 1, 12),
            Self::Mil => (25.4e-6, 254, -7),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_spice_u64_complete;

    #[test]
    fn integer_literals_preserve_full_width_across_decimal_spellings() {
        let mut value = 0_u64;
        for _ in 0..1024 {
            let digits = value.to_string();
            for text in [
                digits.clone(),
                format!("{digits}.000"),
                format!("{digits}0e-1"),
                format!("{}.{}e{}", &digits[..1], &digits[1..], digits.len() - 1),
            ] {
                assert_eq!(parse_spice_u64_complete(&text), Some(value), "{text}");
            }
            value = value.wrapping_mul(6364136223846793005).wrapping_add(1);
        }
        assert_eq!(
            parse_spice_u64_complete("18446744073709551615"),
            Some(u64::MAX)
        );
        assert_eq!(parse_spice_u64_complete("1e7MIL"), Some(254));
        assert_eq!(parse_spice_u64_complete("5e6MIL"), Some(127));
    }

    #[test]
    fn integer_literals_reject_rounding_overflow_and_partial_tokens() {
        for text in [
            "",
            ".",
            "+",
            "-",
            "--0",
            "-1",
            "NaN",
            "inf",
            "1e",
            "1e+",
            "1.2",
            "1e-1000",
            "1e1000",
            "1.0000000001",
            "-0.0000000001",
            "18446744073709551616",
            "9007199254740993.5",
            "1bogus",
            "0bogus",
            "1kHz!",
            "1 2",
        ] {
            assert_eq!(parse_spice_u64_complete(text), None, "{text}");
        }
        for text in [
            "0",
            "-0",
            "-0.0",
            "0e999999999999999999999999",
            "0e-999999999999999999999999",
            " +001.000e00000 ",
        ] {
            let expected = if text.contains('1') { 1 } else { 0 };
            assert_eq!(parse_spice_u64_complete(text), Some(expected), "{text}");
        }
    }
}
