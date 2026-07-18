use super::super::super::BusTargetKind;
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
        if self.read_only || delta == Point::origin() {
            return;
        }
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
        let mut moved_wire_points = std::collections::HashSet::new();

        for wire in &self.wires {
            for (point_idx, point) in wire.points.iter().enumerate() {
                for term_pos in &terminals {
                    if *point == *term_pos {
                        let new_pos = offset_point(*term_pos, delta);
                        wire_updates.push((wire.id, point_idx, new_pos));
                        moved_wire_points.insert(*point);
                        break;
                    }
                }
            }
        }

        // Move the component
        if let Some(comp) = self.components.iter_mut().find(|c| c.id == component_id) {
            comp.pos = offset_point(comp.pos, delta);
        }

        // Apply wire updates
        for (wire_id, point_idx, new_pos) in wire_updates {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id)
                && point_idx < wire.points.len()
            {
                wire.points[point_idx] = new_pos;
            }
        }

        for tap in &mut self.bus_taps {
            if tap.target_kind() == BusTargetKind::Wire
                && moved_wire_points.contains(&tap.connection_point)
            {
                tap.connection_point = offset_point(tap.connection_point, delta);
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
        if self.read_only
            || delta == Point::origin()
            || (self.selection.components.is_empty()
                && self.selection.wires.is_empty()
                && self.selection.buses.is_empty()
                && self.selection.bus_taps.is_empty())
        {
            return;
        }
        let mut tap_targets_moving_conductor = tap_targets_selected_conductor(self);

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
                        let new_pos = offset_point(*point, delta);
                        wire_updates.push((wire_index, point_idx, new_pos));
                    }
                }
            }
        }

        for tap in self
            .bus_taps
            .iter()
            .filter(|tap| tap.target_kind() == BusTargetKind::Wire)
        {
            let target_follows = wires_to_move
                .iter()
                .any(|index| self.wires[*index].contains_point(tap.connection_point))
                || wire_updates.iter().any(|(wire_index, point_index, _)| {
                    self.wires[*wire_index].points.get(*point_index) == Some(&tap.connection_point)
                });
            if target_follows {
                tap_targets_moving_conductor.insert(tap.id);
            }
        }

        // Move selected components.
        for comp in self
            .components
            .iter_mut()
            .filter(|c| self.selection.components.contains(&c.id))
        {
            comp.pos = offset_point(comp.pos, delta);
        }

        // Move selected wires wholesale.
        for wire in self
            .wires
            .iter_mut()
            .filter(|w| self.selection.wires.contains(&w.id))
        {
            for point in &mut wire.points {
                *point = offset_point(*point, delta);
            }
        }

        move_selected_bus_geometry(self, delta, &tap_targets_moving_conductor);

        // Move fully attached wires.
        for wire_index in wires_to_move {
            if let Some(wire) = self.wires.get_mut(wire_index) {
                for point in &mut wire.points {
                    *point = offset_point(*point, delta);
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
        if self.read_only
            || delta == Point::origin()
            || self.wires.iter().all(|wire| wire.id != wire_id)
        {
            return;
        }
        let attached_taps: std::collections::HashSet<u64> = self
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .map(|wire| {
                self.bus_taps
                    .iter()
                    .filter(|tap| {
                        tap.target_kind() == BusTargetKind::Wire
                            && wire.contains_point(tap.connection_point)
                    })
                    .map(|tap| tap.id)
                    .collect()
            })
            .unwrap_or_default();
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
                *point = offset_point(*point, delta);
            }
        }

        for old_pt in old_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos = offset_point(junction.pos, delta);
            }
        }

        for tap in &mut self.bus_taps {
            if attached_taps.contains(&tap.id) {
                tap.connection_point = offset_point(tap.connection_point, delta);
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
        if self.read_only
            || delta == Point::origin()
            || (self.selection.components.is_empty()
                && self.selection.wires.is_empty()
                && self.selection.buses.is_empty()
                && self.selection.bus_taps.is_empty())
        {
            return;
        }
        let mut tap_targets_moving_conductor = tap_targets_selected_conductor(self);
        // Union of selected components' terminals, BEFORE moving.
        let mut terminals: std::collections::HashSet<Point> = std::collections::HashSet::new();
        for comp in self
            .components
            .iter()
            .filter(|c| self.selection.components.contains(&c.id))
        {
            terminals.extend(terminal_points_for(comp));
        }

        for tap in self
            .bus_taps
            .iter()
            .filter(|tap| tap.target_kind() == BusTargetKind::Wire)
        {
            let follows_rubber_band = self.wires.iter().any(|wire| {
                !self.selection.wires.contains(&wire.id)
                    && wire
                        .points
                        .iter()
                        .any(|point| *point == tap.connection_point && terminals.contains(point))
            });
            if follows_rubber_band {
                tap_targets_moving_conductor.insert(tap.id);
            }
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
                        *point = offset_point(*point, delta);
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
            comp.pos = offset_point(comp.pos, delta);
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
                *point = offset_point(*point, delta);
            }
        }

        move_selected_bus_geometry(self, delta, &tap_targets_moving_conductor);

        // Move junctions at selected wire endpoints
        for old_pt in wire_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos = offset_point(junction.pos, delta);
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all wire points at a junction to a new position
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) {
        if self.read_only || old_pos == new_pos {
            return;
        }
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

        for tap in &mut self.bus_taps {
            if tap.target_kind() == BusTargetKind::Wire && tap.connection_point == old_pos {
                tap.connection_point = new_pos;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }
}

fn tap_targets_selected_conductor(state: &SchematicState) -> std::collections::HashSet<u64> {
    state
        .bus_taps
        .iter()
        .filter(|tap| match tap.target_kind() {
            BusTargetKind::Wire => state.wires.iter().any(|wire| {
                state.selection.wires.contains(&wire.id)
                    && wire.contains_point(tap.connection_point)
            }),
            BusTargetKind::Bus => state.buses.iter().any(|bus| {
                state.selection.buses.contains(&bus.id) && bus.contains_point(tap.connection_point)
            }),
        })
        .map(|tap| tap.id)
        .collect()
}

fn move_selected_bus_geometry(
    state: &mut SchematicState,
    delta: Point,
    tap_targets_moving_conductor: &std::collections::HashSet<u64>,
) {
    let selected_bus_ids = state.selection.buses.clone();
    for bus in state
        .buses
        .iter_mut()
        .filter(|bus| selected_bus_ids.contains(&bus.id))
    {
        bus.translate(delta);
    }
    for tap in &mut state.bus_taps {
        if selected_bus_ids.contains(&tap.bus_id) {
            tap.bus_point = offset_point(tap.bus_point, delta);
        }
        if tap_targets_moving_conductor.contains(&tap.id)
            || state.selection.bus_taps.contains(&tap.id)
        {
            tap.connection_point = offset_point(tap.connection_point, delta);
        }
    }
}

fn offset_point(point: Point, delta: Point) -> Point {
    Point::new(
        point.x.saturating_add(delta.x),
        point.y.saturating_add(delta.y),
    )
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
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Cell, Component, ComponentType,
        Junction, Library, LibraryCellInstance, LibraryManager, PortDirection, PortSpec,
        ResolvedCellSymbol, SchematicState, SymbolDocument, SymbolPin, SymbolResolver, View,
        ViewType, Wire,
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

    fn bus_tap_and_scalar_target() -> SchematicState {
        let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
        let bus = Bus::segment(1, Point::new(0, 0), Point::new(10, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            2,
            &bus,
            Point::new(5, 0),
            Point::new(0, 10),
            BusSlice::parse("DATA[2]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut state = SchematicState::default();
        state.buses.push(bus);
        state.bus_taps.push(tap);
        state
            .wires
            .push(Wire::segment(3, Point::new(0, 10), Point::new(10, 10)));
        state
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

    #[test]
    fn source_only_target_only_and_joint_moves_preserve_tap_attachments() {
        let mut source_only = bus_tap_and_scalar_target();
        source_only.selection.select_only_bus(1);
        source_only.move_selection(Point::new(10, 0));
        assert_eq!(source_only.bus_taps[0].bus_point, Point::new(15, 0));
        assert_eq!(source_only.bus_taps[0].connection_point, Point::new(0, 10));

        let mut target_only = bus_tap_and_scalar_target();
        target_only.selection.select_only_wire(3);
        target_only.move_selection(Point::new(0, 10));
        assert_eq!(target_only.bus_taps[0].bus_point, Point::new(5, 0));
        assert_eq!(target_only.bus_taps[0].connection_point, Point::new(0, 20));

        let mut joint = bus_tap_and_scalar_target();
        joint.selection.select_bus(1);
        joint.selection.select_wire(3);
        joint.move_selection(Point::new(4, 6));
        assert_eq!(joint.bus_taps[0].bus_point, Point::new(9, 6));
        assert_eq!(joint.bus_taps[0].connection_point, Point::new(4, 16));
    }

    #[test]
    fn component_rubber_band_and_junction_moves_keep_scalar_tap_attached() {
        let delta = Point::new(3, 4);
        let mut direct = bus_tap_and_scalar_target();
        direct
            .components
            .push(Component::new(4, ComponentType::Resistor, Point::origin()));
        direct.move_component_with_wires_resolved(4, delta, |_| vec![Point::new(0, 10)]);
        assert_eq!(direct.bus_taps[0].connection_point, Point::new(3, 14));

        let mut selected = bus_tap_and_scalar_target();
        selected
            .components
            .push(Component::new(4, ComponentType::Resistor, Point::origin()));
        selected.selection.select_only_component(4);
        selected.move_selection_resolved(delta, |_| vec![Point::new(0, 10)]);
        assert_eq!(selected.bus_taps[0].connection_point, Point::new(3, 14));

        let mut junction = bus_tap_and_scalar_target();
        junction.junctions.push(Junction::new(5, Point::new(0, 10)));
        junction.move_junction(Point::new(0, 10), Point::new(-2, 12));
        assert_eq!(junction.bus_taps[0].connection_point, Point::new(-2, 12));
    }

    #[test]
    fn move_wire_rejects_missing_and_zero_delta_without_document_side_effects() {
        let mut schematic = bus_tap_and_scalar_target();
        schematic.is_dirty = false;
        let topology_before = schematic.topology_version();

        schematic.move_wire(999, Point::new(10, 10));
        schematic.move_wire(3, Point::origin());

        assert!(!schematic.is_dirty);
        assert_eq!(schematic.topology_version(), topology_before);
    }

    #[test]
    fn extreme_selection_moves_saturate_every_attached_geometry() {
        let mut schematic = bus_tap_and_scalar_target();
        schematic.selection.select_bus(1);
        schematic.selection.select_wire(3);

        schematic.move_selection(Point::new(i32::MAX, i32::MAX));

        assert_eq!(schematic.buses[0].points[1], Point::new(i32::MAX, i32::MAX));
        assert_eq!(schematic.wires[0].points[1], Point::new(i32::MAX, i32::MAX));
        assert_eq!(
            schematic.bus_taps[0].connection_point,
            Point::new(i32::MAX, i32::MAX)
        );
    }

    #[test]
    fn bus_and_tap_move_is_one_undoable_redoable_drag_transaction() {
        let mut schematic = bus_tap_and_scalar_target();
        schematic.selection.select_only_bus(1);
        let original_bus = schematic.buses[0].clone();
        let original_tap = schematic.bus_taps[0].clone();

        schematic.begin_operation("move selection");
        schematic.move_selection(Point::new(3, 4));
        schematic.move_selection(Point::new(2, 1));
        assert!(schematic.end_operation());
        let moved_bus = schematic.buses[0].clone();
        let moved_tap = schematic.bus_taps[0].clone();

        assert!(schematic.undo());
        assert_eq!(schematic.buses[0], original_bus);
        assert_eq!(schematic.bus_taps[0], original_tap);
        assert!(schematic.redo());
        assert_eq!(schematic.buses[0], moved_bus);
        assert_eq!(schematic.bus_taps[0], moved_tap);
    }
}
