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
}
