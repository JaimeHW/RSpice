//! Wire document edits and path cleanup, independent of routing gestures.

use super::{
    document::SchematicDocument,
    identity::SchematicIdentity,
    wire::{Wire, WireSegment, WiresThrough},
};
use rspice_design_model::Point;
use std::collections::{HashMap, HashSet};

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

/// Optimize all wires by removing collinear intermediate points, and the
/// vertices a wire doubles back at over a stretch another wire runs all of:
/// that other wire carries whatever reaches the stretch, so the doubled
/// span adds nothing.
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
    for index in 0..document.wires.len() {
        let wire = &document.wires[index];
        if !wire
            .points
            .windows(3)
            .any(|corner| folds_back(corner[0], corner[1], corner[2]))
        {
            continue;
        }
        let wire_id = wire.id;
        let mut points = wire.points.clone();
        let mut renumbered = (0..points.len()).collect::<Vec<_>>();
        if !retract_fold_backs(
            document,
            wire_id,
            &mut points,
            &mut renumbered,
            FoldStretch::Carried,
        ) {
            continue;
        }
        document.wires[index].points = points;
        for connection in document
            .connections
            .iter_mut()
            .filter(|connection| connection.wire_id == wire_id)
        {
            if let Some(&moved_to) = renumbered.get(connection.point_index) {
                connection.point_index = moved_to;
            }
        }
        changed = true;
    }
    changed
}

/// Whether the run from `before` to `vertex` turns straight back on itself
/// to `after`.
pub(super) fn folds_back(before: Point, vertex: Point, after: Point) -> bool {
    let delta = |from: i32, to: i32| i64::from(to) - i64::from(from);
    let (in_x, in_y) = (delta(before.x, vertex.x), delta(before.y, vertex.y));
    let (out_x, out_y) = (delta(vertex.x, after.x), delta(vertex.y, after.y));
    in_x * out_y == in_y * out_x && in_x * out_x + in_y * out_y < 0
}

/// Which doubled-over stretches may come out of a wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FoldStretch {
    /// Only one another wire runs all of.
    Carried,
    /// That, or one nothing but the wire itself reaches.
    CarriedOrBare,
}

/// Take out of `route`, wire `wire_id`'s vertices, each vertex it turns
/// straight back at where `may_go` lets the stretch it doubles over go: from
/// the vertex back to the nearer of its neighbours, which stays on the wire.
/// The ends stay where they are, except an end that runs out and straight
/// back onto the vertex before the turn: that point stays a vertex, so the
/// wire ends at the turn instead and nothing is parted. `point_indices` says
/// where each earlier vertex went and is renumbered to match. Returns
/// whether a vertex went.
pub(super) fn retract_fold_backs(
    document: &SchematicDocument,
    wire_id: u64,
    route: &mut Vec<Point>,
    point_indices: &mut [usize],
    may_go: FoldStretch,
) -> bool {
    let mut retracted = route.clone();
    let mut renumbered = point_indices.to_vec();
    loop {
        let last = retracted.len().saturating_sub(1);
        if last >= 2 && retracted[0] == retracted[2] {
            // The start goes; what went to it goes to its copy.
            for moved_to in &mut renumbered {
                *moved_to = if *moved_to == 0 { 1 } else { *moved_to - 1 };
            }
            retracted.remove(0);
        } else if last >= 2 && retracted[last] == retracted[last - 2] {
            for moved_to in renumbered.iter_mut().filter(|moved_to| **moved_to == last) {
                *moved_to = last - 2;
            }
            retracted.pop();
        } else {
            break;
        }
    }
    let mut index = 1;
    while index + 1 < retracted.len() {
        let (before, vertex, after) =
            (retracted[index - 1], retracted[index], retracted[index + 1]);
        let reach = |point: Point| {
            (i64::from(point.x) - i64::from(vertex.x)).abs()
                + (i64::from(point.y) - i64::from(vertex.y)).abs()
        };
        let kept = if reach(before) <= reach(after) {
            before
        } else {
            after
        };
        if !folds_back(before, vertex, after)
            || !stretch_can_go(document, wire_id, kept, vertex, may_go)
        {
            index += 1;
            continue;
        }
        remove_route_vertex(&mut retracted, &mut renumbered, index);
        if before == after {
            // Out and back to the same point: one of the two copies goes,
            // never an end of the wire.
            if index + 1 < retracted.len() {
                remove_route_vertex(&mut retracted, &mut renumbered, index);
            } else if index > 1 {
                remove_route_vertex(&mut retracted, &mut renumbered, index - 1);
            }
        }
        index = index.saturating_sub(1).max(1);
    }
    // Each run left joins two that lay on one line, so the wire keeps its
    // angles; only a run that shrank to nothing refuses the whole change.
    let retracted_any = retracted.len() < route.len()
        && retracted.len() >= 2
        && retracted.windows(2).all(|pair| pair[0] != pair[1]);
    if retracted_any {
        *route = retracted;
        point_indices.copy_from_slice(&renumbered);
    }
    retracted_any
}

/// Whether the stretch from `kept` to `dropped` can come out of wire
/// `wire_id` with every connection left as it was: another wire runs all of
/// it, and so carries whatever reaches it and meets this wire at `kept`; or,
/// where `may_go` allows, nothing but this wire reaches it, `kept` aside: no
/// other wire, dot, pin, net label or bus tap.
fn stretch_can_go(
    document: &SchematicDocument,
    wire_id: u64,
    kept: Point,
    dropped: Point,
    may_go: FoldStretch,
) -> bool {
    let others = move || document.wires.iter().filter(move |wire| wire.id != wire_id);
    let carried = others().any(|wire| {
        wire.segments()
            .any(|segment| segment.contains_point(kept) && segment.contains_point(dropped))
    });
    if carried {
        return true;
    }
    if may_go == FoldStretch::Carried {
        return false;
    }
    let stretch = WireSegment::new(kept, dropped);
    let on_stretch = |point: Point| point != kept && stretch.contains_point(point);
    let touches_a_wire = others()
        .any(|wire| wire.points.iter().copied().any(on_stretch) || wire.contains_point(dropped));
    !touches_a_wire
        && !document
            .junctions
            .iter()
            .any(|junction| on_stretch(junction.pos))
        && !document
            .net_labels
            .iter()
            .any(|label| on_stretch(label.pos))
        && !document
            .bus_taps
            .iter()
            .any(|tap| on_stretch(tap.connection_point))
        && !super::junction_edit::placed_pins(document)
            .into_iter()
            .any(on_stretch)
}

/// Lay each turn a wire doubles back at as two wires, where the turn could
/// not simply come out because something reaches the stretch it doubles
/// over: the wire goes on without the turn, and that stretch stays as a
/// wire of its own, from where the two runs meet out to the turn. Every
/// point the wire reached it still reaches, and no wire doubles back.
pub fn split_fold_backs(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
) -> bool {
    let mut changed = false;
    let mut index = 0;
    while index < document.wires.len() {
        let points = &document.wires[index].points;
        let Some(turn) = (1..points.len().saturating_sub(1))
            .find(|&turn| folds_back(points[turn - 1], points[turn], points[turn + 1]))
        else {
            index += 1;
            continue;
        };
        let (before, vertex, after) = (points[turn - 1], points[turn], points[turn + 1]);
        let reach = |point: Point| {
            (i64::from(point.x) - i64::from(vertex.x)).abs()
                + (i64::from(point.y) - i64::from(vertex.y)).abs()
        };
        let kept = if reach(before) <= reach(after) {
            before
        } else {
            after
        };
        let mut route = points.clone();
        let mut renumbered = (0..route.len()).collect::<Vec<_>>();
        remove_route_vertex(&mut route, &mut renumbered, turn);
        if before == after && turn + 1 < route.len() {
            remove_route_vertex(&mut route, &mut renumbered, turn);
        }
        if route.len() < 2 || route.windows(2).any(|pair| pair[0] == pair[1]) {
            index += 1;
            continue;
        }
        let wire_id = document.wires[index].id;
        document.wires[index].points = route;
        for connection in document
            .connections
            .iter_mut()
            .filter(|connection| connection.wire_id == wire_id)
        {
            if let Some(&moved_to) = renumbered.get(connection.point_index) {
                connection.point_index = moved_to;
            }
        }
        let stub = identity.allocate(document);
        document.wires.push(Wire::new(stub, vec![kept, vertex]));
        changed = true;
    }
    changed
}

/// Take vertex `index` out of `route` and renumber where the old vertices
/// went: those past it move one place back, and one that went to it goes to
/// the vertex that takes its place.
fn remove_route_vertex(route: &mut Vec<Point>, point_indices: &mut [usize], index: usize) {
    route.remove(index);
    for moved_to in point_indices
        .iter_mut()
        .filter(|moved_to| **moved_to > index)
    {
        *moved_to -= 1;
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
///
/// `parts` holds each part's pins. Where two pins of one part land inside
/// one straight run of a wire, the piece of it between them would run under
/// the part's body from pin to pin and short the part; that piece goes, so
/// the part stands in series where it was dropped.
pub fn split_wires_under_pins(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    parts: &[Vec<Point>],
) -> bool {
    let crossings = super::junction_edit::CrossingIndex::new(&document.wires, &document.junctions);
    let splits = |pin: Point| !crossings.is_unmarked_crossing(pin);
    let mut bridged = Vec::new();
    for pins in parts {
        for (index, &first) in pins.iter().enumerate() {
            for &second in &pins[index + 1..] {
                let one_run = document.wires.iter().any(|wire| {
                    !wire.points.contains(&first)
                        && !wire.points.contains(&second)
                        && wire.segments().any(|segment| {
                            segment.contains_point(first) && segment.contains_point(second)
                        })
                });
                if first != second && one_run && splits(first) && splits(second) {
                    bridged.push((first, second));
                }
            }
        }
    }
    let before = document
        .wires
        .iter()
        .map(|wire| wire.id)
        .collect::<HashSet<_>>();
    let mut changed = false;
    for &pin in parts.iter().flatten() {
        if !splits(pin) {
            continue;
        }
        for wire in wires_to_split_at(document, pin) {
            changed |= split_wire(document, identity, wire, pin).is_some();
        }
    }
    for (first, second) in bridged {
        if let Some(index) = document.wires.iter().position(|wire| {
            !before.contains(&wire.id)
                && (wire.points == [first, second] || wire.points == [second, first])
        }) {
            document.wires.remove(index);
            changed = true;
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
///
/// Pairs merge in a fixed order: the lowest line first (by direction, then
/// offset), and on a line the first pair in drawing order, until none is
/// left; the pieces take the place in the drawing of the wire whose id they
/// keep.
pub fn merge_collinear_wires(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    seeds: Option<&HashSet<u64>>,
    join_end_to_end: bool,
) -> bool {
    let mut merge = CollinearMerge::new(document, seeds.cloned(), join_end_to_end);
    for line in 0..merge.lines.len() {
        while let Some((left, right)) = merge.next_pair(line) {
            merge.merge_pair(identity, line, left, right);
        }
    }
    let Some(rewrite) = merge.finish() else {
        return false;
    };
    rewrite.apply(document);
    true
}

/// Which line a segment lies on: its reduced direction and its offset.
type LineKey = ((i64, i64), i64);

/// One single-segment wire of a line, as a merge holds it.
struct LineWire {
    /// Where it stands in the drawing: the position of the wire it came
    /// from, then its order among that wire's pieces.
    place: (usize, Vec<usize>),
    id: u64,
    ends: [Point; 2],
}

impl LineWire {
    /// [`Wire::contains_point`], for a wire of one segment.
    fn contains(&self, point: Point) -> bool {
        self.ends.contains(&point)
            || WireSegment::new(self.ends[0], self.ends[1]).contains_point(point)
    }
}

/// A merge of collinear wires, worked out against the drawing as it stood
/// and written back once.
///
/// A merge changes nothing off its own line: the pieces lie on the pair's
/// line and cover exactly the points the pair covered. So each line, lowest
/// first, merges until no pair on it qualifies, the order a fresh scan of
/// the whole drawing after every merge would take, and the drawing is
/// rewritten when every line is done. Whether anything else meets a point
/// is answered from what no merge changes (pins, dots, labels, tap ends and
/// the cover of every wire off the line) and from the line's own wires as
/// they stand. A fresh scan after each merge passed over every wire once
/// per merge, and a drawing whose rails join end to end along every row
/// took tens of milliseconds to clean up.
struct CollinearMerge<'a> {
    document: &'a SchematicDocument,
    seeds: Option<HashSet<u64>>,
    join_end_to_end: bool,
    /// Each line's single-segment wires in drawing order, lines in key
    /// order.
    lines: Vec<(LineKey, Vec<LineWire>)>,
    /// The line of each single-segment wire, by its position in the drawing.
    line_of: Vec<Option<LineKey>>,
    /// Pins, dots, label anchors and tap ends.
    fixed: HashSet<Point>,
    /// The drawing's wires through a point, built when first asked.
    through: Option<WiresThrough<'a>>,
    /// Ids the merges took out of the drawing, and ids they added to it.
    gone: HashSet<u64>,
    born: HashSet<u64>,
    /// The connections on each wire, by position in the connection list.
    connections: HashMap<u64, Vec<usize>>,
    /// Each moved connection's wire and vertex now, `None` once dropped.
    reconnected: HashMap<usize, Option<(u64, usize)>>,
    /// Positions in the drawing whose wire took part in a merge.
    merged: HashSet<usize>,
}

impl<'a> CollinearMerge<'a> {
    fn new(
        document: &'a SchematicDocument,
        seeds: Option<HashSet<u64>>,
        join_end_to_end: bool,
    ) -> Self {
        let mut lines: HashMap<LineKey, Vec<LineWire>> = HashMap::new();
        let mut line_of = vec![None; document.wires.len()];
        for (position, wire) in document.wires.iter().enumerate() {
            if let [start, end] = wire.points.as_slice()
                && let Some(key) = line_key(*start, *end)
            {
                line_of[position] = Some(key);
                lines.entry(key).or_default().push(LineWire {
                    place: (position, Vec::new()),
                    id: wire.id,
                    ends: [*start, *end],
                });
            }
        }
        let mut lines = lines.into_iter().collect::<Vec<_>>();
        lines.sort_unstable_by_key(|(key, _)| *key);
        let fixed = super::junction_edit::placed_pins(document)
            .into_iter()
            .chain(document.junctions.iter().map(|junction| junction.pos))
            .chain(document.net_labels.iter().map(|label| label.pos))
            .chain(document.bus_taps.iter().map(|tap| tap.connection_point))
            .collect();
        let mut connections: HashMap<u64, Vec<usize>> = HashMap::new();
        for (position, connection) in document.connections.iter().enumerate() {
            connections
                .entry(connection.wire_id)
                .or_default()
                .push(position);
        }
        Self {
            document,
            seeds,
            join_end_to_end,
            lines,
            line_of,
            fixed,
            through: None,
            gone: HashSet::new(),
            born: HashSet::new(),
            connections,
            reconnected: HashMap::new(),
            merged: HashSet::new(),
        }
    }

    /// The first pair on `line`, in drawing order, that merges: by position
    /// in the line's list.
    fn next_pair(&mut self, line: usize) -> Option<(usize, usize)> {
        let count = self.lines[line].1.len();
        for left in 0..count {
            for right in left + 1..count {
                let wires = &self.lines[line].1;
                let (first, second) = (wires[left].id, wires[right].id);
                if self
                    .seeds
                    .as_ref()
                    .is_some_and(|seeds| !seeds.contains(&first) && !seeds.contains(&second))
                {
                    continue;
                }
                let [a, b] = wires[left].ends;
                let [c, d] = wires[right].ends;
                let left_segment = WireSegment::new(a, b);
                let right_segment = WireSegment::new(c, d);
                let shared = [a, b].into_iter().find(|point| *point == c || *point == d);
                let overlap = left_segment.contains_point(c) && c != a && c != b
                    || left_segment.contains_point(d) && d != a && d != b
                    || right_segment.contains_point(a) && a != c && a != d
                    || right_segment.contains_point(b) && b != c && b != d
                    || (a == c && b == d)
                    || (a == d && b == c);
                if overlap
                    || (self.join_end_to_end
                        && shared.is_some_and(|point| {
                            !self.meets_anything_else(line, point, first, second)
                        }))
                {
                    return Some((left, right));
                }
            }
        }
        None
    }

    /// Whether anything but wires `first` and `second` meets `point` on
    /// `line`.
    fn meets_anything_else(&mut self, line: usize, point: Point, first: u64, second: u64) -> bool {
        if self.fixed.contains(&point) {
            return true;
        }
        let (key, wires) = &self.lines[line];
        if wires
            .iter()
            .any(|wire| wire.id != first && wire.id != second && wire.contains(point))
        {
            return true;
        }
        let document = self.document;
        self.through
            .get_or_insert_with(|| WiresThrough::new(&document.wires))
            .indices(point)
            .into_iter()
            .any(|position| self.line_of[position] != Some(*key))
    }

    /// Whether the drawing as merged so far holds `id`.
    fn in_use(&self, id: u64) -> bool {
        self.born.contains(&id)
            || (!self.gone.contains(&id) && super::identity::live_id_in_use(self.document, id))
    }

    /// Replace the pair at `left` and `right` on `line` with the pieces of
    /// their combined extent, broken only where something else meets a
    /// former end. Connections on either move to the piece that keeps their
    /// vertex.
    fn merge_pair(
        &mut self,
        identity: &mut SchematicIdentity,
        line: usize,
        left: usize,
        right: usize,
    ) {
        let wires = &self.lines[line].1;
        let old = [
            (wires[left].id, wires[left].ends),
            (wires[right].id, wires[right].ends),
        ];
        let (first, second) = (old[0].0, old[1].0);
        let mut ends = old
            .iter()
            .flat_map(|(_, ends)| ends.iter().copied())
            .collect::<Vec<_>>();
        ends.sort_by_key(|point| (point.x, point.y));
        ends.dedup();
        let (start, end) = (ends[0], ends[ends.len() - 1]);
        let mut breaks = vec![start];
        for &point in &ends[1..ends.len() - 1] {
            if self.meets_anything_else(line, point, first, second) {
                breaks.push(point);
            }
        }
        breaks.push(end);
        let mut ids = vec![first.min(second), first.max(second)];
        while ids.len() < breaks.len() - 1 {
            ids.push(identity.allocate_unless(|id| self.in_use(id)));
        }
        ids.truncate(breaks.len() - 1);
        let pieces = breaks
            .windows(2)
            .zip(&ids)
            .map(|(pair, id)| (*id, [pair[0], pair[1]]))
            .collect::<Vec<_>>();

        for id in [first, second] {
            if !ids.contains(&id) && !self.born.remove(&id) {
                self.gone.insert(id);
            }
        }
        for &id in &ids {
            if id != first && id != second && !self.gone.remove(&id) {
                self.born.insert(id);
            }
        }
        // Both wires' connections are taken before any is moved: a piece can
        // keep the id of either wire, and a moved connection is not moved
        // twice.
        let moving =
            old.map(|(wire, ends)| (ends, self.connections.remove(&wire).unwrap_or_default()));
        for (ends, positions) in moving {
            for position in positions {
                let point_index = match self.reconnected.get(&position) {
                    Some(Some((_, point_index))) => *point_index,
                    Some(None) => continue,
                    None => self.document.connections[position].point_index,
                };
                let moved = ends.get(point_index).and_then(|point| {
                    pieces.iter().find_map(|(piece, piece_ends)| {
                        piece_ends
                            .iter()
                            .position(|vertex| vertex == point)
                            .map(|index| (*piece, index))
                    })
                });
                if let Some((piece, _)) = moved {
                    self.connections.entry(piece).or_default().push(position);
                }
                self.reconnected.insert(position, moved);
            }
        }
        if let Some(seeds) = self.seeds.as_mut() {
            seeds.extend(ids.iter().copied());
        }

        // The pieces take the place in the drawing of the wire whose id they
        // keep, so a route that lays nothing new over existing wires leaves
        // the drawing exactly as it was and commits no step.
        let wires = &mut self.lines[line].1;
        self.merged.insert(wires[left].place.0);
        self.merged.insert(wires[right].place.0);
        let (kept, other) = if wires[left].id == first.min(second) {
            (left, right)
        } else {
            (right, left)
        };
        let place = wires[kept].place.clone();
        let split = pieces.len() > 1;
        let laid = pieces
            .into_iter()
            .enumerate()
            .map(|(order, (id, ends))| {
                let mut place = place.clone();
                if split {
                    place.1.push(order);
                }
                LineWire { place, id, ends }
            })
            .collect::<Vec<_>>();
        wires.remove(other);
        let at = if other < kept { kept - 1 } else { kept };
        wires.splice(at..=at, laid);
    }

    /// The rewrite the merges add up to, `None` when nothing merged.
    fn finish(self) -> Option<CollinearRewrite> {
        if self.merged.is_empty() {
            return None;
        }
        let mut pieces: HashMap<usize, Vec<(Vec<usize>, Wire)>> = self
            .merged
            .iter()
            .map(|position| (*position, Vec::new()))
            .collect();
        for (_, wires) in self.lines {
            for wire in wires {
                if let Some(laid) = pieces.get_mut(&wire.place.0) {
                    laid.push((wire.place.1, Wire::new(wire.id, wire.ends.to_vec())));
                }
            }
        }
        for laid in pieces.values_mut() {
            laid.sort_by(|left, right| left.0.cmp(&right.0));
        }
        Some(CollinearRewrite {
            pieces,
            reconnected: self.reconnected,
        })
    }
}

/// What a merge of collinear wires writes back.
struct CollinearRewrite {
    /// The wires that stand in place of each merged wire, in drawing order.
    pieces: HashMap<usize, Vec<(Vec<usize>, Wire)>>,
    /// Each moved connection's wire and vertex, `None` when it is dropped.
    reconnected: HashMap<usize, Option<(u64, usize)>>,
}

impl CollinearRewrite {
    fn apply(mut self, document: &mut SchematicDocument) {
        let wires = std::mem::take(&mut document.wires);
        document.wires.reserve(wires.len());
        for (position, wire) in wires.into_iter().enumerate() {
            match self.pieces.remove(&position) {
                Some(laid) => document
                    .wires
                    .extend(laid.into_iter().map(|(_, piece)| piece)),
                None => document.wires.push(wire),
            }
        }
        let mut position = 0;
        document.connections.retain_mut(|connection| {
            let kept = match self.reconnected.get(&position) {
                None => true,
                Some(None) => false,
                Some(Some((wire, point_index))) => {
                    connection.wire_id = *wire;
                    connection.point_index = *point_index;
                    true
                }
            };
            position += 1;
            kept
        });
    }
}

/// The line a segment lies on, `None` for a segment of no length.
fn line_key(start: Point, end: Point) -> Option<LineKey> {
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

    /// A wire that doubles back over a stretch another wire runs all of
    /// stops where the other wire meets it; one that doubles back over a
    /// stretch only partly run, or run by nothing, keeps its fold.
    #[test]
    fn cleanup_takes_out_a_fold_another_wire_runs() {
        let p = Point::new;
        let folded = vec![p(200, 180), p(200, 100), p(120, 100), p(180, 100)];
        let mut document = SchematicDocument {
            wires: vec![
                Wire::new(1, folded.clone()),
                Wire::new(2, vec![p(120, 100), p(180, 100)]),
            ],
            ..SchematicDocument::default()
        };
        assert!(optimize_all_wires(&mut document));
        assert_eq!(
            document.wires[0].points,
            vec![p(200, 180), p(200, 100), p(180, 100)]
        );

        document.wires = vec![
            Wire::new(1, folded.clone()),
            Wire::new(2, vec![p(140, 100), p(180, 100)]),
        ];
        assert!(!optimize_all_wires(&mut document));
        assert_eq!(document.wires[0].points, folded);
    }

    /// A wire that runs out to a point and straight back to where it began
    /// ends at the turn instead: both points stay on the wire.
    #[test]
    fn cleanup_ends_a_wire_that_runs_out_and_back_at_its_turn() {
        let p = Point::new;
        let mut document = SchematicDocument {
            wires: vec![Wire::new(1, vec![p(160, 120), p(200, 120), p(160, 120)])],
            connections: vec![super::super::wire::WireConnection::new(1, 2, 7, "-")],
            ..SchematicDocument::default()
        };
        assert!(optimize_all_wires(&mut document));
        assert_eq!(document.wires[0].points, vec![p(200, 120), p(160, 120)]);
        assert_eq!(document.connections[0].point_index, 1);
    }

    /// A turn that a T needs comes out of the wire, and the stretch it
    /// doubled over stays as a wire of its own out to the T.
    #[test]
    fn a_fold_something_needs_is_laid_as_two_wires() {
        let p = Point::new;
        let mut document = SchematicDocument {
            wires: vec![
                Wire::new(
                    1,
                    vec![
                        p(240, 60),
                        p(240, 260),
                        p(200, 260),
                        p(200, 100),
                        p(200, 140),
                    ],
                ),
                Wire::new(2, vec![p(120, 100), p(280, 100)]),
            ],
            ..SchematicDocument::default()
        };
        let mut identity = SchematicIdentity::with_cursor(10);
        assert!(split_fold_backs(&mut document, &mut identity));
        assert_eq!(
            document.wires[0].points,
            vec![p(240, 60), p(240, 260), p(200, 260), p(200, 140)]
        );
        assert_eq!(document.wires[2].points, vec![p(200, 140), p(200, 100)]);
        assert!(!split_fold_backs(&mut document, &mut identity));
    }
}
