//! Junctions and net labels.
//!
//! The two annotations that change what a drawing means: a junction makes
//! crossing wires connect, and a net label names the net it sits on. Adding
//! or removing either invalidates cached connectivity.

use super::super::junction_edit;
use super::super::*;

impl SchematicState {
    // =========================================================================
    // Junction Management
    // =========================================================================

    /// Add an explicit junction at a position
    pub fn add_junction(&mut self, pos: Point) -> u64 {
        let (id, inserted) =
            junction_edit::add_junction(&mut self.document, &mut self.identity, pos);
        if inserted {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        id
    }

    /// Remove a junction by ID
    pub fn remove_junction(&mut self, id: u64) -> bool {
        let removed = junction_edit::remove_junction(&mut self.document, id);
        if removed {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Find junction at a position
    pub fn junction_at(&self, pos: Point) -> Option<u64> {
        self.document
            .junctions
            .iter()
            .find(|j| j.pos == pos)
            .map(|j| j.id)
    }

    /// Check if a junction exists at a position
    pub fn has_junction(&self, pos: Point) -> bool {
        self.document.junctions.iter().any(|j| j.pos == pos)
    }

    /// Add a net label at the given position
    pub fn add_net_label(&mut self, pos: Point, name: String) -> u64 {
        let id = junction_edit::add_net_label(&mut self.document, &mut self.identity, pos, name);
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_a_label_invalidates_connectivity_state() {
        let mut schematic = SchematicState::default();
        let topology_before = schematic.topology_version();

        let id = schematic.add_net_label(Point::new(2, 3), "sense".to_owned());

        assert!(schematic.is_dirty);
        assert_eq!(
            schematic.document.net_labels,
            vec![NetLabel::new(id, Point::new(2, 3), "sense")]
        );
        assert_ne!(schematic.topology_version(), topology_before);
    }
}
