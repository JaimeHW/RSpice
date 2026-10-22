//! Test entry into the shipping service entries.
//!
//! Tests call the entry the runner calls, under the default host policy, and
//! parse decks the way the runner parses them.

use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::{Netlist, Value};

use super::{PssRunConfig, ServiceContext};

/// The default host policy with no source path, observing `abort`.
pub(crate) fn ctx(abort: &dyn AbortSignal) -> ServiceContext<'_> {
    ServiceContext::with_defaults(None, abort)
}

/// Parse `text` as the runner does; the test deck must parse.
pub(crate) fn parse(text: &str) -> Netlist {
    ctx(&NoAbort).parse(text).expect("the test deck parses")
}

/// A driven PSS whose tones are every time-varying source the circuit
/// drives, the complete set the periodic source contract accepts.
pub(crate) fn driven_pss_config(
    netlist: &Netlist,
    fundamental: Value,
    harmonics: usize,
    tolerance: Value,
) -> PssRunConfig {
    let tones = rspice_core::engine::Engine::new(super::build_engine_config(netlist, None))
        .transient_source_names_with_abort(netlist, &NoAbort)
        .expect("the deck's sources elaborate");
    PssRunConfig::new(fundamental, tones, harmonics, tolerance)
}
