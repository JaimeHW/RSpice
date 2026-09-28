//! Application clock and project integration for portable stimulus definitions.

pub(crate) use rspice_design::stimulus_library::{definition, draft, provenance};
pub(crate) mod library;

/// The authored library the stimulus surfaces are gated and rendered against.
#[cfg(test)]
pub(crate) mod fixtures;

/// Wall-clock milliseconds, through the shim the browser build needs.
pub(crate) fn now_unix_ms() -> u64 {
    u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX)
}
