//! Utility Functions for Schematic Editor
//!
//! Geometry calculations for hit-testing and distance measurements.

use crate::state::Point;

/// Calculate distance from point to line segment (in grid units)
///
/// Used for hit-testing wires and determining if a click is on a wire segment.
/// Returns the perpendicular distance if the point projects onto the segment,
/// otherwise returns the distance to the nearest endpoint.
///
/// # Arguments
/// * `p` - The point to measure from
/// * `a` - First endpoint of the segment
/// * `b` - Second endpoint of the segment
///
/// # Returns
/// The minimum distance from point `p` to the line segment `a`-`b`
pub fn point_to_segment_dist(p: Point, a: Point, b: Point) -> f64 {
    let px = p.x as f64;
    let py = p.y as f64;
    let ax = a.x as f64;
    let ay = a.y as f64;
    let bx = b.x as f64;
    let by = b.y as f64;

    let dx = bx - ax;
    let dy = by - ay;
    let len_sq = dx * dx + dy * dy;

    if len_sq == 0.0 {
        // Degenerate segment: a and b are the same point
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }

    // Project point onto line, clamping to segment bounds [0, 1]
    let t = ((px - ax) * dx + (py - ay) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj_x = ax + t * dx;
    let proj_y = ay + t * dy;

    ((px - proj_x).powi(2) + (py - proj_y).powi(2)).sqrt()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_segment() {
        // Point exactly on horizontal segment
        let dist = point_to_segment_dist(Point::new(5, 0), Point::new(0, 0), Point::new(10, 0));
        assert!(dist < 0.001);
    }

    #[test]
    fn test_point_perpendicular_above() {
        // Point 3 units above horizontal segment
        let dist = point_to_segment_dist(Point::new(5, 3), Point::new(0, 0), Point::new(10, 0));
        assert!((dist - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_point_beyond_endpoint() {
        // Point beyond segment end (should return distance to endpoint)
        let dist = point_to_segment_dist(Point::new(15, 0), Point::new(0, 0), Point::new(10, 0));
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_point_before_start() {
        // Point before segment start
        let dist = point_to_segment_dist(Point::new(-3, 0), Point::new(0, 0), Point::new(10, 0));
        assert!((dist - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_degenerate_segment() {
        // Degenerate segment (single point) - returns distance to that point
        let dist = point_to_segment_dist(Point::new(3, 4), Point::new(0, 0), Point::new(0, 0));
        assert!((dist - 5.0).abs() < 0.001); // sqrt(3^2 + 4^2) = 5
    }

    #[test]
    fn test_diagonal_segment() {
        // Diagonal segment (0,0) to (10,10), point at (5,5) should be on segment
        let dist = point_to_segment_dist(Point::new(5, 5), Point::new(0, 0), Point::new(10, 10));
        assert!(dist < 0.001);
    }

    #[test]
    fn test_vertical_segment() {
        // Vertical segment, point 4 units to the right
        let dist = point_to_segment_dist(Point::new(4, 5), Point::new(0, 0), Point::new(0, 10));
        assert!((dist - 4.0).abs() < 0.001);
    }
}
