//! Wire Segment Types
//!
//! Individual segment representation with geometric metadata for
//! hit testing, intersection detection, and distance calculations.

use super::super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// WireSegment - Individual wire segment with geometric metadata
// =============================================================================

/// An individual wire segment between two points
///
/// WireSegment provides geometric operations for hit testing, intersection
/// detection, and distance calculations. This is the foundation for
/// robust wire manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireSegment {
    /// Start point of the segment
    pub start: Point,
    /// End point of the segment
    pub end: Point,
}

impl WireSegment {
    /// Create a new wire segment between two points
    #[inline]
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Get the length of the segment in grid units (Manhattan distance for orthogonal)
    #[inline]
    pub fn length(&self) -> i32 {
        let dx = (i64::from(self.end.x) - i64::from(self.start.x)).unsigned_abs();
        let dy = (i64::from(self.end.y) - i64::from(self.start.y)).unsigned_abs();
        i32::try_from(dx.saturating_add(dy)).unwrap_or(i32::MAX)
    }

    /// Get the Euclidean length of the segment
    pub fn euclidean_length(&self) -> f64 {
        let dx = f64::from(self.end.x) - f64::from(self.start.x);
        let dy = f64::from(self.end.y) - f64::from(self.start.y);
        (dx * dx + dy * dy).sqrt()
    }

    /// Get the midpoint of the segment
    #[inline]
    pub fn midpoint(&self) -> Point {
        Point::new(
            (self.start.x + self.end.x) / 2,
            (self.start.y + self.end.y) / 2,
        )
    }

    /// Get the axis-aligned bounding box of the segment
    /// Returns (min_point, max_point)
    pub fn bounding_box(&self) -> (Point, Point) {
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let max_x = self.start.x.max(self.end.x);
        let max_y = self.start.y.max(self.end.y);
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    /// Check if the segment is horizontal (same Y coordinate)
    #[inline]
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// Check if the segment is vertical (same X coordinate)
    #[inline]
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    /// Check if the segment is orthogonal (horizontal or vertical)
    #[inline]
    pub fn is_orthogonal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    /// Check if the segment is a zero-length point
    #[inline]
    pub fn is_zero_length(&self) -> bool {
        self.start == self.end
    }

    /// Check if a point lies exactly on this segment.
    ///
    /// General diagonal geometry uses an integer cross product so extreme
    /// document coordinates remain exact and overflow-free.
    pub fn contains_point(&self, p: Point) -> bool {
        if self.is_zero_length() {
            return p == self.start;
        }

        // For horizontal segments
        if self.is_horizontal() && p.y == self.start.y {
            let (min_x, max_x) = if self.start.x < self.end.x {
                (self.start.x, self.end.x)
            } else {
                (self.end.x, self.start.x)
            };
            return p.x >= min_x && p.x <= max_x;
        }

        // For vertical segments
        if self.is_vertical() && p.x == self.start.x {
            let (min_y, max_y) = if self.start.y < self.end.y {
                (self.start.y, self.end.y)
            } else {
                (self.end.y, self.start.y)
            };
            return p.y >= min_y && p.y <= max_y;
        }

        // For diagonal segments, use parametric line equation
        // Point is on segment if it's collinear and within bounds
        let dx = i128::from(self.end.x) - i128::from(self.start.x);
        let dy = i128::from(self.end.y) - i128::from(self.start.y);
        let px = i128::from(p.x) - i128::from(self.start.x);
        let py = i128::from(p.y) - i128::from(self.start.y);

        // Check collinearity: cross product should be zero
        if px * dy != py * dx {
            return false;
        }

        // Check if within parametric bounds [0, 1]
        let min_x = i128::from(self.start.x.min(self.end.x));
        let max_x = i128::from(self.start.x.max(self.end.x));
        let min_y = i128::from(self.start.y.min(self.end.y));
        let max_y = i128::from(self.start.y.max(self.end.y));
        let point_x = i128::from(p.x);
        let point_y = i128::from(p.y);
        point_x >= min_x && point_x <= max_x && point_y >= min_y && point_y <= max_y
    }

    /// Check if a point is within tolerance of this segment
    pub fn contains_point_with_tolerance(&self, p: Point, tolerance: i32) -> bool {
        let dist = self.distance_to_point(p);
        dist <= tolerance as f64
    }

    /// Calculate the squared distance from a point to this segment
    /// This is more efficient than distance_to_point when comparing distances
    pub fn squared_distance_to_point(&self, p: Point) -> f64 {
        if self.is_zero_length() {
            let dx = f64::from(p.x) - f64::from(self.start.x);
            let dy = f64::from(p.y) - f64::from(self.start.y);
            return dx * dx + dy * dy;
        }

        let dx = f64::from(self.end.x) - f64::from(self.start.x);
        let dy = f64::from(self.end.y) - f64::from(self.start.y);
        let px = f64::from(p.x) - f64::from(self.start.x);
        let py = f64::from(p.y) - f64::from(self.start.y);

        let len_sq = dx * dx + dy * dy;
        let t = ((px * dx + py * dy) / len_sq).clamp(0.0, 1.0);

        let closest_x = self.start.x as f64 + t * dx;
        let closest_y = self.start.y as f64 + t * dy;

        let dist_x = p.x as f64 - closest_x;
        let dist_y = p.y as f64 - closest_y;

        dist_x * dist_x + dist_y * dist_y
    }

    /// Calculate the distance from a point to this segment
    pub fn distance_to_point(&self, p: Point) -> f64 {
        self.squared_distance_to_point(p).sqrt()
    }

    /// Find the closest point on this segment to a given point
    pub fn closest_point(&self, p: Point) -> Point {
        if self.is_zero_length() {
            return self.start;
        }

        let dx = f64::from(self.end.x) - f64::from(self.start.x);
        let dy = f64::from(self.end.y) - f64::from(self.start.y);
        let px = f64::from(p.x) - f64::from(self.start.x);
        let py = f64::from(p.y) - f64::from(self.start.y);

        let len_sq = dx * dx + dy * dy;
        let t = ((px * dx + py * dy) / len_sq).clamp(0.0, 1.0);

        Point::new(
            (self.start.x as f64 + t * dx).round() as i32,
            (self.start.y as f64 + t * dy).round() as i32,
        )
    }

    /// Check if this segment intersects with another segment
    /// Returns the intersection point if they intersect
    pub fn intersection(&self, other: &WireSegment) -> Option<Point> {
        // Using parametric form: P = A + t(B-A), Q = C + u(D-C)
        // Solving for t and u where segments intersect

        let a = self.start;
        let b = self.end;
        let c = other.start;
        let d = other.end;

        let dx1 = i128::from(b.x) - i128::from(a.x);
        let dy1 = i128::from(b.y) - i128::from(a.y);
        let dx2 = i128::from(d.x) - i128::from(c.x);
        let dy2 = i128::from(d.y) - i128::from(c.y);

        let denominator = dx1 * dy2 - dy1 * dx2;

        // Parallel segments
        if denominator == 0 {
            // Check for overlapping collinear segments
            if self.contains_point(other.start) {
                return Some(other.start);
            }
            if self.contains_point(other.end) {
                return Some(other.end);
            }
            if other.contains_point(self.start) {
                return Some(self.start);
            }
            return None;
        }

        let dx3 = i128::from(c.x) - i128::from(a.x);
        let dy3 = i128::from(c.y) - i128::from(a.y);

        let t_num = dx3 * dy2 - dy3 * dx2;
        let u_num = dx3 * dy1 - dy3 * dx1;

        // Check if intersection point is within both segments
        let t = t_num as f64 / denominator as f64;
        let u = u_num as f64 / denominator as f64;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            let x = a.x as f64 + t * dx1 as f64;
            let y = a.y as f64 + t * dy1 as f64;
            Some(Point::new(x.round() as i32, y.round() as i32))
        } else {
            None
        }
    }

    /// Check if this segment overlaps with another (shares more than a point)
    pub fn overlaps(&self, other: &WireSegment) -> bool {
        // Segments must be collinear
        let dx1 = i128::from(self.end.x) - i128::from(self.start.x);
        let dy1 = i128::from(self.end.y) - i128::from(self.start.y);
        let dx2 = i128::from(other.end.x) - i128::from(other.start.x);
        let dy2 = i128::from(other.end.y) - i128::from(other.start.y);

        // Cross product for collinearity check
        if dx1 * dy2 != dy1 * dx2 {
            return false;
        }

        // Check if segments are on the same line
        let dx3 = i128::from(other.start.x) - i128::from(self.start.x);
        let dy3 = i128::from(other.start.y) - i128::from(self.start.y);
        if dx1 * dy3 != dy1 * dx3 {
            return false;
        }

        // Check for overlap along the line
        let (min1, max1) = self.bounding_box();
        let (min2, max2) = other.bounding_box();

        // Overlap exists if bounding boxes overlap (for collinear segments)
        min1.x <= max2.x && max1.x >= min2.x && min1.y <= max2.y && max1.y >= min2.y
    }

    /// Get the direction vector of this segment (normalized to unit length)
    pub fn direction(&self) -> (f64, f64) {
        let len = self.euclidean_length();
        if len == 0.0 {
            return (0.0, 0.0);
        }
        let dx = (f64::from(self.end.x) - f64::from(self.start.x)) / len;
        let dy = (f64::from(self.end.y) - f64::from(self.start.y)) / len;
        (dx, dy)
    }

    /// Get the perpendicular (normal) direction
    pub fn perpendicular(&self) -> (f64, f64) {
        let (dx, dy) = self.direction();
        (-dy, dx)
    }

    /// Reverse the segment direction
    #[inline]
    pub fn reversed(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }

    /// Check if this segment shares an endpoint with another
    pub fn shares_endpoint(&self, other: &WireSegment) -> bool {
        self.start == other.start
            || self.start == other.end
            || self.end == other.start
            || self.end == other.end
    }

    /// Get the shared endpoint with another segment, if any
    pub fn shared_endpoint(&self, other: &WireSegment) -> Option<Point> {
        if self.start == other.start || self.start == other.end {
            Some(self.start)
        } else if self.end == other.start || self.end == other.end {
            Some(self.end)
        } else {
            None
        }
    }
}

// =============================================================================
// WireHitResult - Result of advanced hit testing
// =============================================================================

/// Result of hit testing a wire with tolerance
///
/// This enum provides detailed information about what part of a wire
/// was hit, enabling appropriate cursor feedback and manipulation modes.
#[derive(Debug, Clone, PartialEq)]
pub enum WireHitResult {
    /// Hit a vertex (corner point) - enables corner dragging
    Vertex {
        /// Index of the vertex in the wire's points array
        index: usize,
        /// The vertex point
        point: Point,
        /// Distance from the query point to the vertex
        distance: f64,
    },
    /// Hit a segment (between two vertices) - enables segment dragging
    Segment {
        /// Index of the segment (same as index of start vertex)
        index: usize,
        /// The closest point on the segment to the query point
        closest_point: Point,
        /// Distance from the query point to the segment
        distance: f64,
        /// The segment that was hit
        segment: WireSegment,
    },
    /// No hit - query point is outside tolerance of wire
    None,
}

impl WireHitResult {
    /// Check if this is a hit (not None)
    #[inline]
    pub fn is_hit(&self) -> bool {
        !matches!(self, WireHitResult::None)
    }

    /// Get the distance to the hit point (infinity if no hit)
    pub fn distance(&self) -> f64 {
        match self {
            WireHitResult::Vertex { distance, .. } => *distance,
            WireHitResult::Segment { distance, .. } => *distance,
            WireHitResult::None => f64::INFINITY,
        }
    }

    /// Get the hit point (None if no hit)
    pub fn point(&self) -> Option<Point> {
        match self {
            WireHitResult::Vertex { point, .. } => Some(*point),
            WireHitResult::Segment { closest_point, .. } => Some(*closest_point),
            WireHitResult::None => None,
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
    fn hit_geometry_is_safe_at_the_signed_coordinate_limits() {
        let diagonal = WireSegment::new(
            Point::new(i32::MIN + 1, i32::MIN + 1),
            Point::new(i32::MAX, i32::MAX),
        );
        assert!(diagonal.contains_point(Point::origin()));
        assert_eq!(diagonal.length(), i32::MAX);
        assert!(diagonal.euclidean_length().is_finite());
        assert_eq!(diagonal.closest_point(Point::origin()), Point::origin());
        assert_eq!(diagonal.distance_to_point(Point::origin()), 0.0);

        let crossing = WireSegment::new(
            Point::new(i32::MIN + 1, i32::MAX),
            Point::new(i32::MAX, i32::MIN + 1),
        );
        assert_eq!(diagonal.intersection(&crossing), Some(Point::origin()));
        assert!(!diagonal.overlaps(&crossing));
    }

    #[test]
    fn extreme_collinear_segments_overlap_without_integer_wraparound() {
        let span = WireSegment::new(Point::new(i32::MIN, 7), Point::new(i32::MAX, 7));
        let inner = WireSegment::new(Point::new(-10, 7), Point::new(10, 7));
        assert!(span.overlaps(&inner));
        assert_eq!(span.distance_to_point(Point::new(0, 8)), 1.0);
    }
}
