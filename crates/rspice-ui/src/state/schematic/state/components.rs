use super::*;

impl SchematicState {
    // =========================================================================
    // Component Management
    // =========================================================================

    /// Add a component at the given position
    pub fn add_component(&mut self, kind: ComponentType, pos: Point) -> u64 {
        let id = self.next_id();
        let name = self.generate_name(kind);
        let mut component = Component::new(id, kind, pos);
        component.name = name;
        component.rotation = self.preview_rotation;

        // Set default values
        component.value = kind.default_value().to_string();

        // A port's value IS its interface name — every placement gets a
        // fresh one so two new ports never silently short their nets.
        if kind == ComponentType::Port {
            component.value = self.next_port_name();
        }

        self.components.push(component);
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// First unused `p<N>` port name in this schematic.
    fn next_port_name(&self) -> String {
        let taken: std::collections::HashSet<String> = self
            .components
            .iter()
            .filter(|c| c.kind == ComponentType::Port)
            .map(|c| c.value.trim().to_ascii_lowercase())
            .collect();
        (1..)
            .map(|n| format!("p{n}"))
            .find(|candidate| !taken.contains(candidate))
            .expect("unbounded name space")
    }

    /// Add a generic library/cell/view instance at the given position.
    pub fn add_library_cell_component(
        &mut self,
        pos: Point,
        library_cell: LibraryCellInstance,
    ) -> u64 {
        let id = self.next_id();
        let name = self.generate_name(ComponentType::CellInstance);
        let mut component = Component::new(id, ComponentType::CellInstance, pos);
        component.name = name;
        component.rotation = self.preview_rotation;
        component.value = library_cell.cell.clone();
        component.library_cell = Some(library_cell);

        self.components.push(component);
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Find component at grid position
    pub fn component_at(&self, pos: Point) -> Option<u64> {
        // Check terminals first (precise connection points)
        for comp in &self.components {
            let terminals = comp.terminal_positions();
            for (_, term_pos) in terminals {
                if term_pos == pos {
                    return Some(comp.id);
                }
            }
        }
        // Then check component bounding boxes (uses symbol_dimensions for accurate hit detection)
        for comp in &self.components {
            if comp.contains_point(pos) {
                return Some(comp.id);
            }
        }
        None
    }

    /// Rotate selected components
    pub fn rotate_selection(&mut self) {
        self.transform_selection("rotate selection", |c| c.rotation = c.rotation.rotate_cw());
    }

    /// Mirror selected components horizontally (flip about Y-axis)
    ///
    /// This flips components left-to-right, swapping terminal positions.
    /// Essential for proper transistor orientation in circuit design.
    /// Matches Cadence Virtuoso 'H' key behavior.
    pub fn mirror_selection_h(&mut self) {
        self.transform_selection("mirror horizontally", |c| c.toggle_mirror_h());
    }

    /// Mirror selected components vertically (flip about X-axis)
    ///
    /// This flips components up-to-down, swapping terminal positions.
    /// Matches Cadence Virtuoso 'V' key behavior.
    pub fn mirror_selection_v(&mut self) {
        self.transform_selection("mirror vertically", |c| c.toggle_mirror_v());
    }

    /// Apply an in-place transform (rotate/mirror) to every selected
    /// component as one undoable operation, dragging attached wire points
    /// along so connectivity survives the transform — terminals move under
    /// rotation/mirror, and a wire endpoint left on the old position would
    /// silently disconnect.
    fn transform_selection(&mut self, description: &str, transform: impl Fn(&mut Component)) {
        if self.selection.components.is_empty() {
            return;
        }
        let ids: Vec<u64> = self.selection.components.iter().copied().collect();
        self.with_undo(description, |s| {
            for id in ids {
                let Some(index) = s.components.iter().position(|c| c.id == id) else {
                    continue;
                };
                let before: Vec<Point> = s.components[index]
                    .terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect();
                transform(&mut s.components[index]);
                let after: Vec<Point> = s.components[index]
                    .terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect();

                // Terminal order is positional and stable across transforms.
                // Collect updates against the pre-transform wire state, then
                // apply: a mirror swaps terminal positions, so remapping
                // in place would move the same wire point twice.
                let mut updates: Vec<(usize, usize, Point)> = Vec::new();
                for (old_pos, new_pos) in before.iter().zip(after.iter()) {
                    if old_pos == new_pos {
                        continue;
                    }
                    for (wire_index, wire) in s.wires.iter().enumerate() {
                        for (point_index, point) in wire.points.iter().enumerate() {
                            if point == old_pos {
                                updates.push((wire_index, point_index, *new_pos));
                            }
                        }
                    }
                }
                for (wire_index, point_index, new_pos) in updates {
                    s.wires[wire_index].points[point_index] = new_pos;
                }
            }
            s.is_dirty = true;
            s.bump_topology_version();
        });
    }
}
