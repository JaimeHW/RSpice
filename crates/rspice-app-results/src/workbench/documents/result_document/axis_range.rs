//! An axis interval as text, and the text read back as an interval.
//!
//! One spelling serves the editable range field, the read-only viewport rows
//! above it and the status bar, so the three never state the same interval
//! two ways. It reads the way the axis ticks do — the shortest engineering
//! form with the axis unit, `0 … 1 ms`, `4 … 6 V`. Each end is written to six
//! significant digits of the smaller of itself and the interval's width: an
//! autofit's padding leaves `f64` noise such as `3.9559999999999995`, which
//! reads `3.956`, while a typed `1786.13` still reads as typed. The field
//! commits only text the reader edited, so the rounding never moves an axis.
//!
//! The digits are the app's own engineering formatter and parser; this module
//! only chooses the precision and carries the unit.

use crate::quantity::engineering::{
    EngineeringPrecision, format_engineering_value_with, parse_engineering_value,
};
use rspice_results_ui::waves::unit_carries_its_decade;

/// Significant digits each end keeps of the smaller of its own magnitude and
/// the interval's width.
const SIGNIFICANT_DIGITS: i32 = 6;

/// Most decimals any mantissa is written with.
const MAX_DECIMALS: i32 = 17;

/// `(minimum, maximum)` as the range field writes it.
///
/// The unit is written once, after the maximum, wherever the minimum would
/// carry nothing else — `0 … 1 ms`, `4 … 6 V`. A minimum on a decade of its
/// own keeps its prefix and the unit with it, `500 µs … 2 ms`, and so does a
/// unity minimum under a prefixed maximum, `2 V … 1.5 kV`, which would
/// otherwise read as kilovolts too.
pub(crate) fn format_axis_range((minimum, maximum): (f64, f64), unit: &str) -> String {
    let width = (maximum - minimum).abs();
    let low = AxisValue::of(minimum, width, unit);
    let high = AxisValue::of(maximum, width, unit);
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
    fn of(value: f64, width: f64, unit: &str) -> Self {
        let step = step_exponent(value, width);
        let rounded = (value / 10f64.powi(step)).round() * 10f64.powi(step);
        // A signed zero, or an end that rounds to zero, is written as zero.
        let value = if rounded == 0.0 || !rounded.is_finite() {
            if value.is_finite() { 0.0 } else { value }
        } else {
            rounded
        };
        if value.is_finite() && !unit_carries_its_decade(unit) {
            let decimals = (decade_of(value) - step).clamp(0, MAX_DECIMALS);
            let spice = format_engineering_value_with(
                value,
                EngineeringPrecision::UpTo(u8::try_from(decimals).unwrap_or(0)),
            );
            let mantissa = spice.trim_end_matches(char::is_alphabetic);
            return Self {
                mantissa: mantissa.to_owned(),
                // The parser folds `µ` back to `u`; `Meg` stays as it is,
                // because a bare `M` is milli to every SPICE reader.
                prefix: match &spice[mantissa.len()..] {
                    "u" => "\u{b5}".to_owned(),
                    prefix => prefix.to_owned(),
                },
                is_zero: value == 0.0,
            };
        }
        Self {
            mantissa: plain_text(value, step),
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

/// `value` without a decade, to the power of ten `step`: positional where that
/// reads, `1786.13`, and with an exponent outside it, `4e-21`.
fn plain_text(value: f64, step: i32) -> String {
    let trim = |digits: &str| -> String {
        if digits.contains('.') {
            digits
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_owned()
        } else {
            digits.to_owned()
        }
    };
    let decade = if value == 0.0 {
        0
    } else {
        value.abs().log10().floor() as i32
    };
    if (-6..15).contains(&decade) && -step <= MAX_DECIMALS {
        let decimals = usize::try_from(-step).unwrap_or(0);
        return trim(&format!("{value:.decimals$}"));
    }
    let digits = usize::try_from((decade - step).clamp(0, MAX_DECIMALS)).unwrap_or(0);
    let text = format!("{value:.digits$e}");
    match text.split_once('e') {
        Some((mantissa, exponent)) => format!("{}e{exponent}", trim(mantissa)),
        None => text,
    }
}

/// The power of ten an end is rounded to: [`SIGNIFICANT_DIGITS`] of the
/// smaller of its magnitude and the interval's width. The width alone would
/// round a log axis's `1 kHz` end to zero under a `1 GHz` one; the magnitude
/// alone would write a narrow zoom's two ends identically.
fn step_exponent(value: f64, width: f64) -> i32 {
    let scale = [value.abs(), width]
        .into_iter()
        .filter(|magnitude| magnitude.is_finite() && *magnitude > 0.0)
        .reduce(f64::min);
    // An `f64` decade lies between -324 and 308, so each cast is exact.
    scale.map_or(0, |scale| {
        scale.log10().floor() as i32 - (SIGNIFICANT_DIGITS - 1)
    })
}

/// The decade the engineering formatter writes `value`'s mantissa on: `k` is
/// 3, `m` is -3, and below femto the formatter writes a plain exponent.
fn decade_of(value: f64) -> i32 {
    let magnitude = value.abs();
    if magnitude == 0.0 || !magnitude.is_finite() {
        return 0;
    }
    let decade = magnitude.log10().floor() as i32;
    (decade.div_euclid(3) * 3).clamp(-15, 12)
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
        assert_eq!(
            format_axis_range((5.0e-4, 2.0e-3), "s"),
            "500 \u{b5}s \u{2026} 2 ms"
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

    /// An autofit pads the data by a fraction of its span, and the sum is
    /// `f64` noise. The ends read as the numbers they are near, never as
    /// seventeen digits, and a typed end still reads as typed.
    #[test]
    fn a_range_hides_floating_point_noise_but_keeps_typed_digits() {
        assert_eq!(
            format_axis_range((3.955_999_999_999_999_5, 6.044_000_000_000_000_5), "V"),
            "3.956 \u{2026} 6.044 V"
        );
        assert_eq!(
            format_axis_range((0.1 + 0.2, 0.9), "A"),
            "300 mA \u{2026} 900 mA"
        );
        assert_eq!(
            format_axis_range((0.0, 1786.13), "nV/\u{221a}Hz"),
            "0 \u{2026} 1786.13 nV/\u{221a}Hz"
        );
        assert_eq!(
            format_axis_range((1.234_5e-3, 1.234_6e-3), "s"),
            "1.2345 ms \u{2026} 1.2346 ms"
        );
    }

    /// What the field shows reads back through its own parser to within the
    /// precision it was written to, whatever the value and the unit.
    #[test]
    fn a_range_reads_back_to_within_its_written_precision() {
        let awkward: [f64; 8] = [
            1.0 / 3.0,
            1.0e-15,
            123_456.789,
            2.0 / 3.0 * 1.0e-3,
            0.1 + 0.2,
            -1.0e-15,
            6.022_140_76e23,
            4.0e-21,
        ];
        for unit in ["s", "V", "Hz", "", "K", "\u{b0}", "nV/\u{221a}Hz"] {
            for value in awkward {
                for range in [(value, value.abs() * 4.0 + 1.0), (-1.0e30, value)] {
                    let text = format_axis_range(range, unit);
                    let parsed = parse_axis_range(&text, unit)
                        .unwrap_or_else(|| panic!("{text:?} did not read back in {unit:?}"));
                    let width = (range.1 - range.0).abs();
                    for (written, read) in [(range.0, parsed.0), (range.1, parsed.1)] {
                        let tolerance = written.abs().min(width) * 1.0e-5;
                        assert!(
                            (read - written).abs() <= tolerance,
                            "{range:?} in {unit:?} was written {text:?} and read back as {parsed:?}"
                        );
                    }
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
