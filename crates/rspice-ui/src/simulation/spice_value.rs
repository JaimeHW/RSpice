//! Canonical parser for scalar SPICE values shared by plan and execution code.
//!
//! Every field this reads is compiled into a deck, so a suffix has to mean
//! here exactly what it means there. The scale comes from
//! [`rspice_core::netlist::lexer::spice_suffix_scale`] — the engine's own
//! table, not a second copy of it — because a second copy is how `1A` came to
//! be one ampere to the interactive parser and 1e-18 seconds to this one.
//!
//! One owner has two consequences. This parser now accepts what a deck
//! accepts: unit letters after the scale (`1ns`, `10kHz`, `2.2uF`), bare units
//! (`5V`, `1A`, `2.5s`), `mil`, and the Xyce `X`. And it refuses what a deck
//! refuses, which includes the spelled-out scale words no deck reader has ever
//! had: `1micro` is a rejection here rather than the 1e-3 the engine would
//! actually read out of it, and `1milli` rather than 25.4e-6 for `MIL`.

use rspice_core::netlist::lexer::spice_suffix_scale;

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

    // The engine's reader consumes the suffix it knows and stops. What it
    // leaves behind is not a unit this boundary may quietly ignore, because a
    // deck would tokenize the remainder separately: `1wat` has to stay a
    // rejection rather than becoming a bare `1`.
    let (multiplier, consumed) = spice_suffix_scale(suffix);
    let consumed_bytes: usize = suffix.chars().take(consumed).map(char::len_utf8).sum();
    if consumed_bytes != suffix.len() {
        return Err(format!("unsupported SPICE suffix '{}'", suffix));
    }

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

    /// Every spelling the two numeric surfaces of a simulation field are asked
    /// to read: the ones a deck carries, the ones a UI convenience added, and
    /// the ones that have to stay rejections.
    const CORPUS: &[&str] = &[
        "1e-6", "10n", "1k", "3.3meg", "2.2u", "1m", "1t", "1g", "1p", "1f", "-4.7k", "1kilo",
        "1micro", "1milli", "1nano", "1pico", "1femto", "1tera", "1", "5V", "1ns", "1a", "1A",
        "1atto", "1wat", "NaN", "1k5", "", "1gig", "1mil", "10kHz", "2.2uF", "1MHz",
    ];

    /// This parser and the interactive
    /// [`crate::quantity::parse_engineering_value`] read the same fields of
    /// the same design — a stop time, a source amplitude — so where both
    /// accept a spelling they have to agree on the number.
    ///
    /// Atto is gone from both, so `1a`, `1A` and `1atto` no longer disagree:
    /// with unit letters after a scale factor accepted, `A` is amperes, and a
    /// transient stop time typed `1A` is one second on both paths rather than
    /// 1e-18 on this one.
    ///
    /// One divergence is left and is deliberate. `1MHz` is megahertz to the
    /// engine's deck lexer, which has the three-letter `MHZ`, and one
    /// milli-something to the interactive parser, which reads `m` as milli and
    /// ignores `Hz` as the unit. That is ngspice against this dialect, not a
    /// second table: this parser reads it through the engine's own, which is
    /// the point — the field means what the run will mean.
    ///
    /// The word forms (`1micro`, `1kilo`, …) no longer parse here at all. No
    /// deck reader has them, and the engine reads `micro` as milli and `milli`
    /// as mil, so accepting them would be accepting a value the run would not
    /// honour. They are refused rather than silently rescaled, which is why
    /// they reach neither side of this comparison.
    ///
    /// The test lives here rather than beside the other parser because
    /// `quantity` is the crate's bottom layer and may not name `simulation`.
    #[test]
    fn the_interactive_parser_agrees_wherever_both_accept_a_spelling() {
        use crate::quantity::parse_engineering_value;

        let divergent = ["1MHz"];
        for text in CORPUS {
            let (Ok(interactive), Ok(deck)) = (
                parse_engineering_value(text),
                parse_spice_value_checked(text),
            ) else {
                continue;
            };
            let agrees = (interactive - deck).abs() <= deck.abs() * 1e-12;
            assert_eq!(
                agrees,
                !divergent.contains(text),
                "{text}: the interactive parser reads {interactive}, this one reads {deck}"
            );
        }
    }

    /// The pin behind the one-owner rule: there is no arithmetic between this
    /// parser and the engine's, so every accepted spelling lands on the same
    /// bits the engine would land on reading the same token out of a deck.
    ///
    /// Both directions are checked. The suffix reader is the table this
    /// parser calls; `parse_spice_value_complete` is the whole number path the
    /// deck goes through, and agreeing with it is what makes a typed field and
    /// the netlist it is compiled into the same number.
    #[test]
    fn every_accepted_spelling_is_the_number_the_engine_reads_from_a_deck() {
        use rspice_core::netlist::lexer::{parse_spice_value_complete, spice_suffix_scale};

        let mut accepted = 0;
        for text in CORPUS {
            let Ok(ours) = parse_spice_value_checked(text) else {
                continue;
            };
            accepted += 1;

            let trimmed = text.trim();
            let split = trimmed
                .char_indices()
                .find(|&(_, c)| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+' | 'e' | 'E')))
                .map_or(trimmed.len(), |(index, _)| index);
            let (mantissa, suffix) = trimmed.split_at(split);
            let (multiplier, _) = spice_suffix_scale(suffix);
            let through_core_suffix =
                mantissa.parse::<f64>().expect("a plain mantissa") * multiplier;
            assert_eq!(
                ours.to_bits(),
                through_core_suffix.to_bits(),
                "{text}: this parser reads {ours:e}, the core suffix table reads {through_core_suffix:e}"
            );

            let through_core_lexer =
                parse_spice_value_complete(trimmed).expect("the engine reads what this parser did");
            assert_eq!(
                ours.to_bits(),
                through_core_lexer.to_bits(),
                "{text}: this parser reads {ours:e}, a deck reads {through_core_lexer:e}"
            );
        }
        assert!(
            accepted >= 18,
            "the corpus stopped exercising the parser: only {accepted} spellings parsed"
        );
    }
}
