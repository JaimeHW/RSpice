//! Terminal-to-wire connectivity.
//!
//! Finding the terminal under a point, and rebuilding the connection set
//! after geometry moves. Connectivity is derived, never stored by hand: any
//! operation that changes where a terminal or a wire end sits rebuilds from
//! the terminals rather than patching individual connections.

use super::super::*;

impl SchematicState {
    // =========================================================================
    // Wire Connection Management (for rubber-banding)
    // =========================================================================

    /// Find a component terminal at or near a grid position
    pub fn find_terminal_at(&self, pos: Point) -> Option<(u64, String, Point)> {
        for comp in &self.components {
            for (term_name, term_pos) in comp.terminal_positions() {
                let dx = (pos.x - term_pos.x).abs();
                let dy = (pos.y - term_pos.y).abs();
                if dx <= SNAP_DISTANCE && dy <= SNAP_DISTANCE {
                    return Some((comp.id, term_name.to_string(), term_pos));
                }
            }
        }
        None
    }

    /// Rebuild all wire connections based on current positions
    pub fn rebuild_connections(&mut self) {
        let terminals = self
            .components
            .iter()
            .flat_map(|component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(move |(name, point)| (component.id, name.to_owned(), point))
            })
            .collect::<Vec<_>>();
        self.rebuild_connections_from_terminals(&terminals);
    }

    /// Rebuild the rubber-band cache from authoritative resolved terminal
    /// geometry. Authored library symbols must use this path because their pin
    /// positions can differ from intrinsic fallback geometry.
    pub fn rebuild_connections_from_terminals(&mut self, terminals: &[(u64, String, Point)]) {
        self.connections.clear();

        let wire_endpoints: Vec<(u64, Point, usize)> = self
            .wires
            .iter()
            .filter(|w| !w.points.is_empty())
            .flat_map(|w| {
                let mut endpoints = vec![(w.id, w.points[0], 0usize)];
                let end_idx = w.points.len() - 1;
                if end_idx > 0 {
                    endpoints.push((w.id, w.points[end_idx], end_idx));
                }
                endpoints
            })
            .collect();

        for (wire_id, pos, point_index) in wire_endpoints {
            if let Some((comp_id, term_name, _)) = terminals.iter().find(|(_, _, terminal)| {
                (pos.x - terminal.x).abs() <= SNAP_DISTANCE
                    && (pos.y - terminal.y).abs() <= SNAP_DISTANCE
            }) {
                self.connections.push(WireConnection::new(
                    wire_id,
                    point_index,
                    *comp_id,
                    term_name.clone(),
                ));
            }
        }
    }

    /// Find all connections for a specific component
    pub fn connections_for_component(&self, component_id: u64) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.component_id == component_id)
            .collect()
    }
}
