//! An axis interval as text, and the text read back as an interval.
//!
//! One spelling serves the editable range field, the read-only viewport rows
//! above it and the status bar, so the three never state the same interval
//! two ways. It reads the way the axis ticks do — the shortest engineering
//! form with the axis unit, `0 … 1 ms`, `4 … 6 V`, `−0.105 … 1.415 V` — and
//! fits the details panel's field whole. Both ends share the larger end's
//! decade and are written to a ten-thousandth of the interval's width, which
//! is finer than a pixel of any plot: an autofit's padding leaves `f64` noise
//! such as `3.9559999999999995`, which reads `3.956`, and `-104.849 mV` under
//! `1.41546 V` reads `−0.105`. An end smaller than a hundredth of the other,
//! as on a log axis, keeps its own decade and four significant digits of
//! itself, so `1 kHz` never reads as zero under `1 GHz`. The field commits
//! only text the reader edited, so the rounding never moves an axis.
//!
//! The digits are the app's own engineering formatter and parser; this module
//! only chooses the precision and carries the unit.

use crate::quantity::engineering::{
    EngineeringPrecision, format_engineering_value_with, parse_engineering_value,
};
use rspice_results_ui::waves::unit_carries_its_decade;

/// Significant digits of the interval's width both ends are written to; an
/// end on its own decade keeps as many of the smaller of itself and the
/// width.
const SIGNIFICANT_DIGITS: i32 = 4;

/// The fewest significant digits a shorter spelling of an interval is
/// written to: one digit of the width could misstate it by half.
const MIN_SHORTER_DIGITS: i32 = 2;

/// How many times the larger end's magnitude the smaller may fall short of
/// and still be written on the larger's decade: `0.01` reads, `0.0001`
/// would bury the end's digits behind zeros.
const SHARED_DECADE_SPAN: f64 = 100.0;

/// The minus the axis ticks are drawn with, which the parser reads back.
const MINUS: char = '\u{2212}';

/// Most decimals any mantissa is written with.
const MAX_DECIMALS: i32 = 17;

/// `(minimum, maximum)` as the range field writes it.
///
/// The unit is written once, after the maximum, wherever the minimum would
/// carry nothing else — `0 … 1 ms`, `4 … 6 V`. A minimum on a decade of its
/// own keeps its prefix and the unit with it, `500 µs … 2 ms`, and so does a
/// unity minimum under a prefixed maximum, `2 V … 1.5 kV`, which would
/// otherwise read as kilovolts too.
pub(crate) fn format_axis_range(range: (f64, f64), unit: &str) -> String {
    written(range, unit, SIGNIFICANT_DIGITS)
}

/// `(minimum, maximum)` to fewer digits of the interval's width, finest
/// first, each one shorter than the last: what a field too narrow for
/// [`format_axis_range`]'s spelling shows at rest, `−1.76 … 1.76Meg` for
/// `−1.763 … 1.763Meg`, rather than cutting the interval through its own
/// ellipsis. The field still edits, and reads on hover, the full spelling.
pub(crate) fn shorter_axis_ranges(range: (f64, f64), unit: &str) -> Vec<String> {
    let mut shorter: Vec<String> = Vec::new();
    let mut last = format_axis_range(range, unit);
    for digits in (MIN_SHORTER_DIGITS..SIGNIFICANT_DIGITS).rev() {
        let text = written(range, unit, digits);
        if text.chars().count() < last.chars().count() {
            last.clone_from(&text);
            shorter.push(text);
        }
    }
    shorter
}

/// `(minimum, maximum)` to `digits` significant digits of the interval's
/// width.
fn written((minimum, maximum): (f64, f64), unit: &str, digits: i32) -> String {
    let width = (maximum - minimum).abs();
    if let Some(shared) = shared_decade_range((minimum, maximum), width, unit, digits) {
        return shared;
    }
    let low = AxisValue::of(minimum, width, unit, digits);
    let high = AxisValue::of(maximum, width, unit, digits);
    let low_text = if low.is_zero || (low.prefix.is_empty() && high.prefix.is_empty()) {
        low.mantissa.clone()
    } else if unit.is_empty() && low.prefix.is_empty() && !low.mantissa.contains('e') {
        // A bare minimum reads on the maximum's decade, so one on a decade of
        // its own states it: `1e2 … 100Meg`, never `100 … 100Meg`, which
        // reads back as 100Meg to 100Meg.
        low.mantissa
            .parse::<f64>()
            .map_or_else(|_| low.mantissa.clone(), |value| format!("{value:e}"))
    } else {
        low.text(unit)
    };
    format!("{low_text} \u{2026} {}", high.text(unit))
}

/// `(minimum, maximum)` on the larger end's decade, both ends to the width's
/// resolution: `−0.105 … 1.415 V`. `None` when the smaller end is too small
/// beside the larger to be written on its decade, or the interval has no
/// finite width.
fn shared_decade_range(
    (minimum, maximum): (f64, f64),
    width: f64,
    unit: &str,
    digits: i32,
) -> Option<String> {
    if !(width.is_finite() && width > 0.0 && minimum.is_finite() && maximum.is_finite()) {
        return None;
    }
    let (small, large) = {
        let (a, b) = (minimum.abs(), maximum.abs());
        (a.min(b), a.max(b))
    };
    if small != 0.0 && small * SHARED_DECADE_SPAN < large {
        return None;
    }
    // A decade is between -324 and 308, so each cast is exact.
    let step = width.log10().floor() as i32 - (digits - 1);
    let large_decade = large.log10().floor() as i32;
    // Past a prefix, or positional digits, the ends keep their exponents.
    let (decade, prefix) = if unit_carries_its_decade(unit) {
        if !(-6..15).contains(&large_decade) {
            return None;
        }
        (0, "")
    } else {
        if !(-15..15).contains(&large_decade) {
            return None;
        }
        let decade = large_decade.div_euclid(3) * 3;
        (decade, engineering_prefix(decade, unit))
    };
    let decimals = usize::try_from((decade - step).clamp(0, MAX_DECIMALS)).unwrap_or(0);
    let scale = 10f64.powi(decade);
    let write = |value: f64| {
        let digits = format!("{:.decimals$}", value / scale);
        let digits = if digits.contains('.') {
            digits.trim_end_matches('0').trim_end_matches('.')
        } else {
            digits.as_str()
        };
        match digits.strip_prefix('-') {
            Some(magnitude) if magnitude.bytes().any(|byte| matches!(byte, b'1'..=b'9')) => {
                format!("{MINUS}{magnitude}")
            }
            Some(magnitude) => magnitude.to_owned(),
            None => digits.to_owned(),
        }
    };
    let (low, high) = (write(minimum), write(maximum));
    Some(if unit.is_empty() {
        format!("{low} \u{2026} {high}{prefix}")
    } else {
        format!("{low} \u{2026} {high} {prefix}{unit}")
    })
}

/// The decade letter the axis ticks write for `decade`, a multiple of three
/// between femto and tera.
///
/// Before a unit the letters are SI's, as the ticks, the details pane and
/// Spectre write them: `1 MHz`, never `1 MegHz`. A bare number keeps SPICE's
/// `Meg`, because there a lone `M` is milli.
fn engineering_prefix(decade: i32, unit: &str) -> &'static str {
    match decade {
        12 => "T",
        9 => "G",
        6 if unit.is_empty() => "Meg",
        6 => "M",
        3 => "k",
        -3 => "m",
        -6 => "\u{b5}",
        -9 => "n",
        -12 => "p",
        -15 => "f",
        _ => "",
    }
}

/// Read an interval typed as `min … max`, in engineering notation, with or
/// without the axis unit.
///
/// Accepts the ellipsis [`format_axis_range`] writes, plus `..`, a comma or
/// plain space, so a value pasted from a datasheet or a netlist parses without
/// reformatting, and the typographic minus the axis ticks are drawn with. A
/// unit word after a number belongs to that number: `0 … 1 ms` is zero to one
/// millisecond. A bare minimum takes the maximum's decade, as the field
/// writes it and as a reader reads `0.5 … 2 ms`. A reversed or degenerate
/// interval is refused rather than silently sorted: on a log axis the two are
/// not the same request.
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
    let decade = decade_suffix(maximum, unit);
    let bare = minimum
        .chars()
        .all(|character| character.is_ascii_digit() || matches!(character, '.' | '-' | '+'));
    let minimum = if bare && !decade.is_empty() {
        parse_axis_value(&format!("{minimum}{decade}"), unit)?
    } else {
        parse_axis_value(minimum, unit)?
    };
    let maximum = parse_axis_value(maximum, unit)?;
    (minimum.is_finite() && maximum.is_finite() && maximum > minimum).then_some((minimum, maximum))
}

/// One end of an interval: its number, then its decade and the axis unit.
///
/// The unit is stripped before the engineering parser sees the text, so a
/// unit that is also a scale letter or not a word at all — `K`, `F`, `°`,
/// `nV/√Hz` — is read as the unit it is rather than as a decade or a refusal.
///
/// With a unit the decade letters are SI's, so an `M` before it is mega, as
/// the field writes `1 MHz` and as Spectre reads it; `m` stays milli. A bare
/// number keeps the SPICE reading.
fn parse_axis_value(text: &str, unit: &str) -> Option<f64> {
    let text = text.trim();
    if unit.is_empty() {
        return parse_engineering_value(text).ok();
    }
    let number = text.strip_suffix(unit).unwrap_or(text).trim_end();
    match number.strip_suffix('M') {
        Some(mantissa) if !mantissa.ends_with(char::is_alphabetic) => {
            parse_engineering_value(&format!("{}Meg", mantissa.trim_end())).ok()
        }
        _ => parse_engineering_value(number).ok(),
    }
}

/// The decade letters `text` ends with once its unit is stripped: `m` of
/// `2 ms`, nothing of `1.415 V` or `4e-21`.
fn decade_suffix<'a>(text: &'a str, unit: &str) -> &'a str {
    let text = text.trim();
    let number = if unit.is_empty() {
        text
    } else {
        text.strip_suffix(unit).unwrap_or(text)
    }
    .trim_end();
    let digits = number.trim_end_matches(|character: char| character.is_alphabetic());
    &number[digits.len()..]
}

/// One end of an interval, as written.
struct AxisValue {
    /// The digits, signed: `1.5`, `-250`, `3.3333333333333335e-20`.
    mantissa: String,
    /// The decade the mantissa sits on, as the reader sees it: `k`, `µ`, `M`
    /// before a unit (`Meg` on a bare number), or nothing.
    prefix: String,
    is_zero: bool,
}

impl AxisValue {
    fn of(value: f64, width: f64, unit: &str, digits: i32) -> Self {
        let step = step_exponent(value, width, digits);
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
                // The parser folds `µ` back to `u`. Before a unit `Meg` is
                // SI's `M`; on a bare number it stays, because a lone `M` is
                // milli to every SPICE reader.
                prefix: match &spice[mantissa.len()..] {
                    "u" => "\u{b5}".to_owned(),
                    "Meg" if !unit.is_empty() => "M".to_owned(),
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

/// The power of ten an end is rounded to: `digits` significant digits of the
/// smaller of its magnitude and the interval's width. The width alone would
/// round a log axis's `1 kHz` end to zero under a `1 GHz` one; the magnitude
/// alone would write a narrow zoom's two ends identically.
fn step_exponent(value: f64, width: f64, digits: i32) -> i32 {
    let scale = [value.abs(), width]
        .into_iter()
        .filter(|magnitude| magnitude.is_finite() && *magnitude > 0.0)
        .reduce(f64::min);
    // An `f64` decade lies between -324 and 308, so each cast is exact.
    scale.map_or(0, |scale| scale.log10().floor() as i32 - (digits - 1))
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
    use super::{format_axis_range, parse_axis_range, shorter_axis_ranges};

    /// A pole-zero plane's range read `−1.763 ……763Meg` in a field a few
    /// pixels too narrow: the field cut the interval through its own
    /// ellipsis. The interval offers shorter spellings instead, to fewer
    /// digits, each shorter than the last and each read back as nearly the
    /// same interval.
    #[test]
    fn a_range_too_wide_for_its_field_offers_shorter_spellings() {
        let range = (-1.763e6, 1.763e6);
        assert_eq!(format_axis_range(range, ""), "−1.763 … 1.763Meg");
        assert_eq!(
            shorter_axis_ranges(range, ""),
            ["−1.76 … 1.76Meg", "−1.8 … 1.8Meg"]
        );
        // Nothing is shorter than what already reads whole.
        assert!(shorter_axis_ranges((4.0, 6.0), "V").is_empty());
        let shorter = shorter_axis_ranges((0.0, 1.234_5e-3), "s");
        assert_eq!(shorter.len(), 2);
        for text in shorter {
            let (low, high) = parse_axis_range(&text, "s").expect("reads back");
            assert!(low == 0.0 && (high - 1.234_5e-3).abs() < 0.05e-3, "{text}");
        }
    }

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
            "0.5 \u{2026} 2 ms"
        );
        assert_eq!(format_axis_range((2.0, 1500.0), "V"), "2 V \u{2026} 1.5 kV");
        assert_eq!(
            format_axis_range((-2.5, 2.5), "A"),
            "\u{2212}2.5 \u{2026} 2.5 A"
        );
        assert_eq!(
            format_axis_range((1.0e3, 1.0e9), "Hz"),
            "1 kHz \u{2026} 1 GHz"
        );
        assert_eq!(format_axis_range((0.0, 5.0), ""), "0 \u{2026} 5");
        // A unit that already names its decade takes no second one.
        assert_eq!(
            format_axis_range((0.0, 1786.0), "nV/\u{221a}Hz"),
            "0 \u{2026} 1786 nV/\u{221a}Hz"
        );
    }

    /// James saw an STB sweep's range read `1 Hz … 1 MegHz` in the status
    /// bar. Before a unit the decade letters are SI's, written and read back.
    #[test]
    fn a_megahertz_range_reads_as_si_and_parses_back() {
        assert_eq!(format_axis_range((1.0, 1.0e6), "Hz"), "1 Hz \u{2026} 1 MHz");
        assert_eq!(
            format_axis_range((5.0e5, 2.0e6), "Hz"),
            "0.5 \u{2026} 2 MHz"
        );
        assert_eq!(
            parse_axis_range("1 Hz \u{2026} 1 MHz", "Hz"),
            Some((1.0, 1.0e6))
        );
        assert_eq!(
            parse_axis_range("0.5 \u{2026} 2 MHz", "Hz"),
            Some((5.0e5, 2.0e6))
        );
        assert_eq!(
            parse_axis_range("1 mHz \u{2026} 1 Hz", "Hz"),
            Some((1.0e-3, 1.0))
        );
        assert_eq!(
            parse_axis_range("1 \u{2026} 2Meg", "Hz"),
            Some((1.0e6, 2.0e6))
        );
        // A bare number keeps SPICE's spelling, where a lone M is milli.
        assert_eq!(format_axis_range((0.0, 2.0e6), ""), "0 \u{2026} 2Meg");
    }

    /// A noise sheet's field showed its own `100 … 100Meg` in red: the
    /// parser gives a bare minimum the maximum's decade, so the field read
    /// its own text back as 100Meg to 100Meg. A minimum on a decade of its
    /// own is never written bare.
    #[test]
    fn a_minimum_on_its_own_decade_is_never_written_bare() {
        let decades = (100.0, 100.0e6);
        assert_eq!(format_axis_range(decades, "Hz"), "100 Hz \u{2026} 100 MHz");
        assert_eq!(
            parse_axis_range(&format_axis_range(decades, "Hz"), "Hz"),
            Some(decades)
        );
        assert_eq!(format_axis_range(decades, ""), "1e2 \u{2026} 100Meg");
        assert_eq!(
            parse_axis_range(&format_axis_range(decades, ""), ""),
            Some(decades)
        );
        // On the maximum's decade it stays bare, and reads on that decade.
        assert_eq!(format_axis_range((5.0e5, 2.0e6), ""), "0.5 \u{2026} 2Meg");
        for range in [(1.0, 1.0e3), (2.5, 1.0e9), (-150.0, 2.0e4), (999.0, 1.0e7)] {
            let text = format_axis_range(range, "");
            let (low, high) = parse_axis_range(&text, "").unwrap_or_else(|| {
                panic!("{range:?} was written {text:?}, which reads as nothing")
            });
            assert!(
                (low - range.0).abs() <= 1.0e-3 * range.0.abs()
                    && (high - range.1).abs() <= 1.0e-3 * range.1.abs(),
                "{range:?} was written {text:?} and read back as ({low}, {high})"
            );
        }
    }

    /// An autofit pads the data by a fraction of its span, and the sum is
    /// `f64` noise. The ends read as the numbers they are near, never as
    /// seventeen digits, and a narrow zoom keeps the digits that tell its
    /// ends apart.
    #[test]
    fn a_range_hides_floating_point_noise_but_keeps_the_digits_of_its_width() {
        assert_eq!(
            format_axis_range((3.955_999_999_999_999_5, 6.044_000_000_000_000_5), "V"),
            "3.956 \u{2026} 6.044 V"
        );
        assert_eq!(
            format_axis_range((0.1 + 0.2, 0.9), "A"),
            "300 \u{2026} 900 mA"
        );
        assert_eq!(
            format_axis_range((1.234_5e-3, 1.234_6e-3), "s"),
            "1.2345 \u{2026} 1.2346 ms"
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
                    let larger = range.0.abs().max(range.1.abs());
                    for (written, read) in [(range.0, parsed.0), (range.1, parsed.1)] {
                        // To a ten-thousandth of the width; an end on its own
                        // decade to four digits of the smaller of itself and
                        // the width as well.
                        let own_decade = written.abs() * 100.0 < larger;
                        let tolerance = if own_decade {
                            written.abs().min(width)
                        } else {
                            width
                        } * 1.0e-3;
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
        // A bare minimum is on the maximum's decade, as the field writes it.
        assert_eq!(parse_axis_range("0.5 … 2 ms", "s"), Some((0.5e-3, 2.0e-3)));
        assert_eq!(
            parse_axis_range("\u{2212}0.105 \u{2026} 1.415 V", "V"),
            Some((-0.105, 1.415))
        );
        assert_eq!(parse_axis_range("2 V … 1.5 kV", "V"), Some((2.0, 1500.0)));
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
