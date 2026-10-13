//! An axis interval as text, and the text read back as an interval.
//!
//! One spelling serves the editable range field, the read-only viewport rows
//! above it and the status bar, so the three never state the same interval
//! two ways. It reads the way the axis ticks do — the shortest engineering
//! form with the axis unit, `0 … 1 ms`, `4 … 6 V` — and it is exact: the text
//! goes back through [`parse_axis_range`] to the very `f64`s it was written
//! from, bit for bit. Where no engineering form is exact, the shortest plain
//! decimal that is takes its place. A rounded value is never written, because
//! the field would commit it the moment the reader tabbed through.
//!
//! The digits are the app's own engineering formatter and parser; this module
//! only chooses between the forms they produce and carries the unit.

use crate::quantity::engineering::{
    EngineeringPrecision, format_engineering_value_with, parse_engineering_value,
};
use rspice_results_ui::waves::unit_carries_its_decade;

/// Most decimals the engineering formatter is asked for. Seventeen
/// significant digits name any `f64`, and a mantissa below 1000 never needs
/// more decimals than that.
const MAX_DECIMALS: u8 = 17;

/// `(minimum, maximum)` as the range field writes it.
///
/// The unit is written once, after the maximum, wherever the minimum would
/// carry nothing else — `0 … 1 ms`, `4 … 6 V`. A minimum on a decade of its
/// own keeps its prefix and the unit with it, `500 µs … 2 ms`, and so does a
/// unity minimum under a prefixed maximum, `2 V … 1.5 kV`, which would
/// otherwise read as kilovolts too.
pub(crate) fn format_axis_range((minimum, maximum): (f64, f64), unit: &str) -> String {
    let low = AxisValue::of(minimum, unit);
    let high = AxisValue::of(maximum, unit);
    let low_text = if low.is_zero || (low.prefix.is_empty() && high.prefix.is_empty()) {
        low.mantissa.clone()
    } else {
        low.text(unit)
    };
    format!("{low_text} \u{2026} {}", high.text(unit))
}

/// Read an interval typed as `min … max`, in engineering notation, with or
/// without the axis unit.
///
/// Accepts the ellipsis [`format_axis_range`] writes, plus `..`, a comma or
/// plain space, so a value pasted from a datasheet or a netlist parses without
/// reformatting, and the typographic minus the axis ticks are drawn with. A
/// unit word after a number belongs to that number: `0 … 1 ms` is zero to one
/// millisecond. A reversed or degenerate interval is refused rather than
/// silently sorted: on a log axis the two are not the same request.
pub(crate) fn parse_axis_range(text: &str, unit: &str) -> Option<(f64, f64)> {
    let cleaned = text
        .replace('\u{2212}', "-")
        .replace('\u{2026}', " ")
        .replace("..", " ")
        .replace(',', " ");
    let mut values: Vec<String> = Vec::with_capacity(2);
    for token in cleaned.split_whitespace() {
        let starts_a_number = token
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_digit() || matches!(first, '-' | '+' | '.'));
        match values.last_mut() {
            Some(previous) if !starts_a_number => {
                previous.push(' ');
                previous.push_str(token);
            }
            _ => values.push(token.to_owned()),
        }
    }
    let [minimum, maximum] = values.as_slice() else {
        return None;
    };
    let minimum = parse_axis_value(minimum, unit)?;
    let maximum = parse_axis_value(maximum, unit)?;
    (minimum.is_finite() && maximum.is_finite() && maximum > minimum).then_some((minimum, maximum))
}

/// One end of an interval: its number, then its decade and the axis unit.
///
/// The unit is stripped before the engineering parser sees the text, so a
/// unit that is also a scale letter or not a word at all — `K`, `F`, `°`,
/// `nV/√Hz` — is read as the unit it is rather than as a decade or a refusal.
fn parse_axis_value(text: &str, unit: &str) -> Option<f64> {
    let text = text.trim();
    let number = if unit.is_empty() {
        text
    } else {
        text.strip_suffix(unit).unwrap_or(text)
    };
    parse_engineering_value(number).ok()
}

/// One end of an interval, as written.
struct AxisValue {
    /// The digits, signed: `1.5`, `-250`, `3.3333333333333335e-20`.
    mantissa: String,
    /// The decade the mantissa sits on, as the reader sees it: `k`, `µ`, `Meg`,
    /// or nothing.
    prefix: String,
    is_zero: bool,
}

impl AxisValue {
    fn of(value: f64, unit: &str) -> Self {
        // A signed zero is written, and so reads back, as zero.
        let value = if value == 0.0 { 0.0 } else { value };
        let plain = shortest_plain(value);
        let exact = |text: &str| {
            parse_axis_value(text, unit).is_some_and(|parsed| parsed.to_bits() == value.to_bits())
        };
        if value.is_finite() && !unit_carries_its_decade(unit) {
            let budget = significant_digits(&plain);
            for decimals in 0..=MAX_DECIMALS {
                let spice =
                    format_engineering_value_with(value, EngineeringPrecision::UpTo(decimals));
                let mantissa = spice.trim_end_matches(char::is_alphabetic);
                // More digits than the plain form needs is the scaling's own
                // rounding showing through, not more of the value.
                if significant_digits(mantissa) > budget {
                    break;
                }
                let candidate = Self {
                    mantissa: mantissa.to_owned(),
                    // The parser folds `µ` back to `u`; `Meg` stays as it is,
                    // because a bare `M` is milli to every SPICE reader.
                    prefix: match &spice[mantissa.len()..] {
                        "u" => "\u{b5}".to_owned(),
                        prefix => prefix.to_owned(),
                    },
                    is_zero: value == 0.0,
                };
                if exact(&candidate.text(unit)) {
                    return candidate;
                }
            }
        }
        Self {
            mantissa: plain,
            prefix: String::new(),
            is_zero: value == 0.0,
        }
    }

    fn text(&self, unit: &str) -> String {
        if unit.is_empty() {
            format!("{}{}", self.mantissa, self.prefix)
        } else {
            format!("{} {}{unit}", self.mantissa, self.prefix)
        }
    }
}

/// The shortest decimal that reads back as `value`: Rust's own shortest
/// round-trip forms, positional or with an exponent, whichever is shorter.
fn shortest_plain(value: f64) -> String {
    let positional = format!("{value}");
    let exponent = format!("{value:e}");
    if exponent.len() < positional.len() {
        exponent
    } else {
        positional
    }
}

/// How many significant digits a mantissa writes: its digits before any
/// exponent, less the zeros that only place the point.
fn significant_digits(mantissa: &str) -> usize {
    let digits: String = mantissa
        .split(['e', 'E'])
        .next()
        .unwrap_or_default()
        .chars()
        .filter(char::is_ascii_digit)
        .collect();
    digits.trim_start_matches('0').len().max(1)
}

#[cfg(test)]
mod tests {
    use super::{format_axis_range, parse_axis_range};

    /// What James saw was `0 … 0.0010000` and `4.0000000 … 6.000`: seven
    /// padding zeros that say nothing and cost the field its width. The range
    /// reads the way the axis ticks read it.
    #[test]
    fn a_range_reads_like_the_ticks_of_its_axis() {
        assert_eq!(format_axis_range((0.0, 1.0e-3), "s"), "0 \u{2026} 1 ms");
        assert_eq!(format_axis_range((4.0, 6.0), "V"), "4 \u{2026} 6 V");
        assert_eq!(
            format_axis_range((1.0e-6, 2.0e-3), "s"),
            "1 \u{b5}s \u{2026} 2 ms"
        );
        // The field's parser reads `500 µ` as 500 × 1e-6, which is not the
        // `f64` nearest 5e-4, so the engineering form of 5e-4 would commit a
        // different number than the one the axis holds. The shortest exact
        // form stands in for it.
        assert_eq!(
            format_axis_range((5.0e-4, 2.0e-3), "s"),
            "5e-4 s \u{2026} 2 ms"
        );
        assert_eq!(format_axis_range((2.0, 1500.0), "V"), "2 V \u{2026} 1.5 kV");
        assert_eq!(format_axis_range((-2.5, 2.5), "A"), "-2.5 \u{2026} 2.5 A");
        assert_eq!(
            format_axis_range((1.0e3, 1.0e9), "Hz"),
            "1 kHz \u{2026} 1 GHz"
        );
        assert_eq!(format_axis_range((0.0, 5.0), ""), "0 \u{2026} 5");
        // A unit that already names its decade takes no second one.
        assert_eq!(
            format_axis_range((0.0, 1786.13), "nV/\u{221a}Hz"),
            "0 \u{2026} 1786.13 nV/\u{221a}Hz"
        );
    }

    /// The field commits what it shows, so what it shows has to be the value
    /// itself. Each of these needs more than the short form, and none of them
    /// may come back as a neighbouring `f64`.
    #[test]
    fn a_range_reads_back_as_the_exact_values_it_was_written_from() {
        let awkward = [
            1.0 / 3.0,
            1.0e-15,
            123_456.789,
            2.0 / 3.0 * 1.0e-3,
            0.1 + 0.2,
            -1.0e-15,
            6.02214076e23,
            4.0e-21,
            1.0e-3 + f64::EPSILON * 1.0e-3,
        ];
        for unit in ["s", "V", "Hz", "", "K", "\u{b0}", "nV/\u{221a}Hz"] {
            for value in awkward {
                for range in [(value, value.abs() * 4.0 + 1.0), (-1.0e30, value)] {
                    let text = format_axis_range(range, unit);
                    let parsed = parse_axis_range(&text, unit)
                        .unwrap_or_else(|| panic!("{text:?} did not read back in {unit:?}"));
                    assert_eq!(
                        (parsed.0.to_bits(), parsed.1.to_bits()),
                        (range.0.to_bits(), range.1.to_bits()),
                        "{range:?} in {unit:?} was written {text:?} and read back as {parsed:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn an_interval_parses_from_the_separators_an_engineer_would_paste() {
        for text in [
            "1m … 5m",
            "1m..5m",
            "1m,5m",
            "1m 5m",
            "  1m   5m  ",
            "1 ms … 5 ms",
        ] {
            assert_eq!(
                parse_axis_range(text, "s"),
                Some((1.0e-3, 5.0e-3)),
                "failed on {text:?}"
            );
        }
        assert_eq!(parse_axis_range("-2.5 -1.5", ""), Some((-2.5, -1.5)));
        // The axis ticks are drawn with a typographic minus.
        assert_eq!(parse_axis_range("\u{2212}2 … 1 V", "V"), Some((-2.0, 1.0)));
        // A unit that is a scale letter to SPICE is still the unit here.
        assert_eq!(parse_axis_range("300 … 400 K", "K"), Some((300.0, 400.0)));
    }

    #[test]
    fn a_reversed_or_degenerate_interval_is_refused_rather_than_sorted() {
        // On a log axis "5m … 1m" is not the same request as "1m … 5m", and
        // quietly swapping them would hide the typo behind a correct-looking
        // plot.
        assert_eq!(parse_axis_range("5m … 1m", ""), None);
        assert_eq!(parse_axis_range("1m … 1m", ""), None);
        assert_eq!(parse_axis_range("1m", ""), None);
        assert_eq!(parse_axis_range("1m … 2m … 3m", ""), None);
        assert_eq!(parse_axis_range("", ""), None);
        assert_eq!(parse_axis_range("wide open", ""), None);
    }
}
