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
            if let Some((comp_id, term_name, _)) = self.find_terminal_at(pos) {
                self.connections.push(WireConnection::new(
                    wire_id,
                    point_index,
                    comp_id,
                    term_name,
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
