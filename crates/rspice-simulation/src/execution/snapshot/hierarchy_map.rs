//! The occurrence map one generation pass produced, and the form the run
//! receipt seals it in.
//!
//! The generator's rows name a design occurrence, the master emitted for it,
//! and the engine scope the deck spells that occurrence with. A receipt keeps
//! the same facts as text it can still read after the path type has moved on,
//! so the conversion happens once, here, on the way into the receipt — and
//! with it the check that an engine prefix really names the occurrence beside
//! it, which is the condition under which the map can be read backwards.

use rspice_results::run_receipt::{HierarchyMapRow, SealedHierarchyMap};

use super::PreparedRunSnapshot;
use crate::netlist_gen::CrossProbeSnapshot;
use crate::netlist_gen::EmissionMap;

impl PreparedRunSnapshot {
    /// The master each occurrence was emitted against; a manual deck has none.
    pub fn emission_map(&self) -> EmissionMap {
        self.cross_probe
            .as_ref()
            .map(|design| design.emission_map.clone())
            .unwrap_or_default()
    }
}

/// The map a design's emission produced, in the form the run receipt seals and checked once; a
/// manual deck has none.
pub(super) fn sealed(design: Option<&CrossProbeSnapshot>) -> Result<SealedHierarchyMap, String> {
    let Some(design) = design else {
        return Ok(SealedHierarchyMap::default());
    };
    let rows = design
        .emission_map
        .iter()
        .map(|row| {
            HierarchyMapRow::new(
                &row.occurrence.to_string(),
                row.master.clone(),
                row.engine_prefix.clone(),
                row.master_reference.clone(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    SealedHierarchyMap::new(rows)
}
