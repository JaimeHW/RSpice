//! Test entry into the shipping service entries.
//!
//! Tests call the entry the runner calls, under the default host policy, and
//! parse decks the way the runner parses them.

use rspice_core::abort_signal::AbortSignal;

use super::ServiceContext;

/// The default host policy with no source path, observing `abort`.
pub(crate) fn ctx(abort: &dyn AbortSignal) -> ServiceContext<'_> {
    ServiceContext::with_defaults(None, abort)
}
