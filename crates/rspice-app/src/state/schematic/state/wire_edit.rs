//! Wire document edits and path cleanup, independent of routing gestures.

use super::{Point, SchematicDocument, SchematicIdentity, Wire, WireSegment};

/// Add a wire
pub fn add_wire(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    points: Vec<Point>,
) -> Option<u64> {
    if points.len() < 2 {
        return None;
    }
    let id = identity.allocate(document);
    document.wires.push(Wire::new(id, points));

    Some(id)
}

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
pub fn split_wire(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    wire_id: u64,
    at_point: Point,
) -> Option<(u64, u64)> {
    // Find the wire
    let wire_idx = document.wires.iter().position(|w| w.id == wire_id)?;
    let wire = &document.wires[wire_idx];

    // Validate that the point is on the wire
    if !wire.contains_point(at_point) {
        return None;
    }

    // Don't split at endpoints - nothing to split
    if wire.start() == Some(at_point) || wire.end() == Some(at_point) {
        return None;
    }

    // Find where to split
    let points = wire.points.clone();

    // Check if split point is at an existing vertex
    let vertex_idx = points.iter().position(|p| *p == at_point);

    let (before_points, after_points) = if let Some(v_idx) = vertex_idx {
        // Split at vertex - both wires share this point
        let before: Vec<Point> = points[..=v_idx].to_vec();
        let after: Vec<Point> = points[v_idx..].to_vec();
        (before, after)
    } else {
        // Point is on a segment, need to find which one and insert it
        let mut before_points = Vec::new();
        let mut after_points = Vec::new();
        let mut found_segment = false;

        for i in 0..points.len() - 1 {
            let seg = WireSegment::new(points[i], points[i + 1]);
            if !found_segment {
                before_points.push(points[i]);
                if seg.contains_point(at_point) && points[i] != at_point {
                    before_points.push(at_point);
                    after_points.push(at_point);
                    found_segment = true;
                }
            }
            if found_segment {
                after_points.push(points[i + 1]);
            }
        }

        if !found_segment {
            return None;
        }

        (before_points, after_points)
    };

    // Validate both parts are valid wires
    if before_points.len() < 2 || after_points.len() < 2 {
        return None;
    }

    // Remove original wire
    document.wires.remove(wire_idx);

    // Create two new wires
    let id1 = identity.allocate(document);
    let id2 = identity.allocate(document);

    document.wires.push(Wire::new(id1, before_points));
    document.wires.push(Wire::new(id2, after_points));

    Some((id1, id2))
}

/// Simplify wire path by removing intermediate points on straight segments
/// Does `points` contain any collinear interior vertex that
/// `simplify_wire_path` would remove? Cheap pre-check so cleanup passes
/// don't clone paths that are already minimal.
fn has_collinear_vertices(points: &[Point]) -> bool {
    points
        .windows(3)
        .any(|w| (w[0].x == w[1].x && w[1].x == w[2].x) || (w[0].y == w[1].y && w[1].y == w[2].y))
}

/// Remove intermediate points on horizontal or vertical straight runs.
pub fn simplify_wire_path(points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 2 {
        return points;
    }

    let mut result = Vec::with_capacity(points.len());
    result.push(points[0]);

    for i in 1..points.len() - 1 {
        let prev = &points[i - 1];
        let curr = &points[i];
        let next = &points[i + 1];

        let all_same_x = prev.x == curr.x && curr.x == next.x;
        let all_same_y = prev.y == curr.y && curr.y == next.y;

        if !all_same_x && !all_same_y {
            result.push(*curr);
        }
    }

    result.push(*points.last().unwrap());
    result
}

/// Optimize all wires by removing collinear intermediate points
pub fn optimize_all_wires(document: &mut SchematicDocument) -> bool {
    let mut changed = false;
    for wire in &mut document.wires {
        // Most wires are already minimal — don't clone their paths.
        if !has_collinear_vertices(&wire.points) {
            continue;
        }
        let simplified = simplify_wire_path(std::mem::take(&mut wire.points));
        wire.points = simplified;
        changed = true;
    }
    changed
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
pub fn remove_degenerate_segments(document: &mut SchematicDocument) -> (usize, usize) {
    let mut wires_modified = 0;
    let initial_wire_count = document.wires.len();

    // Phase 1: Remove zero-length segments from each wire
    for wire in &mut document.wires {
        let original_len = wire.points.len();

        // Remove consecutive duplicate points (zero-length segments)
        let mut cleaned = Vec::with_capacity(wire.points.len());
        for point in &wire.points {
            if cleaned.last() != Some(point) {
                cleaned.push(*point);
            }
        }

        if cleaned.len() != original_len {
            wire.points = cleaned;
            wires_modified += 1;
        }
    }

    // Phase 2: Remove wires that are now invalid (< 2 points)
    let wires_to_remove: Vec<u64> = document
        .wires
        .iter()
        .filter(|w| w.points.len() < 2)
        .map(|w| w.id)
        .collect();

    for wire_id in &wires_to_remove {
        log::info!("Removing zero-length wire id={}", wire_id);
    }

    document.wires.retain(|w| w.points.len() >= 2);

    let wires_removed = initial_wire_count - document.wires.len();

    (wires_modified, wires_removed)
}

/// Move matching vertices and junctions after any required T-junction splits.
pub fn move_vertices_at(document: &mut SchematicDocument, old_pos: Point, new_pos: Point) -> bool {
    if old_pos == new_pos {
        return false;
    }

    let mut moved = false;

    // Move all wire vertices at this position
    for wire in &mut document.wires {
        for point in &mut wire.points {
            if *point == old_pos {
                *point = new_pos;
                moved = true;
            }
        }
    }

    // Also move any junction at this position
    for junction in &mut document.junctions {
        if junction.pos == old_pos {
            junction.pos = new_pos;
        }
    }

    moved
}

/// Existing wires crossing a point without a vertex, in document order.
pub fn wires_to_split_at(document: &SchematicDocument, point: Point) -> Vec<u64> {
    document
        .wires
        .iter()
        .filter(|w| {
            // Wire passes through point mid-segment (not at a vertex)
            w.contains_point(point) && !w.points.contains(&point)
        })
        .map(|w| w.id)
        .collect()
}
