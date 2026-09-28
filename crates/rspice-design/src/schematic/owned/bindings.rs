//! Specific changes to the persisted identities carried by placed instances.

use super::super::component_type::ComponentType;
use super::super::{component::LibraryCellInstance, identity::DocumentRepair};
use super::Schematic;
use crate::library::LibraryCatalog;

impl Schematic {
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

    pub fn rename_library_bindings(&mut self, library: &str, new_name: &str) -> usize {
        self.remap_bindings(|binding| {
            let matched = binding.library == library;
            if matched {
                binding.library = new_name.to_owned();
            }
            matched
        })
    }

    pub fn rename_cell_bindings(&mut self, library: &str, cell: &str, new_name: &str) -> usize {
        self.remap_bindings(|binding| {
            let matched = binding.library == library && binding.cell == cell;
            if matched {
                binding.cell = new_name.to_owned();
            }
            matched
        })
    }

    pub fn rename_view_bindings(
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

    pub fn restore_placement_binding(&mut self, object: u64, expected: &LibraryCellInstance) {
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

    pub fn migrate_generated_bindings(&mut self) -> (usize, usize) {
        use super::super::generated_veriloga_catalog::{
            GeneratedVerilogABindingMigration, migrate_generated_veriloga_binding,
        };
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

impl Schematic {
    pub fn remove_master_placements(
        &mut self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> DocumentRepair {
        self.document.components.retain(|component| {
            !component.library_cell.as_ref().is_some_and(|binding| {
                binding.library == library
                    && binding.cell == cell
                    && view.is_none_or(|view| binding.view == view)
            })
        });
        let repaired = self.repair_document();
        self.invalidate_topology();
        repaired
    }

    pub fn remove_cyclic_master_placements(&mut self, master_key: &str) -> DocumentRepair {
        self.document.components.retain(|component| {
            component.library_cell.as_ref().is_none_or(|binding| {
                rspice_design_model::cell_view::CellViewRef::new(
                    &binding.library,
                    &binding.cell,
                    &binding.view,
                )
                .key()
                    != master_key
            })
        });
        let repaired = self.repair_document();
        self.invalidate_topology();
        repaired
    }
}

impl Schematic {
    pub fn revalidate_instance_bindings(&mut self, libraries: &LibraryCatalog) -> Vec<String> {
        let mut missing_masters = Vec::new();
        for component in &mut self.document.components {
            if component.kind != ComponentType::CellInstance {
                continue;
            }
            let Some(binding) = component.library_cell.as_mut() else {
                continue;
            };
            if binding.is_executable_builtin() {
                continue;
            }
            let Some(library) = libraries.get_library(&binding.library) else {
                continue;
            };
            if library.get_cell(&binding.cell).is_some() {
                continue;
            }
            binding.module_name = None;
            binding.source_path = None;
            binding.netlist_template = None;
            binding.parameter_order.clear();
            missing_masters.push(format!(
                "{}/{}/{}",
                binding.library, binding.cell, binding.view
            ));
        }
        missing_masters.sort_unstable();
        missing_masters.dedup();
        missing_masters
    }
}
