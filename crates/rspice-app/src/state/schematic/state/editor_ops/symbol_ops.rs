//! Symbol publication effects on placed instances and their attached conductors.

use crate::state::{CellViewRef, Point, SchematicState};
use std::collections::{BTreeMap, HashMap};

impl SchematicState {
    pub(crate) fn remap_symbol_instance_wires(
        &mut self,
        reference: &CellViewRef,
        pin_remaps: &HashMap<String, (Point, Point)>,
    ) -> bool {
        let changed = self
            .design
            .remap_symbol_instance_wires(reference, pin_remaps);
        if changed {
            self.is_dirty = true;
        }
        changed
    }

    pub(crate) fn rename_instance_terminals(
        &mut self,
        reference: &CellViewRef,
        renames: &BTreeMap<String, String>,
    ) -> usize {
        let renamed = self.design.rename_instance_terminals(reference, renames);
        if renamed > 0 {
            self.is_dirty = true;
        }
        renamed
    }
}
