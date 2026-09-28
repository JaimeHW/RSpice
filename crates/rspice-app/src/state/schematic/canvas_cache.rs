//! Frame-Coherent Canvas Cache
//!
//! Derived geometry the schematic canvas needs every frame: wire bounding
//! boxes for viewport culling and a vertex/junction index for O(1) hover
//! hit-tests. Rebuilt lazily when `topology_version` advances; never
//! persisted, and deliberately reset (not copied) on clone — a clone
//! rebuilds its own on first use.

use std::collections::{HashMap, HashSet};

use super::junction_candidates::{collect_junction_candidates, nearest_junction_candidate};
use super::net_label::Junction;
use super::point::Point;
use super::state::SchematicState;
use super::wire::Wire;

const SPATIAL_CELL: i32 = 256;
const MAX_CELLS_PER_WIRE: i64 = 4_096;

/// Cached per-frame canvas geometry, valid for one topology version.
#[derive(Debug, Default)]
pub struct CanvasCache {
    /// Topology version this cache was built for; `None` = never built.
    version: Option<u64>,

    /// Wire AABBs as (min, max), parallel to the schematic's wire list.
    pub wire_bounds: Vec<(Point, Point)>,

    /// Spatial bins of wire-list indices used to avoid scanning an entire
    /// large design for viewport culling and point hit-testing.
    wire_cells: HashMap<(i32, i32), Vec<usize>>,

    /// Very long conductors are cheaper to test directly than to replicate
    /// into an unbounded number of spatial bins.
    oversized_wires: Vec<usize>,

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
        self.wire_cells.clear();
        self.oversized_wires.clear();

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
            let wire_index = self.wire_bounds.len() - 1;
            let min_cell_x = min.x.div_euclid(SPATIAL_CELL);
            let max_cell_x = max.x.div_euclid(SPATIAL_CELL);
            let min_cell_y = min.y.div_euclid(SPATIAL_CELL);
            let max_cell_y = max.y.div_euclid(SPATIAL_CELL);
            let cells_x = i64::from(max_cell_x)
                .saturating_sub(i64::from(min_cell_x))
                .saturating_add(1);
            let cells_y = i64::from(max_cell_y)
                .saturating_sub(i64::from(min_cell_y))
                .saturating_add(1);
            let cell_count = cells_x.saturating_mul(cells_y);
            if cell_count > MAX_CELLS_PER_WIRE {
                self.oversized_wires.push(wire_index);
            } else {
                for cell_x in min_cell_x..=max_cell_x {
                    for cell_y in min_cell_y..=max_cell_y {
                        self.wire_cells
                            .entry((cell_x, cell_y))
                            .or_default()
                            .push(wire_index);
                    }
                }
            }
        }

        self.junctions.extend(junctions.iter().map(|j| j.pos));
        self.junction_candidates = collect_junction_candidates(wires);
        self.version = Some(version);
    }

    /// Sorted wire-list indices whose cached bounds may intersect a world
    /// rectangle. Large/zoomed-out queries fall back to the complete ordered
    /// list instead of walking more empty cells than there are wires.
    pub fn wire_indices_in_world_rect(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<usize> {
        let to_cell = |value: f32| {
            if !value.is_finite() {
                0
            } else {
                (value.floor().clamp(i32::MIN as f32, i32::MAX as f32) as i32)
                    .div_euclid(SPATIAL_CELL)
            }
        };
        let min_cell_x = to_cell(x0.min(x1));
        let max_cell_x = to_cell(x0.max(x1));
        let min_cell_y = to_cell(y0.min(y1));
        let max_cell_y = to_cell(y0.max(y1));
        let cell_count = i64::from(max_cell_x.saturating_sub(min_cell_x).saturating_add(1))
            .saturating_mul(i64::from(
                max_cell_y.saturating_sub(min_cell_y).saturating_add(1),
            ));
        let scan_threshold = i64::try_from(self.wire_bounds.len())
            .unwrap_or(i64::MAX)
            .saturating_mul(4)
            .max(MAX_CELLS_PER_WIRE);
        if cell_count > scan_threshold {
            return (0..self.wire_bounds.len()).collect();
        }

        let mut indices = self.oversized_wires.clone();
        for cell_x in min_cell_x..=max_cell_x {
            for cell_y in min_cell_y..=max_cell_y {
                if let Some(bucket) = self.wire_cells.get(&(cell_x, cell_y)) {
                    indices.extend(bucket.iter().copied());
                }
            }
        }
        indices.sort_unstable();
        indices.dedup();
        indices
    }

    /// Sorted wire-list candidates for an exact authored grid point.
    pub fn wire_indices_at_point(&self, point: Point) -> Vec<usize> {
        let mut indices = self.oversized_wires.clone();
        if let Some(bucket) = self.wire_cells.get(&(
            point.x.div_euclid(SPATIAL_CELL),
            point.y.div_euclid(SPATIAL_CELL),
        )) {
            indices.extend(bucket.iter().copied());
        }
        indices.sort_unstable();
        indices.dedup();
        indices
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
            cache.rebuild(
                &self.design.document().wires,
                &self.design.document().junctions,
                version,
            );
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
            fallback = collect_junction_candidates(&self.design.document().wires);
            fallback.as_slice()
        };
        nearest_junction_candidate(candidates, pos, radius)
    }
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
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .design
            .document_mut_for_test()
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
        state.design.document_mut_for_test().wires[0].points[0] = Point::new(-20, 0);
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
        state.design.document_mut_for_test().wires = vec![
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
        state.design.document_mut_for_test().wires = vec![
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

    #[test]
    fn large_schematic_viewport_query_stays_spatial_and_keeps_long_wires() {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires = (0_u64..10_000)
            .map(|index| {
                let x = i32::try_from(index).expect("bounded fixture") * 512;
                Wire::new(index + 1, vec![Point::new(x, 0), Point::new(x, 40)])
            })
            .collect();
        state.design.document_mut_for_test().wires.push(Wire::new(
            20_000,
            vec![Point::new(-2_000_000, 20), Point::new(7_000_000, 20)],
        ));
        state.bump_topology_version();
        state.ensure_canvas_cache();

        let cache = state.canvas_cache().expect("cache fresh");
        let indices = cache.wire_indices_in_world_rect(5_100.0, -20.0, 5_250.0, 80.0);
        assert!(
            indices.contains(&10),
            "the nearby short wire is a candidate"
        );
        assert!(
            indices.contains(&10_000),
            "an oversized conductor remains a candidate without unbounded bin replication"
        );
        assert!(
            indices.len() <= 4,
            "a small viewport must not degrade to a 10,001-wire scan"
        );
    }
}
