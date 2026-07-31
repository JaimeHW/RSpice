//! Stable, fail-closed identity for commands that target a logical named net.
//!
//! A conductor does not own a durable object ID of its own. Its stable editing
//! authority is therefore the exact set of authored net labels and interface
//! ports that name it. Commands capture those objects by value and publish one
//! guarded undo transaction; anonymous, ground-owned, stale, or ambiguous
//! conductor selections are never guessed into an editable target.

use std::collections::HashSet;

use crate::simulation::netlist_gen::{
    DesignNet, HierarchySource, NetClass, design_nets_with_hierarchy,
};
use crate::state::{
    Component, ComponentType, NetGraph, NetLabel, NetNamingPolicy, Point, SchematicState,
};

use crate::workbench::app_state::AppState;

/// Exact naming authority captured for one logical named net.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NamedNetTarget {
    pub(crate) name: String,
    pub(crate) labels: Vec<NetLabel>,
    pub(crate) ports: Vec<Component>,
    pub(crate) wire_ids: Vec<u64>,
    pub(crate) preview_position: Point,
}

impl NamedNetTarget {
    pub(crate) fn authority_count(&self) -> usize {
        self.labels.len() + self.ports.len()
    }
}

/// Resolve a wire/segment/vertex-only selection or one selected interface
/// port to exactly one authored, non-ground net.
pub(crate) fn selected_named_net_target(state: &AppState) -> Option<NamedNetTarget> {
    let selection = &state.schematic.selection;
    let selected_port = selection.single_component().and_then(|id| {
        state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id && component.kind == ComponentType::Port)
    });
    let wire_ids = if selected_port.is_none() {
        if !wire_geometry_only(state) {
            return None;
        }
        let mut ids = selection.all_selected_wire_ids();
        ids.sort_unstable();
        ids.dedup();
        ids
    } else {
        Vec::new()
    };

    let hierarchy =
        HierarchySource::from_workspace(&state.library_manager, &state.workspace.schematic_buffers);
    let nets = design_nets_with_hierarchy(&state.schematic, &hierarchy);
    let net =
        if let Some(port) = selected_port {
            let port = port.port_spec()?;
            exactly_one(nets.iter().filter(|net| {
                net.is_port() && net_name_eq(&state.schematic, &net.name, &port.name)
            }))?
        } else {
            resolve_wire_net(&nets, &wire_ids)?
        };
    if !net.authored_name || net.class == NetClass::Ground || net.name == "0" {
        return None;
    }

    capture_target(&state.schematic, net, selected_port, &wire_ids)
}

fn wire_geometry_only(state: &AppState) -> bool {
    let selection = &state.schematic.selection;
    selection.has_any_wire_selection()
        && selection.components.is_empty()
        && selection.junctions.is_empty()
        && selection.buses.is_empty()
        && selection.bus_taps.is_empty()
        && selection.net_labels.is_empty()
        && selection.design_notes.is_empty()
        && selection.documentation_shapes.is_empty()
        && selection.probes.is_empty()
}

fn resolve_wire_net<'a>(nets: &'a [DesignNet], wire_ids: &[u64]) -> Option<&'a DesignNet> {
    let mut resolved_index = None;
    for wire_id in wire_ids {
        let mut matches = nets
            .iter()
            .enumerate()
            .filter(|(_, net)| net.wire_ids.contains(wire_id));
        let (index, _) = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        match resolved_index {
            Some(existing) if existing != index => return None,
            Some(_) => {}
            None => resolved_index = Some(index),
        }
    }
    resolved_index.and_then(|index| nets.get(index))
}

fn capture_target(
    schematic: &SchematicState,
    net: &DesignNet,
    selected_port: Option<&Component>,
    selected_wire_ids: &[u64],
) -> Option<NamedNetTarget> {
    let mut wire_ids = net.wire_ids.clone();
    wire_ids.sort_unstable();
    wire_ids.dedup();
    let wire_id_set = wire_ids.iter().copied().collect::<HashSet<_>>();
    let graph = NetGraph::build(&schematic.wires, &schematic.junctions);
    let point_is_on_net = |point| {
        graph
            .find_connected_wires(point)
            .iter()
            .any(|wire_id| wire_id_set.contains(wire_id))
    };

    let mut labels = schematic
        .net_labels
        .iter()
        .filter(|label| {
            point_is_on_net(label.pos)
                || (wire_ids.is_empty() && net_name_eq(schematic, &label.name, &net.name))
        })
        .cloned()
        .collect::<Vec<_>>();
    labels.sort_by_key(|label| label.id);

    let mut ports = schematic
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Port)
        .filter(|component| {
            let selected = selected_port.is_some_and(|port| port.id == component.id);
            let attached = component
                .terminal_positions()
                .into_iter()
                .next()
                .is_some_and(|(_, point)| point_is_on_net(point));
            let names_net = component
                .port_spec()
                .is_some_and(|port| net_name_eq(schematic, &port.name, &net.name));
            selected || attached || (wire_ids.is_empty() && names_net)
        })
        .cloned()
        .collect::<Vec<_>>();
    ports.sort_by_key(|port| port.id);

    if labels.is_empty() && ports.is_empty() {
        return None;
    }
    let preview_position = selected_port.map_or_else(
        || {
            selected_wire_ids
                .iter()
                .find_map(|id| {
                    schematic
                        .wires
                        .iter()
                        .find(|wire| wire.id == *id)
                        .and_then(|wire| wire.points.first().copied())
                })
                .or_else(|| labels.first().map(|label| label.pos))
                .unwrap_or_else(Point::origin)
        },
        |port| port.pos,
    );
    Some(NamedNetTarget {
        name: net.name.clone(),
        labels,
        ports,
        wire_ids,
        preview_position,
    })
}

fn exactly_one<'a>(mut values: impl Iterator<Item = &'a DesignNet>) -> Option<&'a DesignNet> {
    let value = values.next()?;
    values.next().is_none().then_some(value)
}

fn net_name_eq(schematic: &SchematicState, left: &str, right: &str) -> bool {
    match schematic.document_policy.net_naming {
        NetNamingPolicy::StrictCaseSensitive => left == right,
        NetNamingPolicy::SpiceCompatibleRelaxed => left.eq_ignore_ascii_case(right),
    }
}

/// Validate a captured target and candidate name without changing the design.
pub(crate) fn validate_named_net_rename(
    schematic: &SchematicState,
    target: &NamedNetTarget,
    candidate: &str,
) -> Result<String, String> {
    if schematic.read_only {
        return Err("The active schematic is read-only.".to_owned());
    }
    if target.authority_count() == 0 || target.name == "0" {
        return Err("The selected conductor has no renameable named-net authority.".to_owned());
    }
    validate_target_is_current(schematic, target)?;

    let candidate = candidate.trim();
    if candidate.is_empty() {
        return Err("Enter a non-empty net name.".to_owned());
    }
    NetLabel::validate_name(candidate, schematic.document_policy.net_naming)
        .map_err(|reason| format!("Net name: {reason}."))?;
    reject_external_name_collision(schematic, target, candidate)?;
    Ok(candidate.to_owned())
}

fn validate_target_is_current(
    schematic: &SchematicState,
    target: &NamedNetTarget,
) -> Result<(), String> {
    let label_ids = target
        .labels
        .iter()
        .map(|label| label.id)
        .collect::<HashSet<_>>();
    let port_ids = target
        .ports
        .iter()
        .map(|port| port.id)
        .collect::<HashSet<_>>();
    for expected in &target.labels {
        let Some(current) = schematic
            .net_labels
            .iter()
            .find(|label| label.id == expected.id)
        else {
            return Err("A naming label on the selected net no longer exists.".to_owned());
        };
        if current != expected {
            return Err("A naming label on the selected net changed while editing.".to_owned());
        }
    }
    for expected in &target.ports {
        let Some(current) = schematic
            .components
            .iter()
            .find(|component| component.id == expected.id)
        else {
            return Err("An interface port naming the selected net no longer exists.".to_owned());
        };
        if current != expected || current.kind != ComponentType::Port {
            return Err(
                "An interface port naming the selected net changed while editing.".to_owned(),
            );
        }
    }
    if target
        .wire_ids
        .iter()
        .any(|id| !schematic.wires.iter().any(|wire| wire.id == *id))
    {
        return Err("The selected conductor geometry changed while editing.".to_owned());
    }
    let target_wire_ids = target.wire_ids.iter().copied().collect::<HashSet<_>>();
    let graph = NetGraph::build(&schematic.wires, &schematic.junctions);
    let point_is_on_target = |point| {
        graph
            .find_connected_wires(point)
            .iter()
            .any(|wire_id| target_wire_ids.contains(wire_id))
    };
    if schematic.net_labels.iter().any(|label| {
        !label_ids.contains(&label.id)
            && (net_name_eq(schematic, &label.name, &target.name) || point_is_on_target(label.pos))
    }) {
        return Err("The naming-label set for the selected net changed while editing.".to_owned());
    }
    if schematic.components.iter().any(|component| {
        if component.kind != ComponentType::Port || port_ids.contains(&component.id) {
            return false;
        }
        let names_target = component
            .port_spec()
            .is_some_and(|port| net_name_eq(schematic, &port.name, &target.name));
        let attached = component
            .terminal_positions()
            .into_iter()
            .next()
            .is_some_and(|(_, point)| point_is_on_target(point));
        names_target || attached
    }) {
        return Err(
            "The interface-port set for the selected net changed while editing.".to_owned(),
        );
    }
    Ok(())
}

fn reject_external_name_collision(
    schematic: &SchematicState,
    target: &NamedNetTarget,
    candidate: &str,
) -> Result<(), String> {
    let label_ids = target
        .labels
        .iter()
        .map(|label| label.id)
        .collect::<HashSet<_>>();
    if schematic.net_labels.iter().any(|label| {
        !label_ids.contains(&label.id) && net_name_eq(schematic, &label.name, candidate)
    }) {
        return Err(format!(
            "Another logical net is already named `{candidate}`; merging nets is not a rename."
        ));
    }
    let port_ids = target
        .ports
        .iter()
        .map(|port| port.id)
        .collect::<HashSet<_>>();
    if schematic.components.iter().any(|component| {
        !port_ids.contains(&component.id)
            && component
                .port_spec()
                .is_some_and(|port| net_name_eq(schematic, &port.name, candidate))
    }) {
        return Err(format!(
            "Another interface port is already named `{candidate}`; merging nets is not a rename."
        ));
    }
    Ok(())
}

/// Publish one named-net rename while retaining every label and port ID.
pub(crate) fn apply_named_net_rename(
    schematic: &mut SchematicState,
    target: NamedNetTarget,
    candidate: String,
) -> Result<bool, String> {
    let candidate = validate_named_net_rename(schematic, &target, &candidate)?;
    if target.labels.iter().all(|label| label.name == candidate)
        && target.ports.iter().all(|port| port.value == candidate)
    {
        return Ok(false);
    }
    let label_ids = target
        .labels
        .iter()
        .map(|label| label.id)
        .collect::<HashSet<_>>();
    let port_ids = target
        .ports
        .iter()
        .map(|port| port.id)
        .collect::<HashSet<_>>();
    Ok(schematic.with_undo("rename named net", move |schematic| {
        for label in &mut schematic.net_labels {
            if label_ids.contains(&label.id) {
                label.name = candidate.clone();
            }
        }
        for component in &mut schematic.components {
            if port_ids.contains(&component.id) {
                component.value = candidate.clone();
            }
        }
        schematic.is_dirty = true;
        schematic.bump_topology_version();
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point, Wire};

    fn named_wire_state(name: &str) -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::new(11, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(21, Point::new(20, 0), name));
        state.schematic.selection.select_only_wire(11);
        state
    }

    #[test]
    fn wire_geometry_resolves_one_stable_named_net_and_renames_atomically() {
        let mut state = named_wire_state("sense");
        let target = selected_named_net_target(&state).expect("named wire target");
        assert_eq!(target.name, "sense");
        assert_eq!(
            target
                .labels
                .iter()
                .map(|label| label.id)
                .collect::<Vec<_>>(),
            [21]
        );
        assert!(target.ports.is_empty());

        assert!(
            apply_named_net_rename(&mut state.schematic, target, "sense_p".to_owned()).unwrap()
        );
        assert_eq!(state.schematic.net_labels[0].id, 21);
        assert_eq!(state.schematic.net_labels[0].name, "sense_p");
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.net_labels[0].name, "sense");
        assert!(
            !state.schematic.undo(),
            "rename must be exactly one undo step"
        );
    }

    #[test]
    fn unnamed_and_multi_net_wire_selections_fail_closed() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]));
        state.schematic.selection.select_only_wire(1);
        assert!(selected_named_net_target(&state).is_none());

        state
            .schematic
            .wires
            .push(Wire::new(2, vec![Point::new(0, 20), Point::new(10, 20)]));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(10, Point::new(5, 0), "a"));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(11, Point::new(5, 20), "b"));
        state.schematic.selection.select_wire(1);
        state.schematic.selection.select_wire(2);
        assert!(selected_named_net_target(&state).is_none());
    }

    #[test]
    fn selected_interface_port_renames_its_logical_net_not_reference_designator() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::new(3, vec![Point::new(0, 0), Point::new(20, 0)]));
        let mut port =
            Component::new(31, ComponentType::Port, Point::new(0, 0)).with_name_value("P31", "VIN");
        port.params = "dir=in".to_owned();
        state.schematic.components.push(port);
        state
            .schematic
            .net_labels
            .push(NetLabel::new(32, Point::new(10, 0), "VIN"));
        state.schematic.selection.select_only_component(31);

        let target = selected_named_net_target(&state).expect("selected port net target");
        assert_eq!(
            target.ports.iter().map(|port| port.id).collect::<Vec<_>>(),
            [31]
        );
        assert_eq!(
            target
                .labels
                .iter()
                .map(|label| label.id)
                .collect::<Vec<_>>(),
            [32]
        );
        assert!(
            apply_named_net_rename(&mut state.schematic, target, "VIN_SENSE".to_owned()).unwrap()
        );
        assert_eq!(state.schematic.components[0].name, "P31");
        assert_eq!(state.schematic.components[0].value, "VIN_SENSE");
        assert_eq!(state.schematic.net_labels[0].name, "VIN_SENSE");
    }

    #[test]
    fn stale_authority_and_existing_net_name_are_rejected_without_mutation() {
        let mut state = named_wire_state("sense");
        let target = selected_named_net_target(&state).expect("named wire target");
        state.schematic.net_labels[0].name = "changed_elsewhere".to_owned();
        assert!(
            apply_named_net_rename(&mut state.schematic, target, "sense_p".to_owned()).is_err()
        );
        assert_eq!(state.schematic.net_labels[0].name, "changed_elsewhere");
        assert!(!state.schematic.can_undo());

        let state = {
            let mut state = named_wire_state("sense");
            state
                .schematic
                .wires
                .push(Wire::new(12, vec![Point::new(0, 20), Point::new(40, 20)]));
            state
                .schematic
                .net_labels
                .push(NetLabel::new(22, Point::new(20, 20), "taken"));
            state
        };
        let target = selected_named_net_target(&state).expect("named wire target");
        assert!(validate_named_net_rename(&state.schematic, &target, "taken").is_err());
    }
}
