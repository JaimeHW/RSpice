//! Measurement tables — dimmed metric names left, mono values right, with
//! hairline row separators.

use egui::{Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Render a measurement table across the available width.
pub fn measurement_table(ui: &mut Ui, rows: &[(&str, &str)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();

    for (i, (name, value)) in rows.iter().enumerate() {
        let inner_width = (width - 24.0).max(1.0);
        let gap = 8.0_f32.min(inner_width);
        let columns_width = (inner_width - gap).max(1.0);
        let name_width = columns_width * 0.42;
        let value_width = (columns_width - name_width).max(1.0);
        let name_galley = ui.painter().layout(
            (*name).to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text_dim,
            name_width,
        );
        let value_galley = ui.painter().layout(
            (*value).to_owned(),
            theme::mono(tokens::FS_1, FontWeight::Regular),
            c.text,
            value_width,
        );
        let row_h = (name_galley.size().y.max(value_galley.size().y) + 8.0).max(25.0);
        let (rect, response) = ui.allocate_exact_size(vec2(width, row_h), egui::Sense::hover());
        response.widget_info(|| {
            WidgetInfo::labeled(
                WidgetType::Label,
                ui.is_enabled(),
                format!("{name}: {value}"),
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Row);
        });
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter();
        let name_rect = egui::Rect::from_min_max(
            egui::pos2(rect.left() + 12.0, rect.top()),
            egui::pos2(rect.left() + 12.0 + name_width, rect.bottom()),
        );
        let value_rect = egui::Rect::from_min_max(
            egui::pos2(name_rect.right() + gap, rect.top()),
            egui::pos2(rect.right() - 12.0, rect.bottom()),
        );
        painter.with_clip_rect(name_rect).galley(
            egui::pos2(
                name_rect.left(),
                rect.center().y - name_galley.size().y * 0.5,
            ),
            name_galley,
            c.text_dim,
        );
        painter.with_clip_rect(value_rect).galley(
            egui::pos2(
                value_rect.left(),
                rect.center().y - value_galley.size().y * 0.5,
            ),
            value_galley,
            c.text,
        );
        if i + 1 < rows.len() {
            painter.hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
        }
    }
}
