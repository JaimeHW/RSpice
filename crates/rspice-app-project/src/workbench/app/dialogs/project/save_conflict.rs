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
pub const SAVE_CONFLICT_HINT: &str = "Save over it replaces what the other window saved. Open the \
     saved project gives up the changes here. Esc writes nothing.";

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
            ui.add_space(6.0);
            for difference in &conflict.differences {
                ui.label(
                    egui::RichText::new(format!(
                        "{}  {}",
                        difference.side.label(),
                        difference.document.label()
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
                );
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
