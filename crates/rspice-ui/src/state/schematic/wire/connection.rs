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

// =============================================================================
// Connection Set (for managing multiple connections)
// =============================================================================

/// A collection of wire connections with convenience methods
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionSet {
    connections: Vec<WireConnection>,
}

impl ConnectionSet {
    /// Create an empty connection set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a connection
    pub fn add(&mut self, connection: WireConnection) {
        // Avoid duplicates
        if !self.connections.iter().any(|c| {
            c.wire_id == connection.wire_id
                && c.point_index == connection.point_index
                && c.component_id == connection.component_id
        }) {
            self.connections.push(connection);
        }
    }

    /// Remove connections for a specific wire
    pub fn remove_wire(&mut self, wire_id: u64) {
        self.connections.retain(|c| c.wire_id != wire_id);
    }

    /// Remove connections for a specific component
    pub fn remove_component(&mut self, component_id: u64) {
        self.connections.retain(|c| c.component_id != component_id);
    }

    /// Get all connections for a wire
    pub fn for_wire(&self, wire_id: u64) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.wire_id == wire_id)
            .collect()
    }

    /// Get all connections for a component
    pub fn for_component(&self, component_id: u64) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.component_id == component_id)
            .collect()
    }

    /// Get connections for a specific terminal
    pub fn for_terminal(&self, component_id: u64, terminal_name: &str) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.component_id == component_id && c.terminal_name == terminal_name)
            .collect()
    }

    /// Get the number of connections
    pub fn len(&self) -> usize {
        self.connections.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }

    /// Clear all connections
    pub fn clear(&mut self) {
        self.connections.clear();
    }

    /// Iterate over all connections
    pub fn iter(&self) -> impl Iterator<Item = &WireConnection> {
        self.connections.iter()
    }
}

impl IntoIterator for ConnectionSet {
    type Item = WireConnection;
    type IntoIter = std::vec::IntoIter<WireConnection>;

    fn into_iter(self) -> Self::IntoIter {
        self.connections.into_iter()
    }
}

impl FromIterator<WireConnection> for ConnectionSet {
    fn from_iter<T: IntoIterator<Item = WireConnection>>(iter: T) -> Self {
        let mut set = ConnectionSet::new();
        for conn in iter {
            set.add(conn);
        }
        set
    }
}

// =============================================================================
// Tests
// =============================================================================
