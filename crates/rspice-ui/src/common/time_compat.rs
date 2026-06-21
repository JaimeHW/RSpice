//! Clock shims for the browser build.
//!
//! `std::time::{Instant, SystemTime}` trap at runtime on
//! wasm32-unknown-unknown. Native builds re-export std; wasm builds back
//! `Instant` with `Date.now()` (millisecond monotonic-enough for UI timing)
//! and derive unix timestamps the same way. UI code must use these instead of
//! `std::time` for anything that runs in the app (file metadata timestamps,
//! console stamps, elapsed readouts, DRC/optimizer timing).

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instant(f64);

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self {
        Instant(js_sys::Date::now())
    }

    pub fn elapsed(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f64(((js_sys::Date::now() - self.0).max(0.0)) / 1000.0)
    }

    #[allow(dead_code)]
    pub fn duration_since(&self, earlier: Instant) -> std::time::Duration {
        std::time::Duration::from_secs_f64(((self.0 - earlier.0).max(0.0)) / 1000.0)
    }
}

/// Wall-clock time since the unix epoch (wasm-safe `SystemTime::now()`).
pub(crate) fn unix_epoch() -> std::time::Duration {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::time::Duration::from_secs_f64((js_sys::Date::now() / 1000.0).max(0.0))
    }
}
