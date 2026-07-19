//! Exact primitives for the restored mockup's four-stage operation dialog.

use egui::{Align2, Frame, Sense, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

pub(crate) const BODY_HEIGHT: f32 = 230.0;
pub(crate) const CONTEXT_WIDTH: f32 = 260.0;
pub(crate) const STEPS_HEIGHT: f32 = 36.0;
pub(crate) const SURFACE_HEIGHT: f32 = 406.0;
pub(crate) const TRANSACTION_HEIGHT: f32 = 37.0;

pub(crate) fn operation_steps(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(vec2(width, STEPS_HEIGHT), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let labels = [
        "1 \u{00b7} Scope",
        "2 \u{00b7} Options",
        "3 \u{00b7} Review",
        "4 \u{00b7} Apply",
    ];
    let item_width = rect.width() / labels.len() as f32;
    for (index, label) in labels.into_iter().enumerate() {
        let left = rect.left() + index as f32 * item_width;
        let right = if index + 1 == labels.len() {
            rect.right()
        } else {
            left + item_width
        };
        let item = egui::Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom() - 1.0),
        );
        if index == 1 {
            ui.painter().rect_filled(item, 0.0, t.color.bg_active);
            ui.painter().hline(
                item.x_range(),
                item.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        }
        if index + 1 < labels.len() {
            ui.painter()
                .vline(right, item.y_range(), Stroke::new(1.0, t.color.border));
        }
        ui.painter().text(
            item.left_center() + vec2(10.0, 0.0),
            Align2::LEFT_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            if index == 0 {
                t.color.ok
            } else if index == 1 {
                t.color.text
            } else {
                t.color.text_faint
            },
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Replacement operation progress");
        node.set_description("Scope complete; Options active; Review and Apply follow.");
    });
}

pub(crate) fn impact_preview(ui: &mut Ui, project: &str, revision: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_min_height(BODY_HEIGHT);
        ui.spacing_mut().item_spacing.y = 0.0;
        heading(ui, "Impact preview");
        fact_row(ui, "Project", project);
        fact_row(ui, "Revision", revision);
        fact_row(ui, "Change boundary", "owned source + dependency graph");
        table_row(ui, "Owned source", "reviewed working revision");
        table_row(ui, "Existing results", "retained with original inputs");
        table_row(ui, "Undo boundary", "one transactional operation");
    });
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

fn heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label);
    });
}

fn fact_row(ui: &mut Ui, label: &str, value: &str) {
    two_column_row(ui, label, value, 28.0, false);
}

fn table_row(ui: &mut Ui, label: &str, value: &str) {
    two_column_row(ui, label, value, 31.0, true);
}

fn two_column_row(ui: &mut Ui, label: &str, value: &str, height: f32, table: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    let label_width = if table {
        (rect.width() * 0.42).max(76.0)
    } else {
        (rect.width() * 0.37).max(76.0)
    };
    let split = (rect.left() + label_width).min(rect.right());
    if table {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
        ui.painter()
            .vline(split, rect.y_range(), Stroke::new(1.0, t.color.border));
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        theme::sans(
            tokens::FS_0,
            if table {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        ),
        if table {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let value_rect = egui::Rect::from_min_max(
        egui::pos2(split + 10.0, rect.top()),
        egui::pos2(rect.right() - 8.0, rect.bottom()),
    );
    let value_font = if table {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Medium)
    };
    let (display_value, was_truncated) =
        ellipsized_text(ui, value, value_rect.width(), &value_font, t.color.text);
    ui.painter().with_clip_rect(value_rect).text(
        value_rect.left_center(),
        Align2::LEFT_CENTER,
        display_value,
        value_font,
        t.color.text,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_value(value);
    });
    if was_truncated {
        response.on_hover_text(value);
    }
}

fn ellipsized_text(
    ui: &Ui,
    value: &str,
    available_width: f32,
    font: &egui::FontId,
    color: egui::Color32,
) -> (String, bool) {
    let fits = |candidate: &str| {
        ui.painter()
            .layout_no_wrap(candidate.to_owned(), font.clone(), color)
            .size()
            .x
            <= available_width
    };
    if fits(value) {
        return (value.to_owned(), false);
    }

    let mut prefix = value.to_owned();
    while !prefix.is_empty() {
        prefix.pop();
        let candidate = format!("{}…", prefix.trim_end());
        if fits(&candidate) {
            return (candidate, true);
        }
    }
    ("…".to_owned(), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restored_operation_geometry_is_exact() {
        assert_eq!(SURFACE_HEIGHT, 406.0);
        assert_eq!(STEPS_HEIGHT, 36.0);
        assert_eq!(BODY_HEIGHT, 230.0);
        assert_eq!(CONTEXT_WIDTH, 260.0);
        assert_eq!(TRANSACTION_HEIGHT, 37.0);
    }
}
