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

