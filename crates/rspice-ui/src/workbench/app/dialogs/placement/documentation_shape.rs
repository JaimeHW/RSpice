//! Mockup-owned Draw documentation shape transaction.

use egui::{Align, Context, Frame, Layout, Sense, Stroke, Ui, Vec2};

use crate::state::{
    DocumentationShapeKind, DocumentationShapeLayer, PendingDocumentationShapePlacement, Tool,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    select_with_response,
};

use crate::workbench::app::{AppState, DocumentationShapeDialogState, RSpiceApp};

const EYEBROW: &str = "SCHEMATIC \u{00b7} GRAPHICS";
const TITLE: &str = "Draw documentation shape";
const PRIMARY: &str = "Arm shape tool";
const DESCRIPTION: &str =
    "Draw lines, rectangles, polygons, arcs, or callouts on non-electrical documentation layers.";
const DIALOG_SIZE: DialogSize = DialogSize::Transaction;
const FOOTER_NOTE: &str = "Pointer, touch, stylus, and keyboard entry resolve to the same exact coordinates. Escape cancels without modifying the document.";
const WORKFLOW_HEIGHT: f32 = 376.0;
const PREVIEW_HEIGHT: f32 = 250.0;
const PANE_PADDING: i8 = 14;
const STACKED_BREAKPOINT: f32 = 760.0;
const WIDE_TRACK_BREAKPOINT: f32 = 980.0;
const DISCARD_TITLE: &str = "Unsaved dialog changes";
const DISCARD_DETAIL: &str = "Choose Discard changes again to close, or continue editing. No schematic graphics have been changed.";

#[derive(Debug)]
enum DraftValidation {
    Invalid(String),
    Valid(PendingDocumentationShapePlacement),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid(_) => None,
        }
    }
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_documentation_shape_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.documentation_shape.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_message = validation.message().map(str::to_owned);
        let discard_confirm = self.state.dialogs.documentation_shape.discard_confirm;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DIALOG_SIZE)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            workflow_body(
                ui,
                validation_message.as_deref(),
                &mut self.state.dialogs.documentation_shape,
            )
        });
        match choice {
            DialogChoice::Primary => {
                if let DraftValidation::Valid(pending) = validate_draft(&self.state) {
                    self.state.schematic.pending_documentation_shape = Some(pending);
                    self.state.schematic.documentation_shape_drawing.clear();
                    crate::workbench::commands::arm_schematic_tool(
                        &mut self.state.schematic,
                        Tool::DocumentationShape,
                    );
                    self.state.dialogs.documentation_shape.close();
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.documentation_shape.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.documentation_shape;
    if state.schematic.read_only || state.active_view_read_only() {
        return DraftValidation::Invalid("The active schematic is read-only.".to_owned());
    }
    if draft.design_execution_epoch != state.design_execution_epoch {
        return DraftValidation::Invalid(
            "The design document changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if draft.active_schematic_epoch != state.active_schematic_epoch {
        return DraftValidation::Invalid(
            "The active schematic buffer changed. Close and reopen Draw documentation shape."
                .to_owned(),
        );
    }
    if draft.topology_version != state.schematic.topology_version() {
        return DraftValidation::Invalid(
            "The schematic topology changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if draft.view_path != state.workspace.active_view.display_path() {
        return DraftValidation::Invalid(
            "The active cell/view changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    if draft.expected_shapes != state.schematic.documentation_shapes {
        return DraftValidation::Invalid(
            "The schematic graphics changed. Close and reopen Draw documentation shape.".to_owned(),
        );
    }
    DraftValidation::Valid(
        PendingDocumentationShapePlacement::new(
            draft.kind,
            draft.topology_version,
            &draft.expected_shapes,
        )
        .with_document_authority(
            draft.design_execution_epoch,
            draft.active_schematic_epoch,
            draft.view_path.clone(),
        ),
    )
}

fn workflow_body(
    ui: &mut Ui,
    validation_message: Option<&str>,
    draft: &mut DocumentationShapeDialogState,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let mut focus = None;
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(10.0)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let viewport_width = crate::ui::viewport::root_viewport_width(ui.ctx());
            if workflow_uses_columns(viewport_width) {
                let divider = 1.0;
                let content_width = (ui.available_width() - divider).max(1.0);
                let (right_fraction, right_minimum) = workflow_right_track(viewport_width);
                let right_width = (content_width * right_fraction)
                    .max(right_minimum)
                    .min(content_width - 1.0);
                let left_width = content_width - right_width;
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(left_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| preview_pane(ui, draft),
                    );
                    let divider_rect = ui
                        .allocate_exact_size(Vec2::new(divider, WORKFLOW_HEIGHT), Sense::hover())
                        .0;
                    ui.painter().rect_filled(divider_rect, 0.0, t.color.border);
                    ui.allocate_ui_with_layout(
                        Vec2::new(right_width, WORKFLOW_HEIGHT),
                        Layout::top_down(Align::Min),
                        |ui| focus = fields_pane(ui, validation_message, draft),
                    );
                });
            } else {
                preview_pane(ui, draft);
                let (divider, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
                ui.painter().rect_filled(divider, 0.0, t.color.border);
                focus = fields_pane(ui, validation_message, draft);
            }
        });
    focus
}

fn workflow_uses_columns(viewport_width: f32) -> bool {
    viewport_width > STACKED_BREAKPOINT
}

fn workflow_right_track(width: f32) -> (f32, f32) {
    if width <= WIDE_TRACK_BREAKPOINT {
        (0.72 / 1.72, 240.0)
    } else {
        (0.8 / 2.35, 270.0)
    }
}

fn preview_pane(ui: &mut Ui, draft: &DocumentationShapeDialogState) {
    Frame::new()
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_head(ui, "SHAPE \u{00b7} live schematic preview", "100 mil grid");
            paint_preview(ui, draft);
            ui.add_space(9.0);
            let cards = [
                ("Electrical outcome", "presentation geometry"),
                (
                    "Checks",
                    "connectivity \u{00b7} discipline \u{00b7} hierarchy",
                ),
                ("Commit", "stable IDs + one undo record"),
            ];
            if workflow_uses_columns(crate::ui::viewport::root_viewport_width(ui.ctx())) {
                ui.spacing_mut().item_spacing.x = 8.0;
                ui.columns(3, |columns| {
                    for (column, (label, value)) in columns.iter_mut().zip(cards) {
                        status_card(column, label, value);
                    }
                });
            } else {
                for (label, value) in cards {
                    status_card(ui, label, value);
                    ui.add_space(5.0);
                }
            }
        });
}

fn fields_pane(
    ui: &mut Ui,
    validation_message: Option<&str>,
    draft: &mut DocumentationShapeDialogState,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    let mut focus = None;
    Frame::new()
        .fill(theme::mix(t.color.bg_inset, t.color.bg_panel, 0.94))
        .inner_margin(egui::Margin::same(PANE_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if workflow_uses_columns(crate::ui::viewport::root_viewport_width(ui.ctx())) {
                ui.set_min_height(WORKFLOW_HEIGHT - 2.0 * f32::from(PANE_PADDING));
            }
            ui.spacing_mut().item_spacing.y = 9.0;
            section_head(
                ui,
                "Placement / transform parameters",
                if validation_message.is_some() {
                    "blocked"
                } else {
                    "legal preview"
                },
            );
            let labels = DocumentationShapeKind::ALL.map(|kind| kind.label().to_owned());
            let output = field_label(ui, "Shape", |ui| {
                select_with_response(
                    ui,
                    "documentation-shape-kind",
                    "Shape",
                    draft.kind.label(),
                    &labels,
                    ui.available_width(),
                )
            });
            focus = Some(output.response.id);
            if let Some(index) = output.picked {
                let next = DocumentationShapeKind::ALL[index];
                if next != draft.kind {
                    draft.kind = next;
                    draft.mark_edited();
                }
            }
            read_only_value(
                ui,
                "Layer",
                DocumentationShapeLayer::DrawingDocumentation.label(),
            );
            read_only_value(ui, "Electrical connectivity", "none");
            if let Some(message) = validation_message {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.err),
                    )
                    .wrap(),
                );
            }
            ui.add(
                egui::Label::new(
                    egui::RichText::new(FOOTER_NOTE)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    focus
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
                ui.label(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                        .color(t.color.text),
                );
            });
    });
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

fn paint_preview(ui: &mut Ui, draft: &DocumentationShapeDialogState) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PREVIEW_HEIGHT),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            "Preview presentation geometry for review boundary",
        )
    });
    response.on_hover_text("review boundary\nrounded rectangle");
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
    let center = rect.center();
    let stroke = Stroke::new(1.5, t.color.accent);
    let shape_rect = egui::Rect::from_center_size(center, Vec2::new(156.0, 82.0));
    match draft.kind {
        DocumentationShapeKind::Rectangle => {
            painter.rect_stroke(shape_rect, 8.0, stroke, egui::StrokeKind::Inside)
        }
        DocumentationShapeKind::Line => {
            painter.line_segment([shape_rect.left_bottom(), shape_rect.right_top()], stroke)
        }
        DocumentationShapeKind::Polygon => painter.add(egui::Shape::closed_line(
            vec![
                shape_rect.left_bottom(),
                egui::pos2(shape_rect.center().x, shape_rect.top()),
                shape_rect.right_bottom(),
            ],
            stroke,
        )),
        DocumentationShapeKind::Arc => painter.add(egui::Shape::line(
            (0..=24)
                .map(|step| {
                    let angle = std::f32::consts::PI * step as f32 / 24.0;
                    center + egui::vec2(angle.cos() * 70.0, -angle.sin() * 48.0)
                })
                .collect(),
            stroke,
        )),
        DocumentationShapeKind::Callout => {
            painter.line_segment(
                [
                    shape_rect.left_center() - egui::vec2(54.0, 34.0),
                    shape_rect.left_center(),
                ],
                stroke,
            );
            painter.rect_stroke(shape_rect, 8.0, stroke, egui::StrokeKind::Inside)
        }
    };
    painter.text(
        egui::pos2(shape_rect.left() + 10.0, shape_rect.top() + 10.0),
        egui::Align2::LEFT_TOP,
        "SHAPE",
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
    painter.text(
        shape_rect.center(),
        egui::Align2::CENTER_CENTER,
        "review boundary",
        theme::sans(tokens::FS_1, FontWeight::Medium),
        t.color.text,
    );
    painter.text(
        egui::pos2(shape_rect.center().x, shape_rect.bottom() - 10.0),
        egui::Align2::CENTER_BOTTOM,
        if draft.kind == DocumentationShapeKind::Rectangle {
            "rounded rectangle"
        } else {
            draft.kind.label()
        },
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_contract_is_exact() {
        assert_eq!(TITLE, "Draw documentation shape");
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} GRAPHICS");
        assert_eq!(PRIMARY, "Arm shape tool");
        assert_eq!(DIALOG_SIZE, DialogSize::Transaction);
        assert_eq!(DocumentationShapeKind::ALL.len(), 5);
        assert_eq!(DocumentationShapeKind::Rectangle.label(), "Rectangle");
        assert_eq!(
            DocumentationShapeLayer::DrawingDocumentation.label(),
            "drawing / documentation"
        );
    }

    #[test]
    fn responsive_tracks_match_the_mockup() {
        assert!(!workflow_uses_columns(STACKED_BREAKPOINT));
        assert!(workflow_uses_columns(STACKED_BREAKPOINT + 1.0));
        assert_eq!(workflow_right_track(980.0), (0.72 / 1.72, 240.0));
        assert_eq!(workflow_right_track(981.0), (0.8 / 2.35, 270.0));
        assert_eq!(PREVIEW_HEIGHT, 250.0);
        assert!(section_head_wraps(
            240.0,
            "Placement / transform parameters",
            "legal preview"
        ));
    }
}
