//! Inline waveform expression draft, layout and editing gestures.
use crate::session::ExprEditor;
use egui::Ui;
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};

/// Editing intent; evaluation and document changes are applied by the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    None,
    Commit,
    Cancel,
}

const EXPR_EDITOR_PADDING_X: f32 = 10.0;
const EXPR_EDITOR_PADDING_Y: f32 = 5.0;
const EXPR_EDITOR_GAP: f32 = 8.0;
const EXPR_EDITOR_COMPACT_WIDTH: f32 = 560.0;
const EXPR_EDITOR_MIN_INLINE_INPUT: f32 = 160.0;
const EXPR_EDITOR_ERROR_HEIGHT: f32 = 20.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct EditorSpan {
    start: f32,
    width: f32,
}

impl EditorSpan {
    fn end(self) -> f32 {
        self.start + self.width
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct ExprEditorLayout {
    label: EditorSpan,
    input: EditorSpan,
    error: EditorSpan,
    add: EditorSpan,
    stack_error: bool,
}

fn expr_editor_layout(
    available_width: f32,
    label_natural_width: f32,
    add_natural_width: f32,
    error_natural_width: Option<f32>,
) -> ExprEditorLayout {
    let available_width = available_width.max(0.0);
    // Reserve the commit action from the right edge before allocating any
    // free-form text. This is the invariant the former fixed 340 px input
    // violated on phone-sized strips.
    let add_width = add_natural_width.max(0.0).min(available_width);
    let add = EditorSpan {
        start: available_width - add_width,
        width: add_width,
    };
    let before_add = if add_width > 0.0 {
        (add.start - EXPR_EDITOR_GAP).max(0.0)
    } else {
        available_width
    };
    let label_width = label_natural_width.max(0.0).min(before_add);
    let label = EditorSpan {
        start: 0.0,
        width: label_width,
    };
    let input_start = if label_width > 0.0 {
        (label.end() + EXPR_EDITOR_GAP).min(before_add)
    } else {
        0.0
    };

    let stack_error = error_natural_width.is_some()
        && (available_width <= EXPR_EDITOR_COMPACT_WIDTH
            || before_add - input_start < EXPR_EDITOR_MIN_INLINE_INPUT + 88.0);
    let mut input_end = before_add;
    let mut error = EditorSpan::default();
    if let Some(error_natural_width) = error_natural_width.filter(|_| !stack_error) {
        let error_budget =
            (before_add - input_start - EXPR_EDITOR_MIN_INLINE_INPUT - EXPR_EDITOR_GAP).max(0.0);
        let error_width = error_natural_width
            .max(0.0)
            .min(available_width * 0.28)
            .min(error_budget);
        if error_width > 0.0 {
            error = EditorSpan {
                start: before_add - error_width,
                width: error_width,
            };
            input_end = (error.start - EXPR_EDITOR_GAP).max(input_start);
        }
    }

    ExprEditorLayout {
        label,
        input: EditorSpan {
            start: input_start,
            width: (input_end - input_start).max(0.0),
        },
        error,
        add,
        stack_error,
    }
}

/// Paint an editor already bound to the host-selected analysis.
/// Scope describes the exact source selection that the host will evaluate.
pub fn show(ui: &mut Ui, editor: &mut ExprEditor, scope_note: Option<&str>) -> Action {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let mut action = Action::None;

    let label_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let input_font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let error_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let label_width = ui
        .painter()
        .layout_no_wrap("expr".to_owned(), label_font.clone(), c.text_dim)
        .size()
        .x;
    let add_width = ui
        .painter()
        .layout_no_wrap(
            "Add".to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text,
        )
        .size()
        .x
        + 20.0;
    let error_width = editor.error.as_ref().map(|error| {
        ui.painter()
            .layout_no_wrap(error.clone(), error_font.clone(), c.err)
            .size()
            .x
    });
    let inner_width = (ui.available_width() - 2.0 * EXPR_EDITOR_PADDING_X).max(0.0);
    let layout = expr_editor_layout(inner_width, label_width, add_width, error_width);
    let control_row_height = t.metrics.ctl_h + 2.0 * EXPR_EDITOR_PADDING_Y;
    let total_height = control_row_height
        + if layout.stack_error {
            EXPR_EDITOR_ERROR_HEIGHT
        } else {
            0.0
        };
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), total_height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let control_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.left() + EXPR_EDITOR_PADDING_X,
            rect.top() + EXPR_EDITOR_PADDING_Y,
        ),
        egui::vec2(inner_width, t.metrics.ctl_h),
    );
    let span_rect = |span: EditorSpan| {
        egui::Rect::from_min_max(
            egui::pos2(control_rect.left() + span.start, control_rect.top()),
            egui::pos2(control_rect.left() + span.end(), control_rect.bottom()),
        )
    };
    let label_rect = span_rect(layout.label);
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        "expr",
        label_font,
        c.text_dim,
    );

    let input_rect = span_rect(layout.input);
    let response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(input_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(input_rect);
                ui.add_sized(
                    input_rect.size(),
                    egui::TextEdit::singleline(&mut editor.text)
                        .font(input_font)
                        .hint_text("V(out)/V(in) - dB(V(out)) - deriv(V(out))")
                        .desired_width(input_rect.width()),
                )
            },
        )
        .inner;
    if editor.want_focus {
        response.request_focus();
        editor.want_focus = false;
    }
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        action = Action::Commit;
    }
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        action = Action::Cancel;
    }
    let add_rect = span_rect(layout.add);
    let add_clicked = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(add_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(add_rect);
                rspice_ui_kit::widgets::Button::new("Add")
                    .min_width(add_rect.width())
                    .max_width(add_rect.width())
                    .show(ui)
            },
        )
        .inner
        .clicked();
    if add_clicked {
        action = Action::Commit;
    }
    if let Some(error) = &editor.error {
        let error_rect = if layout.stack_error {
            egui::Rect::from_min_max(
                egui::pos2(
                    control_rect.left() + layout.input.start,
                    rect.top() + control_row_height,
                ),
                egui::pos2(control_rect.right(), rect.bottom()),
            )
        } else {
            span_rect(layout.error)
        };
        if error_rect.width() > 0.0 {
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(error_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    ui.set_clip_rect(error_rect);
                    ui.add_sized(
                        error_rect.size(),
                        egui::Label::new(egui::RichText::new(error).font(error_font).color(c.err))
                            .truncate(),
                    )
                    .on_hover_text(error);
                },
            );
        }
    }

    if let Some(note) = scope_note {
        ui.add(
            egui::Label::new(
                egui::RichText::new(note)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_dim),
            )
            .wrap(),
        );
    }

    action
}

#[cfg(test)]
mod tests;
