//! Validated interface repair using the same terminal routing as replacement.

use super::document::SchematicDocument;
use super::replacement::{
    SchematicReplacementError, SchematicReplacementTerminal, SchematicReplacementWireEdit,
};
use super::replacement_edit::{
    TerminalPlacement, build_wire_edits, connected_source_terminals, normalized, terminal_lookup,
    terminal_placements, validate_replacement_geometry,
};
use super::{
    component::{Component, LibraryCellInstance},
    component_type::ComponentType,
    rotation::Rotation,
    wire::WireConnection,
};
use crate::resolved_symbol::ResolvedCellSymbol;
use rspice_design_model::{Point, port::PortSpec};
use std::collections::{HashMap, HashSet};

/// An interface repair whose validated candidate cannot outlive its target borrow.
#[derive(Debug)]
pub struct InstanceInterfaceRepair<'document, 'ports> {
    document: &'document mut SchematicDocument,
    component_id: u64,
    source: Component,
    binding: LibraryCellInstance,
    master_ports: &'ports [PortSpec],
    disconnected: Vec<String>,
    retained: HashSet<String>,
    repaired: Component,
    wire_edits: Vec<SchematicReplacementWireEdit>,
    affected_connections: Vec<WireConnection>,
}

/// The inputs needed to report a committed interface repair.
#[derive(Debug)]
pub struct RepairedInterface<'ports> {
    source: Component,
    binding: LibraryCellInstance,
    master_ports: &'ports [PortSpec],
    disconnected: Vec<String>,
}

impl RepairedInterface<'_> {
    pub fn summary(&self) -> String {
        repair_summary(
            &self.source.name,
            &self.binding,
            self.master_ports,
            &self.disconnected,
        )
    }
}

impl<'document, 'ports> InstanceInterfaceRepair<'document, 'ports> {
    pub fn prepare(
        document: &'document mut SchematicDocument,
        component_id: u64,
        master_ports: &'ports [PortSpec],
        mut resolve_binding: impl FnMut(&LibraryCellInstance) -> Option<ResolvedCellSymbol>,
    ) -> Result<Self, SchematicReplacementError> {
        let source = document
            .components
            .iter()
            .find(|component| component.id == component_id)
            .cloned()
            .ok_or(SchematicReplacementError::SourceInstanceMissing { component_id })?;
        if source.kind != ComponentType::CellInstance || master_ports.is_empty() {
            return Err(SchematicReplacementError::SelectExactlyOneInstance);
        }
        let binding = source.library_cell.clone().ok_or_else(|| {
            SchematicReplacementError::InvalidSourceContract {
                reason: "the selected instance carries no library binding".to_owned(),
            }
        })?;
        if !interface_is_stale(&binding, master_ports) {
            return Err(SchematicReplacementError::NoChanges);
        }

        let mut repaired_binding = binding.clone();
        repaired_binding.bind_interface(master_ports);
        let mut repaired = source.clone();
        repaired.library_cell = Some(repaired_binding.clone());

        let placed_terminals = authored_terminals(&source, resolve_binding(&binding).as_ref());
        let repaired_terminals =
            authored_terminals(&repaired, resolve_binding(&repaired_binding).as_ref());

        let source_placements = terminal_placements(&source, &placed_terminals)?;
        let target_placements = terminal_placements(&repaired, &repaired_terminals)?;
        let target_lookup = terminal_lookup(&repaired_terminals)?;
        let connected = connected_source_terminals(document, &source, &source_placements)?;

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
        validate_replacement_geometry(document, &source, &repaired)?;

        // The wire router is the replacement's, and it refuses a connection
        // whose terminal the new interface does not name. Routing over a
        // candidate that has already dropped those connections is what makes
        // this a repair rather than a refusal.
        let retained = terminal_map.keys().cloned().collect::<HashSet<_>>();
        let mut routed = document.clone();
        routed.connections.retain(|connection| {
            connection.component_id != component_id
                || retained.contains(&normalized(&connection.terminal_name))
        });
        let wire_result = build_wire_edits(&routed, &source, &source_placements, &terminal_map)?;
        let wire_edits = wire_result.edits;
        let affected_connections = wire_result.connections;

        Ok(Self {
            document,
            component_id,
            source,
            binding,
            master_ports,
            disconnected,
            retained,
            repaired,
            wire_edits,
            affected_connections,
        })
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    pub fn commit(self) -> RepairedInterface<'ports> {
        let Self {
            document: state,
            component_id,
            source,
            binding,
            master_ports,
            disconnected,
            retained,
            repaired,
            wire_edits,
            affected_connections,
        } = self;
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
                if let Some(replacement_index) = edit.point_indices.get(connection.point_index) {
                    connection.point_index = *replacement_index;
                }
            }
        }
        RepairedInterface {
            source,
            binding,
            master_ports,
            disconnected,
        }
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

/// The one stale-interface rule, in netlist generation's spelling: a
/// placement that recorded no terminal order predates the interface contract
/// and resolves its order from the master, so it can never be stale.
pub fn interface_is_stale(binding: &LibraryCellInstance, master_ports: &[PortSpec]) -> bool {
    let master_names = master_ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>();
    !binding.terminal_order.is_empty()
        && !same_terminal_contract(&binding.terminal_order, &master_names)
}

/// Whether a placed interface still presents the authoritative one.
///
/// This is the shared answer to that question: the resolver asks it when
/// it materializes a binding, and netlist generation asks it again — through
/// its own typed defect — before it emits an instance. Two spellings of the
/// comparison would let a binding pass materialization and fail emission for
/// reasons that disagree.
pub fn same_terminal_contract<L, R>(placed: &[L], authoritative: &[R]) -> bool
where
    L: AsRef<str>,
    R: AsRef<str>,
{
    placed.len() == authoritative.len()
        && placed
            .iter()
            .zip(authoritative)
            .all(|(left, right)| left.as_ref().eq_ignore_ascii_case(right.as_ref()))
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
