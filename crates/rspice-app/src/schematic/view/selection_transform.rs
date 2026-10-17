//! Turn or flip the selection on the active sheet.
//!
//! The Rotate and Mirror commands, the canvas context menu and the R and M
//! keys during a move share this one route, so the pivot, the sheet scope and
//! the undo step cannot differ by how the edit was asked for.

use super::SchematicSymbolContext;
use super::sheet_visibility::{retain_selection_on_active_sheet, with_active_wire_topology};
use crate::state::Point;
use crate::workbench::app_state::AppState;
pub(crate) use rspice_design::schematic::component_edit::SelectionTransform;

/// The middle of `bounds`, snapped to the nearest point of a `grid` pitch.
fn snapped_middle((min_x, min_y, max_x, max_y): (i32, i32, i32, i32), grid: i32) -> Point {
    let snap = |value: f64| {
        let grid = f64::from(grid.max(1));
        ((value / grid).round() * grid) as i32
    };
    Point::new(
        snap((f64::from(min_x) + f64::from(max_x)) / 2.0),
        snap((f64::from(min_y) + f64::from(max_y)) / 2.0),
    )
}

/// Turn or flip everything selected on the active sheet as one body about
/// the grid-snapped middle of its bounds, in one undo step. Returns whether
/// anything moved.
pub(crate) fn transform_selection_on_active_sheet(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    transform: SelectionTransform,
) -> bool {
    if state.schematic_edit_read_only() {
        return false;
    }
    retain_selection_on_active_sheet(state);
    let Some(bounds) = symbol_context.selection_bounds(
        state.schematic.document(),
        &state.schematic.session.editor.selection,
    ) else {
        return false;
    };
    let pivot = snapped_middle(bounds, state.schematic.document().grid_size);
    // The undo step opens with every sheet present; the other sheets step
    // out of reach only while the wires are rerouted and tidied.
    state.schematic.begin_operation(transform.description());
    with_active_wire_topology(state, |schematic, scope| {
        schematic.transform_selection(
            transform,
            pivot,
            |component| symbol_context.terminal_points(component),
            scope,
        )
    });
    let recorded = state.schematic.end_operation();
    if recorded {
        state.sync_active_schematic_to_workspace();
    }
    recorded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pivot_is_the_middle_of_the_bounds_on_the_grid() {
        assert_eq!(snapped_middle((0, 0, 30, 20), 10), Point::new(20, 10));
        assert_eq!(snapped_middle((-40, -10, 0, 10), 10), Point::new(-20, 0));
    }

    /// Sheet 2 shares Sheet 1's coordinates: one of its wires ends on the
    /// pin of Sheet 1's resistor, and another runs through where the turned
    /// pins land. Neither follows, splits or gains a dot, and one undo puts
    /// the resistor back with Sheet 2 still whole.
    #[test]
    fn turning_a_part_leaves_another_sheets_wires_whole() {
        use crate::state::{Component, ComponentType, Rotation, Wire};

        let mut state = super::super::sheet_visibility::two_sheet_state(&[10], &[20, 21]);
        let part = Component::new(10, ComponentType::Resistor, Point::new(100, 100));
        let symbols = super::super::schematic_symbol_context(&state);
        let pin = symbols.terminal_points(&part)[0];
        let ending_on_the_pin = Wire::segment(20, Point::new(pin.x - 200, pin.y), pin);
        let through_the_middle = Wire::segment(21, Point::new(100, -1000), Point::new(100, 1000));
        let document = state.schematic.document_mut_for_test();
        document.components.push(part);
        document.wires = vec![ending_on_the_pin.clone(), through_the_middle.clone()];
        state.schematic.init_undo_history();
        state
            .schematic
            .session
            .editor
            .selection
            .select_component(10);

        let symbols = super::super::schematic_symbol_context(&state);
        assert!(transform_selection_on_active_sheet(
            &mut state,
            &symbols,
            SelectionTransform::RotateClockwise,
        ));

        let document = state.schematic.document();
        assert_eq!(document.components[0].rotation, Rotation::R90);
        assert_eq!(
            document.wires,
            vec![ending_on_the_pin.clone(), through_the_middle.clone()]
        );
        assert!(document.junctions.is_empty());

        assert!(state.schematic.undo());
        let document = state.schematic.document();
        assert_eq!(document.components[0].rotation, Rotation::R0);
        assert_eq!(document.wires, vec![ending_on_the_pin, through_the_middle]);
    }
}
