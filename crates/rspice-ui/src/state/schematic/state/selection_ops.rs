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

    /// Remove selected components and wires
    pub fn delete_selection(&mut self) {
        self.components
            .retain(|c| !self.selection.has_component(c.id));
        self.wires.retain(|w| !self.selection.has_wire(w.id));
        self.selection.clear();
        self.is_dirty = true;
        self.bump_topology_version();
    }
}
