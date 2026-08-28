//! SI-prefixed number formatting for axes, readouts and cursor blocks.

/// The engineering ladder, tera down to atto. Beyond either end a value is
/// printed under the last prefix rather than in a notation the rest of the
/// instrument does not use.
const SI_MAX: i32 = 4;
const SI_MIN: i32 = -6;

const SI_LADDER: [(f64, i32); 11] = [
    (1.0e12, 4),
    (1.0e9, 3),
    (1.0e6, 2),
    (1.0e3, 1),
    (1.0, 0),
    (1.0e-3, -1),
    (1.0e-6, -2),
    (1.0e-9, -3),
    (1.0e-12, -4),
    (1.0e-15, -5),
    (1.0e-18, -6),
];

fn si_prefix(index: i32) -> &'static str {
    match index {
        4 => "T",
        3 => "G",
        2 => "M",
        1 => "k",
        0 => "",
        -1 => "m",
        -2 => "µ",
        -3 => "n",
        -4 => "p",
        -5 => "f",
        _ => "a",
    }
}

/// `value` expressed under the prefix `index` names.
///
/// Positive indices divide and negative ones multiply, so the factor is
/// always an exactly representable power of ten and the mantissa carries no
/// rounding the value did not already have.
fn si_scaled(value: f64, index: i32) -> f64 {
    let factor = 10f64.powi(3 * index.abs());
    if index >= 0 {
        value / factor
    } else {
        value * factor
    }
}

/// Engineering ladder index: mantissa in 1‥1000.
fn si_index(magnitude: f64) -> i32 {
    SI_LADDER
        .iter()
        .find(|(floor, _)| magnitude >= *floor)
        .map_or(SI_MIN, |(_, index)| *index)
}

/// Tick ladder index. Human-scale values (0.1 V, 0.8 V, 240) stay plain
/// decimals like the design's voltage axes; prefixes start below 0.1 and at
/// 1000.
fn tick_index(magnitude: f64) -> i32 {
    if (0.1..1.0e3).contains(&magnitude) {
        0
    } else {
        si_index(magnitude)
    }
}

/// Whether printing `scaled` to `decimals` places would carry it out of its
/// own prefix. 999.9999 k is 1 M, and printing it as "1000k" reads as a
/// decade error on the axis.
fn overflows_prefix(scaled: f64, decimals: usize) -> bool {
    let factor = 10f64.powi(decimals.min(15) as i32);
    (scaled.abs() * factor).round() / factor >= 1.0e3
}

/// One ladder placement: the index to print under, promoted when rounding
/// would carry the mantissa past 1000.
fn placed(magnitude: f64, index: i32, decimals: usize) -> i32 {
    if index < SI_MAX && overflows_prefix(si_scaled(magnitude, index), decimals) {
        index + 1
    } else {
        index
    }
}

/// Trim the trailing zeros a fixed-decimal rendering leaves behind, and use
/// the typographic minus the design draws on plots.
fn trimmed(mut text: String) -> String {
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    text.replace('-', "−")
}

/// Format `value` with an SI prefix and unit: `fmt_si(2.0e-6, "s", 3)` →
/// `"2.000 µs"`. Zero renders without a prefix.
pub fn fmt_si(value: f64, unit: &str, digits: usize) -> String {
    let (scaled, prefix) = scale_si(value, digits);
    format!("{scaled:.digits$} {prefix}{unit}")
        .trim_end()
        .to_owned()
}

/// Format a result readout with an exact number of significant digits while
/// retaining SI-prefix presentation. This is intentionally separate from
/// engineering export, which serializes the original `f64` samples.
pub fn fmt_si_significant(value: f64, unit: &str, significant_digits: usize) -> String {
    let magnitude = value.abs();
    let mut index = if magnitude == 0.0 || !magnitude.is_finite() {
        0
    } else {
        si_index(magnitude)
    };
    // The mantissa decides its own decimal count, so the promotion has to be
    // judged against the digits that mantissa will print.
    let decimals = significant_decimals(si_scaled(value, index), significant_digits);
    index = placed(magnitude, index, decimals);
    format_with_significant_digits(
        si_scaled(value, index),
        significant_digits,
        &format!(" {}{unit}", si_prefix(index)),
    )
    .trim_end()
    .to_owned()
}

/// Format a unitless or fixed-unit value with an exact number of significant
/// digits. `suffix` should include its own leading space when desired.
pub fn fmt_significant(value: f64, significant_digits: usize, suffix: &str) -> String {
    format_with_significant_digits(value, significant_digits, suffix)
}

fn scale_si(value: f64, decimals: usize) -> (f64, &'static str) {
    let magnitude = value.abs();
    if magnitude == 0.0 || !magnitude.is_finite() {
        return (value, "");
    }
    let index = placed(magnitude, si_index(magnitude), decimals);
    (si_scaled(value, index), si_prefix(index))
}

fn significant_decimals(value: f64, significant_digits: usize) -> usize {
    let significant_digits = significant_digits.max(1);
    if !value.is_finite() || value == 0.0 {
        return significant_digits - 1;
    }
    let exponent = value.abs().log10().floor() as i32;
    (significant_digits as i32 - 1 - exponent).max(0) as usize
}

fn format_with_significant_digits(value: f64, significant_digits: usize, suffix: &str) -> String {
    let significant_digits = significant_digits.max(1);
    if !value.is_finite() {
        return format!("{value}{suffix}");
    }
    if value == 0.0 {
        return format!(
            "{value:.precision$}{suffix}",
            precision = significant_digits - 1
        );
    }
    let decimals = significant_decimals(value, significant_digits);
    // Extremely small values below the supported SI prefix range would
    // otherwise create hundreds of zeroes. Scientific notation is exact,
    // bounded, and still honors the significant-digit contract.
    if decimals > 24 {
        return format!(
            "{value:.precision$e}{suffix}",
            precision = significant_digits - 1
        );
    }
    format!("{value:.decimals$}{suffix}")
}

/// Short tick label for an axis value: SI-prefixed, no unit, trailing zeros
/// trimmed (`200_000.0` → `"200k"`, `0.4` → `"0.4"`).
///
/// Three decimals is the fallback for a value with no tick spacing to derive
/// its precision from — a readout, an axis end, a delta. An axis with a step
/// uses [`tick_label_with_step`], which is the only way adjacent ticks are
/// guaranteed to read differently.
pub fn tick_label(value: f64) -> String {
    tick_label_with_decimals(value, 3)
}

/// Tick label for an axis whose ticks are `step` apart.
///
/// The decimal count comes from the step, not from a constant: at 1000:1 zoom
/// a fixed three decimals printed every tick on the axis as the same string,
/// which is an axis that has stopped saying anything. Enough places to
/// resolve one step is exactly enough for two neighbours to differ, and
/// trailing zeros are trimmed so an ordinary axis is unchanged.
pub fn tick_label_with_step(value: f64, step: f64) -> String {
    if !(step.is_finite() && step > 0.0) {
        return tick_label(value);
    }
    if value == 0.0 {
        return "0".to_owned();
    }
    let magnitude = value.abs();
    let mut index = tick_index(magnitude);
    let mut decimals = step_decimals(si_scaled(step, index));
    let promoted = placed(magnitude, index, decimals);
    if promoted != index {
        index = promoted;
        decimals = step_decimals(si_scaled(step, index));
    }
    trimmed(format!("{:.*}", decimals, si_scaled(value, index))) + si_prefix(index)
}

/// A tick stated as a signed offset from an axis anchor.
pub fn tick_offset_label(delta: f64, step: f64) -> String {
    if delta == 0.0 {
        return "0".to_owned();
    }
    let sign = if delta < 0.0 { "−" } else { "+" };
    format!("{sign}{}", tick_label_with_step(delta.abs(), step))
}

/// The anchor an offset axis states once, beside its ticks: the value every
/// tick label is an offset from, with its unit.
pub fn offset_anchor_label(anchor: f64, unit: &str, step: f64) -> String {
    let sign = if anchor < 0.0 { "−" } else { "+" };
    let magnitude = anchor.abs();
    let index = if magnitude == 0.0 || !magnitude.is_finite() {
        0
    } else {
        let start = tick_index(magnitude);
        placed(magnitude, start, step_decimals(si_scaled(step, start)))
    };
    let decimals = step_decimals(si_scaled(step, index));
    let mantissa = trimmed(format!("{:.*}", decimals, si_scaled(magnitude, index)));
    format!("{sign}{mantissa} {}{unit}", si_prefix(index))
        .trim_end()
        .to_owned()
}

/// Decimal places needed to resolve one step. A step of 0.25 needs one
/// place, 2e-4 needs four, and 20 needs none.
fn step_decimals(scaled_step: f64) -> usize {
    let step = scaled_step.abs();
    // The finiteness test leads so it is the one that catches NaN: the pair
    // rejects exactly what it rejected when the first half was written `!(step
    // > 0.0)`, which is every step that is not a positive real number.
    if !step.is_finite() || step <= 0.0 {
        return 3;
    }
    (-step.log10()).ceil().clamp(0.0, 15.0) as usize
}

/// Whether an axis this narrow around this centre can label its ticks
/// absolutely at all.
///
/// Past this ratio every tick label shares a leading run of digits with every
/// other, and the reader is left comparing the last two characters of eight.
/// The instrument states the common part once instead.
pub const OFFSET_NOTATION_RATIO: f64 = 1.0e-4;

/// Whether `[min, max]` should be labeled as offsets from a stated anchor.
pub fn wants_offset_notation(min: f64, max: f64) -> bool {
    let span = max - min;
    let centre = (min + max) * 0.5;
    span.is_finite()
        && span > 0.0
        && centre.is_finite()
        && centre != 0.0
        && span / centre.abs() < OFFSET_NOTATION_RATIO
}

fn tick_label_with_decimals(value: f64, decimals: usize) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    if !value.is_finite() {
        return format!("{value}").replace('-', "−");
    }
    let magnitude = value.abs();
    let index = placed(magnitude, tick_index(magnitude), decimals);
    trimmed(format!("{:.*}", decimals, si_scaled(value, index))) + si_prefix(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A step with no positive magnitude falls back to three decimals.
    ///
    /// The guard reads `!step.is_finite() || step <= 0.0`, which is the same
    /// set the earlier `!(step > 0.0) || !step.is_finite()` rejected — the
    /// finiteness half is what catches NaN either way. This pins that: a bare
    /// `step <= 0.0` on its own would let NaN through to
    /// `(-NaN.log10()).ceil().clamp(..)`, whose `as usize` cast is 0, and
    /// every tick on the axis would lose its decimals at once.
    ///
    /// The test is over magnitudes because the function is: it takes `.abs()`
    /// before the guard, so a negative step is an ordinary step read
    /// backwards, not a rejected one.
    #[test]
    fn a_step_with_no_positive_magnitude_keeps_the_default_decimals() {
        for step in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0] {
            assert_eq!(
                step_decimals(step),
                3,
                "a step of {step} did not fall back to the default"
            );
        }
        // The ordinary path is untouched, in either direction.
        assert_eq!(step_decimals(0.25), 1);
        assert_eq!(step_decimals(-0.25), 1);
        assert_eq!(step_decimals(20.0), 0);
    }

    #[test]
    fn si_formatting() {
        assert_eq!(fmt_si(2.0e-6, "s", 3), "2.000 µs");
        assert_eq!(fmt_si(10.4e6, "Hz", 1), "10.4 MHz");
        assert_eq!(fmt_si(0.0, "V", 2), "0.00 V");
    }

    #[test]
    fn tick_labels_trim() {
        assert_eq!(tick_label(200_000.0), "200k");
        assert_eq!(tick_label(0.4), "0.4");
        assert_eq!(tick_label(-20.0), "−20");
        assert_eq!(tick_label(0.0), "0");
        assert_eq!(tick_label(1.0e9), "1G");
    }

    #[test]
    fn the_si_ladder_reaches_tera_and_atto_in_both_directions() {
        assert_eq!(tick_label(1.0e12), "1T");
        assert_eq!(tick_label(2.5e12), "2.5T");
        assert_eq!(tick_label(1.0e-15), "1f");
        assert_eq!(tick_label(4.7e-18), "4.7a");
        assert_eq!(fmt_si(3.3e12, "Hz", 1), "3.3 THz");
        assert_eq!(fmt_si(2.0e-18, "F", 2), "2.00 aF");
    }

    #[test]
    fn a_mantissa_that_rounds_up_promotes_its_prefix() {
        // 999.9999 k is 1 M. Printing it under the smaller prefix reads as a
        // decade error on the axis.
        assert_eq!(tick_label(999_999.9), "1M");
        assert_eq!(tick_label(0.999_999_5), "1");
        assert_eq!(tick_label(999.999_9e-6), "1m");
        assert_eq!(fmt_si(999_999.9, "Hz", 3), "1.000 MHz");
        assert_eq!(fmt_si_significant(999.999_99, "V", 4), "1.0000 kV");
    }

    #[test]
    fn tick_precision_follows_the_step_so_neighbours_differ() {
        assert_eq!(tick_label_with_step(1.000_2, 2.0e-4), "1.0002");
        assert_eq!(tick_label_with_step(1.000_4, 2.0e-4), "1.0004");
        // An ordinary axis is unchanged: the extra places are trimmed.
        assert_eq!(tick_label_with_step(0.4, 0.2), "0.4");
        assert_eq!(tick_label_with_step(200_000.0, 50_000.0), "200k");
        assert_eq!(tick_label_with_step(0.0, 0.25), "0");
        // No step to derive from falls back to the fixed rendering.
        assert_eq!(tick_label_with_step(0.4, 0.0), "0.4");
    }

    #[test]
    fn an_offset_axis_states_its_anchor_once_and_its_ticks_as_deltas() {
        assert_eq!(offset_anchor_label(1.0e-3, "s", 2.0e-8), "+1 ms");
        assert_eq!(offset_anchor_label(-2.5e-6, "s", 1.0e-11), "−2.5 µs");
        assert_eq!(offset_anchor_label(1.000_03e-3, "s", 1.0e-8), "+1.00003 ms");
        assert_eq!(tick_offset_label(2.0e-8, 2.0e-8), "+20n");
        assert_eq!(tick_offset_label(-4.0e-8, 2.0e-8), "−40n");
        assert_eq!(tick_offset_label(0.0, 2.0e-8), "0");
    }

    #[test]
    fn offset_notation_engages_only_where_absolute_labels_stop_working() {
        assert!(!wants_offset_notation(0.0, 1.0));
        assert!(!wants_offset_notation(0.999, 1.001));
        assert!(wants_offset_notation(1.0 - 1.0e-5, 1.0 + 1.0e-5));
        assert!(wants_offset_notation(1.0e-3, 1.0e-3 + 1.0e-9));
        // A window straddling zero has no common part to state.
        assert!(!wants_offset_notation(-1.0, 1.0));
    }

    #[test]
    fn significant_readouts_are_distinct_from_fixed_decimal_formatting() {
        assert_eq!(fmt_si_significant(12.345_678, "V", 7), "12.34568 V");
        assert_eq!(fmt_si_significant(1.234_567_8e-6, "s", 7), "1.234568 µs");
        assert_eq!(fmt_significant(-179.999_9, 7, " °"), "-179.9999 °");
        assert_eq!(fmt_si_significant(0.0, "A", 3), "0.00 A");
    }
}
