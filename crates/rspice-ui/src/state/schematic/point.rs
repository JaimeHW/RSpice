//! Point and Label Position Types
//!
//! Grid-aligned coordinate system for schematic elements.

use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

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
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    /// Subtract another point (vector subtraction)
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        self.sub(rhs)
    }
}

// =============================================================================
// Label Position
// =============================================================================

/// Label position mode for component labels
///
/// Implements smart auto-placement with user override capability.
/// Auto mode uses heuristics to avoid collisions with wires and components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
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

