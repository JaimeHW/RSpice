//! Editor notifications around junction document maintenance.

use super::junction_edit;
use super::*;

impl SchematicState {
    /// Automatically place junctions at all detected intersection points
    ///
    /// This is the main entry point for automatic junction management.
    /// Call this after wire operations to maintain junction consistency.
    pub fn auto_place_junctions(&mut self) {
        if junction_edit::auto_place_junctions(&mut self.document, &mut self.identity) {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Remove junctions that no longer connect at least two distinct wires.
    pub fn remove_orphan_junctions(&mut self) -> usize {
        let removed = self.remove_orphan_junctions_untracked();
        if removed > 0 {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Remove invalid junctions without opening a transaction or invalidating
    /// caches. Composite state operations call this before their single dirty
    /// and topology update.
    pub(super) fn remove_orphan_junctions_untracked(&mut self) -> usize {
        junction_edit::remove_orphan_junctions(&mut self.document)
    }

    /// Update junction markers based on current wire topology: orphaned
    /// markers go first, then geometry places the ones it now implies.
    pub fn update_wire_junctions(&mut self) {
        self.remove_orphan_junctions();
        self.auto_place_junctions();
    }
}
