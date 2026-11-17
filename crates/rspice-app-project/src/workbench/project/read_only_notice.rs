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
        let mut pressed = false;
        Panel::top("project.read_only_notice")
            .exact_size(layout.document_bar_height)
            .frame(Frame::new().fill(t.color.bg_panel_2))
            .show_separator_line(false)
            .show(root, |ui| {
                let rect = ui.max_rect();
                ui.painter().hline(
                    rect.x_range(),
                    (rect.bottom() - 0.5).max(rect.top()),
                    egui::Stroke::new(1.0, t.color.border),
                );
                pressed = notice_row(ui, &sentence, action);
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

/// The mark, the sentence (elided to the room the button leaves, whole on
/// hover and to a screen reader) and the action, on one line.
fn notice_row(ui: &mut Ui, sentence: &str, action: ReadOnlyAction) -> bool {
    let t = Tokens::get(ui.ctx());
    let inset = panel_inset(ui);
    let rect = ui.max_rect().shrink2(vec2(inset, 0.0));
    let button = Button::new(action.label()).compact();
    let button_width = button.measured_width(ui);
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let mark_side = PROBLEM_MARK_SIDE;
    let text_left = rect.left() + mark_side + 8.0;
    let text_width = (rect.right() - button_width - NOTICE_GAP - text_left).max(24.0);
    let shown = elide_text(ui, sentence, &font, text_width);
    let galley = ui.painter().layout_no_wrap(shown, font, t.color.text);
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
