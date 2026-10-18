//! Schematic object identity, reference counters, and deterministic load repair.

use super::{component_type::ComponentType, document::SchematicDocument};
use std::collections::{HashMap, HashSet};

/// Runtime allocation state for one schematic, independent of its editor.
/// The zero default preserves the loaded-state cursor; allocation starts at one.
#[derive(Debug, Clone, Default)]
pub struct SchematicIdentity {
    next_id: u64,
    component_counters: HashMap<&'static str, u32>,
}

/// Live document identities reused when pruning editor references after repair.
pub struct DocumentRepair {
    pub topology_changes: u64,
    pub component_ids: HashSet<u64>,
    pub wire_point_counts: HashMap<u64, usize>,
}

impl SchematicIdentity {
    /// Repair persisted geometry, identities and connections, retaining the
    /// live-object indices needed by callers to reconcile runtime references.
    pub fn repair_document(&mut self, document: &mut SchematicDocument) -> DocumentRepair {
        let wire_count_before_repair = document.wires.len();
        document.wires.retain(|wire| wire.points.len() >= 2);
        document
            .documentation_shapes
            .retain(|shape| shape.validate().is_ok());
        let topology_changes = u64::from(document.wires.len() != wire_count_before_repair)
            .wrapping_add(self.recalculate(document));
        let component_ids: HashSet<u64> = document
            .components
            .iter()
            .map(|component| component.id)
            .collect();
        let wire_point_counts: HashMap<u64, usize> = document
            .wires
            .iter()
            .map(|wire| (wire.id, wire.points.len()))
            .collect();
        document.connections.retain(|connection| {
            component_ids.contains(&connection.component_id)
                && wire_point_counts
                    .get(&connection.wire_id)
                    .is_some_and(|point_count| connection.point_index < *point_count)
        });
        DocumentRepair {
            topology_changes,
            component_ids,
            wire_point_counts,
        }
    }

    pub fn with_cursor(next_id: u64) -> Self {
        Self {
            next_id,
            ..Self::default()
        }
    }

    /// Generate a unique ID
    pub fn allocate(&mut self, document: &SchematicDocument) -> u64 {
        self.allocate_unless(|id| live_id_in_use(document, id))
    }

    /// [`Self::allocate`] for a drawing that is being rewritten and has not
    /// been written back yet: `in_use` says which ids it holds.
    pub(crate) fn allocate_unless(&mut self, in_use: impl Fn(u64) -> bool) -> u64 {
        loop {
            let id = self.next_id.max(1);
            self.next_id = id.checked_add(1).unwrap_or(1);
            if !in_use(id) {
                return id;
            }
        }
    }

    /// Current allocator cursor used to key immutable transaction previews.
    /// Reading it never consumes an identity.
    pub const fn cursor(&self) -> u64 {
        self.next_id
    }

    /// The next free reference name for a new part of `kind` in `document`.
    ///
    /// A number written by a rename or an import counts as used, so the next
    /// name follows the highest one present, and a name already present in
    /// any letter case is never handed out again.
    pub fn generate_name(&mut self, document: &SchematicDocument, kind: ComponentType) -> String {
        let prefix = kind.spice_prefix();
        if prefix.is_empty() {
            return String::new();
        }
        let highest = document
            .components
            .iter()
            .filter_map(|component| reference_number(&component.name, prefix))
            .max()
            .unwrap_or(0);
        let counter = self.component_counters.entry(prefix).or_insert(0);
        *counter = (*counter).max(highest);
        loop {
            *counter = counter.saturating_add(1);
            let candidate = format!("{prefix}{counter}");
            let taken = document
                .components
                .iter()
                .any(|component| component.name.trim().eq_ignore_ascii_case(&candidate));
            if !taken || *counter == u32::MAX {
                return candidate;
            }
        }
    }

    pub fn record_component_number(&mut self, prefix: &'static str, number: u32) {
        let counter = self.component_counters.entry(prefix).or_insert(0);
        *counter = (*counter).max(number);
    }

    pub fn commit_array(&mut self, document: &SchematicDocument, next_id: u64) {
        self.next_id = next_id;
        for component in &document.components {
            let prefix = component.kind.spice_prefix();
            if let Some(number) = component
                .name
                .strip_prefix(prefix)
                .and_then(|suffix| suffix.parse::<u32>().ok())
            {
                let counter = self.component_counters.entry(prefix).or_insert(0);
                *counter = (*counter).max(number);
            }
        }
    }

    /// Rebuild allocation state and repair live object identities after loading.
    /// Returns the original number of electrical topology invalidations.
    pub fn recalculate(&mut self, document: &mut SchematicDocument) -> u64 {
        let mut topology_changes = 0;
        // Find the maximum ID currently in use across every collection that
        // allocates from the shared counter (components, wires, junctions,
        // and net labels).
        let max_component_id = document.components.iter().map(|c| c.id).max().unwrap_or(0);
        let max_wire_id = document.wires.iter().map(|w| w.id).max().unwrap_or(0);
        let max_junction_id = document.junctions.iter().map(|j| j.id).max().unwrap_or(0);
        let max_label_id = document.net_labels.iter().map(|l| l.id).max().unwrap_or(0);
        let max_bus_id = document.buses.iter().map(|bus| bus.id).max().unwrap_or(0);
        let max_bus_tap_id = document
            .bus_taps
            .iter()
            .map(|tap| tap.id)
            .max()
            .unwrap_or(0);
        let max_design_note_id = document
            .design_notes
            .iter()
            .map(|note| note.id)
            .max()
            .unwrap_or(0);
        let max_documentation_shape_id = document
            .documentation_shapes
            .iter()
            .map(|shape| shape.id)
            .max()
            .unwrap_or(0);
        let max_probe_id = document
            .probes
            .iter()
            .map(|probe| probe.id)
            .max()
            .unwrap_or(0);
        let max_id = max_component_id
            .max(max_wire_id)
            .max(max_junction_id)
            .max(max_label_id)
            .max(max_bus_id)
            .max(max_bus_tap_id)
            .max(max_design_note_id)
            .max(max_documentation_shape_id)
            .max(max_probe_id);
        self.next_id = max_id.checked_add(1).unwrap_or(1);

        // Repair duplicate component IDs left behind by earlier counter
        // collisions: later duplicates get fresh IDs. Their wire
        // connections were ambiguous (keyed by the shared ID) and stay
        // with the first occurrence.
        let mut seen = std::collections::HashSet::with_capacity(document.components.len());
        let duplicates: Vec<usize> = document
            .components
            .iter()
            .enumerate()
            .filter(|(_, component)| !seen.insert(component.id))
            .map(|(index, _)| index)
            .collect();
        for index in duplicates {
            document.components[index].id = self.allocate(document);
        }

        // Repair duplicate live wire IDs as well. Wire operations are keyed by
        // ID, so keeping duplicates would make edits apply inconsistently.
        let mut seen = HashSet::with_capacity(document.wires.len());
        let duplicates: Vec<usize> = document
            .wires
            .iter()
            .enumerate()
            .filter(|(_, wire)| !seen.insert(wire.id))
            .map(|(index, _)| index)
            .collect();
        if !duplicates.is_empty() {
            for index in duplicates {
                document.wires[index].id = self.allocate(document);
            }
            topology_changes += 1;
        }

        // Every sheet of a cell view lives in this one document and shares its
        // coordinates, so two junction records at one position can be dots on
        // two sheets. Neither may be dropped here: this repair runs on every
        // undo and redo and cannot see which sheet holds which record, and a
        // dropped dot disconnects the crossing it marked. Two records at one
        // point on the same sheet still mean one junction, and the sheet's
        // next junction maintenance rebuilds its dots. Repair duplicate IDs
        // only, so ID-based removal can never remove two markers at once.
        let junction_count_before_repair = document.junctions.len();
        let mut seen_ids = HashSet::with_capacity(document.junctions.len());
        let duplicate_junction_ids: Vec<usize> = document
            .junctions
            .iter()
            .enumerate()
            .filter(|(_, junction)| !seen_ids.insert(junction.id))
            .map(|(index, _)| index)
            .collect();
        let junctions_repaired = document.junctions.len() != junction_count_before_repair
            || !duplicate_junction_ids.is_empty();
        for index in duplicate_junction_ids {
            let replacement_id = self.allocate(document);
            document.junctions[index].id = replacement_id;
        }
        if junctions_repaired {
            topology_changes += 1;
        }

        // Net labels are edited and selected by stable ID. Components, wires,
        // and junctions are retained first in the document-wide namespace;
        // repair colliding labels and later label duplicates deterministically.
        let mut occupied_label_ids: HashSet<u64> = document
            .components
            .iter()
            .map(|item| item.id)
            .chain(document.wires.iter().map(|item| item.id))
            .chain(document.junctions.iter().map(|item| item.id))
            .collect();
        let mut seen_label_ids = HashSet::with_capacity(document.net_labels.len());
        let colliding_label_ids: Vec<usize> = document
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
                let replacement = self.allocate(document);
                document.net_labels[index].id = replacement;
                occupied_label_ids.insert(replacement);
            }
            topology_changes += 1;
        }

        // Bus and tap IDs share the document-wide identity namespace. Repair
        // collisions with legacy object IDs and update tap ownership when the
        // first occurrence of a bus ID is reassigned.
        let mut occupied_ids: HashSet<u64> = document
            .components
            .iter()
            .map(|item| item.id)
            .chain(document.wires.iter().map(|item| item.id))
            .chain(document.junctions.iter().map(|item| item.id))
            .chain(document.net_labels.iter().map(|item| item.id))
            .collect();
        let mut seen_bus_ids = HashSet::with_capacity(document.buses.len());
        let mut bus_id_remap = HashMap::new();
        let mut bus_ids_repaired = false;
        for index in 0..document.buses.len() {
            let old_id = document.buses[index].id;
            let first_bus_with_id = seen_bus_ids.insert(old_id);
            if occupied_ids.insert(old_id) {
                continue;
            }
            let replacement = self.allocate(document);
            occupied_ids.insert(replacement);
            document.buses[index].id = replacement;
            if first_bus_with_id {
                bus_id_remap.insert(old_id, replacement);
            }
            bus_ids_repaired = true;
        }
        for tap in &mut document.bus_taps {
            if let Some(replacement) = bus_id_remap.get(&tap.bus_id) {
                tap.bus_id = *replacement;
            }
        }

        for index in 0..document.bus_taps.len() {
            let id = document.bus_taps[index].id;
            if !occupied_ids.insert(id) {
                let replacement = self.allocate(document);
                occupied_ids.insert(replacement);
                document.bus_taps[index].id = replacement;
                bus_ids_repaired = true;
            }
        }
        if bus_ids_repaired {
            topology_changes += 1;
        }

        // Documentation IDs share the same stable document namespace but do
        // not alter electrical topology. Repair collisions deterministically
        // and keep review record IDs aligned with the repaired object ID.
        let mut occupied_ids: HashSet<u64> = document
            .components
            .iter()
            .map(|item| item.id)
            .chain(document.wires.iter().map(|item| item.id))
            .chain(document.junctions.iter().map(|item| item.id))
            .chain(document.net_labels.iter().map(|item| item.id))
            .chain(document.buses.iter().map(|item| item.id))
            .chain(document.bus_taps.iter().map(|item| item.id))
            .collect();
        for index in 0..document.design_notes.len() {
            let id = document.design_notes[index].id;
            if !occupied_ids.insert(id) {
                let replacement = self.allocate(document);
                occupied_ids.insert(replacement);
                document.design_notes[index].id = replacement;
            }
            let note_id = document.design_notes[index].id;
            if let Some(review) = document.design_notes[index].review.as_mut() {
                review.record_id = format!("NOTE-{note_id:04}");
            }
        }
        document.design_notes.retain(|note| note.validate().is_ok());

        for index in 0..document.documentation_shapes.len() {
            let id = document.documentation_shapes[index].id;
            if !occupied_ids.insert(id) {
                let replacement = self.allocate(document);
                occupied_ids.insert(replacement);
                document.documentation_shapes[index].id = replacement;
            }
        }
        for index in 0..document.probes.len() {
            let id = document.probes[index].id;
            if !occupied_ids.insert(id) {
                let replacement = self.allocate(document);
                occupied_ids.insert(replacement);
                document.probes[index].id = replacement;
            }
        }
        document.probes.retain(|probe| probe.validate().is_ok());

        // Rebuild component counters from existing component names
        self.component_counters.clear();
        for comp in &document.components {
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

        topology_changes
    }
}

/// The number after `prefix` in a reference name such as `R10` or `r10`.
fn reference_number(name: &str, prefix: &str) -> Option<u32> {
    let name = name.trim();
    let head = name.get(..prefix.len())?;
    if !head.eq_ignore_ascii_case(prefix) {
        return None;
    }
    name[prefix.len()..].parse().ok()
}

pub(crate) fn live_id_in_use(document: &SchematicDocument, id: u64) -> bool {
    document.components.iter().any(|item| item.id == id)
        || document.wires.iter().any(|item| item.id == id)
        || document.junctions.iter().any(|item| item.id == id)
        || document.net_labels.iter().any(|item| item.id == id)
        || document.buses.iter().any(|item| item.id == id)
        || document.bus_taps.iter().any(|item| item.id == id)
        || document.design_notes.iter().any(|item| item.id == id)
        || document
            .documentation_shapes
            .iter()
            .any(|item| item.id == id)
        || document.probes.iter().any(|item| item.id == id)
}

/// Every id [`live_id_in_use`] reports in use, sorted.
pub(crate) fn live_ids(document: &SchematicDocument) -> Vec<u64> {
    let mut ids = document
        .components
        .iter()
        .map(|item| item.id)
        .chain(document.wires.iter().map(|item| item.id))
        .chain(document.junctions.iter().map(|item| item.id))
        .chain(document.net_labels.iter().map(|item| item.id))
        .chain(document.buses.iter().map(|item| item.id))
        .chain(document.bus_taps.iter().map(|item| item.id))
        .chain(document.design_notes.iter().map(|item| item.id))
        .chain(document.documentation_shapes.iter().map(|item| item.id))
        .chain(document.probes.iter().map(|item| item.id))
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids
}

#[cfg(test)]
mod tests {
    use super::super::{
        component_edit::{ComponentPlacement, add_component},
        rotation::Rotation,
    };
    use super::*;
    use rspice_design_model::Point;

    fn place_resistor(document: &mut SchematicDocument, identity: &mut SchematicIdentity) -> u64 {
        add_component(
            document,
            identity,
            ComponentType::Resistor,
            ComponentPlacement {
                position: Point::origin(),
                rotation: Rotation::R0,
                mirror_h: false,
            },
            None,
        )
    }

    /// Place R1 and R2, rename R2 to R10, then place eight more: the
    /// counter used to walk straight into R10 a second time. A renamed
    /// number counts as used, and no placement takes a name already there.
    #[test]
    fn a_new_part_never_takes_a_name_already_in_use() {
        let mut document = SchematicDocument::default();
        let mut identity = SchematicIdentity::default();
        place_resistor(&mut document, &mut identity);
        let second = place_resistor(&mut document, &mut identity);
        document
            .components
            .iter_mut()
            .find(|component| component.id == second)
            .expect("R2")
            .name = "R10".to_owned();

        for _ in 0..8 {
            place_resistor(&mut document, &mut identity);
        }

        let mut names: Vec<String> = document
            .components
            .iter()
            .map(|component| component.name.to_ascii_uppercase())
            .collect();
        names.sort();
        let count = names.len();
        names.dedup();
        assert_eq!(names.len(), count, "a name was handed out twice: {names:?}");
        assert_eq!(document.components.last().expect("placed").name, "R18");
    }

    /// A name written by hand in another case is still the same SPICE name.
    #[test]
    fn a_name_in_use_in_another_case_is_skipped() {
        let mut document = SchematicDocument::default();
        let mut identity = SchematicIdentity::default();
        let mut taken =
            super::super::component::Component::new(50, ComponentType::Resistor, Point::origin());
        taken.name = "r1".to_owned();
        document.components.push(taken);

        let id = place_resistor(&mut document, &mut identity);

        let placed = document
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("placed");
        assert_eq!(placed.name, "R2");
    }
}
