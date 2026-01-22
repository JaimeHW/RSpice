//! Viewport Culling for Schematic Rendering
//!
//! Provides efficient spatial queries to determine which schematic elements
//! are visible in the current view. This is critical for performance when
//! rendering large schematics with thousands of components.
//!
//! Simulators use viewport culling to
//! maintain 60fps even with complex designs by only rendering visible elements.
//!
//! # Usage
//!
//! ```ignore
//! let viewport = Viewport::from_transform(canvas_width, canvas_height, pan, zoom, grid_size);
//!
//! // Filter to only visible elements
//! let visible_components: Vec<_> = components
//!     .iter()
//!     .filter(|c| viewport.is_component_visible(c.pos, 30))
//!     .collect();
//! ```

use crate::state::Point;

// =============================================================================
// Bounding Box
// =============================================================================

/// Axis-aligned bounding box in grid coordinates
///
/// Used for fast intersection testing between schematic elements and the viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Create from center point and half-size
    pub fn from_center(center: Point, half_width: i32, half_height: i32) -> Self {
        Self {
            min_x: center.x - half_width,
            min_y: center.y - half_height,
            max_x: center.x + half_width,
            max_y: center.y + half_height,
        }
    }

    /// Create a bounding box that encompasses all points
    pub fn from_points(points: &[Point]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for p in points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Some(Self {
            min_x,
            min_y,
            max_x,
            max_y,
        })
    }

    /// Check if this box intersects another
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }

    /// Check if a point is inside this box
    pub fn contains_point(&self, p: Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }

    /// Expand box by a margin on all sides
    pub fn expand(&self, margin: i32) -> Self {
        Self {
            min_x: self.min_x - margin,
            min_y: self.min_y - margin,
            max_x: self.max_x + margin,
            max_y: self.max_y + margin,
        }
    }

    /// Get the width of the box
    pub fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    /// Get the height of the box
    pub fn height(&self) -> i32 {
        self.max_y - self.min_y
    }

    /// Get the center point
    pub fn center(&self) -> Point {
        Point::new((self.min_x + self.max_x) / 2, (self.min_y + self.max_y) / 2)
    }
}

// =============================================================================
// Viewport
// =============================================================================

/// Current viewport state computed from pan/zoom transformations
///
/// This represents the visible area of the schematic canvas in grid coordinates.
/// It's recomputed whenever pan or zoom changes.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    /// Visible area in grid coordinates
    pub bounds: BoundingBox,
    /// Current zoom level (1.0 = 100%)
    pub zoom: f64,
    /// Canvas dimensions in pixels
    pub canvas_width: f64,
    pub canvas_height: f64,
}

impl Viewport {
    /// Compute viewport from canvas size, pan offset, and zoom level
    ///
    /// This converts the screen-space viewport rectangle to grid coordinates,
    /// which can then be used for intersection testing with schematic elements.
    ///
    /// # Arguments
    /// * `canvas_width` - Width of the schematic canvas in pixels
    /// * `canvas_height` - Height of the schematic canvas in pixels
    /// * `pan` - Current pan offset in pixels (x, y)
    /// * `zoom` - Current zoom level (1.0 = 100%)
    /// * `grid_size` - Grid cell size in pixels
    pub fn from_transform(
        canvas_width: f64,
        canvas_height: f64,
        pan: (f64, f64),
        zoom: f64,
        grid_size: i32,
    ) -> Self {
        let gs = grid_size as f64;

        // The SVG transform is: translate(pan_x, pan_y) scale(zoom)
        // So screen_pos = pan + (world_pos * zoom)
        // Therefore: world_pos = (screen_pos - pan) / zoom

        // Top-left corner (screen 0,0) in grid coordinates
        let min_x = ((-pan.0) / zoom / gs).floor() as i32;
        let min_y = ((-pan.1) / zoom / gs).floor() as i32;

        // Bottom-right corner in grid coordinates
        let max_x = ((canvas_width - pan.0) / zoom / gs).ceil() as i32;
        let max_y = ((canvas_height - pan.1) / zoom / gs).ceil() as i32;

        Self {
            bounds: BoundingBox::new(min_x, min_y, max_x, max_y),
            zoom,
            canvas_width,
            canvas_height,
        }
    }

    /// Check if a component at the given position is visible
    ///
    /// Components have a symbol footprint, so we check if their bounding box
    /// intersects the viewport.
    ///
    /// # Arguments
    /// * `pos` - Component position in grid coordinates
    /// * `symbol_half_size` - Half the size of the component symbol (in grid units)
    pub fn is_component_visible(&self, pos: Point, symbol_half_size: i32) -> bool {
        let comp_bounds = BoundingBox::from_center(pos, symbol_half_size, symbol_half_size);
        self.bounds.intersects(&comp_bounds)
    }

    /// Check if a wire with the given points is visible
    ///
    /// A wire is visible if any of its points are in the viewport, or if any
    /// of its segments cross the viewport boundary.
    pub fn is_wire_visible(&self, points: &[Point]) -> bool {
        if points.is_empty() {
            return false;
        }

        // Quick check: any point inside viewport?
        for p in points {
            if self.bounds.contains_point(*p) {
                return true;
            }
        }

        // Full check: any segment crosses viewport?
        for segment in points.windows(2) {
            if self.segment_intersects_viewport(segment[0], segment[1]) {
                return true;
            }
        }

        false
    }

    /// Check if a line segment intersects the viewport rectangle
    ///
    /// Uses Cohen-Sutherland-style region checking for orthogonal segments.
    fn segment_intersects_viewport(&self, p1: Point, p2: Point) -> bool {
        let b = &self.bounds;

        // Horizontal segment
        if p1.y == p2.y {
            let y = p1.y;
            if y < b.min_y || y > b.max_y {
                return false;
            }
            let (x1, x2) = if p1.x < p2.x {
                (p1.x, p2.x)
            } else {
                (p2.x, p1.x)
            };
            return x2 >= b.min_x && x1 <= b.max_x;
        }

        // Vertical segment
        if p1.x == p2.x {
            let x = p1.x;
            if x < b.min_x || x > b.max_x {
                return false;
            }
            let (y1, y2) = if p1.y < p2.y {
                (p1.y, p2.y)
            } else {
                (p2.y, p1.y)
            };
            return y2 >= b.min_y && y1 <= b.max_y;
        }

        // For diagonal segments (rare in our orthogonal routing), use conservative check
        // Check if segment bounding box intersects viewport
        let seg_bounds = BoundingBox::new(
            p1.x.min(p2.x),
            p1.y.min(p2.y),
            p1.x.max(p2.x),
            p1.y.max(p2.y),
        );
        self.bounds.intersects(&seg_bounds)
    }

    /// Check if a net label at the given position is visible
    pub fn is_label_visible(&self, pos: Point) -> bool {
        // Labels extend to the right, approximate width ~10 grid units
        let label_bounds = BoundingBox::new(pos.x, pos.y - 1, pos.x + 10, pos.y + 1);
        self.bounds.intersects(&label_bounds)
    }

    /// Check if a junction at the given position is visible
    pub fn is_junction_visible(&self, pos: Point) -> bool {
        // Junction dots are small - just check if point is in viewport with small margin
        let junction_bounds = BoundingBox::from_center(pos, 1, 1);
        self.bounds.intersects(&junction_bounds)
    }

    /// Determine if simplified (LOD) rendering should be used
    ///
    /// At very low zoom levels, detailed component symbols waste rendering time.
    /// Tools switch to simplified representations when zoomed out.
    pub fn use_simplified_symbols(&self) -> bool {
        self.zoom < 0.25
    }

    /// Determine if component labels should be hidden due to zoom level
    ///
    /// Labels become unreadable at low zoom, so hiding them improves performance.
    pub fn should_hide_labels(&self) -> bool {
        self.zoom < 0.4
    }

    /// Get approximate number of grid cells visible
    ///
    /// Useful for estimating rendering complexity.
    pub fn visible_area(&self) -> i64 {
        (self.bounds.width() as i64) * (self.bounds.height() as i64)
    }
}

// =============================================================================
// Visibility Filter (Helper for render loops)
// =============================================================================

/// Helper struct for efficiently filtering schematic elements by visibility
pub struct VisibilityFilter {
    viewport: Viewport,
    component_half_size: i32,
}

impl VisibilityFilter {
    /// Create a new visibility filter
    pub fn new(viewport: Viewport, component_half_size: i32) -> Self {
        Self {
            viewport,
            component_half_size,
        }
    }

    /// Check if component is visible
    pub fn component_visible(&self, pos: Point) -> bool {
        self.viewport
            .is_component_visible(pos, self.component_half_size)
    }

    /// Check if wire is visible
    pub fn wire_visible(&self, points: &[Point]) -> bool {
        self.viewport.is_wire_visible(points)
    }

    /// Check if label is visible
    pub fn label_visible(&self, pos: Point) -> bool {
        self.viewport.is_label_visible(pos)
    }

    /// Check if junction is visible
    pub fn junction_visible(&self, pos: Point) -> bool {
        self.viewport.is_junction_visible(pos)
    }

    /// Get underlying viewport
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_intersects() {
        let a = BoundingBox::new(0, 0, 10, 10);
        let b = BoundingBox::new(5, 5, 15, 15);
        let c = BoundingBox::new(20, 20, 30, 30);

        assert!(a.intersects(&b)); // Overlapping
        assert!(b.intersects(&a)); // Symmetric
        assert!(!a.intersects(&c)); // No overlap
    }

    #[test]
    fn test_bounding_box_contains_point() {
        let bbox = BoundingBox::new(0, 0, 10, 10);

        assert!(bbox.contains_point(Point::new(5, 5))); // Inside
        assert!(bbox.contains_point(Point::new(0, 0))); // On corner
        assert!(bbox.contains_point(Point::new(10, 10))); // On corner
        assert!(!bbox.contains_point(Point::new(11, 5))); // Outside
        assert!(!bbox.contains_point(Point::new(-1, 5))); // Outside
    }

    #[test]
    fn test_viewport_from_transform() {
        // 800x600 canvas, no pan, zoom 1.0, grid size 10
        let viewport = Viewport::from_transform(800.0, 600.0, (0.0, 0.0), 1.0, 10);

        // Viewport should cover 80x60 grid cells
        assert_eq!(viewport.bounds.min_x, 0);
        assert_eq!(viewport.bounds.min_y, 0);
        assert_eq!(viewport.bounds.max_x, 80);
        assert_eq!(viewport.bounds.max_y, 60);
    }

    #[test]
    fn test_viewport_with_pan() {
        // Pan by 100 pixels right and 50 pixels down
        let viewport = Viewport::from_transform(800.0, 600.0, (100.0, 50.0), 1.0, 10);

        // Viewport origin shifts left and up by pan amount
        assert_eq!(viewport.bounds.min_x, -10); // -100/1.0/10
        assert_eq!(viewport.bounds.min_y, -5); // -50/1.0/10
    }

    #[test]
    fn test_viewport_with_zoom() {
        // Zoom 2x means viewport covers half the grid area
        let viewport = Viewport::from_transform(800.0, 600.0, (0.0, 0.0), 2.0, 10);

        // At 2x zoom, viewport covers 40x30 grid cells
        assert_eq!(viewport.bounds.max_x, 40);
        assert_eq!(viewport.bounds.max_y, 30);
    }

    #[test]
    fn test_component_visibility() {
        let viewport = Viewport::from_transform(800.0, 600.0, (0.0, 0.0), 1.0, 10);

        // Component at (40, 30) with half-size 5 - should be visible
        assert!(viewport.is_component_visible(Point::new(40, 30), 5));

        // Component at (100, 100) - outside viewport
        assert!(!viewport.is_component_visible(Point::new(100, 100), 5));

        // Component at edge - half-size allows it to extend into viewport
        assert!(viewport.is_component_visible(Point::new(82, 30), 5));
    }

    #[test]
    fn test_wire_visibility() {
        let viewport = Viewport::from_transform(800.0, 600.0, (0.0, 0.0), 1.0, 10);

        // Wire completely inside
        let wire1 = vec![Point::new(10, 10), Point::new(20, 10), Point::new(20, 20)];
        assert!(viewport.is_wire_visible(&wire1));

        // Wire completely outside
        let wire2 = vec![Point::new(100, 100), Point::new(110, 100)];
        assert!(!viewport.is_wire_visible(&wire2));

        // Wire crossing viewport (starts outside, ends inside)
        let wire3 = vec![Point::new(-10, 30), Point::new(40, 30)];
        assert!(viewport.is_wire_visible(&wire3));
    }

    #[test]
    fn test_segment_intersection() {
        let viewport = Viewport::from_transform(400.0, 300.0, (0.0, 0.0), 1.0, 10);

        // Horizontal segment crossing viewport (left to right)
        assert!(viewport.segment_intersects_viewport(Point::new(-10, 15), Point::new(50, 15)));

        // Vertical segment outside viewport
        assert!(!viewport.segment_intersects_viewport(Point::new(50, 100), Point::new(50, 110)));
    }

    #[test]
    fn test_simplified_symbols_threshold() {
        let normal = Viewport::from_transform(800.0, 600.0, (0.0, 0.0), 1.0, 10);
        assert!(!normal.use_simplified_symbols());

        let zoomed_out = Viewport::from_transform(800.0, 600.0, (0.0, 0.0), 0.2, 10);
        assert!(zoomed_out.use_simplified_symbols());
    }
}
