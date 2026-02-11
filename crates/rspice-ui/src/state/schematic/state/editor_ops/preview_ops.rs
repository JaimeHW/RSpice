use super::super::*;

impl SchematicState {
    /// Get wire points adjusted for a component drag preview
    pub fn get_wire_preview_points(
        &self,
        wire: &Wire,
        dragging_component_id: Option<u64>,
        delta: Point,
    ) -> Vec<Point> {
        let mut points = wire.points.clone();

        let comp_id = match dragging_component_id {
            Some(id) => id,
            None => return points,
        };

        let terminals: Vec<Point> =
            if let Some(comp) = self.components.iter().find(|c| c.id == comp_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return points;
            };

        for point in points.iter_mut() {
            for term_pos in &terminals {
                if *point == *term_pos {
                    *point = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                    break;
                }
            }
        }

        points
    }

    /// Snap wire endpoints to nearby terminals and rebuild connections
    pub fn snap_wire_to_terminals(&mut self, wire_id: u64) {
        let (start_pos, end_pos, end_idx) = {
            if let Some(wire) = self.wires.iter().find(|w| w.id == wire_id) {
                if wire.points.is_empty() {
                    return;
                }
                let end_idx = wire.points.len().saturating_sub(1);
                (
                    Some(wire.points[0]),
                    if end_idx > 0 {
                        Some(wire.points[end_idx])
                    } else {
                        None
                    },
                    end_idx,
                )
            } else {
                return;
            }
        };

        let snap_start = start_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));
        let snap_end = end_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));

        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            if let Some(term_pos) = snap_start {
                wire.points[0] = term_pos;
            }
            if let Some(term_pos) = snap_end {
                if end_idx > 0 {
                    wire.points[end_idx] = term_pos;
                }
            }
        }

        self.rebuild_connections();
    }
}
