//! Toggle chips — compact mono-labeled toggles (corners, temperatures,
//! output flags).

use egui::{Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::theme::{self, mix};
use crate::tokens::Tokens;

/// The width [`chip`] takes for `label`, so a row can make room for one
/// before it draws it.
pub fn chip_width(ui: &Ui, label: &str) -> f32 {
    let t = Tokens::get(ui.ctx());
    let text = ui
        .painter()
        .layout_no_wrap(label.to_owned(), theme::chip_mono(), t.color.text)
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

/// A 22 px toggle chip with a mono label, or as tall as the row's own
/// controls in a row that set a control height shorter than the theme's, so
/// it stands level with the selects beside it. Returns the response; the
/// caller owns the toggle state (`on`).
pub fn chip(ui: &mut Ui, label: &str, on: bool) -> Response {
    let height = row_chip_height(ui);
    chip_sized(ui, label, (on, Lit::Accent), 0.0, height)
}

/// [`chip`] lit without the accent: a neutral fill, the strong border and
/// the full text ink. The toggle for chrome on a pane, where no accent is
/// drawn: a pane header's scale chip, say.
pub fn plain_chip(ui: &mut Ui, label: &str, on: bool) -> Response {
    let height = row_chip_height(ui);
    chip_sized(ui, label, (on, Lit::Plain), 0.0, height)
}

/// How a chip that is on is lit.
#[derive(Clone, Copy)]
enum Lit {
    Accent,
    Plain,
}

/// The height of a [`chip`]: 22 px, or the row's own control height where
/// the row set one shorter, or a touch target.
fn row_chip_height(ui: &Ui) -> f32 {
    let t = Tokens::get(ui.ctx());
    let row = super::control_height(ui);
    if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else if row < t.metrics.ctl_h {
        row
    } else {
        22.0
    }
}

/// [`chip`] at the kit's compact control height, at least `width` wide, its
/// label centred: the chip for a line of compact controls, where it stands
/// as tall as the selects and fields beside it, and where sibling chips
/// stand at one width, the widest one's.
pub fn compact_chip(ui: &mut Ui, label: &str, on: bool, width: f32) -> Response {
    let height = super::compact_height(&Tokens::get(ui.ctx()).metrics);
    chip_sized(ui, label, (on, Lit::Accent), width, height)
}

fn chip_sized(
    ui: &mut Ui,
    label: &str,
    (on, lit): (bool, Lit),
    width: f32,
    height: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;

    let galley = ui.fonts_mut(|f| f.layout_no_wrap(label.to_owned(), theme::chip_mono(), c.text));
    let size = vec2(padded_width(&t, galley.size().x).max(width), height);
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
        match lit {
            Lit::Accent => (c.accent_dim, c.accent, c.text),
            Lit::Plain => (c.bg_active, c.border_strong, c.text),
        }
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
