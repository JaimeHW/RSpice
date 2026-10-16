//! Wall and monotonic clocks shared by desktop, browser, and worker code.
//!
//! `std::time::{Instant, SystemTime}` trap at runtime on
//! wasm32-unknown-unknown. `web_time` re-exports std on native and uses
//! `performance.now()` for browser/worker elapsed time. Only persisted wall
//! timestamps use `Date.now()`. UI code must use these instead of
//! `std::time` for anything that runs in the app (file metadata timestamps,
//! console stamps, elapsed readouts, DRC/optimizer timing).

pub use web_time::Instant;

/// Read wall time without trapping or accepting an invalid browser timestamp.
/// Evidence producers can propagate the failure instead of fabricating a date.
pub(crate) fn checked_unix_epoch() -> Result<std::time::Duration, &'static str> {
    #[cfg(test)]
    if let Some(epoch) = tests::TEST_EPOCH.with(std::cell::Cell::get) {
        return epoch;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "system clock is before the Unix epoch")
    }
    #[cfg(target_arch = "wasm32")]
    {
        let milliseconds = js_sys::Date::now();
        if !(0.0..u64::MAX as f64).contains(&milliseconds) || milliseconds.fract() != 0.0 {
            return Err("system clock returned an invalid Unix timestamp");
        }
        // Date.now() returns integral milliseconds. Converting through f64
        // seconds can round a valid timestamp down by one millisecond.
        Ok(std::time::Duration::from_millis(milliseconds as u64))
    }
}

/// How long ago something happened, in the one vocabulary the workbench uses:
/// `just now`, `7 min ago`, `3 h ago`, `yesterday`, `4 d ago`.
///
/// Eight surfaces each carried their own copy of this ladder and three of them
/// disagreed — `moments ago` against `just now`, `4 days ago` against `4 d
/// ago`, and `yesterday` present on the revision lists and missing everywhere
/// else. A reader comparing a checkpoint to a revision should not have to
/// notice which panel wrote the sentence.
///
/// It takes the elapsed time rather than a timestamp because what to say when
/// there is no timestamp at all is the caller's question, not this one's: a
/// recovery list says `an earlier session`, an evidence row says
/// `time unavailable`, and a navigator row says nothing.
pub(crate) fn relative_age(elapsed: std::time::Duration) -> String {
    match elapsed.as_secs() {
        0..=59 => "just now".to_owned(),
        seconds @ 60..=3_599 => format!("{} min ago", seconds / 60),
        seconds @ 3_600..=86_399 => format!("{} h ago", seconds / 3_600),
        86_400..=172_799 => "yesterday".to_owned(),
        seconds => format!("{} d ago", seconds / 86_400),
    }
}

/// A nonzero millisecond timestamp for durable evidence. Zero is reserved for
/// unknown historical time; a failed clock must not create a new dated record.
pub(crate) fn checked_unix_time_ms() -> Result<u64, &'static str> {
    let milliseconds = checked_unix_epoch()?.as_millis();
    let milliseconds = u64::try_from(milliseconds)
        .map_err(|_| "system clock timestamp exceeds the supported range")?;
    if milliseconds == 0 {
        Err("system clock timestamp is unavailable")
    } else {
        Ok(milliseconds)
    }
}

/// Wall time for callers whose existing contract uses the epoch on failure.
pub(crate) fn unix_epoch() -> std::time::Duration {
    checked_unix_epoch().unwrap_or_default()
}

/// A stored millisecond timestamp written out for a reader, in UTC.
///
/// Every durable record in the project — a checkpoint, a revision, a stimulus
/// definition — keeps its time as milliseconds since the epoch, and every
/// surface that shows one owes the reader the same spelling. UTC rather than
/// the local zone because these timestamps travel with the document: a project
/// mailed across a border must not describe itself as having been edited at two
/// different times.
///
/// Zero is the reserved "unknown historical time" of
/// [`checked_unix_time_ms`], and a timestamp the calendar cannot place is the
/// same absence, so both say so rather than printing 1970.
pub(crate) fn utc_stamp(unix_ms: u64) -> String {
    const UNKNOWN: &str = "at an unknown time";
    if unix_ms == 0 {
        return UNKNOWN.to_owned();
    }
    let Ok(seconds) = i64::try_from(unix_ms / 1_000) else {
        return UNKNOWN.to_owned();
    };
    let Ok(stamp) = time::OffsetDateTime::from_unix_timestamp(seconds) else {
        return UNKNOWN.to_owned();
    };
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02} UTC",
        stamp.year(),
        u8::from(stamp.month()),
        stamp.day(),
        stamp.hour(),
        stamp.minute()
    )
}

/// A millisecond timestamp on the reader's own clock.
///
/// For what the reader saw happen in this session and wants to place against
/// the clock on the wall: a log line, a run's start. `None` for a timestamp the
/// calendar cannot place.
pub(crate) fn local_date_time(unix_ms: u64) -> Option<time::OffsetDateTime> {
    let utc =
        time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(unix_ms) * 1_000_000).ok()?;
    let offset = time::UtcOffset::from_whole_seconds(local_offset_seconds(utc.unix_timestamp()))
        .unwrap_or(time::UtcOffset::UTC);
    Some(utc.to_offset(offset))
}

/// The local zone's offset from UTC at one instant, in seconds east.
///
/// Asked of the platform once per quarter hour of timestamps, because the
/// Console writes the time of every row it paints and an offset moves only
/// at a zone transition, which falls on a quarter hour. UTC where the
/// platform cannot read its zone.
pub(crate) fn local_offset_seconds(unix_seconds: i64) -> i32 {
    thread_local! {
        static HELD: std::cell::RefCell<std::collections::HashMap<i64, i32>> =
            std::cell::RefCell::new(std::collections::HashMap::new());
    }
    const QUARTER_HOUR: i64 = 900;
    const HELD_QUARTERS: usize = 1_024;
    let quarter = unix_seconds.div_euclid(QUARTER_HOUR);
    HELD.with(|held| {
        let mut held = held.borrow_mut();
        if let Some(offset) = held.get(&quarter) {
            return *offset;
        }
        if held.len() >= HELD_QUARTERS {
            held.clear();
        }
        let offset = zone_offset_seconds(quarter * QUARTER_HOUR);
        held.insert(quarter, offset);
        offset
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn zone_offset_seconds(unix_seconds: i64) -> i32 {
    use chrono::{Offset, TimeZone};
    chrono::DateTime::from_timestamp(unix_seconds, 0).map_or(0, |utc| {
        chrono::Local
            .offset_from_utc_datetime(&utc.naive_utc())
            .fix()
            .local_minus_utc()
    })
}

#[cfg(target_arch = "wasm32")]
fn zone_offset_seconds(unix_seconds: i64) -> i32 {
    // `getTimezoneOffset` is minutes *west* of UTC.
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(
        unix_seconds as f64 * 1_000.0,
    ));
    let minutes_west = date.get_timezone_offset();
    if minutes_west.is_finite() {
        (-minutes_west * 60.0) as i32
    } else {
        0
    }
}

#[cfg(test)]
pub(crate) use tests::with_unix_epoch;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    thread_local! {
        pub(super) static TEST_EPOCH: std::cell::Cell<Option<Result<Duration, &'static str>>> = const {
            std::cell::Cell::new(None)
        };
    }

    /// Scoped, thread-local fault injection for synchronous transaction tests.
    pub(crate) fn with_unix_epoch<R>(
        epoch: Result<Duration, &'static str>,
        operation: impl FnOnce() -> R,
    ) -> R {
        struct Restore(Option<Result<Duration, &'static str>>);
        impl Drop for Restore {
            fn drop(&mut self) {
                TEST_EPOCH.with(|clock| clock.set(self.0));
            }
        }
        let _restore = Restore(TEST_EPOCH.with(|clock| clock.replace(Some(epoch))));
        operation()
    }

    #[test]
    fn one_relative_age_vocabulary_covers_every_boundary() {
        for (seconds, expected) in [
            (0, "just now"),
            (59, "just now"),
            (60, "1 min ago"),
            (3_599, "59 min ago"),
            (3_600, "1 h ago"),
            (86_399, "23 h ago"),
            (86_400, "yesterday"),
            (172_799, "yesterday"),
            (172_800, "2 d ago"),
            (864_000, "10 d ago"),
        ] {
            assert_eq!(
                relative_age(Duration::from_secs(seconds)),
                expected,
                "{seconds} s"
            );
        }
    }

    #[test]
    fn evidence_time_rejects_missing_time_without_saturation() {
        with_unix_epoch(Ok(Duration::from_millis(42)), || {
            assert_eq!(checked_unix_time_ms(), Ok(42));
            for epoch in [
                Err("clock unavailable"),
                Ok(Duration::ZERO),
                Ok(Duration::MAX),
            ] {
                with_unix_epoch(epoch, || assert!(checked_unix_time_ms().is_err()));
                assert_eq!(checked_unix_time_ms(), Ok(42));
            }
        });
    }

    /// Whatever zone the machine running the test is in, the local clock is
    /// that zone's offset from the same instant in UTC.
    #[test]
    fn the_local_clock_is_the_same_instant_on_the_zones_offset() {
        let unix_ms = 1_760_613_845_250; // 2025-10-16 11:24:05.250 UTC
        let local = local_date_time(unix_ms).expect("a placeable instant");
        let offset = local_offset_seconds(1_760_613_845);
        assert_eq!(local.offset().whole_seconds(), offset);
        assert_eq!(local.unix_timestamp(), 1_760_613_845);
        assert_eq!(local.millisecond(), 250);
        assert!((-12 * 3_600..=14 * 3_600).contains(&offset), "{offset} s");
    }
}
