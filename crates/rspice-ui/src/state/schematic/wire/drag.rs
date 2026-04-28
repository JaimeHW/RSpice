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
        } else if self.target_index == points.len() - 1
            && let Some(last) = points.last_mut()
        {
            *last = target_pos;
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
