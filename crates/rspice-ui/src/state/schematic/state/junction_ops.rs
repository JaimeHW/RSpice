use std::collections::HashSet;

use super::*;

/// Row/column interval index over orthogonal wire segments. Built once per
/// topology pass; point queries scan only the point's row and column
/// buckets instead of every wire — junction maintenance drops from
/// O(wires² · points) to O(segments + vertices · bucket).
///
/// Diagonal segments are deliberately absent: `Wire::contains_point` only
/// recognizes orthogonal segments, and this index mirrors its semantics.
struct SegmentIndex {
    /// Horizontal segments: y → (x0, x1, wire id) with x0 ≤ x1.
    rows: std::collections::HashMap<i32, Vec<(i32, i32, u64)>>,
    /// Vertical segments: x → (y0, y1, wire id) with y0 ≤ y1.
    cols: std::collections::HashMap<i32, Vec<(i32, i32, u64)>>,
    /// (wire id, vertex) pairs — distinguishes interiors from vertices.
    vertices: HashSet<(u64, Point)>,
    /// Wire ids at each vertex (also covers degenerate single-point wires).
    vertex_wires: std::collections::HashMap<Point, Vec<u64>>,
}

impl SegmentIndex {
    fn build(wires: &[Wire]) -> Self {
        let mut index = Self {
            rows: std::collections::HashMap::new(),
            cols: std::collections::HashMap::new(),
            vertices: HashSet::new(),
            vertex_wires: std::collections::HashMap::new(),
        };
        for wire in wires {
            for point in &wire.points {
                index.vertices.insert((wire.id, *point));
                index.vertex_wires.entry(*point).or_default().push(wire.id);
            }
            for seg in wire.points.windows(2) {
                let (a, b) = (seg[0], seg[1]);
                if a.y == b.y {
                    let (x0, x1) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
                    index.rows.entry(a.y).or_default().push((x0, x1, wire.id));
                } else if a.x == b.x {
                    let (y0, y1) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
                    index.cols.entry(a.x).or_default().push((y0, y1, wire.id));
                }
            }
        }
        index
    }

    /// Wire ids with a segment through `v` (inclusive), deduplicated into
    /// the reused `out` buffer.
    fn wires_through(&self, v: Point, out: &mut Vec<u64>) {
        out.clear();
        if let Some(row) = self.rows.get(&v.y) {
            out.extend(
                row.iter()
                    .filter(|&&(x0, x1, _)| x0 <= v.x && v.x <= x1)
                    .map(|&(_, _, id)| id),
            );
        }
        if let Some(col) = self.cols.get(&v.x) {
            out.extend(
                col.iter()
                    .filter(|&&(y0, y1, _)| y0 <= v.y && v.y <= y1)
                    .map(|&(_, _, id)| id),
            );
        }
        out.sort_unstable();
        out.dedup();
    }

    /// Distinct wire ids that touch `v`, whether by segment or by vertex.
    fn wires_touching(&self, v: Point, out: &mut Vec<u64>) {
        self.wires_through(v, out);
        if let Some(vertex_wires) = self.vertex_wires.get(&v) {
            out.extend(vertex_wires);
            out.sort_unstable();
            out.dedup();
        }
    }

    fn at_least_two_wires_touch(&self, v: Point, out: &mut Vec<u64>) -> bool {
        self.wires_touching(v, out);
        out.len() >= 2
    }
}

impl SchematicState {
    // =========================================================================
    // Automatic Junction Detection System (Commercial-Grade)
    // =========================================================================

    /// Find all intersection points between wires
    ///
    /// This detects where wires cross or connect, which is essential for
    /// automatic junction placement. Like Cadence, we check both:
    /// 1. Wire-to-wire segment intersections
    /// 2. Wire endpoints touching other wires
    pub fn find_wire_intersections(&self) -> Vec<(Point, Vec<u64>)> {
        use std::collections::HashMap;

        let mut intersection_map: HashMap<Point, Vec<u64>> = HashMap::new();

        // Phase 1: Find all wire endpoint connections
        for wire in &self.wires {
            if let Some(start) = wire.start() {
                intersection_map.entry(start).or_default().push(wire.id);
            }
            if let Some(end) = wire.end()
                && wire.points.len() > 1
            {
                intersection_map.entry(end).or_default().push(wire.id);
            }
        }

        // Phase 2: Find wire-to-wire segment intersections
        let wires: Vec<_> = self.wires.iter().collect();
        for i in 0..wires.len() {
            for j in (i + 1)..wires.len() {
                let wire_a = wires[i];
                let wire_b = wires[j];

                // Get all intersection points between these two wires
                let intersections = wire_a.intersections_with_wire(wire_b);
                for point in intersections {
                    let entry = intersection_map.entry(point).or_default();
                    if !entry.contains(&wire_a.id) {
                        entry.push(wire_a.id);
                    }
                    if !entry.contains(&wire_b.id) {
                        entry.push(wire_b.id);
                    }
                }
            }
        }

        // Phase 3: Check if any wire passes through another wire's vertex
        for wire in &self.wires {
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }
                // Check if other_wire passes through any vertex of wire
                for vertex in &wire.points {
                    if other_wire.contains_point(*vertex) {
                        let entry = intersection_map.entry(*vertex).or_default();
                        if !entry.contains(&wire.id) {
                            entry.push(wire.id);
                        }
                        if !entry.contains(&other_wire.id) {
                            entry.push(other_wire.id);
                        }
                    }
                }
            }
        }

        // Convert to sorted vector - only include points with 2+ wires (actual connections)
        let mut result: Vec<_> = intersection_map
            .into_iter()
            .filter(|(_, wire_ids)| wire_ids.len() >= 2)
            .collect();
        result.sort_by(|a, b| a.0.x.cmp(&b.0.x).then_with(|| a.0.y.cmp(&b.0.y)));
        result
    }

    /// Detect junction points that need visual markers
    ///
    /// A junction needs a marker (dot) when:
    /// - 3+ wire segments meet at a point (T-junction or cross)
    /// - A wire endpoint meets another wire mid-segment (T-junction)
    ///
    /// This counts SEGMENTS meeting at each point, not wire IDs:
    /// - A wire endpoint contributes 1 segment
    /// - A wire passing through mid-segment contributes 2 segments
    pub fn detect_junction_points(&self) -> Vec<Point> {
        use std::collections::HashMap;

        let mut segment_counts: HashMap<Point, usize> = HashMap::new();

        // Vertex contributions: endpoints are one segment, interior
        // vertices join two.
        for wire in &self.wires {
            for (i, point) in wire.points.iter().enumerate() {
                let is_endpoint = i == 0 || i == wire.points.len() - 1;
                let count = if is_endpoint { 1 } else { 2 };
                *segment_counts.entry(*point).or_insert(0) += count;
            }
        }

        // A wire whose segment interior passes through a counted vertex
        // contributes two more segments there (it runs straight through).
        let index = SegmentIndex::build(&self.wires);
        let mut through: Vec<u64> = Vec::new();
        for (point, count) in segment_counts.iter_mut() {
            index.wires_through(*point, &mut through);
            for &wire_id in &*through {
                if !index.vertices.contains(&(wire_id, *point)) {
                    *count += 2;
                }
            }
        }

        // Return points where 3+ segments meet (T-junction or more)
        segment_counts
            .into_iter()
            .filter(|(_, count)| *count >= 3)
            .map(|(point, _)| point)
            .collect()
    }

    /// Classify the type of junction at a given point
    pub fn classify_junction_type(&self, pos: Point) -> super::super::wire::JunctionType {
        let wire_count = self.wires.iter().filter(|w| w.contains_point(pos)).count();
        super::super::wire::JunctionType::from_wire_count(wire_count)
    }

    /// Automatically place junctions at all detected intersection points
    ///
    /// This is the main entry point for automatic junction management.
    /// Call this after wire operations to maintain junction consistency.
    pub fn auto_place_junctions(&mut self) {
        let mut detected_points = self.detect_junction_points();
        detected_points.sort_by_key(|point| (point.x, point.y));
        let junction_points: HashSet<Point> = detected_points.iter().copied().collect();
        let existing: HashSet<Point> = self.junctions.iter().map(|j| j.pos).collect();
        let index = SegmentIndex::build(&self.wires);
        let mut touching = Vec::new();
        let mut changes = false;

        // Add junctions at detected points that don't have one
        for point in detected_points {
            if !existing.contains(&point) {
                let id = self.next_id();
                self.junctions.push(Junction::new(id, point));
                changes = true;
            }
        }

        // Automatic markers require the detected 3+ segment topology. An
        // existing explicit marker remains valid at a two-wire touch: it is
        // the user's electrical connection intent and must survive routine
        // topology maintenance.
        let len_before = self.junctions.len();
        self.junctions.retain(|junction| {
            junction_points.contains(&junction.pos)
                || index.at_least_two_wires_touch(junction.pos, &mut touching)
        });
        if self.junctions.len() != len_before {
            changes = true;
        }

        if changes {
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
        let initial_count = self.junctions.len();
        if initial_count > 0 {
            let index = SegmentIndex::build(&self.wires);
            let mut touching = Vec::new();
            self.junctions
                .retain(|junction| index.at_least_two_wires_touch(junction.pos, &mut touching));
        }
        initial_count - self.junctions.len()
    }

    /// Update junction markers based on current wire topology
    ///
    /// This is a more comprehensive update that:
    /// 1. Removes orphan junctions
    /// 2. Places new junctions where needed
    /// 3. Updates junction types
    pub fn update_wire_junctions(&mut self) {
        self.remove_orphan_junctions();
        self.auto_place_junctions();
    }

    /// Find all points where wires could potentially be split
    /// (where they cross other wires without connecting)
    pub fn find_potential_splits(&self) -> Vec<(u64, Point)> {
        let mut splits = Vec::new();

        for wire in &self.wires {
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }

                let intersections = wire.intersections_with_wire(other_wire);
                for point in intersections {
                    // Check if this intersection point is already a vertex on wire
                    let is_vertex = wire.points.contains(&point);
                    if !is_vertex {
                        splits.push((wire.id, point));
                    }
                }
            }
        }

        // Deduplicate
        splits.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.x.cmp(&b.1.x)));
        splits.dedup();
        splits
    }

    /// Create T-junctions by splitting wires at intersection points
    ///
    /// When a wire endpoint touches another wire mid-segment,
    /// this will split the second wire and create proper junction.
    pub fn create_t_junctions_from_endpoints(&mut self) {
        // Find all points where one wire ends on another wire's segment
        // interior — via the interval index, not a wire × wire scan.
        let index = SegmentIndex::build(&self.wires);
        let mut splits_needed: Vec<(u64, Point)> = Vec::new();
        let mut through: Vec<u64> = Vec::new();

        for wire in &self.wires {
            let endpoints = [wire.start(), wire.end()];
            for endpoint in endpoints.into_iter().flatten() {
                index.wires_through(endpoint, &mut through);
                for &other_id in &*through {
                    if other_id != wire.id && !index.vertices.contains(&(other_id, endpoint)) {
                        splits_needed.push((other_id, endpoint));
                    }
                }
            }
        }

        // Perform splits and add junctions
        let has_splits = !splits_needed.is_empty();
        for (wire_id, point) in splits_needed {
            // Insert vertex at the intersection point
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id)
                && let Some((segment_idx, _segment)) = wire.segment_containing_point(point)
            {
                wire.insert_vertex(segment_idx + 1, point);
            }

            // Ensure junction exists at this point
            let has_junction = self.junctions.iter().any(|j| j.pos == point);
            if !has_junction {
                self.add_junction(point);
            }
        }

        if has_splits {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Count how many wire segments connect at a given point
    pub fn count_connections_at(&self, pos: Point) -> usize {
        let mut count = 0;
        for wire in &self.wires {
            // Count endpoints
            if wire.start() == Some(pos) {
                count += 1;
            }
            if wire.end() == Some(pos) && wire.points.len() > 1 {
                count += 1;
            }

            // Count mid-wire vertices (each vertex connects 2 segments)
            for (i, point) in wire.points.iter().enumerate() {
                if *point == pos && i > 0 && i < wire.points.len() - 1 {
                    count += 2; // This vertex connects two segments
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_two_wire_junction_survives_automatic_maintenance() {
        let point = Point::new(10, 10);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 10), point]),
            Wire::new(2, vec![point, Point::new(10, 20)]),
        ];
        schematic.junctions.push(Junction::new(3, point));

        schematic.auto_place_junctions();

        assert_eq!(schematic.junctions, vec![Junction::new(3, point)]);
    }

    #[test]
    fn orphan_cleanup_counts_distinct_wires_not_segments() {
        let point = Point::new(10, 10);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![Wire::new(
            1,
            vec![Point::new(0, 10), point, Point::new(20, 10)],
        )];
        schematic.junctions.push(Junction::new(2, point));

        assert_eq!(schematic.remove_orphan_junctions(), 1);
        assert!(schematic.junctions.is_empty());
    }

    #[test]
    fn orphan_cleanup_retains_two_distinct_touching_wires() {
        let point = Point::new(10, 10);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 10), Point::new(20, 10)]),
            Wire::new(2, vec![Point::new(10, 0), Point::new(10, 20)]),
        ];
        schematic.junctions.push(Junction::new(3, point));

        assert_eq!(schematic.remove_orphan_junctions(), 0);
        assert_eq!(schematic.junctions, vec![Junction::new(3, point)]);
    }
}
