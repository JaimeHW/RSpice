//! Symbol publication effects on placed instances and their attached conductors.

use super::Schematic;
use rspice_design_model::{Point, cell_view::CellViewRef};
use std::collections::{BTreeMap, HashMap};

impl Schematic {
    pub fn remap_symbol_instance_wires(
        &mut self,
        reference: &CellViewRef,
        pin_remaps: &HashMap<String, (Point, Point)>,
    ) -> bool {
        if !crate::symbol::edit::remap_symbol_instance_wires(
            &mut self.document,
            reference,
            pin_remaps,
        ) {
            return false;
        }
        self.invalidate_topology();
        true
    }

    pub fn rename_instance_terminals(
        &mut self,
        reference: &CellViewRef,
        renames: &BTreeMap<String, String>,
    ) -> usize {
        let renamed =
            crate::symbol::edit::rename_instance_terminals(&mut self.document, reference, renames);
        if renamed > 0 {
            self.invalidate_topology();
        }
        renamed
    }
}
