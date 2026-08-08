//! Unpublish — the owner-confirmed publication tombstone.
//!
//! States the exact outcomes before acting: the page starts answering 410
//! for everyone immediately, version history stays privately retained on the
//! account, and copies already fetched are not recalled. The primary stays
//! disabled until the author acknowledges that last fact.

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;

/// Unpublish confirmation state, armed with one exact publication target.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct UnpublishWebDialogState {
    pub open: bool,
    pub acknowledged: bool,
    pub circuit_id: String,
    pub publication_id: String,
    pub url_path: String,
    /// Versions recorded for the circuit when the dialog opened.
    pub version_count: usize,
}

impl RSpiceApp {
    pub(crate) fn open_unpublish_web_dialog(
        &mut self,
        circuit_id: String,
        publication_id: String,
        url_path: String,
        version_count: usize,
    ) {
        self.state.dialogs.unpublish_web = UnpublishWebDialogState {
            open: true,
            acknowledged: false,
            circuit_id,
            publication_id,
            url_path,
            version_count,
        };
    }

    pub(in crate::workbench) fn render_unpublish_web_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.unpublish_web.open {
            return;
        }
        let title = format!("Unpublish {}", self.state.dialogs.unpublish_web.url_path);
        let acknowledged = self.state.dialogs.unpublish_web.acknowledged;
        let version_count = self.state.dialogs.unpublish_web.version_count;

        let dialog_state = &mut self.state.dialogs.unpublish_web;
        let choice = Dialog::new("Publication", &title, "Unpublish")
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

                outcome_row(ui, "Page and raw endpoints", "410 tombstone", c.text);
                outcome_row(
                    ui,
                    &if version_count > 0 {
                        format!("Version history v1–v{version_count}")
                    } else {
                        "Version history".to_owned()
                    },
                    "retained privately in your account",
                    c.text,
                );
                outcome_row(
                    ui,
                    "Copies of downloaded data",
                    "not recalled — anything fetched stays fetched",
                    c.warn,
                );
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
                self.state.dialogs.unpublish_web.open = false;
            }
            DialogChoice::Primary => {
                let dialog = &self.state.dialogs.unpublish_web;
                if dialog.acknowledged {
                    let circuit_id = dialog.circuit_id.clone();
                    let publication_id = dialog.publication_id.clone();
                    let url_path = dialog.url_path.clone();
                    self.cloud_account.unpublish(circuit_id, publication_id);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Unpublish requested — {url_path} will answer 410 once the service \
                         confirms."
                    )));
                    self.state.dialogs.unpublish_web.open = false;
                }
            }
        }
    }
}

fn outcome_row(ui: &mut egui::Ui, what: &str, outcome: &str, outcome_color: egui::Color32) {
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
                    .color(outcome_color),
            );
        });
    });
}
