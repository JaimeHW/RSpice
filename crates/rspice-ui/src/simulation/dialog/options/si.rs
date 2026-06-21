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

    let multiplier = match suffix.as_str() {
        "" => 1.0,
        "t" | "tera" => 1e12,
        "g" | "gig" | "giga" => 1e9,
        "meg" | "mega" | "x" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" => 1e-6,
        micro if micro == "\u{00b5}" || micro == "\u{00c2}\u{00b5}" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        other => return Err(ParseError::UnknownSuffix(other.to_string())),
    };

    finite_value(s, base * multiplier)
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
pub fn format_si_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }

    let abs = v.abs();
    let (scaled, suffix) = if abs >= 1e12 {
        (v / 1e12, "T")
    } else if abs >= 1e9 {
        (v / 1e9, "G")
    } else if abs >= 1e6 {
        (v / 1e6, "Meg")
    } else if abs >= 1e3 {
        (v / 1e3, "k")
    } else if abs >= 1.0 {
        (v, "")
    } else if abs >= 1e-3 {
        (v * 1e3, "m")
    } else if abs >= 1e-6 {
        (v * 1e6, "u")
    } else if abs >= 1e-9 {
        (v * 1e9, "n")
    } else if abs >= 1e-12 {
        (v * 1e12, "p")
    } else if abs >= 1e-15 {
        (v * 1e15, "f")
    } else {
        (v * 1e18, "a")
    };

    let formatted = format!("{:.6}", scaled);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format!("{}{}", trimmed, suffix)
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
    use super::{ParseError, parse_si_value};

    #[test]
    fn parse_si_value_rejects_non_finite_literals() {
        for text in ["NaN", "inf", "-inf", "1e309"] {
            assert!(
                matches!(parse_si_value(text), Err(ParseError::NonFinite(_))),
                "{text} should be rejected as non-finite"
            );
        }
    }
}
