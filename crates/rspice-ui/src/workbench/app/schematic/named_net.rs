//! Stable, fail-closed identity for commands that target a logical named net.
//!
//! A conductor does not own a durable object ID of its own. Its stable editing
//! authority is therefore the exact set of authored net labels and interface
//! ports that name it. Commands capture those objects by value and publish one
//! guarded undo transaction; anonymous, ground-owned, stale, or ambiguous
//! conductor selections are never guessed into an editable target.

use std::collections::HashSet;

use crate::simulation::netlist_gen::extraction::{ExtractedConnectivity, extract};
use crate::simulation::netlist_gen::{DesignNet, NetClass};
use crate::state::{Component, ComponentType, NetLabel, Point, SchematicState};

use crate::workbench::app_state::AppState;

/// Membership in the one net a rename targets, decided by the one extraction.
///
/// Which conductors, labels and interface ports form a net is answered in
/// exactly one place. Capturing the naming authority and re-checking it before
/// the write both read this, so the two can never disagree about which objects
/// name the conductor — and neither can disagree with the deck.
struct NetMembership {
    connectivity: ExtractedConnectivity,
    net_id: Option<usize>,
}

impl NetMembership {
    /// Resolve the net that owns `wire_ids`, falling back to `seeds` for a net
    /// whose only geometry is a terminal an interface port stands on.
    fn resolve(schematic: &SchematicState, wire_ids: &[u64], seeds: &[Point]) -> Self {
        let connectivity = extract(schematic, None);
        let net_id = wire_ids
            .iter()
            .find_map(|wire_id| connectivity.net_of_wire(*wire_id))
            .or_else(|| seeds.iter().find_map(|point| connectivity.net_at(*point)))
            .map(|net| net.id);
        Self {
            connectivity,
            net_id,
        }
    }

    fn contains(&self, point: Point) -> bool {
        self.net_id
            .is_some_and(|id| self.connectivity.point_to_net.get(&point) == Some(&id))
    }
}

/// The point an interface port's single terminal stands on.
fn port_terminal(port: &Component) -> Option<Point> {
    port.terminal_positions()
        .into_iter()
        .next()
        .map(|(_, point)| point)
}

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

    // The naming authority for a conductor is the net the configured design
    // gives it. A design that does not resolve has no such net, which lands in
    // the same place as an ambiguous selection: no editable target, so the
    // rename commands are unavailable rather than acting on a name the run
    // would not use.
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .ok()?;
    let nets = crate::simulation::netlist_gen::projection_nets(
        &state.library_manager,
        &projection,
        &state.workspace.active_view.key(),
    );
    let net = if let Some(port) = selected_port {
        let port = port.port_spec()?;
        exactly_one(
            nets.iter()
                .filter(|net| net.is_port() && net_name_eq(&net.name, &port.name)),
        )?
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
    let seeds = selected_port
        .and_then(port_terminal)
        .into_iter()
        .collect::<Vec<_>>();
    let membership = NetMembership::resolve(schematic, &wire_ids, &seeds);

    let mut labels = schematic
        .net_labels
        .iter()
        .filter(|label| {
            membership.contains(label.pos)
                || (wire_ids.is_empty() && net_name_eq(&label.name, &net.name))
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
            let attached = port_terminal(component).is_some_and(|point| membership.contains(point));
            let names_net = component
                .port_spec()
                .is_some_and(|port| net_name_eq(&port.name, &net.name));
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

/// Two authored names denote one net when the netlister would join them, and
/// it folds ASCII case whatever the document's naming policy says — the deck is
/// case-insensitive. The policy is an authoring-syntax rule, so it decides which
/// characters a rename may contain, never which conductor is being renamed.
fn net_name_eq(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
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
    let seeds = target
        .ports
        .iter()
        .filter_map(port_terminal)
        .collect::<Vec<_>>();
    let membership = NetMembership::resolve(schematic, &target.wire_ids, &seeds);
    if schematic.net_labels.iter().any(|label| {
        !label_ids.contains(&label.id)
            && (net_name_eq(&label.name, &target.name) || membership.contains(label.pos))
    }) {
        return Err("The naming-label set for the selected net changed while editing.".to_owned());
    }
    if schematic.components.iter().any(|component| {
        if component.kind != ComponentType::Port || port_ids.contains(&component.id) {
            return false;
        }
        let names_target = component
            .port_spec()
            .is_some_and(|port| net_name_eq(&port.name, &target.name));
        let attached = port_terminal(component).is_some_and(|point| membership.contains(point));
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
    if schematic
        .net_labels
        .iter()
        .any(|label| !label_ids.contains(&label.id) && net_name_eq(&label.name, candidate))
    {
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
                .is_some_and(|port| net_name_eq(&port.name, candidate))
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

    /// Two drawn groups one name joins are one node in the deck, so the rename
    /// authority is every label that names it. Renaming from either group
    /// retargets the whole net rather than leaving a same-named twin behind
    /// that would silently re-merge under the old name.
    #[test]
    fn renaming_one_group_renames_the_net_the_deck_actually_has() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::new(11, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .schematic
            .wires
            .push(Wire::new(12, vec![Point::new(0, 100), Point::new(40, 100)]));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(21, Point::new(20, 0), "VDD"));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(22, Point::new(20, 100), "VDD"));
        state.schematic.selection.select_only_wire(11);

        let target = selected_named_net_target(&state).expect("named wire target");
        assert_eq!(target.name, "VDD");
        assert_eq!(target.wire_ids, [11, 12]);
        assert_eq!(
            target
                .labels
                .iter()
                .map(|label| label.id)
                .collect::<Vec<_>>(),
            [21, 22],
            "both groups' labels name the one net the deck emits"
        );

        assert!(
            apply_named_net_rename(&mut state.schematic, target, "VDD_CORE".to_owned()).unwrap()
        );
        assert!(
            state
                .schematic
                .net_labels
                .iter()
                .all(|label| label.name == "VDD_CORE"),
            "a rename that left one group behind would mint a second net"
        );
        assert!(state.schematic.undo());
        assert!(
            state
                .schematic
                .net_labels
                .iter()
                .all(|label| label.name == "VDD"),
            "the whole rename is exactly one undo step"
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
