//! Shared rendering primitives for the restored mockup's compact engineering
//! review dialogs.
//!
//! These dialogs intentionally use one exact geometry contract: a 230 point
//! split review body and a 260 point resolved context pane inside the common
//! 760 point transaction shell. Keeping that contract here prevents
//! independently implemented workflows from drifting in spacing, typography,
//! or validation treatment.

use egui::{Response, Sense, TextEdit, Ui, Vec2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Problem;

/// The width a select needs to show the widest of `options` whole: the kit's
/// select starts its value 8 points into the well and keeps 24 clear at the
/// right for its chevron. A select laid at a guessed width cut its longer
/// choices to "Current revisio…carried forward".
pub fn fitted_select_width<S: AsRef<str>>(ui: &Ui, options: &[S]) -> f32 {
    let font = theme::field_sans();
    let widest = options
        .iter()
        .map(|option| {
            ui.painter()
                .layout_no_wrap(
                    option.as_ref().to_owned(),
                    font.clone(),
                    egui::Color32::PLACEHOLDER,
                )
                .size()
                .x
        })
        .fold(0.0_f32, f32::max);
    (widest + 8.0 + 24.0).ceil()
}

/// A problem stated on its own line inside a dialog body: the mark that owns
/// the severity, then neutral text.
///
/// Dialogs that paint notes in a column of their own share this one painter,
/// so a warning in Publish and a warning in Live session are the same mark at
/// the same size with the same gap behind it.
pub fn problem_note(ui: &mut Ui, problem: Problem, text: &str) -> Response {
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

pub fn field_label<R>(ui: &mut Ui, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(5.0);
    content(ui)
}

pub fn input_field(
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
pub fn text_edit_field(
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

pub fn read_only_field(ui: &mut Ui, label: &str, value: &str, description: &str) -> Response {
    let mut text = value.to_owned();
    field_label(ui, label, |ui| {
        let t = Tokens::get(ui.ctx());
        let response = ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            TextEdit::singleline(&mut text)
                .font(theme::field_mono())
                .interactive(false)
                .margin(egui::Margin::symmetric(8, 4)),
        );
        configure_field_validation(ui, &response, label, None, description);
        response
    })
}

pub fn configure_field_validation(
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
