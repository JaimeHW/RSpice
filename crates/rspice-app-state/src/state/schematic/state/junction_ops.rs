//! Editor notifications around junction document maintenance.

use super::*;

impl SchematicState {
    /// Remove junctions that no longer connect at least two distinct wires.
    pub fn remove_orphan_junctions(&mut self, _scope: &ActiveSheetTopology) -> usize {
        let removed = self.design.remove_orphan_junctions();
        if removed > 0 {
            self.session.is_dirty = true;
        }
        removed
    }

    /// Update junction markers based on current wire topology: orphaned
    /// markers go first, then geometry places the ones it now implies.
    pub fn update_wire_junctions(&mut self, _scope: &ActiveSheetTopology) {
        let (removed, placed) = self.design.update_wire_junctions();
        if removed > 0 || placed {
            self.session.is_dirty = true;
        }
    }
}
