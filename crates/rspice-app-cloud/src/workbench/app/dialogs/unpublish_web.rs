//! Unpublish — the owner-confirmed publication tombstone.
//!
//! States the exact outcomes before acting: the page starts answering 410
//! for everyone immediately, version history stays privately retained on the
//! account, and copies already fetched are not recalled. The primary stays
//! disabled until the author acknowledges that last fact.

use egui::Context;

use crate::services::cloud_account::CloudAccountService;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::state::WorkbenchSession;

/// Unpublish confirmation state, armed with one exact publication target.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UnpublishWebDialogState {
    pub open: bool,
    pub acknowledged: bool,
    pub circuit_id: String,
    pub publication_id: String,
    pub url_path: String,
    /// Versions recorded for the circuit when the dialog opened.
    pub version_count: usize,
}

/// What the unpublish dialog reads and writes: its own confirmation, the
/// account service that takes the page down, and the session it prompts in.
pub(crate) struct UnpublishWebParts<'a> {
    pub dialog: &'a mut UnpublishWebDialogState,
    pub account: &'a mut CloudAccountService,
    pub workbench: &'a mut WorkbenchSession,
}

/// Arm `dialog` with the one publication it takes down.
pub(crate) fn open_unpublish_web_dialog(
    dialog: &mut UnpublishWebDialogState,
    circuit_id: String,
    publication_id: String,
    url_path: String,
    version_count: usize,
) {
    *dialog = UnpublishWebDialogState {
        open: true,
        acknowledged: false,
        circuit_id,
        publication_id,
        url_path,
        version_count,
    };
}

/// Draw the unpublish dialog for this frame, when it is open, and carry out
/// what the reader chose in it.
pub(crate) fn render_unpublish_web_dialog(parts: &mut UnpublishWebParts<'_>, ctx: &Context) {
    if !parts.dialog.open {
        return;
    }
    let title = format!("Unpublish {}", parts.dialog.url_path);
    let acknowledged = parts.dialog.acknowledged;
    let version_count = parts.dialog.version_count;

    let dialog_state = &mut *parts.dialog;
    let choice = Dialog::new(&title, "Unpublish")
        .description(
            "Take the published page down for everyone and keep its history privately on \
             this account.",
        )
        .size(DialogSize::Transaction)
        .destructive()
        .ghost("Cancel")
        .hint("tombstone · the identifier is never reissued")
        .primary_enabled(acknowledged)
        .primary_on_enter(false)
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            let c = t.color;

            ui.label(
                egui::RichText::new(
                    "The URL stops resolving for everyone, immediately. Visitors see \
                     \"removed by the author\"; the identifier stays reserved and is never \
                     reissued.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(c.text),
            );
            ui.add_space(tokens::SP_4);

            outcome_row(ui, "Page and raw endpoints", "410 tombstone");
            outcome_row(
                ui,
                &if version_count > 0 {
                    format!("Version history v1–v{version_count}")
                } else {
                    "Version history".to_owned()
                },
                "kept privately in your account",
            );
            outcome_row(ui, "Copies of downloaded data", "not recalled");
            ui.add_space(tokens::SP_4);

            ui.checkbox(
                &mut dialog_state.acknowledged,
                egui::RichText::new("I understand copies already made are not recalled.")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular)),
            );
        });

    match choice {
        DialogChoice::None => {}
        DialogChoice::Cancelled | DialogChoice::Ghost | DialogChoice::Secondary => {
            parts.dialog.open = false;
        }
        DialogChoice::Primary => {
            let dialog = &parts.dialog;
            if dialog.acknowledged {
                let circuit_id = dialog.circuit_id.clone();
                let publication_id = dialog.publication_id.clone();
                let url_path = dialog.url_path.clone();
                parts.account.unpublish(circuit_id, publication_id);
                parts
                    .workbench
                    .prompt(format!("Unpublish requested for {url_path}."));
                parts.dialog.open = false;
            }
        }
    }
}

fn outcome_row(ui: &mut egui::Ui, what: &str, outcome: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(what)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.text_dim),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(outcome)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(c.text),
            );
        });
    });
}
