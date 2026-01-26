//! Net Highlighting
//!
//! Commercial-grade net highlighting for schematic editor.
//! Click a wire or component terminal to highlight the entire connected net.
//!
//! This module provides:
//! - Net connectivity graph construction from wires and junctions
//! - Finding all wires connected to a point
//! - Tracking highlighted net state
//!
//! Matches Cadence Virtuoso behavior where clicking a net highlights
//! all electrically connected segments.

use std::collections::{HashMap, HashSet};

use super::net_label::Junction;
use super::point::Point;
use super::wire::Wire;

// =============================================================================
// Net Connectivity Graph
// =============================================================================

/// NetGraph builds a connectivity graph from wires and junctions
///
/// Points are connected if:
/// 1. They are endpoints of the same wire
/// 2. They share coordinates (wires meet at a point)
/// 3. They are connected via a junction
#[derive(Debug, Clone, Default)]
pub struct NetGraph {
    /// Map from point to set of wire IDs that touch that point
    point_to_wires: HashMap<Point, HashSet<u64>>,
    /// Map from wire ID to its endpoint positions
    wire_endpoints: HashMap<u64, (Point, Point)>,
}

impl NetGraph {
    /// Build a new net graph from wires and junctions
    pub fn build(wires: &[Wire], junctions: &[Junction]) -> Self {
        let mut graph = Self::default();

        for wire in wires {
            if wire.points.len() < 2 {
                continue;
            }

            let start = wire.points[0];
            let end = *wire.points.last().unwrap();

            // Record wire endpoints
            graph.wire_endpoints.insert(wire.id, (start, end));

            // Map each point to this wire
            graph
                .point_to_wires
                .entry(start)
                .or_default()
                .insert(wire.id);
            graph.point_to_wires.entry(end).or_default().insert(wire.id);
        }

        // Junctions connect all wires at their position
        for junction in junctions {
            // Junctions are already handled by point matching above
            // But we ensure the point exists in the map
            graph.point_to_wires.entry(junction.pos).or_default();
        }

        graph
    }

    /// Find all wire IDs in the same net as the given point
    ///
    /// Uses flood-fill algorithm to find all connected wires.
    pub fn find_connected_wires(&self, start_point: Point) -> HashSet<u64> {
        let mut connected = HashSet::new();
        let mut visited_points = HashSet::new();
        let mut stack = vec![start_point];

        while let Some(point) = stack.pop() {
            if visited_points.contains(&point) {
                continue;
            }
            visited_points.insert(point);

            // Get all wires at this point
            if let Some(wire_ids) = self.point_to_wires.get(&point) {
                for &wire_id in wire_ids {
                    if connected.insert(wire_id) {
                        // Add wire's other endpoints to stack
                        if let Some(&(start, end)) = self.wire_endpoints.get(&wire_id) {
                            if !visited_points.contains(&start) {
                                stack.push(start);
                            }
                            if !visited_points.contains(&end) {
                                stack.push(end);
                            }
                        }
                    }
                }
            }
        }

        connected
    }

    /// Find all wire IDs in the same net as the given wire
    pub fn find_connected_wires_from_wire(&self, wire_id: u64) -> HashSet<u64> {
        if let Some(&(start, _)) = self.wire_endpoints.get(&wire_id) {
            self.find_connected_wires(start)
        } else {
            HashSet::new()
        }
    }

    /// Convenience method: Build from just wires (no junctions)
    pub fn build_from_wires(wires: &[Wire]) -> Self {
        Self::build(wires, &[])
    }

    /// Convenience method: Get connected wires by wire ID
    pub fn get_connected_wires(&self, wire_id: u64) -> HashSet<u64> {
        self.find_connected_wires_from_wire(wire_id)
    }
}

// =============================================================================
// Net Highlighting State
// =============================================================================

/// State for tracking highlighted net
#[derive(Debug, Clone, Default)]
pub struct NetHighlightState {
    /// IDs of wires in the currently highlighted net
    pub highlighted_wires: HashSet<u64>,
    /// Whether highlighting is active
    pub active: bool,
}

impl NetHighlightState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self::default()
    }

    /// Highlight wires connected to a point
    pub fn highlight_net(&mut self, graph: &NetGraph, point: Point) {
        self.highlighted_wires = graph.find_connected_wires(point);
        self.active = !self.highlighted_wires.is_empty();
    }

    /// Highlight wires connected to a specific wire
    pub fn highlight_from_wire(&mut self, graph: &NetGraph, wire_id: u64) {
        self.highlighted_wires = graph.find_connected_wires_from_wire(wire_id);
        self.active = !self.highlighted_wires.is_empty();
    }

    /// Clear the highlight
    pub fn clear(&mut self) {
        self.highlighted_wires.clear();
        self.active = false;
    }

    /// Check if a wire is highlighted
    pub fn is_wire_highlighted(&self, wire_id: u64) -> bool {
        self.active && self.highlighted_wires.contains(&wire_id)
    }

    /// Highlight a set of wires directly
    pub fn highlight_wires(&mut self, wire_ids: HashSet<u64>) {
        self.highlighted_wires = wire_ids;
        self.active = !self.highlighted_wires.is_empty();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_wire(id: u64, points: Vec<Point>) -> Wire {
        Wire { id, points }
    }

    #[test]
    fn test_net_graph_single_wire() {
        let wires = vec![create_test_wire(
            1,
            vec![Point::new(0, 0), Point::new(10, 0)],
        )];
        let graph = NetGraph::build(&wires, &[]);

        let connected = graph.find_connected_wires(Point::new(0, 0));
        assert_eq!(connected.len(), 1);
        assert!(connected.contains(&1));
    }

    #[test]
    fn test_net_graph_connected_wires() {
        // Two wires sharing a point
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(10, 0), Point::new(20, 0)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        // Should find both wires from either end
        let connected = graph.find_connected_wires(Point::new(0, 0));
        assert_eq!(connected.len(), 2);
        assert!(connected.contains(&1));
        assert!(connected.contains(&2));
    }

    #[test]
    fn test_net_graph_disconnected_wires() {
        // Two separate wires
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(50, 50), Point::new(60, 50)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        let connected = graph.find_connected_wires(Point::new(0, 0));
        assert_eq!(connected.len(), 1);
        assert!(connected.contains(&1));
        assert!(!connected.contains(&2));
    }

    #[test]
    fn test_net_graph_chain_of_wires() {
        // Chain: A-B-C-D
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(10, 0), Point::new(20, 0)]),
            create_test_wire(3, vec![Point::new(20, 0), Point::new(30, 0)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        // Should find all three from any point
        let connected = graph.find_connected_wires(Point::new(0, 0));
        assert_eq!(connected.len(), 3);
    }

    #[test]
    fn test_net_graph_star_topology() {
        // Star: all wires meet at center
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 10)]),
            create_test_wire(2, vec![Point::new(20, 0), Point::new(10, 10)]),
            create_test_wire(3, vec![Point::new(10, 20), Point::new(10, 10)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        let connected = graph.find_connected_wires(Point::new(10, 10));
        assert_eq!(connected.len(), 3);
    }

    #[test]
    fn test_net_graph_from_wire() {
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(10, 0), Point::new(20, 0)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        let connected = graph.find_connected_wires_from_wire(1);
        assert_eq!(connected.len(), 2);
    }

    #[test]
    fn test_net_highlight_state_basic() {
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(10, 0), Point::new(20, 0)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        let mut state = NetHighlightState::new();
        assert!(!state.active);

        state.highlight_net(&graph, Point::new(0, 0));
        assert!(state.active);
        assert!(state.is_wire_highlighted(1));
        assert!(state.is_wire_highlighted(2));
    }

    #[test]
    fn test_net_highlight_clear() {
        let wires = vec![create_test_wire(
            1,
            vec![Point::new(0, 0), Point::new(10, 0)],
        )];
        let graph = NetGraph::build(&wires, &[]);

        let mut state = NetHighlightState::new();
        state.highlight_net(&graph, Point::new(0, 0));
        assert!(state.active);

        state.clear();
        assert!(!state.active);
        assert!(!state.is_wire_highlighted(1));
    }

    #[test]
    fn test_net_highlight_from_wire() {
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(10, 0), Point::new(20, 0)]),
            create_test_wire(3, vec![Point::new(100, 100), Point::new(110, 100)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        let mut state = NetHighlightState::new();
        state.highlight_from_wire(&graph, 1);

        assert!(state.is_wire_highlighted(1));
        assert!(state.is_wire_highlighted(2));
        assert!(!state.is_wire_highlighted(3));
    }

    #[test]
    fn test_net_graph_empty() {
        let graph = NetGraph::build(&[], &[]);
        let connected = graph.find_connected_wires(Point::new(0, 0));
        assert!(connected.is_empty());
    }

    #[test]
    fn test_net_graph_complex_mesh() {
        // Complex mesh network
        //     2
        //     |
        // 1---+---3
        //     |
        //     4
        let center = Point::new(10, 10);
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 10), center]),
            create_test_wire(2, vec![Point::new(10, 0), center]),
            create_test_wire(3, vec![center, Point::new(20, 10)]),
            create_test_wire(4, vec![center, Point::new(10, 20)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        // All should be connected
        let connected = graph.find_connected_wires(Point::new(0, 10));
        assert_eq!(connected.len(), 4);
    }

    #[test]
    fn test_net_highlight_switch_nets() {
        let wires = vec![
            create_test_wire(1, vec![Point::new(0, 0), Point::new(10, 0)]),
            create_test_wire(2, vec![Point::new(50, 0), Point::new(60, 0)]),
        ];
        let graph = NetGraph::build(&wires, &[]);

        let mut state = NetHighlightState::new();

        // Highlight first net
        state.highlight_net(&graph, Point::new(0, 0));
        assert!(state.is_wire_highlighted(1));
        assert!(!state.is_wire_highlighted(2));

        // Switch to second net
        state.highlight_net(&graph, Point::new(50, 0));
        assert!(!state.is_wire_highlighted(1));
        assert!(state.is_wire_highlighted(2));
    }
}
