use super::super::*;

impl SchematicState {
    // =========================================================================
    // Clipboard Operations
    // =========================================================================

    /// Copy selected components and wires to clipboard
    ///
    /// In addition to explicitly selected wires, automatically includes
    /// any wires that have both endpoints connected to selected components.
    /// This preserves circuit connectivity when copying/pasting.
    pub fn copy_selection(&mut self) {
        if self.selection.is_empty() {
            return;
        }

        let selected_comps: Vec<Component> = self
            .components
            .iter()
            .filter(|c| self.selection.has_component(c.id))
            .cloned()
            .collect();

        // Get all terminal positions for selected components
        let selected_terminals: Vec<Point> = selected_comps
            .iter()
            .flat_map(|c| c.terminal_positions().into_iter().map(|(_, pos)| pos))
            .collect();

        // Find wires that have both endpoints at selected component terminals
        let mut wires_to_copy: Vec<Wire> = Vec::new();

        for wire in &self.wires {
            // Check if explicitly selected
            if self.selection.has_wire(wire.id) {
                wires_to_copy.push(wire.clone());
                continue;
            }

            // Check if both endpoints connect to selected components
            if wire.points.len() >= 2 {
                let start = wire.points[0];
                let end = *wire.points.last().unwrap();

                let start_connected = selected_terminals.contains(&start);
                let end_connected = selected_terminals.contains(&end);

                if start_connected && end_connected {
                    wires_to_copy.push(wire.clone());
                }
            }
        }

        self.clipboard = ClipboardData::from_selection(selected_comps, wires_to_copy);
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position
    pub fn paste_at(&mut self, pos: Point) {
        if !self.can_paste() {
            return;
        }

        let clipboard_components = self.clipboard.components.clone();
        let clipboard_wires = self.clipboard.wires.clone();
        let origin = self.clipboard.origin;

        let offset_x = pos.x - origin.x;
        let offset_y = pos.y - origin.y;

        self.selection.clear();

        // Paste components with new IDs
        for comp in clipboard_components {
            let new_id = self.next_id();
            let mut new_comp = comp;
            new_comp.id = new_id;
            new_comp.pos.x += offset_x;
            new_comp.pos.y += offset_y;
            new_comp.name = self.generate_name(new_comp.kind);
            self.components.push(new_comp);
            self.selection.select_component(new_id);
        }

        // Paste wires with new IDs
        for wire in clipboard_wires {
            let new_id = self.next_id();
            let new_points: Vec<Point> = wire
                .points
                .iter()
                .map(|p| Point::new(p.x + offset_x, p.y + offset_y))
                .collect();
            self.wires.push(Wire::new(new_id, new_points));
            self.selection.select_wire(new_id);
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }
}
