use super::*;

impl SchematicState {
    // =========================================================================
    // Selection Management
    // =========================================================================

    /// Select all components and wires within a rectangular region
    ///
    /// This is used for rubber-band box selection. The user drags to create
    /// a selection rectangle, and all items within the rectangle are selected.
    ///
    /// # Arguments
    /// * `min_x`, `min_y`, `max_x`, `max_y` - The selection rectangle bounds (in grid coordinates)
    /// * `add_to_selection` - If true, add to existing selection; if false, replace selection
    ///
    /// # Returns
    /// The number of items selected
    pub fn select_in_rect(
        &mut self,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
        add_to_selection: bool,
    ) -> usize {
        if !add_to_selection {
            self.selection.clear();
        }

        let mut count = 0;

        // Select components whose center is within the rectangle
        for comp in &self.components {
            if comp.pos.x >= min_x
                && comp.pos.x <= max_x
                && comp.pos.y >= min_y
                && comp.pos.y <= max_y
                && !self.selection.has_component(comp.id)
            {
                self.selection.select_component(comp.id);
                count += 1;
            }
        }

        // Select wires that have at least one point within the rectangle
        for wire in &self.wires {
            let wire_in_rect = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
            if wire_in_rect && !self.selection.has_wire(wire.id) {
                self.selection.select_wire(wire.id);
                count += 1;
            }
        }

        // Select junctions within the rectangle
        for junction in &self.junctions {
            if junction.pos.x >= min_x
                && junction.pos.x <= max_x
                && junction.pos.y >= min_y
                && junction.pos.y <= max_y
                && !self.selection.has_junction(junction.pos)
            {
                self.selection.select_junction(junction.pos);
                count += 1;
            }
        }

        count
    }

    /// Preview selection in rectangle during drag (live highlight feedback)
    ///
    /// This updates the selection to show what would be selected when the drag
    /// is released. It replaces the current selection with items in the rect.
    pub fn preview_selection_in_rect(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        // Clear and rebuild selection based on current rect
        self.selection.clear();

        // Select components whose center is within the rectangle
        for comp in &self.components {
            if comp.pos.x >= min_x
                && comp.pos.x <= max_x
                && comp.pos.y >= min_y
                && comp.pos.y <= max_y
            {
                self.selection.select_component(comp.id);
            }
        }

        // Select wires that have at least one point within the rectangle
        for wire in &self.wires {
            let wire_in_rect = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
            if wire_in_rect {
                self.selection.select_wire(wire.id);
            }
        }

        // Select junctions within the rectangle
        for junction in &self.junctions {
            if junction.pos.x >= min_x
                && junction.pos.x <= max_x
                && junction.pos.y >= min_y
                && junction.pos.y <= max_y
            {
                self.selection.select_junction(junction.pos);
            }
        }
    }

    /// Atomically remove selected components, complete wires, and junctions.
    ///
    /// This state-layer entry point owns the undo transaction so keyboard,
    /// menu, and context-menu deletion all have identical behavior. Wire
    /// segment and vertex selections remain editing handles and are not
    /// interpreted as requests to delete their parent wire.
    pub fn delete_selection(&mut self) -> bool {
        if self.read_only {
            return false;
        }

        let selection = self.selection.clone();
        let has_live_object = self
            .components
            .iter()
            .any(|component| selection.has_component(component.id))
            || self.wires.iter().any(|wire| selection.has_wire(wire.id))
            || self
                .junctions
                .iter()
                .any(|junction| selection.has_junction(junction.pos));
        if !has_live_object {
            return false;
        }

        self.with_undo("delete selection", move |schematic| {
            schematic
                .components
                .retain(|component| !selection.has_component(component.id));
            schematic.wires.retain(|wire| !selection.has_wire(wire.id));
            // A wire deletion may invalidate connection markers that were not
            // explicitly selected. Keep that lifecycle cleanup inside this
            // same undo transaction and topology update.
            schematic.remove_orphan_junctions_untracked();
            schematic
                .junctions
                .retain(|junction| !selection.has_junction(junction.pos));
            schematic.selection.clear();
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn junction_deletion_is_one_undoable_transaction() {
        let point = Point::new(12, 8);
        let mut schematic = SchematicState::default();
        schematic.junctions.push(Junction::new(7, point));
        schematic.init_undo_history();
        schematic.selection.select_only_junction(point);

        assert!(schematic.delete_selection());
        assert!(schematic.junctions.is_empty());
        assert!(schematic.selection.is_empty());
        assert_eq!(schematic.undo_description(), Some("delete selection"));

        assert!(schematic.undo());
        assert_eq!(schematic.junctions, vec![Junction::new(7, point)]);
        assert!(
            !schematic.can_undo(),
            "deletion must create only one undo step"
        );
    }

    #[test]
    fn deleting_a_stale_selection_is_a_no_op() {
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic.selection.select_only_junction(Point::new(9, 9));

        assert!(!schematic.delete_selection());
        assert!(!schematic.can_undo());
    }

    #[test]
    fn wire_deletion_removes_newly_orphaned_junction_in_same_undo_step() {
        let point = Point::new(10, 10);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 10), point]),
            Wire::new(2, vec![point, Point::new(10, 20)]),
        ];
        schematic.junctions = vec![Junction::new(3, point)];
        schematic.init_undo_history();
        schematic.selection.select_only_wire(1);

        assert!(schematic.delete_selection());
        assert_eq!(schematic.wires.len(), 1);
        assert!(schematic.junctions.is_empty());

        assert!(schematic.undo());
        assert_eq!(schematic.wires.len(), 2);
        assert_eq!(schematic.junctions, vec![Junction::new(3, point)]);
        assert!(!schematic.can_undo(), "cleanup belongs to the delete step");
    }
}
