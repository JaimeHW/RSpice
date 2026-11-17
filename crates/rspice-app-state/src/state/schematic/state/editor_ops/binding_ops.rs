//! Specific changes to the persisted identities carried by placed instances.

use crate::state::{LibraryCellInstance, SchematicState};

impl SchematicState {
    pub fn rename_library_bindings(&mut self, library: &str, new_name: &str) -> usize {
        self.design.rename_library_bindings(library, new_name)
    }

    pub fn rename_cell_bindings(&mut self, library: &str, cell: &str, new_name: &str) -> usize {
        self.design.rename_cell_bindings(library, cell, new_name)
    }

    /// Bind every placement of the cell `from` names to the cell `to` names,
    /// each keeping its view.
    pub fn rebind_cell_bindings(&mut self, from: (&str, &str), to: (&str, &str)) -> usize {
        self.design.rebind_cell_bindings(from, to)
    }

    pub fn rename_view_bindings(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> usize {
        self.design
            .rename_view_bindings(library, cell, view, new_name)
    }

    pub(crate) fn restore_placement_binding(
        &mut self,
        object: u64,
        expected: &LibraryCellInstance,
    ) {
        self.design.restore_placement_binding(object, expected)
    }
}

impl SchematicState {
    pub fn remove_master_placements(&mut self, library: &str, cell: &str, view: Option<&str>) {
        let repaired = self.design.remove_master_placements(library, cell, view);
        self.remove_stale_runtime_references(&repaired);
        self.session.is_dirty = true;
    }
}
