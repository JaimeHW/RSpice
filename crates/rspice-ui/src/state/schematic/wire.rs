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
//! - `types` - JunctionType
//! - `wire` - Core Wire polyline type
//! - `routing` - WireRoutingMode and route optimization functions
//! - `drawing` - WireDrawing interactive state machine
//! - `connection` - WireConnection for terminal connections

mod connection;
mod drawing;
mod routing;
mod segment;
mod types;
mod wire;

// Re-export all public types for backwards compatibility
pub use connection::WireConnection;
pub use drawing::WireDrawing;
pub use routing::WireRoutingMode;
pub use segment::WireSegment;
pub use types::JunctionType;
// Note: SnapTarget is intentionally not re-exported here to avoid conflict with snap::SnapTarget
// Use wire::types::SnapTarget internally if needed, or snap::SnapTarget for public use
pub use wire::Wire;

// =============================================================================
// Module-level convenience functions
// =============================================================================

// `create_wire`, `wires_connected`, and `find_wire_intersections` used to sit
// here: one-line free-function wrappers over `Wire::segment`,
// `Wire::connects_to`, and `Wire::intersections_with_wire`. Every caller used
// the methods.

// =============================================================================
// Tests
// =============================================================================
