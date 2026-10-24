//! Time helpers that behave the same on every target: a monotonic-clock shim
//! for diagnostic timing, and the civil calendar of a Unix timestamp.
//!
//! `std::time::Instant::now()` aborts at runtime on `wasm32-unknown-unknown`
//! (no clock in the bare-wasm ABI), which trapped every transient run in the
//! browser. Engine code that times progress/slow-path logging uses this alias
//! instead: native builds get the real `Instant`; wasm builds get a no-op
//! clock whose `elapsed()` is always zero, so threshold-triggered diagnostics
//! simply stay quiet there. Never use this for simulation results — it is for
//! wall-clock diagnostics only.

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug)]
pub struct Instant;

#[cfg(target_arch = "wasm32")]
impl Instant {
    #[inline]
    pub fn now() -> Self {
        Instant
    }

    #[inline]
    pub fn elapsed(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}

/// The proleptic Gregorian `(year, month, day)` of a day count since
/// 1970-01-01.
///
/// Howard Hinnant's civil-from-days algorithm. It moves the epoch to
/// 0000-03-01 so a leap day ends its year; splitting the count into 400-year
/// eras before that shift keeps every intermediate in range, so the whole
/// `i64` domain converts without overflow.
pub fn civil_from_unix_days(days: i64) -> (i64, u8, u8) {
    const ERA_DAYS: i64 = 146_097;
    let shifted = days.rem_euclid(ERA_DAYS) + 719_468;
    let era = days.div_euclid(ERA_DAYS) + shifted.div_euclid(ERA_DAYS);
    let day_of_era = shifted.rem_euclid(ERA_DAYS);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    let year = era * 400 + year_of_era + i64::from(month <= 2);
    (year, month as u8, day as u8)
}

/// An asctime-style UTC timestamp, `Thu Jan 1 00:00:00 1970`, as ngspice
/// writes in a rawfile header.
pub fn asctime_utc(unix_seconds: u64) -> String {
    const WEEKDAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let days = (unix_seconds / 86_400) as i64;
    let time_of_day = unix_seconds % 86_400;
    let (year, month, day) = civil_from_unix_days(days);
    format!(
        "{} {} {day} {:02}:{:02}:{:02} {year}",
        // 1970-01-01 was a Thursday.
        WEEKDAYS[days.rem_euclid(7) as usize],
        MONTHS[usize::from(month - 1)],
        time_of_day / 3600,
        (time_of_day % 3600) / 60,
        time_of_day % 60,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates_match_the_gregorian_calendar() {
        for (days, expected) in [
            (0, (1970, 1, 1)),
            (-1, (1969, 12, 31)),
            (789, (1972, 2, 29)),
            (790, (1972, 3, 1)),
            (11_016, (2000, 2, 29)),
            (11_017, (2000, 3, 1)),
            (47_540, (2100, 2, 28)),
            (47_541, (2100, 3, 1)),
            (-25_508, (1900, 3, 1)),
            (-719_162, (1, 1, 1)),
            (-719_468, (0, 3, 1)),
            (-719_469, (0, 2, 29)),
            (2_932_896, (9999, 12, 31)),
            (i64::MAX, (25_252_734_927_768_524, 7, 27)),
            (i64::MIN, (-25_252_734_927_764_585, 6, 7)),
        ] {
            assert_eq!(civil_from_unix_days(days), expected, "day {days}");
        }
    }

    #[test]
    fn asctime_matches_reference_dates() {
        assert_eq!(asctime_utc(0), "Thu Jan 1 00:00:00 1970");
        assert_eq!(asctime_utc(1_750_000_000), "Sun Jun 15 15:06:40 2025");
    }
}
