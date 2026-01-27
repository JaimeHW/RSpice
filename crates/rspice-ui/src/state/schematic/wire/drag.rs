//! Wire Drag Context
//!
//! State management for active wire drag operations, supporting
//! vertex, segment, endpoint, and rubber-band dragging modes.

use super::super::point::Point;
use super::types::{DragConstraint, SnapTarget, WireDragMode};

// =============================================================================
// Wire Drag Context
// =============================================================================

/// Context for an active wire drag operation
///
/// This structure tracks all state needed to perform a wire drag
/// operation while maintaining wire integrity and orthogonal constraints.
#[derive(Debug, Clone)]
pub struct WireDragContext {
    /// Wire being dragged
    pub wire_id: u64,
    /// Drag mode
    pub mode: WireDragMode,
    /// Constraint for movement
    pub constraint: DragConstraint,
    /// Index of vertex or segment being dragged
    pub target_index: usize,
    /// Original points of wire (before drag started)
    pub original_points: Vec<Point>,
    /// Starting mouse position in grid coordinates
    pub start_pos: Point,
    /// Current mouse position in grid coordinates
    pub current_pos: Point,
    /// Accumulated delta from start
    pub delta: Point,
    /// Snap target if any
    pub snap_target: Option<SnapTarget>,
    /// Whether orthogonal mode is enforced
    pub maintain_orthogonal: bool,
}

impl WireDragContext {
    /// Create a new drag context for moving a vertex
    pub fn new_vertex_drag(
        wire_id: u64,
        vertex_index: usize,
        original_points: Vec<Point>,
        start_pos: Point,
    ) -> Self {
        Self {
            wire_id,
            mode: WireDragMode::MoveVertex,
            constraint: DragConstraint::Free,
            target_index: vertex_index,
            original_points,
            start_pos,
            current_pos: start_pos,
            delta: Point::new(0, 0),
            snap_target: None,
            maintain_orthogonal: true,
        }
    }

    /// Create a new drag context for moving a segment
    pub fn new_segment_drag(
        wire_id: u64,
        segment_index: usize,
        original_points: Vec<Point>,
        start_pos: Point,
        is_horizontal: bool,
    ) -> Self {
        // For orthogonal segment dragging, constrain to perpendicular direction
        let constraint = if is_horizontal {
            DragConstraint::Vertical
        } else {
            DragConstraint::Horizontal
        };

        Self {
            wire_id,
            mode: WireDragMode::MoveSegmentOrthogonal,
            constraint,
            target_index: segment_index,
            original_points,
            start_pos,
            current_pos: start_pos,
            delta: Point::new(0, 0),
            snap_target: None,
            maintain_orthogonal: true,
        }
    }

    /// Create a drag context for moving entire wire
    pub fn new_whole_wire_drag(
        wire_id: u64,
        original_points: Vec<Point>,
        start_pos: Point,
    ) -> Self {
        Self {
            wire_id,
            mode: WireDragMode::MoveWhole,
            constraint: DragConstraint::Free,
            target_index: 0,
            original_points,
            start_pos,
            current_pos: start_pos,
            delta: Point::new(0, 0),
            snap_target: None,
            maintain_orthogonal: false,
        }
    }

    /// Create a drag context for stretching an endpoint
    pub fn new_endpoint_stretch(
        wire_id: u64,
        endpoint_index: usize,
        original_points: Vec<Point>,
        start_pos: Point,
    ) -> Self {
        Self {
            wire_id,
            mode: WireDragMode::StretchEndpoint,
            constraint: DragConstraint::Free,
            target_index: endpoint_index,
            original_points,
            start_pos,
            current_pos: start_pos,
            delta: Point::new(0, 0),
            snap_target: None,
            maintain_orthogonal: false,
        }
    }

    /// Create a drag context for rubber-band stretching (inserting a new vertex)
    pub fn new_rubber_band(
        wire_id: u64,
        segment_index: usize,
        original_points: Vec<Point>,
        start_pos: Point,
    ) -> Self {
        Self {
            wire_id,
            mode: WireDragMode::RubberBand,
            constraint: DragConstraint::Free,
            target_index: segment_index,
            original_points,
            start_pos,
            current_pos: start_pos,
            delta: Point::new(0, 0),
            snap_target: None,
            maintain_orthogonal: false,
        }
    }

    /// Update the drag with a new mouse position
    pub fn update(&mut self, new_pos: Point) {
        self.current_pos = new_pos;
        let raw_dx = new_pos.x - self.start_pos.x;
        let raw_dy = new_pos.y - self.start_pos.y;
        let (dx, dy) = self.constraint.apply(raw_dx, raw_dy);
        self.delta = Point::new(dx, dy);
    }

    /// Set a snap target
    pub fn set_snap_target(&mut self, target: Option<SnapTarget>) {
        self.snap_target = target;
    }

    /// Clear the snap target
    pub fn clear_snap_target(&mut self) {
        self.snap_target = None;
    }

    /// Check if currently snapped
    pub fn is_snapped(&self) -> bool {
        self.snap_target.is_some()
    }

    /// Get the target position (accounting for snap)
    pub fn target_position(&self) -> Point {
        if let Some(ref snap) = self.snap_target {
            snap.position()
        } else {
            Point::new(
                self.start_pos.x + self.delta.x,
                self.start_pos.y + self.delta.y,
            )
        }
    }

    /// Compute the new wire points based on current drag state
    pub fn compute_new_points(&self) -> Vec<Point> {
        match self.mode {
            WireDragMode::MoveWhole => {
                // Move all points by delta
                self.original_points
                    .iter()
                    .map(|p| Point::new(p.x + self.delta.x, p.y + self.delta.y))
                    .collect()
            }
            WireDragMode::MoveVertex => self.compute_vertex_drag_points(),
            WireDragMode::MoveSegmentOrthogonal => self.compute_segment_drag_points(),
            WireDragMode::StretchEndpoint => self.compute_endpoint_stretch_points(),
            WireDragMode::RubberBand => self.compute_rubber_band_points(),
        }
    }

    /// Compute points for vertex drag
    fn compute_vertex_drag_points(&self) -> Vec<Point> {
        let mut points = self.original_points.clone();
        if self.target_index >= points.len() {
            return points;
        }

        let target_pos = self.target_position();

        if self.maintain_orthogonal {
            // Move vertex while maintaining orthogonal neighbors
            points[self.target_index] = target_pos;

            // Adjust previous segment to maintain orthogonality
            if self.target_index > 0 {
                let prev = points[self.target_index - 1];
                if prev.x != target_pos.x && prev.y != target_pos.y {
                    // Need to insert bend point
                    // For now, just snap to horizontal or vertical
                    if (prev.x - target_pos.x).abs() < (prev.y - target_pos.y).abs() {
                        points[self.target_index] = Point::new(prev.x, target_pos.y);
                    } else {
                        points[self.target_index] = Point::new(target_pos.x, prev.y);
                    }
                }
            }
        } else {
            points[self.target_index] = target_pos;
        }

        points
    }

    /// Compute points for segment drag (orthogonal mode)
    fn compute_segment_drag_points(&self) -> Vec<Point> {
        let mut points = self.original_points.clone();
        if self.target_index + 1 >= points.len() {
            return points;
        }

        let seg_start = &self.original_points[self.target_index];
        let seg_end = &self.original_points[self.target_index + 1];

        // Move both segment endpoints by the constrained delta
        let new_start = Point::new(seg_start.x + self.delta.x, seg_start.y + self.delta.y);
        let new_end = Point::new(seg_end.x + self.delta.x, seg_end.y + self.delta.y);

        points[self.target_index] = new_start;
        points[self.target_index + 1] = new_end;

        points
    }

    /// Compute points for endpoint stretch
    fn compute_endpoint_stretch_points(&self) -> Vec<Point> {
        let mut points = self.original_points.clone();
        let target_pos = self.target_position();

        if self.target_index == 0 {
            points[0] = target_pos;
        } else if self.target_index == points.len() - 1 {
            if let Some(last) = points.last_mut() {
                *last = target_pos;
            }
        }

        points
    }

    /// Compute points for rubber-band (midpoint stretch)
    fn compute_rubber_band_points(&self) -> Vec<Point> {
        let mut points = self.original_points.clone();
        if self.target_index + 1 >= points.len() {
            return points;
        }

        let target_pos = self.target_position();

        // Insert the drag point as a new vertex, creating a "rubber band" effect
        // This creates two new segments from the original segment
        points.insert(self.target_index + 1, target_pos);

        points
    }

    /// Check if this is an endpoint being dragged
    pub fn is_endpoint_drag(&self) -> bool {
        self.target_index == 0 || self.target_index == self.original_points.len() - 1
    }

    /// Get the total displacement from start position
    pub fn total_displacement(&self) -> (i32, i32) {
        (self.delta.x, self.delta.y)
    }

    /// Check if the wire has been modified (non-zero displacement)
    pub fn has_moved(&self) -> bool {
        self.delta.x != 0 || self.delta.y != 0
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_wire_points() -> Vec<Point> {
        vec![
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(10, 10),
            Point::new(20, 10),
        ]
    }

    // -------------------------------------------------------------------------
    // Construction Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_new_vertex_drag() {
        let ctx = WireDragContext::new_vertex_drag(1, 2, sample_wire_points(), Point::new(10, 10));

        assert_eq!(ctx.wire_id, 1);
        assert_eq!(ctx.target_index, 2);
        assert_eq!(ctx.mode, WireDragMode::MoveVertex);
        assert_eq!(ctx.constraint, DragConstraint::Free);
        assert!(ctx.maintain_orthogonal);
    }

    #[test]
    fn test_new_segment_drag_horizontal() {
        let ctx = WireDragContext::new_segment_drag(
            1,
            0,
            sample_wire_points(),
            Point::new(5, 0),
            true, // horizontal segment
        );

        assert_eq!(ctx.mode, WireDragMode::MoveSegmentOrthogonal);
        assert_eq!(ctx.constraint, DragConstraint::Vertical);
    }

    #[test]
    fn test_new_segment_drag_vertical() {
        let ctx = WireDragContext::new_segment_drag(
            1,
            1,
            sample_wire_points(),
            Point::new(10, 5),
            false, // vertical segment
        );

        assert_eq!(ctx.constraint, DragConstraint::Horizontal);
    }

    #[test]
    fn test_new_whole_wire_drag() {
        let ctx = WireDragContext::new_whole_wire_drag(1, sample_wire_points(), Point::new(0, 0));

        assert_eq!(ctx.mode, WireDragMode::MoveWhole);
        assert_eq!(ctx.constraint, DragConstraint::Free);
        assert!(!ctx.maintain_orthogonal);
    }

    #[test]
    fn test_new_endpoint_stretch() {
        let ctx =
            WireDragContext::new_endpoint_stretch(1, 0, sample_wire_points(), Point::new(0, 0));

        assert_eq!(ctx.mode, WireDragMode::StretchEndpoint);
        assert_eq!(ctx.target_index, 0);
    }

    #[test]
    fn test_new_rubber_band() {
        let ctx = WireDragContext::new_rubber_band(1, 1, sample_wire_points(), Point::new(10, 5));

        assert_eq!(ctx.mode, WireDragMode::RubberBand);
        assert_eq!(ctx.target_index, 1);
    }

    // -------------------------------------------------------------------------
    // Update Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_update_free_constraint() {
        let mut ctx =
            WireDragContext::new_whole_wire_drag(1, sample_wire_points(), Point::new(0, 0));

        ctx.update(Point::new(5, 7));

        assert_eq!(ctx.current_pos, Point::new(5, 7));
        assert_eq!(ctx.delta, Point::new(5, 7));
    }

    #[test]
    fn test_update_horizontal_constraint() {
        let mut ctx = WireDragContext::new_segment_drag(
            1,
            1,
            sample_wire_points(),
            Point::new(10, 5),
            false, // vertical segment → horizontal constraint
        );

        ctx.update(Point::new(20, 15));

        assert_eq!(ctx.delta.x, 10);
        assert_eq!(ctx.delta.y, 0); // Constrained to horizontal
    }

    #[test]
    fn test_update_vertical_constraint() {
        let mut ctx = WireDragContext::new_segment_drag(
            1,
            0,
            sample_wire_points(),
            Point::new(5, 0),
            true, // horizontal segment → vertical constraint
        );

        ctx.update(Point::new(15, 10));

        assert_eq!(ctx.delta.x, 0); // Constrained to vertical
        assert_eq!(ctx.delta.y, 10);
    }

    // -------------------------------------------------------------------------
    // Snap Target Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_snap_target_management() {
        let mut ctx =
            WireDragContext::new_vertex_drag(1, 0, sample_wire_points(), Point::new(0, 0));

        assert!(!ctx.is_snapped());

        let target = SnapTarget::GridPoint {
            position: Point::new(50, 50),
        };
        ctx.set_snap_target(Some(target));

        assert!(ctx.is_snapped());
        assert_eq!(ctx.target_position(), Point::new(50, 50));

        ctx.clear_snap_target();
        assert!(!ctx.is_snapped());
    }

    #[test]
    fn test_target_position_without_snap() {
        let mut ctx =
            WireDragContext::new_whole_wire_drag(1, sample_wire_points(), Point::new(10, 10));

        ctx.update(Point::new(25, 30));

        assert_eq!(ctx.target_position(), Point::new(25, 30));
    }

    #[test]
    fn test_target_position_with_snap() {
        let mut ctx =
            WireDragContext::new_vertex_drag(1, 0, sample_wire_points(), Point::new(0, 0));

        ctx.update(Point::new(45, 47));

        let target = SnapTarget::GridPoint {
            position: Point::new(50, 50),
        };
        ctx.set_snap_target(Some(target));

        // Snap overrides computed position
        assert_eq!(ctx.target_position(), Point::new(50, 50));
    }

    // -------------------------------------------------------------------------
    // Compute Points Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_compute_points_move_whole() {
        let mut ctx =
            WireDragContext::new_whole_wire_drag(1, sample_wire_points(), Point::new(0, 0));

        ctx.update(Point::new(5, 5));

        let new_points = ctx.compute_new_points();

        assert_eq!(new_points.len(), 4);
        assert_eq!(new_points[0], Point::new(5, 5));
        assert_eq!(new_points[1], Point::new(15, 5));
        assert_eq!(new_points[2], Point::new(15, 15));
        assert_eq!(new_points[3], Point::new(25, 15));
    }

    #[test]
    fn test_compute_points_segment_drag() {
        let mut ctx = WireDragContext::new_segment_drag(
            1,
            0,
            sample_wire_points(),
            Point::new(5, 0),
            true, // horizontal segment
        );

        ctx.update(Point::new(5, 5)); // Only Y changes due to constraint

        let new_points = ctx.compute_new_points();

        assert_eq!(new_points[0], Point::new(0, 5));
        assert_eq!(new_points[1], Point::new(10, 5));
        // Points 2 and 3 unchanged
        assert_eq!(new_points[2], Point::new(10, 10));
        assert_eq!(new_points[3], Point::new(20, 10));
    }

    #[test]
    fn test_compute_points_rubber_band() {
        let mut ctx =
            WireDragContext::new_rubber_band(1, 0, sample_wire_points(), Point::new(5, 0));

        ctx.update(Point::new(5, 5));

        let new_points = ctx.compute_new_points();

        // Should have inserted a new point
        assert_eq!(new_points.len(), 5);
        assert_eq!(new_points[0], Point::new(0, 0));
        assert_eq!(new_points[1], Point::new(5, 5)); // Inserted point
        assert_eq!(new_points[2], Point::new(10, 0));
    }

    #[test]
    fn test_compute_points_endpoint_stretch_start() {
        let mut ctx =
            WireDragContext::new_endpoint_stretch(1, 0, sample_wire_points(), Point::new(0, 0));

        ctx.update(Point::new(-5, -5));

        let new_points = ctx.compute_new_points();

        assert_eq!(new_points[0], Point::new(-5, -5));
        // Other points unchanged
        assert_eq!(new_points[1], Point::new(10, 0));
    }

    #[test]
    fn test_compute_points_endpoint_stretch_end() {
        let points = sample_wire_points();
        let end_index = points.len() - 1;
        let mut ctx =
            WireDragContext::new_endpoint_stretch(1, end_index, points, Point::new(20, 10));

        ctx.update(Point::new(30, 20));

        let new_points = ctx.compute_new_points();

        assert_eq!(new_points.last(), Some(&Point::new(30, 20)));
    }

    // -------------------------------------------------------------------------
    // Utility Method Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_endpoint_drag_start() {
        let ctx = WireDragContext::new_vertex_drag(1, 0, sample_wire_points(), Point::new(0, 0));

        assert!(ctx.is_endpoint_drag());
    }

    #[test]
    fn test_is_endpoint_drag_end() {
        let ctx = WireDragContext::new_vertex_drag(1, 3, sample_wire_points(), Point::new(20, 10));

        assert!(ctx.is_endpoint_drag());
    }

    #[test]
    fn test_is_endpoint_drag_middle() {
        let ctx = WireDragContext::new_vertex_drag(1, 1, sample_wire_points(), Point::new(10, 0));

        assert!(!ctx.is_endpoint_drag());
    }

    #[test]
    fn test_has_moved() {
        let mut ctx =
            WireDragContext::new_whole_wire_drag(1, sample_wire_points(), Point::new(0, 0));

        assert!(!ctx.has_moved());

        ctx.update(Point::new(0, 0));
        assert!(!ctx.has_moved());

        ctx.update(Point::new(1, 0));
        assert!(ctx.has_moved());
    }

    #[test]
    fn test_total_displacement() {
        let mut ctx =
            WireDragContext::new_whole_wire_drag(1, sample_wire_points(), Point::new(10, 10));

        ctx.update(Point::new(25, 15));

        assert_eq!(ctx.total_displacement(), (15, 5));
    }
}
