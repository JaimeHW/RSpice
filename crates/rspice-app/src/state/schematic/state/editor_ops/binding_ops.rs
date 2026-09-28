//! Specific changes to the persisted identities carried by placed instances.

use crate::state::{LibraryCellInstance, SchematicState};

impl SchematicState {
    pub(crate) fn rename_library_bindings(&mut self, library: &str, new_name: &str) -> usize {
        self.design.rename_library_bindings(library, new_name)
    }

    pub(crate) fn rename_cell_bindings(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> usize {
        self.design.rename_cell_bindings(library, cell, new_name)
    }

    pub(crate) fn rename_view_bindings(
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

    pub(crate) fn migrate_generated_bindings(&mut self) -> (usize, usize) {
        self.design.migrate_generated_bindings()
    }
}

impl SchematicState {
    pub(crate) fn remove_master_placements(
        &mut self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) {
        let repaired = self.design.remove_master_placements(library, cell, view);
        self.repair_clipboard_after_load();
        self.remove_stale_runtime_references(&repaired);
        self.is_dirty = true;
    }

    pub(crate) fn remove_cyclic_master_placements(&mut self, master_key: &str) {
        let repaired = self.design.remove_cyclic_master_placements(master_key);
        self.repair_clipboard_after_load();
        self.remove_stale_runtime_references(&repaired);
    }
}
