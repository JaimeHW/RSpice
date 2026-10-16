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

/// The mark's side, the gap behind it, and the line it is centred on.
const PROBLEM_NOTE_MARK: f32 = 11.0;
const PROBLEM_NOTE_GAP: f32 = 6.0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_purpose_line_keeps_its_height() {
        assert_eq!(PURPOSE_HEIGHT, 35.0);
    }

    /// Nothing in the dialog-content slice paints a status in colour.
    ///
    /// Success has no styling at all, so the success tone, the information
    /// tone, a status dot and the two `StatusMark` variants that are not
    /// problems may not appear in any of these files. A warning or a failure
    /// is a `Problem` mark in front of neutral text, and the error tone stays
    /// allowed where it sits under the input it rejects.
    #[test]
    fn no_dialog_content_file_paints_a_status_in_colour() {
        const BANNED: [&str; 6] = [
            "colored_label",
            "status_dot(",
            "StatusMark::Success",
            "StatusMark::Neutral",
            "color.ok",
            "color.info",
        ];
        // Each entry is one shipped file of this slice, read back as it ships.
        const SLICE: [(&str, &str); 20] = [
            ("check_and_save.rs", include_str!("check_and_save.rs")),
            (
                "check_and_save_validation.rs",
                include_str!("check_and_save_validation.rs"),
            ),
            (
                "operation_primitives.rs",
                include_str!("operation_primitives.rs"),
            ),
            ("review_primitives.rs", include_str!("review_primitives.rs")),
            ("live_session.rs", include_str!("live_session.rs")),
            ("publish_web.rs", include_str!("publish_web.rs")),
            ("unpublish_web.rs", include_str!("unpublish_web.rs")),
            ("source_document.rs", include_str!("source_document.rs")),
            ("source_find.rs", include_str!("source_find.rs")),
            (
                "source_language_tools.rs",
                include_str!("source_language_tools.rs"),
            ),
            ("design_management.rs", include_str!("design_management.rs")),
            (
                "design_management/manager.rs",
                include_str!("design_management/manager.rs"),
            ),
            (
                "design_management/subflows.rs",
                include_str!("design_management/subflows.rs"),
            ),
            (
                "design_management/widgets.rs",
                include_str!("design_management/widgets.rs"),
            ),
            (
                "design_management/operations.rs",
                include_str!("design_management/operations.rs"),
            ),
            (
                "design_management/controller.rs",
                include_str!("design_management/controller.rs"),
            ),
            (
                "preferences/shortcut_preferences.rs",
                include_str!("preferences/shortcut_preferences.rs"),
            ),
            (
                "preferences/shortcut_portability_dialogs.rs",
                include_str!("preferences/shortcut_portability_dialogs.rs"),
            ),
            (
                "preferences/preference_pages.rs",
                include_str!("preferences/preference_pages.rs"),
            ),
            (
                "preferences/preferences_shell.rs",
                include_str!("preferences/preferences_shell.rs"),
            ),
        ];
        for (name, source) in SLICE {
            let production = crate::source_guard::production_source(source);
            for banned in BANNED {
                assert!(
                    !production.contains(banned),
                    "{name} still paints a status with {banned:?}"
                );
            }
        }
    }
}
