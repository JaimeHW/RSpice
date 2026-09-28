//! Moving a selection.
//!
//! Dragging components and wires while keeping the connections that cross
//! the selection boundary attached — the rubber-band behaviour a schematic
//! editor is judged on. A wire with one end inside the selection stretches;
//! a wire wholly inside travels.

use super::super::*;
use rspice_design::schematic::component_edit::legacy_terminal_points;
use rspice_design::schematic::movement::{self, MovementImpact, MovementSelection};

impl SchematicState {
    fn apply_movement_impact(&mut self, impact: MovementImpact) {
        if impact.changed {
            self.is_dirty = true;
        }
    }

    /// Whether the current selection still resolves to at least one object
    /// supported by selection movement.
    pub fn has_live_movable_selection(&self) -> bool {
        movement::has_live_movable_selection(
            &self.design.document(),
            movement_selection(&self.selection),
        )
    }

    /// Number of selected movable objects that still exist in this document.
    pub fn live_movable_selection_count(&self) -> usize {
        movement::live_movable_selection_count(
            &self.design.document(),
            movement_selection(&self.selection),
        )
    }

    /// Move a component and update attached wire endpoints using caller-supplied terminal geometry.
    pub fn move_component_with_wires_resolved(
        &mut self,
        component_id: u64,
        delta: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        if self.read_only {
            return;
        }
        let impact = self.design.move_component_with_wires_resolved(
            component_id,
            delta,
            terminal_points_for,
        );
        self.apply_movement_impact(impact);
    }

    /// Move selected components, labels, buses, taps, and wires while
    /// rubber-banding connected wires.
    ///
    /// This is the multi-component version of move_component_with_wires_resolved.
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
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        if self.read_only {
            return;
        }
        let impact = self.design.move_selection_with_rubber_band_resolved(
            movement_selection(&self.selection),
            delta,
            terminal_points_for,
        );
        self.apply_movement_impact(impact);
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
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> Result<bool, MoveSelectionError> {
        if self.read_only {
            return Ok(false);
        }
        let impact = self.design.move_selection_with_mode_resolved(
            movement_selection(&self.selection),
            delta,
            mode,
            terminal_points_for,
        )?;
        self.apply_movement_impact(impact);
        Ok(impact.changed)
    }

    /// Move all points of a wire by a delta
    pub fn move_wire(&mut self, wire_id: u64, delta: Point) {
        if self.read_only {
            return;
        }
        let impact = self.design.move_wire(wire_id, delta);
        self.apply_movement_impact(impact);
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
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) {
        if self.read_only {
            return;
        }
        let impact = self.design.move_selection_resolved(
            movement_selection(&self.selection),
            delta,
            terminal_points_for,
        );
        self.apply_movement_impact(impact);
    }

    /// Move all wire points at a junction to a new position
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) {
        if self.read_only {
            return;
        }
        let impact = self.design.move_junction(old_pos, new_pos);
        self.apply_movement_impact(impact);
    }
}

fn movement_selection(selection: &Selection) -> MovementSelection<'_> {
    MovementSelection {
        components: &selection.components,
        wires: &selection.wires,
        buses: &selection.buses,
        bus_taps: &selection.bus_taps,
        net_labels: &selection.net_labels,
        design_notes: &selection.design_notes,
        documentation_shapes: &selection.documentation_shapes,
        probes: &selection.probes,
    }
}

#[cfg(test)]
mod tests;
