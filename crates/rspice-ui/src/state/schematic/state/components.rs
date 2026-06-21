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
        self.rotate_selection_resolved(legacy_terminal_points);
    }

    /// Rotate selected components using caller-supplied terminal geometry for wire remapping.
    pub fn rotate_selection_resolved(
        &mut self,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        self.transform_selection_resolved("rotate selection", terminal_points_for, |c| {
            c.rotation = c.rotation.rotate_cw()
        });
    }

    /// Mirror selected components horizontally (flip about Y-axis)
    ///
    /// This flips components left-to-right, swapping terminal positions.
    /// Essential for proper transistor orientation in circuit design.
    /// Matches Cadence Virtuoso 'H' key behavior.
    pub fn mirror_selection_h(&mut self) {
        self.mirror_selection_h_resolved(legacy_terminal_points);
    }

    /// Mirror selected components horizontally using caller-supplied terminal geometry.
    pub fn mirror_selection_h_resolved(
        &mut self,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        self.transform_selection_resolved("mirror horizontally", terminal_points_for, |c| {
            c.toggle_mirror_h()
        });
    }

    /// Mirror selected components vertically (flip about X-axis)
    ///
    /// This flips components up-to-down, swapping terminal positions.
    /// Matches Cadence Virtuoso 'V' key behavior.
    pub fn mirror_selection_v(&mut self) {
        self.mirror_selection_v_resolved(legacy_terminal_points);
    }

    /// Mirror selected components vertically using caller-supplied terminal geometry.
    pub fn mirror_selection_v_resolved(
        &mut self,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        self.transform_selection_resolved("mirror vertically", terminal_points_for, |c| {
            c.toggle_mirror_v()
        });
    }

    /// Apply an in-place transform (rotate/mirror) to every selected
    /// component as one undoable operation, dragging attached wire points
    /// along so connectivity survives the transform — terminals move under
    /// rotation/mirror, and a wire endpoint left on the old position would
    /// silently disconnect.
    fn transform_selection_resolved(
        &mut self,
        description: &str,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        transform: impl Fn(&mut Component),
    ) {
        if self.selection.components.is_empty() {
            return;
        }
        let mut ids: Vec<u64> = self.selection.components.iter().copied().collect();
        ids.sort_unstable();
        ids.retain(|id| self.components.iter().any(|component| component.id == *id));
        if ids.is_empty() {
            return;
        }
        self.with_undo(description, move |s| {
            let before: Vec<(u64, Vec<Point>)> = ids
                .iter()
                .filter_map(|&id| {
                    let component = s.components.iter().find(|component| component.id == id)?;
                    Some((id, terminal_points_for(component)))
                })
                .collect();

            for &id in &ids {
                let Some(index) = s.components.iter().position(|component| component.id == id)
                else {
                    continue;
                };
                transform(&mut s.components[index]);
            }

            // Terminal order is positional and stable across transforms.
            // Build one old->new table for the whole selection, then apply it
            // once to the original wire state so selected components cannot
            // remap each other's freshly moved endpoints.
            let mut remaps: Vec<(Point, Point)> = Vec::new();
            for (id, before_points) in before {
                let Some(component) = s.components.iter().find(|component| component.id == id)
                else {
                    continue;
                };
                let after_points = terminal_points_for(component);
                for (old_pos, new_pos) in before_points.into_iter().zip(after_points) {
                    if old_pos != new_pos {
                        remaps.push((old_pos, new_pos));
                    }
                }
            }

            let mut updates: Vec<(usize, usize, Point)> = Vec::new();
            for (wire_index, wire) in s.wires.iter().enumerate() {
                for (point_index, point) in wire.points.iter().enumerate() {
                    if let Some((_, new_pos)) = remaps.iter().find(|(old_pos, _)| point == old_pos)
                    {
                        updates.push((wire_index, point_index, *new_pos));
                    }
                }
            }

            for (wire_index, point_index, new_pos) in updates {
                s.wires[wire_index].points[point_index] = new_pos;
            }
            s.is_dirty = true;
            s.bump_topology_version();
        });
    }
}

fn legacy_terminal_points(component: &Component) -> Vec<Point> {
    component
        .terminal_positions()
        .into_iter()
        .map(|(_, pos)| pos)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Library, LibraryManager, PortDirection, PortSpec, ResolvedCellSymbol, SymbolDocument,
        SymbolPin, SymbolResolver, View, ViewType, Wire,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn resolved_amp_symbol() -> ResolvedCellSymbol {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };

        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        SymbolResolver::new(&libraries, &HashMap::new())
            .resolve_binding(&binding)
            .expect("symbol resolves")
    }

    fn resolved_terminal_points(
        component: &Component,
        resolved: &ResolvedCellSymbol,
    ) -> Vec<Point> {
        component
            .terminal_positions_resolved(Some(resolved))
            .into_iter()
            .map(|(_, pos)| pos)
            .collect()
    }

    fn select_ids_in_iteration_order(schematic: &mut SchematicState) -> (u64, u64) {
        for first in 1..200 {
            for second in 1..200 {
                if first == second {
                    continue;
                }
                schematic.selection.components.clear();
                schematic.selection.select_component(first);
                schematic.selection.select_component(second);
                let order: Vec<u64> = schematic.selection.components.iter().copied().collect();
                if order == [first, second] {
                    return (first, second);
                }
            }
        }
        panic!("could not find deterministic selected-component order");
    }

    #[test]
    fn rotating_selected_cell_uses_resolved_symbol_terminals_for_wire_updates() {
        let resolved = resolved_amp_symbol();
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let mut schematic = SchematicState::default();
        schematic.components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding),
        );
        schematic
            .wires
            .push(Wire::segment(2, Point::new(60, 40), Point::new(60, 0)));
        schematic.selection.select_component(1);

        schematic
            .rotate_selection_resolved(|component| resolved_terminal_points(component, &resolved));

        assert_eq!(schematic.components[0].rotation, Rotation::R90);
        assert_eq!(schematic.wires[0].points[0], Point::new(110, 10));
        assert_eq!(schematic.wires[0].points[1], Point::new(60, 0));
    }

    #[test]
    fn rotating_multiple_selected_cells_applies_wire_remap_once() {
        let resolved = resolved_amp_symbol();
        let mut schematic = SchematicState::default();
        let (first_id, second_id) = select_ids_in_iteration_order(&mut schematic);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        schematic.components.push(
            Component::new(first_id, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding.clone()),
        );
        schematic.components.push(
            Component::new(second_id, ComponentType::CellInstance, Point::new(150, 20))
                .with_library_cell(binding),
        );
        schematic
            .wires
            .push(Wire::segment(9, Point::new(60, 40), Point::new(60, 0)));

        schematic
            .rotate_selection_resolved(|component| resolved_terminal_points(component, &resolved));

        assert_eq!(
            schematic.wires[0].points[0],
            Point::new(110, 10),
            "wire endpoint should follow the first component's pin once, not then match and follow \
             the second component's old pin"
        );
    }

    #[test]
    fn rotating_stale_component_selection_is_noop() {
        let mut schematic = SchematicState::default();
        schematic.selection.select_component(404);
        let topology_version = schematic.topology_version();

        schematic.rotate_selection();

        assert!(!schematic.is_dirty);
        assert_eq!(schematic.topology_version(), topology_version);
        assert!(schematic.components.is_empty());
    }
}
