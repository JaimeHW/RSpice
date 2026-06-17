use super::super::*;

impl SchematicState {
    /// Move a component and update all attached wire endpoints (rubber-banding)
    pub fn move_component_with_wires(&mut self, component_id: u64, delta: Point) {
        self.move_component_with_wires_resolved(component_id, delta, legacy_terminal_points);
    }

    /// Move a component and update attached wire endpoints using caller-supplied terminal geometry.
    pub fn move_component_with_wires_resolved(
        &mut self,
        component_id: u64,
        delta: Point,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        // Get the component's terminal positions BEFORE moving
        let terminals: Vec<Point> = {
            if let Some(comp) = self.components.iter().find(|c| c.id == component_id) {
                terminal_points_for(comp)
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
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id)
                && point_idx < wire.points.len()
            {
                wire.points[point_idx] = new_pos;
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
    /// Runs on every drag frame: one O(1)-membership pass over the wires —
    /// no nested terminal scans, no per-update id searches.
    pub fn move_selection_with_rubber_band(&mut self, delta: Point) {
        self.move_selection_with_rubber_band_resolved(delta, legacy_terminal_points);
    }

    /// Move selected components and rubber-band wires using caller-supplied terminal geometry.
    pub fn move_selection_with_rubber_band_resolved(
        &mut self,
        delta: Point,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        if self.selection.components.is_empty() && self.selection.wires.is_empty() {
            return;
        }

        // Terminal positions of every selected component, BEFORE moving.
        let mut terminals: std::collections::HashSet<Point> = std::collections::HashSet::new();
        for comp in self
            .components
            .iter()
            .filter(|c| self.selection.components.contains(&c.id))
        {
            terminals.extend(terminal_points_for(comp));
        }

        // Classify unselected wires in one pass, by index (stable here):
        // both ends on selected terminals → translate whole; otherwise
        // stretch every point that sits on a selected terminal.
        let mut wire_updates: Vec<(usize, usize, Point)> = Vec::new();
        let mut wires_to_move: Vec<usize> = Vec::new();

        for (wire_index, wire) in self.wires.iter().enumerate() {
            if self.selection.wires.contains(&wire.id) {
                continue; // moved wholesale below
            }
            let start_connected = wire.points.first().is_some_and(|p| terminals.contains(p));
            let end_connected = wire.points.last().is_some_and(|p| terminals.contains(p));

            if start_connected && end_connected {
                wires_to_move.push(wire_index);
            } else {
                for (point_idx, point) in wire.points.iter().enumerate() {
                    if terminals.contains(point) {
                        let new_pos = Point::new(point.x + delta.x, point.y + delta.y);
                        wire_updates.push((wire_index, point_idx, new_pos));
                    }
                }
            }
        }

        // Move selected components.
        for comp in self
            .components
            .iter_mut()
            .filter(|c| self.selection.components.contains(&c.id))
        {
            comp.pos.x += delta.x;
            comp.pos.y += delta.y;
        }

        // Move selected wires wholesale.
        for wire in self
            .wires
            .iter_mut()
            .filter(|w| self.selection.wires.contains(&w.id))
        {
            for point in &mut wire.points {
                point.x += delta.x;
                point.y += delta.y;
            }
        }

        // Move fully attached wires.
        for wire_index in wires_to_move {
            if let Some(wire) = self.wires.get_mut(wire_index) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Apply stretch updates for partially connected wires.
        for (wire_index, point_idx, new_pos) in wire_updates {
            if let Some(wire) = self.wires.get_mut(wire_index)
                && point_idx < wire.points.len()
            {
                wire.points[point_idx] = new_pos;
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
        self.move_selection_resolved(delta, legacy_terminal_points);
    }

    /// Move all selected components and wires using caller-supplied terminal geometry.
    pub fn move_selection_resolved(
        &mut self,
        delta: Point,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        // Union of selected components' terminals, BEFORE moving.
        let mut terminals: std::collections::HashSet<Point> = std::collections::HashSet::new();
        for comp in self
            .components
            .iter()
            .filter(|c| self.selection.components.contains(&c.id))
        {
            terminals.extend(terminal_points_for(comp));
        }

        // Rubber-band stretch: any unselected wire point on a selected
        // terminal follows it.
        if !terminals.is_empty() {
            for wire in self
                .wires
                .iter_mut()
                .filter(|w| !self.selection.wires.contains(&w.id))
            {
                for point in &mut wire.points {
                    if terminals.contains(point) {
                        point.x += delta.x;
                        point.y += delta.y;
                    }
                }
            }
        }

        // Move the selected components.
        for comp in self
            .components
            .iter_mut()
            .filter(|c| self.selection.components.contains(&c.id))
        {
            comp.pos.x += delta.x;
            comp.pos.y += delta.y;
        }

        // Move selected wires entirely, tracking endpoints for junctions.
        let mut wire_endpoints: Vec<Point> = Vec::new();
        for wire in self
            .wires
            .iter_mut()
            .filter(|w| self.selection.wires.contains(&w.id))
        {
            if let Some(first) = wire.points.first() {
                wire_endpoints.push(*first);
            }
            if let Some(last) = wire.points.last() {
                wire_endpoints.push(*last);
            }
            for point in &mut wire.points {
                point.x += delta.x;
                point.y += delta.y;
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
        Cell, Component, ComponentType, Library, LibraryCellInstance, LibraryManager,
        PortDirection, PortSpec, ResolvedCellSymbol, SchematicState, SymbolDocument, SymbolPin,
        SymbolResolver, View, ViewType, Wire,
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

    fn selected_amp_with_wire(resolved_pin: Point) -> SchematicState {
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
        schematic.wires.push(Wire::segment(
            2,
            resolved_pin,
            Point::new(resolved_pin.x, 0),
        ));
        schematic.selection.select_component(1);
        schematic
    }

    #[test]
    fn moving_selected_cell_uses_resolved_symbol_terminals_for_wire_updates() {
        let resolved = resolved_amp_symbol();
        let mut schematic = selected_amp_with_wire(Point::new(60, 40));

        schematic.move_selection_with_rubber_band_resolved(Point::new(10, 5), |component| {
            resolved_terminal_points(component, &resolved)
        });

        assert_eq!(schematic.components[0].pos, Point::new(110, 55));
        assert_eq!(schematic.wires[0].points[0], Point::new(70, 45));
        assert_eq!(schematic.wires[0].points[1], Point::new(60, 0));
    }
}
