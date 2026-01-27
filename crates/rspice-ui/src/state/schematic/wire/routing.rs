//! Wire Routing Mode and Utilities
//!
//! Wire routing algorithms and route optimization functions for
//! interactive wire drawing in schematic editors.

use super::super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// Wire Routing Mode
// =============================================================================

/// Wire routing mode for drawing
///
/// Controls how the cursor position is connected to the last wire point
/// when drawing wires interactively. Professional EDA tools like Cadence
/// and Altium support multiple routing modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WireRoutingMode {
    /// Horizontal first, then vertical (L-shape: →↓)
    /// Standard orthogonal routing with X movement before Y
    #[default]
    HorizontalFirst,

    /// Vertical first, then horizontal (inverted L-shape: ↓→)
    /// Standard orthogonal routing with Y movement before X
    VerticalFirst,

    /// Direct diagonal connection (line from start to end)
    /// Allows any angle, not just orthogonal or 45°
    Diagonal,

    /// 45-degree routing mode (octagonal)
    /// Routes in H/V plus 45° diagonal segments
    /// Similar to what PCB routing tools use
    FortyFiveDegree,
}

impl WireRoutingMode {
    /// Toggle between routing modes
    ///
    /// Cycles: HorizontalFirst -> VerticalFirst -> Diagonal -> FortyFiveDegree -> HorizontalFirst
    pub fn toggle(self) -> Self {
        match self {
            WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
            WireRoutingMode::VerticalFirst => WireRoutingMode::Diagonal,
            WireRoutingMode::Diagonal => WireRoutingMode::FortyFiveDegree,
            WireRoutingMode::FortyFiveDegree => WireRoutingMode::HorizontalFirst,
        }
    }

    /// Toggle only between orthogonal modes (for compatibility)
    pub fn toggle_orthogonal(self) -> Self {
        match self {
            WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
            WireRoutingMode::VerticalFirst => WireRoutingMode::HorizontalFirst,
            // Non-orthogonal modes default to HorizontalFirst
            _ => WireRoutingMode::HorizontalFirst,
        }
    }

    /// Check if this mode is orthogonal-only (no diagonals)
    pub fn is_orthogonal(&self) -> bool {
        matches!(
            self,
            WireRoutingMode::HorizontalFirst | WireRoutingMode::VerticalFirst
        )
    }

    /// Check if this mode allows diagonal segments
    pub fn allows_diagonal(&self) -> bool {
        matches!(
            self,
            WireRoutingMode::Diagonal | WireRoutingMode::FortyFiveDegree
        )
    }

    /// Get a human-readable name for this mode
    pub fn display_name(&self) -> &'static str {
        match self {
            WireRoutingMode::HorizontalFirst => "Orthogonal (H→V)",
            WireRoutingMode::VerticalFirst => "Orthogonal (V→H)",
            WireRoutingMode::Diagonal => "Diagonal",
            WireRoutingMode::FortyFiveDegree => "45° Routing",
        }
    }

    /// Get a short keyboard hint for this mode
    pub fn keyboard_hint(&self) -> &'static str {
        match self {
            WireRoutingMode::HorizontalFirst => "H→V",
            WireRoutingMode::VerticalFirst => "V→H",
            WireRoutingMode::Diagonal => "Diag",
            WireRoutingMode::FortyFiveDegree => "45°",
        }
    }

    /// Suggest a route from one point to another
    ///
    /// Returns a list of intermediate points (excluding start, including end).
    /// The result depends on the routing mode.
    pub fn suggest_route(&self, from: Point, to: Point) -> Vec<Point> {
        if from == to {
            return vec![];
        }

        match self {
            WireRoutingMode::HorizontalFirst => {
                if from.x == to.x || from.y == to.y {
                    vec![to]
                } else {
                    // L-shape: horizontal first
                    vec![Point::new(to.x, from.y), to]
                }
            }
            WireRoutingMode::VerticalFirst => {
                if from.x == to.x || from.y == to.y {
                    vec![to]
                } else {
                    // L-shape: vertical first
                    vec![Point::new(from.x, to.y), to]
                }
            }
            WireRoutingMode::Diagonal => {
                // Direct line
                vec![to]
            }
            WireRoutingMode::FortyFiveDegree => Self::suggest_45_degree_route(from, to),
        }
    }

    /// Suggest a 45-degree route between two points
    ///
    /// Creates a route using only horizontal, vertical, and 45° diagonal segments.
    fn suggest_45_degree_route(from: Point, to: Point) -> Vec<Point> {
        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();

        if dx == 0 || dy == 0 || dx == dy {
            // Already aligned (H/V/45°)
            return vec![to];
        }

        // Route: orthogonal segment, then 45° diagonal
        if dx > dy {
            // More horizontal: go H first for 'remaining', then 45° diagonal
            let mid_x = if to.x > from.x {
                from.x + (dx - dy)
            } else {
                from.x - (dx - dy)
            };
            let mid = Point::new(mid_x, from.y);
            vec![mid, to]
        } else {
            // More vertical: go V first for 'remaining', then 45° diagonal
            let mid_y = if to.y > from.y {
                from.y + (dy - dx)
            } else {
                from.y - (dy - dx)
            };
            let mid = Point::new(from.x, mid_y);
            vec![mid, to]
        }
    }

    /// All routing modes in order
    pub const ALL: [WireRoutingMode; 4] = [
        WireRoutingMode::HorizontalFirst,
        WireRoutingMode::VerticalFirst,
        WireRoutingMode::Diagonal,
        WireRoutingMode::FortyFiveDegree,
    ];
}

// =============================================================================
// Route Optimization Functions
// =============================================================================

/// Optimize a route by removing redundant points
///
/// Removes points that are collinear (on the same line as neighbors).
/// This cleans up routes that have accumulated unnecessary vertices.
pub fn optimize_route(points: &[Point]) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut result = Vec::with_capacity(points.len());
    result.push(points[0]);

    for window in points.windows(3) {
        let (p1, p2, p3) = (window[0], window[1], window[2]);

        // Check if p2 is collinear with p1 and p3
        // For orthogonal: same X or same Y for all three
        let collinear_h = p1.y == p2.y && p2.y == p3.y;
        let collinear_v = p1.x == p2.x && p2.x == p3.x;

        // For diagonal: check if on same line using cross product
        let dx1 = p2.x - p1.x;
        let dy1 = p2.y - p1.y;
        let dx2 = p3.x - p2.x;
        let dy2 = p3.y - p2.y;
        let cross = dx1 * dy2 - dy1 * dx2;
        let collinear_diag = cross == 0;

        if !collinear_h && !collinear_v && !collinear_diag {
            // p2 is a true corner, keep it
            result.push(p2);
        }
    }

    result.push(*points.last().unwrap());
    result
}

/// Convert a route to orthogonal-only segments
///
/// Takes any route and converts diagonal segments into orthogonal (H/V) segments.
/// Useful when the user wants to clean up hand-drawn diagonal wires.
pub fn convert_to_orthogonal(points: &[Point]) -> Vec<Point> {
    if points.len() <= 1 {
        return points.to_vec();
    }

    let mut result = Vec::with_capacity(points.len() * 2);
    result.push(points[0]);

    for window in points.windows(2) {
        let (p1, p2) = (window[0], window[1]);

        // If already orthogonal, just add the endpoint
        if p1.x == p2.x || p1.y == p2.y {
            result.push(p2);
        } else {
            // Diagonal - convert to L-shape (horizontal first)
            result.push(Point::new(p2.x, p1.y));
            result.push(p2);
        }
    }

    optimize_route(&result)
}

/// Calculate the total wire length of a route
///
/// Sums up the length of all segments in the route.
pub fn route_length(points: &[Point]) -> f64 {
    points
        .windows(2)
        .map(|w| {
            let dx = (w[1].x - w[0].x) as f64;
            let dy = (w[1].y - w[0].y) as f64;
            (dx * dx + dy * dy).sqrt()
        })
        .sum()
}

/// Count the number of bends (direction changes) in a route
pub fn count_bends(points: &[Point]) -> usize {
    if points.len() <= 2 {
        return 0;
    }

    let mut bends = 0;
    for window in points.windows(3) {
        let (p1, p2, p3) = (window[0], window[1], window[2]);
        let dx1 = p2.x - p1.x;
        let dy1 = p2.y - p1.y;
        let dx2 = p3.x - p2.x;
        let dy2 = p3.y - p2.y;

        // Normalize directions to signs
        let dir1 = (dx1.signum(), dy1.signum());
        let dir2 = (dx2.signum(), dy2.signum());

        if dir1 != dir2 {
            bends += 1;
        }
    }
    bends
}

/// Check if a route is valid (has at least 2 points and no zero-length segments)
pub fn is_valid_route(points: &[Point]) -> bool {
    if points.len() < 2 {
        return false;
    }

    for window in points.windows(2) {
        if window[0] == window[1] {
            return false;
        }
    }

    true
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // WireRoutingMode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_routing_mode_default() {
        let mode = WireRoutingMode::default();
        assert_eq!(mode, WireRoutingMode::HorizontalFirst);
    }

    #[test]
    fn test_routing_mode_toggle() {
        let mut mode = WireRoutingMode::HorizontalFirst;
        mode = mode.toggle();
        assert_eq!(mode, WireRoutingMode::VerticalFirst);
        mode = mode.toggle();
        assert_eq!(mode, WireRoutingMode::Diagonal);
        mode = mode.toggle();
        assert_eq!(mode, WireRoutingMode::FortyFiveDegree);
        mode = mode.toggle();
        assert_eq!(mode, WireRoutingMode::HorizontalFirst);
    }

    #[test]
    fn test_routing_mode_toggle_orthogonal() {
        let mut mode = WireRoutingMode::HorizontalFirst;
        mode = mode.toggle_orthogonal();
        assert_eq!(mode, WireRoutingMode::VerticalFirst);
        mode = mode.toggle_orthogonal();
        assert_eq!(mode, WireRoutingMode::HorizontalFirst);
    }

    #[test]
    fn test_routing_mode_toggle_orthogonal_from_diagonal() {
        let mode = WireRoutingMode::Diagonal;
        assert_eq!(mode.toggle_orthogonal(), WireRoutingMode::HorizontalFirst);
    }

    #[test]
    fn test_routing_mode_is_orthogonal() {
        assert!(WireRoutingMode::HorizontalFirst.is_orthogonal());
        assert!(WireRoutingMode::VerticalFirst.is_orthogonal());
        assert!(!WireRoutingMode::Diagonal.is_orthogonal());
        assert!(!WireRoutingMode::FortyFiveDegree.is_orthogonal());
    }

    #[test]
    fn test_routing_mode_allows_diagonal() {
        assert!(!WireRoutingMode::HorizontalFirst.allows_diagonal());
        assert!(!WireRoutingMode::VerticalFirst.allows_diagonal());
        assert!(WireRoutingMode::Diagonal.allows_diagonal());
        assert!(WireRoutingMode::FortyFiveDegree.allows_diagonal());
    }

    #[test]
    fn test_routing_mode_display_name() {
        assert_eq!(
            WireRoutingMode::HorizontalFirst.display_name(),
            "Orthogonal (H→V)"
        );
        assert_eq!(
            WireRoutingMode::VerticalFirst.display_name(),
            "Orthogonal (V→H)"
        );
        assert_eq!(WireRoutingMode::Diagonal.display_name(), "Diagonal");
        assert_eq!(
            WireRoutingMode::FortyFiveDegree.display_name(),
            "45° Routing"
        );
    }

    // -------------------------------------------------------------------------
    // suggest_route Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_suggest_route_same_point() {
        let mode = WireRoutingMode::HorizontalFirst;
        let route = mode.suggest_route(Point::new(0, 0), Point::new(0, 0));
        assert!(route.is_empty());
    }

    #[test]
    fn test_suggest_route_horizontal_aligned() {
        let mode = WireRoutingMode::HorizontalFirst;
        let route = mode.suggest_route(Point::new(0, 5), Point::new(10, 5));
        assert_eq!(route, vec![Point::new(10, 5)]);
    }

    #[test]
    fn test_suggest_route_vertical_aligned() {
        let mode = WireRoutingMode::VerticalFirst;
        let route = mode.suggest_route(Point::new(5, 0), Point::new(5, 10));
        assert_eq!(route, vec![Point::new(5, 10)]);
    }

    #[test]
    fn test_suggest_route_horizontal_first() {
        let mode = WireRoutingMode::HorizontalFirst;
        let route = mode.suggest_route(Point::new(0, 0), Point::new(10, 5));
        assert_eq!(route, vec![Point::new(10, 0), Point::new(10, 5)]);
    }

    #[test]
    fn test_suggest_route_vertical_first() {
        let mode = WireRoutingMode::VerticalFirst;
        let route = mode.suggest_route(Point::new(0, 0), Point::new(10, 5));
        assert_eq!(route, vec![Point::new(0, 5), Point::new(10, 5)]);
    }

    #[test]
    fn test_suggest_route_diagonal() {
        let mode = WireRoutingMode::Diagonal;
        let route = mode.suggest_route(Point::new(0, 0), Point::new(10, 5));
        assert_eq!(route, vec![Point::new(10, 5)]);
    }

    #[test]
    fn test_suggest_route_45_degree_more_horizontal() {
        let mode = WireRoutingMode::FortyFiveDegree;
        let route = mode.suggest_route(Point::new(0, 0), Point::new(10, 4));
        // Should be H segment then 45° diagonal
        assert_eq!(route.len(), 2);
        assert_eq!(route[0].y, 0); // First point is on horizontal
        assert_eq!(route[1], Point::new(10, 4));
    }

    #[test]
    fn test_suggest_route_45_degree_more_vertical() {
        let mode = WireRoutingMode::FortyFiveDegree;
        let route = mode.suggest_route(Point::new(0, 0), Point::new(4, 10));
        // Should be V segment then 45° diagonal
        assert_eq!(route.len(), 2);
        assert_eq!(route[0].x, 0); // First point is on vertical
        assert_eq!(route[1], Point::new(4, 10));
    }

    // -------------------------------------------------------------------------
    // Route Optimization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_optimize_route_empty() {
        let result = optimize_route(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_optimize_route_single_point() {
        let result = optimize_route(&[Point::new(0, 0)]);
        assert_eq!(result, vec![Point::new(0, 0)]);
    }

    #[test]
    fn test_optimize_route_two_points() {
        let result = optimize_route(&[Point::new(0, 0), Point::new(10, 0)]);
        assert_eq!(result, vec![Point::new(0, 0), Point::new(10, 0)]);
    }

    #[test]
    fn test_optimize_route_removes_collinear_horizontal() {
        let points = vec![Point::new(0, 0), Point::new(5, 0), Point::new(10, 0)];
        let result = optimize_route(&points);
        assert_eq!(result, vec![Point::new(0, 0), Point::new(10, 0)]);
    }

    #[test]
    fn test_optimize_route_removes_collinear_vertical() {
        let points = vec![Point::new(0, 0), Point::new(0, 5), Point::new(0, 10)];
        let result = optimize_route(&points);
        assert_eq!(result, vec![Point::new(0, 0), Point::new(0, 10)]);
    }

    #[test]
    fn test_optimize_route_keeps_corners() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)];
        let result = optimize_route(&points);
        assert_eq!(result, points);
    }

    // -------------------------------------------------------------------------
    // Convert to Orthogonal Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_convert_to_orthogonal_already_orthogonal() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)];
        let result = convert_to_orthogonal(&points);
        assert_eq!(result, points);
    }

    #[test]
    fn test_convert_to_orthogonal_diagonal() {
        let points = vec![Point::new(0, 0), Point::new(10, 10)];
        let result = convert_to_orthogonal(&points);
        // Should convert to L-shape
        assert_eq!(result.len(), 3);
        assert!(result[1].x == 10 || result[1].y == 0);
    }

    // -------------------------------------------------------------------------
    // Route Length Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_route_length_empty() {
        assert!((route_length(&[]) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_route_length_single_horizontal() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        assert!((route_length(&points) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_route_length_l_shape() {
        let points = vec![Point::new(0, 0), Point::new(3, 0), Point::new(3, 4)];
        // 3 + 4 = 7
        assert!((route_length(&points) - 7.0).abs() < 0.001);
    }

    #[test]
    fn test_route_length_diagonal() {
        let points = vec![Point::new(0, 0), Point::new(3, 4)];
        assert!((route_length(&points) - 5.0).abs() < 0.001);
    }

    // -------------------------------------------------------------------------
    // Count Bends Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_count_bends_straight_line() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        assert_eq!(count_bends(&points), 0);
    }

    #[test]
    fn test_count_bends_l_shape() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)];
        assert_eq!(count_bends(&points), 1);
    }

    #[test]
    fn test_count_bends_z_shape() {
        let points = vec![
            Point::new(0, 0),
            Point::new(5, 0),
            Point::new(5, 5),
            Point::new(10, 5),
        ];
        assert_eq!(count_bends(&points), 2);
    }

    // -------------------------------------------------------------------------
    // Route Validation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_valid_route_empty() {
        assert!(!is_valid_route(&[]));
    }

    #[test]
    fn test_is_valid_route_single_point() {
        assert!(!is_valid_route(&[Point::new(0, 0)]));
    }

    #[test]
    fn test_is_valid_route_valid() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        assert!(is_valid_route(&points));
    }

    #[test]
    fn test_is_valid_route_zero_length_segment() {
        let points = vec![Point::new(0, 0), Point::new(0, 0)];
        assert!(!is_valid_route(&points));
    }

    #[test]
    fn test_is_valid_route_zero_length_middle() {
        let points = vec![
            Point::new(0, 0),
            Point::new(5, 0),
            Point::new(5, 0), // Zero length
            Point::new(10, 0),
        ];
        assert!(!is_valid_route(&points));
    }
}
