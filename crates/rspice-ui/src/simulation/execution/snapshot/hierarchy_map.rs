//! The occurrence map one generation pass produced, and the form the run
//! receipt seals it in.
//!
//! The generator's rows name a design occurrence, the master emitted for it,
//! and the engine scope the deck spells that occurrence with. A receipt keeps
//! the same facts as text it can still read after the path type has moved on,
//! so the conversion happens once, here, on the way into the receipt — and
//! with it the check that an engine prefix really names the occurrence beside
//! it, which is the condition under which the map can be read backwards.

use crate::state::HierarchyMapRow;

use super::{EmissionRow, PreparedRunSnapshot};

impl PreparedRunSnapshot {
    /// The master each occurrence was emitted against; a manual deck has none.
    pub(in crate::simulation) fn emission_map(&self) -> &[EmissionRow] {
        self.cross_probe
            .as_ref()
            .map_or(&[], |design| design.emission_map.as_slice())
    }

    /// The same map in the form the run receipt seals.
    pub(in crate::simulation) fn receipt_hierarchy_map(
        &self,
    ) -> Result<Vec<HierarchyMapRow>, String> {
        self.emission_map()
            .iter()
            .map(|row| {
                let occurrence = row.occurrence.to_string();
                HierarchyMapRow::new(
                    &occurrence,
                    row.master.clone(),
                    row.engine_prefix.clone(),
                    row.master_reference.clone(),
                )
            })
            .collect()
    }
}
