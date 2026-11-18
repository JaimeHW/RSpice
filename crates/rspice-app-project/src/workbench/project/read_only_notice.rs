//! The read-only notice: why this window opened the project read-only, and
//! the one action that changes it. It hangs under the toolbar in every
//! workspace for as long as it holds, as the title of a read-only cellview
//! says so wherever the reader is.

use egui::{Frame, Panel, Sense, Ui, vec2};

use crate::state::write_policy::OpenedReadOnly;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::ProjectFileAction;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use rspice_ui_kit::panels::{elide_text, panel_inset};

/// The one action the notice offers for its reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadOnlyAction {
    /// Another window holds the project: take it for writing all the same.
    OpenAnyway,
    /// The file cannot be written here: keep the project somewhere it can.
    SaveAs,
}

impl ReadOnlyAction {
    pub const fn for_reason(reason: &OpenedReadOnly) -> Self {
        if reason.can_open_anyway() {
            Self::OpenAnyway
        } else {
            Self::SaveAs
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenAnyway => "Open anyway",
            Self::SaveAs => "Save as\u{2026}",
        }
    }
}

/// The text side of the notice and the action button are kept apart by this.
const NOTICE_GAP: f32 = 12.0;

impl ProjectFeature {
    /// Draw the notice while the open project was opened read-only for a
    /// reason this window can act on. Returns whether its action was
    /// pressed, so the shell answers what it raised.
    pub fn show_read_only_notice(cx: &mut Cx<'_, Self>, root: &mut Ui, layout: LayoutSpec) -> bool {
        if !cx.platform.project_lifecycle.is_open() {
            return false;
        }
        let Some(reason) = cx.model.safe_mode.opened_read_only.as_ref() else {
            return false;
        };
        let sentence = reason.sentence();
        let action = ReadOnlyAction::for_reason(reason);
        let t = Tokens::get(root.ctx());
        // Under a finger there is no hover to read a cut sentence: it wraps,
        // and the notice is as tall as its lines.
        let wrap = t.metrics.is_touch();
        let height = if wrap {
            let across = root.available_width();
            let width = notice_text_width(root, across, action);
            let lines = root
                .painter()
                .layout(sentence.clone(), notice_font(), t.color.text, width)
                .size()
                .y;
            layout
                .document_bar_height
                .max(lines + 2.0 * t.metrics.bar_pad)
        } else {
            layout.document_bar_height
        };
        let mut pressed = false;
        Panel::top("project.read_only_notice")
            .exact_size(height)
            .frame(Frame::new().fill(t.color.bg_panel_2))
            .show_separator_line(false)
            .show(root, |ui| {
                let rect = ui.max_rect();
                ui.painter().hline(
                    rect.x_range(),
                    (rect.bottom() - 0.5).max(rect.top()),
                    egui::Stroke::new(1.0, t.color.border),
                );
                pressed = notice_row(ui, &sentence, action, wrap);
            });
        if pressed {
            match action {
                ReadOnlyAction::OpenAnyway => cx.wb.outbox.push(ProjectRequest::OpenAnyway),
                ReadOnlyAction::SaveAs => cx.wb.outbox.push(ProjectFileAction::SaveAs),
            }
        }
        pressed
    }
}

fn notice_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::Regular)
}

/// The room the sentence has across a notice `width` wide: what the inset,
/// the mark and the button leave.
fn notice_text_width(ui: &mut Ui, width: f32, action: ReadOnlyAction) -> f32 {
    let button_width = Button::new(action.label()).compact().measured_width(ui);
    (width - 2.0 * panel_inset(ui) - PROBLEM_MARK_SIDE - 8.0 - button_width - NOTICE_GAP)
        .max(24.0)
}

/// The mark, the sentence and the action. Under a pointer the sentence is
/// one line, elided to the room the button leaves and whole on hover; under
/// a finger it wraps. A screen reader has it whole either way.
fn notice_row(ui: &mut Ui, sentence: &str, action: ReadOnlyAction, wrap: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let inset = panel_inset(ui);
    let rect = ui.max_rect().shrink2(vec2(inset, 0.0));
    let button = Button::new(action.label()).compact();
    let button_width = button.measured_width(ui);
    let font = notice_font();
    let mark_side = PROBLEM_MARK_SIDE;
    let text_left = rect.left() + mark_side + 8.0;
    let across = ui.max_rect().width();
    let text_width = notice_text_width(ui, across, action);
    let galley = if wrap {
        ui.painter()
            .layout(sentence.to_owned(), font, t.color.text, text_width)
    } else {
        let shown = elide_text(ui, sentence, &font, text_width);
        ui.painter().layout_no_wrap(shown, font, t.color.text)
    };
    let text_rect = egui::Rect::from_min_size(
        egui::pos2(text_left, rect.center().y - galley.size().y * 0.5),
        galley.size(),
    );
    paint_problem_mark(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + mark_side * 0.5, rect.center().y),
            egui::Vec2::splat(mark_side),
        ),
        Problem::Warning,
    );
    ui.painter().galley(text_rect.min, galley, t.color.text);
    let text = ui.interact(
        text_rect.union(egui::Rect::from_center_size(
            egui::pos2(rect.left() + mark_side * 0.5, rect.center().y),
            egui::Vec2::splat(mark_side),
        )),
        ui.id().with("project.read_only_notice.text"),
        Sense::hover(),
    );
    text.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, sentence));
    text.on_hover_text(sentence);
    let button_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - button_width, rect.top()),
        vec2(button_width, rect.height()),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
    );
    button.show(&mut child).clicked()
}
