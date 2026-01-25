//! Frustum Culling System
//!
//! Commercial-grade view frustum culling for GPU schematic rendering.
//! Follows professional EDA patterns for efficient culling at any scale.
//!
//! # Architecture
//!
//! Uses axis-aligned bounding boxes (AABB) for fast intersection tests.
//! Components outside the view frustum are skipped during rendering.
//!
//! # Performance
//!
//! - O(1) per-component visibility test
//! - Spatial hashing for large schematics (future enhancement)
//! - Hierarchical culling for component groups (future enhancement)

use crate::state::Point;

// =============================================================================
// AABB (Axis-Aligned Bounding Box)
// =============================================================================

/// Axis-aligned bounding box for culling
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    /// Minimum corner (bottom-left)
    pub min_x: f32,
    pub min_y: f32,
    /// Maximum corner (top-right)
    pub max_x: f32,
    pub max_y: f32,
}

impl AABB {
    /// Create a new AABB from min/max corners
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x: min_x.min(max_x),
            min_y: min_y.min(max_y),
            max_x: min_x.max(max_x),
            max_y: min_y.max(max_y),
        }
    }

    /// Create AABB from center and half-extents
    pub fn from_center(cx: f32, cy: f32, half_width: f32, half_height: f32) -> Self {
        Self {
            min_x: cx - half_width,
            min_y: cy - half_height,
            max_x: cx + half_width,
            max_y: cy + half_height,
        }
    }

    /// Create AABB from position and size
    pub fn from_pos_size(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::new(x, y, x + width, y + height)
    }

    /// Create AABB around a point with radius
    pub fn from_point(x: f32, y: f32, radius: f32) -> Self {
        Self::from_center(x, y, radius, radius)
    }

    /// Width of the AABB
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    /// Height of the AABB
    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    /// Center X coordinate
    pub fn center_x(&self) -> f32 {
        (self.min_x + self.max_x) / 2.0
    }

    /// Center Y coordinate
    pub fn center_y(&self) -> f32 {
        (self.min_y + self.max_y) / 2.0
    }

    /// Area of the AABB
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    /// Check if this AABB contains a point
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Check if this AABB intersects another
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }

    /// Check if this AABB fully contains another
    pub fn contains(&self, other: &AABB) -> bool {
        other.min_x >= self.min_x
            && other.max_x <= self.max_x
            && other.min_y >= self.min_y
            && other.max_y <= self.max_y
    }

    /// Expand AABB by margin on all sides
    pub fn expand(&self, margin: f32) -> Self {
        Self {
            min_x: self.min_x - margin,
            min_y: self.min_y - margin,
            max_x: self.max_x + margin,
            max_y: self.max_y + margin,
        }
    }

    /// Merge with another AABB to create bounding AABB
    pub fn merge(&self, other: &AABB) -> Self {
        Self {
            min_x: self.min_x.min(other.min_x),
            min_y: self.min_y.min(other.min_y),
            max_x: self.max_x.max(other.max_x),
            max_y: self.max_y.max(other.max_y),
        }
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

// =============================================================================
// View Frustum
// =============================================================================

/// View frustum representing the visible area of the canvas
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewFrustum {
    /// Visible area in world coordinates
    pub view_bounds: AABB,
    /// Current zoom level
    pub zoom: f32,
    /// Pan offset
    pub pan_x: f32,
    pub pan_y: f32,
}

impl ViewFrustum {
    /// Create a view frustum from viewport parameters
    pub fn new(
        viewport_width: f32,
        viewport_height: f32,
        pan_x: f32,
        pan_y: f32,
        zoom: f32,
    ) -> Self {
        // Calculate world-space bounds of the viewport
        let world_width = viewport_width / zoom;
        let world_height = viewport_height / zoom;

        // Pan offsets are in screen space, convert to world space
        let world_x = -pan_x / zoom;
        let world_y = -pan_y / zoom;

        let view_bounds = AABB::new(
            world_x,
            world_y,
            world_x + world_width,
            world_y + world_height,
        );

        Self {
            view_bounds,
            zoom,
            pan_x,
            pan_y,
        }
    }

    /// Create from camera parameters
    pub fn from_camera(width: f32, height: f32, pan: (f64, f64), zoom: f64) -> Self {
        Self::new(width, height, pan.0 as f32, pan.1 as f32, zoom as f32)
    }

    /// Check if a point is visible
    pub fn is_point_visible(&self, x: f32, y: f32) -> bool {
        self.view_bounds.contains_point(x, y)
    }

    /// Check if an AABB intersects the view (is potentially visible)
    pub fn is_aabb_visible(&self, aabb: &AABB) -> bool {
        self.view_bounds.intersects(aabb)
    }

    /// Get expanded view bounds for culling margin
    pub fn expanded_bounds(&self, margin: f32) -> AABB {
        self.view_bounds.expand(margin)
    }
}

impl Default for ViewFrustum {
    fn default() -> Self {
        Self::new(800.0, 600.0, 0.0, 0.0, 1.0)
    }
}

// =============================================================================
// Frustum Culler
// =============================================================================

/// Frustum culler for filtering visible objects
#[derive(Debug, Clone)]
pub struct FrustumCuller {
    /// Current view frustum
    pub frustum: ViewFrustum,
    /// Margin for conservative culling (prevents popping)
    pub margin: f32,
    /// Component bounding size estimate
    pub component_half_size: f32,
    /// Minimum visible size in pixels for LOD culling
    pub min_visible_size: f32,
}

impl FrustumCuller {
    /// Create a new frustum culler
    pub fn new(frustum: ViewFrustum) -> Self {
        Self {
            frustum,
            margin: 50.0, // Conservative margin
            component_half_size: 40.0, // Typical component half-size
            min_visible_size: 4.0, // Below this pixel size, cull
        }
    }

    /// Update the view frustum
    pub fn update_frustum(&mut self, frustum: ViewFrustum) {
        self.frustum = frustum;
    }

    /// Update from viewport parameters
    pub fn update(
        &mut self,
        viewport_width: f32,
        viewport_height: f32,
        pan_x: f32,
        pan_y: f32,
        zoom: f32,
    ) {
        self.frustum = ViewFrustum::new(viewport_width, viewport_height, pan_x, pan_y, zoom);
    }

    /// Get expanded culling bounds
    pub fn culling_bounds(&self) -> AABB {
        self.frustum.expanded_bounds(self.margin)
    }

    /// Check if a component at position is visible
    pub fn is_component_visible(&self, x: f32, y: f32) -> bool {
        let bounds = self.culling_bounds();
        let component_aabb = AABB::from_center(x, y, self.component_half_size, self.component_half_size);
        bounds.intersects(&component_aabb)
    }

    /// Check if a component is visible with custom size
    pub fn is_component_visible_sized(&self, x: f32, y: f32, half_size: f32) -> bool {
        let bounds = self.culling_bounds();
        let component_aabb = AABB::from_center(x, y, half_size, half_size);
        bounds.intersects(&component_aabb)
    }

    /// Check if a point (junction) is visible
    pub fn is_point_visible(&self, x: f32, y: f32) -> bool {
        let bounds = self.culling_bounds();
        bounds.contains_point(x, y)
    }

    /// Check if a wire segment is visible
    pub fn is_wire_visible(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
        let wire_aabb = AABB::new(x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));
        let bounds = self.culling_bounds();
        bounds.intersects(&wire_aabb)
    }

    /// Check if an AABB is visible
    pub fn is_aabb_visible(&self, aabb: &AABB) -> bool {
        let bounds = self.culling_bounds();
        bounds.intersects(aabb)
    }

    /// Check if a size is visible at current zoom (LOD culling)
    pub fn is_size_visible(&self, world_size: f32) -> bool {
        let screen_size = world_size * self.frustum.zoom;
        screen_size >= self.min_visible_size
    }

    /// Filter component indices by visibility
    pub fn filter_visible_components<'a>(
        &self,
        components: impl Iterator<Item = (usize, f32, f32)> + 'a,
    ) -> impl Iterator<Item = usize> + 'a {
        let bounds = self.culling_bounds();
        let half_size = self.component_half_size;
        components.filter_map(move |(idx, x, y)| {
            let component_aabb = AABB::from_center(x, y, half_size, half_size);
            if bounds.intersects(&component_aabb) {
                Some(idx)
            } else {
                None
            }
        })
    }

    /// Get culling statistics
    pub fn stats(&self, total: usize, visible: usize) -> CullStats {
        CullStats {
            total,
            visible,
            culled: total.saturating_sub(visible),
            cull_ratio: if total > 0 {
                (total.saturating_sub(visible)) as f32 / total as f32
            } else {
                0.0
            },
        }
    }
}

impl Default for FrustumCuller {
    fn default() -> Self {
        Self::new(ViewFrustum::default())
    }
}

// =============================================================================
// Culling Statistics
// =============================================================================

/// Statistics about frustum culling performance
#[derive(Debug, Clone, Copy, Default)]
pub struct CullStats {
    /// Total objects considered
    pub total: usize,
    /// Visible objects after culling
    pub visible: usize,
    /// Culled (invisible) objects
    pub culled: usize,
    /// Ratio of culled to total (0.0 - 1.0)
    pub cull_ratio: f32,
}

impl CullStats {
    /// Check if any culling occurred
    pub fn has_culling(&self) -> bool {
        self.culled > 0
    }

    /// Get percentage of objects culled
    pub fn cull_percentage(&self) -> f32 {
        self.cull_ratio * 100.0
    }
}

// =============================================================================
// Batch Culling
// =============================================================================

/// Result of batch culling operation
#[derive(Debug, Clone)]
pub struct CullResult {
    /// Indices of visible components
    pub visible_components: Vec<usize>,
    /// Indices of visible wires
    pub visible_wires: Vec<usize>,
    /// Culling statistics for components
    pub component_stats: CullStats,
    /// Culling statistics for wires
    pub wire_stats: CullStats,
}

impl Default for CullResult {
    fn default() -> Self {
        Self {
            visible_components: Vec::new(),
            visible_wires: Vec::new(),
            component_stats: CullStats::default(),
            wire_stats: CullStats::default(),
        }
    }
}

/// Batch cull components and wires
pub fn batch_cull(
    culler: &FrustumCuller,
    component_positions: &[(f32, f32)],
    wire_segments: &[(f32, f32, f32, f32)],
) -> CullResult {
    let mut result = CullResult::default();

    // Cull components
    for (idx, &(x, y)) in component_positions.iter().enumerate() {
        if culler.is_component_visible(x, y) {
            result.visible_components.push(idx);
        }
    }
    result.component_stats = culler.stats(component_positions.len(), result.visible_components.len());

    // Cull wires
    for (idx, &(x1, y1, x2, y2)) in wire_segments.iter().enumerate() {
        if culler.is_wire_visible(x1, y1, x2, y2) {
            result.visible_wires.push(idx);
        }
    }
    result.wire_stats = culler.stats(wire_segments.len(), result.visible_wires.len());

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AABB Tests
    // =========================================================================

    #[test]
    fn test_aabb_new() {
        let aabb = AABB::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(aabb.min_x, 0.0);
        assert_eq!(aabb.max_x, 10.0);
    }

    #[test]
    fn test_aabb_new_swapped() {
        // Should auto-correct swapped min/max
        let aabb = AABB::new(10.0, 10.0, 0.0, 0.0);
        assert_eq!(aabb.min_x, 0.0);
        assert_eq!(aabb.max_x, 10.0);
    }

    #[test]
    fn test_aabb_from_center() {
        let aabb = AABB::from_center(50.0, 50.0, 10.0, 10.0);
        assert_eq!(aabb.min_x, 40.0);
        assert_eq!(aabb.max_x, 60.0);
    }

    #[test]
    fn test_aabb_from_pos_size() {
        let aabb = AABB::from_pos_size(10.0, 20.0, 100.0, 50.0);
        assert_eq!(aabb.min_x, 10.0);
        assert_eq!(aabb.max_x, 110.0);
        assert_eq!(aabb.max_y, 70.0);
    }

    #[test]
    fn test_aabb_from_point() {
        let aabb = AABB::from_point(100.0, 100.0, 5.0);
        assert_eq!(aabb.min_x, 95.0);
        assert_eq!(aabb.max_x, 105.0);
    }

    #[test]
    fn test_aabb_dimensions() {
        let aabb = AABB::new(10.0, 20.0, 50.0, 80.0);
        assert_eq!(aabb.width(), 40.0);
        assert_eq!(aabb.height(), 60.0);
    }

    #[test]
    fn test_aabb_center() {
        let aabb = AABB::new(0.0, 0.0, 100.0, 100.0);
        assert_eq!(aabb.center_x(), 50.0);
        assert_eq!(aabb.center_y(), 50.0);
    }

    #[test]
    fn test_aabb_area() {
        let aabb = AABB::new(0.0, 0.0, 10.0, 20.0);
        assert_eq!(aabb.area(), 200.0);
    }

    #[test]
    fn test_aabb_contains_point_inside() {
        let aabb = AABB::new(0.0, 0.0, 100.0, 100.0);
        assert!(aabb.contains_point(50.0, 50.0));
        assert!(aabb.contains_point(0.0, 0.0));
        assert!(aabb.contains_point(100.0, 100.0));
    }

    #[test]
    fn test_aabb_contains_point_outside() {
        let aabb = AABB::new(0.0, 0.0, 100.0, 100.0);
        assert!(!aabb.contains_point(-1.0, 50.0));
        assert!(!aabb.contains_point(101.0, 50.0));
        assert!(!aabb.contains_point(50.0, -1.0));
        assert!(!aabb.contains_point(50.0, 101.0));
    }

    #[test]
    fn test_aabb_intersects_overlapping() {
        let a = AABB::new(0.0, 0.0, 50.0, 50.0);
        let b = AABB::new(25.0, 25.0, 75.0, 75.0);
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn test_aabb_intersects_touching() {
        let a = AABB::new(0.0, 0.0, 50.0, 50.0);
        let b = AABB::new(50.0, 0.0, 100.0, 50.0);
        assert!(a.intersects(&b));
    }

    #[test]
    fn test_aabb_intersects_separate() {
        let a = AABB::new(0.0, 0.0, 50.0, 50.0);
        let b = AABB::new(100.0, 100.0, 150.0, 150.0);
        assert!(!a.intersects(&b));
    }

    #[test]
    fn test_aabb_contains_other() {
        let outer = AABB::new(0.0, 0.0, 100.0, 100.0);
        let inner = AABB::new(25.0, 25.0, 75.0, 75.0);
        assert!(outer.contains(&inner));
        assert!(!inner.contains(&outer));
    }

    #[test]
    fn test_aabb_expand() {
        let aabb = AABB::new(10.0, 10.0, 20.0, 20.0);
        let expanded = aabb.expand(5.0);
        assert_eq!(expanded.min_x, 5.0);
        assert_eq!(expanded.max_x, 25.0);
    }

    #[test]
    fn test_aabb_merge() {
        let a = AABB::new(0.0, 0.0, 50.0, 50.0);
        let b = AABB::new(40.0, 40.0, 100.0, 100.0);
        let merged = a.merge(&b);
        assert_eq!(merged.min_x, 0.0);
        assert_eq!(merged.max_x, 100.0);
    }

    // =========================================================================
    // ViewFrustum Tests
    // =========================================================================

    #[test]
    fn test_view_frustum_new() {
        let frustum = ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 1.0);
        assert_eq!(frustum.view_bounds.width(), 800.0);
        assert_eq!(frustum.view_bounds.height(), 600.0);
    }

    #[test]
    fn test_view_frustum_with_zoom() {
        let frustum = ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 2.0);
        assert_eq!(frustum.view_bounds.width(), 400.0); // Zoomed in = smaller world area
        assert_eq!(frustum.view_bounds.height(), 300.0);
    }

    #[test]
    fn test_view_frustum_with_pan() {
        let frustum = ViewFrustum::new(800.0, 600.0, 100.0, 50.0, 1.0);
        assert_eq!(frustum.view_bounds.min_x, -100.0);
        assert_eq!(frustum.view_bounds.min_y, -50.0);
    }

    #[test]
    fn test_view_frustum_from_camera() {
        let frustum = ViewFrustum::from_camera(800.0, 600.0, (100.0, 50.0), 2.0);
        assert_eq!(frustum.zoom, 2.0);
    }

    #[test]
    fn test_view_frustum_point_visible() {
        let frustum = ViewFrustum::new(100.0, 100.0, 0.0, 0.0, 1.0);
        assert!(frustum.is_point_visible(50.0, 50.0));
        assert!(!frustum.is_point_visible(150.0, 50.0));
    }

    #[test]
    fn test_view_frustum_aabb_visible() {
        let frustum = ViewFrustum::new(100.0, 100.0, 0.0, 0.0, 1.0);
        let inside = AABB::new(25.0, 25.0, 75.0, 75.0);
        let outside = AABB::new(200.0, 200.0, 250.0, 250.0);
        assert!(frustum.is_aabb_visible(&inside));
        assert!(!frustum.is_aabb_visible(&outside));
    }

    // =========================================================================
    // FrustumCuller Tests
    // =========================================================================

    #[test]
    fn test_frustum_culler_new() {
        let culler = FrustumCuller::default();
        assert!(culler.margin > 0.0);
    }

    #[test]
    fn test_frustum_culler_update() {
        let mut culler = FrustumCuller::default();
        culler.update(1920.0, 1080.0, 0.0, 0.0, 1.0);
        assert_eq!(culler.frustum.view_bounds.width(), 1920.0);
    }

    #[test]
    fn test_frustum_culler_component_visible() {
        let culler = FrustumCuller::new(ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 1.0));
        assert!(culler.is_component_visible(100.0, 100.0));
        assert!(!culler.is_component_visible(2000.0, 2000.0));
    }

    #[test]
    fn test_frustum_culler_component_at_edge() {
        let culler = FrustumCuller::new(ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 1.0));
        // Component just inside margin
        assert!(culler.is_component_visible(820.0, 300.0)); // Within margin
    }

    #[test]
    fn test_frustum_culler_point_visible() {
        let culler = FrustumCuller::new(ViewFrustum::new(100.0, 100.0, 0.0, 0.0, 1.0));
        assert!(culler.is_point_visible(50.0, 50.0));
        assert!(!culler.is_point_visible(500.0, 500.0));
    }

    #[test]
    fn test_frustum_culler_wire_visible() {
        let culler = FrustumCuller::new(ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 1.0));
        // Wire fully inside
        assert!(culler.is_wire_visible(100.0, 100.0, 200.0, 100.0));
        // Wire crossing edge
        assert!(culler.is_wire_visible(750.0, 300.0, 900.0, 300.0));
        // Wire fully outside
        assert!(!culler.is_wire_visible(2000.0, 2000.0, 2100.0, 2000.0));
    }

    #[test]
    fn test_frustum_culler_size_visible() {
        let culler = FrustumCuller::new(ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 1.0));
        assert!(culler.is_size_visible(10.0)); // 10 pixels
        assert!(!culler.is_size_visible(1.0)); // 1 pixel (below min)
    }

    #[test]
    fn test_frustum_culler_size_visible_zoomed() {
        let mut culler = FrustumCuller::new(ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 0.1));
        culler.min_visible_size = 4.0;
        // At 0.1 zoom, 10 world units = 1 pixel (not visible)
        assert!(!culler.is_size_visible(10.0));
        // At 0.1 zoom, 100 world units = 10 pixels (visible)
        assert!(culler.is_size_visible(100.0));
    }

    // =========================================================================
    // CullStats Tests
    // =========================================================================

    #[test]
    fn test_cull_stats() {
        let culler = FrustumCuller::default();
        let stats = culler.stats(100, 75);
        assert_eq!(stats.total, 100);
        assert_eq!(stats.visible, 75);
        assert_eq!(stats.culled, 25);
        assert!((stats.cull_ratio - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_cull_stats_no_culling() {
        let stats = CullStats {
            total: 50,
            visible: 50,
            culled: 0,
            cull_ratio: 0.0,
        };
        assert!(!stats.has_culling());
    }

    #[test]
    fn test_cull_stats_percentage() {
        let stats = CullStats {
            total: 100,
            visible: 50,
            culled: 50,
            cull_ratio: 0.5,
        };
        assert_eq!(stats.cull_percentage(), 50.0);
    }

    // =========================================================================
    // Batch Culling Tests
    // =========================================================================

    #[test]
    fn test_batch_cull_empty() {
        let culler = FrustumCuller::default();
        let result = batch_cull(&culler, &[], &[]);
        assert!(result.visible_components.is_empty());
        assert!(result.visible_wires.is_empty());
    }

    #[test]
    fn test_batch_cull_all_visible() {
        let culler = FrustumCuller::new(ViewFrustum::new(1000.0, 1000.0, 0.0, 0.0, 1.0));
        let components = vec![(100.0, 100.0), (200.0, 200.0), (300.0, 300.0)];
        let wires = vec![(0.0, 0.0, 100.0, 0.0)];
        let result = batch_cull(&culler, &components, &wires);
        assert_eq!(result.visible_components.len(), 3);
        assert_eq!(result.visible_wires.len(), 1);
    }

    #[test]
    fn test_batch_cull_some_visible() {
        let culler = FrustumCuller::new(ViewFrustum::new(200.0, 200.0, 0.0, 0.0, 1.0));
        let components = vec![
            (50.0, 50.0),   // Inside
            (100.0, 100.0), // Inside
            (500.0, 500.0), // Outside
            (600.0, 600.0), // Outside
        ];
        let result = batch_cull(&culler, &components, &[]);
        assert_eq!(result.visible_components.len(), 2);
        assert!(result.visible_components.contains(&0));
        assert!(result.visible_components.contains(&1));
        assert!(!result.visible_components.contains(&2));
    }

    #[test]
    fn test_batch_cull_stats() {
        let culler = FrustumCuller::new(ViewFrustum::new(200.0, 200.0, 0.0, 0.0, 1.0));
        let components = vec![(50.0, 50.0), (500.0, 500.0)];
        let result = batch_cull(&culler, &components, &[]);
        assert_eq!(result.component_stats.total, 2);
        assert_eq!(result.component_stats.visible, 1);
        assert_eq!(result.component_stats.culled, 1);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_aabb_zero_size() {
        let aabb = AABB::new(50.0, 50.0, 50.0, 50.0);
        assert_eq!(aabb.width(), 0.0);
        assert_eq!(aabb.height(), 0.0);
        assert!(aabb.contains_point(50.0, 50.0));
    }

    #[test]
    fn test_frustum_very_small_zoom() {
        let frustum = ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 0.01);
        // Very zoomed out = very large world view
        assert!(frustum.view_bounds.width() > 10000.0);
    }

    #[test]
    fn test_frustum_very_large_zoom() {
        let frustum = ViewFrustum::new(800.0, 600.0, 0.0, 0.0, 100.0);
        // Very zoomed in = very small world view
        assert!(frustum.view_bounds.width() < 10.0);
    }

    #[test]
    fn test_culler_with_negative_coordinates() {
        let culler = FrustumCuller::new(ViewFrustum::new(800.0, 600.0, 400.0, 300.0, 1.0));
        // View centered around origin
        assert!(culler.is_component_visible(0.0, 0.0));
        assert!(culler.is_component_visible(-100.0, -100.0));
    }
}
