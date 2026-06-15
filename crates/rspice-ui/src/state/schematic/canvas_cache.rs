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
use super::wire::Wire;

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
}
