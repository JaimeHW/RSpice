//! SI-prefixed number formatting for axes, readouts and cursor blocks.

/// The engineering ladder, yotta down to yocto: the scale-factor suffixes
/// Cadence's analog expression language prints (`aelSuffixNotation`, Y to y).
///
/// Past either end the mantissa stays in 1‥1000 and the decade is stated as
/// an exponent on the number, `24.03e-27 V`, as that language leaves the `e`
/// and exponent in place once its suffixes run out. Piling the value onto the
/// last prefix instead (`0.02402879 yV`) prints a mantissa no engineering
/// readout ever shows and hides the decade in a run of leading zeros.
const SI_MAX: i32 = 8;
const SI_MIN: i32 = -8;

const SI_LADDER: [(f64, i32); 17] = [
    (1.0e24, 8),
    (1.0e21, 7),
    (1.0e18, 6),
    (1.0e15, 5),
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
    (1.0e-21, -7),
    (1.0e-24, -8),
];

const SI_PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

/// The prefix the unit takes at ladder index `index`; none past the ladder,
/// where [`si_exponent`] states the decade instead.
fn si_prefix(index: i32) -> &'static str {
    if (SI_MIN..=SI_MAX).contains(&index) {
        SI_PREFIXES[(index - SI_MIN) as usize]
    } else {
        ""
    }
}

/// The exponent the mantissa takes at ladder index `index`: none on the
/// ladder, `e-27` one step below yocto, `e27` one step above yotta.
fn si_exponent(index: i32) -> String {
    if (SI_MIN..=SI_MAX).contains(&index) {
        String::new()
    } else {
        format!("e{}", 3 * index)
    }
}

/// The scale an axis tick ends in: the prefix, or past the ladder the
/// exponent with the typographic minus every tick is signed with.
fn tick_scale(index: i32) -> String {
    si_exponent(index).replace('-', "−") + si_prefix(index)
}

/// `value` expressed under the prefix `index` names.
///
/// Positive indices divide and negative ones multiply, so on the ladder up to
/// 10^21 the factor is an exactly representable power of ten and the mantissa
/// carries no rounding the value did not already have. The factor is applied
/// in two steps past 10^300, which a subnormal needs and no single `f64` holds.
fn si_scaled(value: f64, index: i32) -> f64 {
    let decades = 3 * index.abs();
    let first = 10f64.powi(decades.min(300));
    let rest = 10f64.powi(decades - decades.min(300));
    if index >= 0 {
        value / first / rest
    } else {
        value * first * rest
    }
}

/// Engineering ladder index: mantissa in 1‥1000. Zero and the non-finite
/// values take no scale at all.
fn si_index(magnitude: f64) -> i32 {
    if magnitude == 0.0 || !magnitude.is_finite() {
        return 0;
    }
    if (SI_LADDER[SI_LADDER.len() - 1].0..1.0e27).contains(&magnitude) {
        return SI_LADDER
            .iter()
            .find(|(floor, _)| magnitude >= *floor)
            .map_or(SI_MIN, |(_, index)| *index);
    }
    // Past the ladder, the decade that puts the mantissa in 1‥1000. The
    // logarithm can land a hair either side of an exact power, so the guess
    // is settled against the mantissa it produces.
    let index = (magnitude.log10() / 3.0).floor() as i32;
    let mantissa = si_scaled(magnitude, index);
    if mantissa >= 1.0e3 {
        index + 1
    } else if mantissa < 1.0 {
        index - 1
    } else {
        index
    }
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
/// would carry the mantissa past 1000. Past yotta the promotion moves the
/// exponent on, so no end of the ladder prints a four-digit mantissa.
fn placed(magnitude: f64, index: i32, decimals: usize) -> i32 {
    if overflows_prefix(si_scaled(magnitude, index), decimals) {
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

/// An axis tick that carries its own unit: `si_tick_label(5e-4, "s", 2)` →
/// `"500 µs"`, `si_tick_label(-3e-3, "V", 2)` → `"−3 mV"`.
///
/// [`tick_label_with_step`] drops the unit, because a result viewer's axis
/// states the quantity once in its own caption and has room to. A 56 point
/// waveform strip has no caption track, so its ticks carry the unit
/// themselves — and they have to be trimmed and signed exactly as every other
/// tick in the product is, or the same number reads two ways on two surfaces.
pub fn si_tick_label(value: f64, unit: &str, digits: usize) -> String {
    let (scaled, index) = scale_si(value, digits);
    let mantissa = trimmed(format!("{scaled:.digits$}"));
    let exponent = si_exponent(index).replace('-', "−");
    let prefix = si_prefix(index);
    format!("{mantissa}{exponent} {prefix}{unit}")
        .trim_end()
        .to_owned()
}

/// A ladder tick with the unit its axis has no caption to carry:
/// `tick_with_unit("2m", "V")` → `"2 mV"`, `tick_with_unit("1.5", "V")` →
/// `"1.5 V"`.
///
/// The spelling [`si_tick_label`] gives a tick that is not on a ladder, so a
/// number reads one way on every surface whose ticks state their own unit.
pub fn tick_with_unit(label: &str, unit: &str) -> String {
    let mantissa = label.trim_end_matches(char::is_alphabetic);
    let prefix = &label[mantissa.len()..];
    format!("{mantissa} {prefix}{unit}").trim_end().to_owned()
}

/// The one spelling of a value a sheet prints: `significant_digits`
/// significant digits, an SI prefix on the unit, and a true minus sign.
///
/// - A value is rounded once, to the digits asked for; a rounding that
///   carries into the next decade drops the digit it no longer has room for
///   (999.95 at four digits is `1.000 kV`, never `1.0000 kV`).
/// - Zeros are printed only as digits of a rounded value. A value that has
///   fewer digits than the preference prints the digits it has: a node at
///   exactly 4.1 V reads `4.1 V`, not `4.100000 V`, while 1.7999999 V at four
///   digits reads `1.800 V` because those zeros were resolved.
/// - A unit that takes no prefix ([`unit_takes_prefix`]) is never given one:
///   a ratio of 0.0012 reads `0.0012`, a damping factor never `316.2 m`, and
///   far from unity the decade is an exponent on the number (`4.1e−12`).
/// - Negative zero reads `0`.
///
/// Engineering export serializes the original `f64` samples, not this text.
pub fn fmt_si_significant(value: f64, unit: &str, significant_digits: usize) -> String {
    let (number, prefix) = significant_number(value, significant_digits, unit_takes_prefix(unit));
    format!("{number} {prefix}{unit}").trim_end().to_owned()
}

/// Whether a value in `unit` is written with an SI prefix.
///
/// A dimensionless value (no unit, a ratio stated as `/V` or `1/V`, a count
/// of standard deviations, a fraction of a unit interval), a level in
/// decibels, an angle (`°`, `rad`, but not `rad/s`), a temperature in degrees
/// and a percentage are never prefixed, and a compound unit is prefixed as
/// its numerator is (`2 MV/s`, `−20 dB/dec`, `45 °/kHz`). Nor is a unit that
/// already names its own decade: `1786 nV/√Hz`, never `1.786 knV/√Hz`.
#[must_use]
pub fn unit_takes_prefix(unit: &str) -> bool {
    if unit_carries_its_decade(unit) {
        return false;
    }
    let numerator = unit.split('/').next().unwrap_or_default();
    !(numerator.is_empty()
        || unit == "rad"
        || numerator.starts_with("dB")
        || matches!(
            numerator,
            "1" | "°" | "°C" | "°F" | "deg" | "%" | "σ" | "sigma" | "UI"
        ))
}

/// Whether a unit already names its decade, so a value in it is written as
/// it stands: the noise densities the noise sheet reports in.
#[must_use]
pub fn unit_carries_its_decade(unit: &str) -> bool {
    matches!(unit, "nV/√Hz" | "nA/√Hz" | "ns/√Hz")
}

/// [`fmt_si_significant`] with an explicit sign on a positive value, for a
/// change or a margin: `+1.5 dB`, `−0.6 dB`, and zero unsigned.
pub fn fmt_si_signed(value: f64, unit: &str, significant_digits: usize) -> String {
    let text = fmt_si_significant(value, unit, significant_digits);
    if value > 0.0 && !text.starts_with('0') {
        format!("+{text}")
    } else {
        text
    }
}

/// The number part of a value and the prefix its unit takes.
fn significant_number(
    value: f64,
    significant_digits: usize,
    prefixed: bool,
) -> (String, &'static str) {
    let digits = significant_digits.clamp(1, 17);
    if value.is_nan() {
        return ("NaN".to_owned(), "");
    }
    if value.is_infinite() {
        let text = if value < 0.0 { "−inf" } else { "inf" };
        return (text.to_owned(), "");
    }
    if value == 0.0 {
        return ("0".to_owned(), "");
    }
    let magnitude = value.abs();
    let mut index = band_index(magnitude, prefixed);
    let mut text = significant_mantissa(si_scaled(magnitude, index), digits);
    // A rounding that carries the mantissa out of its band (999.95 → 1000)
    // is printed under the next band, where it has its digits back.
    // The test stays in mantissa space: scaled back, a carry at the top of
    // the double would overflow to infinity.
    let ceiling = match (prefixed, index) {
        (false, 0) => 1.0e6,
        (false, -2) => 1.0e2,
        _ => 1.0e3,
    };
    if text.parse::<f64>().is_ok_and(|rounded| rounded >= ceiling) {
        // Without a prefix, the plain band spans 0.0001 to a million, so a
        // carry into it or out of it skips the indices it covers.
        index = match (prefixed, index) {
            (false, -2) => 0,
            (false, 0) => 2,
            _ => index + 1,
        };
        text = significant_mantissa(si_scaled(magnitude, index), digits);
    }
    let on_ladder = prefixed && (SI_MIN..=SI_MAX).contains(&index);
    let exponent = if index == 0 || on_ladder {
        String::new()
    } else {
        format!("e{}", 3 * index).replace('-', "−")
    };
    let sign = if value < 0.0 { "−" } else { "" };
    let prefix = if on_ladder { si_prefix(index) } else { "" };
    (format!("{sign}{text}{exponent}"), prefix)
}

/// The ladder index a magnitude is printed under: the SI prefix ladder for a
/// prefixed unit; plain decimals from 0.0001 to a million for a unit that
/// takes no prefix, as `%g` keeps them, and an engineering exponent past
/// them.
fn band_index(magnitude: f64, prefixed: bool) -> i32 {
    if prefixed || !(1.0e-4..1.0e6).contains(&magnitude) {
        si_index(magnitude)
    } else {
        0
    }
}

/// `mantissa` (positive) to `digits` significant digits, keeping only the
/// zeros the rounding resolved.
fn significant_mantissa(mantissa: f64, digits: usize) -> String {
    let mut decade = mantissa.log10().floor() as i32;
    let mut text = round_to_significant(mantissa, digits, decade);
    // Rounding up to the next power of ten (9.9996 → 10.000) leaves one digit
    // more than was asked for; the decade it reached sets the decimals.
    if text
        .parse::<f64>()
        .is_ok_and(|rounded| rounded >= 10f64.powi(decade + 1))
    {
        decade += 1;
        text = round_to_significant(mantissa, digits, decade);
    }
    let decimals = (digits as i32 - 1 - decade).max(0);
    let trimmed = if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text.as_str()
    };
    // The zeros go when the value is the shorter number to well within the
    // last printed digit, or to within the arithmetic noise of a double (a
    // kelvin-to-celsius subtraction leaves 27.000000000001): they would claim
    // a resolution the value does not need.
    let resolution = 10f64.powi(-decimals) * 1.0e-6;
    let tolerance = resolution.max(mantissa * 1.0e-12);
    match trimmed.parse::<f64>() {
        Ok(short) if (short - mantissa).abs() <= tolerance => trimmed.to_owned(),
        _ => text,
    }
}

/// `mantissa` rounded to `digits` significant digits, given its decade.
fn round_to_significant(mantissa: f64, digits: usize, decade: i32) -> String {
    let decimals = digits as i32 - 1 - decade;
    if decimals >= 0 {
        format!("{:.*}", decimals as usize, mantissa)
    } else {
        let quantum = 10f64.powi(-decimals);
        format!("{:.0}", (mantissa / quantum).round() * quantum)
    }
}

/// `value` placed on the ladder for `decimals` places: the mantissa and the
/// ladder index it is printed under.
fn scale_si(value: f64, decimals: usize) -> (f64, i32) {
    let magnitude = value.abs();
    if magnitude == 0.0 || !magnitude.is_finite() {
        return (value, 0);
    }
    let index = placed(magnitude, si_index(magnitude), decimals);
    (si_scaled(value, index), index)
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
    if value == 0.0 || !value.is_finite() {
        return tick_label(value);
    }
    let magnitude = value.abs();
    let mut index = tick_index(magnitude);
    let mut decimals = step_decimals(si_scaled(step, index));
    let promoted = placed(magnitude, index, decimals);
    if promoted != index {
        index = promoted;
        decimals = step_decimals(si_scaled(step, index));
    }
    trimmed(format!("{:.*}", decimals, si_scaled(value, index))) + &tick_scale(index)
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
    let exponent = si_exponent(index).replace('-', "−");
    format!("{sign}{mantissa}{exponent} {}{unit}", si_prefix(index))
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
    trimmed(format!("{:.*}", decimals, si_scaled(value, index))) + &tick_scale(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A ladder tick and an off-ladder tick of the same number read alike,
    /// which is the whole reason the first one has a spelling of its own.
    #[test]
    fn a_ladder_tick_with_its_unit_reads_like_an_off_ladder_one() {
        assert_eq!(tick_with_unit("2m", "V"), "2 mV");
        assert_eq!(tick_with_unit("500µ", "s"), "500 µs");
        assert_eq!(tick_with_unit("−1.5", "A"), "−1.5 A");
        assert_eq!(tick_with_unit("4k", ""), "4 k");
        assert_eq!(tick_with_unit("2m", "V"), si_tick_label(2e-3, "V", 2));
        assert_eq!(tick_with_unit("500µ", "s"), si_tick_label(5e-4, "s", 2));
    }

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
        assert_eq!(fmt_si_significant(2.0e-6, "s", 4), "2 µs");
        assert_eq!(fmt_si_significant(10.4e6, "Hz", 3), "10.4 MHz");
        assert_eq!(fmt_si_significant(0.0, "V", 3), "0 V");
        assert_eq!(fmt_si_signed(1.5, "dB", 4), "+1.5 dB");
        assert_eq!(fmt_si_signed(-0.6, "dB", 4), "\u{2212}0.6 dB");
        assert_eq!(fmt_si_signed(0.0, "dB", 4), "0 dB");
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
    fn the_si_ladder_reaches_yotta_and_yocto_in_both_directions() {
        assert_eq!(tick_label(1.0e12), "1T");
        assert_eq!(tick_label(2.5e12), "2.5T");
        assert_eq!(tick_label(1.0e-15), "1f");
        assert_eq!(tick_label(4.7e-18), "4.7a");
        assert_eq!(fmt_si_significant(3.3e12, "Hz", 4), "3.3 THz");
        assert_eq!(fmt_si_significant(2.0e-18, "F", 4), "2 aF");
        // Past tera and atto the ladder carries on through the prefixes the
        // reference tools print, rather than parking the value on the last.
        assert_eq!(fmt_si_significant(2.402_879e-20, "V", 7), "24.02879 zV");
        assert_eq!(fmt_si_significant(3.1e-24, "C", 4), "3.1 yC");
        assert_eq!(fmt_si_significant(4.2e15, "Hz", 4), "4.2 PHz");
        assert_eq!(fmt_si_significant(1.5e18, "Hz", 4), "1.5 EHz");
        assert_eq!(fmt_si_significant(6.0e21, "W", 4), "6 ZW");
        assert_eq!(fmt_si_significant(7.5e24, "W", 4), "7.5 YW");
        assert_eq!(tick_label(5.0e-21), "5z");
        assert_eq!(tick_label(-2.0e15), "−2P");
        assert_eq!(si_tick_label(1.0e-24, "s", 2), "1 ys");
    }

    /// Below yocto and above yotta the mantissa stays in 1‥1000 and the
    /// decade moves into an exponent on the number, as Cadence's suffix
    /// notation leaves the `e` and exponent once its suffixes run out. A value
    /// is never parked on the last prefix with a run of leading zeros.
    #[test]
    fn past_the_ladder_a_value_states_its_decade_as_an_exponent() {
        assert_eq!(fmt_si_significant(2.402_879e-26, "V", 7), "24.02879e−27 V");
        assert_eq!(fmt_si_significant(2.402_879e-26, "V", 7), "24.02879e−27 V");
        assert_eq!(fmt_si_significant(-5.0e-30, "A", 4), "−5e−30 A");
        assert_eq!(fmt_si_significant(3.0e27, "Hz", 4), "3e27 Hz");
        assert_eq!(fmt_si_significant(4.56e31, "Hz", 4), "45.6e30 Hz");
        assert_eq!(fmt_si_significant(2.5e-28, "F", 4), "250e−30 F");
        assert_eq!(fmt_si_significant(2.5e-28, "", 4), "250e−30");
        // A tick signs its exponent as it signs its value.
        assert_eq!(tick_label(-3.0e-27), "−3e−27");
        assert_eq!(si_tick_label(3.0e-27, "V", 2), "3e−27 V");
        assert_eq!(tick_with_unit(&tick_label(3.0e-27), "V"), "3e−27 V");
        assert_eq!(offset_anchor_label(2.0e-27, "s", 1.0e-33), "+2e−27 s");
        // Rounding past 1000 promotes across the ladder's ends both ways.
        assert_eq!(fmt_si_significant(999.999_99e-27, "V", 4), "1.000 yV");
        assert_eq!(fmt_si_significant(999.999_99e24, "V", 4), "1.000e27 V");
        // The far ends of the double stay finite and bounded.
        assert_eq!(
            fmt_si_significant(f64::MIN_POSITIVE, "V", 4),
            "22.25e−309 V"
        );
        assert_eq!(fmt_si_significant(5.0e-324, "V", 2), "4.9e−324 V");
        assert_eq!(fmt_si_significant(f64::MAX, "V", 4), "179.8e306 V");
        assert_eq!(fmt_si_significant(f64::INFINITY, "V", 4), "inf V");
        assert_eq!(tick_label_with_step(f64::NAN, 1.0), "NaN");
    }

    #[test]
    fn a_mantissa_that_rounds_up_promotes_its_prefix() {
        // 999.9999 k is 1 M. Printing it under the smaller prefix reads as a
        // decade error on the axis.
        assert_eq!(tick_label(999_999.9), "1M");
        assert_eq!(tick_label(0.999_999_5), "1");
        assert_eq!(tick_label(999.999_9e-6), "1m");
        assert_eq!(fmt_si_significant(999_999.9, "Hz", 4), "1.000 MHz");
        assert_eq!(fmt_si_significant(999.999_99, "V", 4), "1.000 kV");
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
        assert_eq!(fmt_si_significant(0.0, "A", 3), "0 A");
    }

    /// A rounding that carries into the next decade prints exactly the digits
    /// asked for, under the prefix it carried into.
    #[test]
    fn a_rounding_carry_keeps_the_digit_count() {
        assert_eq!(fmt_si_significant(999.95, "V", 4), "1.000 kV");
        assert_eq!(fmt_si_significant(9.999_99, "V", 4), "10.00 V");
        assert_eq!(fmt_si_significant(99.996, "A", 4), "100.0 A");
        assert_eq!(fmt_si_significant(1.0e-24, "V", 3), "1 yV");
        assert_eq!(fmt_si_significant(0.000_999_99, "", 3), "0.00100");
        assert_eq!(fmt_si_significant(999_999.96, "", 7), "1.000000e6");
    }

    /// Negative zero is zero, and every negative value carries the true
    /// minus the axes beside it are drawn with.
    #[test]
    fn negative_values_take_the_true_minus_and_negative_zero_takes_none() {
        assert_eq!(fmt_si_significant(-0.0, "V", 4), "0 V");
        assert_eq!(fmt_si_significant(-0.0, "", 7), "0");
        assert_eq!(fmt_si_significant(-3.0e-3, "V", 4), "\u{2212}3 mV");
        assert_eq!(fmt_si_significant(-179.999_9, "°", 7), "\u{2212}179.9999 °");
        assert_eq!(
            fmt_si_significant(-4.1e-12, "", 4),
            "\u{2212}4.1e\u{2212}12"
        );
        assert!(!fmt_si_significant(-12.5, "dB", 4).contains('-'));
    }

    /// A ratio, a damping factor and a percentage are never given a prefix,
    /// and nor is a unit that already names its decade.
    #[test]
    fn a_dimensionless_value_takes_no_prefix() {
        assert_eq!(fmt_si_significant(0.4266, "sigma", 4), "0.4266 sigma");
        assert_eq!(fmt_si_significant(0.62, "UI", 4), "0.62 UI");
        assert_eq!(fmt_si_significant(0.002, "1/V", 4), "0.002 1/V");
        assert_eq!(fmt_si_significant(0.316_227_8, "", 5), "0.31623");
        assert_eq!(fmt_si_significant(0.707, "", 4), "0.707");
        assert_eq!(fmt_si_significant(12.5, "%", 4), "12.5 %");
        assert_eq!(fmt_si_significant(-0.999_990_2, "", 7), "\u{2212}0.9999902");
        assert_eq!(fmt_si_significant(4.1e-12, "", 4), "4.1e\u{2212}12");
        assert_eq!(fmt_si_significant(1.234_567e7, "", 4), "12.35e6");
        assert_eq!(fmt_si_significant(-42.0, "dBc", 4), "\u{2212}42 dBc");
        assert_eq!(fmt_si_significant(1786.13, "nV/√Hz", 6), "1786.13 nV/√Hz");
        assert_eq!(fmt_si_significant(2.5e-3, "/V", 4), "0.0025 /V");
        // A unit keeps its prefix.
        assert_eq!(fmt_si_significant(0.707, "V", 4), "707 mV");
    }

    /// Zeros are digits of a rounded value, never padding of an exact one.
    #[test]
    fn zeros_are_printed_only_where_the_rounding_resolved_them() {
        assert_eq!(fmt_si_significant(4.1, "V", 7), "4.1 V");
        assert_eq!(fmt_si_significant(4.1e-3, "V", 7), "4.1 mV");
        assert_eq!(fmt_si_significant(1.5e-15, "F", 7), "1.5 fF");
        assert_eq!(fmt_si_significant(1.799_999_9, "V", 4), "1.800 V");
        assert_eq!(fmt_si_significant(1.000_2, "V", 4), "1.000 V");
        assert_eq!(fmt_si_significant(1.000_2, "V", 7), "1.0002 V");
        assert_eq!(fmt_si_significant(1.49, "V", 3), "1.49 V");
    }

    /// A panel readout prints the digits the value has, not the digits the
    /// preference allows: `4.100000 V` is a formatter default, not a reading.
    #[test]
    fn a_panel_readout_drops_the_zeros_that_only_pad_it_out() {
        assert_eq!(fmt_si_significant(4.1, "V", 7), "4.1 V");
        assert_eq!(fmt_si_significant(0.9, "V", 7), "900 mV");
        assert_eq!(fmt_si_significant(0.0, "A", 7), "0 A");
        // Digits the value does have are kept, to the stated precision.
        assert_eq!(fmt_si_significant(12.345_678, "V", 7), "12.34568 V");
        assert_eq!(fmt_si_significant(96.861_12e-6, "A", 4), "96.86 µA");
        // A bound is the same readout at the compact precision.
        assert_eq!(fmt_si_significant(1.0e7, "Hz", 4), "10 MHz");
    }
}
