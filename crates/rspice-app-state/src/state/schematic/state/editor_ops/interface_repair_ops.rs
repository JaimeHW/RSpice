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
//! would still emit, or withhold it from one the deck refuses. A Verilog-A
//! master's interface is the one its view states, measured by the comparison
//! its materialization refuses on.

use super::super::super::{
    Component, ComponentType, LibraryCellInstance, PortSpec, SchematicReplacementError,
};
use super::super::*;
use crate::state::{LibraryManager, SymbolResolver};
use rspice_design::library::metadata::metadata_ports;
use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::interface_repair::interface_is_stale;
use std::collections::HashMap;

impl SchematicState {
    /// Whether the one selected instance still presents its master's
    /// interface.
    ///
    /// Every surface that offers the repair asks this, so a disabled row and
    /// the deck's own refusal cannot disagree about which instances are stale.
    pub fn selected_instance_interface_is_stale<S: AsRef<SchematicDocument>>(
        &self,
        libraries: &LibraryManager,
        masters: &HashMap<String, S>,
    ) -> bool {
        let Some((component_id, master_ports)) =
            selected_master_interface(self, libraries, masters)
        else {
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
    /// A review surface lists these, and Update all instances repairs
    /// exactly these, so the two never answer the staleness question
    /// differently.
    pub fn stale_instance_interfaces<S: AsRef<SchematicDocument>>(
        &self,
        libraries: &LibraryManager,
        masters: &HashMap<String, S>,
    ) -> Vec<String> {
        stale_instances(self, libraries, masters)
            .map(|(component, _)| component.name.clone())
            .collect()
    }

    /// Re-bind every stale instance of this drawing to the interface its
    /// master presents now, as one undo step: the repair
    /// [`Self::update_selected_instance_interface`] makes, for each instance
    /// [`Self::stale_instance_interfaces`] lists. The answer is what the
    /// status line says either way.
    pub fn update_all_instance_interfaces<S: AsRef<SchematicDocument>>(
        &mut self,
        libraries: &LibraryManager,
        masters: &HashMap<String, S>,
    ) -> Result<String, String> {
        if self.session.read_only {
            return Err(SchematicReplacementError::ReadOnly.to_string());
        }
        let repairs = stale_instances(self, libraries, masters)
            .map(|(component, ports)| (component.id, ports))
            .collect::<Vec<_>>();
        if repairs.is_empty() {
            return Err(
                "Every instance already presents its master's current interface.".to_owned(),
            );
        }
        let resolver = SymbolResolver::new(libraries, masters);
        let edit = self
            .design
            .update_instance_interfaces(&repairs, |binding| resolver.resolve_binding(binding));
        let summary = edit.value.summary();
        if edit.value.repaired.is_empty() {
            return Err(summary);
        }
        self.finish_document_edit(edit.committed);
        if !edit.committed {
            return Err(SchematicReplacementError::CommitFailed.to_string());
        }
        Ok(summary)
    }

    /// Rebind the selected instance with validation completed before history begins.
    pub fn update_selected_instance_interface<S: AsRef<SchematicDocument>>(
        &mut self,
        libraries: &LibraryManager,
        masters: &HashMap<String, S>,
    ) -> Result<String, SchematicReplacementError> {
        if self.session.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        let (component_id, master_ports) = selected_master_interface(self, libraries, masters)
            .ok_or(SchematicReplacementError::SelectExactlyOneInstance)?;
        let resolver = SymbolResolver::new(libraries, masters);
        let edit =
            self.design
                .update_instance_interface(component_id, &master_ports, |binding| {
                    resolver.resolve_binding(binding)
                })?;
        self.session
            .editor
            .selection
            .select_only_component(component_id);
        self.finish_document_edit(edit.committed);
        if !edit.committed {
            return Err(SchematicReplacementError::CommitFailed);
        }
        Ok(edit.value.summary())
    }
}

/// Every cell instance of the drawing whose master's interface moved on
/// since it was placed, with the interface that master presents now.
fn stale_instances<'a, S: AsRef<SchematicDocument>>(
    schematic: &'a SchematicState,
    libraries: &'a LibraryManager,
    masters: &'a HashMap<String, S>,
) -> impl Iterator<Item = (&'a Component, Vec<PortSpec>)> + 'a {
    schematic
        .design
        .document()
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::CellInstance)
        .filter_map(move |component| {
            let binding = component.library_cell.as_ref()?;
            let ports = master_interface(binding, libraries, masters)?;
            interface_is_stale(binding, &ports).then_some((component, ports))
        })
}

/// The selected instance and the interface its master presents now, when the
/// selection is exactly one cell instance bound to a project schematic or a
/// Verilog-A view. Any other selection has no master interface to be
/// measured against.
fn selected_master_interface<S: AsRef<SchematicDocument>>(
    schematic: &SchematicState,
    libraries: &LibraryManager,
    masters: &HashMap<String, S>,
) -> Option<(u64, Vec<PortSpec>)> {
    let component_id = schematic.session.editor.selection.single_component()?;
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
    master_interface(binding, libraries, masters).map(|ports| (component_id, ports))
}

/// The interface the binding's master presents now: a project schematic's
/// ports, or the ports a Verilog-A view states — which its last clean compile
/// wrote. A binding with neither has nothing to be measured against.
fn master_interface<S: AsRef<SchematicDocument>>(
    binding: &LibraryCellInstance,
    libraries: &LibraryManager,
    masters: &HashMap<String, S>,
) -> Option<Vec<PortSpec>> {
    let ports = match masters.get(&format!(
        "{}/{}/{}",
        binding.library, binding.cell, binding.view
    )) {
        Some(master) => master.as_ref().interface_ports(),
        None => {
            let cell = libraries
                .get_library(&binding.library)?
                .get_cell(&binding.cell)?;
            let view = cell
                .get_view(&binding.view)
                .filter(|view| view.view_type.is_hdl_source())?;
            metadata_ports(&view.metadata).or_else(|| metadata_ports(&cell.metadata))?
        }
    };
    (!ports.is_empty()).then_some(ports)
}

#[cfg(test)]
mod tests;
