use super::*;
use std::collections::{HashMap, HashSet};

impl SchematicState {
    // =========================================================================
    // ID and Name Generation
    // =========================================================================

    /// Generate a unique ID
    pub fn next_id(&mut self) -> u64 {
        loop {
            let id = self.next_id.max(1);
            self.next_id = id.checked_add(1).unwrap_or(1);
            if !self.live_id_in_use(id) {
                return id;
            }
        }
    }

    fn live_id_in_use(&self, id: u64) -> bool {
        self.components.iter().any(|item| item.id == id)
            || self.wires.iter().any(|item| item.id == id)
            || self.junctions.iter().any(|item| item.id == id)
            || self.net_labels.iter().any(|item| item.id == id)
            || self.buses.iter().any(|item| item.id == id)
            || self.bus_taps.iter().any(|item| item.id == id)
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
        let wire_count_before_repair = self.wires.len();
        self.wires.retain(|wire| wire.points.len() >= 2);
        self.clipboard.wires.retain(|wire| wire.points.len() >= 2);
        self.clipboard.buses.retain(|bus| bus.validate().is_ok());
        if self.wires.len() != wire_count_before_repair {
            self.bump_topology_version();
        }

        // Find the maximum ID currently in use across every collection that
        // allocates from the shared counter (components, wires, junctions,
        // and net labels).
        let max_component_id = self.components.iter().map(|c| c.id).max().unwrap_or(0);
        let max_wire_id = self.wires.iter().map(|w| w.id).max().unwrap_or(0);
        let max_junction_id = self.junctions.iter().map(|j| j.id).max().unwrap_or(0);
        let max_label_id = self.net_labels.iter().map(|l| l.id).max().unwrap_or(0);
        let max_bus_id = self.buses.iter().map(|bus| bus.id).max().unwrap_or(0);
        let max_bus_tap_id = self.bus_taps.iter().map(|tap| tap.id).max().unwrap_or(0);
        let max_id = max_component_id
            .max(max_wire_id)
            .max(max_junction_id)
            .max(max_label_id)
            .max(max_bus_id)
            .max(max_bus_tap_id);
        self.next_id = max_id.checked_add(1).unwrap_or(1);

        // Repair duplicate component IDs left behind by earlier counter
        // collisions: later duplicates get fresh IDs. Their wire
        // connections were ambiguous (keyed by the shared ID) and stay
        // with the first occurrence.
        let mut seen = std::collections::HashSet::with_capacity(self.components.len());
        let duplicates: Vec<usize> = self
            .components
            .iter()
            .enumerate()
            .filter(|(_, component)| !seen.insert(component.id))
            .map(|(index, _)| index)
            .collect();
        for index in duplicates {
            self.components[index].id = self.next_id();
        }

        // Repair duplicate live wire IDs as well. Wire operations are keyed by
        // ID, so keeping duplicates would make edits apply inconsistently.
        let mut seen = HashSet::with_capacity(self.wires.len());
        let duplicates: Vec<usize> = self
            .wires
            .iter()
            .enumerate()
            .filter(|(_, wire)| !seen.insert(wire.id))
            .map(|(index, _)| index)
            .collect();
        if !duplicates.is_empty() {
            for index in duplicates {
                self.wires[index].id = self.next_id();
            }
            self.bump_topology_version();
        }

        // A junction position has one electrical meaning and one stable
        // identity. Preserve the first record at each position, then repair
        // duplicate IDs among the remaining records so ID-based removal can
        // never remove multiple markers or reveal a hidden duplicate.
        let junction_count_before_repair = self.junctions.len();
        let mut seen_positions = HashSet::with_capacity(self.junctions.len());
        self.junctions
            .retain(|junction| seen_positions.insert(junction.pos));
        let mut seen_ids = HashSet::with_capacity(self.junctions.len());
        let duplicate_junction_ids: Vec<usize> = self
            .junctions
            .iter()
            .enumerate()
            .filter(|(_, junction)| !seen_ids.insert(junction.id))
            .map(|(index, _)| index)
            .collect();
        let junctions_repaired = self.junctions.len() != junction_count_before_repair
            || !duplicate_junction_ids.is_empty();
        for index in duplicate_junction_ids {
            let replacement_id = self.next_id();
            self.junctions[index].id = replacement_id;
        }
        if junctions_repaired {
            self.bump_topology_version();
        }

        // Net labels are edited and selected by stable ID. Components, wires,
        // and junctions are retained first in the document-wide namespace;
        // repair colliding labels and later label duplicates deterministically.
        let mut occupied_label_ids: HashSet<u64> = self
            .components
            .iter()
            .map(|item| item.id)
            .chain(self.wires.iter().map(|item| item.id))
            .chain(self.junctions.iter().map(|item| item.id))
            .collect();
        let mut seen_label_ids = HashSet::with_capacity(self.net_labels.len());
        let colliding_label_ids: Vec<usize> = self
            .net_labels
            .iter()
            .enumerate()
            .filter(|(_, label)| {
                !seen_label_ids.insert(label.id) || occupied_label_ids.contains(&label.id)
            })
            .map(|(index, _)| index)
            .collect();
        if !colliding_label_ids.is_empty() {
            for index in colliding_label_ids {
                let replacement = self.next_id();
                self.net_labels[index].id = replacement;
                occupied_label_ids.insert(replacement);
            }
            self.bump_topology_version();
        }

        // Bus and tap IDs share the document-wide identity namespace. Repair
        // collisions with legacy object IDs and update tap ownership when the
        // first occurrence of a bus ID is reassigned.
        let mut occupied_ids: HashSet<u64> = self
            .components
            .iter()
            .map(|item| item.id)
            .chain(self.wires.iter().map(|item| item.id))
            .chain(self.junctions.iter().map(|item| item.id))
            .chain(self.net_labels.iter().map(|item| item.id))
            .collect();
        let mut seen_bus_ids = HashSet::with_capacity(self.buses.len());
        let mut bus_id_remap = HashMap::new();
        let mut bus_ids_repaired = false;
        for index in 0..self.buses.len() {
            let old_id = self.buses[index].id;
            let first_bus_with_id = seen_bus_ids.insert(old_id);
            if occupied_ids.insert(old_id) {
                continue;
            }
            let replacement = self.next_id();
            occupied_ids.insert(replacement);
            self.buses[index].id = replacement;
            if first_bus_with_id {
                bus_id_remap.insert(old_id, replacement);
            }
            bus_ids_repaired = true;
        }
        for tap in &mut self.bus_taps {
            if let Some(replacement) = bus_id_remap.get(&tap.bus_id) {
                tap.bus_id = *replacement;
            }
        }

        for index in 0..self.bus_taps.len() {
            let id = self.bus_taps[index].id;
            if !occupied_ids.insert(id) {
                let replacement = self.next_id();
                occupied_ids.insert(replacement);
                self.bus_taps[index].id = replacement;
                bus_ids_repaired = true;
            }
        }
        if bus_ids_repaired {
            self.bump_topology_version();
        }

        // Rebuild component counters from existing component names
        self.component_counters.clear();
        for comp in &self.components {
            let prefix = comp.kind.spice_prefix();
            if !prefix.is_empty() {
                // Extract number from name like "R1", "C5", etc.
                if let Some(num_str) = comp.name.strip_prefix(prefix)
                    && let Ok(num) = num_str.parse::<u32>()
                {
                    let counter = self.component_counters.entry(prefix).or_insert(0);
                    *counter = (*counter).max(num);
                }
            }
        }

        self.remove_stale_runtime_references();
    }

    fn remove_stale_runtime_references(&mut self) {
        let component_ids: HashSet<u64> = self
            .components
            .iter()
            .map(|component| component.id)
            .collect();
        let wire_point_counts: HashMap<u64, usize> = self
            .wires
            .iter()
            .map(|wire| (wire.id, wire.points.len()))
            .collect();
        let junction_positions: HashSet<Point> =
            self.junctions.iter().map(|junction| junction.pos).collect();
        let net_label_ids: HashSet<u64> = self.net_labels.iter().map(|label| label.id).collect();
        let bus_ids: HashSet<u64> = self.buses.iter().map(|bus| bus.id).collect();
        let bus_tap_ids: HashSet<u64> = self.bus_taps.iter().map(|tap| tap.id).collect();

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

        self.connections.retain(|connection| {
            component_ids.contains(&connection.component_id)
                && wire_point_counts
                    .get(&connection.wire_id)
                    .is_some_and(|point_count| connection.point_index < *point_count)
        });
    }

    /// Generate a unique component name
    pub fn generate_name(&mut self, kind: ComponentType) -> String {
        let prefix = kind.spice_prefix();
        if prefix.is_empty() {
            return String::new();
        }
        let counter = self.component_counters.entry(prefix).or_insert(0);
        *counter += 1;
        format!("{}{}", prefix, counter)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
        Junction, NetLabel, Point, SchematicState,
    };
    use std::collections::HashSet;

    #[test]
    fn recalculated_schematic_does_not_reuse_component_ids() {
        let mut schematic = SchematicState::default();
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        schematic.add_component(ComponentType::Capacitor, Point::new(10, 0));

        // Workspace buffers round-trip through serde, which skips the
        // runtime ID counter — simulate the post-deserialization state.
        schematic.next_id = 0;
        schematic.recalculate_runtime_state();

        let new_id = schematic.add_component(ComponentType::Resistor, Point::new(20, 0));
        let matches = schematic
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
        let stolen = schematic.components[0].id;
        schematic.components[1].id = stolen;

        schematic.recalculate_runtime_state();

        let matches = schematic
            .components
            .iter()
            .filter(|component| component.id == stolen)
            .count();
        assert_eq!(matches, 1, "duplicate ids must be reassigned");
    }

    #[test]
    fn recalculate_deduplicates_junction_positions_and_repairs_ids() {
        let mut schematic = SchematicState::default();
        schematic.junctions = vec![
            Junction::new(7, Point::new(10, 10)),
            Junction::new(7, Point::new(20, 20)),
            Junction::new(9, Point::new(10, 10)),
        ];
        schematic.selection.select_junction(Point::new(10, 10));
        schematic.selection.select_junction(Point::new(20, 20));

        schematic.recalculate_runtime_state();

        assert_eq!(schematic.junctions.len(), 2);
        assert_eq!(schematic.junctions[0], Junction::new(7, Point::new(10, 10)));
        assert_ne!(schematic.junctions[0].id, schematic.junctions[1].id);
        assert!(schematic.selection.has_junction(Point::new(10, 10)));
        assert!(schematic.selection.has_junction(Point::new(20, 20)));
        let new_id = schematic.add_junction(Point::new(30, 30));
        assert_eq!(
            schematic
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
        schematic
            .components
            .push(Component::new(59, ComponentType::Resistor, Point::origin()));
        schematic.net_labels = vec![
            NetLabel::new(60, Point::new(1, 2), "first"),
            NetLabel::new(60, Point::new(3, 4), "second"),
            NetLabel::new(59, Point::new(5, 6), "component_collision"),
        ];

        schematic.recalculate_runtime_state();

        assert_eq!(schematic.net_labels[0].id, 60);
        assert_ne!(schematic.net_labels[1].id, 60);
        assert_ne!(schematic.net_labels[2].id, 59);
        assert_ne!(schematic.net_labels[0].id, schematic.net_labels[1].id);
        assert_ne!(schematic.net_labels[1].id, schematic.net_labels[2].id);
        assert_eq!(schematic.components[0].id, 59);
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
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);

        schematic.recalculate_runtime_state();

        assert_ne!(schematic.buses[0].id, schematic.components[0].id);
        assert_eq!(schematic.bus_taps[0].bus_id, schematic.buses[0].id);
        assert_ne!(schematic.bus_taps[0].id, schematic.buses[0].id);
    }

    #[test]
    fn recalculate_wraps_from_max_id_without_panicking_or_reusing_live_identity() {
        let mut schematic = SchematicState::default();
        let mut component =
            crate::state::Component::new(u64::MAX, ComponentType::Resistor, Point::origin());
        component.name = "RMAX".to_owned();
        schematic.components.push(component);

        schematic.recalculate_runtime_state();
        let new_id = schematic.add_component(ComponentType::Capacitor, Point::new(10, 0));

        assert_ne!(new_id, u64::MAX);
        assert_eq!(new_id, 1);
        assert_eq!(
            schematic
                .components
                .iter()
                .map(|component| component.id)
                .collect::<HashSet<_>>()
                .len(),
            schematic.components.len()
        );
    }

    #[test]
    fn recalculate_preserves_malformed_bus_records_for_recovery_and_drc() {
        let mut schematic = SchematicState::default();
        schematic.buses.push(Bus {
            id: 70,
            points: vec![Point::new(3, 4)],
            declaration: None,
        });
        schematic.bus_taps.push(BusTap {
            id: 71,
            bus_id: 70,
            bus_point: Point::new(3, 4),
            connection_point: Point::new(3, 10),
            slice: BusSlice::parse("DATA[0]").unwrap(),
            orientation: BusTapOrientation::Down,
        });

        schematic.recalculate_runtime_state();
        let analysis = crate::schematic::bus_connectivity::analyze_bus_connectivity(&schematic);

        assert_eq!(schematic.buses.len(), 1);
        assert_eq!(schematic.bus_taps.len(), 1);
        assert!(analysis.diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == crate::schematic::bus_connectivity::BusDiagnosticKind::MalformedBus
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
        let migrated: SchematicState = serde_json::from_value(value).unwrap();
        assert!(migrated.buses.is_empty());
        assert!(migrated.bus_taps.is_empty());
        assert!(migrated.net_labels.is_empty());
    }
}
