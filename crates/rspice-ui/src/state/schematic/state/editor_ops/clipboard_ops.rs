use super::super::*;

impl SchematicState {
    // =========================================================================
    // Clipboard Operations
    // =========================================================================

    /// Copy selected components, wires, and explicit junctions to clipboard.
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
        let mut junctions_to_copy: Vec<Point> = self
            .junctions
            .iter()
            .map(|j| j.pos)
            .filter(|pos| {
                self.selection.has_junction(*pos)
                    || wires_to_copy.iter().any(|wire| wire.contains_point(*pos))
            })
            .collect();
        junctions_to_copy.sort_by_key(|point| (point.x, point.y));
        junctions_to_copy.dedup();

        self.clipboard =
            ClipboardData::from_selection(selected_comps, wires_to_copy, junctions_to_copy);
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position (one undo entry)
    pub fn paste_at(&mut self, pos: Point) -> bool {
        if !self.can_paste() {
            return false;
        }

        let junction_only = self.clipboard.components.is_empty() && self.clipboard.wires.is_empty();
        // A junction-only clipboard is a connectivity edit, not decoration.
        // Snap its anchor through the same ambiguous-crossing candidate set as
        // the junction tool, then reject it before opening an undo transaction
        // unless at least one translated marker would create a new connection.
        let paste_pos = if junction_only {
            let Some(candidate) = self.nearest_junction_candidate(pos, self.grid_size) else {
                return false;
            };
            candidate
        } else {
            pos
        };
        if junction_only {
            let offset_x = paste_pos.x - self.clipboard.origin.x;
            let offset_y = paste_pos.y - self.clipboard.origin.y;
            let has_valid_target = self.clipboard.junctions.iter().any(|junction| {
                let target = Point::new(junction.x + offset_x, junction.y + offset_y);
                !self.has_junction(target)
                    && self.nearest_junction_candidate(target, 0) == Some(target)
            });
            if !has_valid_target {
                return false;
            }
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

            let offset_x = paste_pos.x - origin.x;
            let offset_y = paste_pos.y - origin.y;

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

            // Re-create junction dots only where at least two distinct wires
            // meet. This makes junction-only copy/paste useful without ever
            // manufacturing an electrically meaningless floating marker.
            for junction in clipboard_junctions {
                let target = Point::new(junction.x + offset_x, junction.y + offset_y);
                let valid_target = if junction_only {
                    s.nearest_junction_candidate(target, 0) == Some(target)
                } else {
                    s.wires
                        .iter()
                        .filter(|wire| wire.contains_point(target))
                        .map(|wire| wire.id)
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                        >= 2
                };
                if valid_target && !s.has_junction(target) {
                    s.add_junction(target);
                    s.selection.select_junction(target);
                }
            }

            s.is_dirty = true;
            s.bump_topology_version();
        })
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

    #[test]
    fn junction_only_clipboard_pastes_only_on_a_valid_intersection() {
        let source = Point::new(20, 20);
        let target = Point::new(80, 80);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
            Wire::new(3, vec![Point::new(60, 80), Point::new(100, 80)]),
            Wire::new(4, vec![Point::new(80, 60), Point::new(80, 100)]),
        ];
        schematic.add_junction(source);
        schematic.selection.select_only_junction(source);
        schematic.copy_selection();

        assert!(schematic.can_paste());
        assert_eq!(schematic.clipboard.origin, source);
        assert!(schematic.paste_at(Point::new(target.x + 1, target.y - 1)));

        assert!(schematic.has_junction(target));
        assert!(schematic.selection.has_junction(target));
        assert!(schematic.can_undo());
    }

    #[test]
    fn junction_only_paste_rejects_empty_space_without_an_undo_step() {
        let source = Point::new(20, 20);
        let mut schematic = SchematicState::default();
        schematic.wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
        ];
        schematic.add_junction(source);
        schematic.selection.select_only_junction(source);
        schematic.copy_selection();

        assert!(!schematic.paste_at(Point::new(200, 200)));
        assert!(!schematic.can_undo());
        assert_eq!(schematic.junctions.len(), 1);
    }
}
