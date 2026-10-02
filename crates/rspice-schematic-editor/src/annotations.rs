//! Annotation placement forms over editable fields and explicit preview data.

use egui::{Align, Frame, Layout, Stroke, Ui, Vec2};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

pub mod note_placement;
pub mod shape_placement;

#[derive(Debug, Default)]
pub struct AnnotationFormResponse {
    pub focus: Option<egui::Id>,
    pub edited: bool,
}

fn field_label<R>(ui: &mut Ui, label: &str, body: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        body(ui)
    })
    .inner
}

fn section_head(ui: &mut Ui, title: &str, status: &str) {
    if section_head_wraps(ui.available_width(), title, status) {
        let width = ui.available_width();
        section_title(ui, title);
        ui.add_space(2.0);
        ui.allocate_ui_with_layout(
            Vec2::new(width, 14.0),
            Layout::right_to_left(Align::Center),
            |ui| section_status(ui, status),
        );
        return;
    }
    ui.horizontal(|ui| {
        section_title(ui, title);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            section_status(ui, status);
        });
    });
}

fn section_head_wraps(width: f32, title: &str, status: &str) -> bool {
    width < 292.0 && title.chars().count() + status.chars().count() > 34
}

fn section_title(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .wrap(),
    );
}

fn section_status(ui: &mut Ui, status: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(status)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}

fn status_card(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(6.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(40.0);
            ui.label(
                egui::RichText::new(label.to_uppercase())
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn narrow_section_heads_wrap_before_labels_overlap() {
        assert!(section_head_wraps(
            270.0,
            "Documentation parameters",
            "legal preview"
        ));
        assert!(!section_head_wraps(
            410.0,
            "Documentation parameters",
            "legal preview"
        ));
    }
}
