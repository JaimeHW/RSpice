//! Form primitives — inspector rows, key-value rows, checkboxes, and the
//! mono value input.

use egui::{Response, TextEdit, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Label column width of inspector form grids.
const LABEL_COL: f32 = 92.0;

/// Announce `name` as a control's accessible name.
///
/// Every label in this design system is painted rather than laid out as a
/// widget, so there is no label node for a control to be `labelled_by`. A
/// screen reader that reaches an input therefore hears whatever the widget
/// publishes for itself, which for [`TextEdit`] is nothing at all and for
/// [`egui::Checkbox`] is the word it happens to be showing — "Enabled" as the
/// name of the control rather than as its state.
///
/// Naming the node is the only route that survives that, and it belongs to the
/// constructors that use it so a call site cannot forget it: `mono_input` takes
/// the row's own label and there is no spelling of it that omits one.
pub(crate) fn name_control(ui: &Ui, response: &Response, name: &str) {
    debug_assert!(
        !name.trim().is_empty(),
        "a control announces the label its row states; an unnamed one is unreachable"
    );
    let name = name.to_owned();
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_label(name));
}

/// A monospace value input filling `width`, styled as an inset well.
///
/// `label` is what the row states beside or above the well, and becomes the
/// control's accessible name — see [`name_control`].
pub fn mono_input(ui: &mut Ui, label: &str, value: &mut String, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        TextEdit::singleline(value)
            .font(egui::TextStyle::Monospace)
            .margin(egui::Margin::symmetric(8, 4)),
    );
    name_control(ui, &response, label);
    response
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
            mono_input(ui, label, value, ui.available_width())
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

/// A labeled chip-choice row for small enums (sweep type, distribution).
/// Returns `true` if the selection changed.
pub fn choice_row(ui: &mut Ui, label: &str, options: &[&str], value: &mut usize) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;
    let mut changed = false;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_COL + 4.0, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            for (idx, option) in options.iter().enumerate() {
                if super::chip(ui, option, *value == idx).clicked() {
                    *value = idx;
                    changed = true;
                }
            }
        },
    );
    changed
}
