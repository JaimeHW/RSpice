//! The zoom box: frame the rectangle dragged on the canvas.
//!
//! Its command arms it for one rectangle, as Virtuoso's `z` does. While it is
//! armed the canvas's primary drag draws the box instead of acting through
//! the current tool, and the release frames it.

use egui::{Painter, Pos2, Rect, Response, Stroke, Ui};

use super::parts::CanvasParts;
use super::viewport::Viewport;
use crate::workbench::app_state::Model;

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
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
) {
    let editor = &mut state.model.schematic.session.editor;
    let zoom = f64::from(viewport.zoom);
    if let Some(request) = editor
        .zoom_box
        .drive(ui, response, zoom, |pos| screen_to_world(viewport, pos))
    {
        editor.zoom_request = Some(request);
    }
}

/// Outline the box being dragged.
pub(super) fn paint_zoom_box(painter: &Painter, model: &Model, viewport: &Viewport) {
    let Some((min_x, min_y, max_x, max_y)) = model.schematic.session.editor.zoom_box.world_rect()
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
        Stroke::new(
            1.0,
            crate::ui::tokens::Tokens::get(painter.ctx()).color.accent,
        ),
        egui::StrokeKind::Inside,
    );
}
