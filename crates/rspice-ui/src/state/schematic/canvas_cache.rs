//! Frame-Coherent Canvas Cache
//!
//! Derived geometry the schematic canvas needs every frame: wire bounding
//! boxes for viewport culling and a vertex/junction index for O(1) hover
//! hit-tests. Rebuilt lazily when `topology_version` advances; never
//! persisted, and deliberately reset (not copied) on clone — a clone
//! rebuilds its own on first use.

use std::collections::{HashMap, HashSet};

use super::net_label::Junction;
use super::point::Point;
use super::state::SchematicState;
use super::wire::{Wire, WireSegment};

/// Cached per-frame canvas geometry, valid for one topology version.
#[derive(Debug, Default)]
pub struct CanvasCache {
    /// Topology version this cache was built for; `None` = never built.
    version: Option<u64>,

    /// Wire AABBs as (min, max), parallel to the schematic's wire list.
    pub wire_bounds: Vec<(Point, Point)>,

    /// First (wire id, vertex index) at each grid point, in wire order —
    /// matching the linear-scan semantics of `wire_vertex_at`.
    pub wire_vertices: HashMap<Point, (u64, usize)>,

    /// Junction marker positions.
    pub junctions: HashSet<Point>,

    /// Ambiguous interior/interior crossings between distinct wires. Endpoint
    /// and T contacts are already electrically connected and are not valid
    /// targets for the explicit-junction authoring tool.
    pub junction_candidates: Vec<Point>,
}

impl Clone for CanvasCache {
    /// Cloning a schematic must not copy derived caches.
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl CanvasCache {
    /// The cache, if it matches `version`; `None` means callers must fall
    /// back to scanning live data.
    pub fn fresh(&self, version: u64) -> Option<&Self> {
        (self.version == Some(version)).then_some(self)
    }

    fn rebuild(&mut self, wires: &[Wire], junctions: &[Junction], version: u64) {
        self.wire_bounds.clear();
        self.wire_bounds.reserve(wires.len());
        self.wire_vertices.clear();
        self.junctions.clear();
        self.junction_candidates.clear();

        for wire in wires {
            let mut min = Point::new(i32::MAX, i32::MAX);
            let mut max = Point::new(i32::MIN, i32::MIN);
            for (index, point) in wire.points.iter().enumerate() {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
                self.wire_vertices.entry(*point).or_insert((wire.id, index));
            }
            self.wire_bounds.push((min, max));
        }

        self.junctions.extend(junctions.iter().map(|j| j.pos));
        self.junction_candidates = collect_junction_candidates(wires);
        self.version = Some(version);
    }
}

impl SchematicState {
    /// Rebuild the canvas cache if the topology changed since it was last
    /// built. Call once per frame before painting or hover hit-tests.
    pub fn ensure_canvas_cache(&mut self) {
        let version = self.topology_version();
        if self.canvas_cache.version != Some(version) {
            // Split the borrow: take the cache out, rebuild, put it back.
            let mut cache = std::mem::take(&mut self.canvas_cache);
            cache.rebuild(&self.wires, &self.junctions, version);
            self.canvas_cache = cache;
        }
    }

    /// The canvas cache if it is current for this topology version.
    pub fn canvas_cache(&self) -> Option<&CanvasCache> {
        self.canvas_cache.fresh(self.topology_version())
    }

    /// Return the nearest valid explicit-junction target within `radius`.
    /// The frame cache serves the hot path; the fallback keeps the first
    /// interactive frame correct before derived geometry has been rebuilt.
    pub fn nearest_junction_candidate(&self, pos: Point, radius: i32) -> Option<Point> {
        let fallback;
        let candidates = if let Some(cache) = self.canvas_cache() {
            cache.junction_candidates.as_slice()
        } else {
            fallback = collect_junction_candidates(&self.wires);
            fallback.as_slice()
        };
        let radius_sq = i128::from(radius.max(0)).pow(2);
        candidates
            .iter()
            .copied()
            .filter_map(|candidate| {
                let dx = i128::from(candidate.x) - i128::from(pos.x);
                let dy = i128::from(candidate.y) - i128::from(pos.y);
                let distance_sq = dx * dx + dy * dy;
                (distance_sq <= radius_sq).then_some((distance_sq, candidate))
            })
            .min_by_key(|(distance_sq, candidate)| (*distance_sq, candidate.x, candidate.y))
            .map(|(_, candidate)| candidate)
    }
}

fn collect_junction_candidates(wires: &[Wire]) -> Vec<Point> {
    const CELL: i32 = 256;
    let segments: Vec<(u64, WireSegment)> = wires
        .iter()
        .flat_map(|wire| wire.segments().map(move |segment| (wire.id, segment)))
        .collect();
    let mut cells: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (index, (_, segment)) in segments.iter().enumerate() {
        for cell_x in segment.start.x.min(segment.end.x).div_euclid(CELL)
            ..=segment.start.x.max(segment.end.x).div_euclid(CELL)
        {
            for cell_y in segment.start.y.min(segment.end.y).div_euclid(CELL)
                ..=segment.start.y.max(segment.end.y).div_euclid(CELL)
            {
                cells.entry((cell_x, cell_y)).or_default().push(index);
            }
        }
    }

    let mut candidates = HashSet::new();
    let mut tested = HashSet::new();
    for bucket in cells.values() {
        for (offset, &left) in bucket.iter().enumerate() {
            for &right in &bucket[offset + 1..] {
                let pair = if left < right {
                    (left, right)
                } else {
                    (right, left)
                };
                if !tested.insert(pair) || segments[left].0 == segments[right].0 {
                    continue;
                }
                if let Some(point) = segments[left].1.intersection(&segments[right].1)
                    && point != segments[left].1.start
                    && point != segments[left].1.end
                    && point != segments[right].1.start
                    && point != segments[right].1.end
                {
                    candidates.insert(point);
                }
            }
        }
    }
    let mut candidates: Vec<_> = candidates.into_iter().collect();
    candidates.sort_by_key(|point| (point.x, point.y));
    candidates
}

#[cfg(test)]
mod tests {
    use super::super::state::SchematicState;
    use super::super::wire::Wire;
    use super::Point;

    /// The cached hit-test answers must match the linear-scan fallback,
    /// and topology bumps must invalidate.
    #[test]
    fn cache_matches_linear_scan_and_invalidates() {
        let mut state = SchematicState::default();
        state
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .wires
            .push(Wire::new(2, vec![Point::new(40, 0), Point::new(40, 40)]));
        state.bump_topology_version();

        // Fallback answers (cache not built yet).
        assert!(state.canvas_cache().is_none());
        assert_eq!(state.wire_vertex_at(Point::new(40, 0)), Some((1, 1)));
        assert!(state.is_draggable_wire_point(Point::new(40, 40)));

        // Cached answers are identical.
        state.ensure_canvas_cache();
        assert_eq!(state.wire_vertex_at(Point::new(40, 0)), Some((1, 1)));
        assert!(state.is_draggable_wire_point(Point::new(40, 40)));
        assert!(!state.is_draggable_wire_point(Point::new(99, 99)));
        let cache = state.canvas_cache().expect("cache fresh");
        assert_eq!(
            cache.wire_bounds[1],
            (Point::new(40, 0), Point::new(40, 40))
        );

        // A topology bump invalidates; rebuilding picks up the new bounds.
        state.wires[0].points[0] = Point::new(-20, 0);
        state.bump_topology_version();
        assert!(state.canvas_cache().is_none());
        state.ensure_canvas_cache();
        let cache = state.canvas_cache().expect("cache rebuilt");
        assert_eq!(
            cache.wire_bounds[0],
            (Point::new(-20, 0), Point::new(40, 0))
        );
    }

    #[test]
    fn junction_candidates_are_deduplicated_cached_and_nearest() {
        let mut state = SchematicState::default();
        state.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
            Wire::new(3, vec![Point::new(0, 0), Point::new(40, 40)]),
        ];
        state.bump_topology_version();

        assert_eq!(
            state.nearest_junction_candidate(Point::new(19, 21), 4),
            Some(Point::new(20, 20))
        );
        state.ensure_canvas_cache();
        let cache = state.canvas_cache().expect("cache fresh");
        assert_eq!(cache.junction_candidates, vec![Point::new(20, 20)]);
        assert_eq!(
            state.nearest_junction_candidate(Point::new(19, 21), 4),
            Some(Point::new(20, 20))
        );
        assert_eq!(
            state.nearest_junction_candidate(Point::new(100, 100), 4),
            None
        );
    }

    #[test]
    fn endpoint_and_t_contacts_are_not_explicit_junction_targets() {
        let mut state = SchematicState::default();
        state.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 20), Point::new(20, 40)]),
        ];
        state.bump_topology_version();

        assert_eq!(
            state.nearest_junction_candidate(Point::new(20, 20), 4),
            None
        );
        state.ensure_canvas_cache();
        assert!(
            state
                .canvas_cache()
                .expect("cache fresh")
                .junction_candidates
                .is_empty()
        );
    }
}
