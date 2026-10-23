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
use crate::ui::widgets::Problem;

pub(crate) const PURPOSE_HEIGHT: f32 = 35.0;

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
                    ui.set_min_size(vec2(width, PURPOSE_HEIGHT));
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let (icon_rect, icon_response) =
                        ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
                    ui.painter().circle_stroke(
                        icon_rect.center(),
                        5.0,
                        Stroke::new(1.0, t.color.text_faint),
                    );
                    ui.painter().text(
                        icon_rect.center() + vec2(0.0, -0.25),
                        Align2::CENTER_CENTER,
                        "i",
                        theme::mono(tokens::FS_0, FontWeight::Medium),
                        t.color.text_faint,
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

/// A problem stated on its own line inside a dialog body: the mark that owns
/// the severity, then neutral text.
///
/// Dialogs that paint notes in a column of their own share this one painter,
/// so a warning in Publish and a warning in Live session are the same mark at
/// the same size with the same gap behind it.
pub(crate) fn problem_note(ui: &mut Ui, problem: Problem, text: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = PROBLEM_NOTE_GAP;
        let (rect, _) =
            ui.allocate_exact_size(vec2(PROBLEM_NOTE_MARK, PROBLEM_NOTE_LINE), Sense::hover());
        crate::ui::widgets::paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(rect.center().x, rect.top() + PROBLEM_NOTE_LINE * 0.5),
                Vec2::splat(PROBLEM_NOTE_MARK),
            ),
            problem,
        );
        let response = ui.add(
            egui::Label::new(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
        let announcement = problem.announce(text);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
        });
        response
    })
    .inner
}

/// The mark's side and the gap behind it, read from the kit so a note's mark
/// is the mark every other panel sets.
use crate::ui::widgets::{
    PROBLEM_MARK_GAP as PROBLEM_NOTE_GAP, PROBLEM_MARK_SIDE as PROBLEM_NOTE_MARK,
};
/// The line the mark is centred on.
const PROBLEM_NOTE_LINE: f32 = 16.0;

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
    text_edit_field(
        ui,
        label,
        TextEdit::singleline(value).hint_text(hint),
        error,
        description,
    )
}

/// A compact input whose caller supplies its stable identity and editor options.
pub(crate) fn text_edit_field(
    ui: &mut Ui,
    label: &str,
    edit: TextEdit<'_>,
    error: Option<&str>,
    description: &str,
) -> Response {
    field_label(ui, label, |ui| {
        let t = Tokens::get(ui.ctx());
        let response = ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            edit.font(egui::TextStyle::Monospace)
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

fn paint_dashed_hline(ui: &Ui, range: egui::Rangef, y: f32, stroke: Stroke) {
    let mut x = range.min;
    while x < range.max {
        let end = (x + 3.0).min(range.max);
        ui.painter()
            .line_segment([egui::pos2(x, y), egui::pos2(end, y)], stroke);
        x += 6.0;
    }
}

/// Where a dialog's rows start: its title and footer sit this far in from
/// the frame, and so does everything between them.
pub(crate) const DIALOG_BODY_INSET: f32 = 12.0;

/// A section's band across a flush dialog body or one pane of it: its title
/// on the body's left edge, an optional count at its right. A dialog wears
/// this one header style for every section it has.
pub(crate) fn band_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            ui.available_width(),
            crate::workbench::design_system::PANEL_SECTION_H,
        ),
        Sense::hover(),
    );
    let spoken = meta.map_or_else(|| title.to_owned(), |meta| format!("{title}, {meta}"));
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &spoken));
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(DIALOG_BODY_INSET, 0.0),
        Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    if let Some(meta) = meta {
        ui.painter().text(
            rect.right_center() - vec2(DIALOG_BODY_INSET, 0.0),
            Align2::RIGHT_CENTER,
            meta,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
}
