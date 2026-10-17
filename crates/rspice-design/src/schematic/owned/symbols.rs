//! Symbol publication effects on placed instances and their attached conductors.

use super::Schematic;
use crate::symbol::edit::SymbolInstanceWiring;
use rspice_design_model::{Point, cell_view::CellViewRef};
use std::collections::{BTreeMap, HashMap};

impl Schematic {
    /// Move the wire vertices the wiring holds to where their terminals now
    /// stand. History is the caller's: a symbol edit reaches each parent as
    /// one undo record spanning the whole edit, which no single frame of it
    /// can take on its own.
    pub fn apply_symbol_instance_wiring(
        &mut self,
        wiring: &SymbolInstanceWiring,
        pin_remaps: &HashMap<String, (Point, Point)>,
    ) -> bool {
        if !crate::symbol::edit::apply_symbol_instance_wiring(
            &mut self.document,
            wiring,
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
