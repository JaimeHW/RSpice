//! Toggle chips — compact mono-labeled toggles (corners, temperatures,
//! output flags).

use egui::{Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::theme::{self, FontWeight, mix};
use crate::tokens::{self, Tokens};

/// The width [`chip`] takes for `label`, so a row can make room for one
/// before it draws it.
pub fn chip_width(ui: &Ui, label: &str) -> f32 {
    let t = Tokens::get(ui.ctx());
    let text = ui
        .painter()
        .layout_no_wrap(
            label.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text,
        )
        .size()
        .x;
    padded_width(&t, text)
}

fn padded_width(t: &Tokens, text: f32) -> f32 {
    let floor = if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else {
        0.0
    };
    (text + 18.0).max(floor)
}

/// A 22 px toggle chip with a mono label. Returns the response; the caller
/// owns the toggle state (`on`).
pub fn chip(ui: &mut Ui, label: &str, on: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;

    let galley = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            label.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text,
        )
    });
    let size = vec2(
        padded_width(&t, galley.size().x),
        if t.metrics.is_touch() {
            t.metrics.touch_target()
        } else {
            22.0
        },
    );
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::SelectableLabel, ui.is_enabled(), on, label)
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);

    let (fill, border, text_color) = if on {
        (c.accent_dim, c.accent, c.text)
    } else {
        (
            egui::Color32::TRANSPARENT,
            mix(c.border, c.border_strong, hover),
            mix(c.text_dim, c.text, hover),
        )
    };

    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        fill,
        Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
    painter.galley(
        egui::pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        text_color,
    );

    theme::paint_focus_ring(ui, &response, rect);

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
