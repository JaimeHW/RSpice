//! SI value parsing and formatting for option fields.

use std::fmt;

/// Parse a value string with SI prefix (e.g., "1u", "10n", "1e-9").
/// Uses SPICE conventions: Meg for mega (not M), m for milli.
pub fn parse_si_value(s: &str) -> Result<f64, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    if let Ok(v) = s.parse::<f64>() {
        return finite_value(s, v);
    }

    let bytes = s.as_bytes();
    let mut split_idx = 0;
    let mut in_exponent = false;

    for (i, &b) in bytes.iter().enumerate() {
        let c = b as char;
        if c.is_ascii_digit() || c == '.' {
            split_idx = i + 1;
        } else if i == 0 && (c == '-' || c == '+') {
            // A leading sign belongs to the numeric part. Without this the
            // scanner stopped at index 0 and every negative suffixed value —
            // including the ones this module's own formatter produces — was
            // rejected as having no numeric part.
            split_idx = 1;
        } else if c == 'e' || c == 'E' {
            if i + 1 < bytes.len() {
                let next_c = bytes[i + 1] as char;
                if next_c.is_ascii_digit() || next_c == '-' || next_c == '+' {
                    split_idx = i + 1;
                    in_exponent = true;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else if (c == '-' || c == '+') && in_exponent {
            split_idx = i + 1;
        } else {
            break;
        }
    }

    if split_idx == 0 {
        return Err(ParseError::NoNumericPart);
    }

    let (num_part, suffix) = s.split_at(split_idx);
    let base: f64 = num_part
        .parse()
        .map_err(|_| ParseError::InvalidNumber(num_part.to_string()))?;
    if !base.is_finite() {
        return Err(ParseError::NonFinite(num_part.to_string()));
    }
    let suffix = suffix.trim().to_lowercase();

    let decade = match suffix.as_str() {
        "" => 0,
        "t" | "tera" => 12,
        "g" | "gig" | "giga" => 9,
        "meg" | "mega" | "x" => 6,
        "k" | "kilo" => 3,
        "m" | "milli" => -3,
        "u" | "micro" => -6,
        micro if micro == "\u{00b5}" || micro == "\u{00c2}\u{00b5}" => -6,
        "n" | "nano" => -9,
        "p" | "pico" => -12,
        "f" | "femto" => -15,
        "a" | "atto" => -18,
        other => return Err(ParseError::UnknownSuffix(other.to_string())),
    };

    finite_value(s, apply_decade(num_part, base, decade)?)
}

/// Apply an SI decade to a literal without multiplying by a power of ten.
///
/// `45u` is not `45.0 * 1e-6`: that product carries the rounding of the
/// multiplication, so the value drifts by an ulp and a round trip through the
/// options dialog no longer returns the number the user typed. Folding the
/// decade into the literal's own exponent instead hands the whole thing to the
/// correctly-rounded float parser, which returns the nearest double to the
/// decimal value exactly once.
fn apply_decade(num_part: &str, base: f64, decade: i32) -> Result<f64, ParseError> {
    if decade == 0 {
        return Ok(base);
    }
    let (mantissa, own_exponent) = match num_part.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (
            mantissa,
            exponent
                .parse::<i32>()
                .map_err(|_| ParseError::InvalidNumber(num_part.to_string()))?,
        ),
        None => (num_part, 0),
    };
    let combined = own_exponent
        .checked_add(decade)
        .ok_or_else(|| ParseError::InvalidNumber(num_part.to_string()))?;
    format!("{mantissa}e{combined}")
        .parse::<f64>()
        .map_err(|_| ParseError::InvalidNumber(num_part.to_string()))
}

fn finite_value(source: &str, value: f64) -> Result<f64, ParseError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ParseError::NonFinite(source.to_string()))
    }
}

/// Format a value with SI prefix.
/// Uses SPICE conventions: Meg for mega (not M, which means milli).
///
/// The result parses back to exactly the same `f64`. That is the whole point:
/// this pair is the persistence format for every solver option, so a value
/// that changed on a display round trip would mean the number the engine ran
/// under was not the number the project recorded. Two things are needed for
/// it — the decade is applied by shifting the decimal point rather than by
/// dividing, and the mantissa keeps every digit the shortest round-trip
/// representation needs rather than being truncated to six decimals.
pub fn format_si_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    if !v.is_finite() {
        return format!("{v}");
    }

    // `{:e}` is the shortest decimal that reads back as exactly this double.
    let scientific = format!("{v:e}");
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("LowerExp always emits an exponent");
    let Ok(exponent) = exponent.parse::<i32>() else {
        return format!("{v}");
    };

    let (decade, suffix) = match exponent {
        e if e >= 12 => (12, "T"),
        e if e >= 9 => (9, "G"),
        e if e >= 6 => (6, "Meg"),
        e if e >= 3 => (3, "k"),
        e if e >= 0 => (0, ""),
        e if e >= -3 => (-3, "m"),
        e if e >= -6 => (-6, "u"),
        e if e >= -9 => (-9, "n"),
        e if e >= -12 => (-12, "p"),
        e if e >= -15 => (-15, "f"),
        _ => (-18, "a"),
    };
    // `decade` is the multiple of three at or below `exponent`, so the shift
    // is 0, 1 or 2 places — except far below atto, where the value keeps its
    // own exponent rather than growing a run of leading zeros.
    let Ok(shift) = u32::try_from(exponent - decade) else {
        return format!("{v}");
    };
    if shift > 2 {
        return format!("{mantissa}e{exponent}");
    }
    format!("{}{}", shift_decimal_point(mantissa, shift), suffix)
}

/// Move a decimal mantissa's point `places` digits to the right.
///
/// Purely textual, so it cannot introduce a rounding error the way
/// multiplying by a power of ten does.
fn shift_decimal_point(mantissa: &str, places: u32) -> String {
    if places == 0 {
        return mantissa.to_owned();
    }
    let (sign, digits) = match mantissa.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", mantissa),
    };
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let places = places as usize;
    let mut moved: String = fraction.chars().take(places).collect();
    while moved.len() < places {
        moved.push('0');
    }
    let remainder: String = fraction.chars().skip(places).collect();
    if remainder.is_empty() {
        format!("{sign}{whole}{moved}")
    } else {
        format!("{sign}{whole}{moved}.{remainder}")
    }
}

/// Parse error types.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Empty,
    NoNumericPart,
    InvalidNumber(String),
    NonFinite(String),
    UnknownSuffix(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Empty string"),
            ParseError::NoNumericPart => write!(f, "No numeric part found"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::NonFinite(s) => write!(f, "Non-finite numeric value: {}", s),
            ParseError::UnknownSuffix(s) => write!(f, "Unknown SI suffix: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::{ParseError, format_si_value, parse_si_value};

    #[test]
    fn parse_si_value_rejects_non_finite_literals() {
        for text in ["NaN", "inf", "-inf", "1e309"] {
            assert!(
                matches!(parse_si_value(text), Err(ParseError::NonFinite(_))),
                "{text} should be rejected as non-finite"
            );
        }
    }

    /// This pair is the persistence format for every solver option, so a value
    /// that changed across a display round trip would mean the engine ran under
    /// a different number than the project recorded.
    #[test]
    fn formatting_and_parsing_round_trip_every_option_magnitude_exactly() {
        let values = [
            1e-3,
            1e-6,
            1e-9,
            1e-12,
            1e-14,
            1e-15,
            1e-18,
            4.5e-5,
            3.5e-7,
            8.5e-13,
            2.5e-14,
            1.234_567_891e-5,
            0.125,
            1.0,
            7.0,
            27.0,
            8.0,
            1e3,
            1.5e6,
            6.02e23,
            -1e-12,
            -4.5e-5,
        ];
        for value in values {
            let text = format_si_value(value);
            let parsed = parse_si_value(&text)
                .unwrap_or_else(|error| panic!("{value:e} formatted as {text:?} but {error}"));
            assert_eq!(
                parsed.to_bits(),
                value.to_bits(),
                "{value:e} formatted as {text:?} and parsed back as {parsed:e}"
            );
        }
    }

    /// The decade must be applied to the literal, not multiplied in. `45u`
    /// through a `1e-6` multiplication lands one ulp away from `4.5e-5`.
    #[test]
    fn an_si_suffix_never_introduces_a_multiplication_rounding_error() {
        assert_eq!(parse_si_value("45u").expect("valid"), 4.5e-5);
        assert_eq!(parse_si_value("850f").expect("valid"), 8.5e-13);
        assert_eq!(parse_si_value("0.35u").expect("valid"), 3.5e-7);
        assert_eq!(parse_si_value("1.5e3m").expect("valid"), 1.5);
    }

    #[test]
    fn zero_and_plain_magnitudes_keep_their_conventional_spelling() {
        assert_eq!(format_si_value(0.0), "0");
        assert_eq!(format_si_value(1e-12), "1p");
        assert_eq!(format_si_value(1e-6), "1u");
        assert_eq!(format_si_value(1e6), "1Meg");
        assert_eq!(format_si_value(27.0), "27");
    }
}
