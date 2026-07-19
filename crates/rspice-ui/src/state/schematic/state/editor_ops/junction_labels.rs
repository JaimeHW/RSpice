use super::super::*;

impl SchematicState {
    // =========================================================================
    // Junction Management
    // =========================================================================

    /// Add an explicit junction at a position
    pub fn add_junction(&mut self, pos: Point) -> u64 {
        // Check if junction already exists at this position
        if let Some(existing) = self.junctions.iter().find(|j| j.pos == pos) {
            return existing.id;
        }

        let id = self.next_id();
        self.junctions.push(Junction::new(id, pos));
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Remove a junction by ID
    pub fn remove_junction(&mut self, id: u64) -> bool {
        let len_before = self.junctions.len();
        self.junctions.retain(|j| j.id != id);
        let removed = self.junctions.len() < len_before;
        if removed {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Find junction at a position
    pub fn junction_at(&self, pos: Point) -> Option<u64> {
        self.junctions.iter().find(|j| j.pos == pos).map(|j| j.id)
    }

    /// Check if a junction exists at a position
    pub fn has_junction(&self, pos: Point) -> bool {
        self.junctions.iter().any(|j| j.pos == pos)
    }

    /// Add a net label at the given position
    pub fn add_net_label(&mut self, pos: Point, name: String) -> u64 {
        let id = self.next_id();
        self.net_labels.push(NetLabel::new(id, pos, name));
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
            schematic.net_labels,
            vec![NetLabel::new(id, Point::new(2, 3), "sense")]
        );
        assert_ne!(schematic.topology_version(), topology_before);
    }
}
