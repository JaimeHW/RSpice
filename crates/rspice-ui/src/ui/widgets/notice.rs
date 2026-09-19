//! The grammar one notice is drawn in, wherever it is drawn.
//!
//! A toast is a retained record arriving, and the notification panel is the
//! same record looked up an hour later. Both are a severity glyph, a title, one
//! clause of detail and at most one offer, so the pieces are painted once here
//! and a notice reads the same in either place.
//!
//! Severity is carried by the glyph's *shape* as well as its colour — a ring
//! with a tick, a ring with an `i`, a triangle, an octagon — so it survives a
//! colour-blind reader and a monochrome capture.

use egui::{Color32, Painter, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::toast::ToastKind;

/// Side of the severity glyph, in both the toast and the panel row.
pub(crate) const GLYPH_SIDE: f32 = 16.0;
/// Side of the dismiss mark's own target on a pointer device.
pub(crate) const DISMISS_SIDE: f32 = 20.0;
/// Height of the offer link on a pointer device.
pub(crate) const OFFER_HEIGHT: f32 = 22.0;
const OFFER_ARROW_SIDE: f32 = 12.0;
const OFFER_ARROW_GAP: f32 = 4.0;

/// The colour a severity is drawn in.
pub(crate) fn tone_color(tokens: &Tokens, kind: ToastKind) -> Color32 {
    match kind {
        ToastKind::Success => tokens.color.ok,
        ToastKind::Info => tokens.color.info,
        ToastKind::Warn => tokens.color.warn,
        ToastKind::Error => tokens.color.err,
    }
}

/// Paint the severity glyph into `rect`. Authored on a 24-unit grid.
pub(crate) fn paint_tone_glyph(painter: &Painter, rect: Rect, kind: ToastKind, color: Color32) {
    let side = rect.width().min(rect.height());
    let scale = side / 24.0;
    let origin = rect.center() - Vec2::splat(side * 0.5);
    let point = |x: f32, y: f32| pos2(origin.x + x * scale, origin.y + y * scale);
    let stroke = Stroke::new((1.7 * scale).max(1.2), color);
    let line = |points: &[(f32, f32)]| {
        painter.add(Shape::line(
            points.iter().map(|&(x, y)| point(x, y)).collect(),
            stroke,
        ));
    };
    let outline = |points: &[(f32, f32)]| {
        painter.add(Shape::closed_line(
            points.iter().map(|&(x, y)| point(x, y)).collect(),
            stroke,
        ));
    };
    let dot = |x: f32, y: f32| painter.circle_filled(point(x, y), stroke.width * 0.62, color);

    match kind {
        ToastKind::Success => {
            painter.circle_stroke(point(12.0, 12.0), 9.0 * scale, stroke);
            line(&[(8.0, 12.4), (10.8, 15.2), (16.0, 9.6)]);
        }
        ToastKind::Info => {
            painter.circle_stroke(point(12.0, 12.0), 9.0 * scale, stroke);
            line(&[(12.0, 11.0), (12.0, 16.5)]);
            dot(12.0, 7.6);
        }
        ToastKind::Warn => {
            outline(&[(12.0, 3.5), (21.2, 19.5), (2.8, 19.5)]);
            line(&[(12.0, 9.5), (12.0, 14.0)]);
            dot(12.0, 16.8);
        }
        ToastKind::Error => {
            outline(&[
                (8.4, 3.0),
                (15.6, 3.0),
                (21.0, 8.4),
                (21.0, 15.6),
                (15.6, 21.0),
                (8.4, 21.0),
                (3.0, 15.6),
                (3.0, 8.4),
            ]);
            line(&[(12.0, 7.8), (12.0, 12.8)]);
            dot(12.0, 16.2);
        }
    }
}

/// The one positive action a notice carries: accent text and an arrow.
///
/// A link rather than a button, because a framed button in every row turns a
/// list of events into a list of forms. The arrow says the offer *leaves* —
/// taking it navigates away and closes whatever drew it.
///
/// `subject` is the notice the offer belongs to. A list of notices is a list
/// of identical links to a screen reader unless each one says whose it is.
pub(crate) fn offer_link(ui: &mut Ui, label: &str, subject: &str, large_target: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, t.color.accent);
    let height = if large_target {
        tokens::TOUCH_TARGET
    } else {
        OFFER_HEIGHT
    };
    let width = galley.size().x + OFFER_ARROW_GAP + OFFER_ARROW_SIDE + 2.0;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Link,
            ui.is_enabled(),
            format!("{label}: {subject}"),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let lit = response.hovered() || response.has_focus();
    let text_pos = pos2(rect.left(), rect.center().y - galley.size().y * 0.5);
    let text_right = text_pos.x + galley.size().x;
    let baseline = text_pos.y + galley.size().y;
    ui.painter().galley(text_pos, galley, t.color.accent);
    if lit {
        ui.painter().hline(
            text_pos.x..=text_right,
            baseline + 1.0,
            Stroke::new(1.0, t.color.accent),
        );
    }
    // The arrow steps two points toward where it leads while the pointer is
    // on the link; with animation disabled it simply sits there.
    let shift = ui
        .ctx()
        .animate_bool_with_time(response.id.with("offer-arrow"), lit, 0.12)
        * 2.0;
    let arrow = Rect::from_center_size(
        pos2(
            text_right + OFFER_ARROW_GAP + OFFER_ARROW_SIDE * 0.5 + shift,
            rect.center().y + 0.5,
        ),
        Vec2::splat(OFFER_ARROW_SIDE),
    );
    paint_offer_arrow(ui.painter(), arrow, t.color.accent);
    theme::paint_focus_ring(ui, &response, rect.expand2(vec2(3.0, 0.0)));
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn paint_offer_arrow(painter: &Painter, rect: Rect, color: Color32) {
    let scale = rect.width() / 24.0;
    let point = |x: f32, y: f32| pos2(rect.left() + x * scale, rect.top() + y * scale);
    let stroke = Stroke::new(1.3, color);
    painter.line_segment([point(5.0, 12.0), point(18.0, 12.0)], stroke);
    painter.add(Shape::line(
        vec![point(13.0, 6.5), point(18.5, 12.0), point(13.0, 17.5)],
        stroke,
    ));
}

/// The dismiss mark: a small cross that lights under the pointer.
///
/// `label` names what is dismissed, because a screen reader meets a column of
/// these and "Dismiss" alone would say the same thing on every row.
pub(crate) fn dismiss_button(ui: &Ui, rect: Rect, id: egui::Id, label: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let response = ui.interact(rect, id, Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    let lit = response.hovered() || response.has_focus();
    let mark = Rect::from_center_size(rect.center(), Vec2::splat(DISMISS_SIDE));
    if lit {
        ui.painter().rect_filled(mark, t.radius, t.color.bg_hover);
    }
    let cross = Rect::from_center_size(rect.center(), Vec2::splat(8.0));
    let stroke = Stroke::new(
        1.3,
        if lit {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    ui.painter()
        .line_segment([cross.left_top(), cross.right_bottom()], stroke);
    ui.painter()
        .line_segment([cross.right_top(), cross.left_bottom()], stroke);
    theme::paint_focus_ring(ui, &response, mark);
    response.on_hover_text(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every severity paints, at the size both surfaces use, without leaving
    /// its box: a glyph that bleeds is a glyph that overlaps the title.
    #[test]
    fn every_severity_glyph_stays_inside_its_box() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let rect = Rect::from_min_size(pos2(20.0, 20.0), Vec2::splat(GLYPH_SIDE));
        for kind in [
            ToastKind::Success,
            ToastKind::Info,
            ToastKind::Warn,
            ToastKind::Error,
        ] {
            // A foreground layer of its own, so the pass's shapes are the
            // glyph's and nothing a panel painted behind it.
            let layer = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("glyph-probe"));
            let output = ctx.run_ui(egui::RawInput::default(), |ui| {
                let painter = ui.ctx().layer_painter(layer);
                paint_tone_glyph(&painter, rect, kind, Color32::WHITE);
            });
            let bounds = output
                .shapes
                .iter()
                .map(|clipped| clipped.shape.visual_bounding_rect())
                .filter(|bounds| bounds.is_positive())
                .fold(Rect::NOTHING, Rect::union);
            assert!(bounds.is_positive(), "{kind:?} painted nothing");
            assert!(
                rect.expand(1.0).contains_rect(bounds),
                "{kind:?} glyph {bounds:?} leaves its {rect:?} box"
            );
        }
    }

    #[test]
    fn severities_are_told_apart_by_colour_as_well_as_shape() {
        let tokens = Tokens::default();
        let colours = [
            tone_color(&tokens, ToastKind::Success),
            tone_color(&tokens, ToastKind::Info),
            tone_color(&tokens, ToastKind::Warn),
            tone_color(&tokens, ToastKind::Error),
        ];
        for (index, colour) in colours.iter().enumerate() {
            assert!(
                !colours[index + 1..].contains(colour),
                "two severities share {colour:?}"
            );
        }
    }
}
