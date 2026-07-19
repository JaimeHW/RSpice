//! Shared rendering primitives for the restored mockup's compact engineering
//! review dialogs.
//!
//! These dialogs intentionally use one exact geometry contract: a 35 point
//! purpose strip, a 230 point split review body, and a 260 point resolved
//! context pane inside the common 760 point transaction shell. Keeping that
//! contract here prevents independently implemented workflows from drifting
//! in spacing, typography, or validation treatment.

use egui::{
    Align, Align2, Frame, Layout, Margin, Response, Sense, Stroke, TextEdit, Ui, Vec2, vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

pub(crate) const BODY_HEIGHT: f32 = 230.0;
pub(crate) const CONTEXT_WIDTH: f32 = 260.0;
pub(crate) const PURPOSE_HEIGHT: f32 = 35.0;
pub(crate) const SURFACE_HEIGHT: f32 = 370.0;
pub(crate) const TRANSACTION_HEIGHT: f32 = 37.0;

const CONTEXT_EXPLANATION: &str = "The exact source identities, dependent artifacts, permission boundary, validation policy, and transactional commit point are reviewed before this action can complete.";

pub(crate) fn purpose_line(ui: &mut Ui, description: &str) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 0))
        .show(ui, |ui| {
            let width = ui.available_width();
            ui.allocate_ui_with_layout(
                vec2(width, PURPOSE_HEIGHT),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let (icon_rect, icon_response) =
                        ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
                    ui.painter().circle_stroke(
                        icon_rect.center(),
                        5.0,
                        Stroke::new(1.0, t.color.info),
                    );
                    ui.painter().text(
                        icon_rect.center() + vec2(0.0, -0.25),
                        Align2::CENTER_CENTER,
                        "i",
                        theme::mono(tokens::FS_0, FontWeight::Medium),
                        t.color.info,
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(description)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                    ui.ctx().accesskit_node_builder(icon_response.id, |node| {
                        node.set_label("Workflow purpose");
                        node.set_description(description);
                    });
                },
            );
        });
    paint_dashed_hline(
        ui,
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

pub(crate) fn resolved_context(ui: &mut Ui, project_name: &str, project_revision: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(BODY_HEIGHT);
        ui.spacing_mut().item_spacing.y = 0.0;
        context_heading(ui, "Resolved context");
        context_row(ui, "Project", project_name);
        context_row(ui, "Revision", project_revision);
        context_row(ui, "Change boundary", "owned source + dependency graph");
        Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(CONTEXT_EXPLANATION)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    });
}

pub(crate) fn field_label<R>(ui: &mut Ui, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(5.0);
    content(ui)
}

pub(crate) fn input_field(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    error: Option<&str>,
    description: &str,
) -> Response {
    field_label(ui, label, |ui| {
        let t = Tokens::get(ui.ctx());
        let response = ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            TextEdit::singleline(value)
                .font(egui::TextStyle::Monospace)
                .hint_text(hint)
                .margin(egui::Margin::symmetric(8, 4)),
        );
        configure_field_validation(ui, &response, label, error, description);
        response
    })
}

pub(crate) fn read_only_field(
    ui: &mut Ui,
    label: &str,
    value: &str,
    description: &str,
) -> Response {
    let mut text = value.to_owned();
    field_label(ui, label, |ui| {
        let t = Tokens::get(ui.ctx());
        let response = ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            TextEdit::singleline(&mut text)
                .font(egui::TextStyle::Monospace)
                .interactive(false)
                .margin(egui::Margin::symmetric(8, 4)),
        );
        configure_field_validation(ui, &response, label, None, description);
        response
    })
}

pub(crate) fn configure_field_validation(
    ui: &Ui,
    response: &Response,
    label: &str,
    error: Option<&str>,
    description: &str,
) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_description(if let Some(error) = error {
            format!("{description}. {error}")
        } else {
            description.to_owned()
        });
        if error.is_some() {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
    if error.is_some() {
        let t = Tokens::get(ui.ctx());
        ui.painter().rect_stroke(
            response.rect,
            t.radius,
            egui::Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
    }
}

pub(crate) fn paint_body_dividers(ui: &Ui, body_rect: egui::Rect, form_width: f32) {
    let t = Tokens::get(ui.ctx());
    ui.painter().vline(
        body_rect.left() + form_width,
        body_rect.y_range(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        body_rect.x_range(),
        body_rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn context_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label);
    });
}

fn context_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, 28.0), Sense::hover());
    let content = rect.shrink2(vec2(10.0, 0.0));
    let value_x = content.left() + (content.width() * 0.37).max(76.0);
    ui.painter().text(
        content.left_center(),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let value_rect = egui::Rect::from_min_max(
        egui::pos2(value_x + 8.0, content.top()),
        content.right_bottom(),
    );
    ui.painter().with_clip_rect(value_rect).text(
        value_rect.left_center(),
        Align2::LEFT_CENTER,
        value,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_value(value);
    });
}

fn paint_dashed_hline(ui: &Ui, range: egui::Rangef, y: f32, stroke: Stroke) {
    let mut x = range.min;
    while x < range.max {
        let end = (x + 3.0).min(range.max);
        ui.painter()
            .line_segment([egui::pos2(x, y), egui::pos2(end, y)], stroke);
        x += 6.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_review_geometry_matches_the_restored_mockup() {
        assert_eq!(SURFACE_HEIGHT, 370.0);
        assert_eq!(PURPOSE_HEIGHT, 35.0);
        assert_eq!(BODY_HEIGHT, 230.0);
        assert_eq!(CONTEXT_WIDTH, 260.0);
        assert_eq!(TRANSACTION_HEIGHT, 37.0);
        assert_eq!(
            CONTEXT_EXPLANATION,
            "The exact source identities, dependent artifacts, permission boundary, validation policy, and transactional commit point are reviewed before this action can complete."
        );
    }
}
