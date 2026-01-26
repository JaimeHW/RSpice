//! Wire Types
//!
//! Wire segments, wire drawing state, and wire connections.
//!
//! This module provides robust wire handling with:
//! - Individual segment representation with geometric metadata
//! - Advanced hit testing with tolerance-based picking
//! - Vertex manipulation for corner editing
//! - Segment intersection and distance calculations

use super::point::Point;
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
        self.start.manhattan_distance(self.end)
    }

    /// Get the Euclidean length of the segment
    pub fn euclidean_length(&self) -> f64 {
        let dx = (self.end.x - self.start.x) as f64;
        let dy = (self.end.y - self.start.y) as f64;
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

    /// Check if a point lies exactly on this segment (for orthogonal segments)
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
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let px = p.x - self.start.x;
        let py = p.y - self.start.y;

        // Check collinearity: cross product should be zero
        if px * dy != py * dx {
            return false;
        }

        // Check if within parametric bounds [0, 1]
        if dx != 0 {
            let t = px as f64 / dx as f64;
            return (0.0..=1.0).contains(&t);
        }
        if dy != 0 {
            let t = py as f64 / dy as f64;
            return (0.0..=1.0).contains(&t);
        }

        false
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
            let dx = (p.x - self.start.x) as f64;
            let dy = (p.y - self.start.y) as f64;
            return dx * dx + dy * dy;
        }

        let dx = (self.end.x - self.start.x) as f64;
        let dy = (self.end.y - self.start.y) as f64;
        let px = (p.x - self.start.x) as f64;
        let py = (p.y - self.start.y) as f64;

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

        let dx = (self.end.x - self.start.x) as f64;
        let dy = (self.end.y - self.start.y) as f64;
        let px = (p.x - self.start.x) as f64;
        let py = (p.y - self.start.y) as f64;

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

        let dx1 = (b.x - a.x) as i64;
        let dy1 = (b.y - a.y) as i64;
        let dx2 = (d.x - c.x) as i64;
        let dy2 = (d.y - c.y) as i64;

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

        let dx3 = (c.x - a.x) as i64;
        let dy3 = (c.y - a.y) as i64;

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
        let dx1 = self.end.x - self.start.x;
        let dy1 = self.end.y - self.start.y;
        let dx2 = other.end.x - other.start.x;
        let dy2 = other.end.y - other.start.y;

        // Cross product for collinearity check
        if (dx1 as i64) * (dy2 as i64) != (dy1 as i64) * (dx2 as i64) {
            return false;
        }

        // Check if segments are on the same line
        let dx3 = other.start.x - self.start.x;
        let dy3 = other.start.y - self.start.y;
        if (dx1 as i64) * (dy3 as i64) != (dy1 as i64) * (dx3 as i64) {
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
        let dx = (self.end.x - self.start.x) as f64 / len;
        let dy = (self.end.y - self.start.y) as f64 / len;
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
// Wire Drag Operations (Robust)
// =============================================================================

/// Mode of wire dragging operation
///
/// Different drag modes produce different geometric behaviors,
/// similar to how EDA tools handle wire manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WireDragMode {
    /// Move entire wire (translate all points)
    MoveWhole,
    /// Move single vertex (corner point), maintaining connectivity
    MoveVertex,
    /// Move segment while adjusting adjacent segments to maintain orthogonality
    MoveSegmentOrthogonal,
    /// Stretch endpoint to new position
    StretchEndpoint,
    /// Rubber-band: stretch segment while preserving endpoint positions
    RubberBand,
}

/// Direction constraint for wire segment movement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragConstraint {
    /// No constraint - move freely
    Free,
    /// Constrain to horizontal movement only
    Horizontal,
    /// Constrain to vertical movement only
    Vertical,
    /// Snap to 45-degree angles
    Angle45,
    /// Maintain orthogonal wire structure
    Orthogonal,
}

impl DragConstraint {
    /// Apply constraint to a delta movement
    pub fn apply(&self, dx: i32, dy: i32) -> (i32, i32) {
        match self {
            DragConstraint::Free => (dx, dy),
            DragConstraint::Horizontal => (dx, 0),
            DragConstraint::Vertical => (0, dy),
            DragConstraint::Angle45 => {
                // Snap to nearest 45-degree direction
                let abs_dx = dx.abs();
                let abs_dy = dy.abs();
                if abs_dx > abs_dy * 2 {
                    (dx, 0) // Horizontal
                } else if abs_dy > abs_dx * 2 {
                    (0, dy) // Vertical
                } else {
                    // Diagonal - use smaller magnitude for both
                    let mag = abs_dx.min(abs_dy);
                    (dx.signum() * mag, dy.signum() * mag)
                }
            }
            DragConstraint::Orthogonal => {
                // Choose horizontal or vertical based on larger magnitude
                if dx.abs() >= dy.abs() {
                    (dx, 0)
                } else {
                    (0, dy)
                }
            }
        }
    }
}

/// Target for wire endpoint snapping
///
/// Represents a potential snap target that wire endpoints
/// can connect to during drawing or manipulation.
#[derive(Debug, Clone, PartialEq)]
pub enum SnapTarget {
    /// Snap to a component terminal
    Terminal {
        /// Component ID
        component_id: u64,
        /// Terminal name (e.g., "+", "-", "G", "D", "S")
        terminal_name: String,
        /// Terminal position in grid coordinates
        position: Point,
    },
    /// Snap to an existing wire vertex
    WireVertex {
        /// Wire ID
        wire_id: u64,
        /// Vertex index
        vertex_index: usize,
        /// Vertex position
        position: Point,
    },
    /// Snap to a point on a wire segment (will create junction)
    WireSegment {
        /// Wire ID
        wire_id: u64,
        /// Segment index
        segment_index: usize,
        /// Snap position on segment
        position: Point,
    },
    /// Snap to grid point
    GridPoint {
        /// Grid position
        position: Point,
    },
    /// Snap to existing junction
    Junction {
        /// Junction ID
        junction_id: u64,
        /// Junction position
        position: Point,
    },
}

impl SnapTarget {
    /// Get the position of this snap target
    pub fn position(&self) -> Point {
        match self {
            SnapTarget::Terminal { position, .. } => *position,
            SnapTarget::WireVertex { position, .. } => *position,
            SnapTarget::WireSegment { position, .. } => *position,
            SnapTarget::GridPoint { position } => *position,
            SnapTarget::Junction { position, .. } => *position,
        }
    }

    /// Get priority for snap target selection (lower = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            SnapTarget::Terminal { .. } => 0, // Highest priority
            SnapTarget::Junction { .. } => 1,
            SnapTarget::WireVertex { .. } => 2,
            SnapTarget::WireSegment { .. } => 3,
            SnapTarget::GridPoint { .. } => 4, // Lowest priority
        }
    }

    /// Check if this is a terminal snap
    pub fn is_terminal(&self) -> bool {
        matches!(self, SnapTarget::Terminal { .. })
    }

    /// Check if this is a wire snap (vertex or segment)
    pub fn is_wire(&self) -> bool {
        matches!(
            self,
            SnapTarget::WireVertex { .. } | SnapTarget::WireSegment { .. }
        )
    }
}

/// Type of wire junction
///
/// Junctions are classified by how many wires meet at a point,
/// which affects both electrical connectivity and visual rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JunctionType {
    /// No junction - two collinear wire segments (pass-through)
    PassThrough,
    /// L-junction - two wires at 90 degrees (corner)
    Corner,
    /// T-junction - three wires meeting (electrical connection)
    TJunction,
    /// Cross junction - four wires meeting (electrical connection)
    CrossJunction,
    /// Complex junction - more than 4 wires meeting
    Complex { wire_count: usize },
}

impl JunctionType {
    /// Determine junction type from number of connected wire segments
    pub fn from_wire_count(count: usize) -> Self {
        match count {
            0 | 1 => JunctionType::PassThrough,
            2 => JunctionType::Corner, // Could be pass-through or corner
            3 => JunctionType::TJunction,
            4 => JunctionType::CrossJunction,
            n => JunctionType::Complex { wire_count: n },
        }
    }

    /// Check if this junction represents an electrical connection
    /// (not just a corner or pass-through)
    pub fn is_electrical_connection(&self) -> bool {
        matches!(
            self,
            JunctionType::TJunction | JunctionType::CrossJunction | JunctionType::Complex { .. }
        )
    }

    /// Get the number of wires at this junction
    pub fn wire_count(&self) -> usize {
        match self {
            JunctionType::PassThrough => 1,
            JunctionType::Corner => 2,
            JunctionType::TJunction => 3,
            JunctionType::CrossJunction => 4,
            JunctionType::Complex { wire_count } => *wire_count,
        }
    }
}

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
}

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

    // =========================================================================
    // Segment-Level Operations (Robust)
    // =========================================================================

    /// Get an iterator over all segments of the wire
    pub fn segments(&self) -> impl Iterator<Item = WireSegment> + '_ {
        self.points.windows(2).map(|w| WireSegment::new(w[0], w[1]))
    }

    /// Get a specific segment by index
    /// Returns None if index is out of bounds
    pub fn segment_at(&self, index: usize) -> Option<WireSegment> {
        if index < self.segment_count() {
            Some(WireSegment::new(self.points[index], self.points[index + 1]))
        } else {
            None
        }
    }

    /// Find the segment containing a point and its index
    /// Returns (segment_index, segment) if found
    pub fn segment_containing_point(&self, p: Point) -> Option<(usize, WireSegment)> {
        for (idx, segment) in self.segments().enumerate() {
            if segment.contains_point(p) {
                return Some((idx, segment));
            }
        }
        None
    }

    /// Find the segment closest to a point within tolerance
    /// Returns (segment_index, segment, distance) if within tolerance
    pub fn segment_near_point(
        &self,
        p: Point,
        tolerance: f64,
    ) -> Option<(usize, WireSegment, f64)> {
        let mut best: Option<(usize, WireSegment, f64)> = None;

        for (idx, segment) in self.segments().enumerate() {
            let dist = segment.distance_to_point(p);
            if dist <= tolerance {
                match &best {
                    None => best = Some((idx, segment, dist)),
                    Some((_, _, best_dist)) if dist < *best_dist => {
                        best = Some((idx, segment, dist));
                    }
                    _ => {}
                }
            }
        }

        best
    }

    // =========================================================================
    // Vertex Operations (for corner manipulation)
    // =========================================================================

    /// Get a vertex by index
    pub fn vertex_at(&self, index: usize) -> Option<Point> {
        self.points.get(index).copied()
    }

    /// Get the number of vertices in the wire
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.points.len()
    }

    /// Insert a vertex at the specified index
    /// The index must be in range [0, vertex_count]
    pub fn insert_vertex(&mut self, index: usize, point: Point) {
        if index <= self.points.len() {
            self.points.insert(index, point);
        }
    }

    /// Remove a vertex at the specified index
    /// Returns the removed point, or None if index is invalid
    /// Will not remove if it would make the wire invalid (< 2 points)
    pub fn remove_vertex(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() && self.points.len() > 2 {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    /// Move a vertex to a new position
    /// Returns true if successful
    pub fn move_vertex(&mut self, index: usize, new_pos: Point) -> bool {
        if index < self.points.len() {
            self.points[index] = new_pos;
            true
        } else {
            false
        }
    }

    /// Check if a vertex is near a point within tolerance
    /// Returns (vertex_index, vertex_point, distance) if found
    pub fn vertex_near(&self, p: Point, tolerance: i32) -> Option<(usize, Point, f64)> {
        let tolerance_f = tolerance as f64;
        let mut best: Option<(usize, Point, f64)> = None;

        for (idx, vertex) in self.points.iter().enumerate() {
            let dx = (p.x - vertex.x) as f64;
            let dy = (p.y - vertex.y) as f64;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= tolerance_f {
                match &best {
                    None => best = Some((idx, *vertex, dist)),
                    Some((_, _, best_dist)) if dist < *best_dist => {
                        best = Some((idx, *vertex, dist));
                    }
                    _ => {}
                }
            }
        }

        best
    }

    // =========================================================================
    // Bounding Box and Geometry
    // =========================================================================

    /// Get the axis-aligned bounding box of the wire
    /// Returns (min_point, max_point) or None if wire is empty
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for p in &self.points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Some((Point::new(min_x, min_y), Point::new(max_x, max_y)))
    }

    /// Check if all segments of this wire are orthogonal
    pub fn is_orthogonal(&self) -> bool {
        self.segments().all(|s| s.is_orthogonal())
    }

    /// Get the total Euclidean length of the wire
    pub fn euclidean_length(&self) -> f64 {
        self.segments().map(|s| s.euclidean_length()).sum()
    }

    /// Find the closest point on the wire to a given position
    pub fn closest_point(&self, p: Point) -> Option<(Point, f64)> {
        let mut best: Option<(Point, f64)> = None;

        for segment in self.segments() {
            let closest = segment.closest_point(p);
            let dist = segment.distance_to_point(p);

            match &best {
                None => best = Some((closest, dist)),
                Some((_, best_dist)) if dist < *best_dist => {
                    best = Some((closest, dist));
                }
                _ => {}
            }
        }

        best
    }

    // =========================================================================
    // Advanced Hit Testing
    // =========================================================================

    /// Perform hit testing on the wire with tolerance
    ///
    /// Returns a WireHitResult indicating what was hit:
    /// - Vertex: A corner point was hit (for corner manipulation)
    /// - Segment: A wire segment was hit (for segment selection/manipulation)
    /// - None: Nothing within tolerance
    ///
    /// Vertices have priority over segments at the same location
    pub fn hit_test(&self, pos: Point, tolerance: f64) -> WireHitResult {
        // First check vertices (higher priority for manipulation)
        if let Some((idx, vertex, dist)) = self.vertex_near(pos, tolerance as i32) {
            return WireHitResult::Vertex {
                index: idx,
                point: vertex,
                distance: dist,
            };
        }

        // Then check segments
        if let Some((idx, segment, dist)) = self.segment_near_point(pos, tolerance) {
            return WireHitResult::Segment {
                index: idx,
                closest_point: segment.closest_point(pos),
                distance: dist,
                segment,
            };
        }

        WireHitResult::None
    }

    /// Hit test with separate tolerances for vertices and segments
    /// This allows for larger vertex hit zones for easier corner manipulation
    pub fn hit_test_with_tolerances(
        &self,
        pos: Point,
        vertex_tolerance: f64,
        segment_tolerance: f64,
    ) -> WireHitResult {
        // First check vertices with vertex tolerance
        if let Some((idx, vertex, dist)) = self.vertex_near(pos, vertex_tolerance as i32) {
            return WireHitResult::Vertex {
                index: idx,
                point: vertex,
                distance: dist,
            };
        }

        // Then check segments with segment tolerance
        if let Some((idx, segment, dist)) = self.segment_near_point(pos, segment_tolerance) {
            return WireHitResult::Segment {
                index: idx,
                closest_point: segment.closest_point(pos),
                distance: dist,
                segment,
            };
        }

        WireHitResult::None
    }

    /// Check if this wire intersects with a segment
    /// Returns all intersection points
    pub fn intersections_with_segment(&self, seg: &WireSegment) -> Vec<Point> {
        let mut intersections = Vec::new();
        for wire_seg in self.segments() {
            if let Some(p) = wire_seg.intersection(seg) {
                // Avoid duplicates at vertices
                if !intersections.contains(&p) {
                    intersections.push(p);
                }
            }
        }
        intersections
    }

    /// Check if this wire intersects with another wire
    /// Returns all intersection points
    pub fn intersections_with_wire(&self, other: &Wire) -> Vec<Point> {
        let mut intersections = Vec::new();
        for seg in other.segments() {
            for p in self.intersections_with_segment(&seg) {
                if !intersections.contains(&p) {
                    intersections.push(p);
                }
            }
        }
        intersections
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

    // =========================================================================
    // WireSegment Tests - Comprehensive Robust Coverage
    // =========================================================================

    #[test]
    fn test_wire_segment_new() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert_eq!(seg.start, Point::new(0, 0));
        assert_eq!(seg.end, Point::new(10, 0));
    }

    #[test]
    fn test_wire_segment_length_horizontal() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert_eq!(seg.length(), 10);
    }

    #[test]
    fn test_wire_segment_length_vertical() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(0, 15));
        assert_eq!(seg.length(), 15);
    }

    #[test]
    fn test_wire_segment_length_diagonal() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(3, 4));
        assert_eq!(seg.length(), 7); // Manhattan: 3 + 4
    }

    #[test]
    fn test_wire_segment_euclidean_length() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(3, 4));
        assert!((seg.euclidean_length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_euclidean_length_horizontal() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert!((seg.euclidean_length() - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_midpoint() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert_eq!(seg.midpoint(), Point::new(5, 0));
    }

    #[test]
    fn test_wire_segment_midpoint_odd_length() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(11, 0));
        // Integer division: (0 + 11) / 2 = 5
        assert_eq!(seg.midpoint(), Point::new(5, 0));
    }

    #[test]
    fn test_wire_segment_midpoint_vertical() {
        let seg = WireSegment::new(Point::new(5, 0), Point::new(5, 20));
        assert_eq!(seg.midpoint(), Point::new(5, 10));
    }

    #[test]
    fn test_wire_segment_bounding_box_horizontal() {
        let seg = WireSegment::new(Point::new(10, 5), Point::new(0, 5));
        let (min, max) = seg.bounding_box();
        assert_eq!(min, Point::new(0, 5));
        assert_eq!(max, Point::new(10, 5));
    }

    #[test]
    fn test_wire_segment_bounding_box_vertical() {
        let seg = WireSegment::new(Point::new(5, 20), Point::new(5, 0));
        let (min, max) = seg.bounding_box();
        assert_eq!(min, Point::new(5, 0));
        assert_eq!(max, Point::new(5, 20));
    }

    #[test]
    fn test_wire_segment_bounding_box_diagonal() {
        let seg = WireSegment::new(Point::new(10, 20), Point::new(5, 10));
        let (min, max) = seg.bounding_box();
        assert_eq!(min, Point::new(5, 10));
        assert_eq!(max, Point::new(10, 20));
    }

    #[test]
    fn test_wire_segment_is_horizontal() {
        assert!(WireSegment::new(Point::new(0, 5), Point::new(10, 5)).is_horizontal());
        assert!(!WireSegment::new(Point::new(0, 0), Point::new(0, 10)).is_horizontal());
        assert!(!WireSegment::new(Point::new(0, 0), Point::new(5, 5)).is_horizontal());
    }

    #[test]
    fn test_wire_segment_is_vertical() {
        assert!(WireSegment::new(Point::new(5, 0), Point::new(5, 10)).is_vertical());
        assert!(!WireSegment::new(Point::new(0, 5), Point::new(10, 5)).is_vertical());
        assert!(!WireSegment::new(Point::new(0, 0), Point::new(5, 5)).is_vertical());
    }

    #[test]
    fn test_wire_segment_is_orthogonal() {
        assert!(WireSegment::new(Point::new(0, 5), Point::new(10, 5)).is_orthogonal());
        assert!(WireSegment::new(Point::new(5, 0), Point::new(5, 10)).is_orthogonal());
        assert!(!WireSegment::new(Point::new(0, 0), Point::new(5, 5)).is_orthogonal());
    }

    #[test]
    fn test_wire_segment_is_zero_length() {
        assert!(WireSegment::new(Point::new(5, 5), Point::new(5, 5)).is_zero_length());
        assert!(!WireSegment::new(Point::new(0, 0), Point::new(1, 0)).is_zero_length());
    }

    #[test]
    fn test_wire_segment_contains_point_horizontal() {
        let seg = WireSegment::new(Point::new(0, 5), Point::new(10, 5));
        assert!(seg.contains_point(Point::new(0, 5))); // Start
        assert!(seg.contains_point(Point::new(10, 5))); // End
        assert!(seg.contains_point(Point::new(5, 5))); // Middle
        assert!(!seg.contains_point(Point::new(5, 6))); // Off-line
        assert!(!seg.contains_point(Point::new(-1, 5))); // Before start
        assert!(!seg.contains_point(Point::new(11, 5))); // After end
    }

    #[test]
    fn test_wire_segment_contains_point_vertical() {
        let seg = WireSegment::new(Point::new(5, 0), Point::new(5, 10));
        assert!(seg.contains_point(Point::new(5, 0))); // Start
        assert!(seg.contains_point(Point::new(5, 10))); // End
        assert!(seg.contains_point(Point::new(5, 5))); // Middle
        assert!(!seg.contains_point(Point::new(6, 5))); // Off-line
        assert!(!seg.contains_point(Point::new(5, -1))); // Before start
        assert!(!seg.contains_point(Point::new(5, 11))); // After end
    }

    #[test]
    fn test_wire_segment_contains_point_diagonal() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 10));
        assert!(seg.contains_point(Point::new(0, 0)));
        assert!(seg.contains_point(Point::new(10, 10)));
        assert!(seg.contains_point(Point::new(5, 5)));
        assert!(!seg.contains_point(Point::new(5, 6))); // Off diagonal
    }

    #[test]
    fn test_wire_segment_contains_point_zero_length() {
        let seg = WireSegment::new(Point::new(5, 5), Point::new(5, 5));
        assert!(seg.contains_point(Point::new(5, 5)));
        assert!(!seg.contains_point(Point::new(5, 6)));
    }

    #[test]
    fn test_wire_segment_contains_point_with_tolerance() {
        let seg = WireSegment::new(Point::new(0, 5), Point::new(10, 5));
        assert!(seg.contains_point_with_tolerance(Point::new(5, 5), 1));
        assert!(seg.contains_point_with_tolerance(Point::new(5, 6), 2));
        assert!(!seg.contains_point_with_tolerance(Point::new(5, 8), 2));
    }

    #[test]
    fn test_wire_segment_distance_to_point_on_segment() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert!(seg.distance_to_point(Point::new(5, 0)).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_distance_to_point_perpendicular() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert!((seg.distance_to_point(Point::new(5, 3)) - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_distance_to_point_past_end() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        // Point at (15, 0) is 5 units past the end
        assert!((seg.distance_to_point(Point::new(15, 0)) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_distance_to_point_past_start() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        // Point at (-5, 0) is 5 units before the start
        assert!((seg.distance_to_point(Point::new(-5, 0)) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_closest_point_on_segment() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert_eq!(seg.closest_point(Point::new(5, 3)), Point::new(5, 0));
    }

    #[test]
    fn test_wire_segment_closest_point_past_end() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert_eq!(seg.closest_point(Point::new(15, 3)), Point::new(10, 0));
    }

    #[test]
    fn test_wire_segment_closest_point_past_start() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert_eq!(seg.closest_point(Point::new(-5, 3)), Point::new(0, 0));
    }

    #[test]
    fn test_wire_segment_intersection_perpendicular() {
        let seg1 = WireSegment::new(Point::new(0, 5), Point::new(10, 5));
        let seg2 = WireSegment::new(Point::new(5, 0), Point::new(5, 10));
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(5, 5)));
    }

    #[test]
    fn test_wire_segment_intersection_no_intersect() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(5, 0));
        let seg2 = WireSegment::new(Point::new(6, 0), Point::new(10, 0));
        assert_eq!(seg1.intersection(&seg2), None);
    }

    #[test]
    fn test_wire_segment_intersection_parallel() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let seg2 = WireSegment::new(Point::new(0, 5), Point::new(10, 5));
        assert_eq!(seg1.intersection(&seg2), None);
    }

    #[test]
    fn test_wire_segment_intersection_t_junction() {
        let seg1 = WireSegment::new(Point::new(0, 5), Point::new(10, 5));
        let seg2 = WireSegment::new(Point::new(5, 5), Point::new(5, 10));
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(5, 5)));
    }

    #[test]
    fn test_wire_segment_intersection_at_endpoint() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(5, 5));
        let seg2 = WireSegment::new(Point::new(5, 5), Point::new(10, 0));
        assert_eq!(seg1.intersection(&seg2), Some(Point::new(5, 5)));
    }

    #[test]
    fn test_wire_segment_overlaps_collinear() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let seg2 = WireSegment::new(Point::new(5, 0), Point::new(15, 0));
        assert!(seg1.overlaps(&seg2));
    }

    #[test]
    fn test_wire_segment_overlaps_no_collinear() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(4, 0));
        let seg2 = WireSegment::new(Point::new(6, 0), Point::new(10, 0));
        assert!(!seg1.overlaps(&seg2));
    }

    #[test]
    fn test_wire_segment_overlaps_different_lines() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let seg2 = WireSegment::new(Point::new(0, 5), Point::new(10, 5));
        assert!(!seg1.overlaps(&seg2));
    }

    #[test]
    fn test_wire_segment_direction_horizontal() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let (dx, dy) = seg.direction();
        assert!((dx - 1.0).abs() < 0.001);
        assert!(dy.abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_direction_vertical() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(0, 10));
        let (dx, dy) = seg.direction();
        assert!(dx.abs() < 0.001);
        assert!((dy - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_direction_zero_length() {
        let seg = WireSegment::new(Point::new(5, 5), Point::new(5, 5));
        let (dx, dy) = seg.direction();
        assert!(dx.abs() < 0.001);
        assert!(dy.abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_perpendicular() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let (px, py) = seg.perpendicular();
        assert!(px.abs() < 0.001);
        assert!((py - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_segment_reversed() {
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 5));
        let rev = seg.reversed();
        assert_eq!(rev.start, Point::new(10, 5));
        assert_eq!(rev.end, Point::new(0, 0));
    }

    #[test]
    fn test_wire_segment_shares_endpoint() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let seg2 = WireSegment::new(Point::new(10, 0), Point::new(10, 10));
        let seg3 = WireSegment::new(Point::new(20, 0), Point::new(30, 0));
        assert!(seg1.shares_endpoint(&seg2));
        assert!(!seg1.shares_endpoint(&seg3));
    }

    #[test]
    fn test_wire_segment_shared_endpoint() {
        let seg1 = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        let seg2 = WireSegment::new(Point::new(10, 0), Point::new(10, 10));
        let seg3 = WireSegment::new(Point::new(20, 0), Point::new(30, 0));
        assert_eq!(seg1.shared_endpoint(&seg2), Some(Point::new(10, 0)));
        assert_eq!(seg1.shared_endpoint(&seg3), None);
    }

    // =========================================================================
    // WireHitResult Tests
    // =========================================================================

    #[test]
    fn test_wire_hit_result_is_hit() {
        let vertex = WireHitResult::Vertex {
            index: 0,
            point: Point::new(0, 0),
            distance: 0.0,
        };
        let segment = WireHitResult::Segment {
            index: 0,
            closest_point: Point::new(5, 0),
            distance: 1.0,
            segment: WireSegment::new(Point::new(0, 0), Point::new(10, 0)),
        };
        let none = WireHitResult::None;

        assert!(vertex.is_hit());
        assert!(segment.is_hit());
        assert!(!none.is_hit());
    }

    #[test]
    fn test_wire_hit_result_distance() {
        let vertex = WireHitResult::Vertex {
            index: 0,
            point: Point::new(0, 0),
            distance: 1.5,
        };
        let none = WireHitResult::None;

        assert!((vertex.distance() - 1.5).abs() < 0.001);
        assert!(none.distance().is_infinite());
    }

    #[test]
    fn test_wire_hit_result_point() {
        let vertex = WireHitResult::Vertex {
            index: 0,
            point: Point::new(5, 5),
            distance: 0.0,
        };
        let none = WireHitResult::None;

        assert_eq!(vertex.point(), Some(Point::new(5, 5)));
        assert_eq!(none.point(), None);
    }

    // =========================================================================
    // Wire Enhanced Methods Tests
    // =========================================================================

    #[test]
    fn test_wire_segments_iterator() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let segments: Vec<_> = wire.segments().collect();
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].start, Point::new(0, 0));
        assert_eq!(segments[0].end, Point::new(10, 0));
        assert_eq!(segments[1].start, Point::new(10, 0));
        assert_eq!(segments[1].end, Point::new(10, 10));
    }

    #[test]
    fn test_wire_segments_iterator_empty() {
        let wire = Wire::new(1, vec![Point::new(0, 0)]);
        let segments: Vec<_> = wire.segments().collect();
        assert_eq!(segments.len(), 0);
    }

    #[test]
    fn test_wire_segment_at() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let seg0 = wire.segment_at(0).unwrap();
        let seg1 = wire.segment_at(1).unwrap();
        assert_eq!(seg0.start, Point::new(0, 0));
        assert_eq!(seg1.end, Point::new(10, 10));
        assert!(wire.segment_at(2).is_none());
    }

    #[test]
    fn test_wire_segment_containing_point() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let (idx, seg) = wire.segment_containing_point(Point::new(5, 0)).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(seg.start, Point::new(0, 0));

        let (idx2, _) = wire.segment_containing_point(Point::new(10, 5)).unwrap();
        assert_eq!(idx2, 1);

        assert!(wire.segment_containing_point(Point::new(5, 5)).is_none());
    }

    #[test]
    fn test_wire_segment_near_point() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        // Point 2 units above the segment
        let result = wire.segment_near_point(Point::new(5, 2), 3.0);
        assert!(result.is_some());
        let (idx, _, dist) = result.unwrap();
        assert_eq!(idx, 0);
        assert!((dist - 2.0).abs() < 0.001);

        // Point too far
        assert!(wire.segment_near_point(Point::new(5, 10), 5.0).is_none());
    }

    #[test]
    fn test_wire_vertex_at() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        assert_eq!(wire.vertex_at(0), Some(Point::new(0, 0)));
        assert_eq!(wire.vertex_at(1), Some(Point::new(10, 0)));
        assert_eq!(wire.vertex_at(2), Some(Point::new(10, 10)));
        assert_eq!(wire.vertex_at(3), None);
    }

    #[test]
    fn test_wire_vertex_count() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        assert_eq!(wire.vertex_count(), 3);
    }

    #[test]
    fn test_wire_insert_vertex() {
        let mut wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        wire.insert_vertex(1, Point::new(5, 5));
        assert_eq!(wire.vertex_count(), 3);
        assert_eq!(wire.vertex_at(1), Some(Point::new(5, 5)));
    }

    #[test]
    fn test_wire_insert_vertex_at_end() {
        let mut wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        wire.insert_vertex(2, Point::new(20, 0));
        assert_eq!(wire.vertex_count(), 3);
        assert_eq!(wire.vertex_at(2), Some(Point::new(20, 0)));
    }

    #[test]
    fn test_wire_remove_vertex() {
        let mut wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(5, 0), Point::new(10, 0)],
        );
        let removed = wire.remove_vertex(1);
        assert_eq!(removed, Some(Point::new(5, 0)));
        assert_eq!(wire.vertex_count(), 2);
    }

    #[test]
    fn test_wire_remove_vertex_invalid() {
        let mut wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        // Can't remove - would leave < 2 vertices
        assert!(wire.remove_vertex(0).is_none());
        assert_eq!(wire.vertex_count(), 2);
    }

    #[test]
    fn test_wire_move_vertex() {
        let mut wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        assert!(wire.move_vertex(1, Point::new(15, 5)));
        assert_eq!(wire.vertex_at(1), Some(Point::new(15, 5)));
    }

    #[test]
    fn test_wire_move_vertex_invalid_index() {
        let mut wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        assert!(!wire.move_vertex(5, Point::new(15, 5)));
    }

    #[test]
    fn test_wire_vertex_near() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let result = wire.vertex_near(Point::new(1, 1), 2);
        assert!(result.is_some());
        let (idx, pt, dist) = result.unwrap();
        assert_eq!(idx, 0);
        assert_eq!(pt, Point::new(0, 0));
        assert!((dist - 1.414).abs() < 0.1); // sqrt(2)
    }

    #[test]
    fn test_wire_vertex_near_no_match() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        assert!(wire.vertex_near(Point::new(100, 100), 5).is_none());
    }

    #[test]
    fn test_wire_bounding_box() {
        let wire = Wire::new(
            1,
            vec![
                Point::new(5, 10),
                Point::new(20, 10),
                Point::new(20, 30),
                Point::new(0, 30),
            ],
        );
        let (min, max) = wire.bounding_box().unwrap();
        assert_eq!(min, Point::new(0, 10));
        assert_eq!(max, Point::new(20, 30));
    }

    #[test]
    fn test_wire_bounding_box_empty() {
        let wire = Wire::new(1, vec![]);
        assert!(wire.bounding_box().is_none());
    }

    #[test]
    fn test_wire_is_orthogonal() {
        let orthogonal = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let diagonal = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 10)]);

        assert!(orthogonal.is_orthogonal());
        assert!(!diagonal.is_orthogonal());
    }

    #[test]
    fn test_wire_euclidean_length() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(3, 4)]);
        assert!((wire.euclidean_length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_closest_point() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let (closest, dist) = wire.closest_point(Point::new(5, 3)).unwrap();
        assert_eq!(closest, Point::new(5, 0));
        assert!((dist - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_hit_test_vertex() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        let result = wire.hit_test(Point::new(0, 0), 2.0);
        match result {
            WireHitResult::Vertex { index, point, .. } => {
                assert_eq!(index, 0);
                assert_eq!(point, Point::new(0, 0));
            }
            _ => panic!("Expected vertex hit"),
        }
    }

    #[test]
    fn test_wire_hit_test_segment() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        let result = wire.hit_test(Point::new(5, 1), 2.0);
        match result {
            WireHitResult::Segment { index, .. } => {
                assert_eq!(index, 0);
            }
            _ => panic!("Expected segment hit"),
        }
    }

    #[test]
    fn test_wire_hit_test_none() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        let result = wire.hit_test(Point::new(5, 10), 2.0);
        assert!(!result.is_hit());
    }

    #[test]
    fn test_wire_hit_test_vertex_priority() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        // Point exactly at vertex should return vertex, not segment
        let result = wire.hit_test(Point::new(0, 0), 2.0);
        assert!(matches!(result, WireHitResult::Vertex { .. }));
    }

    #[test]
    fn test_wire_hit_test_with_tolerances() {
        let wire = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        // Larger vertex tolerance should capture near-vertex points
        let result = wire.hit_test_with_tolerances(Point::new(1, 1), 3.0, 1.0);
        assert!(matches!(result, WireHitResult::Vertex { .. }));
    }

    #[test]
    fn test_wire_intersections_with_segment() {
        let wire = Wire::new(
            1,
            vec![Point::new(0, 5), Point::new(10, 5), Point::new(10, 15)],
        );
        let seg = WireSegment::new(Point::new(5, 0), Point::new(5, 10));
        let intersections = wire.intersections_with_segment(&seg);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0], Point::new(5, 5));
    }

    #[test]
    fn test_wire_intersections_with_wire() {
        let wire1 = Wire::new(1, vec![Point::new(0, 5), Point::new(20, 5)]);
        let wire2 = Wire::new(
            2,
            vec![
                Point::new(5, 0),
                Point::new(5, 10),
                Point::new(15, 10),
                Point::new(15, 0),
            ],
        );
        let intersections = wire1.intersections_with_wire(&wire2);
        assert_eq!(intersections.len(), 2);
        assert!(intersections.contains(&Point::new(5, 5)));
        assert!(intersections.contains(&Point::new(15, 5)));
    }

    #[test]
    fn test_wire_intersections_no_intersect() {
        let wire1 = Wire::new(1, vec![Point::new(0, 0), Point::new(10, 0)]);
        let wire2 = Wire::new(2, vec![Point::new(0, 10), Point::new(10, 10)]);
        let intersections = wire1.intersections_with_wire(&wire2);
        assert!(intersections.is_empty());
    }

    // =========================================================================
    // DragConstraint Tests
    // =========================================================================

    #[test]
    fn test_drag_constraint_free() {
        let constraint = DragConstraint::Free;
        assert_eq!(constraint.apply(5, 10), (5, 10));
        assert_eq!(constraint.apply(-3, 7), (-3, 7));
    }

    #[test]
    fn test_drag_constraint_horizontal() {
        let constraint = DragConstraint::Horizontal;
        assert_eq!(constraint.apply(5, 10), (5, 0));
        assert_eq!(constraint.apply(-3, -7), (-3, 0));
    }

    #[test]
    fn test_drag_constraint_vertical() {
        let constraint = DragConstraint::Vertical;
        assert_eq!(constraint.apply(5, 10), (0, 10));
        assert_eq!(constraint.apply(-3, -7), (0, -7));
    }

    #[test]
    fn test_drag_constraint_angle45_horizontal() {
        let constraint = DragConstraint::Angle45;
        // Large horizontal movement should snap to horizontal
        assert_eq!(constraint.apply(10, 2), (10, 0));
    }

    #[test]
    fn test_drag_constraint_angle45_vertical() {
        let constraint = DragConstraint::Angle45;
        // Large vertical movement should snap to vertical
        assert_eq!(constraint.apply(2, 10), (0, 10));
    }

    #[test]
    fn test_drag_constraint_angle45_diagonal() {
        let constraint = DragConstraint::Angle45;
        // Equal movement snaps to diagonal
        let (dx, dy) = constraint.apply(5, 5);
        assert_eq!(dx.abs(), dy.abs());
    }

    #[test]
    fn test_drag_constraint_orthogonal_horizontal_dominant() {
        let constraint = DragConstraint::Orthogonal;
        assert_eq!(constraint.apply(10, 5), (10, 0));
    }

    #[test]
    fn test_drag_constraint_orthogonal_vertical_dominant() {
        let constraint = DragConstraint::Orthogonal;
        assert_eq!(constraint.apply(3, 8), (0, 8));
    }

    #[test]
    fn test_drag_constraint_orthogonal_equal() {
        let constraint = DragConstraint::Orthogonal;
        // When equal, should prefer horizontal
        let (dx, dy) = constraint.apply(5, 5);
        assert!(dx == 5 || dy == 5);
    }

    // =========================================================================
    // SnapTarget Tests
    // =========================================================================

    #[test]
    fn test_snap_target_terminal_position() {
        let target = SnapTarget::Terminal {
            component_id: 1,
            terminal_name: "pin1".to_string(),
            position: Point::new(10, 20),
        };
        assert_eq!(target.position(), Point::new(10, 20));
        assert!(target.is_terminal());
        assert!(!target.is_wire());
        assert_eq!(target.priority(), 0);
    }

    #[test]
    fn test_snap_target_wire_vertex() {
        let target = SnapTarget::WireVertex {
            wire_id: 5,
            vertex_index: 2,
            position: Point::new(30, 40),
        };
        assert_eq!(target.position(), Point::new(30, 40));
        assert!(!target.is_terminal());
        assert!(target.is_wire());
        assert_eq!(target.priority(), 2);
    }

    #[test]
    fn test_snap_target_wire_segment() {
        let target = SnapTarget::WireSegment {
            wire_id: 7,
            segment_index: 1,
            position: Point::new(15, 25),
        };
        assert_eq!(target.position(), Point::new(15, 25));
        assert!(target.is_wire());
        assert_eq!(target.priority(), 3);
    }

    #[test]
    fn test_snap_target_grid_point() {
        let target = SnapTarget::GridPoint {
            position: Point::new(50, 60),
        };
        assert_eq!(target.position(), Point::new(50, 60));
        assert!(!target.is_terminal());
        assert!(!target.is_wire());
        assert_eq!(target.priority(), 4);
    }

    #[test]
    fn test_snap_target_junction() {
        let target = SnapTarget::Junction {
            junction_id: 99,
            position: Point::new(70, 80),
        };
        assert_eq!(target.position(), Point::new(70, 80));
        assert_eq!(target.priority(), 1);
    }

    #[test]
    fn test_snap_target_priority_ordering() {
        let terminal = SnapTarget::Terminal {
            component_id: 1,
            terminal_name: "p".to_string(),
            position: Point::new(0, 0),
        };
        let junction = SnapTarget::Junction {
            junction_id: 1,
            position: Point::new(0, 0),
        };
        let vertex = SnapTarget::WireVertex {
            wire_id: 1,
            vertex_index: 0,
            position: Point::new(0, 0),
        };
        let segment = SnapTarget::WireSegment {
            wire_id: 1,
            segment_index: 0,
            position: Point::new(0, 0),
        };
        let grid = SnapTarget::GridPoint {
            position: Point::new(0, 0),
        };

        assert!(terminal.priority() < junction.priority());
        assert!(junction.priority() < vertex.priority());
        assert!(vertex.priority() < segment.priority());
        assert!(segment.priority() < grid.priority());
    }

    // =========================================================================
    // JunctionType Tests
    // =========================================================================

    #[test]
    fn test_junction_type_from_wire_count() {
        assert_eq!(JunctionType::from_wire_count(0), JunctionType::PassThrough);
        assert_eq!(JunctionType::from_wire_count(1), JunctionType::PassThrough);
        assert_eq!(JunctionType::from_wire_count(2), JunctionType::Corner);
        assert_eq!(JunctionType::from_wire_count(3), JunctionType::TJunction);
        assert_eq!(
            JunctionType::from_wire_count(4),
            JunctionType::CrossJunction
        );
        assert_eq!(
            JunctionType::from_wire_count(6),
            JunctionType::Complex { wire_count: 6 }
        );
    }

    #[test]
    fn test_junction_type_is_electrical_connection() {
        assert!(!JunctionType::PassThrough.is_electrical_connection());
        assert!(!JunctionType::Corner.is_electrical_connection());
        assert!(JunctionType::TJunction.is_electrical_connection());
        assert!(JunctionType::CrossJunction.is_electrical_connection());
        assert!(JunctionType::Complex { wire_count: 5 }.is_electrical_connection());
    }

    #[test]
    fn test_junction_type_wire_count() {
        assert_eq!(JunctionType::PassThrough.wire_count(), 1);
        assert_eq!(JunctionType::Corner.wire_count(), 2);
        assert_eq!(JunctionType::TJunction.wire_count(), 3);
        assert_eq!(JunctionType::CrossJunction.wire_count(), 4);
        assert_eq!(JunctionType::Complex { wire_count: 7 }.wire_count(), 7);
    }

    // =========================================================================
    // WireDragContext Tests
    // =========================================================================

    #[test]
    fn test_wire_drag_context_new_vertex_drag() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)];
        let ctx = WireDragContext::new_vertex_drag(1, 1, points.clone(), Point::new(10, 0));

        assert_eq!(ctx.wire_id, 1);
        assert_eq!(ctx.mode, WireDragMode::MoveVertex);
        assert_eq!(ctx.target_index, 1);
        assert_eq!(ctx.original_points, points);
        assert!(ctx.maintain_orthogonal);
    }

    #[test]
    fn test_wire_drag_context_new_segment_drag_horizontal() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let ctx = WireDragContext::new_segment_drag(1, 0, points, Point::new(5, 0), true);

        assert_eq!(ctx.mode, WireDragMode::MoveSegmentOrthogonal);
        assert_eq!(ctx.constraint, DragConstraint::Vertical);
    }

    #[test]
    fn test_wire_drag_context_new_segment_drag_vertical() {
        let points = vec![Point::new(0, 0), Point::new(0, 10)];
        let ctx = WireDragContext::new_segment_drag(1, 0, points, Point::new(0, 5), false);

        assert_eq!(ctx.constraint, DragConstraint::Horizontal);
    }

    #[test]
    fn test_wire_drag_context_new_whole_wire_drag() {
        let points = vec![Point::new(0, 0), Point::new(10, 10)];
        let ctx = WireDragContext::new_whole_wire_drag(1, points, Point::new(5, 5));

        assert_eq!(ctx.mode, WireDragMode::MoveWhole);
        assert_eq!(ctx.constraint, DragConstraint::Free);
        assert!(!ctx.maintain_orthogonal);
    }

    #[test]
    fn test_wire_drag_context_update() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let mut ctx = WireDragContext::new_whole_wire_drag(1, points, Point::new(0, 0));

        ctx.update(Point::new(5, 3));
        assert_eq!(ctx.current_pos, Point::new(5, 3));
        assert_eq!(ctx.delta, Point::new(5, 3));
    }

    #[test]
    fn test_wire_drag_context_update_with_constraint() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let mut ctx = WireDragContext::new_segment_drag(1, 0, points, Point::new(0, 0), true);

        // Horizontal segment, so movement is constrained to vertical
        ctx.update(Point::new(5, 8));
        assert_eq!(ctx.delta, Point::new(0, 8)); // X constrained to 0
    }

    #[test]
    fn test_wire_drag_context_target_position_no_snap() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let mut ctx = WireDragContext::new_whole_wire_drag(1, points, Point::new(0, 0));
        ctx.update(Point::new(5, 3));

        assert_eq!(ctx.target_position(), Point::new(5, 3));
    }

    #[test]
    fn test_wire_drag_context_target_position_with_snap() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let mut ctx = WireDragContext::new_whole_wire_drag(1, points, Point::new(0, 0));
        ctx.update(Point::new(7, 4));
        ctx.set_snap_target(Some(SnapTarget::GridPoint {
            position: Point::new(10, 5),
        }));

        assert_eq!(ctx.target_position(), Point::new(10, 5)); // Snapped position
    }

    #[test]
    fn test_wire_drag_context_compute_move_whole() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let mut ctx = WireDragContext::new_whole_wire_drag(1, points, Point::new(0, 0));
        ctx.update(Point::new(5, 5));

        let new_points = ctx.compute_new_points();
        assert_eq!(new_points, vec![Point::new(5, 5), Point::new(15, 5)]);
    }

    #[test]
    fn test_wire_drag_context_compute_segment_drag() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)];
        let mut ctx = WireDragContext::new_segment_drag(1, 0, points, Point::new(5, 0), true);
        ctx.update(Point::new(5, 5));

        let new_points = ctx.compute_new_points();
        // Segment 0 moved vertically by 5
        assert_eq!(new_points[0], Point::new(0, 5));
        assert_eq!(new_points[1], Point::new(10, 5));
        // Third point unchanged
        assert_eq!(new_points[2], Point::new(10, 10));
    }

    #[test]
    fn test_wire_drag_context_is_endpoint_drag() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)];

        let ctx1 = WireDragContext::new_vertex_drag(1, 0, points.clone(), Point::new(0, 0));
        assert!(ctx1.is_endpoint_drag());

        let ctx2 = WireDragContext::new_vertex_drag(1, 2, points.clone(), Point::new(10, 10));
        assert!(ctx2.is_endpoint_drag());

        let ctx3 = WireDragContext::new_vertex_drag(1, 1, points, Point::new(10, 0));
        assert!(!ctx3.is_endpoint_drag());
    }

    #[test]
    fn test_wire_drag_context_compute_endpoint_stretch() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let mut ctx = WireDragContext {
            wire_id: 1,
            mode: WireDragMode::StretchEndpoint,
            constraint: DragConstraint::Free,
            target_index: 1,
            original_points: points,
            start_pos: Point::new(10, 0),
            current_pos: Point::new(15, 5),
            delta: Point::new(5, 5),
            snap_target: None,
            maintain_orthogonal: false,
        };

        let new_points = ctx.compute_new_points();
        assert_eq!(new_points[0], Point::new(0, 0)); // Start unchanged
        assert_eq!(new_points[1], Point::new(15, 5)); // End stretched
    }

    #[test]
    fn test_wire_drag_context_compute_rubber_band() {
        let points = vec![Point::new(0, 0), Point::new(20, 0)];
        let mut ctx = WireDragContext {
            wire_id: 1,
            mode: WireDragMode::RubberBand,
            constraint: DragConstraint::Free,
            target_index: 0,
            original_points: points,
            start_pos: Point::new(10, 0),
            current_pos: Point::new(10, 10),
            delta: Point::new(0, 10),
            snap_target: None,
            maintain_orthogonal: false,
        };

        let new_points = ctx.compute_new_points();
        assert_eq!(new_points.len(), 3); // New point inserted
        assert_eq!(new_points[0], Point::new(0, 0));
        assert_eq!(new_points[1], Point::new(10, 10)); // Rubber band point
        assert_eq!(new_points[2], Point::new(20, 0));
    }

    #[test]
    fn test_wire_drag_context_out_of_bounds_vertex() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let ctx = WireDragContext::new_vertex_drag(1, 10, points.clone(), Point::new(0, 0));

        // Should return original points unchanged
        let new_points = ctx.compute_new_points();
        assert_eq!(new_points, points);
    }

    #[test]
    fn test_wire_drag_context_out_of_bounds_segment() {
        let points = vec![Point::new(0, 0), Point::new(10, 0)];
        let ctx = WireDragContext::new_segment_drag(1, 5, points.clone(), Point::new(0, 0), true);

        // Should return original points unchanged
        let new_points = ctx.compute_new_points();
        assert_eq!(new_points, points);
    }
}
