//! Formatting a span of wall-clock time for display.

/// A span of wall-clock time as a person reads it: `0.043 s`, `12.40 s`,
/// `3 min 25 s`, `2 h 4 min 9 s`.
///
/// Under a minute the reading keeps the precision a timing comparison needs;
/// past one, it is whole seconds, because a long run read as `12345.67 s`
/// has to be divided before it means anything. A time that is not a finite,
/// non-negative number of seconds was never measured, and says so.
#[must_use]
pub fn format_elapsed(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "not available".to_owned();
    }
    // Each branch decides on the value it will print, so 59.996 s reads as
    // `1 min 0 s` rather than `60.00 s`.
    if (seconds * 1_000.0).round() < 1_000.0 {
        return format!("{seconds:.3} s");
    }
    if (seconds * 100.0).round() < 6_000.0 {
        return format!("{seconds:.2} s");
    }
    let whole = seconds.round() as u64;
    let (hours, minutes, seconds) = (whole / 3_600, whole % 3_600 / 60, whole % 60);
    if hours == 0 {
        format!("{minutes} min {seconds} s")
    } else {
        format!("{hours} h {minutes} min {seconds} s")
    }
}

#[cfg(test)]
mod tests {
    use super::format_elapsed;

    #[test]
    fn a_span_reads_in_the_largest_units_that_fit() {
        for (seconds, expected) in [
            (0.0, "0.000 s"),
            (0.0434, "0.043 s"),
            (0.9994, "0.999 s"),
            (0.9996, "1.00 s"),
            (12.4, "12.40 s"),
            (59.994, "59.99 s"),
            (59.996, "1 min 0 s"),
            (90.0, "1 min 30 s"),
            (205.4, "3 min 25 s"),
            (3_599.6, "1 h 0 min 0 s"),
            (7_449.0, "2 h 4 min 9 s"),
            (12_345.67, "3 h 25 min 46 s"),
        ] {
            assert_eq!(format_elapsed(seconds), expected, "{seconds}");
        }
    }

    #[test]
    fn a_time_never_measured_says_so() {
        for seconds in [f64::NAN, f64::INFINITY, -1.0] {
            assert_eq!(format_elapsed(seconds), "not available", "{seconds}");
        }
    }
}
