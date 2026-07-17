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
    /// Map from electrical connection points to the wire IDs joined there.
    point_to_wires: HashMap<Point, HashSet<u64>>,
    /// Wire-level connectivity used for transitive net traversal.
    wire_adjacency: HashMap<u64, HashSet<u64>>,
    /// Segment geometry retained so an arbitrary point on a wire can seed a
    /// highlight even when it is not itself a connection point.
    wire_segments: HashMap<u64, Vec<(Point, Point)>>,
}

impl NetGraph {
    /// Build a new net graph from wires and junctions
    pub fn build(wires: &[Wire], junctions: &[Junction]) -> Self {
        let mut graph = Self::default();
        let mut segments = Vec::new();

        for wire in wires {
            if wire.points.len() < 2 {
                continue;
            }

            graph.wire_adjacency.entry(wire.id).or_default();
            for &point in &wire.points {
                graph
                    .point_to_wires
                    .entry(point)
                    .or_default()
                    .insert(wire.id);
            }
            for points in wire.points.windows(2) {
                let segment = (points[0], points[1]);
                graph
                    .wire_segments
                    .entry(wire.id)
                    .or_default()
                    .push(segment);
                segments.push((wire.id, segment.0, segment.1));
            }
        }

        // Spatially index segment bounding boxes so endpoint/T and junction
        // checks remain responsive for large schematics.
        const CELL: i32 = 256;
        let mut cells: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        for (index, &(_, start, end)) in segments.iter().enumerate() {
            for cell_x in start.x.min(end.x).div_euclid(CELL)..=start.x.max(end.x).div_euclid(CELL)
            {
                for cell_y in
                    start.y.min(end.y).div_euclid(CELL)..=start.y.max(end.y).div_euclid(CELL)
                {
                    cells.entry((cell_x, cell_y)).or_default().push(index);
                }
            }
        }

        // A wire vertex touching another wire's segment is an endpoint/T
        // connection. Pure segment-interior crossings are not examined here,
        // so they remain electrically separate unless explicitly marked.
        for wire in wires {
            for &point in &wire.points {
                let cell = (point.x.div_euclid(CELL), point.y.div_euclid(CELL));
                let Some(candidates) = cells.get(&cell) else {
                    continue;
                };
                for &segment_index in candidates {
                    let (other_wire_id, start, end) = segments[segment_index];
                    if point_on_segment(point, start, end) {
                        let wires_at_point = graph.point_to_wires.entry(point).or_default();
                        wires_at_point.insert(wire.id);
                        wires_at_point.insert(other_wire_id);
                    }
                }
            }
        }

        // A persisted junction joins every segment that passes through its
        // position, including otherwise-disconnected mid-segment crossings.
        for junction in junctions {
            let point = junction.pos;
            let cell = (point.x.div_euclid(CELL), point.y.div_euclid(CELL));
            let wires_at_point = graph.point_to_wires.entry(point).or_default();
            if let Some(candidates) = cells.get(&cell) {
                for &segment_index in candidates {
                    let (wire_id, start, end) = segments[segment_index];
                    if point_on_segment(point, start, end) {
                        wires_at_point.insert(wire_id);
                    }
                }
            }
        }

        // Collapse connection points into a wire-level graph. Traversing this
        // graph naturally propagates through any number of junctions and Ts.
        for wire_ids in graph.point_to_wires.values() {
            for &wire_id in wire_ids {
                graph.wire_adjacency.entry(wire_id).or_default();
                for &other_wire_id in wire_ids {
                    if wire_id != other_wire_id {
                        graph
                            .wire_adjacency
                            .entry(wire_id)
                            .or_default()
                            .insert(other_wire_id);
                    }
                }
            }
        }

        graph
    }

    /// Find all wire IDs in the same net as the given point
    ///
    /// Uses flood-fill algorithm to find all connected wires.
    pub fn find_connected_wires(&self, start_point: Point) -> HashSet<u64> {
        if let Some(wire_ids) = self.point_to_wires.get(&start_point) {
            let mut connected = HashSet::new();
            for &wire_id in wire_ids {
                connected.extend(self.find_connected_wires_from_wire(wire_id));
            }
            return connected;
        }

        // Away from a connection point, seed from the wire containing the
        // requested location. At an ambiguous unmarked crossing, use a stable
        // single seed rather than incorrectly joining both electrical nets.
        self.wire_segments
            .iter()
            .filter(|(_, segments)| {
                segments
                    .iter()
                    .any(|&(start, end)| point_on_segment(start_point, start, end))
            })
            .map(|(&wire_id, _)| wire_id)
            .min()
            .map(|wire_id| self.find_connected_wires_from_wire(wire_id))
            .unwrap_or_default()
    }

    /// Find all wire IDs in the same net as the given wire
    pub fn find_connected_wires_from_wire(&self, wire_id: u64) -> HashSet<u64> {
        if !self.wire_adjacency.contains_key(&wire_id) {
            return HashSet::new();
        }

        let mut connected = HashSet::new();
        let mut stack = vec![wire_id];
        while let Some(current) = stack.pop() {
            if !connected.insert(current) {
                continue;
            }
            if let Some(neighbors) = self.wire_adjacency.get(&current) {
                stack.extend(neighbors.iter().copied());
            }
        }
        connected
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

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    let point_x = i128::from(point.x);
    let point_y = i128::from(point.y);
    let start_x = i128::from(start.x);
    let start_y = i128::from(start.y);
    let end_x = i128::from(end.x);
    let end_y = i128::from(end.y);

    let cross = (point_x - start_x) * (end_y - start_y) - (point_y - start_y) * (end_x - start_x);
    cross == 0
        && point_x >= start_x.min(end_x)
        && point_x <= start_x.max(end_x)
        && point_y >= start_y.min(end_y)
        && point_y <= start_y.max(end_y)
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

    fn ids(values: &[u64]) -> HashSet<u64> {
        values.iter().copied().collect()
    }

    #[test]
    fn unmarked_mid_segment_crossing_stays_disconnected() {
        let wires = vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(0, -100), Point::new(0, 100)),
        ];

        let graph = NetGraph::build(&wires, &[]);

        assert_eq!(graph.get_connected_wires(1), ids(&[1]));
        assert_eq!(graph.get_connected_wires(2), ids(&[2]));
    }

    #[test]
    fn explicit_junction_connects_mid_segment_crossing() {
        let wires = vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(0, -100), Point::new(0, 100)),
        ];
        let junctions = vec![Junction::new(10, Point::new(0, 0))];

        let graph = NetGraph::build(&wires, &junctions);

        assert_eq!(graph.get_connected_wires(1), ids(&[1, 2]));
        assert_eq!(graph.find_connected_wires(Point::new(0, 0)), ids(&[1, 2]));
    }

    #[test]
    fn endpoint_to_segment_contact_connects_without_junction() {
        let wires = vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(0, 0), Point::new(0, 100)),
        ];

        let graph = NetGraph::build(&wires, &[]);

        assert_eq!(graph.get_connected_wires(1), ids(&[1, 2]));
    }

    #[test]
    fn connectivity_propagates_transitively_across_junctions() {
        let wires = vec![
            Wire::segment(1, Point::new(-100, 0), Point::new(100, 0)),
            Wire::segment(2, Point::new(0, -100), Point::new(0, 100)),
            Wire::segment(3, Point::new(-100, 60), Point::new(100, 60)),
        ];
        let junctions = vec![
            Junction::new(10, Point::new(0, 0)),
            Junction::new(11, Point::new(0, 60)),
        ];

        let graph = NetGraph::build(&wires, &junctions);

        assert_eq!(graph.get_connected_wires(1), ids(&[1, 2, 3]));
        assert_eq!(graph.get_connected_wires(3), ids(&[1, 2, 3]));
    }
}
