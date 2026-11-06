//! The starter model library compiled into the binary.
//!
//! This is the `rspice-foundation` pack authored in this repository. It is
//! deliberately small and embedded so the same starter models exist on every
//! platform without shipping the third-party development corpus.
//!
//! One source file is shared by library browsing, source mapping and engine
//! fallback resolution, so those consumers cannot drift onto different cards.
//! This module hands out the bytes and nothing more: interpreting them is the
//! parser's job, and `netlist::builtin_subcircuits` does it there.

/// Generic RSpice-authored starter models and the foundation op-amp subcircuit.
#[doc(hidden)]
pub const FOUNDATION_LIB: &str =
    include_str!("../../../models/spice/foundation/lib/foundation.lib");

/// The functional parts placed from the Component shelf: RSpice-authored
/// subcircuits (op-amp, comparator, timer, gates, ...) built on the native
/// special-function devices. Resolved by name exactly like the foundation
/// subcircuits, so a deck may instantiate them without an include.
#[doc(hidden)]
pub const FUNCTIONAL_LIB: &str =
    include_str!("../../../models/spice/functional/lib/functional.lib");
