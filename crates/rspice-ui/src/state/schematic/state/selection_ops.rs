#[cfg(test)]
use super::super::{BusDeclaration, BusSlice, BusTapOrientation};
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

        // Select wires whose routed polyline intersects the rectangle.
        for wire in &self.wires {
            let wire_in_rect = polyline_intersects_rect(&wire.points, min_x, min_y, max_x, max_y);
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

        // Select buses whose routed polyline intersects the rectangle, even
        // when both segment endpoints are outside it.
        for bus in &self.buses {
            let in_rect = polyline_intersects_rect(&bus.points, min_x, min_y, max_x, max_y);
            if in_rect && !self.selection.has_bus(bus.id) {
                self.selection.select_bus(bus.id);
                count += 1;
            }
        }

        // A tap is included when either end of its connection is inside.
        for tap in &self.bus_taps {
            let in_rect = segment_intersects_rect(
                tap.bus_point,
                tap.connection_point,
                min_x,
                min_y,
                max_x,
                max_y,
            );
            if in_rect && !self.selection.has_bus_tap(tap.id) {
                self.selection.select_bus_tap(tap.id);
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

        // Select wires whose routed polyline intersects the rectangle.
        for wire in &self.wires {
            let wire_in_rect = polyline_intersects_rect(&wire.points, min_x, min_y, max_x, max_y);
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

        for bus in &self.buses {
            if polyline_intersects_rect(&bus.points, min_x, min_y, max_x, max_y) {
                self.selection.select_bus(bus.id);
            }
        }

        for tap in &self.bus_taps {
            if segment_intersects_rect(
                tap.bus_point,
                tap.connection_point,
                min_x,
                min_y,
                max_x,
                max_y,
            ) {
                self.selection.select_bus_tap(tap.id);
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
                .any(|junction| selection.has_junction(junction.pos))
            || self.buses.iter().any(|bus| selection.has_bus(bus.id))
            || self
                .bus_taps
                .iter()
                .any(|tap| selection.has_bus_tap(tap.id));
        if !has_live_object {
            return false;
        }

        self.with_undo("delete selection", move |schematic| {
            schematic
                .components
                .retain(|component| !selection.has_component(component.id));
            schematic.wires.retain(|wire| !selection.has_wire(wire.id));
            let removed_bus_ids: std::collections::HashSet<u64> = schematic
                .buses
                .iter()
                .filter(|bus| selection.has_bus(bus.id))
                .map(|bus| bus.id)
                .collect();
            schematic
                .buses
                .retain(|bus| !removed_bus_ids.contains(&bus.id));
            schematic.bus_taps.retain(|tap| {
                !removed_bus_ids.contains(&tap.bus_id) && !selection.has_bus_tap(tap.id)
            });
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

    /// Select every complete design object in the schematic.
    pub fn select_all_objects(&mut self) {
        self.selection.clear();
        self.selection.components = self.components.iter().map(|item| item.id).collect();
        self.selection.wires = self.wires.iter().map(|item| item.id).collect();
        self.selection.buses = self.buses.iter().map(|item| item.id).collect();
        self.selection.bus_taps = self.bus_taps.iter().map(|item| item.id).collect();
        for position in self.junctions.iter().map(|item| item.pos) {
            self.selection.select_junction(position);
        }
    }
}

fn polyline_intersects_rect(
    points: &[Point],
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    points
        .windows(2)
        .any(|pair| segment_intersects_rect(pair[0], pair[1], min_x, min_y, max_x, max_y))
}

fn segment_intersects_rect(
    start: Point,
    end: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    let inside =
        |point: Point| point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y;
    if inside(start) || inside(end) {
        return true;
    }
    let corners = [
        Point::new(min_x, min_y),
        Point::new(max_x, min_y),
        Point::new(max_x, max_y),
        Point::new(min_x, max_y),
    ];
    (0..4).any(|index| segments_intersect(start, end, corners[index], corners[(index + 1) % 4]))
}

fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    fn orientation(a: Point, b: Point, c: Point) -> i128 {
        (i128::from(b.x) - i128::from(a.x)) * (i128::from(c.y) - i128::from(a.y))
            - (i128::from(b.y) - i128::from(a.y)) * (i128::from(c.x) - i128::from(a.x))
    }
    fn on_segment(a: Point, b: Point, point: Point) -> bool {
        point.x >= a.x.min(b.x)
            && point.x <= a.x.max(b.x)
            && point.y >= a.y.min(b.y)
            && point.y <= a.y.max(b.y)
    }
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);
    (o1.signum() != o2.signum() && o3.signum() != o4.signum())
        || (o1 == 0 && on_segment(a, b, c))
        || (o2 == 0 && on_segment(a, b, d))
        || (o3 == 0 && on_segment(c, d, a))
        || (o4 == 0 && on_segment(c, d, b))
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

    #[test]
    fn crossing_bus_and_tap_are_box_selected_without_vertices_inside() {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus =
            Bus::segment(11, Point::new(-10, 0), Point::new(10, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            12,
            &bus,
            Point::new(0, 0),
            Point::new(0, 10),
            BusSlice::parse("DATA[2]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic
            .wires
            .push(Wire::segment(13, Point::new(-10, -1), Point::new(10, -1)));
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);
        assert_eq!(schematic.select_in_rect(-1, -1, 1, 1, false), 3);
        assert!(schematic.selection.has_wire(13));
        assert!(schematic.selection.has_bus(11));
        assert!(schematic.selection.has_bus_tap(12));
        assert!(segment_intersects_rect(
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
            -1,
            -1,
            1,
            1,
        ));
    }

    #[test]
    fn deleting_bus_cascades_taps_and_undo_restores_both() {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus = Bus::segment(21, Point::new(0, 0), Point::new(10, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            22,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[1]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);
        schematic.selection.select_only_bus(21);
        assert!(schematic.delete_selection());
        assert!(schematic.buses.is_empty() && schematic.bus_taps.is_empty());
        assert!(schematic.undo());
        assert_eq!(schematic.buses.len(), 1);
        assert_eq!(schematic.bus_taps.len(), 1);
    }
}
