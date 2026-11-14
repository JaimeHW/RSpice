//! The size of the text in the source editors. The preference holds it, every
//! editor reads it here, and an editor's zoom keys ask for a step of it.

use egui::{Context, Id};

/// The size a row and a gutter are proportioned against.
const BASE_SIZE: f32 = 11.0;
const BASE_LINE_HEIGHT: f32 = 17.05;
const BASE_GUTTER_WIDTH: f32 = 47.0;

fn size_id() -> Id {
    Id::new("rspice.editor-font.size")
}

fn zoom_id() -> Id {
    Id::new("rspice.editor-font.zoom")
}

/// Set the size the editors draw at from this frame on.
pub fn publish(ctx: &Context, size: f32) {
    ctx.data_mut(|data| data.insert_temp(size_id(), size));
}

/// The size the editors draw their text at.
pub(crate) fn text_size(ctx: &Context) -> f32 {
    ctx.data(|data| data.get_temp(size_id()))
        .unwrap_or(BASE_SIZE)
}

/// The row pitch of text at `size`.
pub(crate) fn line_height(size: f32) -> f32 {
    size * BASE_LINE_HEIGHT / BASE_SIZE
}

/// The width of the line-number gutter beside text at `size`.
pub(crate) fn gutter_width(size: f32) -> f32 {
    size * BASE_GUTTER_WIDTH / BASE_SIZE
}

/// Ask for the text to be `step` sizes larger or smaller.
pub(crate) fn request_zoom(ctx: &Context, step: i8) {
    ctx.data_mut(|data| {
        let held = data.get_temp_mut_or_default::<i8>(zoom_id());
        *held = held.saturating_add(step);
    });
    ctx.request_repaint();
}

/// The step asked for since the last frame, once.
pub fn take_zoom_request(ctx: &Context) -> Option<i8> {
    ctx.data_mut(|data| data.remove_temp::<i8>(zoom_id()))
        .filter(|step| *step != 0)
}
