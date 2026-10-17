//! Junction and net-label document edits and geometric maintenance.

use super::{
    document::SchematicDocument,
    identity::SchematicIdentity,
    net_label::{Junction, NetLabel},
    wire::{Wire, WireSegment},
};
use rspice_design_model::Point;
use std::collections::HashSet;

/// Row/column interval index over wire segments. Built once per topology
/// pass; point queries scan only the point's row and column buckets instead
/// of every wire — junction maintenance drops from O(wires² · points) to
/// O(segments + vertices · bucket).
///
/// 45° and free-angle segments sit in a plain list. The extractor connects
/// through a junction on a diagonal's interior (`WireSegment::contains_point`
/// is exact at any angle), so maintenance must see those interiors too, or it
/// deletes the junction that joins them.
struct SegmentIndex {
    /// Horizontal segments: y → (x0, x1, wire id) with x0 ≤ x1.
    rows: std::collections::HashMap<i32, Vec<(i32, i32, u64)>>,
    /// Vertical segments: x → (y0, y1, wire id) with y0 ≤ y1.
    cols: std::collections::HashMap<i32, Vec<(i32, i32, u64)>>,
    /// Segments that are neither horizontal nor vertical.
    diagonals: Vec<(WireSegment, u64)>,
    /// (wire id, vertex) pairs — distinguishes interiors from vertices.
    vertices: HashSet<(u64, Point)>,
}

impl SegmentIndex {
    fn build(wires: &[Wire]) -> Self {
        let mut index = Self {
            rows: std::collections::HashMap::new(),
            cols: std::collections::HashMap::new(),
            diagonals: Vec::new(),
            vertices: HashSet::new(),
        };
        for wire in wires {
            for point in &wire.points {
                index.vertices.insert((wire.id, *point));
            }
            for seg in wire.points.windows(2) {
                let (a, b) = (seg[0], seg[1]);
                if a.y == b.y {
                    let (x0, x1) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
                    index.rows.entry(a.y).or_default().push((x0, x1, wire.id));
                } else if a.x == b.x {
                    let (y0, y1) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
                    index.cols.entry(a.x).or_default().push((y0, y1, wire.id));
                } else {
                    index.diagonals.push((WireSegment::new(a, b), wire.id));
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
        out.extend(
            self.diagonals
                .iter()
                .filter(|(segment, _)| segment.contains_point(v))
                .map(|&(_, id)| id),
        );
        out.sort_unstable();
        out.dedup();
    }

    /// How many different directions the segments whose interior holds `v`
    /// run in — a segment that ends at `v` does not count. Two or more is a
    /// crossing; any number of segments along one line is not.
    fn interior_directions(&self, v: Point) -> usize {
        let mut directions: Vec<(i64, i64)> = Vec::new();
        let mut add = |direction| {
            if !directions.contains(&direction) {
                directions.push(direction);
            }
        };
        if self
            .rows
            .get(&v.y)
            .is_some_and(|row| row.iter().any(|&(x0, x1, _)| x0 < v.x && v.x < x1))
        {
            add((1, 0));
        }
        if self
            .cols
            .get(&v.x)
            .is_some_and(|col| col.iter().any(|&(y0, y1, _)| y0 < v.y && v.y < y1))
        {
            add((0, 1));
        }
        for (segment, _) in &self.diagonals {
            if segment.start != v && segment.end != v && segment.contains_point(v) {
                add(line_direction(segment.start, segment.end));
            }
        }
        directions.len()
    }
}

/// The direction of the line through `a` and `b`, the same for either order
/// and for any two points on the line.
fn line_direction(a: Point, b: Point) -> (i64, i64) {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    let (dx, dy) = (
        i64::from(b.x) - i64::from(a.x),
        i64::from(b.y) - i64::from(a.y),
    );
    let divisor = gcd(dx.abs(), dy.abs()).max(1);
    let (dx, dy) = (dx / divisor, dy / divisor);
    if dx < 0 || (dx == 0 && dy < 0) {
        (-dx, -dy)
    } else {
        (dx, dy)
    }
}

/// Where wire interiors cross with no vertex and no dot. Nothing joins the
/// wires there, so a net label, a pin or a bus-tap end placed exactly there
/// does not say which wire it means.
pub struct CrossingIndex {
    index: SegmentIndex,
    vertices: HashSet<Point>,
    junctions: HashSet<Point>,
}

impl CrossingIndex {
    /// The crossings among `wires`, given the dots in `junctions`.
    pub fn new(wires: &[Wire], junctions: &[Junction]) -> Self {
        Self {
            index: SegmentIndex::build(wires),
            vertices: wires
                .iter()
                .flat_map(|wire| wire.points.iter().copied())
                .collect(),
            junctions: junctions.iter().map(|junction| junction.pos).collect(),
        }
    }

    /// Whether two wires cross at `point` with neither having a vertex there
    /// and no junction marking it.
    pub fn is_unmarked_crossing(&self, point: Point) -> bool {
        !self.junctions.contains(&point)
            && !self.vertices.contains(&point)
            && self.index.interior_directions(point) >= 2
    }
}

/// Where a junction dot belongs: the one rule automatic placement adds dots
/// by, orphan cleanup keeps them by, and so the canvas paints by.
///
/// A dot marks a point where the extractor joins three or more conductor
/// ends:
/// - three or more wire ends meet;
/// - a wire ends on another wire's interior, at any angle (a T);
/// - two or more wires meet at a component pin.
///
/// A wire's interior vertex is two ends, and so is a wire running straight
/// through. Where wire interiors cross with no vertex, only a dot the user
/// placed joins them: the dot is kept there and never added, even under a
/// pin or a label, which the design check reports as ambiguous instead.
pub struct JunctionRule {
    index: SegmentIndex,
    /// Wire ends at each vertex.
    vertex_ends: std::collections::HashMap<Point, usize>,
    /// Component pins at each point.
    pins: std::collections::HashMap<Point, usize>,
}

impl JunctionRule {
    /// The rule over `document`'s wires and the given pin positions.
    pub fn new(document: &SchematicDocument, pins: impl IntoIterator<Item = Point>) -> Self {
        let mut vertex_ends = std::collections::HashMap::new();
        for wire in &document.wires {
            let last = wire.points.len().saturating_sub(1);
            for (index, point) in wire.points.iter().enumerate() {
                let ends = if index == 0 || index == last { 1 } else { 2 };
                *vertex_ends.entry(*point).or_insert(0) += ends;
            }
        }
        let mut pin_counts = std::collections::HashMap::new();
        for pin in pins {
            *pin_counts.entry(pin).or_insert(0) += 1;
        }
        Self {
            index: SegmentIndex::build(&document.wires),
            vertex_ends,
            pins: pin_counts,
        }
    }

    /// The rule over `document`, its pins read from each part's placement.
    pub fn for_document(document: &SchematicDocument) -> Self {
        Self::new(document, placed_pins(document))
    }

    /// Whether a dot belongs at `point`. `marked` says whether the user's
    /// dot is already there, which is all that joins an interior crossing.
    pub fn dot_belongs(&self, point: Point, marked: bool) -> bool {
        let vertex_ends = self.vertex_ends.get(&point).copied().unwrap_or(0);
        let mut through = Vec::new();
        self.index.wires_through(point, &mut through);
        let interiors = through
            .iter()
            .filter(|&&wire| !self.index.vertices.contains(&(wire, point)))
            .count();
        if vertex_ends == 0 && !marked && self.index.interior_directions(point) >= 2 {
            return false;
        }
        let pins = self.pins.get(&point).copied().unwrap_or(0);
        vertex_ends + 2 * interiors + pins >= 3
    }

    /// Every point where a dot belongs, given the dots `document` has, in a
    /// stable order.
    pub fn points(&self, document: &SchematicDocument) -> Vec<Point> {
        let marked = document
            .junctions
            .iter()
            .map(|junction| junction.pos)
            .collect::<HashSet<_>>();
        let mut points = self
            .vertex_ends
            .keys()
            .chain(self.pins.keys())
            .chain(marked.iter())
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|point| self.dot_belongs(*point, marked.contains(point)))
            .collect::<Vec<_>>();
        points.sort_by_key(|point| (point.x, point.y));
        points
    }
}

/// Every pin of every placed part, from the part's own placement.
pub fn placed_pins(document: &SchematicDocument) -> Vec<Point> {
    document
        .components
        .iter()
        .flat_map(|component| component.terminal_positions_resolved(None))
        .map(|(_, point)| point)
        .collect()
}

/// Where a dot belongs in `document`; see [`JunctionRule`].
pub fn junction_points(document: &SchematicDocument) -> Vec<Point> {
    JunctionRule::for_document(document).points(document)
}

/// Bring the dots in line with [`JunctionRule`]: drop each one that marks
/// nothing, then add one wherever a dot belongs and none is.
pub fn auto_place_junctions(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
) -> bool {
    let rule = JunctionRule::for_document(document);
    let mut changes = retain_meaningful_junctions(document, &rule) > 0;
    let mut marked = document
        .junctions
        .iter()
        .map(|junction| junction.pos)
        .collect::<HashSet<_>>();
    for point in rule.points(document) {
        if marked.insert(point) {
            let id = identity.allocate(document);
            document.junctions.push(Junction::new(id, point));
            changes = true;
        }
    }
    changes
}

/// Remove the dots that mark nothing, without opening a transaction or
/// invalidating caches. Composite state operations call this before their
/// single dirty and topology update.
pub fn remove_orphan_junctions(document: &mut SchematicDocument) -> usize {
    if document.junctions.is_empty() {
        return 0;
    }
    let rule = JunctionRule::for_document(document);
    retain_meaningful_junctions(document, &rule)
}

/// Keep the dots that mark something, one per point. A turn or a move can
/// carry a dot onto a point that already has one, and the second marks
/// nothing the first does not; the first in drawing order stays.
fn retain_meaningful_junctions(document: &mut SchematicDocument, rule: &JunctionRule) -> usize {
    let before = document.junctions.len();
    let mut marked = HashSet::with_capacity(before);
    document
        .junctions
        .retain(|junction| marked.insert(junction.pos) && rule.dot_belongs(junction.pos, true));
    before - document.junctions.len()
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

    /// Two wire ends meeting join each other with or without a dot, so a dot
    /// there marks nothing. It is exactly what deleting a T's stem leaves on
    /// the straight run, and maintenance removes it.
    #[test]
    fn a_dot_where_only_two_wire_ends_meet_is_removed() {
        let point = Point::new(10, 10);
        let mut document = SchematicDocument {
            wires: vec![
                Wire::new(1, vec![Point::new(0, 10), point]),
                Wire::new(2, vec![point, Point::new(10, 20)]),
            ],
            ..SchematicDocument::default()
        };
        document.junctions.push(Junction::new(3, point));

        maintain(&mut document);

        assert!(document.junctions.is_empty());
    }

    fn maintain(document: &mut SchematicDocument) {
        let mut schematic = super::super::owned::Schematic::from_document(document.clone());
        schematic.auto_place_junctions();
        *document = schematic.document().clone();
    }

    fn resistor(id: u64, position: Point) -> super::super::component::Component {
        super::super::component::Component::new(
            id,
            super::super::component_type::ComponentType::Resistor,
            position,
        )
    }

    /// A resistor at (20, 0) has its pins at the origin and at (40, 0).
    #[test]
    fn two_wires_meeting_at_a_pin_get_a_dot_and_one_does_not() {
        let mut document = SchematicDocument {
            components: vec![resistor(1, Point::new(20, 0))],
            wires: vec![
                Wire::segment(2, Point::origin(), Point::new(-20, 0)),
                Wire::segment(3, Point::origin(), Point::new(0, 20)),
                Wire::segment(4, Point::new(40, 0), Point::new(60, 0)),
            ],
            ..SchematicDocument::default()
        };

        maintain(&mut document);

        assert_eq!(
            document
                .junctions
                .iter()
                .map(|junction| junction.pos)
                .collect::<Vec<_>>(),
            vec![Point::origin()]
        );
    }

    /// A pin on a crossing of two wire interiors is ambiguous: no dot is
    /// invented there; the design check asks the user to mark it.
    #[test]
    fn a_pin_on_an_unmarked_crossing_gets_no_dot() {
        let mut document = SchematicDocument {
            components: vec![resistor(1, Point::new(20, 0))],
            wires: vec![
                Wire::segment(2, Point::new(-20, 0), Point::new(20, 0)),
                Wire::segment(3, Point::new(0, -20), Point::new(0, 20)),
            ],
            ..SchematicDocument::default()
        };

        maintain(&mut document);

        assert!(document.junctions.is_empty());
    }

    /// Conductor ends at `point`, counted the slow way: (ends of wires with a
    /// vertex there, wires running through it, pins on it).
    fn ends_at(
        document: &SchematicDocument,
        pins: &[Point],
        point: Point,
    ) -> (usize, usize, usize) {
        let mut vertex_ends = 0;
        let mut interiors = 0;
        for wire in &document.wires {
            let last = wire.points.len().saturating_sub(1);
            let at_vertex = wire
                .points
                .iter()
                .enumerate()
                .filter(|(_, vertex)| **vertex == point)
                .map(|(index, _)| if index == 0 || index == last { 1 } else { 2 })
                .sum::<usize>();
            if at_vertex > 0 {
                vertex_ends += at_vertex;
            } else if wire.segments().any(|segment| segment.contains_point(point)) {
                interiors += 1;
            }
        }
        let pins = pins.iter().filter(|pin| **pin == point).count();
        (vertex_ends, interiors, pins)
    }

    /// The dots maintenance leaves are exactly the points where the extractor
    /// joins three or more conductor ends without needing a dot, plus the
    /// crossings a dot itself joins; and every dot joins what touches it.
    fn assert_dots_follow_the_extractor(name: &str, document: &SchematicDocument) {
        let pins = document
            .components
            .iter()
            .flat_map(|component| component.terminal_positions_resolved(None))
            .map(|(_, point)| point)
            .collect::<Vec<_>>();
        let extracted = crate::connectivity::extract(document, |_| None, |_| None);
        let dots = document
            .junctions
            .iter()
            .map(|junction| junction.pos)
            .collect::<HashSet<_>>();
        assert_eq!(
            dots.len(),
            document.junctions.len(),
            "{name}: one dot per point"
        );
        for &dot in &dots {
            let (vertex_ends, interiors, on_pins) = ends_at(document, &pins, dot);
            assert!(
                vertex_ends + 2 * interiors + on_pins >= 3,
                "{name}: the dot at {dot:?} marks fewer than three conductor ends"
            );
            let net = extracted
                .net_at(dot)
                .unwrap_or_else(|| panic!("{name}: the dot at {dot:?} is on no net"));
            for wire in document
                .wires
                .iter()
                .filter(|wire| wire.contains_point(dot))
            {
                assert_eq!(
                    extracted.net_of_wire(wire.id).map(|net| net.id),
                    Some(net.id),
                    "{name}: the dot at {dot:?} does not join wire {}",
                    wire.id
                );
            }
            for terminal in extracted.terminals.iter().filter(|t| t.point == dot) {
                assert_eq!(
                    terminal.net_name,
                    net.spice_name(),
                    "{name}: pin at {dot:?}"
                );
            }
        }
        let candidates = document
            .wires
            .iter()
            .flat_map(|wire| wire.points.iter().copied())
            .chain(pins.iter().copied())
            .collect::<HashSet<_>>();
        for point in candidates {
            let (vertex_ends, interiors, on_pins) = ends_at(document, &pins, point);
            let crossing = vertex_ends == 0 && interiors >= 2;
            if !crossing && vertex_ends + 2 * interiors + on_pins >= 3 {
                assert!(
                    dots.contains(&point),
                    "{name}: no dot at the join {point:?}"
                );
            }
        }
    }

    fn fixture_sheets() -> Vec<(&'static str, SchematicDocument)> {
        let sheet = |wires: Vec<Wire>, components, junctions| SchematicDocument {
            wires,
            components,
            junctions,
            ..SchematicDocument::default()
        };
        vec![
            (
                "orthogonal T",
                sheet(
                    vec![
                        Wire::segment(1, Point::origin(), Point::new(40, 0)),
                        Wire::segment(2, Point::new(20, 0), Point::new(20, 20)),
                    ],
                    vec![],
                    vec![],
                ),
            ),
            (
                "T on a diagonal",
                sheet(
                    vec![
                        Wire::segment(1, Point::origin(), Point::new(40, 40)),
                        Wire::segment(2, Point::new(20, 60), Point::new(20, 20)),
                    ],
                    vec![],
                    vec![],
                ),
            ),
            (
                "marked crossing",
                sheet(
                    vec![
                        Wire::segment(1, Point::new(0, 20), Point::new(40, 20)),
                        Wire::segment(2, Point::new(20, 0), Point::new(20, 40)),
                    ],
                    vec![],
                    vec![Junction::new(3, Point::new(20, 20))],
                ),
            ),
            (
                "unmarked crossing",
                sheet(
                    vec![
                        Wire::segment(1, Point::new(0, 20), Point::new(40, 20)),
                        Wire::segment(2, Point::new(20, 0), Point::new(20, 40)),
                    ],
                    vec![],
                    vec![],
                ),
            ),
            (
                "four wire ends",
                sheet(
                    vec![
                        Wire::segment(1, Point::origin(), Point::new(20, 0)),
                        Wire::segment(2, Point::new(40, 0), Point::new(20, 0)),
                        Wire::segment(3, Point::new(20, 20), Point::new(20, 0)),
                        Wire::segment(4, Point::new(20, -20), Point::new(20, 0)),
                    ],
                    vec![],
                    vec![],
                ),
            ),
            (
                "pin and two wires",
                sheet(
                    vec![
                        Wire::segment(2, Point::origin(), Point::new(-20, 0)),
                        Wire::segment(3, Point::origin(), Point::new(0, 20)),
                    ],
                    vec![resistor(1, Point::new(20, 0))],
                    vec![],
                ),
            ),
            (
                "pin inside a wire",
                sheet(
                    vec![Wire::segment(2, Point::new(-20, 0), Point::new(20, 0))],
                    vec![resistor(1, Point::new(20, 0))],
                    vec![],
                ),
            ),
            (
                "dot on an L corner",
                sheet(
                    vec![
                        Wire::segment(1, Point::origin(), Point::new(20, 0)),
                        Wire::segment(2, Point::new(20, 0), Point::new(20, 20)),
                    ],
                    vec![],
                    vec![Junction::new(3, Point::new(20, 0))],
                ),
            ),
        ]
    }

    /// A small deterministic generator: wires between grid points and a few
    /// resistors, so the rule meets shapes nobody wrote down.
    fn random_sheet(seed: u64) -> SchematicDocument {
        let mut state = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let mut next = |bound: u64| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 33) % bound
        };
        let mut document = SchematicDocument::default();
        let mut id = 1;
        for _ in 0..8 {
            let x = i32::try_from(next(5)).unwrap_or(0) * 20;
            let y = i32::try_from(next(5)).unwrap_or(0) * 20;
            let length = i32::try_from(next(3) + 1).unwrap_or(1) * 20;
            let end = match next(3) {
                0 => Point::new(x + length, y),
                1 => Point::new(x, y + length),
                _ => Point::new(x + length, y + length),
            };
            document
                .wires
                .push(Wire::segment(id, Point::new(x, y), end));
            id += 1;
        }
        for _ in 0..next(3) {
            let x = i32::try_from(next(5)).unwrap_or(0) * 20 + 20;
            let y = i32::try_from(next(5)).unwrap_or(0) * 20;
            document.components.push(resistor(id, Point::new(x, y)));
            id += 1;
        }
        document
    }

    #[test]
    fn dots_sit_exactly_where_the_extractor_joins_three_conductor_ends() {
        let mut sheets = fixture_sheets();
        sheets.extend((0..200).map(|seed| ("random sheet", random_sheet(seed))));
        for (name, mut document) in sheets {
            maintain(&mut document);
            assert_dots_follow_the_extractor(name, &document);
        }
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

    /// The extractor joins two wires through a junction on a diagonal's
    /// interior, so maintenance must see that interior: dropping the dot
    /// would silently split one net into two.
    #[test]
    fn a_junction_on_a_diagonal_crossing_survives_maintenance() {
        let point = Point::new(20, 20);
        for crossing in [
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(1, vec![Point::new(0, 40), Point::new(40, 0)]),
        ] {
            let mut document = SchematicDocument {
                wires: vec![
                    crossing,
                    Wire::new(2, vec![Point::new(0, 0), Point::new(40, 40)]),
                ],
                ..SchematicDocument::default()
            };
            document.junctions.push(Junction::new(3, point));

            assert_eq!(remove_orphan_junctions(&mut document), 0);
            auto_place_junctions(&mut document, &mut SchematicIdentity::with_cursor(4));
            assert_eq!(document.junctions, vec![Junction::new(3, point)]);
        }
    }

    #[test]
    fn a_wire_ending_inside_a_diagonal_gets_a_dot() {
        let mut document = SchematicDocument {
            wires: vec![
                Wire::new(1, vec![Point::new(0, 0), Point::new(40, 40)]),
                Wire::new(2, vec![Point::new(20, 60), Point::new(20, 20)]),
            ],
            ..SchematicDocument::default()
        };

        assert!(auto_place_junctions(
            &mut document,
            &mut SchematicIdentity::with_cursor(3)
        ));
        assert_eq!(
            document
                .junctions
                .iter()
                .map(|junction| junction.pos)
                .collect::<Vec<_>>(),
            vec![Point::new(20, 20)]
        );
    }
}
