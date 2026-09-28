//! Specific changes to the persisted identities carried by placed instances.

use crate::state::{LibraryCellInstance, SchematicState};

impl SchematicState {
    fn remap_bindings(&mut self, rebind: impl Fn(&mut LibraryCellInstance) -> bool) -> usize {
        let mut remapped = 0;
        for component in &mut self.document.components {
            if let Some(binding) = component.library_cell.as_mut()
                && rebind(binding)
            {
                remapped += 1;
            }
        }
        remapped
    }

    pub(crate) fn rename_library_bindings(&mut self, library: &str, new_name: &str) -> usize {
        self.remap_bindings(|binding| {
            let matched = binding.library == library;
            if matched {
                binding.library = new_name.to_owned();
            }
            matched
        })
    }

    pub(crate) fn rename_cell_bindings(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> usize {
        self.remap_bindings(|binding| {
            let matched = binding.library == library && binding.cell == cell;
            if matched {
                binding.cell = new_name.to_owned();
            }
            matched
        })
    }

    pub(crate) fn rename_view_bindings(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> usize {
        self.remap_bindings(|binding| {
            let matched =
                binding.library == library && binding.cell == cell && binding.view == view;
            if matched {
                binding.view = new_name.to_owned();
            }
            matched
        })
    }

    pub(crate) fn restore_placement_binding(
        &mut self,
        object: u64,
        expected: &LibraryCellInstance,
    ) {
        let Some(binding) = self
            .document
            .components
            .iter_mut()
            .find(|component| component.id == object)
            .and_then(|component| component.library_cell.as_mut())
        else {
            return;
        };
        if binding.library == expected.library
            && binding.cell == expected.cell
            && binding.view == expected.view
        {
            *binding = expected.clone();
        }
    }

    pub(crate) fn migrate_generated_bindings(&mut self) -> (usize, usize) {
        use crate::state::{GeneratedVerilogABindingMigration, migrate_generated_veriloga_binding};
        let mut migrated_generated_bindings = 0usize;
        let mut unresolved_generated_bindings = 0usize;
        for component in &mut self.document.components {
            let Some(binding) = component.library_cell.as_mut() else {
                continue;
            };
            if binding.generated_veriloga.is_none() {
                continue;
            }
            match migrate_generated_veriloga_binding(binding) {
                GeneratedVerilogABindingMigration::Current => {}
                GeneratedVerilogABindingMigration::Migrated => {
                    migrated_generated_bindings += 1;
                }
                GeneratedVerilogABindingMigration::Unresolved(_reason) => {
                    unresolved_generated_bindings += 1;
                }
            }
        }
        (migrated_generated_bindings, unresolved_generated_bindings)
    }
}

impl SchematicState {
    pub(crate) fn remove_master_placements(
        &mut self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) {
        self.document.components.retain(|component| {
            !component.library_cell.as_ref().is_some_and(|binding| {
                binding.library == library
                    && binding.cell == cell
                    && view.is_none_or(|view| binding.view == view)
            })
        });
        self.recalculate_runtime_state();
        self.bump_topology_version();
        self.is_dirty = true;
    }

    pub(crate) fn remove_cyclic_master_placements(&mut self, master_key: &str) {
        self.document.components.retain(|component| {
            component.library_cell.as_ref().is_none_or(|binding| {
                crate::state::CellViewRef::new(&binding.library, &binding.cell, &binding.view).key()
                    != master_key
            })
        });
        self.recalculate_runtime_state();
        self.bump_topology_version();
    }
}
