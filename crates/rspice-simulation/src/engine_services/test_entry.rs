//! Test entry into the shipping service entries.
//!
//! Tests call the entry the runner calls, under the default host policy, and
//! parse decks the way the runner parses them.

use rspice_core::Netlist;
use rspice_core::abort_signal::{AbortSignal, NoAbort};

use super::ServiceContext;

/// The default host policy with no source path, observing `abort`.
pub(crate) fn ctx(abort: &dyn AbortSignal) -> ServiceContext<'_> {
    ServiceContext::with_defaults(None, abort)
}

/// Parse `text` as the runner does; the test deck must parse.
pub(crate) fn parse(text: &str) -> Netlist {
    ctx(&NoAbort).parse(text).expect("the test deck parses")
}
