//! The canvas's gesture commits, for tests outside the canvas that drive an
//! edit the way the pointer does.
//!
//! Each function is the call the canvas makes once a gesture lands: a click
//! with a placement or junction tool armed, Enter on an armed Move or
//! Stretch, the frames of a drag on a selected part. A test that calls these
//! runs the edit the user's gesture runs, without painting a canvas.

use super::{
    AppState, interaction, move_interaction, schematic_symbol_context, selection_drag,
    stretch_interaction,
};
use crate::state::{ComponentType, Point};
use rspice_schematic_editor::view::pointer_target::PointerTarget;
use rspice_schematic_editor::view::selection_drag::SelectionDragAction;

/// A click with the placement tool for `component_type` armed, landing at
/// `at` on the grid.
pub(crate) fn place_component(state: &mut AppState, component_type: ComponentType, at: Point) {
    interaction::place_component(state, component_type, at);
}

/// A click with the junction tool at `at`: it adds a dot where two
/// conductors meet, or takes away the one already there.
pub(crate) fn click_junction(state: &mut AppState, at: Point) {
    interaction::handle_junction_click(state, at);
}

/// Enter on an armed Move whose preview stands at its current delta.
pub(crate) fn commit_move(state: &mut AppState) {
    let symbols = schematic_symbol_context(state);
    move_interaction::commit_armed_move_selection(state, &symbols);
}

/// Enter on an armed Stretch whose preview stands at its current delta.
pub(crate) fn commit_stretch(state: &mut AppState) {
    let symbols = schematic_symbol_context(state);
    stretch_interaction::commit_armed_stretch_selection(state, &symbols);
}

/// Press on `target` at `from`, drag through each of `steps` (offsets from
/// `from`, one frame each) and release: one whole drag of the selection.
pub(crate) fn drag_selection(
    state: &mut AppState,
    ctx: &egui::Context,
    target: PointerTarget,
    from: Point,
    steps: &[Point],
) {
    let symbols = schematic_symbol_context(state);
    let begin = SelectionDragAction::BeginSelection {
        target,
        position: from,
    };
    selection_drag::dispatch(state, ctx, &symbols, begin);
    let Some(operation_id) = state.schematic.pending_operation_id() else {
        return;
    };
    let mut last = from;
    for step in steps {
        let position = Point::new(from.x + step.x, from.y + step.y);
        let movement = SelectionDragAction::MoveSelection {
            operation_id,
            from: last,
            delta: Point::new(position.x - last.x, position.y - last.y),
            position,
        };
        selection_drag::dispatch(state, ctx, &symbols, movement);
        last = position;
    }
    let finish = SelectionDragAction::Finish { operation_id };
    selection_drag::dispatch(state, ctx, &symbols, finish);
}
