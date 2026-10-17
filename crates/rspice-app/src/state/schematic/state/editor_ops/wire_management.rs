//! Wire creation and hit-testing.
//!
//! Adding wires, and locating what is under a point: the wire itself, one of
//! its vertices, or an endpoint. The distinction drives the cursor and what
//! a drag does.

use super::super::*;
use rspice_design::schematic::wire_edit;

impl SchematicState {
    // =========================================================================
    // Wire Management
    // =========================================================================

    /// Add a wire
    pub fn add_wire(&mut self, points: Vec<Point>) -> Option<u64> {
        let id = self.design.add_wire(points);
        if id.is_some() {
            self.session.is_dirty = true;
        }
        id
    }

    /// Find wire at grid position
    pub fn wire_at(&self, pos: Point) -> Option<u64> {
        for wire in &self.design.document().wires {
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
        for wire in &self.design.document().wires {
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
        self.wire_vertex_at(pos).is_some()
            || self
                .design
                .document()
                .junctions
                .iter()
                .any(|j| j.pos == pos)
    }

    /// Start drawing a wire at position
    pub fn start_wire(&mut self, pos: Point) {
        if self.session.read_only {
            return;
        }

        log::info!("[Wire] start_wire at {:?}", pos);
        self.session.editor.wire_drawing.start(pos);
    }

    /// Extend the editor draft while the document remains editable.
    pub fn extend_wire(&mut self, pos: Point) {
        if !self.session.read_only {
            self.session.editor.wire_drawing.add_point(pos);
        }
    }

    /// Take back the draft's last placed vertex. The route ends, with nothing
    /// drawn, when its first point goes.
    pub fn retract_wire(&mut self) {
        self.session.editor.wire_drawing.remove_last_point();
    }

    /// Finish drawing the current wire
    ///
    /// Implements professional EDA behavior:
    /// - When a wire endpoint lands on another wire mid-segment, the other wire
    ///   is automatically split at that point (creating a proper vertex)
    /// - This ensures correct rubber-banding: all wires at a T-junction share
    ///   a common endpoint vertex, so moving any wire keeps the junction intact
    ///
    /// `hidden` names the conductors of the cell view's other sheets. They
    /// share this document's coordinates, so a route drawn here would
    /// otherwise split their wires and re-home the halves onto this sheet.
    pub fn finish_wire(&mut self, hidden: impl Fn(u64) -> bool) -> Option<u64> {
        if !self.session.editor.wire_drawing.active {
            return None;
        }

        let points = std::mem::take(&mut self.session.editor.wire_drawing.points);
        self.session.editor.wire_drawing.clear();

        if self.session.read_only {
            return None;
        }

        let simplified = wire_edit::simplify_wire_path(points);

        if simplified.len() < 2 {
            return None;
        }

        let mut last_wire_id = None;
        self.with_undo("draw wire", |schematic| {
            // The undo record opened with every sheet present; the other
            // sheets step out only while the route is laid down.
            schematic.with_active_wire_topology(&hidden, |schematic, scope| {
                // Split the path into individual 2-point wire segments. The
                // complete route and every topology repair it causes are one
                // atomic user operation.
                let mut endpoints_to_check = Vec::new();
                let existing = schematic
                    .design
                    .document()
                    .wires
                    .iter()
                    .map(|wire| wire.id)
                    .collect::<std::collections::HashSet<_>>();

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
                    schematic.split_wires_at_t_junction(point, scope);
                }

                // A route drawn along an existing wire is one conductor
                // there, not two lying on each other.
                let laid = schematic
                    .design
                    .document()
                    .wires
                    .iter()
                    .map(|wire| wire.id)
                    .filter(|id| !existing.contains(id))
                    .collect();
                if schematic.merge_overlapping_wires(&laid, scope) {
                    let exists = |id: &u64| {
                        schematic
                            .design
                            .document()
                            .wires
                            .iter()
                            .any(|wire| wire.id == *id)
                    };
                    last_wire_id = last_wire_id
                        .filter(exists)
                        .or_else(|| laid.iter().copied().filter(exists).max());
                }

                // Dots follow the document's junction policy, as every other
                // cleanup does: under manual junctions the user places them.
                if schematic
                    .design
                    .document()
                    .document_policy
                    .wire_junctions
                    .automatic_junctions()
                {
                    schematic.update_wire_junctions(scope);
                } else {
                    schematic.remove_orphan_junctions(scope);
                }
            });
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
    pub fn split_wires_at_t_junction(&mut self, point: Point, _scope: &ActiveSheetTopology) {
        for wire_id in wire_edit::wires_to_split_at(self.design.document(), point) {
            let _ = self.split_wire(wire_id, point);
        }
    }

    /// Fold each wire in `seeds` together with any wire lying along it on
    /// one line, so no stretch of conductor is drawn twice.
    pub fn merge_overlapping_wires(
        &mut self,
        seeds: &std::collections::HashSet<u64>,
        _scope: &ActiveSheetTopology,
    ) -> bool {
        if self.session.read_only {
            return false;
        }
        let changed = self.design.merge_collinear_wires(Some(seeds), false);
        if changed {
            self.session.is_dirty = true;
        }
        changed
    }

    /// Join Collinear Wires: lay every pair of wires that overlap on one
    /// line, or meet end to end on one with nothing else at the meeting
    /// point, as one wire. A point where a pin, a dot, a label, a tap or a
    /// third wire meets them stays a vertex.
    pub fn join_collinear_wires(&mut self, _scope: &ActiveSheetTopology) -> bool {
        if self.session.read_only {
            return false;
        }
        let changed = self.design.merge_collinear_wires(None, true);
        if changed {
            self.session.is_dirty = true;
        }
        changed
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
        schematic.session.is_dirty = false;

        schematic.start_wire(Point::new(0, 0));
        schematic.extend_wire(Point::new(20, 0));
        schematic.extend_wire(Point::new(20, 20));

        assert!(!schematic.has_pending_operation());
        assert!(schematic.finish_wire(|_| false).is_some());
        assert_eq!(schematic.history().undo_count(), 1);
        assert_eq!(schematic.undo_description(), Some("draw wire"));
        assert_eq!(schematic.design.document().wires.len(), 3);
        assert!(!schematic.session.editor.wire_drawing.active);
        assert!(schematic.session.is_dirty);

        assert!(schematic.undo());
        assert_eq!(schematic.design.document().wires.len(), 1);
        assert_eq!(schematic.design.document().wires[0].id, retained_wire);
        assert!(!schematic.can_undo());
        assert!(schematic.can_redo());

        assert!(schematic.redo());
        assert_eq!(schematic.design.document().wires.len(), 3);
        assert_eq!(schematic.history().undo_count(), 1);
    }

    fn spans(schematic: &SchematicState) -> Vec<Vec<Point>> {
        let mut spans = schematic
            .design
            .document()
            .wires
            .iter()
            .map(|wire| wire.points.clone())
            .collect::<Vec<_>>();
        spans.sort_by_key(|points| (points[0].x, points[0].y));
        spans
    }

    /// A route drawn along part of an existing wire is one conductor there:
    /// no doubled stretch of wire, and no dot where nothing branches.
    #[test]
    fn a_route_drawn_over_a_wire_leaves_one_conductor_and_no_dot() {
        let mut schematic = SchematicState::default();
        schematic.add_wire(vec![Point::new(0, 0), Point::new(40, 0)]);
        schematic.reset_undo_history();

        schematic.start_wire(Point::new(20, 0));
        schematic.extend_wire(Point::new(60, 0));
        assert!(schematic.finish_wire(|_| false).is_some());

        assert_eq!(
            spans(&schematic),
            [
                vec![Point::new(0, 0), Point::new(20, 0)],
                vec![Point::new(20, 0), Point::new(60, 0)],
            ]
        );
        assert!(schematic.design.document().junctions.is_empty());
        assert!(schematic.undo());
        assert_eq!(
            spans(&schematic),
            [vec![Point::new(0, 0), Point::new(40, 0)]]
        );
    }

    /// Where the wire drawn over ended on a pin, the pin keeps its vertex,
    /// so moving the part still carries the wire.
    #[test]
    fn a_route_drawn_over_a_wire_keeps_the_vertex_on_a_pin() {
        let mut schematic = SchematicState::default();
        let id = schematic.add_component(crate::state::ComponentType::Resistor, Point::origin());
        let pin = schematic.design.document().components[0].terminal_positions()[0].1;
        let at = Point::new(40, 0);
        schematic
            .design
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the resistor")
            .pos = Point::new(at.x - pin.x, at.y - pin.y);
        schematic.add_wire(vec![Point::new(0, 0), at]);

        schematic.start_wire(Point::new(20, 0));
        schematic.extend_wire(Point::new(60, 0));
        schematic.finish_wire(|_| false);

        assert_eq!(
            spans(&schematic),
            [
                vec![Point::new(0, 0), Point::new(20, 0)],
                vec![Point::new(20, 0), at],
                vec![at, Point::new(60, 0)],
            ]
        );
    }

    /// Under manual junctions the user places every dot: a drawn T still
    /// splits the wire it lands on, but gets no dot of the tool's making.
    /// Under automatic junctions it does.
    #[test]
    fn a_drawn_t_gets_a_dot_only_under_automatic_junctions() {
        for (policy, dots) in [
            (crate::state::WireJunctionPolicy::OrthogonalManual, 0),
            (crate::state::WireJunctionPolicy::OrthogonalAutomatic, 1),
            (crate::state::WireJunctionPolicy::AnyAngle, 1),
        ] {
            let mut schematic = SchematicState::default();
            schematic
                .design
                .document_mut_for_test()
                .document_policy
                .wire_junctions = policy;
            schematic.add_wire(vec![Point::new(0, 0), Point::new(40, 0)]);

            schematic.start_wire(Point::new(20, 40));
            schematic.extend_wire(Point::new(20, 0));
            schematic.finish_wire(|_| false);

            assert_eq!(schematic.design.document().wires.len(), 3, "{policy:?}");
            assert_eq!(
                schematic.design.document().junctions.len(),
                dots,
                "{policy:?}"
            );
        }
    }

    #[test]
    fn cancelled_and_non_committing_routes_do_not_create_history() {
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic.session.is_dirty = true;

        schematic.start_wire(Point::new(0, 0));
        schematic.extend_wire(Point::new(10, 0));
        schematic.session.editor.wire_drawing.clear();

        assert!(!schematic.session.editor.wire_drawing.active);
        assert!(!schematic.has_pending_operation());
        assert!(!schematic.can_undo());
        assert!(schematic.session.is_dirty);
        assert!(schematic.design.document().wires.is_empty());

        schematic.start_wire(Point::new(5, 5));
        assert_eq!(schematic.finish_wire(|_| false), None);
        assert!(!schematic.has_pending_operation());
        assert!(!schematic.can_undo());
        assert!(schematic.session.is_dirty);
        assert!(schematic.design.document().wires.is_empty());
    }

    #[test]
    fn read_only_wire_gestures_cannot_start_or_commit() {
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic.session.read_only = true;

        schematic.start_wire(Point::new(0, 0));
        schematic.extend_wire(Point::new(10, 0));

        assert!(!schematic.session.editor.wire_drawing.active);
        assert_eq!(schematic.finish_wire(|_| false), None);
        assert!(!schematic.can_undo());
        assert!(schematic.design.document().wires.is_empty());
        assert!(!schematic.session.is_dirty);
    }

    /// Cleanup lays two wires that meet end to end on one line, with nothing
    /// else at the meeting point, as one wire. Where a third wire meets them
    /// they stay apart, so the T keeps its vertex.
    #[test]
    fn cleanup_joins_wires_meeting_end_to_end_on_one_line() {
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().wires = vec![
            Wire::segment(1, Point::new(0, 0), Point::new(40, 0)),
            Wire::segment(2, Point::new(40, 0), Point::new(100, 0)),
            Wire::segment(3, Point::new(100, 0), Point::new(160, 0)),
            Wire::segment(4, Point::new(100, 0), Point::new(100, 60)),
        ];

        schematic.with_single_sheet_topology(|schematic, scope| {
            schematic.cleanup_wire_topology_with_junction_policy(true, scope);
        });

        assert_eq!(
            spans(&schematic),
            [
                vec![Point::new(0, 0), Point::new(100, 0)],
                vec![Point::new(100, 0), Point::new(160, 0)],
                vec![Point::new(100, 0), Point::new(100, 60)],
            ]
        );
    }
}
