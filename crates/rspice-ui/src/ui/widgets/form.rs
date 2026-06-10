//! Form primitives — inspector rows, key-value rows, checkboxes, and the
//! mono value input.

use egui::{Response, TextEdit, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Label column width of inspector form grids.
const LABEL_COL: f32 = 92.0;

/// A monospace value input filling `width`, styled as an inset well.
pub fn mono_input(ui: &mut Ui, value: &mut String, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        TextEdit::singleline(value)
            .font(egui::TextStyle::Monospace)
            .margin(egui::Margin::symmetric(8.0, 4.0)),
    )
}

/// An inspector form row: dimmed label in a fixed column, editable mono
/// value filling the remainder. Returns the edit response.
pub fn input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_COL, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            mono_input(ui, value, ui.available_width())
        },
    )
    .inner
}

/// A read-only inspector row (value rendered dimmed, still selectable).
pub fn input_row_readonly(ui: &mut Ui, label: &str, value: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_COL, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            let mut text = value.to_owned();
            ui.add_sized(
                vec2(ui.available_width(), t.metrics.ctl_h),
                TextEdit::singleline(&mut text)
                    .font(egui::TextStyle::Monospace)
                    .text_color(c.text_dim)
                    .interactive(false)
                    .margin(egui::Margin::symmetric(8.0, 4.0)),
            )
        },
    )
    .inner
}

/// A key-value row: dimmed key left, mono value right.
pub fn kv_row(ui: &mut Ui, key: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, 22.0), egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter();
    painter.text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        key,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        c.text_dim,
    );
    painter.text(
        egui::pos2(rect.right(), rect.center().y),
        egui::Align2::RIGHT_CENTER,
        value,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        c.text,
    );
}

/// A labeled checkbox row. Returns `true` if the value changed.
pub fn check_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let row = super::tree::TreeRow::new(label)
        .checkbox(value)
        .height(t.metrics.row_h);
    row.show(ui).checkbox_changed
}
