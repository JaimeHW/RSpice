//! SI-prefixed number formatting for axes, readouts and cursor blocks.

/// Format `value` with an SI prefix and unit: `fmt_si(2.0e-6, "s", 3)` →
/// `"2.000 µs"`. Zero renders without a prefix.
pub fn fmt_si(value: f64, unit: &str, digits: usize) -> String {
    let (scaled, prefix) = scale_si(value);
    format!("{scaled:.digits$} {prefix}{unit}")
        .trim_end()
        .to_owned()
}

/// Format a result readout with an exact number of significant digits while
/// retaining SI-prefix presentation. This is intentionally separate from
/// engineering export, which serializes the original `f64` samples.
pub fn fmt_si_significant(value: f64, unit: &str, significant_digits: usize) -> String {
    let (scaled, prefix) = scale_si(value);
    format_with_significant_digits(scaled, significant_digits, &format!(" {prefix}{unit}"))
        .trim_end()
        .to_owned()
}

/// Format a unitless or fixed-unit value with an exact number of significant
/// digits. `suffix` should include its own leading space when desired.
pub fn fmt_significant(value: f64, significant_digits: usize, suffix: &str) -> String {
    format_with_significant_digits(value, significant_digits, suffix)
}

fn scale_si(value: f64) -> (f64, &'static str) {
    let magnitude = value.abs();
    if magnitude == 0.0 || !magnitude.is_finite() {
        (value, "")
    } else if magnitude >= 1e9 {
        (value / 1e9, "G")
    } else if magnitude >= 1e6 {
        (value / 1e6, "M")
    } else if magnitude >= 1e3 {
        (value / 1e3, "k")
    } else if magnitude >= 1.0 {
        (value, "")
    } else if magnitude >= 1e-3 {
        (value * 1e3, "m")
    } else if magnitude >= 1e-6 {
        (value * 1e6, "µ")
    } else if magnitude >= 1e-9 {
        (value * 1e9, "n")
    } else if magnitude >= 1e-12 {
        (value * 1e12, "p")
    } else {
        (value * 1e15, "f")
    }
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
    let exponent = value.abs().log10().floor() as i32;
    let decimals = (significant_digits as i32 - 1 - exponent).max(0) as usize;
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
pub fn tick_label(value: f64) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    let magnitude = value.abs();
    // Human-scale values (0.1 V, 0.8 V, 240) stay plain decimals like the
    // design's voltage axes; prefixes start below 0.1 and at 1000.
    let (scaled, prefix) = if magnitude >= 1e9 {
        (value / 1e9, "G")
    } else if magnitude >= 1e6 {
        (value / 1e6, "M")
    } else if magnitude >= 1e3 {
        (value / 1e3, "k")
    } else if magnitude >= 0.0995 {
        (value, "")
    } else if magnitude >= 0.995e-3 {
        (value * 1e3, "m")
    } else if magnitude >= 0.995e-6 {
        (value * 1e6, "µ")
    } else if magnitude >= 0.995e-9 {
        (value * 1e9, "n")
    } else {
        (value * 1e12, "p")
    };
    let mut s = format!("{scaled:.3}");
    while s.contains('.') && (s.ends_with('0') || s.ends_with('.')) {
        s.pop();
    }
    // The design uses the typographic minus on plots.
    s = s.replace('-', "−");
    s + prefix
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn significant_readouts_are_distinct_from_fixed_decimal_formatting() {
        assert_eq!(fmt_si_significant(12.345_678, "V", 7), "12.34568 V");
        assert_eq!(fmt_si_significant(1.234_567_8e-6, "s", 7), "1.234568 µs");
        assert_eq!(fmt_significant(-179.999_9, 7, " °"), "-179.9999 °");
        assert_eq!(fmt_si_significant(0.0, "A", 3), "0.00 A");
    }
}
