//! Wire Drawing State
//!
//! Interactive wire placement state machine for schematic editors.
//! Tracks in-progress wire drawing with preview and routing mode support.

use super::super::point::Point;
use super::routing::WireRoutingMode;
use serde::{Deserialize, Serialize};

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

    /// Start drawing a wire at the given position
    pub fn start(&mut self, pos: Point) {
        self.points.clear();
        self.points.push(pos);
        self.active = true;
        self.preview_pos = Some(pos);
    }

    /// Add a point to the current wire
    ///
    /// Also adds any intermediate corner points based on routing mode.
    pub fn add_point(&mut self, pos: Point) {
        if !self.active || self.points.is_empty() {
            return;
        }

        // Add corner point if needed for orthogonal routing
        if let Some(corner) = self.get_route_corner(pos) {
            self.points.push(corner);
        }

        self.points.push(pos);
        self.preview_pos = Some(pos);
    }

    /// Update the preview position
    pub fn update_preview(&mut self, pos: Point) {
        if self.active {
            self.preview_pos = Some(pos);
        }
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
            WireRoutingMode::Diagonal => {
                // No corner needed - direct line
                None
            }
            WireRoutingMode::FortyFiveDegree => {
                // Use the 45-degree routing: return first intermediate point
                let route = self.routing_mode.suggest_route(*last, target);
                if route.len() > 1 {
                    // Return the first intermediate point (before final target)
                    Some(route[0])
                } else {
                    None
                }
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

    /// Get the full wire path including preview
    pub fn get_full_path(&self) -> Vec<Point> {
        let mut path = self.points.clone();

        if let Some(target) = self.preview_pos
            && let Some(&last) = self.points.last()
            && last != target
        {
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

    /// Check if the wire has enough points to be valid
    pub fn is_valid(&self) -> bool {
        self.points.len() >= 2
    }

    /// Toggle the routing mode
    pub fn toggle_routing_mode(&mut self) {
        self.routing_mode = self.routing_mode.toggle();
    }

    /// Toggle only between orthogonal routing modes
    pub fn toggle_orthogonal_mode(&mut self) {
        self.routing_mode = self.routing_mode.toggle_orthogonal();
    }

    /// Set the routing mode
    pub fn set_routing_mode(&mut self, mode: WireRoutingMode) {
        self.routing_mode = mode;
    }

    /// Finish the wire and return the points
    ///
    /// Returns None if the wire is not valid (< 2 points).
    pub fn finish(&mut self) -> Option<Vec<Point>> {
        if !self.is_valid() {
            self.clear();
            return None;
        }

        let points = std::mem::take(&mut self.points);
        self.clear();
        Some(points)
    }

    /// Finish the wire including the current preview position
    pub fn finish_at(&mut self, pos: Point) -> Option<Vec<Point>> {
        if self.active && !self.points.is_empty() {
            self.add_point(pos);
        }
        self.finish()
    }

    /// Clear the wire drawing state
    pub fn clear(&mut self) {
        self.points.clear();
        self.active = false;
        self.preview_pos = None;
    }

    /// Cancel the current wire drawing
    pub fn cancel(&mut self) {
        self.clear();
    }

    /// Undo the last point (backspace)
    ///
    /// Returns true if a point was removed, false if no points left.
    pub fn undo_last_point(&mut self) -> bool {
        if self.points.len() > 1 {
            self.points.pop();
            true
        } else {
            false
        }
    }

    /// Get the starting point of the wire
    pub fn start_point(&self) -> Option<Point> {
        self.points.first().copied()
    }

    /// Get the last committed point
    pub fn last_point(&self) -> Option<Point> {
        self.points.last().copied()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Construction Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_drawing_default() {
        let drawing = WireDrawing::new();
        assert!(!drawing.is_active());
        assert_eq!(drawing.point_count(), 0);
        assert!(!drawing.is_valid());
    }

    #[test]
    fn test_wire_drawing_start() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(10, 20));

        assert!(drawing.is_active());
        assert_eq!(drawing.point_count(), 1);
        assert_eq!(drawing.start_point(), Some(Point::new(10, 20)));
    }

    // -------------------------------------------------------------------------
    // Point Management Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_drawing_add_point() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 0));

        assert_eq!(drawing.point_count(), 2);
        assert!(drawing.is_valid());
    }

    #[test]
    fn test_wire_drawing_add_point_with_corner() {
        let mut drawing = WireDrawing::new();
        drawing.routing_mode = WireRoutingMode::HorizontalFirst;
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 10));

        // Should have 3 points: start, corner, end
        assert_eq!(drawing.point_count(), 3);
        assert_eq!(drawing.points[1], Point::new(10, 0)); // Corner
    }

    #[test]
    fn test_wire_drawing_add_point_diagonal_mode() {
        let mut drawing = WireDrawing::new();
        drawing.routing_mode = WireRoutingMode::Diagonal;
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 10));

        // Should have only 2 points: start, end (no corner)
        assert_eq!(drawing.point_count(), 2);
    }

    #[test]
    fn test_wire_drawing_undo() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 0));
        drawing.add_point(Point::new(20, 0));

        assert_eq!(drawing.point_count(), 3);

        assert!(drawing.undo_last_point());
        assert_eq!(drawing.point_count(), 2);

        assert!(drawing.undo_last_point());
        assert_eq!(drawing.point_count(), 1);

        // Can't undo below 1 point
        assert!(!drawing.undo_last_point());
        assert_eq!(drawing.point_count(), 1);
    }

    // -------------------------------------------------------------------------
    // Preview Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_drawing_preview() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));
        drawing.update_preview(Point::new(10, 0));

        assert_eq!(drawing.preview_pos, Some(Point::new(10, 0)));
    }

    #[test]
    fn test_wire_drawing_preview_path_horizontal() {
        let mut drawing = WireDrawing::new();
        drawing.routing_mode = WireRoutingMode::HorizontalFirst;
        drawing.start(Point::new(0, 0));
        drawing.update_preview(Point::new(10, 0));

        let path = drawing.get_preview_path();
        assert_eq!(path, vec![Point::new(0, 0), Point::new(10, 0)]);
    }

    #[test]
    fn test_wire_drawing_preview_path_l_shape() {
        let mut drawing = WireDrawing::new();
        drawing.routing_mode = WireRoutingMode::HorizontalFirst;
        drawing.start(Point::new(0, 0));
        drawing.update_preview(Point::new(10, 10));

        let path = drawing.get_preview_path();
        assert_eq!(
            path,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)]
        );
    }

    // -------------------------------------------------------------------------
    // Routing Mode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_drawing_toggle_routing_mode() {
        let mut drawing = WireDrawing::new();
        assert_eq!(drawing.routing_mode, WireRoutingMode::HorizontalFirst);

        drawing.toggle_routing_mode();
        assert_eq!(drawing.routing_mode, WireRoutingMode::VerticalFirst);

        drawing.toggle_routing_mode();
        assert_eq!(drawing.routing_mode, WireRoutingMode::Diagonal);
    }

    #[test]
    fn test_wire_drawing_toggle_orthogonal_mode() {
        let mut drawing = WireDrawing::new();
        assert_eq!(drawing.routing_mode, WireRoutingMode::HorizontalFirst);

        drawing.toggle_orthogonal_mode();
        assert_eq!(drawing.routing_mode, WireRoutingMode::VerticalFirst);

        drawing.toggle_orthogonal_mode();
        assert_eq!(drawing.routing_mode, WireRoutingMode::HorizontalFirst);
    }

    #[test]
    fn test_wire_drawing_set_routing_mode() {
        let mut drawing = WireDrawing::new();
        drawing.set_routing_mode(WireRoutingMode::FortyFiveDegree);
        assert_eq!(drawing.routing_mode, WireRoutingMode::FortyFiveDegree);
    }

    // -------------------------------------------------------------------------
    // Finish/Cancel Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_drawing_finish_valid() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 0));

        let points = drawing.finish();
        assert!(points.is_some());
        assert_eq!(points.unwrap().len(), 2);
        assert!(!drawing.is_active());
    }

    #[test]
    fn test_wire_drawing_finish_invalid() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));

        let points = drawing.finish();
        assert!(points.is_none());
        assert!(!drawing.is_active());
    }

    #[test]
    fn test_wire_drawing_finish_at() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));

        let points = drawing.finish_at(Point::new(10, 0));
        assert!(points.is_some());
        let pts = points.unwrap();
        assert_eq!(pts.len(), 2);
        assert_eq!(pts[1], Point::new(10, 0));
    }

    #[test]
    fn test_wire_drawing_cancel() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 0));

        drawing.cancel();

        assert!(!drawing.is_active());
        assert_eq!(drawing.point_count(), 0);
    }

    #[test]
    fn test_wire_drawing_clear() {
        let mut drawing = WireDrawing::new();
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 0));

        drawing.clear();

        assert!(!drawing.is_active());
        assert_eq!(drawing.point_count(), 0);
        assert!(drawing.preview_pos.is_none());
    }

    // -------------------------------------------------------------------------
    // Full Path Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_drawing_full_path() {
        let mut drawing = WireDrawing::new();
        drawing.routing_mode = WireRoutingMode::HorizontalFirst;
        drawing.start(Point::new(0, 0));
        drawing.add_point(Point::new(10, 0));
        drawing.update_preview(Point::new(10, 10));

        let path = drawing.get_full_path();
        assert_eq!(
            path,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)]
        );
    }

    #[test]
    fn test_wire_drawing_full_path_with_corner() {
        let mut drawing = WireDrawing::new();
        drawing.routing_mode = WireRoutingMode::HorizontalFirst;
        drawing.start(Point::new(0, 0));
        drawing.update_preview(Point::new(10, 10));

        let path = drawing.get_full_path();
        assert_eq!(
            path,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)]
        );
    }
}
