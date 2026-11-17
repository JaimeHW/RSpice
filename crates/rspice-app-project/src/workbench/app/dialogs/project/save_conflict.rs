//! The answer to a save that met its file changed since this window opened
//! it: what differs, document by document and on which side, and what each
//! choice keeps.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::lifecycle::project_lifecycle::SaveConflict;

/// What the reader chose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveConflictAnswer {
    /// Keep this window's project as a new project, with a history of its
    /// own; the saved file stays as the other window left it.
    SaveCopy,
    /// Replace the saved file with this window's project.
    SaveOver,
    /// Open the saved file in place of this window's project.
    OpenSaved,
    /// Write nothing; the changes here stay unsaved.
    Keep,
}

/// What the dialog's hint says each answer does to whose work.
pub const SAVE_CONFLICT_HINT: &str =
    "Save over it replaces the other window's save. Open the saved project drops the changes here.";

/// Ask what to do with a save that met its file changed. Returns the answer
/// once the reader gives one.
pub fn save_conflict_dialog(conflict: &SaveConflict, ctx: &Context) -> Option<SaveConflictAnswer> {
    let sentence = conflict.sentence();
    let choice = Dialog::new("The saved project changed", "Save a copy\u{2026}")
        .description(&sentence)
        .size(DialogSize::Transaction)
        .secondary("Save over it")
        .ghost("Open the saved project")
        .hint(SAVE_CONFLICT_HINT)
        .show(ctx, |ui| {
            let c = Tokens::get(ui.ctx()).color;
            ui.label(
                egui::RichText::new(&sentence)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text),
            );
            ui.add_space(tokens::SP_4);
            for difference in &conflict.differences {
                difference_row(ui, &difference.document.label(), difference.side.label());
            }
        });
    match choice {
        DialogChoice::None => None,
        DialogChoice::Primary => Some(SaveConflictAnswer::SaveCopy),
        DialogChoice::Secondary => Some(SaveConflictAnswer::SaveOver),
        DialogChoice::Ghost => Some(SaveConflictAnswer::OpenSaved),
        DialogChoice::Cancelled => Some(SaveConflictAnswer::Keep),
    }
}

/// One document that differs: its name, and on which side it changed, set
/// apart at the row's end as a table's last column is.
fn difference_row(ui: &mut egui::Ui, document: &str, side: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), t.metrics.row_h),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, true, format!("{document}: {side}"))
    });
    let side_width = ui
        .painter()
        .layout_no_wrap(side.to_owned(), font.clone(), c.text_dim)
        .size()
        .x;
    let shown = rspice_ui_kit::panels::elide_text(
        ui,
        document,
        &font,
        rect.width() - side_width - tokens::SP_5,
    );
    ui.painter()
        .hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, c.border));
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        shown,
        font.clone(),
        c.text,
    );
    ui.painter().text(
        egui::pos2(rect.right(), rect.center().y),
        egui::Align2::RIGHT_CENTER,
        side,
        font,
        c.text_dim,
    );
}
