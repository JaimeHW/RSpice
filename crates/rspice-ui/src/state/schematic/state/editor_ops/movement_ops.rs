use super::super::*;

impl SchematicState {
    /// Move a component and update all attached wire endpoints (rubber-banding)
    pub fn move_component_with_wires(&mut self, component_id: u64, delta: Point) {
        // Get the component's terminal positions BEFORE moving
        let terminals: Vec<Point> = {
            if let Some(comp) = self.components.iter().find(|c| c.id == component_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return;
            }
        };

        // Find ALL wire points that are at ANY terminal position
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();

        for wire in &self.wires {
            for (point_idx, point) in wire.points.iter().enumerate() {
                for term_pos in &terminals {
                    if *point == *term_pos {
                        let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                        wire_updates.push((wire.id, point_idx, new_pos));
                        break;
                    }
                }
            }
        }

        // Move the component
        if let Some(comp) = self.components.iter_mut().find(|c| c.id == component_id) {
            comp.pos.x += delta.x;
            comp.pos.y += delta.y;
        }

        // Apply wire updates
        for (wire_id, point_idx, new_pos) in wire_updates {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if point_idx < wire.points.len() {
                    wire.points[point_idx] = new_pos;
                }
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all selected components and rubber-band connected wires
    ///
    /// This is the multi-component version of move_component_with_wires.
    /// Wires connected to selected components are stretched to maintain
    /// the connection. Wires that connect two selected components are
    /// moved entirely (not stretched).
    pub fn move_selection_with_rubber_band(&mut self, delta: Point) {
        let selected_components: Vec<u64> = self.selection.components.to_vec();
        if selected_components.is_empty() && self.selection.wires.is_empty() {
            return;
        }

        // Collect all terminal positions for selected components BEFORE moving
        let mut all_terminals: Vec<(u64, Point)> = Vec::new();
        for comp_id in &selected_components {
            if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                for (_, pos) in comp.terminal_positions() {
                    all_terminals.push((*comp_id, pos));
                }
            }
        }

        // Find wires that should be stretched (one end connected to selection)
        // vs moved entirely (both ends connected to selection)
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
        let mut wires_to_move: Vec<u64> = Vec::new();

        for wire in &self.wires {
            let start = wire.points.first().copied();
            let end = wire.points.last().copied();

            // Check if endpoints connect to selected components
            let start_connected = start.is_some_and(|p| {
                all_terminals.iter().any(|(_, term_pos)| *term_pos == p)
            });
            let end_connected = end.is_some_and(|p| {
                all_terminals.iter().any(|(_, term_pos)| *term_pos == p)
            });

            if start_connected && end_connected {
                // Both ends connected to selection - move entire wire
                wires_to_move.push(wire.id);
            } else {
                // Stretch endpoints that are connected
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for (_, term_pos) in &all_terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(point.x + delta.x, point.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }
        }

        // Move selected components
        for comp_id in &selected_components {
            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }
        }

        // Move selected wires (from selection, not from rubber-banding)
        for wire_id in self.selection.wires.iter() {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Move wires that have both ends connected to selection
        for wire_id in wires_to_move {
            // Skip if already in selection (already moved above)
            if self.selection.wires.contains(&wire_id) {
                continue;
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Apply stretch updates for partially connected wires
        for (wire_id, point_idx, new_pos) in wire_updates {
            // Skip if wire was already moved entirely
            if self.selection.wires.contains(&wire_id) {
                continue;
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if point_idx < wire.points.len() {
                    wire.points[point_idx] = new_pos;
                }
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all points of a wire by a delta
    pub fn move_wire(&mut self, wire_id: u64, delta: Point) {
        let old_endpoints: Vec<Point> = self
            .wires
            .iter()
            .find(|w| w.id == wire_id)
            .map(|w| {
                let mut eps = Vec::new();
                if let Some(first) = w.points.first() {
                    eps.push(*first);
                }
                if let Some(last) = w.points.last() {
                    eps.push(*last);
                }
                eps
            })
            .unwrap_or_default();

        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            for point in &mut wire.points {
                point.x += delta.x;
                point.y += delta.y;
            }
        }

        for old_pt in old_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos.x += delta.x;
                junction.pos.y += delta.y;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all selected components and wires by a delta
    pub fn move_selection(&mut self, delta: Point) {
        let selection = self.selection.clone();

        let selected_wire_ids: std::collections::HashSet<u64> =
            selection.wires.iter().copied().collect();

        // Move all selected components with rubber-banding
        for comp_id in &selection.components {
            let terminals: Vec<Point> = {
                if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                    comp.terminal_positions()
                        .into_iter()
                        .map(|(_, pos)| pos)
                        .collect()
                } else {
                    continue;
                }
            };

            let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
            for wire in &self.wires {
                if selected_wire_ids.contains(&wire.id) {
                    continue;
                }
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for term_pos in &terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }

            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }

            for (wire_id, point_idx, new_pos) in wire_updates {
                if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                    if point_idx < wire.points.len() {
                        wire.points[point_idx] = new_pos;
                    }
                }
            }
        }

        // Move all selected wires entirely
        let mut wire_endpoints: Vec<Point> = Vec::new();
        for wire_id in &selection.wires {
            if let Some(wire) = self.wires.iter().find(|w| w.id == *wire_id) {
                if let Some(first) = wire.points.first() {
                    wire_endpoints.push(*first);
                }
                if let Some(last) = wire.points.last() {
                    wire_endpoints.push(*last);
                }
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Move junctions at selected wire endpoints
        for old_pt in wire_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos.x += delta.x;
                junction.pos.y += delta.y;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all wire points at a junction to a new position
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) {
        for wire in &mut self.wires {
            for point in &mut wire.points {
                if *point == old_pos {
                    *point = new_pos;
                }
            }
        }

        if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pos) {
            junction.pos = new_pos;
        }

        self.is_dirty = true;
    }
}
