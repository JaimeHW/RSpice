//! Wire mutations and attachment rebuilding owned by the live document.
use super::super::{terminal_connection, wire_edit};
use super::Schematic;
use rspice_design_model::Point;

impl Schematic {
    pub fn add_wire(&mut self, points: Vec<Point>) -> Option<u64> {
        let id = wire_edit::add_wire(&mut self.document, &mut self.identity, points);
        if id.is_some() {
            self.invalidate_topology();
        }
        id
    }
    pub fn split_wire(&mut self, wire_id: u64, at_point: Point) -> Option<(u64, u64)> {
        let split =
            wire_edit::split_wire(&mut self.document, &mut self.identity, wire_id, at_point);
        if split.is_some() {
            self.invalidate_topology();
        }
        split
    }
    pub fn optimize_all_wires(&mut self) -> bool {
        let changed = wire_edit::optimize_all_wires(&mut self.document);
        if changed {
            self.invalidate_topology();
        }
        changed
    }
    pub fn remove_degenerate_segments(&mut self) -> (usize, usize) {
        let counts = wire_edit::remove_degenerate_segments(&mut self.document);
        if counts.0 > 0 || counts.1 > 0 {
            self.invalidate_topology();
        }
        counts
    }
    pub fn move_vertices_at(&mut self, old_pos: Point, new_pos: Point) -> bool {
        let moved = wire_edit::move_vertices_at(&mut self.document, old_pos, new_pos);
        if moved {
            self.invalidate_topology();
        }
        moved
    }
    pub fn rebuild_connections(&mut self) {
        terminal_connection::rebuild_connections(&mut self.document);
    }
    pub fn rebuild_connections_from_terminals(&mut self, terminals: &[(u64, String, Point)]) {
        terminal_connection::rebuild_connections_from_terminals(&mut self.document, terminals);
    }
}
