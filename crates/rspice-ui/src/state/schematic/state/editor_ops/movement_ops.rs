//! Moving a selection.
//!
//! Dragging components and wires while keeping the connections that cross
//! the selection boundary attached — the rubber-band behaviour a schematic
//! editor is judged on. A wire with one end inside the selection stretches;
//! a wire wholly inside travels.

use super::super::super::{BusTargetKind, clamped_documentation_shape_translation};
use super::super::*;

const MAX_SHOVE_LANES: i32 = 8;

impl SchematicState {
    /// Whether the current selection still resolves to at least one object
    /// supported by selection movement.
    pub fn has_live_movable_selection(&self) -> bool {
        has_live_movable_selection(self)
    }

    /// Number of selected movable objects that still exist in this document.
    pub fn live_movable_selection_count(&self) -> usize {
        self.components
            .iter()
            .filter(|item| self.selection.has_component(item.id))
            .count()
            + self
                .wires
                .iter()
                .filter(|item| self.selection.has_wire(item.id))
                .count()
            + self
                .buses
                .iter()
                .filter(|item| self.selection.has_bus(item.id))
                .count()
            + self
                .bus_taps
                .iter()
                .filter(|item| self.selection.has_bus_tap(item.id))
                .count()
            + self
                .net_labels
                .iter()
                .filter(|item| self.selection.has_net_label(item.id))
                .count()
            + self
                .design_notes
                .iter()
                .filter(|item| self.selection.has_design_note(item.id))
                .count()
            + self
                .documentation_shapes
                .iter()
                .filter(|item| self.selection.has_documentation_shape(item.id))
                .count()
    }

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

    /// Move selected components, labels, buses, taps, and wires while
    /// rubber-banding connected wires.
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
        if self.read_only || delta == Point::origin() || !has_live_movable_selection(self) {
            return;
        }
        let documentation_shape_delta = clamped_documentation_shape_translation(
            self.documentation_shapes
                .iter()
                .filter(|shape| self.selection.has_documentation_shape(shape.id)),
            delta,
        );
        let electrical_selection = has_live_electrical_selection(self);
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

        for label in self
            .net_labels
            .iter_mut()
            .filter(|label| self.selection.has_net_label(label.id))
        {
            label.pos = offset_point(label.pos, delta);
        }

        for note in self
            .design_notes
            .iter_mut()
            .filter(|note| self.selection.has_design_note(note.id))
        {
            note.translate(delta);
        }

        for shape in self
            .documentation_shapes
            .iter_mut()
            .filter(|shape| self.selection.has_documentation_shape(shape.id))
        {
            shape.translate(documentation_shape_delta);
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
        if electrical_selection {
            self.bump_topology_version();
        }
    }

    /// Move the current selection under an explicit connectivity policy.
    ///
    /// `Ok(false)` is a clean no-op (read-only document, zero delta, or no
    /// live movable selection). Guarded modes build and validate a candidate
    /// geometry first, so every `Err` leaves the document bit-for-bit
    /// unchanged.
    pub fn move_selection_with_mode(
        &mut self,
        delta: Point,
        mode: MoveSelectionMode,
    ) -> Result<bool, MoveSelectionError> {
        self.move_selection_with_mode_resolved(delta, mode, legacy_terminal_points)
    }

    /// Mode-aware selection movement using caller-resolved terminal geometry.
    pub fn move_selection_with_mode_resolved(
        &mut self,
        delta: Point,
        mode: MoveSelectionMode,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> Result<bool, MoveSelectionError> {
        if self.read_only || delta == Point::origin() || !has_live_movable_selection(self) {
            return Ok(false);
        }
        let terminal_points_by_component = self
            .components
            .iter()
            .map(|component| (component.id, terminal_points_for(component)))
            .collect::<std::collections::HashMap<_, _>>();
        let selected_terminals = self
            .selection
            .components
            .iter()
            .filter_map(|id| terminal_points_by_component.get(id))
            .flatten()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        let tap_motion = tap_motion_for_rigid_selection(self);
        let junctions_to_move = junctions_following_selected_wires(self);
        preflight_rigid_translation(self, delta, &tap_motion, &junctions_to_move)?;
        if mode == MoveSelectionMode::Connected {
            preflight_connected_translation(self, delta, &selected_terminals)?;
            let electrical_selection = has_live_electrical_selection(self);
            let mut candidate = self.clone();
            candidate.move_selection_with_rubber_band_resolved(delta, |component| {
                terminal_points_by_component
                    .get(&component.id)
                    .cloned()
                    .unwrap_or_default()
            });
            orthogonalize_connected_rubber_bands(self, &mut candidate)?;
            commit_movement_candidate(self, candidate, electrical_selection);
            return Ok(true);
        }

        let electrical_selection = has_live_electrical_selection(self);
        let mut candidate = self.clone();
        apply_rigid_selection_translation(&mut candidate, delta, &tap_motion, &junctions_to_move);

        if mode == MoveSelectionMode::Shove {
            let candidate_terminals =
                translated_terminal_points(self, delta, &terminal_points_by_component)?;
            shove_attached_wires(
                self,
                &mut candidate,
                delta,
                &selected_terminals,
                &terminal_points_by_component,
                &candidate_terminals,
            )?;
            reject_new_selected_conductor_overlaps(self, &candidate)?;
            reject_selected_wire_component_body_overlaps(&candidate, &candidate_terminals)?;
        } else {
            update_connections_after_rigid_move(&mut candidate);
        }
        validate_moved_tap_sources(&candidate, &tap_motion)?;
        if mode == MoveSelectionMode::Shove {
            validate_moved_tap_targets(&candidate, &tap_motion)?;
        }

        commit_movement_candidate(self, candidate, electrical_selection);
        Ok(true)
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

    /// Move all selected complete objects supported by selection dragging.
    pub fn move_selection(&mut self, delta: Point) {
        self.move_selection_resolved(delta, legacy_terminal_points);
    }

    /// Move all selected complete objects using caller-supplied component
    /// terminal geometry.
    pub fn move_selection_resolved(
        &mut self,
        delta: Point,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        if self.read_only || delta == Point::origin() || !has_live_movable_selection(self) {
            return;
        }
        let documentation_shape_delta = clamped_documentation_shape_translation(
            self.documentation_shapes
                .iter()
                .filter(|shape| self.selection.has_documentation_shape(shape.id)),
            delta,
        );
        let electrical_selection = has_live_electrical_selection(self);
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

        for label in self
            .net_labels
            .iter_mut()
            .filter(|label| self.selection.has_net_label(label.id))
        {
            label.pos = offset_point(label.pos, delta);
        }

        for note in self
            .design_notes
            .iter_mut()
            .filter(|note| self.selection.has_design_note(note.id))
        {
            note.translate(delta);
        }

        for shape in self
            .documentation_shapes
            .iter_mut()
            .filter(|shape| self.selection.has_documentation_shape(shape.id))
        {
            shape.translate(documentation_shape_delta);
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
        if electrical_selection {
            self.bump_topology_version();
        }
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

fn has_live_movable_selection(state: &SchematicState) -> bool {
    state
        .components
        .iter()
        .any(|item| state.selection.has_component(item.id))
        || state
            .wires
            .iter()
            .any(|item| state.selection.has_wire(item.id))
        || state
            .buses
            .iter()
            .any(|item| state.selection.has_bus(item.id))
        || state
            .bus_taps
            .iter()
            .any(|item| state.selection.has_bus_tap(item.id))
        || state
            .net_labels
            .iter()
            .any(|item| state.selection.has_net_label(item.id))
        || state
            .design_notes
            .iter()
            .any(|item| state.selection.has_design_note(item.id))
        || state
            .documentation_shapes
            .iter()
            .any(|item| state.selection.has_documentation_shape(item.id))
}

fn has_live_electrical_selection(state: &SchematicState) -> bool {
    state
        .components
        .iter()
        .any(|item| state.selection.has_component(item.id))
        || state
            .wires
            .iter()
            .any(|item| state.selection.has_wire(item.id))
        || state
            .buses
            .iter()
            .any(|item| state.selection.has_bus(item.id))
        || state
            .bus_taps
            .iter()
            .any(|item| state.selection.has_bus_tap(item.id))
        || state
            .net_labels
            .iter()
            .any(|item| state.selection.has_net_label(item.id))
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

#[derive(Debug, Clone, Copy, Default)]
struct TapMotion {
    source: bool,
    target: bool,
}

fn tap_motion_for_rigid_selection(
    state: &SchematicState,
) -> std::collections::HashMap<u64, TapMotion> {
    state
        .bus_taps
        .iter()
        .map(|tap| {
            let selected = state.selection.has_bus_tap(tap.id);
            let source = selected || state.selection.has_bus(tap.bus_id);
            let target = selected
                || match tap.target_kind() {
                    BusTargetKind::Wire => state.wires.iter().any(|wire| {
                        state.selection.has_wire(wire.id)
                            && wire.contains_point(tap.connection_point)
                    }),
                    BusTargetKind::Bus => state.buses.iter().any(|bus| {
                        state.selection.has_bus(bus.id) && bus.contains_point(tap.connection_point)
                    }),
                };
            (tap.id, TapMotion { source, target })
        })
        .collect()
}

fn junctions_following_selected_wires(state: &SchematicState) -> std::collections::HashSet<Point> {
    state
        .junctions
        .iter()
        .filter_map(|junction| {
            let incident = state
                .wires
                .iter()
                .filter(|wire| wire.contains_point(junction.pos))
                .collect::<Vec<_>>();
            (!incident.is_empty()
                && incident
                    .iter()
                    .all(|wire| state.selection.has_wire(wire.id)))
            .then_some(junction.pos)
        })
        .collect()
}

fn preflight_rigid_translation(
    state: &SchematicState,
    delta: Point,
    tap_motion: &std::collections::HashMap<u64, TapMotion>,
    junctions_to_move: &std::collections::HashSet<Point>,
) -> Result<(), MoveSelectionError> {
    let selected_points = state
        .components
        .iter()
        .filter(|item| state.selection.has_component(item.id))
        .map(|item| item.pos)
        .chain(
            state
                .wires
                .iter()
                .filter(|item| state.selection.has_wire(item.id))
                .flat_map(|item| item.points.iter().copied()),
        )
        .chain(
            state
                .buses
                .iter()
                .filter(|item| state.selection.has_bus(item.id))
                .flat_map(|item| item.points.iter().copied()),
        )
        .chain(
            state
                .net_labels
                .iter()
                .filter(|item| state.selection.has_net_label(item.id))
                .map(|item| item.pos),
        )
        .chain(
            state
                .design_notes
                .iter()
                .filter(|item| state.selection.has_design_note(item.id))
                .map(|item| item.pos),
        )
        .chain(
            state
                .documentation_shapes
                .iter()
                .filter(|item| state.selection.has_documentation_shape(item.id))
                .flat_map(|item| item.geometry.points()),
        )
        .chain(junctions_to_move.iter().copied());
    for point in selected_points {
        checked_offset(point, delta)?;
    }
    for tap in &state.bus_taps {
        let motion = tap_motion.get(&tap.id).copied().unwrap_or_default();
        if motion.source {
            checked_offset(tap.bus_point, delta)?;
        }
        if motion.target {
            checked_offset(tap.connection_point, delta)?;
        }
    }
    Ok(())
}

fn preflight_connected_translation(
    state: &SchematicState,
    delta: Point,
    selected_terminals: &std::collections::HashSet<Point>,
) -> Result<(), MoveSelectionError> {
    let mut moved_wire_ids = std::collections::HashSet::new();
    let mut moved_wire_points = std::collections::HashSet::new();
    for wire in &state.wires {
        if state.selection.has_wire(wire.id) {
            moved_wire_ids.insert(wire.id);
            moved_wire_points.extend(wire.points.iter().copied());
            continue;
        }
        let start_connected = wire
            .points
            .first()
            .is_some_and(|point| selected_terminals.contains(point));
        let end_connected = wire
            .points
            .last()
            .is_some_and(|point| selected_terminals.contains(point));
        if start_connected && end_connected {
            moved_wire_ids.insert(wire.id);
            for point in &wire.points {
                checked_offset(*point, delta)?;
                moved_wire_points.insert(*point);
            }
        } else {
            for point in wire
                .points
                .iter()
                .filter(|point| selected_terminals.contains(point))
            {
                checked_offset(*point, delta)?;
                moved_wire_points.insert(*point);
            }
        }
    }
    for tap in state
        .bus_taps
        .iter()
        .filter(|tap| tap.target_kind() == BusTargetKind::Wire)
    {
        let target_moves = moved_wire_points.contains(&tap.connection_point)
            || state.wires.iter().any(|wire| {
                moved_wire_ids.contains(&wire.id) && wire.contains_point(tap.connection_point)
            });
        if target_moves {
            checked_offset(tap.connection_point, delta)?;
        }
    }
    Ok(())
}

fn orthogonalize_connected_rubber_bands(
    original: &SchematicState,
    candidate: &mut SchematicState,
) -> Result<(), MoveSelectionError> {
    for old_wire in original
        .wires
        .iter()
        .filter(|wire| !original.selection.has_wire(wire.id))
    {
        let candidate_index = candidate
            .wires
            .iter()
            .position(|wire| wire.id == old_wire.id)
            .expect("movement candidates preserve wire identities");
        let candidate_points = candidate.wires[candidate_index].points.clone();
        if candidate_points == old_wire.points {
            continue;
        }
        if !old_wire.is_orthogonal() {
            return Err(MoveSelectionError::NonOrthogonalWire {
                wire_id: old_wire.id,
            });
        }
        let (route, point_indices) = orthogonal_route_for_corresponding_points(
            old_wire.id,
            &old_wire.points,
            &candidate_points,
        )?;

        for connection in candidate
            .connections
            .iter_mut()
            .filter(|connection| connection.wire_id == old_wire.id)
        {
            connection.point_index = *point_indices.get(connection.point_index).ok_or(
                MoveSelectionError::UnsupportedInteriorConnection {
                    wire_id: old_wire.id,
                },
            )?;
        }
        candidate.wires[candidate_index].points = route;

        for tap in candidate.bus_taps.iter().filter(|tap| {
            tap.target_kind() == BusTargetKind::Wire
                && original.bus_taps.iter().any(|old_tap| {
                    old_tap.id == tap.id && old_wire.contains_point(old_tap.connection_point)
                })
        }) {
            if !candidate.wires[candidate_index].contains_point(tap.connection_point) {
                return Err(MoveSelectionError::AttachedTapCannotBePreserved { tap_id: tap.id });
            }
        }
    }
    Ok(())
}

pub(super) fn orthogonal_route_for_corresponding_points(
    wire_id: u64,
    original: &[Point],
    moved: &[Point],
) -> Result<(Vec<Point>, Vec<usize>), MoveSelectionError> {
    if original.len() != moved.len() || original.len() < 2 {
        return Err(MoveSelectionError::UnsupportedInteriorConnection { wire_id });
    }
    let changed = original
        .iter()
        .zip(moved)
        .enumerate()
        .filter_map(|(index, (before, after))| (before != after).then_some(index))
        .collect::<Vec<_>>();
    if changed == [0] {
        return endpoint_rubber_band_route(wire_id, original, moved);
    }
    if changed == [original.len() - 1] {
        let reversed_original = original.iter().rev().copied().collect::<Vec<_>>();
        let reversed_moved = moved.iter().rev().copied().collect::<Vec<_>>();
        let (mut route, reversed_indices) =
            endpoint_rubber_band_route(wire_id, &reversed_original, &reversed_moved)?;
        route.reverse();
        let route_last = route.len() - 1;
        let mut point_indices = vec![0; original.len()];
        for (reversed_index, route_index) in reversed_indices.into_iter().enumerate() {
            point_indices[original.len() - 1 - reversed_index] = route_last - route_index;
        }
        return Ok((route, point_indices));
    }
    let mut route = vec![moved[0]];
    let mut point_indices = vec![0; moved.len()];
    for index in 0..original.len() - 1 {
        let old_start = original[index];
        let old_end = original[index + 1];
        let new_start = moved[index];
        let new_end = moved[index + 1];
        let bend = if old_start.y == old_end.y {
            Point::new(new_end.x, new_start.y)
        } else if old_start.x == old_end.x {
            Point::new(new_start.x, new_end.y)
        } else {
            return Err(MoveSelectionError::NonOrthogonalWire { wire_id });
        };
        push_distinct_point(&mut route, bend);
        push_distinct_point(&mut route, new_end);
        point_indices[index + 1] = route.len() - 1;
    }
    if route
        .windows(2)
        .any(|pair| pair[0] == pair[1] || (pair[0].x != pair[1].x && pair[0].y != pair[1].y))
    {
        return Err(MoveSelectionError::NonOrthogonalWire { wire_id });
    }
    Ok((route, point_indices))
}

fn endpoint_rubber_band_route(
    wire_id: u64,
    original: &[Point],
    moved: &[Point],
) -> Result<(Vec<Point>, Vec<usize>), MoveSelectionError> {
    let old_endpoint = original[0];
    let old_neighbor = original[1];
    let new_endpoint = moved[0];
    let bend = if old_endpoint.y == old_neighbor.y {
        Point::new(old_endpoint.x, new_endpoint.y)
    } else if old_endpoint.x == old_neighbor.x {
        Point::new(new_endpoint.x, old_endpoint.y)
    } else {
        return Err(MoveSelectionError::NonOrthogonalWire { wire_id });
    };

    let mut route = vec![new_endpoint];
    push_distinct_point(&mut route, bend);
    push_distinct_point(&mut route, old_endpoint);
    if route.len() >= 2
        && positive_length_overlap(
            WireSegment::new(route[route.len() - 2], route[route.len() - 1]),
            WireSegment::new(old_endpoint, old_neighbor),
        )
    {
        route.pop();
    }

    let mut point_indices = vec![0; original.len()];
    for (index, point) in original.iter().copied().enumerate().skip(1) {
        push_distinct_point(&mut route, point);
        point_indices[index] = route.len() - 1;
    }
    if route
        .windows(2)
        .any(|pair| pair[0] == pair[1] || (pair[0].x != pair[1].x && pair[0].y != pair[1].y))
    {
        return Err(MoveSelectionError::NonOrthogonalWire { wire_id });
    }
    Ok((route, point_indices))
}

fn push_distinct_point(route: &mut Vec<Point>, point: Point) {
    if route.last() != Some(&point) {
        route.push(point);
    }
}

fn translated_terminal_points(
    original: &SchematicState,
    delta: Point,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
) -> Result<std::collections::HashMap<u64, Vec<Point>>, MoveSelectionError> {
    terminal_points_by_component
        .iter()
        .map(|(component_id, points)| {
            let moved = original.selection.has_component(*component_id);
            let points = points
                .iter()
                .map(|point| {
                    if moved {
                        checked_offset(*point, delta)
                    } else {
                        Ok(*point)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok((*component_id, points))
        })
        .collect()
}

fn apply_rigid_selection_translation(
    state: &mut SchematicState,
    delta: Point,
    tap_motion: &std::collections::HashMap<u64, TapMotion>,
    junctions_to_move: &std::collections::HashSet<Point>,
) {
    for component in state
        .components
        .iter_mut()
        .filter(|item| state.selection.has_component(item.id))
    {
        component.pos = exact_offset(component.pos, delta);
    }
    for wire in state
        .wires
        .iter_mut()
        .filter(|item| state.selection.has_wire(item.id))
    {
        for point in &mut wire.points {
            *point = exact_offset(*point, delta);
        }
    }
    for bus in state
        .buses
        .iter_mut()
        .filter(|item| state.selection.has_bus(item.id))
    {
        for point in &mut bus.points {
            *point = exact_offset(*point, delta);
        }
    }
    for label in state
        .net_labels
        .iter_mut()
        .filter(|item| state.selection.has_net_label(item.id))
    {
        label.pos = exact_offset(label.pos, delta);
    }
    for note in state
        .design_notes
        .iter_mut()
        .filter(|item| state.selection.has_design_note(item.id))
    {
        note.translate(delta);
    }
    for shape in state
        .documentation_shapes
        .iter_mut()
        .filter(|item| state.selection.has_documentation_shape(item.id))
    {
        shape.translate(delta);
    }
    for junction in &mut state.junctions {
        if junctions_to_move.contains(&junction.pos) {
            junction.pos = exact_offset(junction.pos, delta);
        }
    }
    for tap in &mut state.bus_taps {
        let motion = tap_motion.get(&tap.id).copied().unwrap_or_default();
        if motion.source {
            tap.bus_point = exact_offset(tap.bus_point, delta);
        }
        if motion.target {
            tap.connection_point = exact_offset(tap.connection_point, delta);
        }
    }
}

fn update_connections_after_rigid_move(state: &mut SchematicState) {
    state.connections.retain(|connection| {
        state.selection.has_component(connection.component_id)
            == state.selection.has_wire(connection.wire_id)
    });
}

fn validate_moved_tap_sources(
    state: &SchematicState,
    tap_motion: &std::collections::HashMap<u64, TapMotion>,
) -> Result<(), MoveSelectionError> {
    for tap in &state.bus_taps {
        let motion = tap_motion.get(&tap.id).copied().unwrap_or_default();
        if !motion.source {
            continue;
        }
        let valid = state
            .buses
            .iter()
            .find(|bus| bus.id == tap.bus_id)
            .is_some_and(|bus| tap.validate_against_bus(bus).is_ok());
        if !valid {
            return Err(MoveSelectionError::InvalidTapAttachment { tap_id: tap.id });
        }
    }
    Ok(())
}

fn validate_moved_tap_targets(
    state: &SchematicState,
    tap_motion: &std::collections::HashMap<u64, TapMotion>,
) -> Result<(), MoveSelectionError> {
    for tap in &state.bus_taps {
        if !tap_motion.get(&tap.id).is_some_and(|motion| motion.target) {
            continue;
        }
        let attached = match tap.target_kind() {
            BusTargetKind::Wire => state
                .wires
                .iter()
                .any(|wire| wire.contains_point(tap.connection_point)),
            BusTargetKind::Bus => state
                .buses
                .iter()
                .filter(|bus| bus.id != tap.bus_id)
                .any(|bus| bus.contains_point(tap.connection_point)),
        };
        if !attached {
            return Err(MoveSelectionError::AttachedTapCannotBePreserved { tap_id: tap.id });
        }
    }
    Ok(())
}

fn shove_attached_wires(
    original: &SchematicState,
    candidate: &mut SchematicState,
    delta: Point,
    selected_terminals: &std::collections::HashSet<Point>,
    original_terminals: &std::collections::HashMap<u64, Vec<Point>>,
    candidate_terminals: &std::collections::HashMap<u64, Vec<Point>>,
) -> Result<(), MoveSelectionError> {
    let stationary_terminals = original
        .components
        .iter()
        .filter(|component| !original.selection.has_component(component.id))
        .filter_map(|component| candidate_terminals.get(&component.id))
        .flatten()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut affected = original
        .wires
        .iter()
        .filter(|wire| {
            (!original.selection.has_wire(wire.id)
                && wire
                    .points
                    .iter()
                    .any(|point| selected_terminals.contains(point)))
                || original.selection.has_wire(wire.id)
        })
        .map(|wire| wire.id)
        .collect::<Vec<_>>();
    affected.sort_unstable();

    for wire_id in affected {
        let old_wire = original
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .expect("affected wire came from the source document");
        if !old_wire.is_orthogonal() {
            return Err(MoveSelectionError::NonOrthogonalWire { wire_id });
        }
        let last_index = old_wire.points.len().saturating_sub(1);
        if old_wire.points.iter().enumerate().any(|(index, point)| {
            selected_terminals.contains(point) && index != 0 && index != last_index
        }) || original.connections.iter().any(|connection| {
            connection.wire_id == wire_id
                && connection.point_index != 0
                && connection.point_index != last_index
        }) {
            return Err(MoveSelectionError::UnsupportedInteriorConnection { wire_id });
        }

        let old_start = old_wire.points[0];
        let old_end = old_wire.points[last_index];
        let selected_wire = original.selection.has_wire(wire_id);
        let start_is_stationary = selected_wire
            && (stationary_terminals.contains(&old_start)
                || endpoint_has_stationary_component_connection(original, wire_id, 0));
        let end_is_stationary = selected_wire
            && (stationary_terminals.contains(&old_end)
                || endpoint_has_stationary_component_connection(original, wire_id, last_index));
        let new_start = if start_is_stationary {
            old_start
        } else if selected_wire || selected_terminals.contains(&old_start) {
            checked_offset(old_start, delta)?
        } else {
            old_start
        };
        let new_end = if end_is_stationary {
            old_end
        } else if selected_wire || selected_terminals.contains(&old_end) {
            checked_offset(old_end, delta)?
        } else {
            old_end
        };
        let route = orthogonal_route_candidates(new_start, new_end, candidate.grid_size.max(1))
            .into_iter()
            .find(|route| {
                !route_collides(
                    original,
                    candidate,
                    old_wire,
                    route,
                    original_terminals,
                    candidate_terminals,
                )
            })
            .ok_or(MoveSelectionError::NoLegalShoveRoute { wire_id })?;

        remap_taps_for_shoved_wire(original, candidate, old_wire, &route)?;
        let new_last_index = route.len() - 1;
        for connection in candidate
            .connections
            .iter_mut()
            .filter(|connection| connection.wire_id == wire_id)
        {
            if connection.point_index == last_index {
                connection.point_index = new_last_index;
            }
        }
        candidate
            .wires
            .iter_mut()
            .find(|wire| wire.id == wire_id)
            .expect("candidate preserves source wire identities")
            .points = route;

        if start_is_stationary {
            restore_shoved_endpoint_junction(
                candidate,
                checked_offset(old_start, delta)?,
                old_start,
            );
        }
        if end_is_stationary {
            restore_shoved_endpoint_junction(candidate, checked_offset(old_end, delta)?, old_end);
        }
    }
    Ok(())
}

fn endpoint_has_stationary_component_connection(
    state: &SchematicState,
    wire_id: u64,
    point_index: usize,
) -> bool {
    state.connections.iter().any(|connection| {
        connection.wire_id == wire_id
            && connection.point_index == point_index
            && !state.selection.has_component(connection.component_id)
    })
}

fn restore_shoved_endpoint_junction(state: &mut SchematicState, moved: Point, anchored: Point) {
    if let Some(junction) = state
        .junctions
        .iter_mut()
        .find(|junction| junction.pos == moved)
    {
        junction.pos = anchored;
    }
}

fn remap_taps_for_shoved_wire(
    original: &SchematicState,
    candidate: &mut SchematicState,
    old_wire: &Wire,
    route: &[Point],
) -> Result<(), MoveSelectionError> {
    let old_start = old_wire.points[0];
    let old_end = *old_wire.points.last().expect("wire has endpoints");
    let new_start = route[0];
    let new_end = *route.last().expect("route has endpoints");
    for old_tap in original.bus_taps.iter().filter(|tap| {
        tap.target_kind() == BusTargetKind::Wire && old_wire.contains_point(tap.connection_point)
    }) {
        if original.selection.has_bus_tap(old_tap.id) {
            continue;
        }
        let candidate_point = candidate
            .bus_taps
            .iter()
            .find(|tap| tap.id == old_tap.id)
            .map(|tap| tap.connection_point)
            .unwrap_or(old_tap.connection_point);
        let mapped = if old_tap.connection_point == old_start {
            Some(new_start)
        } else if old_tap.connection_point == old_end {
            Some(new_end)
        } else if polyline_contains_point(route, candidate_point) {
            Some(candidate_point)
        } else if polyline_contains_point(route, old_tap.connection_point) {
            Some(old_tap.connection_point)
        } else {
            None
        };
        let Some(mapped) = mapped else {
            return Err(MoveSelectionError::AttachedTapCannotBePreserved { tap_id: old_tap.id });
        };
        if let Some(tap) = candidate
            .bus_taps
            .iter_mut()
            .find(|tap| tap.id == old_tap.id)
        {
            tap.connection_point = mapped;
        }
    }
    Ok(())
}

fn orthogonal_route_candidates(start: Point, end: Point, lane_step: i32) -> Vec<Vec<Point>> {
    let mut candidates = Vec::new();
    push_route_candidate(&mut candidates, vec![start, end]);
    push_route_candidate(
        &mut candidates,
        vec![start, Point::new(end.x, start.y), end],
    );
    push_route_candidate(
        &mut candidates,
        vec![start, Point::new(start.x, end.y), end],
    );
    for lane in 1..=MAX_SHOVE_LANES {
        let Some(offset) = lane_step.checked_mul(lane) else {
            break;
        };
        for y in [
            start.y.min(end.y).checked_sub(offset),
            start.y.max(end.y).checked_add(offset),
        ]
        .into_iter()
        .flatten()
        {
            push_route_candidate(
                &mut candidates,
                vec![start, Point::new(start.x, y), Point::new(end.x, y), end],
            );
        }
        for x in [
            start.x.min(end.x).checked_sub(offset),
            start.x.max(end.x).checked_add(offset),
        ]
        .into_iter()
        .flatten()
        {
            push_route_candidate(
                &mut candidates,
                vec![start, Point::new(x, start.y), Point::new(x, end.y), end],
            );
        }
    }
    candidates
}

fn push_route_candidate(candidates: &mut Vec<Vec<Point>>, points: Vec<Point>) {
    let route = simplify_orthogonal_route(points);
    if route.len() >= 2
        && route
            .windows(2)
            .all(|pair| pair[0] != pair[1] && (pair[0].x == pair[1].x || pair[0].y == pair[1].y))
        && !candidates.contains(&route)
    {
        candidates.push(route);
    }
}

fn simplify_orthogonal_route(points: Vec<Point>) -> Vec<Point> {
    let mut deduplicated = Vec::with_capacity(points.len());
    for point in points {
        if deduplicated.last() != Some(&point) {
            deduplicated.push(point);
        }
    }
    if deduplicated.len() <= 2 {
        return deduplicated;
    }
    let mut simplified = vec![deduplicated[0]];
    for index in 1..deduplicated.len() - 1 {
        let previous = *simplified.last().expect("route starts with one point");
        let current = deduplicated[index];
        let next = deduplicated[index + 1];
        if !((previous.x == current.x && current.x == next.x)
            || (previous.y == current.y && current.y == next.y))
        {
            simplified.push(current);
        }
    }
    simplified.push(*deduplicated.last().expect("route is non-empty"));
    simplified
}

fn route_collides(
    original: &SchematicState,
    candidate: &SchematicState,
    old_wire: &Wire,
    route: &[Point],
    original_terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
) -> bool {
    let wire_id = old_wire.id;
    let old_start = old_wire.points[0];
    let old_end = *old_wire.points.last().expect("wire has endpoints");
    let route_start = route[0];
    let route_end = *route.last().expect("route has endpoints");
    let candidate_segments = route
        .windows(2)
        .map(|pair| WireSegment::new(pair[0], pair[1]))
        .collect::<Vec<_>>();
    for (left_index, left) in candidate_segments.iter().enumerate() {
        for right in candidate_segments.iter().skip(left_index + 2) {
            if segments_conflict(*left, *right, route_start, route_end, false, false) {
                return true;
            }
        }
    }
    for wire in candidate.wires.iter().filter(|wire| wire.id != wire_id) {
        let source_wire = original.wires.iter().find(|source| source.id == wire.id);
        let retained_start = source_wire.is_some_and(|source| source.contains_point(old_start))
            && wire.contains_point(route_start);
        let retained_end = source_wire.is_some_and(|source| source.contains_point(old_end))
            && wire.contains_point(route_end);
        for candidate_segment in &candidate_segments {
            for obstacle in wire.segments() {
                if segments_conflict(
                    *candidate_segment,
                    obstacle,
                    route_start,
                    route_end,
                    retained_start,
                    retained_end,
                ) {
                    return true;
                }
            }
        }
    }
    for bus in &candidate.buses {
        let source_bus = original.buses.iter().find(|source| source.id == bus.id);
        let retained_start = source_bus.is_some_and(|source| source.contains_point(old_start))
            && bus.contains_point(route_start);
        let retained_end = source_bus.is_some_and(|source| source.contains_point(old_end))
            && bus.contains_point(route_end);
        for candidate_segment in &candidate_segments {
            for pair in bus.points.windows(2) {
                if segments_conflict(
                    *candidate_segment,
                    WireSegment::new(pair[0], pair[1]),
                    route_start,
                    route_end,
                    retained_start,
                    retained_end,
                ) {
                    return true;
                }
            }
        }
    }
    for component in &candidate.components {
        let terminals = terminal_points_by_component
            .get(&component.id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let source_terminals = original_terminal_points_by_component
            .get(&component.id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let retained_start =
            source_terminals.contains(&old_start) && terminals.contains(&route_start);
        let retained_end = source_terminals.contains(&old_end) && terminals.contains(&route_end);
        for terminal in terminals {
            if candidate_segments
                .iter()
                .any(|segment| segment.contains_point(*terminal))
                && !(*terminal == route_start && retained_start
                    || *terminal == route_end && retained_end)
            {
                return true;
            }
        }
        if candidate_segments
            .iter()
            .any(|segment| segment_enters_component_body(*segment, component, terminals))
        {
            return true;
        }
    }
    false
}

fn segment_enters_component_body(
    segment: WireSegment,
    component: &Component,
    terminals: &[Point],
) -> bool {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = component.bounding_box();
    // Resolved authored symbols may place pins beyond the generic component
    // dimensions available to core state. Their terminal hull is therefore a
    // conservative extension of the no-route body, never a reason to shrink it.
    for terminal in terminals {
        min_x = min_x.min(terminal.x);
        min_y = min_y.min(terminal.y);
        max_x = max_x.max(terminal.x);
        max_y = max_y.max(terminal.y);
    }
    let overlaps_interior = if segment.is_horizontal() {
        segment.start.y > min_y
            && segment.start.y < max_y
            && segment.start.x.min(segment.end.x).max(min_x)
                < segment.start.x.max(segment.end.x).min(max_x)
    } else if segment.is_vertical() {
        segment.start.x > min_x
            && segment.start.x < max_x
            && segment.start.y.min(segment.end.y).max(min_y)
                < segment.start.y.max(segment.end.y).min(max_y)
    } else {
        true
    };
    if !overlaps_interior {
        return false;
    }

    let start_is_outward_terminal = terminals.contains(&segment.start)
        && terminal_access_is_outward(segment.start, segment.end, min_x, min_y, max_x, max_y);
    let end_is_outward_terminal = terminals.contains(&segment.end)
        && terminal_access_is_outward(segment.end, segment.start, min_x, min_y, max_x, max_y);

    !(start_is_outward_terminal || end_is_outward_terminal)
}

fn terminal_access_is_outward(
    terminal: Point,
    other: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    if terminal.y == other.y {
        (terminal.x <= min_x && other.x <= terminal.x)
            || (terminal.x >= max_x && other.x >= terminal.x)
    } else if terminal.x == other.x {
        (terminal.y <= min_y && other.y <= terminal.y)
            || (terminal.y >= max_y && other.y >= terminal.y)
    } else {
        false
    }
}

fn segments_conflict(
    candidate: WireSegment,
    obstacle: WireSegment,
    route_start: Point,
    route_end: Point,
    allow_route_start: bool,
    allow_route_end: bool,
) -> bool {
    if positive_length_overlap(candidate, obstacle) {
        return true;
    }
    candidate.intersection(&obstacle).is_some_and(|point| {
        !(point == route_start && allow_route_start || point == route_end && allow_route_end)
    })
}

fn positive_length_overlap(left: WireSegment, right: WireSegment) -> bool {
    if left.is_horizontal() && right.is_horizontal() && left.start.y == right.start.y {
        let left_min = left.start.x.min(left.end.x);
        let left_max = left.start.x.max(left.end.x);
        let right_min = right.start.x.min(right.end.x);
        let right_max = right.start.x.max(right.end.x);
        return left_min.max(right_min) < left_max.min(right_max);
    }
    if left.is_vertical() && right.is_vertical() && left.start.x == right.start.x {
        let left_min = left.start.y.min(left.end.y);
        let left_max = left.start.y.max(left.end.y);
        let right_min = right.start.y.min(right.end.y);
        let right_max = right.start.y.max(right.end.y);
        return left_min.max(right_min) < left_max.min(right_max);
    }
    false
}

fn reject_new_selected_conductor_overlaps(
    original: &SchematicState,
    candidate: &SchematicState,
) -> Result<(), MoveSelectionError> {
    for moved in candidate
        .wires
        .iter()
        .filter(|wire| candidate.selection.has_wire(wire.id))
    {
        for other in candidate.wires.iter().filter(|wire| wire.id != moved.id) {
            let overlaps = moved.segments().any(|left| {
                other
                    .segments()
                    .any(|right| positive_length_overlap(left, right))
            });
            let existed = original
                .wires
                .iter()
                .find(|wire| wire.id == moved.id)
                .zip(original.wires.iter().find(|wire| wire.id == other.id))
                .is_some_and(|(left_wire, right_wire)| {
                    left_wire.segments().any(|left| {
                        right_wire
                            .segments()
                            .any(|right| positive_length_overlap(left, right))
                    })
                });
            if overlaps && !existed {
                return Err(MoveSelectionError::GeometryOverlap {
                    object_id: moved.id,
                });
            }
        }
        for bus in &candidate.buses {
            if moved.segments().any(|left| {
                bus.points
                    .windows(2)
                    .any(|pair| positive_length_overlap(left, WireSegment::new(pair[0], pair[1])))
            }) {
                return Err(MoveSelectionError::GeometryOverlap {
                    object_id: moved.id,
                });
            }
        }
    }
    Ok(())
}

fn reject_selected_wire_component_body_overlaps(
    candidate: &SchematicState,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
) -> Result<(), MoveSelectionError> {
    for wire in candidate
        .wires
        .iter()
        .filter(|wire| candidate.selection.has_wire(wire.id))
    {
        for component in &candidate.components {
            let terminals = terminal_points_by_component
                .get(&component.id)
                .map(Vec::as_slice)
                .unwrap_or_default();
            if wire
                .segments()
                .any(|segment| segment_enters_component_body(segment, component, terminals))
            {
                return Err(MoveSelectionError::GeometryOverlap { object_id: wire.id });
            }
        }
    }
    Ok(())
}

fn polyline_contains_point(points: &[Point], point: Point) -> bool {
    points
        .windows(2)
        .any(|pair| WireSegment::new(pair[0], pair[1]).contains_point(point))
}

fn commit_movement_candidate(
    state: &mut SchematicState,
    candidate: SchematicState,
    electrical_selection: bool,
) {
    state.components = candidate.components;
    state.wires = candidate.wires;
    state.buses = candidate.buses;
    state.bus_taps = candidate.bus_taps;
    state.net_labels = candidate.net_labels;
    state.junctions = candidate.junctions;
    state.design_notes = candidate.design_notes;
    state.documentation_shapes = candidate.documentation_shapes;
    state.connections = candidate.connections;
    state.is_dirty = true;
    if electrical_selection {
        state.bump_topology_version();
    }
}

fn checked_offset(point: Point, delta: Point) -> Result<Point, MoveSelectionError> {
    Ok(Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(MoveSelectionError::CoordinateOverflow)?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(MoveSelectionError::CoordinateOverflow)?,
    ))
}

fn exact_offset(point: Point, delta: Point) -> Point {
    Point::new(
        point
            .x
            .checked_add(delta.x)
            .expect("guarded move preflighted the x coordinate"),
        point
            .y
            .checked_add(delta.y)
            .expect("guarded move preflighted the y coordinate"),
    )
}


#[cfg(test)]
mod tests;
