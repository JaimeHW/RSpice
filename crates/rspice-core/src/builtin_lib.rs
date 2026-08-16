//! The starter model library compiled into the binary.
//!
//! This is the `rspice-foundation` pack authored in this repository. It is
//! deliberately small and embedded so the same starter models exist on every
//! platform without shipping the third-party development corpus.
//!
//! One source file is shared by library browsing, source mapping and engine
//! fallback resolution, so those consumers cannot drift onto different cards.
//! This module hands out the bytes and nothing more: interpreting them is the
//! parser's job, and `netlist::foundation_subcircuits` does it there.

/// Generic RSpice-authored starter models and the foundation op-amp subcircuit.
pub(crate) const FOUNDATION_LIB: &str =
    include_str!("../../../models/spice/foundation/lib/foundation.lib");
