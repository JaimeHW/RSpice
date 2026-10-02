//! Painting for host-resolved operating-point annotations.

use super::super::viewport::Viewport;
use egui::{Painter, Rect, Stroke};
use rspice_design_model::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPointCanvasAnnotation {
    pub position: Point,
    pub label: String,
    pub selected_current: bool,
}

pub(super) fn draw_operating_point_annotations(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    annotations: impl IntoIterator<Item = OperatingPointCanvasAnnotation>,
) {
    use rspice_ui_kit::theme::{self, FontWeight};

    let palette = rspice_ui_kit::tokens::active_palette();
    for annotation in annotations {
        let anchor = viewport.schematic_to_screen(annotation.position);
        if !available.expand(12.0).contains(anchor) {
            continue;
        }
        let galley = painter.layout_no_wrap(
            annotation.label,
            theme::mono(rspice_ui_kit::tokens::FS_0, FontWeight::Medium),
            if annotation.selected_current {
                palette.info
            } else {
                palette.net_label
            },
        );
        let offset = if annotation.selected_current {
            egui::vec2(8.0, 12.0)
        } else {
            egui::vec2(7.0, -galley.size().y - 7.0)
        };
        let mut text_pos = anchor + offset;
        text_pos.x = text_pos.x.clamp(
            available.left() + 3.0,
            available.right() - galley.size().x - 3.0,
        );
        text_pos.y = text_pos.y.clamp(
            available.top() + 3.0,
            available.bottom() - galley.size().y - 3.0,
        );
        let background = Rect::from_min_size(text_pos, galley.size()).expand2(egui::vec2(4.0, 2.0));
        painter.rect_filled(background, 2.0, palette.bg_elevated);
        painter.rect_stroke(
            background,
            2.0,
            Stroke::new(1.0, palette.border),
            egui::StrokeKind::Inside,
        );
        painter.galley(text_pos, galley, palette.text);
    }
}
