//! Capturing complete schematic objects and pasting them with remapped identities.

use super::{
    bus::{Bus, BusTap},
    clipboard::ClipboardData,
    component::Component,
    design_note::DesignNote,
    document::SchematicDocument,
    documentation_shape::DocumentationShape,
    junction_candidates::{collect_junction_candidates, nearest_junction_candidate},
    net_label::{Junction, NetLabel},
    point::Point,
    probe::SchematicProbe,
    reference_edit::PreparedCopyReferences,
    wire::Wire,
};
use rspice_design::schematic::{
    documentation_shape::clamped_documentation_shape_translation, identity::SchematicIdentity,
    junction_edit,
};
use std::collections::HashSet;

/// Complete object identities requested by a copy operation. Editor handles and
/// gesture state are deliberately absent; junctions are supplied as positions.
pub struct CopySelection<'a> {
    pub components: &'a HashSet<u64>,
    pub wires: &'a HashSet<u64>,
    pub net_labels: &'a HashSet<u64>,
    pub design_notes: &'a HashSet<u64>,
    pub documentation_shapes: &'a HashSet<u64>,
    pub probes: &'a HashSet<u64>,
    pub buses: &'a HashSet<u64>,
    pub bus_taps: &'a HashSet<u64>,
}

/// Copy the requested objects and the conductors, source buses and junctions
/// required by the existing complete-object copy policy, in document order.
pub fn capture_complete_selection(
    document: &SchematicDocument,
    selection: CopySelection<'_>,
    junctions: impl Iterator<Item = Point> + Clone,
    mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
) -> ClipboardData {
    let selected_comps: Vec<Component> = document
        .components
        .iter()
        .filter(|c| selection.components.contains(&c.id))
        .cloned()
        .collect();

    // Get all terminal positions for selected components
    let selected_terminals: Vec<Point> = selected_comps
        .iter()
        .flat_map(&mut terminal_points_for)
        .collect();

    // Find wires that have both endpoints at selected component terminals
    let mut wires_to_copy: Vec<Wire> = Vec::new();

    for wire in &document.wires {
        // Check if explicitly selected
        if selection.wires.contains(&wire.id) {
            if wire.points.len() >= 2 {
                wires_to_copy.push(wire.clone());
            }
            continue;
        }

        // Check if both endpoints connect to selected components
        if wire.points.len() >= 2 {
            let start = wire.points[0];
            let end = *wire.points.last().unwrap();

            let start_connected = selected_terminals.contains(&start);
            let end_connected = selected_terminals.contains(&end);

            if start_connected && end_connected {
                wires_to_copy.push(wire.clone());
            }
        }
    }

    // Junction dots that sit on a copied wire travel with the selection;
    // a pasted multi-way joint must keep its explicit connection dots.
    let mut junctions_to_copy: Vec<Point> = document
        .junctions
        .iter()
        .map(|j| j.pos)
        .filter(|pos| {
            junctions.clone().any(|selected| selected == *pos)
                || wires_to_copy.iter().any(|wire| wire.contains_point(*pos))
        })
        .collect();
    junctions_to_copy.sort_by_key(|point| (point.x, point.y));
    junctions_to_copy.dedup();

    let explicitly_selected_bus_ids = selection.buses;
    let mut bus_ids_to_copy = explicitly_selected_bus_ids.clone();
    bus_ids_to_copy.extend(
        document
            .bus_taps
            .iter()
            .filter(|tap| selection.bus_taps.contains(&tap.id))
            .map(|tap| tap.bus_id),
    );
    let buses_to_copy: Vec<Bus> = document
        .buses
        .iter()
        .filter(|bus| bus_ids_to_copy.contains(&bus.id))
        .cloned()
        .collect();
    let bus_taps_to_copy: Vec<BusTap> = document
        .bus_taps
        .iter()
        .filter(|tap| {
            selection.bus_taps.contains(&tap.id)
                || explicitly_selected_bus_ids.contains(&tap.bus_id)
        })
        .cloned()
        .collect();
    let net_labels_to_copy: Vec<NetLabel> = document
        .net_labels
        .iter()
        .filter(|label| selection.net_labels.contains(&label.id))
        .cloned()
        .collect();
    let design_notes_to_copy: Vec<DesignNote> = document
        .design_notes
        .iter()
        .filter(|note| selection.design_notes.contains(&note.id))
        .cloned()
        .collect();
    let documentation_shapes_to_copy: Vec<DocumentationShape> = document
        .documentation_shapes
        .iter()
        .filter(|shape| selection.documentation_shapes.contains(&shape.id))
        .cloned()
        .collect();
    let probes_to_copy: Vec<SchematicProbe> = document
        .probes
        .iter()
        .filter(|probe| selection.probes.contains(&probe.id))
        .cloned()
        .collect();

    ClipboardData::from_complete_selection(ClipboardData {
        components: selected_comps,
        wires: wires_to_copy,
        junctions: junctions_to_copy,
        buses: buses_to_copy,
        bus_taps: bus_taps_to_copy,
        net_labels: net_labels_to_copy,
        design_notes: design_notes_to_copy,
        documentation_shapes: documentation_shapes_to_copy,
        probes: probes_to_copy,
        origin: Point::origin(),
    })
}

/// A validated paste bound to its destination and allocation state. Commit
/// consumes it; neither the source clipboard nor the destination can change
/// while it is prepared.
pub struct ClipboardPaste<'document, 'clipboard> {
    document: &'document mut SchematicDocument,
    identity: &'document mut SchematicIdentity,
    clipboard: &'clipboard ClipboardData,
    references: PreparedCopyReferences,
    paste_pos: Point,
    junction_candidates: Option<Vec<Point>>,
}

/// Borrowed objects appended by one paste, for editor selection and notification.
/// These are results, not inputs authorizing another mutation.
pub struct PastedObjects<'a> {
    pub components: &'a [Component],
    pub wires: &'a [Wire],
    pub net_labels: &'a [NetLabel],
    pub design_notes: &'a [DesignNote],
    pub documentation_shapes: &'a [DocumentationShape],
    pub probes: &'a [SchematicProbe],
    pub buses: &'a [Bus],
    pub bus_taps: &'a [BusTap],
    pub junctions: &'a [Junction],
    pub topology_changes: u64,
}

impl PastedObjects<'_> {
    pub fn has_content(&self) -> bool {
        !self.components.is_empty()
            || !self.wires.is_empty()
            || !self.net_labels.is_empty()
            || !self.design_notes.is_empty()
            || !self.documentation_shapes.is_empty()
            || !self.probes.is_empty()
            || !self.buses.is_empty()
            || !self.bus_taps.is_empty()
            || !self.junctions.is_empty()
    }
}

impl<'document, 'clipboard> ClipboardPaste<'document, 'clipboard> {
    pub fn prepare(
        document: &'document mut SchematicDocument,
        identity: &'document mut SchematicIdentity,
        clipboard: &'clipboard ClipboardData,
        pos: Point,
    ) -> Result<Option<Self>, String> {
        if clipboard.is_empty() {
            return Ok(None);
        }
        let junction_only = clipboard.components.is_empty()
            && clipboard.wires.is_empty()
            && clipboard.buses.is_empty()
            && clipboard.bus_taps.is_empty()
            && clipboard.net_labels.is_empty()
            && clipboard.design_notes.is_empty()
            && clipboard.documentation_shapes.is_empty()
            && clipboard.probes.is_empty();
        let candidates = junction_only.then(|| collect_junction_candidates(&document.wires));
        // A junction-only clipboard is a connectivity edit, not decoration.
        // Snap its anchor through the same ambiguous-crossing candidate set as
        // the junction tool, then reject it before opening an undo transaction
        // unless at least one translated marker would create a new connection.
        let paste_pos = if junction_only {
            let Some(candidate) = nearest_junction_candidate(
                candidates.as_deref().unwrap_or_default(),
                pos,
                document.grid_size,
            ) else {
                return Ok(None);
            };
            candidate
        } else {
            pos
        };
        if junction_only {
            let offset_x = paste_pos.x.saturating_sub(clipboard.origin.x);
            let offset_y = paste_pos.y.saturating_sub(clipboard.origin.y);
            let has_valid_target = clipboard.junctions.iter().any(|junction| {
                let target = Point::new(
                    junction.x.saturating_add(offset_x),
                    junction.y.saturating_add(offset_y),
                );
                !document
                    .junctions
                    .iter()
                    .any(|junction| junction.pos == target)
                    && nearest_junction_candidate(
                        candidates.as_deref().unwrap_or_default(),
                        target,
                        0,
                    ) == Some(target)
            });
            if !has_valid_target {
                return Ok(None);
            }
        }

        let references = PreparedCopyReferences::new(&clipboard.components)?;

        Ok(Some(Self {
            document,
            identity,
            clipboard,
            references,
            paste_pos,
            junction_candidates: candidates,
        }))
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    pub fn commit(self) -> PastedObjects<'document> {
        let Self {
            document,
            identity,
            clipboard,
            references,
            paste_pos,
            junction_candidates,
        } = self;
        let components_start = document.components.len();
        let wires_start = document.wires.len();
        let net_labels_start = document.net_labels.len();
        let design_notes_start = document.design_notes.len();
        let documentation_shapes_start = document.documentation_shapes.len();
        let probes_start = document.probes.len();
        let buses_start = document.buses.len();
        let bus_taps_start = document.bus_taps.len();
        let junctions_start = document.junctions.len();
        let clipboard_components = clipboard.components.clone();
        let clipboard_wires: Vec<Wire> = clipboard
            .wires
            .iter()
            .filter(|wire| wire.points.len() >= 2)
            .cloned()
            .collect();
        let clipboard_junctions = clipboard.junctions.clone();
        let clipboard_net_labels = clipboard.net_labels.clone();
        let clipboard_buses = clipboard.buses.clone();
        let clipboard_bus_taps = clipboard.bus_taps.clone();
        let clipboard_design_notes = clipboard.design_notes.clone();
        let clipboard_documentation_shapes = clipboard.documentation_shapes.clone();
        let clipboard_probes = clipboard.probes.clone();
        let origin = clipboard.origin;

        let offset_x = paste_pos.x.saturating_sub(origin.x);
        let offset_y = paste_pos.y.saturating_sub(origin.y);
        let documentation_shape_offset = clamped_documentation_shape_translation(
            clipboard_documentation_shapes
                .iter()
                .filter(|shape| shape.validate().is_ok()),
            Point::new(offset_x, offset_y),
        );

        let mut electrical_committed = false;

        // Allocate every identity before remapping references within the copy.
        let mut copied_components = Vec::with_capacity(clipboard_components.len());
        for comp in clipboard_components {
            electrical_committed = true;
            let new_id = identity.allocate(document);
            let mut new_comp = comp;
            new_comp.id = new_id;
            new_comp.pos.x = new_comp.pos.x.saturating_add(offset_x);
            new_comp.pos.y = new_comp.pos.y.saturating_add(offset_y);
            new_comp.name = identity.generate_name(new_comp.kind);
            copied_components.push(new_comp);
        }
        references.apply(&mut copied_components);
        document.components.extend(copied_components);

        // Paste wires with new IDs
        for wire in clipboard_wires {
            electrical_committed = true;
            let new_id = identity.allocate(document);
            let new_points: Vec<Point> = wire
                .points
                .iter()
                .map(|p| Point::new(p.x.saturating_add(offset_x), p.y.saturating_add(offset_y)))
                .collect();
            document.wires.push(Wire::new(new_id, new_points));
        }

        // Labels retain their user-facing net names while receiving new
        // document identities and translated attachment anchors.
        for mut label in clipboard_net_labels {
            electrical_committed = true;
            let new_id = identity.allocate(document);
            label.id = new_id;
            label.pos = Point::new(
                label.pos.x.saturating_add(offset_x),
                label.pos.y.saturating_add(offset_y),
            );
            document.net_labels.push(label);
        }

        // Documentation objects retain their typed semantics and source
        // text, but receive a fresh stable document/review identity.
        for note in clipboard_design_notes {
            if note.validate().is_err() {
                continue;
            }
            let target = Point::new(
                note.pos.x.saturating_add(offset_x),
                note.pos.y.saturating_add(offset_y),
            );
            let new_id = identity.allocate(document);
            let Ok(new_note) = DesignNote::new(new_id, target, note.kind, note.text) else {
                continue;
            };
            document.design_notes.push(new_note);
        }

        for mut shape in clipboard_documentation_shapes {
            if shape.validate().is_err() {
                continue;
            }
            shape.translate(documentation_shape_offset);
            if shape.validate().is_err() {
                continue;
            }
            let new_id = identity.allocate(document);
            shape.id = new_id;
            document.documentation_shapes.push(shape);
        }

        // Probe markers are non-electrical authored output intent. Bound
        // markers retain their exact raw expression; unbound markers get
        // a fresh display reference matching their new stable identity.
        for mut probe in clipboard_probes {
            if probe.validate().is_err() {
                continue;
            }
            let new_id = identity.allocate(document);
            probe.id = new_id;
            probe.position = Point::new(
                probe.position.x.saturating_add(offset_x),
                probe.position.y.saturating_add(offset_y),
            );
            if probe.source_expression.is_none() {
                probe.reference = format!("P{new_id}");
            }
            if probe.validate().is_err() {
                continue;
            }
            document.probes.push(probe);
        }

        // Paste buses before taps so every source reference can be
        // remapped to a fresh stable document identity.
        let mut bus_id_map = std::collections::HashMap::new();
        for mut bus in clipboard_buses {
            let old_id = bus.id;
            bus.translate(Point::new(offset_x, offset_y));
            if bus.validate().is_err() {
                continue;
            }
            electrical_committed = true;
            let new_id = identity.allocate(document);
            bus.id = new_id;
            document.buses.push(bus);
            bus_id_map.entry(old_id).or_insert(new_id);
        }

        for mut tap in clipboard_bus_taps {
            let Some(&new_bus_id) = bus_id_map.get(&tap.bus_id) else {
                continue;
            };
            tap.bus_id = new_bus_id;
            tap.translate(Point::new(offset_x, offset_y));
            let Some(source) = document.buses.iter().find(|bus| bus.id == new_bus_id) else {
                continue;
            };
            if tap.validate_against_bus(source).is_ok() {
                electrical_committed = true;
                tap.id = identity.allocate(document);
                document.bus_taps.push(tap);
            }
        }

        // Re-create junction dots only where at least two distinct wires
        // meet. This makes junction-only copy/paste useful without ever
        // manufacturing an electrically meaningless floating marker.
        for junction in clipboard_junctions {
            let target = Point::new(
                junction.x.saturating_add(offset_x),
                junction.y.saturating_add(offset_y),
            );
            let valid_target = if let Some(candidates) = &junction_candidates {
                nearest_junction_candidate(candidates, target, 0) == Some(target)
            } else {
                document
                    .wires
                    .iter()
                    .filter(|wire| wire.contains_point(target))
                    .map(|wire| wire.id)
                    .collect::<std::collections::HashSet<_>>()
                    .len()
                    >= 2
            };
            if valid_target
                && !document
                    .junctions
                    .iter()
                    .any(|junction| junction.pos == target)
            {
                electrical_committed = true;
                junction_edit::add_junction(document, identity, target);
            }
        }

        // Each inserted explicit junction historically invalidates topology,
        // followed by the paste's one electrical-change notification.
        let topology_changes =
            (document.junctions.len() - junctions_start) as u64 + u64::from(electrical_committed);
        PastedObjects {
            components: &document.components[components_start..],
            wires: &document.wires[wires_start..],
            net_labels: &document.net_labels[net_labels_start..],
            design_notes: &document.design_notes[design_notes_start..],
            documentation_shapes: &document.documentation_shapes[documentation_shapes_start..],
            probes: &document.probes[probes_start..],
            buses: &document.buses[buses_start..],
            bus_taps: &document.bus_taps[bus_taps_start..],
            junctions: &document.junctions[junctions_start..],
            topology_changes,
        }
    }
}
