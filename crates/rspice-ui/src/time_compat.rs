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
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "system clock is before the Unix epoch")
    }
    #[cfg(target_arch = "wasm32")]
    {
        let milliseconds = js_sys::Date::now();
        if !(0.0..u64::MAX as f64).contains(&milliseconds) {
            return Err("system clock returned an invalid Unix timestamp");
        }
        // Date.now() returns integral milliseconds. Converting through f64
        // seconds can round a valid timestamp down by one millisecond.
        Ok(std::time::Duration::from_millis(milliseconds as u64))
    }
}

/// Wall time for callers whose existing contract uses the epoch on failure.
pub(crate) fn unix_epoch() -> std::time::Duration {
    checked_unix_epoch().unwrap_or_default()
}

/// Millisecond wall time, saturating when it cannot fit the persisted range.
pub(crate) fn unix_time_ms() -> u64 {
    unix_epoch().as_millis().try_into().unwrap_or(u64::MAX)
}
