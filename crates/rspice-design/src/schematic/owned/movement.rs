//! Geometry mutations update the live owner's topology revision.
use super::super::component::Component;
use super::super::movement::{
    self, MoveSelectionError, MoveSelectionMode, MovementImpact, MovementSelection,
};
use super::super::stretch::{self, StretchOrthogonalPolicy, StretchSelectionError, StretchTarget};
use super::Schematic;
use rspice_design_model::Point;
impl Schematic {
    pub fn move_component_with_wires_resolved(
        &mut self,
        component_id: u64,
        delta: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> MovementImpact {
        let impact = movement::move_component_with_wires_resolved(
            &mut self.document,
            component_id,
            delta,
            terminal_points_for,
        );
        if impact.topology_changed {
            self.invalidate_topology();
        }
        impact
    }
    pub fn move_selection_with_rubber_band_resolved(
        &mut self,
        selection: MovementSelection<'_>,
        delta: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> MovementImpact {
        let impact = movement::move_selection_with_rubber_band_resolved(
            &mut self.document,
            selection,
            delta,
            terminal_points_for,
        );
        if impact.topology_changed {
            self.invalidate_topology();
        }
        impact
    }
    pub fn move_selection_with_mode_resolved(
        &mut self,
        selection: MovementSelection<'_>,
        delta: Point,
        mode: MoveSelectionMode,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> Result<MovementImpact, MoveSelectionError> {
        let impact = movement::move_selection_with_mode_resolved(
            &mut self.document,
            selection,
            delta,
            mode,
            terminal_points_for,
        )?;
        if impact.topology_changed {
            self.invalidate_topology();
        }
        Ok(impact)
    }
    pub fn move_wire(&mut self, wire_id: u64, delta: Point) -> MovementImpact {
        let impact = movement::move_wire(&mut self.document, wire_id, delta);
        if impact.topology_changed {
            self.invalidate_topology();
        }
        impact
    }
    pub fn move_selection_resolved(
        &mut self,
        selection: MovementSelection<'_>,
        delta: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    ) -> MovementImpact {
        let impact = movement::move_selection_resolved(
            &mut self.document,
            selection,
            delta,
            terminal_points_for,
        );
        if impact.topology_changed {
            self.invalidate_topology();
        }
        impact
    }
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) -> MovementImpact {
        let impact = movement::move_junction(&mut self.document, old_pos, new_pos);
        if impact.topology_changed {
            self.invalidate_topology();
        }
        impact
    }
    pub fn stretch_target_resolved(
        &mut self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<bool, StretchSelectionError> {
        let changed = stretch::stretch_target_resolved(
            &mut self.document,
            delta,
            target,
            policy,
            terminal_points_for,
            component_bounds_for,
        )?;
        if changed
            && matches!(
                target,
                StretchTarget::WireSegment { .. } | StretchTarget::BusSegment { .. }
            )
        {
            self.invalidate_topology();
        }
        Ok(changed)
    }
}
