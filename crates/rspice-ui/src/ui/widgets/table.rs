//! Measurement tables — dimmed metric names left, mono values right, with
//! hairline row separators.

use egui::{Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Render a measurement table across the available width.
pub fn measurement_table(ui: &mut Ui, rows: &[(&str, &str)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = 25.0;
    let width = ui.available_width();

    for (i, (name, value)) in rows.iter().enumerate() {
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
        painter.text(
            egui::pos2(rect.left() + 12.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            name,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text_dim,
        );
        painter.text(
            egui::pos2(rect.right() - 12.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_1, FontWeight::Regular),
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
