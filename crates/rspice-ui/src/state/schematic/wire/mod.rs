//! Wire Types Module
//!
//! Wire segments, wire drawing state, and wire connections for schematic capture.

#![allow(clippy::module_inception)]
//!
//! This module provides robust wire handling with:
//! - Individual segment representation with geometric metadata
//! - Advanced hit testing with tolerance-based picking
//! - Vertex manipulation for corner editing
//! - Segment intersection and distance calculations
//! - Interactive wire drawing state machine
//! - Wire routing with multiple modes (orthogonal, diagonal, 45°)
//! - Wire-terminal connection tracking for rubber-banding
//!
//! # Module Organization
//!
//! - `segment` - WireSegment and WireHitResult for geometric operations
//! - `types` - WireDragMode, DragConstraint, SnapTarget, JunctionType
//! - `drag` - WireDragContext for active drag operations
//! - `wire` - Core Wire polyline type
//! - `routing` - WireRoutingMode and route optimization functions
//! - `drawing` - WireDrawing interactive state machine
//! - `connection` - WireConnection for terminal connections

mod connection;
mod drag;
mod drawing;
mod routing;
mod segment;
mod types;
mod wire;

// Re-export all public types for backwards compatibility
pub use connection::{ConnectionSet, WireConnection};
pub use drag::WireDragContext;
pub use drawing::WireDrawing;
pub use routing::{
    WireRoutingMode, convert_to_orthogonal, count_bends, is_valid_route, optimize_route,
    route_length,
};
pub use segment::{WireHitResult, WireSegment};
pub use types::{DragConstraint, JunctionType, WireDragMode};
// Note: SnapTarget is intentionally not re-exported here to avoid conflict with snap::SnapTarget
// Use wire::types::SnapTarget internally if needed, or snap::SnapTarget for public use
pub use wire::Wire;

// =============================================================================
// Module-level convenience functions
// =============================================================================

/// Create a simple two-point wire
pub fn create_wire(id: u64, start: super::point::Point, end: super::point::Point) -> Wire {
    Wire::segment(id, start, end)
}

/// Check if two wires are connected at any endpoint
pub fn wires_connected(wire1: &Wire, wire2: &Wire) -> bool {
    wire1.connects_to(wire2)
}

/// Find all intersection points between two wires
pub fn find_wire_intersections(wire1: &Wire, wire2: &Wire) -> Vec<super::point::Point> {
    wire1.intersections_with_wire(wire2)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::schematic::point::Point;

    #[test]
    fn test_create_wire() {
        let wire = create_wire(1, Point::new(0, 0), Point::new(10, 0));
        assert_eq!(wire.id, 1);
        assert_eq!(wire.segment_count(), 1);
    }

    #[test]
    fn test_wires_connected() {
        let wire1 = create_wire(1, Point::new(0, 0), Point::new(10, 0));
        let wire2 = create_wire(2, Point::new(10, 0), Point::new(10, 10));
        let wire3 = create_wire(3, Point::new(20, 0), Point::new(30, 0));

        assert!(wires_connected(&wire1, &wire2));
        assert!(!wires_connected(&wire1, &wire3));
    }

    #[test]
    fn test_find_wire_intersections() {
        let wire1 = create_wire(1, Point::new(0, 5), Point::new(10, 5));
        let wire2 = create_wire(2, Point::new(5, 0), Point::new(5, 10));

        let intersections = find_wire_intersections(&wire1, &wire2);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0], Point::new(5, 5));
    }

    // Test that all types are properly re-exported
    #[test]
    fn test_type_exports() {
        // WireSegment
        let seg = WireSegment::new(Point::new(0, 0), Point::new(10, 0));
        assert!(seg.is_horizontal());

        // WireHitResult
        let hit = WireHitResult::None;
        assert!(!hit.is_hit());

        // WireDragMode
        let _ = WireDragMode::MoveVertex;

        // DragConstraint
        let constraint = DragConstraint::Horizontal;
        assert_eq!(constraint.apply(10, 20), (10, 0));

        // SnapTarget (internal to wire module, not re-exported to avoid conflict with snap::SnapTarget)
        let target = types::SnapTarget::GridPoint {
            position: Point::new(0, 0),
        };
        assert!(target.is_grid());

        // JunctionType
        let junction = JunctionType::from_wire_count(3);
        assert!(junction.is_electrical_connection());

        // WireRoutingMode
        let mode = WireRoutingMode::default();
        assert!(mode.is_orthogonal());

        // WireDrawing
        let drawing = WireDrawing::new();
        assert!(!drawing.is_active());

        // WireConnection
        let conn = WireConnection::new(1, 0, 10, "+");
        assert!(conn.is_start());

        // ConnectionSet
        let set = ConnectionSet::new();
        assert!(set.is_empty());
    }

    // Test routing functions
    #[test]
    fn test_routing_functions() {
        let points = vec![Point::new(0, 0), Point::new(5, 0), Point::new(10, 0)];

        // optimize_route
        let optimized = optimize_route(&points);
        assert_eq!(optimized.len(), 2);

        // route_length
        let len = route_length(&points);
        assert!((len - 10.0).abs() < 0.001);

        // count_bends
        let bends = count_bends(&points);
        assert_eq!(bends, 0);

        // is_valid_route
        assert!(is_valid_route(&points));
    }
}
