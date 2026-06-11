//! Monotonic-clock shim for diagnostic timing.
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
