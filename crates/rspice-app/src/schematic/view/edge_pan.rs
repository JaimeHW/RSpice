//! The view pans by itself while a gesture carries something to the canvas
//! edge: a move, a stretch, a wire or bus route, a held drag or a floating
//! paste. Every reference editor does this; without it a part cannot be
//! carried past what is on screen.

use super::AppState;
use egui::{Popup, Rect, Response, Ui};
use rspice_schematic_editor::view::navigation::apply_edge_pan;

/// Pan toward the edge the pointer rests near, if a gesture is carrying
/// something, and keep frames coming while it does.
pub(super) fn edge_pan(ui: &Ui, response: &Response, canvas: Rect, state: &mut AppState) {
    if !state.ui.preferences.canvas_navigation().edge_pan
        || !carrying(state)
        || state.application_modal_open()
        || Popup::is_any_open(ui.ctx())
    {
        return;
    }
    let Some(pointer) = ui.input(|input| input.pointer.latest_pos()) else {
        return;
    };
    let held = response.is_pointer_button_down_on();
    let dt = ui.input(|input| input.stable_dt);
    if apply_edge_pan(
        &mut state.schematic.session.editor.pan,
        canvas,
        pointer,
        dt,
        held,
    ) {
        ui.ctx().request_repaint();
    }
}

/// Whether a gesture is carrying something across the canvas.
fn carrying(state: &AppState) -> bool {
    let editor = &state.schematic.session.editor;
    let move_draft = &state.dialogs.move_selection;
    let stretch_draft = &state.dialogs.stretch_selection;
    state.dialogs.floating_paste.is_some()
        || state.schematic_drag_in_progress()
        || editor.wire_drawing.active
        || editor.bus_drawing.active
        || (move_draft.armed && move_draft.canvas.anchor.is_some())
        || (stretch_draft.armed && stretch_draft.canvas.gesture.anchor.is_some())
}
