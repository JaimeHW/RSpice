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
                if wire.points.len() >= 2 {
                    wires_to_copy.push(wire.clone());
                }
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

        // Junction dots that sit on a copied wire travel with the selection;
        // a pasted multi-way joint must keep its explicit connection dots.
        let junctions_to_copy: Vec<Point> = self
            .junctions
            .iter()
            .map(|j| j.pos)
            .filter(|pos| wires_to_copy.iter().any(|w| w.points.contains(pos)))
            .collect();

        self.clipboard =
            ClipboardData::from_selection(selected_comps, wires_to_copy, junctions_to_copy);
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position (one undo entry)
    pub fn paste_at(&mut self, pos: Point) {
        if !self.can_paste() {
            return;
        }

        self.with_undo("paste", |s| {
            let clipboard_components = s.clipboard.components.clone();
            let clipboard_wires: Vec<Wire> = s
                .clipboard
                .wires
                .iter()
                .filter(|wire| wire.points.len() >= 2)
                .cloned()
                .collect();
            let clipboard_junctions = s.clipboard.junctions.clone();
            let origin = s.clipboard.origin;

            if clipboard_components.is_empty()
                && clipboard_wires.is_empty()
                && clipboard_junctions.is_empty()
            {
                return;
            }

            let offset_x = pos.x - origin.x;
            let offset_y = pos.y - origin.y;

            s.selection.clear();

            // Paste components with new IDs
            for comp in clipboard_components {
                let new_id = s.next_id();
                let mut new_comp = comp;
                new_comp.id = new_id;
                new_comp.pos.x += offset_x;
                new_comp.pos.y += offset_y;
                new_comp.name = s.generate_name(new_comp.kind);
                s.components.push(new_comp);
                s.selection.select_component(new_id);
            }

            // Paste wires with new IDs
            for wire in clipboard_wires {
                let new_id = s.next_id();
                let new_points: Vec<Point> = wire
                    .points
                    .iter()
                    .map(|p| Point::new(p.x + offset_x, p.y + offset_y))
                    .collect();
                s.wires.push(Wire::new(new_id, new_points));
                s.selection.select_wire(new_id);
            }

            // Re-create junction dots on the pasted wires (deduplicated).
            for junction in clipboard_junctions {
                s.add_junction(Point::new(junction.x + offset_x, junction.y + offset_y));
            }

            s.is_dirty = true;
            s.bump_topology_version();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_drops_malformed_wires_from_corrupt_import_state() {
        let mut schematic = SchematicState::default();
        schematic.wires.push(Wire::new(10, Vec::new()));
        schematic.wires.push(Wire::new(11, vec![Point::new(5, 5)]));
        schematic.selection.select_wire(10);
        schematic.selection.select_wire(11);
        let original_wire_count = schematic.wires.len();

        schematic.copy_selection();

        assert!(
            schematic.clipboard.wires.is_empty(),
            "malformed wires must not propagate into clipboard state"
        );
        schematic.paste_at(Point::new(20, 20));
        assert_eq!(
            schematic.wires.len(),
            original_wire_count,
            "paste must not create additional invalid wires"
        );
    }
}
