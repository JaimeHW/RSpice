//! Repairing an instance whose master's interface moved on without it.
//!
//! A placement records the interface its master presented when it was
//! dropped, and netlist generation refuses to emit an instance whose record
//! no longer matches. The repair is a degenerate replacement: the same
//! master, re-bound to the interface the master presents now. Every terminal
//! the new interface still names is carried across with its wiring; a
//! terminal it no longer names is reported by name and left disconnected,
//! because a repair that guessed a substitute pin would silently rewire the
//! design.
//!
//! Staleness is decided by the comparison netlist generation's typed defect
//! uses, so a surface can never offer this repair for an instance the deck
//! would still emit, or withhold it from one the deck refuses.

use super::super::super::{
    ComponentType, LibraryCellInstance, PortSpec, SchematicReplacementError,
};
use super::super::*;
use crate::state::{LibraryManager, SymbolResolver};
use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::interface_repair::interface_is_stale;
use std::collections::HashMap;

impl SchematicState {
    /// Whether the one selected instance still presents its master's
    /// interface.
    ///
    /// Every surface that offers the repair asks this, so a disabled row and
    /// the deck's own refusal cannot disagree about which instances are stale.
    pub fn selected_instance_interface_is_stale(&self, masters: &HashMap<String, Self>) -> bool {
        let Some((component_id, master_ports)) = selected_master_interface(self, masters) else {
            return false;
        };
        self.design
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .and_then(|component| component.library_cell.as_ref())
            .is_some_and(|binding| interface_is_stale(binding, &master_ports))
    }

    /// Every placed instance whose master's interface moved on since it was
    /// placed, by the reference designator the author reads on the canvas.
    ///
    /// A review surface lists these; the repair itself still acts on the
    /// selection, so the two never answer the staleness question differently.
    pub fn stale_instance_interfaces<S: AsRef<SchematicDocument>>(
        &self,
        masters: &HashMap<String, S>,
    ) -> Vec<String> {
        self.design
            .document()
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter(|component| {
                component.library_cell.as_ref().is_some_and(|binding| {
                    master_interface(binding, masters)
                        .is_some_and(|ports| interface_is_stale(binding, &ports))
                })
            })
            .map(|component| component.name.clone())
            .collect()
    }

    /// Rebind the selected instance with validation completed before history begins.
    pub fn update_selected_instance_interface(
        &mut self,
        libraries: &LibraryManager,
        masters: &HashMap<String, Self>,
    ) -> Result<String, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        let (component_id, master_ports) = selected_master_interface(self, masters)
            .ok_or(SchematicReplacementError::SelectExactlyOneInstance)?;
        let resolver = SymbolResolver::new(libraries, masters);
        let edit =
            self.design
                .update_instance_interface(component_id, &master_ports, |binding| {
                    resolver.resolve_binding(binding)
                })?;
        self.selection.select_only_component(component_id);
        self.finish_document_edit(edit.committed);
        if !edit.committed {
            return Err(SchematicReplacementError::CommitFailed);
        }
        Ok(edit.value.summary())
    }
}

/// The selected instance and the interface its master presents now, when the
/// selection is exactly one cell instance bound to a project schematic. Any
/// other selection has no master interface to be measured against.
fn selected_master_interface(
    schematic: &SchematicState,
    masters: &HashMap<String, SchematicState>,
) -> Option<(u64, Vec<PortSpec>)> {
    let component_id = schematic.selection.single_component()?;
    let component = schematic
        .design
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)?;
    if component.kind != ComponentType::CellInstance {
        return None;
    }
    let binding = component.library_cell.as_ref()?;
    master_interface(binding, masters).map(|ports| (component_id, ports))
}

/// The interface the binding's master presents now, when the master is a
/// project schematic that declares one. A binding with no such master has
/// nothing to be measured against.
fn master_interface<S: AsRef<SchematicDocument>>(
    binding: &LibraryCellInstance,
    masters: &HashMap<String, S>,
) -> Option<Vec<PortSpec>> {
    let master = masters.get(&format!(
        "{}/{}/{}",
        binding.library, binding.cell, binding.view
    ))?;
    let ports = master.as_ref().interface_ports();
    (!ports.is_empty()).then_some(ports)
}

#[cfg(test)]
mod tests;
