//! Design-note placement fields and preview.

use super::{AnnotationFormResponse, field_label, section_head, status_card};
use egui::{Align, Frame, Layout, Sense, Stroke, TextEdit, Ui, Vec2};
use rspice_design::schematic::design_note::{DesignNoteKind, DesignNoteLayer};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::select;

const WORKFLOW_HEIGHT: f32 = 356.0;
const SPLIT_MIN_WIDTH: f32 = 660.0;
const SPLIT_PREVIEW_HEIGHT: f32 = 220.0;
const STACKED_PREVIEW_HEIGHT: f32 = 116.0;
const PANE_PADDING: i8 = 14;
const STATUS_CARDS_INLINE_MIN_WIDTH: f32 = 320.0;

pub fn text_id() -> egui::Id {
    egui::Id::new(("rspice.design-note", "text"))
}

pub fn show(
    ui: &mut Ui,
    validation_message: Option<&str>,
    preview_text: &str,
    kind: &mut DesignNoteKind,
    text: &mut String,
) -> AnnotationFormResponse {
    let t = Tokens::get(ui.ctx());
    let mut response = AnnotationFormResponse::default();
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            if uses_split_layout(ui.available_width()) {
                let divider = 1.0;
                let content_width = (ui.available_width() - divider).max(1.0);
                let right_width = (content_width * 0.38).max(270.0).min(content_width - 1.0);
                let left_width = content_width - right_width;
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(left_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| preview_pane(ui, *kind, text, preview_text, SPLIT_PREVIEW_HEIGHT),
                    );
                    let divider_rect = ui
                        .allocate_exact_size(Vec2::new(divider, WORKFLOW_HEIGHT), Sense::hover())
                        .0;
                    ui.painter().rect_filled(divider_rect, 0.0, t.color.border);
                    ui.allocate_ui_with_layout(
                        Vec2::new(right_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| response = fields_pane(ui, validation_message, kind, text),
                    );
                });
            } else {
                preview_pane(ui, *kind, text, preview_text, STACKED_PREVIEW_HEIGHT);
                let (divider, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
                ui.painter().rect_filled(divider, 0.0, t.color.border);
                response = fields_pane(ui, validation_message, kind, text);
            }
        });
    response
}

fn uses_split_layout(available_width: f32) -> bool {
    available_width >= SPLIT_MIN_WIDTH
}

fn preview_pane(
    ui: &mut Ui,
    kind: DesignNoteKind,
    text: &str,
    preview_text: &str,
    preview_height: f32,
) {
    Frame::new()
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_head(ui, "TEXT · live schematic preview", "100 mil grid");
            paint_preview(ui, kind, text, preview_text, preview_height);
            ui.add_space(9.0);
            let values = [
                ("Object", "Calibration path"),
                ("Layer", "annotation only"),
                ("Electrical", "non-electrical note"),
            ];
            if ui.available_width() >= STATUS_CARDS_INLINE_MIN_WIDTH {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.columns(3, |columns| {
                    for (column, (label, value)) in columns.iter_mut().zip(values) {
                        status_card(column, label, value);
                    }
                });
            } else {
                for (label, value) in values {
                    status_card(ui, label, value);
                    ui.add_space(5.0);
                }
            }
        });
}

fn fields_pane(
    ui: &mut Ui,
    validation_message: Option<&str>,
    kind: &mut DesignNoteKind,
    text: &mut String,
) -> AnnotationFormResponse {
    let t = Tokens::get(ui.ctx());
    let mut response = AnnotationFormResponse::default();
    Frame::new()
        .fill(theme::mix(t.color.bg_inset, t.color.bg_panel, 0.94))
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 9.0;
            section_head(
                ui,
                "Documentation parameters",
                if validation_message.is_some() {
                    "blocked"
                } else {
                    "legal preview"
                },
            );
            let labels = DesignNoteKind::ALL.map(|kind| kind.label().to_owned());
            let kind_changed = field_label(ui, "Type", |ui| {
                select(
                    ui,
                    "design-note-kind",
                    "Type",
                    kind.label(),
                    &labels,
                    ui.available_width(),
                )
            })
            .is_some_and(|index| {
                let replace_default = is_default_text(text);
                *kind = DesignNoteKind::ALL[index];
                if replace_default {
                    *text = default_text(*kind).to_owned();
                }
                true
            });
            let text_response = field_label(ui, "Text", |ui| {
                ui.add_sized(
                    [ui.available_width(), 72.0],
                    TextEdit::multiline(text)
                        .id(text_id())
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .hint_text("Bias network"),
                )
            });
            response.focus = Some(text_response.id);
            read_only_value(ui, "Layer", DesignNoteLayer::DrawingAnnotation.label());
            if kind_changed || text_response.changed() {
                response.edited = true;
            }
            if let Some(message) = validation_message {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    )
                    .wrap(),
                );
            } else {
                ui.label(
                    egui::RichText::new(
                        "The placed object is documentation-only. It is retained with the schematic and cannot create or rename electrical connectivity.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            }
        });
    response
}

fn default_text(kind: DesignNoteKind) -> &'static str {
    match kind {
        DesignNoteKind::PlainText => "Bias network",
        DesignNoteKind::PropertyDisplay => "${component_count} components",
        DesignNoteKind::RequirementLink => "REQ-19",
        DesignNoteKind::ReviewNote => "Review bias network",
    }
}

fn is_default_text(text: &str) -> bool {
    DesignNoteKind::ALL
        .into_iter()
        .any(|kind| text == default_text(kind))
}

fn read_only_value(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    field_label(ui, label, |ui| {
        Frame::new()
            .fill(t.color.bg_app)
            .stroke(Stroke::new(1.0, t.color.border))
            .corner_radius(3.0)
            .inner_margin(egui::Margin::symmetric(8, 6))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
            });
    });
}

fn paint_preview(ui: &mut Ui, kind: DesignNoteKind, text: &str, preview_text: &str, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            format!("{} preview: {}", kind.label(), text),
        )
    });
    ui.painter().rect(
        rect,
        8.0,
        t.color.canvas_bg,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    let mut y = rect.top() + 7.0;
    while y < rect.bottom() {
        let mut x = rect.left() + 7.0;
        while x < rect.right() {
            ui.painter()
                .circle_filled(egui::pos2(x, y), 0.7, t.color.canvas_grid);
            x += 12.0;
        }
        y += 12.0;
    }
    let painter = ui.painter().with_clip_rect(rect.shrink(2.0));
    let marker = egui::pos2(rect.left() + 28.0, rect.center().y);
    let color = match kind {
        DesignNoteKind::ReviewNote => t.color.warn,
        DesignNoteKind::RequirementLink | DesignNoteKind::PropertyDisplay => t.color.accent,
        DesignNoteKind::PlainText => t.color.text,
    };
    match kind {
        DesignNoteKind::PlainText => {
            painter.circle_filled(marker, 2.5, color);
        }
        DesignNoteKind::PropertyDisplay => {
            painter.rect_stroke(
                egui::Rect::from_center_size(marker, Vec2::splat(5.0)),
                0.0,
                Stroke::new(1.0, color),
                egui::StrokeKind::Inside,
            );
        }
        DesignNoteKind::RequirementLink => {
            painter.circle_stroke(marker, 2.5, Stroke::new(1.0, color));
        }
        DesignNoteKind::ReviewNote => {
            painter.line_segment(
                [
                    marker + egui::vec2(-2.5, -2.5),
                    marker + egui::vec2(2.5, 2.5),
                ],
                Stroke::new(1.0, color),
            );
            painter.line_segment(
                [
                    marker + egui::vec2(2.5, -2.5),
                    marker + egui::vec2(-2.5, 2.5),
                ],
                Stroke::new(1.0, color),
            );
        }
    }
    let origin = marker + egui::vec2(9.0, 0.0);
    for (index, line) in preview_text.split('\n').enumerate() {
        let position = origin + egui::vec2(0.0, index as f32 * 17.0);
        let galley = painter.layout_no_wrap(
            if line.is_empty() { " " } else { line }.to_owned(),
            theme::mono(tokens::FS_2, FontWeight::Regular),
            color,
        );
        painter.galley(position, galley.clone(), color);
        if kind == DesignNoteKind::RequirementLink {
            painter.line_segment(
                [
                    position + egui::vec2(0.0, galley.size().y),
                    position + galley.size(),
                ],
                Stroke::new(1.0, color),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn changing_type_preserves_authored_text_but_recognizes_known_defaults() {
        assert!(is_default_text("Bias network"));
        assert!(is_default_text("REQ-19"));
        assert!(!is_default_text("User-authored note"));
    }
    #[test]
    fn responsive_workflow_switches_at_split_threshold() {
        assert!(uses_split_layout(SPLIT_MIN_WIDTH));
        assert!(!uses_split_layout(SPLIT_MIN_WIDTH - 1.0));
    }
}
