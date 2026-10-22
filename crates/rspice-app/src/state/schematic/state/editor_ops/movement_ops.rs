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
            self.session.is_dirty = true;
        }
    }

    /// Whether the current selection still resolves to at least one object
    /// supported by selection movement.
    pub fn has_live_movable_selection(&self) -> bool {
        movement::has_live_movable_selection(
            self.design.document(),
            movement_selection(&self.session.editor.selection),
        )
    }

    /// Number of selected movable objects that still exist in this document.
    pub fn live_movable_selection_count(&self) -> usize {
        movement::live_movable_selection_count(
            self.design.document(),
            movement_selection(&self.session.editor.selection),
        )
    }

    /// Move a component and update attached wire endpoints using caller-supplied terminal geometry.
    pub fn move_component_with_wires_resolved(
        &mut self,
        component_id: u64,
        delta: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        _scope: &ActiveSheetTopology,
    ) {
        if self.session.read_only {
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
    /// rubber-banding connected wires, using caller-supplied terminal geometry.
    ///
    /// Wires connected to selected components are stretched to maintain the
    /// connection, square unless `rubber_band` asks for angles. Wires that
    /// connect two selected components are moved entirely (not stretched).
    /// Runs on every drag frame, laying the wires from where the pending
    /// gesture began.
    pub fn move_selection_with_rubber_band_resolved(
        &mut self,
        delta: Point,
        rubber_band: DragRubberBand,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        _scope: &ActiveSheetTopology,
    ) {
        if self.session.read_only {
            return;
        }
        let impact = self.design.drag_selection_with_rubber_band_resolved(
            movement_selection(&self.session.editor.selection),
            delta,
            rubber_band,
            terminal_points_for,
        );
        self.apply_movement_impact(impact);
    }

    /// Move the current selection under an explicit connectivity policy,
    /// using caller-resolved terminal geometry.
    ///
    /// `Ok(false)` is a clean no-op (read-only document, zero delta, or no
    /// live movable selection). Guarded modes build and validate a candidate
    /// geometry first, so every `Err` leaves the document bit-for-bit
    /// unchanged.
    pub fn move_selection_with_mode_resolved(
        &mut self,
        delta: Point,
        mode: MoveSelectionMode,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        _scope: &ActiveSheetTopology,
    ) -> Result<bool, MoveSelectionError> {
        if self.session.read_only {
            return Ok(false);
        }
        let impact = self.design.move_selection_with_mode_resolved(
            movement_selection(&self.session.editor.selection),
            delta,
            mode,
            terminal_points_for,
        )?;
        self.apply_movement_impact(impact);
        Ok(impact.changed)
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
        if self.session.read_only {
            return;
        }
        let impact = self.design.move_selection_resolved(
            movement_selection(&self.session.editor.selection),
            delta,
            terminal_points_for,
        );
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
