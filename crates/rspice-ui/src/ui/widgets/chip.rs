//! Toggle chips — compact mono-labeled toggles (corners, temperatures,
//! output flags).

use egui::{Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

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
    let (rect, response) =
        ui.allocate_exact_size(vec2(galley.size().x + 18.0, 22.0), Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::SelectableLabel, ui.is_enabled(), on, label)
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.16);

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
