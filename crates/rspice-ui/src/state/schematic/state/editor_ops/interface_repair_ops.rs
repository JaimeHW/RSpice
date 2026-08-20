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

use std::collections::{HashMap, HashSet};

use crate::state::workspace::same_terminal_contract;
use crate::state::{LibraryManager, ResolvedCellSymbol, SymbolResolver};

use super::super::super::{
    Component, ComponentType, LibraryCellInstance, Point, PortSpec, Rotation,
    SchematicReplacementError, SchematicReplacementTerminal,
};
use super::super::*;
use super::replace_ops::{
    TerminalPlacement, build_wire_edits, connected_source_terminals, normalized, terminal_lookup,
    terminal_placements, validate_replacement_geometry,
};

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
        self.components
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
    pub fn stale_instance_interfaces(&self, masters: &HashMap<String, Self>) -> Vec<String> {
        self.components
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

    /// Re-bind the selected instance to its master's current interface in one
    /// undo step.
    ///
    /// The summary is the operation's own account of what it did, so every
    /// surface that offers the repair reports the same outcome — including
    /// the name of any connected terminal the new interface dropped, which is
    /// the one thing the author has to decide about afterwards.
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
        let source = self
            .components
            .iter()
            .find(|component| component.id == component_id)
            .cloned()
            .ok_or(SchematicReplacementError::SourceInstanceMissing { component_id })?;
        let binding = source.library_cell.clone().ok_or_else(|| {
            SchematicReplacementError::InvalidSourceContract {
                reason: "the selected instance carries no library binding".to_owned(),
            }
        })?;
        if !interface_is_stale(&binding, &master_ports) {
            return Err(SchematicReplacementError::NoChanges);
        }

        let mut repaired_binding = binding.clone();
        repaired_binding.bind_interface(&master_ports);
        let mut repaired = source.clone();
        repaired.library_cell = Some(repaired_binding.clone());

        let resolver = SymbolResolver::new(libraries, masters);
        let placed_terminals =
            authored_terminals(&source, resolver.resolve_binding(&binding).as_ref());
        let repaired_terminals = authored_terminals(
            &repaired,
            resolver.resolve_binding(&repaired_binding).as_ref(),
        );

        let source_placements = terminal_placements(&source, &placed_terminals)?;
        let target_placements = terminal_placements(&repaired, &repaired_terminals)?;
        let target_lookup = terminal_lookup(&repaired_terminals)?;
        let connected = connected_source_terminals(self, &source, &source_placements)?;

        // Name first, then any alias the new interface records for a pin —
        // the replacement's own lookup, so the two operations resolve a
        // terminal identically.
        let mut terminal_map = HashMap::<String, &TerminalPlacement<'_>>::new();
        let mut claimed = HashSet::new();
        let mut disconnected = Vec::new();
        for placement in &source_placements {
            let key = normalized(placement.terminal.name());
            let Some((index, _)) = target_lookup.get(&key).copied() else {
                if connected.contains(&key) {
                    disconnected.push(placement.terminal.name().to_owned());
                }
                continue;
            };
            if !claimed.insert(index) {
                return Err(SchematicReplacementError::AmbiguousTerminalAlias { name: key });
            }
            let target = target_placements.get(index).ok_or_else(|| {
                SchematicReplacementError::InvalidTargetContract {
                    reason: "terminal placement index is inconsistent".to_owned(),
                }
            })?;
            terminal_map.insert(key, target);
        }

        // A replacement refuses a target pin that lands on wiring it did not
        // already own; a repair must not, because the wiring it lands on is
        // the author's own drawing for the pin that stood there. The body may
        // still not collide with another instance, so the geometry check
        // stays.
        validate_replacement_geometry(self, &source, &repaired)?;

        // The wire router is the replacement's, and it refuses a connection
        // whose terminal the new interface does not name. Routing over a
        // candidate that has already dropped those connections is what makes
        // this a repair rather than a refusal.
        let retained = terminal_map.keys().cloned().collect::<HashSet<_>>();
        let mut routed = self.clone();
        routed.connections.retain(|connection| {
            connection.component_id != component_id
                || retained.contains(&normalized(&connection.terminal_name))
        });
        let wire_result = build_wire_edits(&routed, &source, &source_placements, &terminal_map)?;
        let wire_edits = wire_result.edits;
        let affected_connections = wire_result.connections;

        let committed = self.with_undo("update instance interface", move |state| {
            if let Some(component) = state
                .components
                .iter_mut()
                .find(|component| component.id == component_id)
            {
                *component = repaired;
            }
            state.connections.retain(|connection| {
                connection.component_id != component_id
                    || retained.contains(&normalized(&connection.terminal_name))
            });
            for replacement_connection in affected_connections {
                if let Some(connection) = state.connections.iter_mut().find(|connection| {
                    connection.component_id == component_id
                        && connection.wire_id == replacement_connection.wire_id
                        && connection.point_index == replacement_connection.point_index
                }) {
                    connection.terminal_name = replacement_connection.terminal_name;
                }
            }
            for edit in wire_edits {
                if let Some(wire) = state.wires.iter_mut().find(|wire| wire.id == edit.wire_id) {
                    wire.points = edit.replacement_points;
                }
                for connection in state
                    .connections
                    .iter_mut()
                    .filter(|connection| connection.wire_id == edit.wire_id)
                {
                    if let Some(replacement_index) = edit.point_indices.get(connection.point_index)
                    {
                        connection.point_index = *replacement_index;
                    }
                }
            }
            state.selection.select_only_component(component_id);
            state.is_dirty = true;
            state.bump_topology_version();
        });
        if !committed {
            return Err(SchematicReplacementError::CommitFailed);
        }
        Ok(repair_summary(
            &source.name,
            &binding,
            &master_ports,
            &disconnected,
        ))
    }
}

/// What the repair did, in one sentence.
fn repair_summary(
    instance: &str,
    binding: &LibraryCellInstance,
    master_ports: &[PortSpec],
    disconnected: &[String],
) -> String {
    let interface = master_ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let mut summary = format!(
        "{instance} now presents the current {}/{} interface ({interface})",
        binding.library, binding.cell
    );
    if !disconnected.is_empty() {
        summary.push_str(&format!(
            "; the master no longer names {}, so that wiring was left in place and disconnected",
            disconnected.join(", ")
        ));
    }
    summary
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
fn master_interface(
    binding: &LibraryCellInstance,
    masters: &HashMap<String, SchematicState>,
) -> Option<Vec<PortSpec>> {
    let master = masters.get(&format!(
        "{}/{}/{}",
        binding.library, binding.cell, binding.view
    ))?;
    let ports = master.interface_ports();
    (!ports.is_empty()).then_some(ports)
}

/// The one stale-interface rule, in netlist generation's spelling: a
/// placement that recorded no terminal order predates the interface contract
/// and resolves its order from the master, so it can never be stale.
fn interface_is_stale(binding: &LibraryCellInstance, master_ports: &[PortSpec]) -> bool {
    let master_names = master_ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    !binding.terminal_order.is_empty()
        && !same_terminal_contract(&binding.terminal_order, &master_names)
}

/// One binding's pin layout as unrotated offsets, in the order its resolved
/// symbol presents them.
fn authored_terminals(
    component: &Component,
    symbol: Option<&ResolvedCellSymbol>,
) -> Vec<SchematicReplacementTerminal> {
    let mut template = component.clone();
    template.pos = Point::origin();
    template.rotation = Rotation::R0;
    template.mirror_h = false;
    template.mirror_v = false;
    template
        .terminal_positions_resolved(symbol)
        .into_iter()
        .map(|(name, offset)| {
            let direction = template.library_cell.as_ref().and_then(|binding| {
                binding
                    .terminal_order
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(&name))
                    .and_then(|index| binding.terminal_dirs.get(index))
                    .copied()
            });
            let terminal = SchematicReplacementTerminal::new(name, offset);
            match direction {
                Some(direction) => terminal.with_direction(direction),
                None => terminal,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
