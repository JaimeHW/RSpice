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

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // WireConnection Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_wire_connection_new() {
        let conn = WireConnection::new(1, 0, 10, "+");
        assert_eq!(conn.wire_id, 1);
        assert_eq!(conn.point_index, 0);
        assert_eq!(conn.component_id, 10);
        assert_eq!(conn.terminal_name, "+");
    }

    #[test]
    fn test_wire_connection_is_start() {
        let start_conn = WireConnection::new(1, 0, 10, "+");
        let mid_conn = WireConnection::new(1, 1, 10, "-");

        assert!(start_conn.is_start());
        assert!(!mid_conn.is_start());
    }

    #[test]
    fn test_wire_connection_is_end() {
        let end_conn = WireConnection::new(1, 4, 10, "+");
        let mid_conn = WireConnection::new(1, 2, 10, "-");

        assert!(end_conn.is_end(5)); // 5 points, index 4 is last
        assert!(!mid_conn.is_end(5));
    }

    #[test]
    fn test_wire_connection_is_endpoint() {
        let start = WireConnection::new(1, 0, 10, "+");
        let end = WireConnection::new(1, 4, 10, "-");
        let mid = WireConnection::new(1, 2, 10, "C");

        assert!(start.is_endpoint(5));
        assert!(end.is_endpoint(5));
        assert!(!mid.is_endpoint(5));
    }

    #[test]
    fn test_wire_connection_is_wire() {
        let conn = WireConnection::new(42, 0, 10, "+");
        assert!(conn.is_wire(42));
        assert!(!conn.is_wire(1));
    }

    #[test]
    fn test_wire_connection_is_component() {
        let conn = WireConnection::new(1, 0, 42, "+");
        assert!(conn.is_component(42));
        assert!(!conn.is_component(1));
    }

    #[test]
    fn test_wire_connection_is_terminal() {
        let conn = WireConnection::new(1, 0, 10, "GATE");
        assert!(conn.is_terminal("GATE"));
        assert!(!conn.is_terminal("DRAIN"));
    }

    #[test]
    fn test_wire_connection_description() {
        let conn = WireConnection::new(1, 2, 10, "GND");
        let desc = conn.description();
        assert!(desc.contains("Wire 1"));
        assert!(desc.contains("point 2"));
        assert!(desc.contains("Component 10"));
        assert!(desc.contains("GND"));
    }

    // -------------------------------------------------------------------------
    // ConnectionSet Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_connection_set_new() {
        let set = ConnectionSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_connection_set_add() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_connection_set_no_duplicates() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(1, 0, 10, "+")); // Duplicate
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_connection_set_remove_wire() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(1, 1, 11, "-"));
        set.add(WireConnection::new(2, 0, 12, "+"));

        set.remove_wire(1);
        assert_eq!(set.len(), 1);
        assert!(set.for_wire(2).len() == 1);
    }

    #[test]
    fn test_connection_set_remove_component() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(2, 0, 10, "-"));
        set.add(WireConnection::new(3, 0, 20, "+"));

        set.remove_component(10);
        assert_eq!(set.len(), 1);
        assert!(set.for_component(20).len() == 1);
    }

    #[test]
    fn test_connection_set_for_wire() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(1, 5, 11, "-"));
        set.add(WireConnection::new(2, 0, 12, "+"));

        let wire1_conns = set.for_wire(1);
        assert_eq!(wire1_conns.len(), 2);
    }

    #[test]
    fn test_connection_set_for_component() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(2, 0, 10, "-"));
        set.add(WireConnection::new(3, 0, 20, "+"));

        let comp10_conns = set.for_component(10);
        assert_eq!(comp10_conns.len(), 2);
    }

    #[test]
    fn test_connection_set_for_terminal() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(2, 0, 10, "-"));
        set.add(WireConnection::new(3, 0, 10, "+"));

        let plus_conns = set.for_terminal(10, "+");
        assert_eq!(plus_conns.len(), 2);
    }

    #[test]
    fn test_connection_set_clear() {
        let mut set = ConnectionSet::new();
        set.add(WireConnection::new(1, 0, 10, "+"));
        set.add(WireConnection::new(2, 0, 11, "-"));

        set.clear();
        assert!(set.is_empty());
    }

    #[test]
    fn test_connection_set_from_iter() {
        let conns = vec![
            WireConnection::new(1, 0, 10, "+"),
            WireConnection::new(2, 0, 11, "-"),
        ];

        let set: ConnectionSet = conns.into_iter().collect();
        assert_eq!(set.len(), 2);
    }
}
