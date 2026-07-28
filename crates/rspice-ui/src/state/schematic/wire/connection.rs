//! Wire Connection
//!
//! Represents connections between wire endpoints and component terminals.
//! Used for rubber-banding when components move.

use serde::{Deserialize, Serialize};

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

    /// Check if this connection is to the end of the wire
    ///
    /// Note: This requires knowing the wire's point count.
    pub fn is_end(&self, wire_point_count: usize) -> bool {
        wire_point_count > 0 && self.point_index == wire_point_count - 1
    }

    /// Check if this connection is to an endpoint (start or end)
    pub fn is_endpoint(&self, wire_point_count: usize) -> bool {
        self.is_start() || self.is_end(wire_point_count)
    }

    /// Check if this connection belongs to a specific wire
    pub fn is_wire(&self, wire_id: u64) -> bool {
        self.wire_id == wire_id
    }

    /// Check if this connection belongs to a specific component
    pub fn is_component(&self, component_id: u64) -> bool {
        self.component_id == component_id
    }

    /// Check if this connection is to a specific terminal
    pub fn is_terminal(&self, terminal_name: &str) -> bool {
        self.terminal_name == terminal_name
    }

    /// Get a descriptive string for this connection
    pub fn description(&self) -> String {
        format!(
            "Wire {} point {} → Component {} terminal {}",
            self.wire_id, self.point_index, self.component_id, self.terminal_name
        )
    }
}

