//! Measurement tables — dimmed metric names left, mono values right, with
//! hairline row separators.

use egui::{Ui, WidgetInfo, WidgetType, vec2};

use crate::panels::{PROPERTY_ROW_PAD, property_row_columns};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// Render a measurement table across the available width.
///
/// Its columns are a property row's columns: a table of measurements sits in
/// the same inspector as the rows naming what was measured, and a name or a
/// value starting a few points off theirs reads as a second, misaligned grid.
pub fn measurement_table(ui: &mut Ui, rows: &[(&str, &str)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();

    for (i, (name, value)) in rows.iter().enumerate() {
        let (name_width, gap, value_width) = property_row_columns(width);
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
            egui::pos2(rect.left() + PROPERTY_ROW_PAD, rect.top()),
            egui::pos2(rect.left() + PROPERTY_ROW_PAD + name_width, rect.bottom()),
        );
        let value_rect = egui::Rect::from_min_max(
            egui::pos2(name_rect.right() + gap, rect.top()),
            egui::pos2(rect.right() - PROPERTY_ROW_PAD, rect.bottom()),
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
