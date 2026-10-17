//! The canvas half of a floating Paste or Duplicate: the copy follows the
//! pointer, the arrow keys nudge it, a click or Enter places it, and the
//! armed Move's preview draws what the placement would leave.

use super::super::symbols::SymbolLibrary;
use super::preview::{preview_view, transform_preview_style};
use super::sheet_visibility::with_active_wire_topology_in;
use super::snap_resolution::resolve_grid_pointer;
use super::{SchematicSymbolContext, viewport::Viewport};
use crate::state::Point;
use crate::workbench::app::{cancel_floating_paste, floating_paste_scope, on_grid};
use crate::workbench::app_state::AppState;
use egui::{Key, Modifiers, Painter, PointerButton, Response, Ui};
use rspice_schematic_editor::view::preview;

/// Carry the floating copy with the pointer and place it on a click.
/// Returns whether a copy was floating, in which case it owned the frame's
/// canvas input.
pub(super) fn handle_floating_paste(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) -> bool {
    if state.dialogs.floating_paste.is_none() {
        return false;
    }
    if state.application_modal_open() {
        return true;
    }
    if let Err(reason) = floating_paste_scope(state) {
        cancel_floating_paste(state);
        state.prompt(format!("Paste cancelled: {reason}."));
        return true;
    }
    let grid = state.schematic.document().grid_size;
    let on_canvas = |position| {
        on_grid(
            resolve_grid_pointer(state, viewport, position).snapped_position,
            grid,
        )
    };
    let clicked = response
        .clicked_by(PointerButton::Primary)
        .then(|| response.interact_pointer_pos())
        .flatten()
        .map(on_canvas);
    let hovered = response.hover_pos().map(on_canvas);
    let keyboard = (response.has_focus() || response.contains_pointer())
        .then(|| ui.input_mut(|input| consume_keyboard(input, grid)));
    let Some(floating) = state.dialogs.floating_paste.as_mut() else {
        return true;
    };
    if let Some(pointer) = clicked.or(hovered) {
        floating.follow(pointer);
    }
    let (step, enter) = keyboard.unwrap_or((Point::origin(), false));
    if step != Point::origin() {
        floating.nudge(step);
    }
    if clicked.is_some() || enter {
        response.request_focus();
        state.place_floating_paste();
    }
    true
}

/// The arrow keys' grid step and whether Enter was pressed.
fn consume_keyboard(input: &mut egui::InputState, grid: i32) -> (Point, bool) {
    let mut step = Point::origin();
    for (key, x, y) in [
        (Key::ArrowLeft, -grid, 0),
        (Key::ArrowRight, grid, 0),
        (Key::ArrowUp, 0, -grid),
        (Key::ArrowDown, 0, grid),
    ] {
        if input.consume_key(Modifiers::NONE, key) {
            step = step + Point::new(x, y);
        }
    }
    (step, input.consume_key(Modifiers::NONE, Key::Enter))
}

/// Draw the floating copy where the click would land it, joined to the
/// wiring under it, through the armed Move's candidate painter.
pub(super) fn draw_floating_paste_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    let content_version = state.schematic.content_version();
    let Some(floating) = state.dialogs.floating_paste.as_ref() else {
        return;
    };
    if floating.current_preview(content_version).is_none() {
        let at = floating.at();
        let mut candidate = state.schematic.clone();
        let result = candidate.paste_at_checked(&floating.objects, at);
        if matches!(result, Ok(true)) {
            with_active_wire_topology_in(&state.workspace, &mut candidate, |candidate, scope| {
                candidate.connect_pasted_selection(
                    |component| symbol_context.terminal_points(component),
                    scope,
                );
            });
        }
        let candidate = candidate.document().clone();
        if let Some(floating) = state.dialogs.floating_paste.as_mut() {
            floating.remember_preview(content_version, candidate, result);
        }
    }
    let Some(floating) = state.dialogs.floating_paste.as_ref() else {
        return;
    };
    let Some(preview_state) = floating.current_preview(content_version) else {
        return;
    };
    let at = floating.at();
    let (valid, detail) = match &preview_state.result {
        Ok(true) => (
            true,
            match floating.objects.count() {
                1 => format!("{}, {} \u{b7} 1 object", at.x, at.y),
                count => format!("{}, {} \u{b7} {count} objects", at.x, at.y),
            },
        ),
        Ok(false) => return,
        Err(error) => (false, error.clone()),
    };
    if valid {
        preview::draw_move_candidate(
            painter,
            viewport,
            &preview_view(state),
            &preview_state.candidate,
            &transform_preview_style(state, symbol_context, symbol_library),
            || state.workspace.content.active_view.display_path(),
        );
    }
    preview::draw_transform_feedback(painter, response, valid, detail);
}
