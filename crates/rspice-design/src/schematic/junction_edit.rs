//! Junction and net-label document edits and geometric maintenance.

use super::{
    document::SchematicDocument,
    identity::SchematicIdentity,
    net_label::{Junction, NetLabel},
    wire::Wire,
};
use rspice_design_model::Point;
use std::collections::HashSet;

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

/// Detect junction points that need visual markers
///
/// A junction needs a marker (dot) when:
/// - 3+ wire segments meet at a point (T-junction or cross)
/// - A wire endpoint meets another wire mid-segment (T-junction)
///
/// This counts SEGMENTS meeting at each point, not wire IDs:
/// - A wire endpoint contributes 1 segment
/// - A wire passing through mid-segment contributes 2 segments
fn detect_junction_points(document: &SchematicDocument) -> Vec<Point> {
    use std::collections::HashMap;

    let mut segment_counts: HashMap<Point, usize> = HashMap::new();

    // Vertex contributions: endpoints are one segment, interior
    // vertices join two.
    for wire in &document.wires {
        for (i, point) in wire.points.iter().enumerate() {
            let is_endpoint = i == 0 || i == wire.points.len() - 1;
            let count = if is_endpoint { 1 } else { 2 };
            *segment_counts.entry(*point).or_insert(0) += count;
        }
    }

    // A wire whose segment interior passes through a counted vertex
    // contributes two more segments there (it runs straight through).
    let index = SegmentIndex::build(&document.wires);
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

/// Automatically place junctions at all detected intersection points
///
/// This is the main entry point for automatic junction management.
/// Call this after wire operations to maintain junction consistency.
pub fn auto_place_junctions(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
) -> bool {
    let mut detected_points = detect_junction_points(document);
    detected_points.sort_by_key(|point| (point.x, point.y));
    let junction_points: HashSet<Point> = detected_points.iter().copied().collect();
    let existing: HashSet<Point> = document.junctions.iter().map(|j| j.pos).collect();
    let index = SegmentIndex::build(&document.wires);
    let mut touching = Vec::new();
    let mut changes = false;

    // Add junctions at detected points that don't have one
    for point in detected_points {
        if !existing.contains(&point) {
            let id = identity.allocate(document);
            document.junctions.push(Junction::new(id, point));
            changes = true;
        }
    }

    // Automatic markers require the detected 3+ segment topology. An
    // existing explicit marker remains valid at a two-wire touch: it is
    // the user's electrical connection intent and must survive routine
    // topology maintenance.
    let len_before = document.junctions.len();
    document.junctions.retain(|junction| {
        junction_points.contains(&junction.pos)
            || index.at_least_two_wires_touch(junction.pos, &mut touching)
    });
    if document.junctions.len() != len_before {
        changes = true;
    }

    changes
}

/// Remove invalid junctions without opening a transaction or invalidating
/// caches. Composite state operations call this before their single dirty
/// and topology update.
pub fn remove_orphan_junctions(document: &mut SchematicDocument) -> usize {
    let initial_count = document.junctions.len();
    if initial_count > 0 {
        let index = SegmentIndex::build(&document.wires);
        let mut touching = Vec::new();
        document
            .junctions
            .retain(|junction| index.at_least_two_wires_touch(junction.pos, &mut touching));
    }
    initial_count - document.junctions.len()
}

/// Add an explicit junction, returning its ID and whether it was inserted.
pub fn add_junction(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    pos: Point,
) -> (u64, bool) {
    // Check if junction already exists at this position
    if let Some(existing) = document.junctions.iter().find(|j| j.pos == pos) {
        return (existing.id, false);
    }

    let id = identity.allocate(document);
    document.junctions.push(Junction::new(id, pos));

    (id, true)
}

/// Remove a junction by ID
pub fn remove_junction(document: &mut SchematicDocument, id: u64) -> bool {
    let len_before = document.junctions.len();
    document.junctions.retain(|j| j.id != id);
    document.junctions.len() < len_before
}

/// Add a net label at the given position
pub fn add_net_label(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    pos: Point,
    name: String,
) -> u64 {
    let id = identity.allocate(document);
    document.net_labels.push(NetLabel::new(id, pos, name));

    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_two_wire_junction_survives_automatic_maintenance() {
        let point = Point::new(10, 10);
        let mut document = SchematicDocument {
            wires: vec![
                Wire::new(1, vec![Point::new(0, 10), point]),
                Wire::new(2, vec![point, Point::new(10, 20)]),
            ],
            ..SchematicDocument::default()
        };
        document.junctions.push(Junction::new(3, point));

        auto_place_junctions(&mut document, &mut SchematicIdentity::with_cursor(1));

        assert_eq!(document.junctions, vec![Junction::new(3, point)]);
    }

    #[test]
    fn orphan_cleanup_counts_distinct_wires_not_segments() {
        let point = Point::new(10, 10);
        let mut document = SchematicDocument {
            wires: vec![Wire::new(
                1,
                vec![Point::new(0, 10), point, Point::new(20, 10)],
            )],
            ..SchematicDocument::default()
        };
        document.junctions.push(Junction::new(2, point));

        assert_eq!(remove_orphan_junctions(&mut document), 1);
        assert!(document.junctions.is_empty());
    }

    #[test]
    fn orphan_cleanup_retains_two_distinct_touching_wires() {
        let point = Point::new(10, 10);
        let mut document = SchematicDocument {
            wires: vec![
                Wire::new(1, vec![Point::new(0, 10), Point::new(20, 10)]),
                Wire::new(2, vec![Point::new(10, 0), Point::new(10, 20)]),
            ],
            ..SchematicDocument::default()
        };
        document.junctions.push(Junction::new(3, point));

        assert_eq!(remove_orphan_junctions(&mut document), 0);
        assert_eq!(document.junctions, vec![Junction::new(3, point)]);
    }
}
