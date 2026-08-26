//! Canonical parser for scalar SPICE values shared by plan and execution code.

pub(crate) fn parse_spice_value_checked(s: &str) -> Result<f64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("value is empty".to_string());
    }

    if let Ok(v) = s.parse::<f64>() {
        if v.is_finite() {
            return Ok(v);
        }
        return Err("value is not finite".to_string());
    }

    let mut num_end = 0;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }

    if num_end == 0 || num_end == s.len() && s.parse::<f64>().is_err() {
        return Err(format!("invalid numeric value '{}'", s));
    }

    let (num_str, suffix) = s.split_at(num_end);
    let base: f64 = num_str
        .parse()
        .map_err(|_| format!("invalid numeric value '{}'", s))?;

    let multiplier = match suffix.to_ascii_lowercase().as_str() {
        "t" | "tera" => 1e12,
        "g" | "gig" => 1e9,
        "meg" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        "" => 1.0,
        _ => return Err(format!("unsupported SPICE suffix '{}'", suffix)),
    };

    let value = base * multiplier;
    if value.is_finite() {
        Ok(value)
    } else {
        Err("value is not finite".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::parse_spice_value_checked;

    #[test]
    fn parses_scientific_notation_and_spice_suffixes_without_non_finite_values() {
        assert_eq!(parse_spice_value_checked("1e-6").unwrap(), 1.0e-6);
        assert_eq!(parse_spice_value_checked("10n").unwrap(), 10.0e-9);
        assert!(parse_spice_value_checked("NaN").is_err());
        assert!(parse_spice_value_checked("1wat").is_err());
    }

    /// This parser and the interactive
    /// [`crate::quantity::parse_engineering_value`] read the same fields of
    /// the same design — a stop time, a source amplitude — so where both
    /// accept a spelling they have to agree on the number.
    ///
    /// The three rows listed as divergent are the ones this parser still
    /// reads as atto, which the interactive parser dropped: with unit letters
    /// after a scale factor accepted, `A` is amperes, so `1A` cannot also be
    /// 1e-18. Until atto leaves this table too, a transient stop time typed
    /// `1A` is 1e-18 on this path and one ampere on the other.
    ///
    /// The test lives here rather than beside the other parser because
    /// `quantity` is the crate's bottom layer and may not name `simulation`.
    #[test]
    fn the_interactive_parser_agrees_wherever_both_accept_a_spelling() {
        use crate::quantity::parse_engineering_value;

        let divergent = ["1a", "1A", "1atto"];
        for text in [
            "1e-6", "10n", "1k", "3.3meg", "2.2u", "1m", "1t", "1g", "1p", "1f", "-4.7k", "1kilo",
            "1micro", "1milli", "1nano", "1pico", "1femto", "1tera", "1", "5V", "1ns", "1a", "1A",
            "1atto", "1wat", "NaN", "1k5", "", "1gig", "1mil", "10kHz",
        ] {
            let (Ok(interactive), Ok(deck)) = (
                parse_engineering_value(text),
                parse_spice_value_checked(text),
            ) else {
                continue;
            };
            let agrees = (interactive - deck).abs() <= deck.abs() * 1e-12;
            assert_eq!(
                agrees,
                !divergent.contains(&text),
                "{text}: the interactive parser reads {interactive}, this one reads {deck}"
            );
        }
    }
}
