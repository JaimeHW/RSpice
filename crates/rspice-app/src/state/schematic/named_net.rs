//! Captured named-net authority, validation and atomic document-local rename.

use crate::state::{Component, ComponentType, NetLabel, Point, SchematicState};
use rspice_design::connectivity::ExtractedConnectivity;
use std::collections::HashSet;

/// Membership in the one net a rename targets, decided by the one extraction.
///
/// Which conductors, labels and interface ports form a net is answered in
/// exactly one place. Capturing the naming authority and re-checking it before
/// the write both read this, so the two can never disagree about which objects
/// name the conductor — and neither can disagree with the deck.
pub(crate) struct NetMembership {
    connectivity: ExtractedConnectivity,
    net_id: Option<usize>,
}

impl NetMembership {
    /// Resolve the net that owns `wire_ids`, falling back to `seeds` for a net
    /// whose only geometry is a terminal an interface port stands on.
    pub(crate) fn resolve(schematic: &SchematicState, wire_ids: &[u64], seeds: &[Point]) -> Self {
        let connectivity =
            rspice_design::connectivity::extract(&schematic.document, |_| None, |_| None);
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

    pub(crate) fn contains(&self, point: Point) -> bool {
        self.net_id
            .is_some_and(|id| self.connectivity.point_to_net.get(&point) == Some(&id))
    }
}

/// The point an interface port's single terminal stands on.
pub(crate) fn port_terminal(port: &Component) -> Option<Point> {
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

/// Two authored names denote one net when the netlister would join them, and
/// it folds ASCII case whatever the document's naming policy says — the deck is
/// case-insensitive. The policy is an authoring-syntax rule, so it decides which
/// characters a rename may contain, never which conductor is being renamed.
pub(crate) fn net_name_eq(left: &str, right: &str) -> bool {
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
    NetLabel::validate_name(candidate, schematic.document.document_policy.net_naming)
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
            .document
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
            .document
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
        .any(|id| !schematic.document.wires.iter().any(|wire| wire.id == *id))
    {
        return Err("The selected conductor geometry changed while editing.".to_owned());
    }
    let seeds = target
        .ports
        .iter()
        .filter_map(port_terminal)
        .collect::<Vec<_>>();
    let membership = NetMembership::resolve(schematic, &target.wire_ids, &seeds);
    if schematic.document.net_labels.iter().any(|label| {
        !label_ids.contains(&label.id)
            && (net_name_eq(&label.name, &target.name) || membership.contains(label.pos))
    }) {
        return Err("The naming-label set for the selected net changed while editing.".to_owned());
    }
    if schematic.document.components.iter().any(|component| {
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
        .document
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
    if schematic.document.components.iter().any(|component| {
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
        for label in &mut schematic.document.net_labels {
            if label_ids.contains(&label.id) {
                label.name = candidate.clone();
            }
        }
        for component in &mut schematic.document.components {
            if port_ids.contains(&component.id) {
                component.value = candidate.clone();
            }
        }
        schematic.is_dirty = true;
        schematic.bump_topology_version();
    }))
}
