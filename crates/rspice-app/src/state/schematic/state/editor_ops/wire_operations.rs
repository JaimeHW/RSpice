//! Reshaping existing wires.
//!
//! Splitting wires, moving shared vertices, and maintaining conductor topology.

use super::super::*;

impl SchematicState {
    // =========================================================================
    // Wire Operations (Commercial-Grade)
    // =========================================================================

    /// Split a wire into two wires at the given point
    ///
    /// If the point is exactly on the wire (either at a vertex or on a segment),
    /// this will create two new wires: one from the original start to the split point,
    /// and one from the split point to the original end.
    ///
    /// Returns `Some((wire_before_id, wire_after_id))` if successful, `None` otherwise.
    ///
    /// # Arguments
    /// * `wire_id` - The ID of the wire to split
    /// * `at_point` - The point at which to split (must be on the wire)
    pub fn split_wire(&mut self, wire_id: u64, at_point: Point) -> Option<(u64, u64)> {
        let split = self.design.split_wire(wire_id, at_point);
        if split.is_some() {
            self.session.is_dirty = true;
        }
        split
    }

    /// Split the wires that pins of `components` land inside, so each pin
    /// owns a vertex there and the wire follows the part. A pin on an
    /// unmarked crossing of two wires is left for the user to resolve, and a
    /// part with two pins inside one run of a wire stands in series on it.
    pub fn split_wires_under_pins(
        &mut self,
        components: &std::collections::HashSet<u64>,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        _scope: &ActiveSheetTopology,
    ) -> bool {
        let parts = self
            .design
            .document()
            .components
            .iter()
            .filter(|component| components.contains(&component.id))
            .map(&mut terminal_points_for)
            .collect::<Vec<_>>();
        let changed = self.design.split_wires_under_pins(&parts);
        if changed {
            self.session.is_dirty = true;
        }
        changed
    }

    /// Connect the pins of `components` where a move or turn left them: a
    /// pin taken off a pin it touched gets a square wire back to it, and a
    /// pin that landed inside a wire splits it. Where the parts stood comes
    /// from the pending operation, which the caller opens first.
    pub fn connect_moved_pins(
        &mut self,
        components: &std::collections::HashSet<u64>,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        scope: &ActiveSheetTopology,
    ) -> bool {
        if self.session.read_only {
            return false;
        }
        let bridged = self
            .design
            .bridge_broken_abutments(components, &mut terminal_points_for);
        if bridged {
            self.session.is_dirty = true;
        }
        let split = self.split_wires_under_pins(components, terminal_points_for, scope);
        bridged || split
    }

    /// Optimize all wires by removing collinear intermediate points
    pub fn optimize_all_wires(&mut self, _scope: &ActiveSheetTopology) {
        if self.design.optimize_all_wires() {
            self.session.is_dirty = true;
        }
    }

    /// Remove degenerate segments from all wires
    ///
    /// This is a cleanup operation that removes:
    /// 1. Zero-length segments (consecutive identical points)
    /// 2. Wires that become invalid after cleanup (< 2 points)
    ///
    /// This is called automatically after wire editing operations to ensure
    /// the schematic maintains valid topology. Matches Cadence Virtuoso behavior.
    ///
    /// # Returns
    /// A tuple of (wires_modified, wires_removed) counts
    pub fn remove_degenerate_segments(&mut self, _scope: &ActiveSheetTopology) -> (usize, usize) {
        let (wires_modified, wires_removed) = self.design.remove_degenerate_segments();
        if wires_modified > 0 || wires_removed > 0 {
            self.session.is_dirty = true;
        }
        (wires_modified, wires_removed)
    }

    /// Clean up wire topology after editing operations
    ///
    /// This comprehensive cleanup method should be called after bulk editing:
    /// 1. Removes degenerate (zero-length) segments
    /// 2. Optimizes wire paths (removes collinear points)
    /// 3. Updates junction markers
    ///
    /// This matches commercial EDA tool behavior for maintaining clean topology.
    pub fn cleanup_wire_topology(&mut self, scope: &ActiveSheetTopology) {
        self.cleanup_wire_topology_with_junction_policy(true, scope);
    }

    /// Clean topology while respecting the resolved document/user junction
    /// policy. Manual-junction projects still receive safe geometric cleanup,
    /// but existing explicit junction ownership is never synthesized away.
    pub fn cleanup_wire_topology_with_junction_policy(
        &mut self,
        automatic_junctions: bool,
        scope: &ActiveSheetTopology,
    ) {
        self.remove_degenerate_segments(scope);
        self.optimize_all_wires(scope);
        self.join_collinear_wires(scope);
        if automatic_junctions {
            self.update_wire_junctions(scope);
        } else {
            self.remove_orphan_junctions(scope);
        }
    }

    /// Move ALL wire vertices at a given position to a new position
    ///
    /// This is the professional EDA behavior for junction/corner dragging:
    /// when you drag a point where multiple wires meet, all of them move together.
    ///
    /// For T-junctions where a wire passes through without a vertex, we first
    /// split that wire at the junction point so it can move with the others.
    ///
    /// # Arguments
    /// * `old_pos` - The current position of the vertices to move
    /// * `new_pos` - The new position
    ///
    /// Returns true if any vertices were moved
    pub fn move_all_vertices_at(
        &mut self,
        old_pos: Point,
        new_pos: Point,
        scope: &ActiveSheetTopology,
    ) -> bool {
        if old_pos == new_pos {
            return false;
        }

        // First, check if this is a junction point where wires might pass through
        // without having a vertex. If so, split those wires first.
        let is_junction = self
            .design
            .document()
            .junctions
            .iter()
            .any(|j| j.pos == old_pos);
        if is_junction {
            // Split any wires that pass through this junction point but don't have a vertex there
            self.split_wires_at_t_junction(old_pos, scope);
        }

        let moved = self.design.move_vertices_at(old_pos, new_pos);
        if moved {
            self.session.is_dirty = true;
        }
        moved
    }
}
