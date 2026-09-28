//! Stable, fail-closed identity for commands that target a logical named net.
//!
//! A conductor does not own a durable object ID of its own. Its stable editing
//! authority is therefore the exact set of authored net labels and interface
//! ports that name it. Commands capture those objects by value and publish one
//! guarded undo transaction; anonymous, ground-owned, stale, or ambiguous
//! conductor selections are never guessed into an editable target.

use crate::simulation::netlist_gen::{DesignNet, NetClass};
#[cfg(test)]
use crate::state::NetLabel;
pub(crate) use crate::state::named_net::{
    NamedNetTarget, apply_named_net_rename, validate_named_net_rename,
};
use crate::state::named_net::{NetMembership, net_name_eq, port_terminal};
use crate::state::{Component, ComponentType, Point, SchematicState};

use crate::workbench::app_state::AppState;

/// Resolve a wire/segment/vertex-only selection or one selected interface
/// port to exactly one authored, non-ground net.
pub(crate) fn selected_named_net_target(state: &AppState) -> Option<NamedNetTarget> {
    let selection = &state.schematic.session.selection;
    let selected_port = selection.single_component().and_then(|id| {
        state
            .schematic
            .document()
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
    let selection = &state.schematic.session.selection;
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
    let membership = NetMembership::resolve(schematic.document(), &wire_ids, &seeds);

    let mut labels = schematic
        .document()
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
        .document()
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
                        .document()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point, Wire};

    fn named_wire_state(name: &str) -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(11, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(21, Point::new(20, 0), name));
        state.schematic.session.selection.select_only_wire(11);
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
        assert_eq!(state.schematic.document().net_labels[0].id, 21);
        assert_eq!(state.schematic.document().net_labels[0].name, "sense_p");
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().net_labels[0].name, "sense");
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
            .document_mut_for_test()
            .wires
            .push(Wire::new(11, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(12, vec![Point::new(0, 100), Point::new(40, 100)]));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(21, Point::new(20, 0), "VDD"));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(22, Point::new(20, 100), "VDD"));
        state.schematic.session.selection.select_only_wire(11);

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
                .document()
                .net_labels
                .iter()
                .all(|label| label.name == "VDD_CORE"),
            "a rename that left one group behind would mint a second net"
        );
        assert!(state.schematic.undo());
        assert!(
            state
                .schematic
                .document()
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
            .document_mut_for_test()
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]));
        state.schematic.session.selection.select_only_wire(1);
        assert!(selected_named_net_target(&state).is_none());

        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(2, vec![Point::new(0, 20), Point::new(10, 20)]));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(10, Point::new(5, 0), "a"));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(11, Point::new(5, 20), "b"));
        state.schematic.session.selection.select_wire(1);
        state.schematic.session.selection.select_wire(2);
        assert!(selected_named_net_target(&state).is_none());
    }

    #[test]
    fn selected_interface_port_renames_its_logical_net_not_reference_designator() {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(3, vec![Point::new(0, 0), Point::new(20, 0)]));
        let mut port =
            Component::new(31, ComponentType::Port, Point::new(0, 0)).with_name_value("P31", "VIN");
        port.params = "dir=in".to_owned();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(port);
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(32, Point::new(10, 0), "VIN"));
        state.schematic.session.selection.select_only_component(31);

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
        assert_eq!(state.schematic.document().components[0].name, "P31");
        assert_eq!(state.schematic.document().components[0].value, "VIN_SENSE");
        assert_eq!(state.schematic.document().net_labels[0].name, "VIN_SENSE");
    }

    #[test]
    fn stale_authority_and_existing_net_name_are_rejected_without_mutation() {
        let mut state = named_wire_state("sense");
        let target = selected_named_net_target(&state).expect("named wire target");
        state.schematic.document_mut_for_test().net_labels[0].name = "changed_elsewhere".to_owned();
        assert!(
            apply_named_net_rename(&mut state.schematic, target, "sense_p".to_owned()).is_err()
        );
        assert_eq!(
            state.schematic.document().net_labels[0].name,
            "changed_elsewhere"
        );
        assert!(!state.schematic.can_undo());

        let state = {
            let mut state = named_wire_state("sense");
            state
                .schematic
                .document_mut_for_test()
                .wires
                .push(Wire::new(12, vec![Point::new(0, 20), Point::new(40, 20)]));
            state
                .schematic
                .document_mut_for_test()
                .net_labels
                .push(NetLabel::new(22, Point::new(20, 20), "taken"));
            state
        };
        let target = selected_named_net_target(&state).expect("named wire target");
        assert!(validate_named_net_rename(&state.schematic, &target, "taken").is_err());
    }
}
