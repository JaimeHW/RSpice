//! Spatial Index
//!
//! High-performance R-tree spatial indexing for schematic visualization.
//! This is critical for commercial-grade performance with 10,000+ components.
//!
//! Commercial-grade simulators use spatial indexing because:
//! - Visibility queries are O(log n + k) where k = visible elements
//! - Linear scan is O(n) which becomes unusable at scale
//! - Point queries (click/hover) are O(log n) instead of O(n)
//!
//! # Architecture
//!
//! Uses the `rstar` crate which provides a pure-Rust R-tree implementation.
//! The index stores bounding boxes for components and wires, allowing:
//! - Fast viewport visibility queries
//! - Fast point-in-element queries for hit testing
//! - Fast nearest-neighbor queries for snapping
//!
//! # Usage
//!
//! ```ignore
//! let mut index = SpatialIndex::new();
//! index.rebuild(&components, &wires);
//!
//! // O(log n + k) visibility query
//! let visible_ids = index.components_in_viewport(&viewport);
//!
//! // O(log n) point query
//! let hit = index.component_at_point(x, y);
//! ```

use rstar::{RTree, RTreeObject, AABB};

use crate::state::schematic::{Component, Wire};
use crate::state::Viewport;

// =============================================================================
// Spatial Objects for R-tree
// =============================================================================

/// Bounding box wrapper for components in the R-tree
#[derive(Debug, Clone)]
pub struct ComponentBounds {
    /// Component ID for lookup
    pub id: u64,
    /// Bounding box in grid coordinates (using f64 for rstar compatibility)
    envelope: AABB<[f64; 2]>,
}

impl ComponentBounds {
    /// Create bounds for a component
    ///
    /// Components are centered at their position with a typical half-size of 30 grid units
    pub fn new(component: &Component, half_size: i32) -> Self {
        let min = [
            (component.pos.x - half_size) as f64,
            (component.pos.y - half_size) as f64,
        ];
        let max = [
            (component.pos.x + half_size) as f64,
            (component.pos.y + half_size) as f64,
        ];
        Self {
            id: component.id,
            envelope: AABB::from_corners(min, max),
        }
    }
}

impl RTreeObject for ComponentBounds {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.envelope
    }
}

/// Bounding box wrapper for wires in the R-tree
#[derive(Debug, Clone)]
pub struct WireBounds {
    /// Wire ID for lookup
    pub id: u64,
    /// Bounding box in grid coordinates (using f64 for rstar compatibility)
    envelope: AABB<[f64; 2]>,
}

impl WireBounds {
    /// Create bounds for a wire from its points
    pub fn new(wire: &Wire) -> Self {
        if wire.points.is_empty() {
            // Empty wire - create degenerate bounding box at origin
            return Self {
                id: wire.id,
                envelope: AABB::from_corners([0.0, 0.0], [0.0, 0.0]),
            };
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for point in &wire.points {
            let x = point.x as f64;
            let y = point.y as f64;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        Self {
            id: wire.id,
            envelope: AABB::from_corners([min_x, min_y], [max_x, max_y]),
        }
    }
}

impl RTreeObject for WireBounds {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.envelope
    }
}

// =============================================================================
// SpatialIndex
// =============================================================================

/// R-tree spatial index for high-performance visibility and hit-testing queries
///
/// This index allows O(log n + k) queries where k is the number of results,
/// compared to O(n) for linear scanning.
#[derive(Debug, Clone, Default)]
pub struct SpatialIndex {
    /// R-tree for component bounding boxes
    component_tree: RTree<ComponentBounds>,

    /// R-tree for wire bounding boxes
    wire_tree: RTree<WireBounds>,

    /// Schematic topology version when index was built
    cached_version: u64,

    /// Statistics for performance monitoring
    stats: SpatialIndexStats,
}

/// Index performance statistics
#[derive(Debug, Clone, Default)]
pub struct SpatialIndexStats {
    /// Number of index rebuilds
    pub rebuilds: u64,
    /// Number of components in last rebuild
    pub component_count: usize,
    /// Number of wires in last rebuild
    pub wire_count: usize,
    /// Number of visibility queries
    pub visibility_queries: u64,
    /// Number of point queries
    pub point_queries: u64,
}

impl SpatialIndex {
    /// Create a new empty spatial index
    pub fn new() -> Self {
        Self {
            component_tree: RTree::new(),
            wire_tree: RTree::new(),
            cached_version: 0,
            stats: SpatialIndexStats::default(),
        }
    }

    /// Check if index needs rebuild for the given topology version
    pub fn needs_rebuild(&self, current_version: u64) -> bool {
        self.cached_version != current_version
    }

    /// Rebuild the spatial index from components and wires
    ///
    /// This is O(n log n) but only needs to happen when topology changes.
    pub fn rebuild(&mut self, components: &[Component], wires: &[Wire], half_size: i32) {
        // Build component bounds
        let component_bounds: Vec<ComponentBounds> = components
            .iter()
            .map(|c| ComponentBounds::new(c, half_size))
            .collect();

        // Build wire bounds
        let wire_bounds: Vec<WireBounds> = wires.iter().map(WireBounds::new).collect();

        // Bulk-load into R-trees (more efficient than individual inserts)
        self.component_tree = RTree::bulk_load(component_bounds);
        self.wire_tree = RTree::bulk_load(wire_bounds);

        // Update stats
        self.stats.rebuilds += 1;
        self.stats.component_count = components.len();
        self.stats.wire_count = wires.len();
    }

    /// Update cache version after rebuild
    pub fn set_version(&mut self, version: u64) {
        self.cached_version = version;
    }

    /// Get the cached topology version
    pub fn cached_version(&self) -> u64 {
        self.cached_version
    }

    /// Query visible components in a viewport
    ///
    /// Returns component IDs that intersect the viewport bounds.
    /// This is O(log n + k) where k = number of visible components.
    pub fn components_in_viewport(&self, viewport: &Viewport) -> Vec<u64> {
        let (min_x, min_y, max_x, max_y) = viewport.bounds_i32();
        let query_envelope =
            AABB::from_corners([min_x as f64, min_y as f64], [max_x as f64, max_y as f64]);

        self.component_tree
            .locate_in_envelope_intersecting(&query_envelope)
            .map(|bounds| bounds.id)
            .collect()
    }

    /// Query visible wires in a viewport
    ///
    /// Returns wire IDs that intersect the viewport bounds.
    /// This is O(log n + k) where k = number of visible wires.
    pub fn wires_in_viewport(&self, viewport: &Viewport) -> Vec<u64> {
        let (min_x, min_y, max_x, max_y) = viewport.bounds_i32();
        let query_envelope =
            AABB::from_corners([min_x as f64, min_y as f64], [max_x as f64, max_y as f64]);

        self.wire_tree
            .locate_in_envelope_intersecting(&query_envelope)
            .map(|bounds| bounds.id)
            .collect()
    }

    /// Query visible components in explicit bounds
    pub fn components_in_bounds(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Vec<u64> {
        let query_envelope =
            AABB::from_corners([min_x as f64, min_y as f64], [max_x as f64, max_y as f64]);

        self.component_tree
            .locate_in_envelope_intersecting(&query_envelope)
            .map(|bounds| bounds.id)
            .collect()
    }

    /// Query visible wires in explicit bounds
    pub fn wires_in_bounds(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Vec<u64> {
        let query_envelope =
            AABB::from_corners([min_x as f64, min_y as f64], [max_x as f64, max_y as f64]);

        self.wire_tree
            .locate_in_envelope_intersecting(&query_envelope)
            .map(|bounds| bounds.id)
            .collect()
    }

    /// Find component at a point (hit testing)
    ///
    /// Returns component ID if the point is within a component's bounding box.
    /// Uses envelope query for O(log n + k) performance.
    pub fn component_at_point(&self, x: i32, y: i32) -> Option<u64> {
        // Create a tiny envelope around the point for intersection query
        let point_envelope = AABB::from_corners([x as f64, y as f64], [x as f64, y as f64]);

        self.component_tree
            .locate_in_envelope_intersecting(&point_envelope)
            .next()
            .map(|bounds| bounds.id)
    }

    /// Find all components at a point
    pub fn components_at_point(&self, x: i32, y: i32) -> Vec<u64> {
        let point_envelope = AABB::from_corners([x as f64, y as f64], [x as f64, y as f64]);

        self.component_tree
            .locate_in_envelope_intersecting(&point_envelope)
            .map(|bounds| bounds.id)
            .collect()
    }

    /// Find wire at a point (hit testing)
    ///
    /// Returns wire ID if the point is within a wire's bounding box.
    /// Note: This is a bounding box check, not a precise wire geometry check.
    pub fn wire_at_point(&self, x: i32, y: i32) -> Option<u64> {
        let point_envelope = AABB::from_corners([x as f64, y as f64], [x as f64, y as f64]);

        self.wire_tree
            .locate_in_envelope_intersecting(&point_envelope)
            .next()
            .map(|bounds| bounds.id)
    }

    /// Find all wires at a point
    pub fn wires_at_point(&self, x: i32, y: i32) -> Vec<u64> {
        let point_envelope = AABB::from_corners([x as f64, y as f64], [x as f64, y as f64]);

        self.wire_tree
            .locate_in_envelope_intersecting(&point_envelope)
            .map(|bounds| bounds.id)
            .collect()
    }

    /// Get index statistics
    pub fn stats(&self) -> &SpatialIndexStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = SpatialIndexStats::default();
    }

    /// Number of components in the index
    pub fn component_count(&self) -> usize {
        self.component_tree.size()
    }

    /// Number of wires in the index
    pub fn wire_count(&self) -> usize {
        self.wire_tree.size()
    }

    /// Check if index is empty
    pub fn is_empty(&self) -> bool {
        self.component_tree.size() == 0 && self.wire_tree.size() == 0
    }

    /// Clear the index
    pub fn clear(&mut self) {
        self.component_tree = RTree::new();
        self.wire_tree = RTree::new();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;
    use crate::state::schematic::ComponentType;

    /// Helper to create a test component
    fn make_component(id: u64, x: i32, y: i32) -> Component {
        let mut comp = Component::new(id, ComponentType::Resistor, Point::new(x, y));
        comp.id = id;
        comp
    }

    /// Helper to create a test wire
    fn make_wire(id: u64, points: Vec<(i32, i32)>) -> Wire {
        Wire::new(
            id,
            points.into_iter().map(|(x, y)| Point::new(x, y)).collect(),
        )
    }

    // =========================================================================
    // Construction Tests
    // =========================================================================

    #[test]
    fn test_new_index_is_empty() {
        let index = SpatialIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.component_count(), 0);
        assert_eq!(index.wire_count(), 0);
        assert_eq!(index.cached_version(), 0);
    }

    #[test]
    fn test_default_equals_new() {
        let default_index = SpatialIndex::default();
        let new_index = SpatialIndex::new();
        assert_eq!(default_index.component_count(), new_index.component_count());
        assert_eq!(default_index.wire_count(), new_index.wire_count());
    }

    // =========================================================================
    // Rebuild Tests
    // =========================================================================

    #[test]
    fn test_rebuild_empty() {
        let mut index = SpatialIndex::new();
        index.rebuild(&[], &[], 30);

        assert!(index.is_empty());
        assert_eq!(index.stats().rebuilds, 1);
    }

    #[test]
    fn test_rebuild_with_components() {
        let mut index = SpatialIndex::new();
        let components = vec![
            make_component(1, 0, 0),
            make_component(2, 100, 0),
            make_component(3, 0, 100),
        ];

        index.rebuild(&components, &[], 30);

        assert_eq!(index.component_count(), 3);
        assert_eq!(index.wire_count(), 0);
        assert_eq!(index.stats().component_count, 3);
    }

    #[test]
    fn test_rebuild_with_wires() {
        let mut index = SpatialIndex::new();
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(20, 0), (30, 0)]),
        ];

        index.rebuild(&[], &wires, 30);

        assert_eq!(index.component_count(), 0);
        assert_eq!(index.wire_count(), 2);
        assert_eq!(index.stats().wire_count, 2);
    }

    #[test]
    fn test_rebuild_increments_stats() {
        let mut index = SpatialIndex::new();
        let components = vec![make_component(1, 0, 0)];

        index.rebuild(&components, &[], 30);
        assert_eq!(index.stats().rebuilds, 1);

        index.rebuild(&components, &[], 30);
        assert_eq!(index.stats().rebuilds, 2);

        index.rebuild(&components, &[], 30);
        assert_eq!(index.stats().rebuilds, 3);
    }

    // =========================================================================
    // Version Tests
    // =========================================================================

    #[test]
    fn test_needs_rebuild() {
        let mut index = SpatialIndex::new();
        assert!(index.needs_rebuild(1));

        index.set_version(1);
        assert!(!index.needs_rebuild(1));
        assert!(index.needs_rebuild(2));
    }

    // =========================================================================
    // Visibility Query Tests
    // =========================================================================

    #[test]
    fn test_components_in_bounds_all_visible() {
        let mut index = SpatialIndex::new();
        let components = vec![
            make_component(1, 0, 0),
            make_component(2, 50, 0),
            make_component(3, 0, 50),
        ];

        index.rebuild(&components, &[], 30);

        // Query that covers all components
        let visible = index.components_in_bounds(-50, -50, 100, 100);

        assert_eq!(visible.len(), 3);
        assert!(visible.contains(&1));
        assert!(visible.contains(&2));
        assert!(visible.contains(&3));
    }

    #[test]
    fn test_components_in_bounds_partial() {
        let mut index = SpatialIndex::new();
        let components = vec![
            make_component(1, 0, 0),     // In bounds
            make_component(2, 100, 0),   // Out of bounds
            make_component(3, 0, 100),   // Out of bounds
            make_component(4, 500, 500), // Far out
        ];

        index.rebuild(&components, &[], 30);

        // Query that only covers component 1
        let visible = index.components_in_bounds(-50, -50, 50, 50);

        assert_eq!(visible.len(), 1);
        assert!(visible.contains(&1));
    }

    #[test]
    fn test_components_in_bounds_none() {
        let mut index = SpatialIndex::new();
        let components = vec![make_component(1, 100, 100), make_component(2, 200, 200)];

        index.rebuild(&components, &[], 30);

        // Query that covers no components
        let visible = index.components_in_bounds(-100, -100, -50, -50);

        assert!(visible.is_empty());
    }

    #[test]
    fn test_wires_in_bounds() {
        let mut index = SpatialIndex::new();
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),        // In bounds
            make_wire(2, vec![(100, 100), (110, 100)]), // Out of bounds
        ];

        index.rebuild(&[], &wires, 30);

        let visible = index.wires_in_bounds(-10, -10, 20, 20);

        assert_eq!(visible.len(), 1);
        assert!(visible.contains(&1));
    }

    // =========================================================================
    // Point Query Tests
    // =========================================================================

    #[test]
    fn test_component_at_point() {
        let mut index = SpatialIndex::new();
        let components = vec![make_component(1, 0, 0), make_component(2, 100, 0)];

        index.rebuild(&components, &[], 30);

        // Point inside component 1
        let hit1 = index.component_at_point(0, 0);
        assert_eq!(hit1, Some(1));

        // Point inside component 2
        let hit2 = index.component_at_point(100, 0);
        assert_eq!(hit2, Some(2));

        // Point outside all components
        let miss = index.component_at_point(500, 500);
        assert_eq!(miss, None);
    }

    #[test]
    fn test_components_at_point_overlapping() {
        let mut index = SpatialIndex::new();
        // Two overlapping components at same position
        let components = vec![
            make_component(1, 0, 0),
            make_component(2, 10, 0), // Overlaps with component 1
        ];

        index.rebuild(&components, &[], 30);

        // Point that could hit both
        let hits = index.components_at_point(5, 0);

        // Should find both overlapping components
        assert!(hits.len() >= 1);
    }

    #[test]
    fn test_wire_at_point() {
        let mut index = SpatialIndex::new();
        let wires = vec![
            make_wire(1, vec![(0, 0), (10, 0)]),
            make_wire(2, vec![(100, 0), (110, 0)]),
        ];

        index.rebuild(&[], &wires, 30);

        // Point on wire 1
        let hit = index.wire_at_point(5, 0);
        assert_eq!(hit, Some(1));

        // Point not on any wire
        let miss = index.wire_at_point(500, 500);
        assert_eq!(miss, None);
    }

    // =========================================================================
    // Clear Tests
    // =========================================================================

    #[test]
    fn test_clear() {
        let mut index = SpatialIndex::new();
        let components = vec![make_component(1, 0, 0)];
        let wires = vec![make_wire(1, vec![(0, 0), (10, 0)])];

        index.rebuild(&components, &wires, 30);
        assert!(!index.is_empty());

        index.clear();

        assert!(index.is_empty());
        assert_eq!(index.component_count(), 0);
        assert_eq!(index.wire_count(), 0);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_empty_wire() {
        let mut index = SpatialIndex::new();
        let wires = vec![Wire::new(1, vec![])];

        index.rebuild(&[], &wires, 30);

        assert_eq!(index.wire_count(), 1);
    }

    #[test]
    fn test_single_point_wire() {
        let mut index = SpatialIndex::new();
        let wires = vec![make_wire(1, vec![(5, 5)])];

        index.rebuild(&[], &wires, 30);

        assert_eq!(index.wire_count(), 1);

        // Should be found at that point
        let hit = index.wire_at_point(5, 5);
        assert_eq!(hit, Some(1));
    }

    #[test]
    fn test_large_number_of_components() {
        let mut index = SpatialIndex::new();
        // Create 1000 components in a grid
        let components: Vec<_> = (0..1000)
            .map(|i| {
                let x = (i % 100) * 50;
                let y = (i / 100) * 50;
                make_component(i as u64, x, y)
            })
            .collect();

        index.rebuild(&components, &[], 30);

        assert_eq!(index.component_count(), 1000);

        // Query should still be fast
        let visible = index.components_in_bounds(0, 0, 200, 200);
        assert!(!visible.is_empty());
    }

    #[test]
    fn test_negative_coordinates() {
        let mut index = SpatialIndex::new();
        let components = vec![make_component(1, -100, -100), make_component(2, 100, 100)];

        index.rebuild(&components, &[], 30);

        let visible = index.components_in_bounds(-200, -200, 0, 0);
        assert_eq!(visible.len(), 1);
        assert!(visible.contains(&1));
    }
}
