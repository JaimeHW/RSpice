//! Point and Label Position Types
//!
//! Grid-aligned coordinate system for schematic elements.

use serde::{Deserialize, Serialize};

// =============================================================================
// Point
// =============================================================================

/// Grid-aligned point (in grid units, not pixels)
///
/// The schematic uses a grid-based coordinate system where all elements
/// snap to grid intersections. This ensures clean, aligned circuit diagrams.
///
/// # Coordinate System
/// - Origin (0, 0) is at the center of the canvas
/// - X increases to the right
/// - Y increases downward (screen coordinates)
/// - Grid size is typically 10 pixels per unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point at (x, y)
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Create a point at the origin (0, 0)
    #[inline]
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Convert to pixel coordinates
    ///
    /// # Arguments
    /// * `grid_size` - Size of each grid cell in pixels
    #[inline]
    pub fn to_pixels(self, grid_size: i32) -> (f64, f64) {
        ((self.x * grid_size) as f64, (self.y * grid_size) as f64)
    }

    /// Create from pixel coordinates (snaps to nearest grid point)
    ///
    /// # Arguments
    /// * `px` - X coordinate in pixels
    /// * `py` - Y coordinate in pixels
    /// * `grid_size` - Size of each grid cell in pixels
    #[inline]
    pub fn from_pixels(px: f64, py: f64, grid_size: i32) -> Self {
        Self {
            x: (px / grid_size as f64).round() as i32,
            y: (py / grid_size as f64).round() as i32,
        }
    }

    /// Get the 4 adjacent points (cardinal directions)
    ///
    /// Returns neighbors in order: left, right, up, down
    #[inline]
    pub fn neighbors(self) -> [Point; 4] {
        [
            Point::new(self.x - 1, self.y), // Left
            Point::new(self.x + 1, self.y), // Right
            Point::new(self.x, self.y - 1), // Up
            Point::new(self.x, self.y + 1), // Down
        ]
    }

    /// Calculate Manhattan distance to another point
    #[inline]
    pub fn manhattan_distance(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Calculate squared Euclidean distance to another point
    /// (avoids sqrt for comparison purposes)
    #[inline]
    pub fn distance_squared(self, other: Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Add another point (vector addition)
    #[inline]
    pub fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract another point (vector subtraction)
    #[inline]
    pub fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

// =============================================================================
// Label Position
// =============================================================================

/// Label position mode for component labels
///
/// Implements smart auto-placement with user override capability.
/// Auto mode uses heuristics to avoid collisions with wires and components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum LabelPosition {
    /// Automatic smart placement - avoids collisions with wires and components
    #[default]
    Auto,
    /// User-defined custom offset from default position (in pixels)
    Custom { x_offset: f64, y_offset: f64 },
}


impl LabelPosition {
    /// Create a custom label position with the given offsets
    pub fn custom(x_offset: f64, y_offset: f64) -> Self {
        LabelPosition::Custom { x_offset, y_offset }
    }

    /// Check if this is an auto-positioned label
    pub fn is_auto(&self) -> bool {
        matches!(self, LabelPosition::Auto)
    }

    /// Get the custom offsets, or (0, 0) if auto
    pub fn offsets(&self) -> (f64, f64) {
        match self {
            LabelPosition::Auto => (0.0, 0.0),
            LabelPosition::Custom { x_offset, y_offset } => (*x_offset, *y_offset),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new(5, 10);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn test_point_origin() {
        let p = Point::origin();
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }

    #[test]
    fn test_point_to_pixels() {
        let p = Point::new(3, 4);
        let (px, py) = p.to_pixels(10);
        assert_eq!(px, 30.0);
        assert_eq!(py, 40.0);
    }

    #[test]
    fn test_point_from_pixels_snapping() {
        // Exact grid point
        let p1 = Point::from_pixels(30.0, 40.0, 10);
        assert_eq!(p1, Point::new(3, 4));

        // Snap to nearest (round up)
        let p2 = Point::from_pixels(35.0, 45.0, 10);
        assert_eq!(p2, Point::new(4, 5));

        // Snap to nearest (round down)
        let p3 = Point::from_pixels(34.0, 44.0, 10);
        assert_eq!(p3, Point::new(3, 4));
    }

    #[test]
    fn test_point_neighbors() {
        let p = Point::new(5, 5);
        let neighbors = p.neighbors();
        assert_eq!(neighbors[0], Point::new(4, 5)); // Left
        assert_eq!(neighbors[1], Point::new(6, 5)); // Right
        assert_eq!(neighbors[2], Point::new(5, 4)); // Up
        assert_eq!(neighbors[3], Point::new(5, 6)); // Down
    }

    #[test]
    fn test_point_manhattan_distance() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.manhattan_distance(b), 7);
        assert_eq!(b.manhattan_distance(a), 7);
    }

    #[test]
    fn test_point_distance_squared() {
        let a = Point::new(0, 0);
        let b = Point::new(3, 4);
        assert_eq!(a.distance_squared(b), 25); // 3² + 4² = 25
    }

    #[test]
    fn test_point_add_sub() {
        let a = Point::new(5, 10);
        let b = Point::new(3, 4);
        assert_eq!(a.add(b), Point::new(8, 14));
        assert_eq!(a.sub(b), Point::new(2, 6));
    }

    #[test]
    fn test_label_position_default() {
        let pos = LabelPosition::default();
        assert!(pos.is_auto());
    }

    #[test]
    fn test_label_position_custom() {
        let pos = LabelPosition::custom(10.5, -5.0);
        assert!(!pos.is_auto());
        assert_eq!(pos.offsets(), (10.5, -5.0));
    }

    #[test]
    fn test_label_position_auto_offsets() {
        let pos = LabelPosition::Auto;
        assert_eq!(pos.offsets(), (0.0, 0.0));
    }
}
