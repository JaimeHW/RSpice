//! Symbol publication effects on placed instances and their attached conductors.

use crate::state::{CellViewRef, Point, SchematicState};
use std::collections::{BTreeMap, HashMap};

impl SchematicState {
    pub(crate) fn remap_symbol_instance_wires(
        &mut self,
        reference: &CellViewRef,
        pin_remaps: &HashMap<String, (Point, Point)>,
    ) -> bool {
        if !rspice_design::symbol::edit::remap_symbol_instance_wires(
            &mut self.document,
            reference,
            pin_remaps,
        ) {
            return false;
        }
        self.is_dirty = true;
        self.bump_topology_version();
        true
    }

    pub(crate) fn rename_instance_terminals(
        &mut self,
        reference: &CellViewRef,
        renames: &BTreeMap<String, String>,
    ) -> usize {
        let renamed = rspice_design::symbol::edit::rename_instance_terminals(
            &mut self.document,
            reference,
            renames,
        );
        if renamed > 0 {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        renamed
    }
}
