//! Shared workbench menu-row primitives used by context menus.

use egui::{Sense, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

pub(crate) fn item(ui: &mut Ui, label: &str, shortcut: Option<&str>) -> bool {
    row(ui, label, shortcut, true).0
}

pub(crate) fn item_disabled(ui: &mut Ui, label: &str, shortcut: Option<&str>) {
    let _ = row(ui, label, shortcut, false);
}

fn row(ui: &mut Ui, label: &str, shortcut: Option<&str>, enabled: bool) -> (bool, egui::Response) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(220.0), 28.0),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled && ui.is_enabled(), label));
    if enabled && response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_2, FontWeight::Regular),
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    if let Some(shortcut) = shortcut {
        ui.painter().text(
            egui::pos2(rect.right() - 10.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            shortcut,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    (enabled && response.clicked(), response)
}

pub(crate) fn separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 11.0), Sense::hover());
    ui.painter().hline(
        egui::Rangef::new(rect.left() + 5.0, rect.right() - 5.0),
        rect.center().y,
        egui::Stroke::new(1.0, t.color.border),
    );
}
