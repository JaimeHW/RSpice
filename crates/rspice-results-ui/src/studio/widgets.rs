//! Shared Studio panel, form, and table widgets.

use super::PANEL_HEADING_HEIGHT;
use egui::{Align, Color32, Frame, Layout, Margin, Rect, RichText, Sense, Stroke, Ui, Vec2, vec2};
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

pub fn dock_intro(ui: &mut Ui, eyebrow: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(eyebrow)
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.accent),
    );
    ui.label(
        RichText::new(description)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(8.0);
}

pub fn separator(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter()
        .hline(rect.x_range(), rect.center().y, Stroke::new(1.0, color));
}

pub fn paint_top_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.top() + 0.5, Stroke::new(1.0, color));
}

pub fn paint_bottom_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom() - 0.5, Stroke::new(1.0, color));
}

pub fn panel_heading(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(8, 0))
        .show(ui, |ui| {
            ui.set_min_height(PANEL_HEADING_HEIGHT);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(title.to_uppercase())
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(detail)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                });
            });
        });
}

pub fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
}

pub fn empty_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
}

pub fn concept_banner(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    Frame::NONE
        .fill(t.color.accent_dim)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
                WorkbenchIcon::Info.paint(ui.painter(), icon_rect, t.color.info);
                ui.label(
                    RichText::new(text)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        });
}

pub fn policy_row(ui: &mut Ui, label: &str, value: &str) {
    table_header(ui, label);
    ui.label(value);
    ui.end_row();
}

/// A label above its combo — a wrapped-row form field beside [`numeric_policy`],
/// not the design system's full-width `property_row_combo`. Its options arrive as
/// a closure so autoscale can disable the fit policies the renderer cannot honour.
pub fn labeled_combo(ui: &mut Ui, label: &str, selected: &str, add_contents: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(("visualization.combo", label))
            .selected_text(selected)
            .show_ui(ui, add_contents);
    });
}

pub fn numeric_policy(
    ui: &mut Ui,
    label: &str,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    suffix: &str,
) {
    ui.vertical(|ui| {
        ui.label(label);
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(value).range(range));
            ui.monospace(suffix);
        });
    });
}
