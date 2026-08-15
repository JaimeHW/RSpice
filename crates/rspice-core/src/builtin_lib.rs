//! The starter model library compiled into the binary.
//!
//! This is the `rspice-foundation` pack authored in this repository. It is
//! deliberately small and embedded so the same starter models exist on every
//! platform without shipping the third-party development corpus.
//!
//! One source file is shared by library browsing, source mapping and engine
//! fallback resolution, so those consumers cannot drift onto different cards.

use std::sync::OnceLock;

use crate::netlist::SubcircuitDef;

/// Generic RSpice-authored starter models and the foundation op-amp subcircuit.
pub(crate) const FOUNDATION_LIB: &str =
    include_str!("../../../models/spice/foundation/lib/foundation.lib");

/// Parsed fallback subcircuits supplied by the foundation library.
pub(crate) fn foundation_subcircuits() -> &'static [SubcircuitDef] {
    static SUBCIRCUITS: OnceLock<Vec<SubcircuitDef>> = OnceLock::new();
    SUBCIRCUITS.get_or_init(|| match crate::netlist::parse_netlist(FOUNDATION_LIB) {
        Ok(netlist) => netlist.subcircuits,
        Err(error) => {
            log::error!("embedded RSpice foundation library did not parse: {error}");
            Vec::new()
        }
    })
}
