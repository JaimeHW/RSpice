//! Wire creation and hit-testing.
//!
//! Adding wires, and locating what is under a point: the wire itself, one of
//! its vertices, or an endpoint. The distinction drives the cursor and what
//! a drag does.

use super::super::wire_edit;
use super::super::*;

impl SchematicState {
    // =========================================================================
    // Wire Management
    // =========================================================================

    /// Add a wire
    pub fn add_wire(&mut self, points: Vec<Point>) -> Option<u64> {
        let id = wire_edit::add_wire(&mut self.document, &mut self.identity, points);
        if id.is_some() {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        id
    }

    /// Find wire at grid position
    pub fn wire_at(&self, pos: Point) -> Option<u64> {
        for wire in &self.document.wires {
            if wire.contains_point(pos) {
                return Some(wire.id);
            }
        }
        None
    }

    /// Find wire vertex at a grid position for dragging
    ///
    /// Returns (wire_id, vertex_index) if there's a wire vertex at this position.
    /// This is used for wire corner dragging - a professional EDA feature.
    /// O(1) via the canvas cache when it's current; linear scan otherwise.
    pub fn wire_vertex_at(&self, pos: Point) -> Option<(u64, usize)> {
        if let Some(cache) = self.canvas_cache() {
            return cache.wire_vertices.get(&pos).copied();
        }
        for wire in &self.document.wires {
            for (idx, point) in wire.points.iter().enumerate() {
                if *point == pos {
                    return Some((wire.id, idx));
                }
            }
        }
        None
    }

    /// Check if a position is a draggable wire point
    ///
    /// Returns true if there's either:
    /// - A wire vertex at this position
    /// - A junction marker at this position
    ///
    /// This runs on every pointer-move frame, so it must not scan the
    /// schematic; the canvas cache answers in O(1) when current.
    pub fn is_draggable_wire_point(&self, pos: Point) -> bool {
        if let Some(cache) = self.canvas_cache() {
            return cache.wire_vertices.contains_key(&pos) || cache.junctions.contains(&pos);
        }
        self.wire_vertex_at(pos).is_some() || self.document.junctions.iter().any(|j| j.pos == pos)
    }

    /// Start drawing a wire at position
    pub fn start_wire(&mut self, pos: Point) {
        if self.read_only {
            return;
        }

        log::info!("[Wire] start_wire at {:?}", pos);
        self.wire_drawing.clear();
        self.wire_drawing.points.push(pos);
        self.wire_drawing.active = true;
    }

    /// Update the wire preview position (called on mouse move)
    pub fn update_wire_preview(&mut self, pos: Point) {
        if self.wire_drawing.active {
            self.wire_drawing.preview_pos = Some(pos);
        }
    }

    /// Toggle wire routing mode (horizontal-first vs vertical-first)
    pub fn toggle_wire_routing(&mut self) {
        self.wire_drawing.routing_mode = self.wire_drawing.routing_mode.toggle();
    }

    /// Add a point to the current wire using orthogonal routing
    pub fn extend_wire(&mut self, pos: Point) {
        if self.read_only || !self.wire_drawing.active {
            return;
        }

        if let Some(last) = self.wire_drawing.points.last().copied() {
            if last == pos {
                return; // Same point, skip
            }

            // Add corner point for orthogonal routing if needed
            if let Some(corner) = self.wire_drawing.get_route_corner(pos)
                && corner != last
                && corner != pos
            {
                self.wire_drawing.points.push(corner);
            }

            self.wire_drawing.points.push(pos);
        }
    }

    /// Finish drawing the current wire
    ///
    /// Implements professional EDA behavior:
    /// - When a wire endpoint lands on another wire mid-segment, the other wire
    ///   is automatically split at that point (creating a proper vertex)
    /// - This ensures correct rubber-banding: all wires at a T-junction share
    ///   a common endpoint vertex, so moving any wire keeps the junction intact
    pub fn finish_wire(&mut self) -> Option<u64> {
        if !self.wire_drawing.active {
            return None;
        }

        let points = std::mem::take(&mut self.wire_drawing.points);
        self.wire_drawing.clear();

        if self.read_only {
            return None;
        }

        let simplified = wire_edit::simplify_wire_path(points);

        if simplified.len() < 2 {
            return None;
        }

        let mut last_wire_id = None;
        self.with_undo("draw wire", |schematic| {
            // Split the path into individual 2-point wire segments. The
            // complete route and every topology repair it causes are one
            // atomic user operation.
            let mut endpoints_to_check = Vec::new();

            for i in 0..simplified.len() - 1 {
                let segment = vec![simplified[i], simplified[i + 1]];
                endpoints_to_check.push(simplified[i]);
                if i == simplified.len() - 2 {
                    endpoints_to_check.push(simplified[i + 1]);
                }
                if let Some(wire_id) = schematic.add_wire(segment) {
                    last_wire_id = Some(wire_id);
                }
            }

            // Professional EDA behavior: split existing wires at T-junction
            // points. Keeping this inside the route transaction guarantees
            // that one Undo restores both the route and the prior topology.
            for point in endpoints_to_check {
                schematic.split_wires_at_t_junction(point);
            }

            // Add junction markers where 3+ wire endpoints meet.
            schematic.update_wire_junctions();
        });

        last_wire_id
    }

    /// Split all wires that pass through a point mid-segment (T-junction creation)
    ///
    /// When a wire endpoint lands on another wire's mid-segment, we split the
    /// through wire at that point. This implements professional EDA behavior
    /// where T-junctions are formed by splitting wires, not just by visual overlap.
    ///
    /// This ensures correct rubber-banding: since all wires at the junction
    /// share the same endpoint vertex, moving any attached wire keeps the
    /// junction topology intact.
    pub fn split_wires_at_t_junction(&mut self, point: Point) {
        for wire_id in wire_edit::wires_to_split_at(&self.document, point) {
            let _ = self.split_wire(wire_id, point);
        }
    }

    /// Cancel wire drawing
    pub fn cancel_wire(&mut self) {
        self.wire_drawing.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completed_interactive_route_is_one_undo_unit() {
        let mut schematic = SchematicState::default();
        let retained_wire = schematic
            .add_wire(vec![Point::new(-20, -20), Point::new(-10, -20)])
            .expect("baseline wire");
        schematic.reset_undo_history();
        schematic.is_dirty = false;

        schematic.start_wire(Point::new(0, 0));
        schematic.extend_wire(Point::new(20, 0));
        schematic.extend_wire(Point::new(20, 20));

        assert!(!schematic.has_pending_operation());
        assert!(schematic.finish_wire().is_some());
        assert_eq!(schematic.undo_history.committed.undo_count(), 1);
        assert_eq!(schematic.undo_description(), Some("draw wire"));
        assert_eq!(schematic.document.wires.len(), 3);
        assert!(!schematic.wire_drawing.active);
        assert!(schematic.is_dirty);

        assert!(schematic.undo());
        assert_eq!(schematic.document.wires.len(), 1);
        assert_eq!(schematic.document.wires[0].id, retained_wire);
        assert!(!schematic.can_undo());
        assert!(schematic.can_redo());

        assert!(schematic.redo());
        assert_eq!(schematic.document.wires.len(), 3);
        assert_eq!(schematic.undo_history.committed.undo_count(), 1);
    }

    #[test]
    fn cancelled_and_non_committing_routes_do_not_create_history() {
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic.is_dirty = true;

        schematic.start_wire(Point::new(0, 0));
        schematic.extend_wire(Point::new(10, 0));
        schematic.cancel_wire();

        assert!(!schematic.wire_drawing.active);
        assert!(!schematic.has_pending_operation());
        assert!(!schematic.can_undo());
        assert!(schematic.is_dirty);
        assert!(schematic.document.wires.is_empty());

        schematic.start_wire(Point::new(5, 5));
        assert_eq!(schematic.finish_wire(), None);
        assert!(!schematic.has_pending_operation());
        assert!(!schematic.can_undo());
        assert!(schematic.is_dirty);
        assert!(schematic.document.wires.is_empty());
    }

    #[test]
    fn read_only_wire_gestures_cannot_start_or_commit() {
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic.read_only = true;

        schematic.start_wire(Point::new(0, 0));
        schematic.extend_wire(Point::new(10, 0));

        assert!(!schematic.wire_drawing.active);
        assert_eq!(schematic.finish_wire(), None);
        assert!(!schematic.can_undo());
        assert!(schematic.document.wires.is_empty());
        assert!(!schematic.is_dirty);
    }
}
