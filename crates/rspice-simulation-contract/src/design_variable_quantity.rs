//! The quantity a design variable is labelled with, and the units a value
//! of that label may be written with.

use serde::{Deserialize, Serialize};

/// The physical quantity a design variable is labelled with. It is a label: a
/// value may carry the label's unit or none, and the unit never refuses a
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignVariableQuantity {
    Resistance,
    Capacitance,
    Voltage,
    Current,
    Temperature,
    Dimensionless,
}

impl DesignVariableQuantity {
    pub const ALL: [Self; 6] = [
        Self::Resistance,
        Self::Capacitance,
        Self::Voltage,
        Self::Current,
        Self::Temperature,
        Self::Dimensionless,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Resistance => "Resistance",
            Self::Capacitance => "Capacitance",
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Temperature => "Temperature",
            Self::Dimensionless => "Dimensionless",
        }
    }
}

impl DesignVariableQuantity {
    /// The units a value of this label may be written with. `ohm` is read in
    /// any case; a one-letter unit only as SI spells it, because a lower-case
    /// letter after a number is SPICE's scale factor (`10f` is ten femto).
    const fn units(self) -> &'static [&'static str] {
        match self {
            Self::Resistance => &["ohm", "\u{3a9}"],
            Self::Capacitance => &["F"],
            Self::Voltage => &["V"],
            Self::Current => &["A"],
            Self::Temperature => &["\u{b0}C"],
            Self::Dimensionless => &[],
        }
    }
}

/// `text` as the engine reads it for a variable labelled `quantity`, when it
/// is one number written with a unit; `None` when it is not, and the text is
/// then the engine's to read as it stands.
///
/// The label's unit is read off, with or without a space before it, and the
/// scale prefix in front of it is kept as the engine spells it: `1.8 V` is
/// `1.8`, `100 nF` is `100n`, `10 kohm` is `10k`, and `1 M\u{3a9}` is `1Meg`,
/// because a prefix in front of a unit is SI's, where `M` is mega. Any other
/// word after a space is read as the engine reads a suffix written against
/// the number (`1.8 V` on a variable with no label is `1.8V`, which is 1.8).
/// A number with no unit at all is left alone: `1.8m` is 1.8 milli, as SPICE
/// reads it.
pub fn number_written_with_unit(text: &str, quantity: DesignVariableQuantity) -> Option<String> {
    let text = text.trim();
    for unit in quantity.units() {
        let Some(scaled) = strip_unit(text, unit) else {
            continue;
        };
        let (number, prefix) = split_number(scaled.trim_end())?;
        let prefix = if prefix == "M" { "Meg" } else { prefix };
        return Some(format!("{number}{prefix}"));
    }
    let (number, word) = text.split_once(char::is_whitespace)?;
    let word = word.trim();
    let reads_as_number = split_number(number).is_some_and(|(_, prefix)| prefix.is_empty());
    (reads_as_number && !word.is_empty() && word.chars().all(char::is_alphabetic))
        .then(|| format!("{number}{word}"))
}

/// `text` without `unit` at its end, or `None` when it does not end in it.
fn strip_unit<'a>(text: &'a str, unit: &str) -> Option<&'a str> {
    if unit.len() > 1 && unit.is_ascii() {
        let split = text.len().checked_sub(unit.len())?;
        text.get(split..)
            .filter(|tail| tail.eq_ignore_ascii_case(unit))
            .map(|_| &text[..split])
    } else {
        text.strip_suffix(unit)
    }
}

/// `text` as a decimal number and the scale prefix after it (letters only,
/// a space between the two allowed), or `None` when it is not one.
fn split_number(text: &str) -> Option<(&str, &str)> {
    let bytes = text.as_bytes();
    let mut end = 0;
    if matches!(bytes.first(), Some(b'+' | b'-')) {
        end += 1;
    }
    let digits_start = end;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    if bytes.get(end) == Some(&b'.') {
        end += 1;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
    }
    if !text[digits_start..end]
        .bytes()
        .any(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let mut exponent = end + 1;
        if matches!(bytes.get(exponent), Some(b'+' | b'-')) {
            exponent += 1;
        }
        if bytes.get(exponent).is_some_and(u8::is_ascii_digit) {
            while bytes.get(exponent).is_some_and(u8::is_ascii_digit) {
                exponent += 1;
            }
            end = exponent;
        }
    }
    let prefix = text[end..].trim_start();
    prefix
        .chars()
        .all(char::is_alphabetic)
        .then_some((&text[..end], prefix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_variable::DesignVariable;
    use crate::design_variable_resolution::resolve_design_variables;

    fn resolved(text: &str, quantity: DesignVariableQuantity) -> f64 {
        let variable = DesignVariable::new("X", text, quantity, "", None, true)
            .unwrap_or_else(|error| panic!("{text} is refused: {error}"));
        *resolve_design_variables(&[variable])
            .value(0)
            .unwrap()
            .as_ref()
            .unwrap_or_else(|error| panic!("{text} does not resolve: {error}"))
    }

    fn close(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() <= expected.abs() * 1e-12
    }

    /// A voltage needs no unit, takes one when it is typed, and a scale
    /// factor without a unit reads as SPICE reads it.
    #[test]
    fn a_plain_number_needs_no_unit() {
        use DesignVariableQuantity::{Capacitance, Dimensionless, Resistance, Voltage};
        for text in ["1.8", "1.8V", "1.8 V"] {
            assert_eq!(resolved(text, Voltage), 1.8, "{text}");
        }
        assert!(close(resolved("1.8m", Voltage), 0.0018));
        assert!(close(resolved("1.8 mV", Voltage), 0.0018));
        assert_eq!(resolved("10 kohm", Resistance), 10e3);
        assert_eq!(resolved("1 M\u{3a9}", Resistance), 1e6);
        assert!(close(resolved("100 nF", Capacitance), 100e-9));
        assert!(close(resolved("1e-12F", Capacitance), 1e-12));
        assert_eq!(resolved("1.8 V", Dimensionless), 1.8);
    }
}
