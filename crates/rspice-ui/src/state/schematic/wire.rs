//! Wire Types
//!
//! Wire segments, wire drawing state, and wire connections.

use super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// Wire
// =============================================================================

/// A wire segment connecting two or more points
///
/// Wires represent electrical connections in the schematic.
/// Each wire is a polyline (sequence of connected points).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    /// Unique identifier within the schematic
    pub id: u64,

    /// Wire path (sequence of connected points)
    ///
    /// Points are in grid coordinates. The wire connects
    /// point[0] → point[1] → ... → point[n-1].
    pub points: Vec<Point>,
}

impl Wire {
    /// Create a new wire with the given points
    pub fn new(id: u64, points: Vec<Point>) -> Self {
        Self { id, points }
    }

    /// Create a two-point wire (single segment)
    pub fn segment(id: u64, start: Point, end: Point) -> Self {
        Self {
            id,
            points: vec![start, end],
        }
    }

    /// Check if wire contains a point (on any segment)
    pub fn contains_point(&self, p: Point) -> bool {
        // Check vertices
        if self.points.contains(&p) {
            return true;
        }

        // Check segments
        for segment in self.points.windows(2) {
            if Self::point_on_segment(p, segment[0], segment[1]) {
                return true;
            }
        }
        false
    }

    /// Check if point lies on a horizontal or vertical segment
    fn point_on_segment(p: Point, a: Point, b: Point) -> bool {
        // Horizontal segment
        if a.y == b.y && p.y == a.y {
            let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
            return p.x >= min_x && p.x <= max_x;
        }
        // Vertical segment
        if a.x == b.x && p.x == a.x {
            let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
            return p.y >= min_y && p.y <= max_y;
        }
        false
    }

    /// Get the start point (first point)
    pub fn start(&self) -> Option<Point> {
        self.points.first().copied()
    }

    /// Get the end point (last point)
    pub fn end(&self) -> Option<Point> {
        self.points.last().copied()
    }

    /// Get both endpoints
    pub fn endpoints(&self) -> (Option<Point>, Option<Point>) {
        (self.start(), self.end())
    }

    /// Check if this wire connects to another at any endpoint
    pub fn connects_to(&self, other: &Wire) -> bool {
        let self_endpoints = [self.start(), self.end()];
        let other_endpoints = [other.start(), other.end()];

        for se in &self_endpoints {
            for oe in &other_endpoints {
                if let (Some(s), Some(o)) = (se, oe) {
                    if s == o {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get the total length of the wire in grid units
    pub fn length(&self) -> i32 {
        let mut total = 0;
        for segment in self.points.windows(2) {
            total += segment[0].manhattan_distance(segment[1]);
        }
        total
    }

    /// Check if the wire is empty (no points or single point)
    pub fn is_empty(&self) -> bool {
        self.points.len() < 2
    }

    /// Get number of segments in the wire
    pub fn segment_count(&self) -> usize {
        if self.points.len() < 2 {
            0
        } else {
            self.points.len() - 1
        }
    }
}

// =============================================================================
// Wire Routing Mode
// =============================================================================

/// Wire routing mode for orthogonal drawing
///
/// Controls how the cursor position is connected to the last wire point
/// when drawing wires interactively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WireRoutingMode {
    /// Horizontal first, then vertical (L-shape: →↓)
    #[default]
    HorizontalFirst,
    /// Vertical first, then horizontal (inverted L-shape: ↓→)
    VerticalFirst,
}

impl WireRoutingMode {
    /// Toggle between routing modes
    pub fn toggle(self) -> Self {
        match self {
            WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
            WireRoutingMode::VerticalFirst => WireRoutingMode::HorizontalFirst,
        }
    }
}

// =============================================================================
// Wire Drawing State
// =============================================================================

/// Wire drawing state for interactive wire placement
///
/// Tracks the in-progress wire being drawn by the user.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WireDrawing {
    /// Points in the current wire being drawn (committed vertices)
    pub points: Vec<Point>,

    /// Whether currently drawing
    pub active: bool,

    /// Current mouse position for preview (grid-aligned)
    pub preview_pos: Option<Point>,

    /// Routing mode for orthogonal wires
    pub routing_mode: WireRoutingMode,
}

impl WireDrawing {
    /// Create a new empty wire drawing state
    pub fn new() -> Self {
        Self::default()
    }

    /// Get intermediate points for orthogonal routing from last point to target
    ///
    /// Returns the corner point for L-shaped routing.
    /// Returns None if points are already aligned (no corner needed).
    pub fn get_route_corner(&self, target: Point) -> Option<Point> {
        let last = self.points.last()?;
        if last.x == target.x || last.y == target.y {
            // Already aligned - no corner needed
            return None;
        }

        match self.routing_mode {
            WireRoutingMode::HorizontalFirst => {
                // Go horizontal first, then vertical
                Some(Point::new(target.x, last.y))
            }
            WireRoutingMode::VerticalFirst => {
                // Go vertical first, then horizontal
                Some(Point::new(last.x, target.y))
            }
        }
    }

    /// Get preview path from last committed point to mouse position
    ///
    /// Returns the path that would be drawn if the user clicked at the
    /// current preview position.
    pub fn get_preview_path(&self) -> Vec<Point> {
        let mut path = Vec::new();

        if let (Some(&last), Some(target)) = (self.points.last(), self.preview_pos) {
            path.push(last);

            if let Some(corner) = self.get_route_corner(target) {
                path.push(corner);
            }

            path.push(target);
        }

        path
    }

    /// Check if wire drawing is in progress
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get number of committed points
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// Clear the wire drawing state
    pub fn clear(&mut self) {
        self.points.clear();
        self.active = false;
        self.preview_pos = None;
    }
}

// =============================================================================
// Wire Connection
// =============================================================================

/// Represents a connection between a wire endpoint and a component terminal
///
/// Used for rubber-banding: when a component moves, connected wire
/// endpoints move with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireConnection {
    /// Wire ID
    pub wire_id: u64,

    /// Index in wire's points array (0 = start, len-1 = end)
    pub point_index: usize,

    /// Connected component ID
    pub component_id: u64,

    /// Terminal name ("+", "-", "C", "E", etc.)
    pub terminal_name: String,
}

impl WireConnection {
    /// Create a new wire connection
    pub fn new(
        wire_id: u64,
        point_index: usize,
        component_id: u64,
        terminal_name: impl Into<String>,
    ) -> Self {
        Self {
            wire_id,
            point_index,
            component_id,
            terminal_name: terminal_name.into(),
        }
    }

    /// Check if this connection is to the start of the wire
    pub fn is_start(&self) -> bool {
        self.point_index == 0
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_new() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        assert_eq!(wire.id, 1);
        assert_eq!(wire.points.len(), 2);
    }

    #[test]
    fn test_wire_segment() {
        let wire = Wire::segment(1, Point::new(0, 0), Point::new(10, 0));
        assert_eq!(wire.points.len(), 2);
        assert_eq!(wire.start(), Some(Point::new(0, 0)));
        assert_eq!(wire.end(), Some(Point::new(10, 0)));
    }

    #[test]
    fn test_wire_contains_point_vertex() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        assert!(wire.contains_point(Point::new(0, 0)));
        assert!(wire.contains_point(Point::new(10, 0)));
        assert!(wire.contains_point(Point::new(10, 10)));
    }

    #[test]
    fn test_wire_contains_point_on_segment() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        assert!(wire.contains_point(Point::new(5, 0))); // On horizontal segment
        assert!(!wire.contains_point(Point::new(5, 1))); // Off the segment
    }

    #[test]
    fn test_wire_contains_point_vertical_segment() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(0, 10)]);
        assert!(wire.contains_point(Point::new(0, 5))); // On vertical segment
        assert!(!wire.contains_point(Point::new(1, 5))); // Off the segment
    }

    #[test]
    fn test_wire_connects_to() {
        let wire1 = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        let wire2 = Wire::new(2, vec![Point::new(10, 0), Point::new(10, 10)]);
        let wire3 = Wire::new(3, vec![Point::new(20, 20), Point::new(30, 20)]);

        assert!(wire1.connects_to(&wire2)); // Share point (10, 0)
        assert!(!wire1.connects_to(&wire3)); // No shared points
    }

    #[test]
    fn test_wire_length() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 5)],
        );
        assert_eq!(wire.length(), 15); // 10 + 5
    }

    #[test]
    fn test_wire_is_empty() {
        assert!(Wire::new(1, vec![]).is_empty());
        assert!(Wire::new(1, vec![Point::new(0, 0)]).is_empty());
        assert!(!Wire::new(1, vec![Point::new(0, 0), Point::new(1, 0)]).is_empty());
    }

    #[test]
    fn test_wire_segment_count() {
        assert_eq!(Wire::new(1, vec![]).segment_count(), 0);
        assert_eq!(Wire::new(1, vec![Point::new(0, 0)]).segment_count(), 0);
        assert_eq!(
            Wire::new(1, vec![Point::new(0, 0), Point::new(1, 0)]).segment_count(),
            1
        );
        assert_eq!(
            Wire::new(
                1,
                vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1)]
            )
            .segment_count(),
            2
        );
    }

    #[test]
    fn test_routing_mode_toggle() {
        let mode = WireRoutingMode::HorizontalFirst;
        assert_eq!(mode.toggle(), WireRoutingMode::VerticalFirst);
        assert_eq!(mode.toggle().toggle(), WireRoutingMode::HorizontalFirst);
    }

    #[test]
    fn test_wire_drawing_get_route_corner_horizontal_first() {
        let mut wd = WireDrawing::new();
        wd.points.push(Point::new(0, 0));
        wd.routing_mode = WireRoutingMode::HorizontalFirst;

        // Target at (10, 5) - should route (0,0) → (10,0) → (10,5)
        let corner = wd.get_route_corner(Point::new(10, 5));
        assert_eq!(corner, Some(Point::new(10, 0)));
    }

    #[test]
    fn test_wire_drawing_get_route_corner_vertical_first() {
        let mut wd = WireDrawing::new();
        wd.points.push(Point::new(0, 0));
        wd.routing_mode = WireRoutingMode::VerticalFirst;

        // Target at (10, 5) - should route (0,0) → (0,5) → (10,5)
        let corner = wd.get_route_corner(Point::new(10, 5));
        assert_eq!(corner, Some(Point::new(0, 5)));
    }

    #[test]
    fn test_wire_drawing_get_route_corner_aligned() {
        let mut wd = WireDrawing::new();
        wd.points.push(Point::new(0, 0));

        // Target on same horizontal line - no corner needed
        assert_eq!(wd.get_route_corner(Point::new(10, 0)), None);

        // Target on same vertical line - no corner needed
        assert_eq!(wd.get_route_corner(Point::new(0, 10)), None);
    }

    #[test]
    fn test_wire_drawing_get_preview_path() {
        let mut wd = WireDrawing::new();
        wd.points.push(Point::new(0, 0));
        wd.preview_pos = Some(Point::new(10, 5));
        wd.routing_mode = WireRoutingMode::HorizontalFirst;

        let path = wd.get_preview_path();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Point::new(0, 0));
        assert_eq!(path[1], Point::new(10, 0)); // Corner
        assert_eq!(path[2], Point::new(10, 5));
    }

    #[test]
    fn test_wire_connection() {
        let conn = WireConnection::new(1, 0, 5, "+");
        assert_eq!(conn.wire_id, 1);
        assert_eq!(conn.point_index, 0);
        assert_eq!(conn.component_id, 5);
        assert_eq!(conn.terminal_name, "+");
        assert!(conn.is_start());
    }
}
