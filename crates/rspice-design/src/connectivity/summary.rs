//! Connectivity summaries shared by design inspection, publication and simulation preparation.

use super::{
    deck_bit_name, extract_with_hierarchy, net_at_schematic_point,
    terminal_positions_with_hierarchy,
};
use crate::hierarchy::HierarchySource;
use crate::library::metadata::{
    VERILOGA_PORTS_KEY, decode_source_view_ports, module_interface_view,
};
use crate::schematic::component::{BuiltinXspicePortType, Component, LibraryCellInstance};
use crate::schematic::document::SchematicDocument;
use rspice_core_base::event_net::{NetDomain, NetEndpoints, PortCarrier};
use rspice_design_model::port::PortDirection;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Electrical class of a design net.
///
/// The class is derived from declared design intent — the resolved ground
/// net, a declared `dir=supply` interface port, or a power net label — and
/// never guessed from the spelling of a name alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetClass {
    /// The simulation reference node.
    Ground,
    /// A declared supply/power rail.
    Supply,
    /// Any other conductor.
    Signal,
}

impl NetClass {
    /// Lower-case vocabulary shared with the inspector and the navigator.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Ground => "ground",
            Self::Supply => "supply",
            Self::Signal => "signal",
        }
    }
}

/// One component terminal bound to a net.
#[derive(Debug, Clone)]
pub struct NetTerminal {
    /// Owning component, for selection and cross-probing.
    pub component_id: u64,
    /// Instance reference designator as drawn.
    pub reference: String,
    /// Terminal name on that instance.
    pub pin: String,
}

/// One electrical net of the open schematic, summarized for navigation:
/// the rail's Nets segment, the net inspector, cross-probe highlighting,
/// search.
#[derive(Debug, Clone)]
pub struct DesignNet {
    /// SPICE name ("0", a label/port name, or autonamed `netN`).
    pub name: String,
    /// `true` when the name comes from an authored label, interface port, or
    /// ground symbol rather than the netlister's isolated-node fallback.
    pub authored_name: bool,
    /// Electrical class of the conductor.
    pub class: NetClass,
    /// Component terminals on this net, in document order.
    pub terminals: Vec<NetTerminal>,
    /// Declared direction when the net is an interface port of the cell.
    pub port: Option<PortDirection>,
    /// Wires belonging to this net (for canvas highlighting).
    pub wire_ids: Vec<u64>,
    /// What the net carries at run: electrical, logic, real, or a mix the
    /// engine converts. Read from the engine's own rule over what each pin
    /// on it carries, so it is known before anything has run.
    pub domain: NetDomain,
}

impl DesignNet {
    /// Component terminals bound to this net.
    pub fn pin_count(&self) -> usize {
        self.terminals.len()
    }

    /// `true` when the net is an interface port of the cell.
    pub const fn is_port(&self) -> bool {
        self.port.is_some()
    }
}

/// Live net summary: connectivity + ports + labels + ground, no instance
/// generation. Cheap enough to recompute on topology change; callers cache
/// by `topology_version`.
pub fn design_nets(schematic: &impl AsRef<SchematicDocument>) -> Vec<DesignNet> {
    let schematic = schematic.as_ref();
    collect_design_nets(schematic, None)
}

/// Live net summary with project hierarchy/symbol resolution enabled.
pub fn design_nets_with_hierarchy(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
) -> Vec<DesignNet> {
    let schematic = schematic.as_ref();
    collect_design_nets(schematic, Some(hierarchy))
}

/// Net summary of one cell view of a frozen design projection, extracted once
/// and then retained by the projection itself.
///
/// The projection keeps the slot type-erased so the design model never names
/// a generator type; the downcast back to [`DesignNet`] belongs here, where
/// the type is owned. A cell view the projection does not carry has no nets
/// rather than an error: the projection is the authority on which views exist.
pub fn projection_nets(
    catalog: &crate::library::LibraryCatalog,
    projection: &crate::projection::DesignProjection,
    cell_view_key: &str,
) -> Arc<Vec<DesignNet>> {
    let extract = || -> Arc<Vec<DesignNet>> {
        Arc::new(
            projection
                .schematic_buffers()
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(cell_view_key))
                .map(|(_, schematic)| {
                    design_nets_with_hierarchy(
                        schematic,
                        &HierarchySource::from_design_projection(catalog, projection),
                    )
                })
                .unwrap_or_default(),
        )
    };
    projection
        .memo_nets(cell_view_key, || extract() as Arc<dyn Any + Send + Sync>)
        .downcast::<Vec<DesignNet>>()
        .unwrap_or_else(|_| extract())
}

/// Resolved terminal names for every placed component, in the same order and
/// with the same authored-symbol authority used by hierarchical netlisting.
///
/// Publication uses this alongside the hierarchy-aware net summary so
/// disconnected pins remain visible without inventing names or falling back
/// to generic pin numbers when an authored symbol is available.
pub fn component_pin_names_with_hierarchy(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: &HierarchySource<'_>,
) -> HashMap<u64, Vec<String>> {
    let schematic = schematic.as_ref();
    schematic
        .components
        .iter()
        .map(|component| {
            (
                component.id,
                terminal_positions_with_hierarchy(component, Some(hierarchy))
                    .into_iter()
                    .map(|(name, _)| name)
                    .collect(),
            )
        })
        .collect()
}

fn collect_design_nets(
    schematic: &SchematicDocument,
    hierarchy: Option<&HierarchySource<'_>>,
) -> Vec<DesignNet> {
    let super::ExtractedConnectivity {
        nets: extracted_nets,
        point_to_net,
        ground_net,
        vector_nets,
        ..
    } = extract_with_hierarchy(schematic, hierarchy);
    let net_at = |point| net_at_schematic_point(schematic, &extracted_nets, &point_to_net, point);

    let ports: HashMap<String, PortDirection> = schematic
        .interface_ports()
        .iter()
        .map(|port| (port.name.to_ascii_lowercase(), port.direction))
        .collect();

    // Terminals are collected in document order so the inspector's
    // connectivity table reads the same way twice for the same drawing.
    let mut terminals: HashMap<usize, Vec<NetTerminal>> = HashMap::new();
    let mut endpoints: HashMap<usize, NetEndpoints> = HashMap::new();
    // What the pin at each vector terminal carries, for the bits of its bus.
    let mut vector_pins = HashMap::new();
    let vector_terminals: HashSet<_> = vector_nets
        .nets
        .iter()
        .flat_map(|vector| vector.terminals.iter().copied())
        .collect();
    // A module's ports are read once per master, however often it is placed.
    let mut module_carriers: HashMap<String, PinCarriers> = HashMap::new();
    let no_carriers = PinCarriers::new();
    for component in &schematic.components {
        // A ground symbol or an interface port is a name for a net, not a
        // conductor on it.
        let conductor = !component.kind.spice_prefix().is_empty();
        let contract;
        let carriers = match component.library_cell.as_ref() {
            Some(binding) if binding.builtin_xspice.is_some() => {
                contract = code_model_carriers(binding);
                &contract
            }
            Some(binding) => module_carriers
                .entry(HierarchySource::placed_master(binding).key())
                .or_insert_with(|| module_port_carriers(binding, hierarchy)),
            None => &no_carriers,
        };
        for (pin, position) in terminal_positions_with_hierarchy(component, hierarchy) {
            let (carrier, module_port) = carriers
                .get(&pin.to_ascii_lowercase())
                .copied()
                .unwrap_or((PortCarrier::Electrical, false));
            let on_bus = vector_terminals.contains(&position);
            if conductor && on_bus {
                vector_pins.insert(position, (carrier, module_port));
            }
            if let Some(net) = net_at(position) {
                if conductor && !on_bus {
                    endpoints
                        .entry(net.id)
                        .or_default()
                        .add(carrier, module_port);
                }
                terminals.entry(net.id).or_default().push(NetTerminal {
                    component_id: component.id,
                    reference: component.name.clone(),
                    pin,
                });
            }
        }
    }
    // A vector pin puts one conductor on every bit of the bus it meets.
    let net_ids: HashMap<String, usize> = extracted_nets
        .iter()
        .map(|net| (net.spice_name().to_ascii_lowercase(), net.id))
        .collect();
    for vector in &vector_nets.nets {
        for point in &vector.terminals {
            let Some(&(carrier, module_port)) = vector_pins.get(point) else {
                continue;
            };
            for member in vector.declaration.members() {
                let name = deck_bit_name(&vector.declaration.name, member.index);
                if let Some(id) = net_ids.get(&name.to_ascii_lowercase()) {
                    endpoints.entry(*id).or_default().add(carrier, module_port);
                }
            }
        }
    }
    // A power label anywhere on the net declares it a supply rail.
    let mut power_labelled: HashSet<usize> = HashSet::new();
    for label in &schematic.net_labels {
        if label.is_power_net()
            && !label.is_ground()
            && let Some(net) = net_at(label.pos)
        {
            power_labelled.insert(net.id);
        }
    }

    let mut nets: Vec<DesignNet> = extracted_nets
        .iter()
        .map(|net| {
            let name = net.spice_name();
            let authored_name = net.label.is_some();
            let port = ports.get(&name.to_ascii_lowercase()).copied();
            let ground = ground_net == Some(net.id) || name == "0";
            let mut domain = endpoints.remove(&net.id).unwrap_or_default();
            domain.ground = ground;
            let class = if ground {
                NetClass::Ground
            } else if port == Some(PortDirection::Supply) || power_labelled.contains(&net.id) {
                NetClass::Supply
            } else {
                NetClass::Signal
            };
            DesignNet {
                authored_name,
                class,
                port,
                terminals: terminals.remove(&net.id).unwrap_or_default(),
                wire_ids: net.wires.clone(),
                domain: domain.domain(),
                name,
            }
        })
        .collect();

    // Reading order: interface ports, then named nets, then autonamed.
    let autonamed = |name: &str| {
        name.strip_prefix("net")
            .is_some_and(|n| n.chars().all(|c| c.is_ascii_digit()))
    };
    nets.sort_by(|a, b| {
        (
            !a.is_port(),
            autonamed(&a.name),
            a.name.to_ascii_lowercase(),
        )
            .cmp(&(
                !b.is_port(),
                autonamed(&b.name),
                b.name.to_ascii_lowercase(),
            ))
    });
    nets
}

/// What each pin of a placed component carries, by folded pin name, with
/// whether it is a module port. A pin it does not name is electrical.
type PinCarriers = HashMap<String, (PortCarrier, bool)>;

/// A built-in code model's event pins, as its frozen contract types them.
fn code_model_carriers(binding: &LibraryCellInstance) -> PinCarriers {
    let mut carriers = PinCarriers::new();
    for port in binding
        .builtin_xspice
        .iter()
        .flat_map(|contract| &contract.ports)
    {
        let carrier = match port.port_type {
            BuiltinXspicePortType::Digital => PortCarrier::Logic,
            BuiltinXspicePortType::Real => PortCarrier::Real,
            _ => continue,
        };
        for pin in port
            .terminals
            .iter()
            .filter_map(|terminal| binding.terminal_order.get(*terminal))
        {
            carriers.insert(pin.to_ascii_lowercase(), (carrier, false));
        }
    }
    carriers
}

/// A module's pins, as the last clean compile of the HDL view it binds, or
/// that defines its cell's ports, stated them.
fn module_port_carriers(
    binding: &LibraryCellInstance,
    hierarchy: Option<&HierarchySource<'_>>,
) -> PinCarriers {
    let mut carriers = PinCarriers::new();
    let Some((cell, view)) = hierarchy
        .and_then(|source| source.cell_declaration(&HierarchySource::placed_master(binding)))
    else {
        return carriers;
    };
    let ports = view
        .filter(|view| view.view_type.is_hdl_source())
        .or_else(|| module_interface_view(cell))
        .and_then(|view| view.metadata.get(VERILOGA_PORTS_KEY))
        .and_then(|encoded| decode_source_view_ports(encoded));
    for port in ports.into_iter().flatten() {
        carriers.insert(port.name.to_ascii_lowercase(), (port.carrier, true));
    }
    carriers
}

/// Net name for every terminal in the design, keyed by instance and pin.
pub fn net_names_by_terminal(
    schematic: &impl AsRef<SchematicDocument>,
) -> HashMap<(u64, String), String> {
    keyed_by_terminal(design_nets(schematic))
}

pub fn keyed_by_terminal(
    nets: impl IntoIterator<Item = DesignNet>,
) -> HashMap<(u64, String), String> {
    nets.into_iter()
        .flat_map(|net| {
            let name = net.name.clone();
            net.terminals
                .into_iter()
                .map(move |terminal| ((terminal.component_id, terminal.pin), name.clone()))
        })
        .collect()
}

/// This source's nets, in the order its pins are declared.
///
/// A terminal with no net is reported as unconnected rather than skipped: a
/// source driving nothing is exactly what a reader scanning this list is
/// looking for, and dropping the entry would hide it.
pub fn terminal_nets(component: &Component, nets: &HashMap<(u64, String), String>) -> Vec<String> {
    component
        .terminal_positions()
        .into_iter()
        .map(|(pin, _)| {
            nets.get(&(component.id, pin.to_owned()))
                .cloned()
                .unwrap_or_else(|| "unconnected".to_owned())
        })
        .collect()
}

#[cfg(test)]
mod tests;
