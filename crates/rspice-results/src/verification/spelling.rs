//! How a verified number is spelled, wherever a run's verdict is stated: the
//! Verify views, the console's measurements, a report. One spelling, so the
//! same value never reads two ways on two surfaces.
//!
//! A value is four significant digits with an SI prefix where its unit takes
//! one; a limit is the bound its author wrote, without padding zeros; a
//! difference from a value, a margin or a change, sits at that value's
//! resolution in its notation.

use crate::number_format::{fmt_si_bound, fmt_si_significant, fmt_significant};
use crate::specification::SpecificationComparison;

/// Whether `unit` takes an SI prefix. A level in decibels, a percentage and
/// an angle in degrees do not: half a decibel is `0.5 dB`, never `500 mdB`.
fn takes_si_prefix(unit: &str) -> bool {
    !(unit.starts_with("dB") || matches!(unit, "%" | "deg" | "\u{b0}"))
}

/// `number` followed by `unit`, or alone when there is no unit.
fn with_unit(number: &str, unit: &str) -> String {
    if unit.is_empty() {
        number.to_owned()
    } else {
        format!("{number} {unit}")
    }
}

/// A measured value in engineering notation with its unit: four significant
/// digits, an SI prefix where the unit takes one, never a bare exponent.
#[must_use]
pub fn format_value(value: f64, unit: &str) -> String {
    if takes_si_prefix(unit) {
        fmt_si_significant(value, unit, 4)
    } else {
        with_unit(&fmt_significant(value, 4, ""), unit)
    }
}

/// One bound of a limit in the notation of the values held against it,
/// without the zeros nobody wrote it with: `40 dB`, `1.2 mA`.
#[must_use]
pub fn format_bound(value: f64, unit: &str) -> String {
    if takes_si_prefix(unit) {
        return fmt_si_bound(value, unit);
    }
    let text = fmt_significant(value, 4, "");
    // Scientific notation carries its own exponent, and trimming its mantissa
    // would change the number.
    let number = if text.contains('.') && !text.contains(['e', 'E']) {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text.as_str()
    };
    with_unit(number, unit)
}

/// A limit as a verdict states it: `≥ 40 dB`, `≤ 1.2 mA`, `10 MHz … 20 MHz`,
/// `50 Ω ± 1 Ω`; `None` for a tracked specification, which bounds nothing.
#[must_use]
pub fn limit_text(comparison: &SpecificationComparison, unit: &str) -> Option<String> {
    let bound = |value: f64| format_bound(value, unit);
    match *comparison {
        SpecificationComparison::Tracked => None,
        SpecificationComparison::Minimum { limit } => Some(format!("\u{2265} {}", bound(limit))),
        SpecificationComparison::Maximum { limit } => Some(format!("\u{2264} {}", bound(limit))),
        SpecificationComparison::Range { minimum, maximum } => {
            Some(format!("{} \u{2026} {}", bound(minimum), bound(maximum)))
        }
        SpecificationComparison::EqualWithin { target, tolerance } => {
            Some(format!("{} \u{00b1} {}", bound(target), bound(tolerance)))
        }
    }
}

/// `value` times ten to the `decades`, by an exact power of ten.
fn shifted(value: f64, decades: i32) -> f64 {
    let factor = 10f64.powi(decades.unsigned_abs().min(300) as i32);
    if decades >= 0 {
        value * factor
    } else {
        value / factor
    }
}

/// The decade of the last digit [`format_value`] prints of `value`: four
/// significant digits end three decades under the leading one, whatever
/// prefix the unit takes. Zero prints to three decimals.
fn last_digit_decade(value: f64) -> i32 {
    if value == 0.0 || !value.is_finite() {
        return -3;
    }
    value.abs().log10().floor() as i32 - 3
}

/// A difference taken from `of` — a margin to a limit, a change from a
/// reference — in the notation of the values beside it and at their
/// resolution: rounded to the last digit `of` is printed with, no more
/// significant digits than a value, and signed either way. A margin of
/// -0.6 dB from 39.40 dB reads `-0.60 dB`, and 40 µA from 1.160 mA `+40 µA`.
#[must_use]
pub fn difference_text(difference: f64, of: f64, unit: &str) -> String {
    let decade = last_digit_decade(of);
    let steps = shifted(difference, -decade).round();
    if !steps.is_finite() {
        return format_value(difference, unit);
    }
    if steps == 0.0 {
        // Nothing at the value's resolution: zero spelled as the value is,
        // on the side the difference falls. A negative zero is a failure at
        // the limit itself, and keeps its sign.
        let sign = if difference > 0.0 {
            "+"
        } else if difference.is_sign_negative() {
            "-"
        } else {
            ""
        };
        let shown = format_value(of, unit);
        let (number, rest) = shown.split_once(' ').unwrap_or((shown.as_str(), ""));
        let (mantissa, exponent) = number
            .find(['e', 'E'])
            .map_or((number, ""), |at| number.split_at(at));
        let decimals = mantissa
            .split_once('.')
            .map_or(0, |(_, digits)| digits.len());
        return with_unit(&format!("{sign}{:.decimals$}{exponent}", 0.0), rest);
    }
    let digits = (steps.abs().log10().floor() as usize + 1).min(4);
    let rounded = shifted(steps, decade);
    let sign = if rounded > 0.0 { "+" } else { "" };
    let text = if takes_si_prefix(unit) {
        fmt_si_significant(rounded, unit, digits)
    } else {
        with_unit(&fmt_significant(rounded, digits, ""), unit)
    };
    format!("{sign}{}", text.trim_end())
}
