//! Frame-Coherent Canvas Cache
//!
//! Derived geometry the schematic canvas needs every frame: wire bounding
//! boxes for viewport culling and a vertex/junction index for O(1) hover
//! hit-tests. Rebuilt lazily when `topology_version` advances; never
//! persisted, and deliberately reset (not copied) on clone — a clone
//! rebuilds its own on first use.

use std::collections::{HashMap, HashSet};

use rspice_design::schematic::net_label::Junction;
use rspice_design::schematic::wire::Wire;
use rspice_design_model::Point;

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

    pub(super) fn rebuild(&mut self, wires: &[Wire], junctions: &[Junction], version: u64) {
        self.wire_bounds.clear();
        self.wire_bounds.reserve(wires.len());
        self.wire_vertices.clear();
        self.junctions.clear();
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
