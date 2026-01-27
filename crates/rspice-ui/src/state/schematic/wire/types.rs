//! Wire Operation Types
//!
//! Core types for wire manipulation operations including drag modes,
//! constraints, snap targets, and junction classification.

use super::super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// Wire Drag Mode
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

// =============================================================================
// Drag Constraint
// =============================================================================

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

// =============================================================================
// Snap Target
// =============================================================================

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

    /// Check if this is a junction snap
    pub fn is_junction(&self) -> bool {
        matches!(self, SnapTarget::Junction { .. })
    }

    /// Check if this is a grid snap
    pub fn is_grid(&self) -> bool {
        matches!(self, SnapTarget::GridPoint { .. })
    }
}

// =============================================================================
// Junction Type
// =============================================================================

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

    /// Check if this is a simple corner (no electrical junction dot needed)
    pub fn is_corner(&self) -> bool {
        matches!(self, JunctionType::Corner)
    }

    /// Check if this junction needs a visual dot indicator
    pub fn needs_dot(&self) -> bool {
        self.is_electrical_connection()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // DragConstraint Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_drag_constraint_free() {
        let constraint = DragConstraint::Free;
        assert_eq!(constraint.apply(10, 20), (10, 20));
        assert_eq!(constraint.apply(-5, 15), (-5, 15));
    }

    #[test]
    fn test_drag_constraint_horizontal() {
        let constraint = DragConstraint::Horizontal;
        assert_eq!(constraint.apply(10, 20), (10, 0));
        assert_eq!(constraint.apply(-5, 15), (-5, 0));
    }

    #[test]
    fn test_drag_constraint_vertical() {
        let constraint = DragConstraint::Vertical;
        assert_eq!(constraint.apply(10, 20), (0, 20));
        assert_eq!(constraint.apply(-5, 15), (0, 15));
    }

    #[test]
    fn test_drag_constraint_orthogonal_horizontal_dominant() {
        let constraint = DragConstraint::Orthogonal;
        assert_eq!(constraint.apply(10, 5), (10, 0));
    }

    #[test]
    fn test_drag_constraint_orthogonal_vertical_dominant() {
        let constraint = DragConstraint::Orthogonal;
        assert_eq!(constraint.apply(5, 10), (0, 10));
    }

    #[test]
    fn test_drag_constraint_orthogonal_equal() {
        let constraint = DragConstraint::Orthogonal;
        // Equal magnitudes, horizontal wins
        assert_eq!(constraint.apply(10, 10), (10, 0));
    }

    #[test]
    fn test_drag_constraint_angle45_horizontal() {
        let constraint = DragConstraint::Angle45;
        // Large horizontal, small vertical → snap to horizontal
        assert_eq!(constraint.apply(10, 2), (10, 0));
    }

    #[test]
    fn test_drag_constraint_angle45_vertical() {
        let constraint = DragConstraint::Angle45;
        // Small horizontal, large vertical → snap to vertical
        assert_eq!(constraint.apply(2, 10), (0, 10));
    }

    #[test]
    fn test_drag_constraint_angle45_diagonal() {
        let constraint = DragConstraint::Angle45;
        // Similar magnitudes → snap to 45°
        let (dx, dy) = constraint.apply(8, 10);
        assert_eq!(dx.abs(), dy.abs());
    }

    // -------------------------------------------------------------------------
    // SnapTarget Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_snap_target_terminal_position() {
        let target = SnapTarget::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
            position: Point::new(10, 20),
        };
        assert_eq!(target.position(), Point::new(10, 20));
    }

    #[test]
    fn test_snap_target_wire_vertex_position() {
        let target = SnapTarget::WireVertex {
            wire_id: 1,
            vertex_index: 0,
            position: Point::new(5, 5),
        };
        assert_eq!(target.position(), Point::new(5, 5));
    }

    #[test]
    fn test_snap_target_grid_position() {
        let target = SnapTarget::GridPoint {
            position: Point::new(100, 200),
        };
        assert_eq!(target.position(), Point::new(100, 200));
    }

    #[test]
    fn test_snap_target_priority_ordering() {
        let terminal = SnapTarget::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
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

        // Terminal has highest priority (lowest number)
        assert!(terminal.priority() < junction.priority());
        assert!(junction.priority() < vertex.priority());
        assert!(vertex.priority() < segment.priority());
        assert!(segment.priority() < grid.priority());
    }

    #[test]
    fn test_snap_target_is_terminal() {
        let terminal = SnapTarget::Terminal {
            component_id: 1,
            terminal_name: "+".to_string(),
            position: Point::new(0, 0),
        };
        let grid = SnapTarget::GridPoint {
            position: Point::new(0, 0),
        };
        assert!(terminal.is_terminal());
        assert!(!grid.is_terminal());
    }

    #[test]
    fn test_snap_target_is_wire() {
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
        assert!(vertex.is_wire());
        assert!(segment.is_wire());
        assert!(!grid.is_wire());
    }

    // -------------------------------------------------------------------------
    // JunctionType Tests
    // -------------------------------------------------------------------------

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
            JunctionType::from_wire_count(5),
            JunctionType::Complex { wire_count: 5 }
        );
    }

    #[test]
    fn test_junction_type_wire_count() {
        assert_eq!(JunctionType::PassThrough.wire_count(), 1);
        assert_eq!(JunctionType::Corner.wire_count(), 2);
        assert_eq!(JunctionType::TJunction.wire_count(), 3);
        assert_eq!(JunctionType::CrossJunction.wire_count(), 4);
        assert_eq!(JunctionType::Complex { wire_count: 6 }.wire_count(), 6);
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
    fn test_junction_type_is_corner() {
        assert!(!JunctionType::PassThrough.is_corner());
        assert!(JunctionType::Corner.is_corner());
        assert!(!JunctionType::TJunction.is_corner());
    }

    #[test]
    fn test_junction_type_needs_dot() {
        assert!(!JunctionType::PassThrough.needs_dot());
        assert!(!JunctionType::Corner.needs_dot());
        assert!(JunctionType::TJunction.needs_dot());
        assert!(JunctionType::CrossJunction.needs_dot());
        assert!(JunctionType::Complex { wire_count: 5 }.needs_dot());
    }
}
