//! The zoom box: frame the rectangle dragged on the canvas.
//!
//! Its command arms it for one rectangle, as Virtuoso's `z` does. While it is
//! armed the canvas's primary drag draws the box instead of acting through
//! the current tool, and the release frames it.

use egui::{Painter, PointerButton, Pos2, Rect, Response, Stroke, Ui};
use rspice_schematic_editor::view::navigation::ZoomRequest;

use super::viewport::Viewport;
use crate::workbench::app_state::AppState;

/// A box narrower or shorter than this on screen is a slip of the pointer,
/// not a rectangle to frame.
const MIN_BOX_PIXELS: f64 = 4.0;

fn screen_to_world(viewport: &Viewport, pos: Pos2) -> (f64, f64) {
    let zoom = f64::from(viewport.zoom).max(f64::EPSILON);
    (
        (f64::from(pos.x) - f64::from(viewport.bounds.min.x) - f64::from(viewport.offset.x)) / zoom,
        (f64::from(pos.y) - f64::from(viewport.bounds.min.y) - f64::from(viewport.offset.y)) / zoom,
    )
}

/// Drive the armed zoom box with this frame's pointer. A drag frames its
/// rectangle on release; a click, or a box too small to frame, lets go and
/// leaves the view as it was.
pub(super) fn handle_zoom_box(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) {
    let zoom_box = &mut state.schematic.session.editor.zoom_box;
    if response.drag_started_by(PointerButton::Primary)
        && let Some(origin) = ui.input(|input| input.pointer.press_origin())
    {
        let anchor = screen_to_world(viewport, origin);
        zoom_box.drag = Some((anchor, anchor));
    }
    if (response.dragged_by(PointerButton::Primary)
        || response.drag_stopped_by(PointerButton::Primary))
        && let Some(pointer) = response.interact_pointer_pos()
        && let Some((anchor, _)) = zoom_box.drag
    {
        zoom_box.drag = Some((anchor, screen_to_world(viewport, pointer)));
    }
    if response.drag_stopped_by(PointerButton::Primary) {
        let zoom = f64::from(viewport.zoom);
        let framed = zoom_box
            .world_rect()
            .filter(|(min_x, min_y, max_x, max_y)| {
                (max_x - min_x) * zoom >= MIN_BOX_PIXELS && (max_y - min_y) * zoom >= MIN_BOX_PIXELS
            });
        zoom_box.disarm();
        if let Some(rect) = framed {
            state.schematic.session.editor.zoom_request = Some(ZoomRequest::Frame(rect));
        }
    } else if response.clicked_by(PointerButton::Primary) {
        zoom_box.disarm();
    }
}

/// Outline the box being dragged.
pub(super) fn paint_zoom_box(painter: &Painter, state: &AppState, viewport: &Viewport) {
    let Some((min_x, min_y, max_x, max_y)) = state.schematic.session.editor.zoom_box.world_rect()
    else {
        return;
    };
    let to_screen = |x: f64, y: f64| {
        Pos2::new(
            viewport.bounds.min.x + viewport.offset.x + x as f32 * viewport.zoom,
            viewport.bounds.min.y + viewport.offset.y + y as f32 * viewport.zoom,
        )
    };
    painter.rect_stroke(
        Rect::from_min_max(to_screen(min_x, min_y), to_screen(max_x, max_y)),
        0.0,
        Stroke::new(1.0, crate::ui::tokens::active_palette().accent),
        egui::StrokeKind::Inside,
    );
}
