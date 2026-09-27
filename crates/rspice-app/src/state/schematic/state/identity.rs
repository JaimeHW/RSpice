//! Object identity and reference designators.
//!
//! Allocates the ids a schematic addresses its objects by, and the R1/C2
//! style designators shown on the canvas. Both are stable across a save and
//! reload; neither is reused after a delete.

use super::*;
use std::collections::{HashMap, HashSet};

impl SchematicState {
    // =========================================================================
    // ID and Name Generation
    // =========================================================================

    /// Generate a unique ID
    pub fn next_id(&mut self) -> u64 {
        self.identity.allocate(&self.document)
    }

    /// Current allocator cursor used to key immutable transaction previews.
    /// Reading it never consumes an identity.
    pub(crate) const fn identity_cursor(&self) -> u64 {
        self.identity.cursor()
    }

    /// Get the current topology version
    ///
    /// This version is incremented whenever the schematic topology changes
    /// (components, wires, junctions added/removed/moved). Used by caches
    /// to detect when they need to be rebuilt.
    pub fn topology_version(&self) -> u64 {
        self.topology_version
    }

    /// Increment the topology version
    ///
    /// Call this after any structural change to invalidate caches.
    /// This is automatically called by mutation methods like add_component,
    /// add_wire, move_component_with_wires, etc.
    pub fn bump_topology_version(&mut self) {
        self.topology_version = self.topology_version.wrapping_add(1);
    }

    /// Recalculate runtime state after loading from file
    /// This MUST be called after deserialization to prevent ID collisions
    pub fn recalculate_runtime_state(&mut self) {
        let wire_count_before_repair = self.document.wires.len();
        self.document.wires.retain(|wire| wire.points.len() >= 2);
        self.clipboard.wires.retain(|wire| wire.points.len() >= 2);
        self.clipboard.buses.retain(|bus| bus.validate().is_ok());
        self.document
            .documentation_shapes
            .retain(|shape| shape.validate().is_ok());
        self.clipboard
            .documentation_shapes
            .retain(|shape| shape.validate().is_ok());
        self.clipboard
            .probes
            .retain(|probe| probe.validate().is_ok());
        if self.document.wires.len() != wire_count_before_repair {
            self.bump_topology_version();
        }

        let topology_changes = self.identity.recalculate(&mut self.document);
        self.topology_version = self.topology_version.wrapping_add(topology_changes);

        self.remove_stale_runtime_references();
    }

    fn remove_stale_runtime_references(&mut self) {
        let component_ids: HashSet<u64> = self
            .document
            .components
            .iter()
            .map(|component| component.id)
            .collect();
        let wire_point_counts: HashMap<u64, usize> = self
            .document
            .wires
            .iter()
            .map(|wire| (wire.id, wire.points.len()))
            .collect();
        let junction_positions: HashSet<Point> = self
            .document
            .junctions
            .iter()
            .map(|junction| junction.pos)
            .collect();
        let net_label_ids: HashSet<u64> = self
            .document
            .net_labels
            .iter()
            .map(|label| label.id)
            .collect();
        let bus_ids: HashSet<u64> = self.document.buses.iter().map(|bus| bus.id).collect();
        let bus_tap_ids: HashSet<u64> = self.document.bus_taps.iter().map(|tap| tap.id).collect();
        let design_note_ids: HashSet<u64> = self
            .document
            .design_notes
            .iter()
            .map(|note| note.id)
            .collect();
        let documentation_shape_ids: HashSet<u64> = self
            .document
            .documentation_shapes
            .iter()
            .map(|shape| shape.id)
            .collect();
        let probe_ids: HashSet<u64> = self.document.probes.iter().map(|probe| probe.id).collect();

        self.selection
            .components
            .retain(|id| component_ids.contains(id));
        self.selection
            .wires
            .retain(|id| wire_point_counts.contains_key(id));
        self.selection.wire_segments.retain(|segment| {
            wire_point_counts
                .get(&segment.wire_id)
                .is_some_and(|point_count| segment.segment_index < point_count.saturating_sub(1))
        });
        self.selection.wire_vertices.retain(|vertex| {
            wire_point_counts
                .get(&vertex.wire_id)
                .is_some_and(|point_count| vertex.vertex_index < *point_count)
        });
        self.selection
            .junctions
            .retain(|junction| junction_positions.contains(&junction.pos));
        self.selection
            .net_labels
            .retain(|id| net_label_ids.contains(id));
        self.selection.buses.retain(|id| bus_ids.contains(id));
        self.selection
            .bus_taps
            .retain(|id| bus_tap_ids.contains(id));
        self.selection
            .design_notes
            .retain(|id| design_note_ids.contains(id));
        self.selection
            .documentation_shapes
            .retain(|id| documentation_shape_ids.contains(id));
        self.selection.probes.retain(|id| probe_ids.contains(id));

        self.document.connections.retain(|connection| {
            component_ids.contains(&connection.component_id)
                && wire_point_counts
                    .get(&connection.wire_id)
                    .is_some_and(|point_count| connection.point_index < *point_count)
        });
    }

    /// Generate a unique component name
    pub fn generate_name(&mut self, kind: ComponentType) -> String {
        self.identity.generate_name(kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
        DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry, Junction,
        NetLabel, Point, SchematicProbe, SchematicState,
    };
    use std::collections::HashSet;

    #[test]
    fn recalculated_schematic_does_not_reuse_component_ids() {
        let mut schematic = SchematicState::default();
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        schematic.add_component(ComponentType::Capacitor, Point::new(10, 0));

        // Workspace buffers round-trip through serde, which skips the
        // runtime ID counter — simulate the post-deserialization state.
        schematic.identity = super::SchematicIdentity::default();
        schematic.recalculate_runtime_state();

        let new_id = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
        let matches = schematic
            .document
            .components
            .iter()
            .filter(|component| component.id == new_id)
            .count();
        assert_eq!(matches, 1, "a fresh component must get a unique id");
    }

    #[test]
    fn recalculate_repairs_duplicate_component_ids() {
        let mut schematic = SchematicState::default();
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        schematic.add_component(ComponentType::Capacitor, Point::new(10, 0));
        let stolen = schematic.document.components[0].id;
        schematic.document.components[1].id = stolen;

        schematic.recalculate_runtime_state();

        let matches = schematic
            .document
            .components
            .iter()
            .filter(|component| component.id == stolen)
            .count();
        assert_eq!(matches, 1, "duplicate ids must be reassigned");
    }

    #[test]
    fn recalculate_deduplicates_junction_positions_and_repairs_ids() {
        let mut schematic = SchematicState::default();
        schematic.document.junctions = vec![
            Junction::new(7, Point::new(10, 10)),
            Junction::new(7, Point::new(20, 20)),
            Junction::new(9, Point::new(10, 10)),
        ];
        schematic.selection.select_junction(Point::new(10, 10));
        schematic.selection.select_junction(Point::new(20, 20));

        schematic.recalculate_runtime_state();

        assert_eq!(schematic.document.junctions.len(), 2);
        assert_eq!(
            schematic.document.junctions[0],
            Junction::new(7, Point::new(10, 10))
        );
        assert_ne!(
            schematic.document.junctions[0].id,
            schematic.document.junctions[1].id
        );
        assert!(schematic.selection.has_junction(Point::new(10, 10)));
        assert!(schematic.selection.has_junction(Point::new(20, 20)));
        let new_id = schematic.add_junction(Point::new(30, 30));
        assert_eq!(
            schematic
                .document
                .junctions
                .iter()
                .filter(|junction| junction.id == new_id)
                .count(),
            1
        );
    }

    #[test]
    fn recalculate_drops_stale_net_label_selection_ids() {
        let mut schematic = SchematicState::default();
        schematic
            .document
            .net_labels
            .push(NetLabel::new(50, Point::new(1, 2), "live"));
        schematic.selection.select_net_label(50);
        schematic.selection.select_net_label(999);

        schematic.recalculate_runtime_state();

        assert!(schematic.selection.has_net_label(50));
        assert!(!schematic.selection.has_net_label(999));
        assert_eq!(schematic.selection.single_net_label(), Some(50));
    }

    #[test]
    fn recalculate_repairs_duplicate_and_cross_class_net_label_ids() {
        let mut schematic = SchematicState::default();
        schematic.document.components.push(Component::new(
            59,
            ComponentType::Resistor,
            Point::origin(),
        ));
        schematic.document.net_labels = vec![
            NetLabel::new(60, Point::new(1, 2), "first"),
            NetLabel::new(60, Point::new(3, 4), "second"),
            NetLabel::new(59, Point::new(5, 6), "component_collision"),
        ];

        schematic.recalculate_runtime_state();

        assert_eq!(schematic.document.net_labels[0].id, 60);
        assert_ne!(schematic.document.net_labels[1].id, 60);
        assert_ne!(schematic.document.net_labels[2].id, 59);
        assert_ne!(
            schematic.document.net_labels[0].id,
            schematic.document.net_labels[1].id
        );
        assert_ne!(
            schematic.document.net_labels[1].id,
            schematic.document.net_labels[2].id
        );
        assert_eq!(schematic.document.components[0].id, 59);
    }

    #[test]
    fn recalculate_repairs_review_note_identity_and_lifecycle_record_together() {
        let mut schematic = SchematicState::default();
        schematic.document.components.push(Component::new(
            59,
            ComponentType::Resistor,
            Point::origin(),
        ));
        schematic.document.design_notes.push(
            DesignNote::new(
                59,
                Point::new(5, 6),
                DesignNoteKind::ReviewNote,
                "Review bias path",
            )
            .unwrap(),
        );
        schematic.selection.select_design_note(59);
        schematic.selection.select_design_note(999);
        let topology = schematic.topology_version();

        schematic.recalculate_runtime_state();

        let note = &schematic.document.design_notes[0];
        assert_ne!(note.id, 59);
        assert_eq!(
            note.review.as_ref().unwrap().record_id,
            format!("NOTE-{:04}", note.id)
        );
        assert!(!schematic.selection.has_design_note(59));
        assert!(!schematic.selection.has_design_note(999));
        assert_eq!(schematic.topology_version(), topology);
    }

    #[test]
    fn recalculate_repairs_bus_identity_and_preserves_tap_ownership() {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus = Bus::segment(1, Point::new(0, 0), Point::new(10, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            1,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[1]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic.document.buses.push(bus);
        schematic.document.bus_taps.push(tap);

        schematic.recalculate_runtime_state();

        assert_ne!(
            schematic.document.buses[0].id,
            schematic.document.components[0].id
        );
        assert_eq!(
            schematic.document.bus_taps[0].bus_id,
            schematic.document.buses[0].id
        );
        assert_ne!(
            schematic.document.bus_taps[0].id,
            schematic.document.buses[0].id
        );
    }

    #[test]
    fn recalculate_wraps_from_max_id_without_panicking_or_reusing_live_identity() {
        let mut schematic = SchematicState::default();
        let mut component =
            crate::state::Component::new(u64::MAX, ComponentType::Resistor, Point::origin());
        component.name = "RMAX".to_owned();
        schematic.document.components.push(component);

        schematic.recalculate_runtime_state();
        let new_id = schematic.add_component(ComponentType::Capacitor, Point::new(10, 0));

        assert_ne!(new_id, u64::MAX);
        assert_eq!(new_id, 1);
        assert_eq!(
            schematic
                .document
                .components
                .iter()
                .map(|component| component.id)
                .collect::<HashSet<_>>()
                .len(),
            schematic.document.components.len()
        );
    }

    #[test]
    fn recalculate_preserves_malformed_bus_records_for_recovery_and_drc() {
        let mut schematic = SchematicState::default();
        schematic.document.buses.push(Bus {
            id: 70,
            points: vec![Point::new(3, 4)],
            declaration: None,
        });
        schematic.document.bus_taps.push(BusTap {
            id: 71,
            bus_id: 70,
            bus_point: Point::new(3, 4),
            connection_point: Point::new(3, 10),
            slice: BusSlice::parse("DATA[0]").unwrap(),
            orientation: BusTapOrientation::Down,
        });

        schematic.recalculate_runtime_state();
        let analysis =
            rspice_design::connectivity::bus::analyze_bus_connectivity(&schematic.document);

        assert_eq!(schematic.document.buses.len(), 1);
        assert_eq!(schematic.document.bus_taps.len(), 1);
        assert!(analysis.diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == rspice_design::connectivity::bus::BusDiagnosticKind::MalformedBus
                && diagnostic.bus_id == Some(70)
        }));
        assert!(
            analysis
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.tap_id == Some(71))
        );
    }

    #[test]
    fn legacy_state_without_new_object_fields_migrates_to_empty_collections() {
        let state = SchematicState::default();
        let mut value = serde_json::to_value(state).unwrap();
        let object = value.as_object_mut().unwrap();
        object.remove("buses");
        object.remove("bus_taps");
        object.remove("net_labels");
        object.remove("design_notes");
        object.remove("probes");
        let migrated: SchematicState = serde_json::from_value(value).unwrap();
        assert!(migrated.document.buses.is_empty());
        assert!(migrated.document.bus_taps.is_empty());
        assert!(migrated.document.net_labels.is_empty());
        assert!(migrated.document.design_notes.is_empty());
        assert!(migrated.document.probes.is_empty());
    }

    #[test]
    fn recalculate_repairs_documentation_shape_collisions_and_stale_selection() {
        let mut schematic = SchematicState::default();
        schematic.document.components.push(Component::new(
            59,
            ComponentType::Resistor,
            Point::origin(),
        ));
        schematic.document.documentation_shapes = vec![
            DocumentationShape::new(
                59,
                DocumentationShapeGeometry::Line {
                    start: Point::new(0, 0),
                    end: Point::new(10, 0),
                },
            )
            .unwrap(),
            DocumentationShape::new(
                59,
                DocumentationShapeGeometry::Rectangle {
                    first: Point::new(20, 20),
                    opposite: Point::new(30, 30),
                },
            )
            .unwrap(),
        ];
        schematic.selection.select_documentation_shape(59);
        schematic.selection.select_documentation_shape(999);
        let topology = schematic.topology_version();

        schematic.recalculate_runtime_state();

        let ids: HashSet<_> = schematic
            .document
            .documentation_shapes
            .iter()
            .map(|shape| shape.id)
            .collect();
        assert_eq!(ids.len(), 2);
        assert!(
            !ids.contains(&59),
            "component identity retains namespace priority"
        );
        assert!(ids.iter().all(|id| *id != 0));
        assert!(schematic.selection.documentation_shapes.is_empty());
        assert_eq!(schematic.topology_version(), topology);
        let fresh = schematic.next_id();
        assert_ne!(fresh, 59);
        assert!(!ids.contains(&fresh));
    }

    #[test]
    fn recalculate_repairs_probe_collisions_and_prunes_stale_probe_selection() {
        let mut schematic = SchematicState::default();
        schematic.document.components.push(Component::new(
            59,
            ComponentType::Resistor,
            Point::origin(),
        ));
        schematic.document.probes = vec![
            SchematicProbe::new(59, Point::new(10, 20), "V(out)", Some("V(out)".to_owned()))
                .unwrap(),
            SchematicProbe::new(59, Point::new(30, 40), "V(in)", Some("V(in)".to_owned())).unwrap(),
        ];
        schematic.selection.select_probe(59);
        schematic.selection.select_probe(999);
        let topology = schematic.topology_version();

        schematic.recalculate_runtime_state();

        let ids: HashSet<_> = schematic
            .document
            .probes
            .iter()
            .map(|probe| probe.id)
            .collect();
        assert_eq!(ids.len(), 2);
        assert!(!ids.contains(&59), "component identity retains priority");
        assert!(schematic.selection.probes.is_empty());
        assert_eq!(schematic.topology_version(), topology);
        let fresh = schematic.next_id();
        assert_ne!(fresh, 59);
        assert!(!ids.contains(&fresh));
    }
}
