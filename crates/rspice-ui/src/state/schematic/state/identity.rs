use super::*;

impl SchematicState {
    // =========================================================================
    // ID and Name Generation
    // =========================================================================

    /// Generate a unique ID
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
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
        // Find the maximum ID currently in use across every collection that
        // allocates from the shared counter (components, wires, junctions,
        // and net labels).
        let max_component_id = self.components.iter().map(|c| c.id).max().unwrap_or(0);
        let max_wire_id = self.wires.iter().map(|w| w.id).max().unwrap_or(0);
        let max_junction_id = self.junctions.iter().map(|j| j.id).max().unwrap_or(0);
        let max_label_id = self.net_labels.iter().map(|l| l.id).max().unwrap_or(0);
        self.next_id = max_component_id
            .max(max_wire_id)
            .max(max_junction_id)
            .max(max_label_id)
            + 1;

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
    use crate::state::{ComponentType, Point, SchematicState};

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
}
