//! Numbers and names inside a Spectre-language value.
//!
//! Spectre's scale factors are case-sensitive and differ from SPICE's in
//! meaning: `M` is mega where SPICE reads milli, and `%`, `c`, `_` and `a`
//! have no SPICE spelling at all. A value is copied to the canonical card as
//! written, so every numeric literal in it is rewritten here to a spelling the
//! canonical reader takes to the exact decimal value the Spectre literal
//! denotes. Factors that mean the same in both dialects keep their spelling,
//! which the canonical reader already scales exactly (see
//! `rspice_core_base::spice_number::scaled_spice_value`); the others move
//! the literal's decimal exponent, so no rounding is introduced either way.
//!
//! Anything else after a number is refused by name: a SPICE suffix such as
//! `meg` or `mil` would be read as a different value, and a unit word or a
//! second letter is not Spectre-language syntax.

use std::borrow::Cow;

use super::{SpectreModelAdapterError, error};

/// The Spectre-language scale factors, case-sensitive, with the decimal
/// exponent each one stands for.
pub(super) const SPECTRE_SCALE_FACTORS: &[(&str, i32)] = &[
    ("T", 12),
    ("G", 9),
    ("M", 6),
    ("K", 3),
    ("k", 3),
    ("_", 0),
    ("%", -2),
    ("c", -2),
    ("m", -3),
    ("u", -6),
    ("n", -9),
    ("p", -12),
    ("f", -15),
    ("a", -18),
];

/// The factors the canonical reader scales by the same power of ten, so the
/// literal keeps its spelling. (`K` and `k` both read as kilo there, and its
/// single-letter suffixes are case-insensitive.)
const SHARED_SPELLINGS: &[&str] = &["T", "G", "K", "k", "m", "u", "n", "p", "f"];

/// Rewrite every numeric literal of a Spectre-language `value` to its exact
/// canonical spelling. Quoted text is copied untouched: a quoted Spectre value
/// is a string, and the one quoted expression the adapter lowers (a
/// `bsource` equation) is passed through here again once its quotes are off.
pub(super) fn lower_spectre_value(
    value: &str,
    parameter: &str,
    line: usize,
) -> Result<String, SpectreModelAdapterError> {
    let bytes = value.as_bytes();
    let mut lowered = String::with_capacity(value.len() + 4);
    let mut index = 0usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if matches!(byte, b'\'' | b'"') {
            let end = quoted_end(bytes, index);
            lowered.push_str(&value[index..end]);
            index = end;
        } else if is_identifier_start(byte) {
            let end = access_end(value, index, identifier_end(bytes, index));
            lowered.push_str(&value[index..end]);
            index = end;
        } else if starts_number(bytes, index) {
            let literal = SpectreNumber::scan(value, index);
            lowered.push_str(&literal.lowered(parameter, line)?);
            index = literal.end;
        } else {
            let width = value[index..].chars().next().map_or(1, char::len_utf8);
            lowered.push_str(&value[index..index + width]);
            index += width;
        }
    }
    Ok(lowered)
}

/// A lowered value as one element-card token. Spectre writes an expression
/// bare (`r=rr*gg`), while an element card reads a bare value as one number
/// or name and an expression only inside braces, so any value that is more
/// than a single number or name, and is not already delimited, is braced.
pub(super) fn card_value(value: &str) -> Cow<'_, str> {
    let delimited = [("{", "}"), ("[", "]"), ("\"", "\""), ("'", "'")]
        .iter()
        .any(|(open, close)| value.len() > 1 && value.starts_with(open) && value.ends_with(close));
    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    let bytes = unsigned.as_bytes();
    let single = !bytes.is_empty()
        && if is_identifier_start(bytes[0]) {
            identifier_end(bytes, 0) == bytes.len()
        } else {
            starts_number(bytes, 0) && SpectreNumber::scan(unsigned, 0).end == bytes.len()
        };
    if delimited || single {
        Cow::Borrowed(value)
    } else {
        Cow::Owned(format!("{{{value}}}"))
    }
}

/// The names a Spectre-language value refers to: every identifier outside a
/// quoted string that is not called as a function. Numbers and their scale
/// factors are skipped.
pub(super) fn spectre_value_identifiers(value: &str) -> Vec<&str> {
    let bytes = value.as_bytes();
    let mut identifiers = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if matches!(byte, b'\'' | b'"') {
            index = quoted_end(bytes, index);
        } else if is_identifier_start(byte) {
            let end = identifier_end(bytes, index);
            let called = value[end..].trim_start().starts_with('(');
            if !called {
                identifiers.push(&value[index..end]);
            }
            index = access_end(value, index, end);
        } else if starts_number(bytes, index) {
            index = SpectreNumber::scan(value, index).end;
        } else {
            index += value[index..].chars().next().map_or(1, char::len_utf8);
        }
    }
    identifiers
}

/// One numeric literal: its mantissa, its decimal exponent, and whatever
/// letters or marks follow it.
struct SpectreNumber<'a> {
    literal: &'a str,
    mantissa: &'a str,
    exponent: Option<&'a str>,
    tail: &'a str,
    end: usize,
}

impl<'a> SpectreNumber<'a> {
    fn scan(value: &'a str, start: usize) -> Self {
        let bytes = value.as_bytes();
        let mut index = start;
        while bytes.get(index).is_some_and(u8::is_ascii_digit) {
            index += 1;
        }
        if bytes.get(index) == Some(&b'.') {
            index += 1;
            while bytes.get(index).is_some_and(u8::is_ascii_digit) {
                index += 1;
            }
        }
        let mantissa_end = index;
        let mut exponent = None;
        if matches!(bytes.get(index), Some(b'e' | b'E')) {
            let mut digits = index + 1;
            if matches!(bytes.get(digits), Some(b'+' | b'-')) {
                digits += 1;
            }
            if bytes.get(digits).is_some_and(u8::is_ascii_digit) {
                let mut end = digits;
                while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                    end += 1;
                }
                exponent = Some(&value[index + 1..end]);
                index = end;
            }
        }
        let tail_start = index;
        while bytes
            .get(index)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'%'))
        {
            index += 1;
        }
        Self {
            literal: &value[start..index],
            mantissa: &value[start..mantissa_end],
            exponent,
            tail: &value[tail_start..index],
            end: index,
        }
    }

    fn lowered(&self, parameter: &str, line: usize) -> Result<String, SpectreModelAdapterError> {
        if self.tail.is_empty() || SHARED_SPELLINGS.contains(&self.tail) {
            return Ok(self.literal.to_owned());
        }
        let Some(&(_, decade)) = SPECTRE_SCALE_FACTORS
            .iter()
            .find(|(factor, _)| *factor == self.tail)
        else {
            return Err(self.refusal(parameter, line));
        };
        // The literal's own exponent fits an i64 whenever the canonical reader
        // could read it at all; a longer one is refused rather than wrapped.
        let written = match self.exponent {
            Some(exponent) => exponent
                .parse::<i64>()
                .map_err(|_| self.refusal(parameter, line))?,
            None => 0,
        };
        let exponent = written + i64::from(decade);
        if exponent == 0 && self.exponent.is_none() {
            Ok(self.mantissa.to_owned())
        } else {
            Ok(format!("{}e{exponent}", self.mantissa))
        }
    }

    fn refusal(&self, parameter: &str, line: usize) -> SpectreModelAdapterError {
        let factors = SPECTRE_SCALE_FACTORS
            .iter()
            .map(|(factor, _)| *factor)
            .collect::<Vec<_>>()
            .join(" ");
        let number = match self.exponent {
            Some(exponent) => format!("{}e{exponent}", self.mantissa),
            None => self.mantissa.to_owned(),
        };
        let fix = if self.tail.eq_ignore_ascii_case("meg") {
            format!("Spectre reads 'M' as mega, so write '{number}M'")
        } else {
            format!(
                "write the number with one of those factors, or with an exponent, and no unit (for example '{number}')"
            )
        };
        error(
            line,
            format!(
                "Spectre number '{}' in '{parameter}' ends in '{}', which is not a Spectre scale factor; a Spectre-language number takes at most one case-sensitive scale factor ({factors}), so {fix}",
                self.literal, self.tail
            ),
        )
    }
}

/// Where the identifier at `start..end` ends together with its argument list
/// when it is the access function `V` or `I`. Their arguments name nodes and
/// branches, never numbers or parameters, so `V(2M)` reads node `2M`.
fn access_end(value: &str, start: usize, end: usize) -> usize {
    if !matches!(&value[start..end], "V" | "I") {
        return end;
    }
    let rest = &value[end..];
    let Some(arguments) = rest.trim_start().strip_prefix('(') else {
        return end;
    };
    let open = value.len() - arguments.len();
    arguments
        .find(')')
        .map_or(value.len(), |close| open + close + 1)
}

fn starts_number(bytes: &[u8], index: usize) -> bool {
    bytes[index].is_ascii_digit()
        || (bytes[index] == b'.' && bytes.get(index + 1).is_some_and(u8::is_ascii_digit))
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$')
}

fn identifier_end(bytes: &[u8], start: usize) -> usize {
    let mut index = start;
    while bytes
        .get(index)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$' | b'.'))
    {
        index += 1;
    }
    index
}

/// The index just past the string opening at `start`, or the end of the
/// value when it never closes (the value scanner has already refused that).
fn quoted_end(bytes: &[u8], start: usize) -> usize {
    let quote = bytes[start];
    let mut index = start + 1;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        index += 1;
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == quote {
            return index;
        }
    }
    bytes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::lexer::parse_spice_value_complete;

    fn lowered(value: &str) -> String {
        lower_spectre_value(value, "r", 1).expect("value lowers")
    }

    /// Every Spectre factor, written as a literal whose exact decimal value is
    /// spelled out beside it. The canonical reader must take the lowered
    /// spelling to the same `f64`, bit for bit, as the decimal itself.
    #[test]
    fn every_scale_factor_lowers_to_its_exact_decimal_value() {
        for (literal, decimal) in [
            ("1.5T", "1.5e12"),
            ("2.2G", "2.2e9"),
            ("1M", "1e6"),
            ("4.7M", "4.7e6"),
            ("4.7K", "4.7e3"),
            ("3.3k", "3.3e3"),
            ("47_", "47"),
            ("250%", "2.5"),
            ("7%", "0.07"),
            ("680c", "6.8"),
            ("820m", "0.82"),
            ("10u", "1e-5"),
            ("1500u", "1.5e-3"),
            ("2.7n", "2.7e-9"),
            ("3.9p", "3.9e-12"),
            ("5.6f", "5.6e-15"),
            ("6.8a", "6.8e-18"),
            (".5M", "5e5"),
            ("1e3M", "1e9"),
            ("2.5e-1%", "2.5e-3"),
            ("-1M", "-1e6"),
        ] {
            let text = lowered(literal);
            let canonical = parse_spice_value_complete(text.trim_start_matches('-'))
                .unwrap_or_else(|error| panic!("'{literal}' lowered to '{text}': {error}"));
            let canonical = if text.starts_with('-') {
                -canonical
            } else {
                canonical
            };
            let exact = decimal.parse::<f64>().expect("decimal parses");
            assert_eq!(
                canonical.to_bits(),
                exact.to_bits(),
                "'{literal}' lowered to '{text}', read as {canonical:e}, not {exact:e}"
            );
        }
    }

    #[test]
    fn literals_whose_meaning_agrees_keep_their_spelling() {
        for literal in [
            "1k", "4.7K", "2.2G", "1.5T", "1m", "10u", "2n", "3p", "5f", "1e-3", "42",
        ] {
            assert_eq!(lowered(literal), literal);
        }
    }

    #[test]
    fn only_numbers_are_rewritten_inside_expressions() {
        assert_eq!(lowered("rr*gg"), "rr*gg");
        assert_eq!(lowered("(rr*1M+x1M)/2%"), "(rr*1e6+x1M)/2e-2");
        assert_eq!(lowered("[0 0 1n 1M]"), "[0 0 1n 1e6]");
        assert_eq!(lowered("\"1M.txt\""), "\"1M.txt\"");
        assert_eq!(lowered("2**3"), "2**3");
        assert_eq!(lowered("1_"), "1");
        assert_eq!(lowered("V(2M, 1a)*1M + I(v1)"), "V(2M, 1a)*1e6 + I(v1)");
    }

    #[test]
    fn suffixes_spectre_does_not_define_are_refused_by_name() {
        for (literal, tail) in [
            ("1meg", "meg"),
            ("1MEG", "MEG"),
            ("1mil", "mil"),
            ("1ns", "ns"),
            ("1kohm", "kohm"),
            ("2U", "U"),
            ("1F", "F"),
            ("3e", "e"),
            ("1M2", "M2"),
        ] {
            let message = lower_spectre_value(literal, "r", 7)
                .expect_err("undefined suffix is refused")
                .to_string();
            assert!(
                message.starts_with("line 7: ")
                    && message.contains(&format!("'{literal}' in 'r' ends in '{tail}'")),
                "{message}"
            );
        }
        let meg = lower_spectre_value("1meg", "r", 1).unwrap_err().to_string();
        assert!(meg.contains("so write '1M'"), "{meg}");
    }

    #[test]
    fn element_card_values_brace_expressions_only() {
        for (value, card) in [
            ("1e6", "1e6"),
            ("-1k", "-1k"),
            ("rr", "rr"),
            ("rr*gg", "{rr*gg}"),
            ("(rr)", "{(rr)}"),
            ("-rr*2", "{-rr*2}"),
            ("{rr*gg}", "{rr*gg}"),
            ("[0 1]", "[0 1]"),
            ("\"text\"", "\"text\""),
        ] {
            assert_eq!(card_value(value), card, "{value}");
        }
    }

    #[test]
    fn identifiers_skip_numbers_strings_and_calls() {
        assert_eq!(
            spectre_value_identifiers("rr*gg + exp(x1) - 1M + \"name\" + V(a, b)"),
            ["rr", "gg", "x1"]
        );
    }

    /// Every place a Spectre value reaches a card takes the Spectre factors,
    /// and a `simulator lang=spice` region in the same file keeps SPICE's.
    #[test]
    fn every_spectre_value_position_lowers_its_scale_factors() {
        let source = [
            "simulator lang=spectre",
            "parameters rr=1M gg=2",
            "model dm diode is=1e-14 rs=1M",
            "subckt load (a) rv=1M",
            "parameters rw=2M",
            "  r1 (a 0) resistor r=rv+rw",
            "ends load",
            "x1 (n1) load rv=3M",
            "r2 (b 0) resistor r=rr*gg",
            "v1 (b 0) vsource dc=1 type=sine ampl=1 freq=1M",
            "b1 (c 0) bsource v=\"V(b)*1M\"",
            "ac1 ac start=1 stop=1M dec=10",
            "tran1 tran stop=1M",
            "simulator lang=spice",
            "R9 d 0 1M",
            "R10 e 0 1MEG",
            "simulator lang=spectre",
            "r3 (f 0) resistor r=1M",
        ]
        .join("\n");
        let adapted =
            super::super::adapt_spectre_model_library(std::path::Path::new("deck.scs"), &source)
                .expect("deck lowers");
        let lines = adapted.lines().collect::<Vec<_>>();
        for (line, expected) in [
            (2, ".param rr=1e6 gg=2"),
            (3, ".model dm D ( is=1e-14 rs=1e6 )"),
            (4, ".subckt load a PARAMS: rv=1e6"),
            (5, ".param rw=2e6"),
            (8, "Xx1 n1 load rv=3e6"),
            (9, "Rr2 b 0 {rr*gg}"),
            (10, "Vv1 b 0 SIN(1 1 1e6 0 0 0)"),
            (11, "Bb1 c 0 V={V(b)*1e6}"),
            (12, ".AC DEC 10 1 1e6"),
            (13, ".TRAN 0 1e6"),
            (15, "R9 d 0 1M"),
            (16, "R10 e 0 1MEG"),
            (18, "Rr3 f 0 1e6"),
        ] {
            assert_eq!(lines[line - 1], expected, "line {line} of\n{adapted}");
        }
    }

    #[test]
    fn a_spice_suffix_in_a_spectre_region_is_refused_at_its_line() {
        let error = super::super::adapt_spectre_model_library(
            std::path::Path::new("deck.scs"),
            "simulator lang=spectre\nv1 (a 0) vsource dc=1\nr1 (a 0) resistor r=1meg\n",
        )
        .expect_err("1meg is not a Spectre number");
        assert_eq!(error.line, 3);
        assert!(
            error.message.contains("'1meg' in 'r' ends in 'meg'")
                && error.message.contains("so write '1M'"),
            "{error}"
        );
    }
}
