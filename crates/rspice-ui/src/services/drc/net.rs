//! The per-node facts the rules judge.
//!
//! Folded from the design's one connectivity extraction and the terminals bound
//! to it. There is deliberately no tracing here: a disjoint-set over the
//! drawing's geometry used to live in this module, and its only job was to
//! derive connectivity a second time. Two derivations disagree — over a
//! terminal that sits mid-segment, over which label wins a net — and a checker
//! that disagrees with the netlister signs off on a circuit the engine never
//! runs.

use std::collections::{HashMap, HashSet};

use super::checker::component_identity;
use super::input::ComponentInfo;
use super::netlist_gen::extraction::ExtractedConnectivity;

/// What the rules know about one electrical node.
#[derive(Debug, Clone, Default)]
pub(super) struct NetInfo {
    /// Placed terminals on this node.
    pub(super) connection_count: usize,
    /// How many of those terminals belong to drawn ground symbols. A ground
    /// symbol that reaches nothing else grounds nothing.
    pub(super) ground_symbol_count: usize,
    pub(super) has_voltage_source: bool,
    pub(super) has_current_source: bool,
    pub(super) is_ground: bool,
    pub(super) connected_components: Vec<String>,
    pub(super) output_drivers: HashSet<String>,
    pub(super) voltage_source_drivers: HashSet<String>,
    pub(super) declared_output_drivers: HashSet<String>,
    pub(super) wire_ids: Vec<u64>,
    /// A typed bus tap binds this node: a real electrical endpoint that no
    /// placed terminal represents.
    pub(super) tapped: bool,
    /// One conductor of a declared vector, whether written as a member name or
    /// projected from a bus.
    pub(super) bus_member: bool,
    /// The node name as the drawing spells it, which is the one a finding
    /// quotes. It differs from the key only for a projected vector bit.
    pub(super) display_name: String,
}

/// Fold the extraction and its terminals into one entry per node name.
///
/// Nodes are keyed by the name the deck writes, because that is the identity
/// every finding quotes and the identity the engine will use. Two extracted
/// nets that resolve to one deck name — separate ground symbols, for instance —
/// are therefore one entry, which is what the simulation will have.
pub(super) fn fold_nets(
    components: &[ComponentInfo],
    connectivity: &ExtractedConnectivity,
) -> HashMap<String, NetInfo> {
    let mut nets: HashMap<String, NetInfo> = HashMap::new();
    for net in &connectivity.nets {
        let name = net.spice_name();
        let tapped = connectivity.tapped_nets.contains(&net.id);
        let entry = nets.entry(name.clone()).or_default();
        entry.is_ground |= crate::state::is_ground_reference(&name);
        entry.wire_ids.extend(net.wires.iter().copied());
        entry.tapped |= tapped;
    }

    for component in components {
        let identity = component_identity(component);
        for pin in &component.pins {
            let entry = nets.entry(pin.net_name.clone()).or_default();
            entry.is_ground |= crate::state::is_ground_reference(&pin.net_name);
            entry.connection_count += 1;
            if component.is_ground_symbol {
                entry.ground_symbol_count += 1;
            }
            entry.has_voltage_source |= component.is_voltage_source;
            entry.has_current_source |= component.is_current_source;
            if !entry.connected_components.contains(&identity) {
                entry.connected_components.push(identity.clone());
            }
            if pin.is_output {
                entry.output_drivers.insert(identity.clone());
                if component.is_voltage_source {
                    entry.voltage_source_drivers.insert(identity.clone());
                } else {
                    entry.declared_output_drivers.insert(identity.clone());
                }
            }
        }
    }

    for (name, net) in &mut nets {
        net.display_name = connectivity
            .display_names
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.clone());
        // A projected bit is exactly a node whose deck spelling differs from
        // the drawing's; a member name written by hand parses as a slice.
        net.bus_member = connectivity.display_names.contains_key(name)
            || crate::state::BusSlice::parse(name).is_ok_and(|slice| slice.is_scalar());
        net.connected_components.sort();
        net.connected_components.dedup();
        net.wire_ids.sort_unstable();
        net.wire_ids.dedup();
    }
    nets
}
