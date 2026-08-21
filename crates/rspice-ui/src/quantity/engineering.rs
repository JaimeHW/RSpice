//! Engineering-notation parsing and formatting ("1k", "10u", "3.3Meg").

/// Parse an engineering notation value (e.g., "1k", "10u", "3.3meg").
///
/// Returns the numeric value or an error message.
pub fn parse_engineering_value(input: &str) -> Result<f64, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("Empty value".to_string());
    }

    // Find where the number ends and the suffix begins
    let mut numeric_end = 0;
    let chars: Vec<char> = trimmed.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+' || *c == 'e' || *c == 'E' {
            numeric_end = i + 1;
        } else {
            break;
        }
    }

    // Parse the numeric part
    let numeric_str = &trimmed[..trimmed.chars().take(numeric_end).collect::<String>().len()];
    let base_value: f64 = numeric_str
        .parse()
        .map_err(|_| format!("Cannot parse '{}' as number", numeric_str))?;

    // Parse the suffix - take the rest and compare lowercase/normalized
    let suffix: String = trimmed.chars().skip(numeric_end).collect();
    let suffix = suffix.trim();
    let suffix_lower = suffix.to_lowercase();

    // Normalize micro sign: U+00B5 (µ MICRO SIGN) and U+03BC (μ GREEK SMALL LETTER MU) both map to 'u'
    let normalized_suffix = suffix_lower.replace(['\u{00B5}', '\u{03BC}'], "u");

    let multiplier = match normalized_suffix.as_str() {
        "" => 1.0,
        "t" | "tera" => 1e12,
        "g" | "giga" => 1e9,
        "meg" | "mega" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        _ => {
            return Err(format!(
                "Unknown suffix: '{}' (normalized: '{}')",
                suffix, normalized_suffix
            ));
        }
    };

    Ok(base_value * multiplier)
}

/// Format a value with engineering notation.
pub fn format_engineering_value(value: f64) -> String {
    let abs_value = value.abs();

    // Special case: zero should never have a suffix
    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "Meg")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    // Format with appropriate precision, using epsilon for floating-point comparison
    let eps = 1e-9;
    let is_int = (scaled.round() - scaled).abs() < eps;
    let is_one_decimal = ((scaled * 10.0).round() - scaled * 10.0).abs() < eps;
    let is_two_decimal = ((scaled * 100.0).round() - scaled * 100.0).abs() < eps;

    if is_int {
        format!("{:.0}{}", scaled.round(), suffix)
    } else if is_one_decimal {
        format!("{:.1}{}", scaled, suffix)
    } else if is_two_decimal {
        format!("{:.2}{}", scaled, suffix)
    } else {
        format!("{:.3}{}", scaled, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::{format_engineering_value, parse_engineering_value};

    /// The suffix ladder every engineering-notation surface in the crate
    /// reads, including the two Unicode micro signs a keyboard or a pasted
    /// datasheet can produce: U+00B5 MICRO SIGN and U+03BC GREEK SMALL LETTER
    /// MU, which look alike and must mean the same decade.
    #[test]
    fn the_suffix_ladder_follows_spice_and_accepts_both_micro_signs() {
        for (text, expected) in [
            ("10", 10.0),
            ("1.5k", 1.5e3),
            ("1.5K", 1.5e3),
            ("2.2u", 2.2e-6),
            ("2.2\u{00B5}", 2.2e-6),
            ("2.2\u{03BC}", 2.2e-6),
            ("2.2micro", 2.2e-6),
            // `m` is milli and `meg` is mega, in any case. SPICE has no
            // case-sensitive `M`, so a value that reads as mega to a human
            // has to be spelled out to be read as mega here.
            ("3m", 3e-3),
            ("3M", 3e-3),
            ("3meg", 3e6),
            ("3Meg", 3e6),
            ("3MEG", 3e6),
            ("1t", 1e12),
            ("1g", 1e9),
            ("1n", 1e-9),
            ("1p", 1e-12),
            ("1f", 1e-15),
            ("1a", 1e-18),
            ("1e3", 1e3),
            ("-4.7k", -4.7e3),
        ] {
            let parsed = parse_engineering_value(text)
                .unwrap_or_else(|error| panic!("{text} should parse: {error}"));
            assert!(
                (parsed - expected).abs() <= expected.abs() * 1e-12,
                "{text} parsed as {parsed}, expected {expected}"
            );
        }
    }

    /// A suffix the ladder does not name is a parse failure. Defaulting it to
    /// a multiplier of one would read the leading digits of any text at all —
    /// a part number, a unit-bearing label — as a quantity.
    #[test]
    fn an_unnamed_suffix_is_an_error_rather_than_a_multiplier_of_one() {
        for text in ["", "12x", "1N4148", "5 volts", "1kohm", "abc"] {
            assert!(
                parse_engineering_value(text).is_err(),
                "{text:?} must not parse as an engineering value"
            );
        }
    }

    #[test]
    fn formatting_round_trips_through_the_parser_at_every_decade() {
        for value in [
            3.3e6, 4.7e3, 1.0, 2.2e-3, 1.5e-6, 100.0e-9, 47.0e-12, 1.0e-15, 0.0, -6.8e3,
        ] {
            let formatted = format_engineering_value(value);
            let parsed = parse_engineering_value(&formatted)
                .unwrap_or_else(|error| panic!("{formatted} should parse back: {error}"));
            assert!(
                (parsed - value).abs() <= value.abs() * 1e-9,
                "{value} formatted as {formatted}, which parsed back as {parsed}"
            );
        }
    }
}
