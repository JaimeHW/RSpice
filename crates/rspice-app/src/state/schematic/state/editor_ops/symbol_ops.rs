//! Symbol publication effects on placed instances and their attached conductors.

use crate::state::{CellViewRef, Point, SchematicState};
use rspice_design::symbol::edit::SymbolInstanceWiring;
use std::collections::{BTreeMap, HashMap};

impl SchematicState {
    pub(crate) fn apply_symbol_instance_wiring(
        &mut self,
        wiring: &SymbolInstanceWiring,
        pin_remaps: &HashMap<String, (Point, Point)>,
    ) -> bool {
        let changed = self.design.apply_symbol_instance_wiring(wiring, pin_remaps);
        if changed {
            self.session.is_dirty = true;
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
            self.session.is_dirty = true;
        }
        renamed
    }
}
