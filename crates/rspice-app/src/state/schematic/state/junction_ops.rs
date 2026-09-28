//! Editor notifications around junction document maintenance.

use super::*;

impl SchematicState {
    /// Automatically place junctions at all detected intersection points
    ///
    /// This is the main entry point for automatic junction management.
    /// Call this after wire operations to maintain junction consistency.
    pub fn auto_place_junctions(&mut self) {
        if self.design.auto_place_junctions() {
            self.is_dirty = true;
        }
    }

    /// Remove junctions that no longer connect at least two distinct wires.
    pub fn remove_orphan_junctions(&mut self) -> usize {
        let removed = self.design.remove_orphan_junctions();
        if removed > 0 {
            self.is_dirty = true;
        }
        removed
    }

    /// Update junction markers based on current wire topology: orphaned
    /// markers go first, then geometry places the ones it now implies.
    pub fn update_wire_junctions(&mut self) {
        self.remove_orphan_junctions();
        self.auto_place_junctions();
    }
}
