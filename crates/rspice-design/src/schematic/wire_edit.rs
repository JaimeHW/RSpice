//! Wire document edits and path cleanup, independent of routing gestures.

use super::{
    document::SchematicDocument,
    identity::SchematicIdentity,
    wire::{Wire, WireSegment},
};
use rspice_design_model::Point;
use std::collections::HashSet;

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

/// Does `points` contain any interior vertex that `simplify_wire_path` would
/// remove? Cheap pre-check so cleanup passes don't clone paths that are
/// already minimal.
fn has_collinear_vertices(points: &[Point]) -> bool {
    points
        .windows(3)
        .any(|w| redundant_vertex(w[0], w[1], w[2]))
}

/// Whether `vertex` adds nothing to the run from `previous` to `next`: the
/// three lie on one horizontal or vertical line and `vertex` is not past
/// either end. A vertex the run folds back at is where the conductor
/// reaches farthest, so it stays.
fn redundant_vertex(previous: Point, vertex: Point, next: Point) -> bool {
    let on_one_line = (previous.x == vertex.x && vertex.x == next.x)
        || (previous.y == vertex.y && vertex.y == next.y);
    let delta = |from: i32, to: i32| i64::from(to) - i64::from(from);
    let onward = delta(previous.x, vertex.x) * delta(vertex.x, next.x)
        + delta(previous.y, vertex.y) * delta(vertex.y, next.y);
    on_one_line && onward >= 0
}

/// Remove intermediate points on horizontal or vertical straight runs.
pub fn simplify_wire_path(points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 2 {
        return points;
    }

    let mut result: Vec<Point> = Vec::with_capacity(points.len());
    result.push(points[0]);

    for i in 1..points.len() - 1 {
        let previous = *result.last().unwrap_or(&points[0]);
        if !redundant_vertex(previous, points[i], points[i + 1]) {
            result.push(points[i]);
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

    // Also move any junction at this position. A junction counts as a move:
    // the caller advances the gesture's anchor only when this reports one, and
    // a point that carries a junction but no wire vertex would otherwise move
    // once and then be dragged from an anchor it no longer sits on, freezing
    // the gesture for as long as the pointer is held.
    for junction in &mut document.junctions {
        if junction.pos == old_pos {
            junction.pos = new_pos;
            moved = true;
        }
    }

    moved
}

/// Split each wire a pin lands inside, so the pin owns a vertex there and the
/// wire follows the part when it moves. The extractor already joins a pin to
/// a wire it lies on; without the vertex, a move leaves the wire behind.
///
/// A pin exactly where wire interiors cross with no junction is left alone:
/// splitting there would join both wires, and which one the pin means is the
/// user's call, which the design check asks for.
pub fn split_wires_under_pins(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    pins: &[Point],
) -> bool {
    let crossings = super::junction_edit::CrossingIndex::new(&document.wires, &document.junctions);
    let mut changed = false;
    for &pin in pins {
        if crossings.is_unmarked_crossing(pin) {
            continue;
        }
        for wire in wires_to_split_at(document, pin) {
            changed |= split_wire(document, identity, wire, pin).is_some();
        }
    }
    changed
}

/// Lay single-segment wires that run along each other on one line as one
/// conductor, so no stretch of wire is drawn twice and no dot marks a
/// branch that is not there. With `join_end_to_end`, wires that meet end to
/// end on one line with nothing else at the meeting point are joined too.
///
/// A pair takes part only when one of them is in `seeds`, or always when
/// `seeds` is `None`. The pair becomes one wire over their combined extent,
/// under the lower id. An end of either that falls inside that extent stays
/// a vertex when anything else meets it there (another wire, a pin, a dot,
/// a label or a tap end), so whatever met it still meets a vertex.
pub fn merge_collinear_wires(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    seeds: Option<&HashSet<u64>>,
    join_end_to_end: bool,
) -> bool {
    let pins = super::junction_edit::placed_pins(document)
        .into_iter()
        .collect::<HashSet<_>>();
    let mut seeds = seeds.cloned();
    let mut changed = false;
    while let Some((first, second)) =
        next_collinear_pair(document, seeds.as_ref(), &pins, join_end_to_end)
    {
        let laid = merge_wire_pair(document, identity, first, second, &pins);
        if let Some(seeds) = seeds.as_mut() {
            seeds.extend(laid);
        }
        changed = true;
    }
    changed
}

/// Which line a segment lies on: its reduced direction and its offset.
fn line_key(start: Point, end: Point) -> Option<((i64, i64), i64)> {
    let mut dx = i64::from(end.x) - i64::from(start.x);
    let mut dy = i64::from(end.y) - i64::from(start.y);
    if dx == 0 && dy == 0 {
        return None;
    }
    let divisor = greatest_common_divisor(dx.abs(), dy.abs());
    dx /= divisor;
    dy /= divisor;
    if dx < 0 || (dx == 0 && dy < 0) {
        dx = -dx;
        dy = -dy;
    }
    Some(((dx, dy), dy * i64::from(start.x) - dx * i64::from(start.y)))
}

fn greatest_common_divisor(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Whether anything but wires `first` and `second` meets `point`.
fn meets_anything_else(
    document: &SchematicDocument,
    pins: &HashSet<Point>,
    point: Point,
    first: u64,
    second: u64,
) -> bool {
    pins.contains(&point)
        || document
            .wires
            .iter()
            .any(|wire| wire.id != first && wire.id != second && wire.contains_point(point))
        || document
            .junctions
            .iter()
            .any(|junction| junction.pos == point)
        || document.net_labels.iter().any(|label| label.pos == point)
        || document
            .bus_taps
            .iter()
            .any(|tap| tap.connection_point == point)
}

fn next_collinear_pair(
    document: &SchematicDocument,
    seeds: Option<&HashSet<u64>>,
    pins: &HashSet<Point>,
    join_end_to_end: bool,
) -> Option<(u64, u64)> {
    let mut lines: std::collections::HashMap<_, Vec<&Wire>> = std::collections::HashMap::new();
    for wire in &document.wires {
        if let [start, end] = wire.points.as_slice()
            && let Some(key) = line_key(*start, *end)
        {
            lines.entry(key).or_default().push(wire);
        }
    }
    let mut keys = lines.keys().copied().collect::<Vec<_>>();
    keys.sort_unstable();
    for key in keys {
        let wires = &lines[&key];
        for (index, left) in wires.iter().enumerate() {
            for right in &wires[index + 1..] {
                if seeds
                    .is_some_and(|seeds| !seeds.contains(&left.id) && !seeds.contains(&right.id))
                {
                    continue;
                }
                let (a, b) = (left.points[0], left.points[1]);
                let (c, d) = (right.points[0], right.points[1]);
                let left_segment = WireSegment::new(a, b);
                let right_segment = WireSegment::new(c, d);
                let shared = [a, b].into_iter().find(|point| *point == c || *point == d);
                let overlap = left_segment.contains_point(c) && c != a && c != b
                    || left_segment.contains_point(d) && d != a && d != b
                    || right_segment.contains_point(a) && a != c && a != d
                    || right_segment.contains_point(b) && b != c && b != d
                    || (a == c && b == d)
                    || (a == d && b == c);
                let end_to_end = !overlap
                    && shared.is_some_and(|point| {
                        !meets_anything_else(document, pins, point, left.id, right.id)
                    });
                if overlap || (join_end_to_end && end_to_end) {
                    return Some((left.id, right.id));
                }
            }
        }
    }
    None
}

/// Replace two collinear wires with the pieces of their combined extent,
/// broken only where something else meets a former end. Returns the ids
/// of the pieces. Pin connections on either wire move to the piece that
/// keeps their vertex.
fn merge_wire_pair(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    first: u64,
    second: u64,
    pins: &HashSet<Point>,
) -> Vec<u64> {
    let old = document
        .wires
        .iter()
        .filter(|wire| wire.id == first || wire.id == second)
        .map(|wire| (wire.id, wire.points.clone()))
        .collect::<Vec<_>>();
    let mut ends = old
        .iter()
        .flat_map(|(_, points)| points.iter().copied())
        .collect::<Vec<_>>();
    ends.sort_by_key(|point| (point.x, point.y));
    ends.dedup();
    let (Some(&start), Some(&end)) = (ends.first(), ends.last()) else {
        return Vec::new();
    };
    let mut breaks = vec![start];
    breaks.extend(
        ends[1..ends.len() - 1]
            .iter()
            .copied()
            .filter(|point| meets_anything_else(document, pins, *point, first, second)),
    );
    breaks.push(end);
    let mut ids = vec![first.min(second), first.max(second)];
    while ids.len() < breaks.len() - 1 {
        ids.push(identity.allocate(document));
    }
    ids.truncate(breaks.len() - 1);
    let pieces = breaks
        .windows(2)
        .zip(&ids)
        .map(|(pair, id)| Wire::new(*id, pair.to_vec()))
        .collect::<Vec<_>>();
    let position = document
        .wires
        .iter()
        .position(|wire| wire.id == first || wire.id == second)
        .unwrap_or(document.wires.len());
    document
        .wires
        .retain(|wire| wire.id != first && wire.id != second);
    let position = position.min(document.wires.len());
    document
        .wires
        .splice(position..position, pieces.iter().cloned());
    document.connections.retain_mut(|connection| {
        let Some((_, points)) = old.iter().find(|(id, _)| *id == connection.wire_id) else {
            return true;
        };
        let Some(point) = points.get(connection.point_index) else {
            return false;
        };
        let Some((piece, index)) = pieces.iter().find_map(|piece| {
            piece
                .points
                .iter()
                .position(|vertex| vertex == point)
                .map(|index| (piece.id, index))
        }) else {
            return false;
        };
        connection.wire_id = piece;
        connection.point_index = index;
        true
    });
    ids
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

#[cfg(test)]
mod tests {
    use super::super::net_label::Junction;
    use super::*;

    /// A point can carry a junction without any wire holding a vertex there —
    /// a junction left behind by an earlier edit is the common way. Dragging
    /// it moves it, so it has to be reported as a move: the caller advances
    /// the gesture's anchor only on `true`, and an anchor left behind freezes
    /// the drag after its first step.
    #[test]
    fn moving_a_point_that_only_carries_a_junction_reports_the_move() {
        let mut document = SchematicDocument::default();
        document
            .junctions
            .push(Junction::new(1, Point::new(100, 100)));

        assert!(move_vertices_at(
            &mut document,
            Point::new(100, 100),
            Point::new(100, 90)
        ));
        assert_eq!(document.junctions[0].pos, Point::new(100, 90));
    }

    /// And a point nothing sits on still reports nothing, so a gesture that
    /// grabbed empty space does not claim an edit.
    #[test]
    fn moving_a_point_nothing_sits_on_reports_no_move() {
        let mut document = SchematicDocument::default();
        document
            .junctions
            .push(Junction::new(1, Point::new(100, 100)));

        assert!(!move_vertices_at(
            &mut document,
            Point::new(400, 400),
            Point::new(400, 390)
        ));
        assert_eq!(document.junctions[0].pos, Point::new(100, 100));
    }

    /// A route that runs out to (40, 0) and folds back to (20, 0) still
    /// reaches (40, 0); only a vertex strictly between its neighbours on one
    /// line is redundant.
    #[test]
    fn simplification_keeps_a_folded_back_span() {
        let folded = vec![Point::origin(), Point::new(40, 0), Point::new(20, 0)];
        assert_eq!(simplify_wire_path(folded.clone()), folded);

        assert_eq!(
            simplify_wire_path(vec![Point::origin(), Point::new(20, 0), Point::new(40, 0)]),
            vec![Point::origin(), Point::new(40, 0)]
        );
        assert_eq!(
            simplify_wire_path(vec![Point::origin(), Point::origin(), Point::new(0, 40)]),
            vec![Point::origin(), Point::new(0, 40)]
        );

        let mut document = SchematicDocument {
            wires: vec![Wire::new(1, folded.clone())],
            ..SchematicDocument::default()
        };
        optimize_all_wires(&mut document);
        assert_eq!(document.wires[0].points, folded);
    }
}
