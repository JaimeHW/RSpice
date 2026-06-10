//! Save-changes confirmation — the template instance of the VOLTA modal
//! grammar (`design/volta-dialogs.html` §02): small surface, one sentence,
//! three explicit verbs. Ghost = Cancel, secondary = Don't save,
//! primary = Save.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use super::{ConfirmationAction, ConfirmationResponse, RSpiceApp};

impl RSpiceApp {
    pub(super) fn render_confirmation_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.confirmation_dialog.visible {
            return;
        }

        let pending_action = self.state.dialogs.confirmation_dialog.pending_action;
        let title = pending_action
            .as_ref()
            .map(ConfirmationAction::dialog_title)
            .unwrap_or("Save changes?");
        let prompt = pending_action
            .as_ref()
            .map(ConfirmationAction::prompt_message)
            .unwrap_or(
                "The current schematic has unsaved changes.\nDo you want to save before continuing?",
            );

        let choice = Dialog::new("Schematic", title, "Save")
            .size(DialogSize::Sm)
            .secondary("Don't save")
            .ghost("Cancel")
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                for line in prompt.lines() {
                    ui.label(
                        egui::RichText::new(line)
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                }
            });

        let response = match choice {
            DialogChoice::Primary => Some(ConfirmationResponse::Yes),
            DialogChoice::Secondary => Some(ConfirmationResponse::No),
            DialogChoice::Ghost | DialogChoice::Cancelled => Some(ConfirmationResponse::Cancel),
            DialogChoice::None => None,
        };
        if let Some(r) = response {
            self.handle_confirmation_response(r);
        }
    }
}
