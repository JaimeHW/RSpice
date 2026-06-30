//! License activation — the egui implementation of
//! the product dialog pattern.
//!
//! Three phases on one VOLTA modal: Entry (textarea + field note),
//! Error (same pane, note in `err`), Verified (summary card with the
//! grant + feature chips + offline-storage note). Verification is fully
//! offline; Activate stores the key locally and nothing phones home.

use egui::Context;

use crate::services::license::{self, LicenseInfo};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

use super::{ConsoleMessage, LicensePhase, RSpiceApp};

impl RSpiceApp {
    /// Open the dialog: fresh entry, or the stored grant when already
    /// activated (primary becomes Close, secondary removes the license).
    pub(crate) fn open_license_dialog(&mut self) {
        let dialog = &mut self.state.dialogs.license_dialog;
        dialog.open = true;
        dialog.text.clear();
        dialog.phase = match &self.state.license {
            Some(info) => LicensePhase::Verified(info.clone()),
            None => LicensePhase::Entry,
        };
    }

    pub(super) fn render_license_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.license_dialog.open {
            return;
        }
        let already_active = self.state.license.is_some()
            && matches!(
                self.state.dialogs.license_dialog.phase,
                LicensePhase::Verified(_)
            )
            && self.state.dialogs.license_dialog.text.is_empty();

        let phase = self.state.dialogs.license_dialog.phase.clone();
        let (primary, secondary, hint): (&str, &str, String) = match &phase {
            LicensePhase::Entry | LicensePhase::Error(_) => (
                "Verify",
                "Paste sample key",
                "signature: ed25519 · key id 01".to_owned(),
            ),
            LicensePhase::Verified(_) if already_active => (
                "Close",
                "Remove license",
                "verified · ed25519 key id 01 · denylist clear".to_owned(),
            ),
            LicensePhase::Verified(_) => (
                "Activate",
                "Paste sample key",
                "verified · ed25519 key id 01 · denylist clear".to_owned(),
            ),
        };
        let primary_enabled = match &phase {
            LicensePhase::Entry => !self.state.dialogs.license_dialog.text.trim().is_empty(),
            _ => true,
        };

        // Ctrl+Enter verifies; bare Enter belongs to the textarea (the
        // field note explicitly invites line breaks).
        let ctrl_enter = ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter));

        let state = &mut self.state;
        let choice = Dialog::new("License", "Enter license key", primary)
            .size(DialogSize::Md)
            .secondary(secondary)
            .ghost("Cancel")
            .hint(&hint)
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                let subtext = match &state.dialogs.license_dialog.phase {
                    LicensePhase::Verified(_) => {
                        "Key verified on this machine. Activating stores it locally; nothing is sent anywhere."
                    }
                    _ => {
                        "Paste the key from your purchase email. Verification happens on this machine — RSpice never phones home."
                    }
                };
                ui.label(
                    egui::RichText::new(subtext)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                );
                ui.add_space(tokens::SP_2);

                match state.dialogs.license_dialog.phase.clone() {
                    LicensePhase::Entry | LicensePhase::Error(_) => {
                        ui.add(
                            egui::TextEdit::multiline(&mut state.dialogs.license_dialog.text)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .desired_rows(3)
                                .desired_width(f32::INFINITY)
                                .hint_text("RSPICE-K1.XXXXX-XXXXX-…"),
                        );
                        ui.add_space(4.0);
                        let (note, color) = match &state.dialogs.license_dialog.phase {
                            LicensePhase::Error(message) => (message.as_str(), c.err),
                            _ => (
                                "Line breaks and grouping dashes are fine — the whole string from the email works.",
                                c.text_faint,
                            ),
                        };
                        ui.label(
                            egui::RichText::new(note)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(color),
                        );
                    }
                    LicensePhase::Verified(info) => {
                        summary_card(ui, &info);
                        ui.add_space(tokens::SP_2);
                        ui.label(
                            egui::RichText::new(
                                "key will be stored at %APPDATA%\\rspice\\license.key — yours to keep, works offline",
                            )
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                        );
                    }
                }
            });

        let choice = if choice == DialogChoice::None && ctrl_enter && primary_enabled {
            DialogChoice::Primary
        } else {
            choice
        };

        match choice {
            DialogChoice::None => {}
            DialogChoice::Cancelled | DialogChoice::Ghost => {
                self.state.dialogs.license_dialog.open = false;
            }
            DialogChoice::Secondary => match &self.state.dialogs.license_dialog.phase {
                LicensePhase::Verified(_) if already_active => self.remove_license(),
                _ => {
                    // Paste sample key — fills the field and verifies, the
                    // same walkthrough the design specifies.
                    self.state.dialogs.license_dialog.text = license::SAMPLE_KEY.to_owned();
                    self.verify_license_input();
                }
            },
            DialogChoice::Primary => match self.state.dialogs.license_dialog.phase.clone() {
                LicensePhase::Entry | LicensePhase::Error(_) => self.verify_license_input(),
                LicensePhase::Verified(_) if already_active => {
                    self.state.dialogs.license_dialog.open = false;
                }
                LicensePhase::Verified(info) => self.activate_license(ctx, info),
            },
        }
    }

    fn verify_license_input(&mut self) {
        let text = self.state.dialogs.license_dialog.text.clone();
        self.state.dialogs.license_dialog.phase = match license::parse_and_verify(&text) {
            Ok(info) => LicensePhase::Verified(info),
            Err(error) => LicensePhase::Error(error.message().to_owned()),
        };
    }

    fn activate_license(&mut self, ctx: &Context, info: LicenseInfo) {
        let key = self.state.dialogs.license_dialog.text.trim().to_owned();
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(error) = license::store_key(&key) {
            self.state
                .push_user_message(ConsoleMessage::warning(format!(
                    "License active for this session, but storing it failed: {error}"
                )));
        }
        self.state.license_key = Some(key);
        let message = format!(
            "License activated — {} until {}",
            info.tier, info.updates_until
        );
        self.state.license = Some(info);
        self.state.dialogs.license_dialog.open = false;
        self.state.shell.toasts.info(ctx, &message);
        self.state.push_user_message(ConsoleMessage::info(message));
    }

    fn remove_license(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(path) = license::license_file_path() {
            let _ = std::fs::remove_file(path);
        }
        self.state.license_key = None;
        self.state.license = None;
        self.state.dialogs.license_dialog.open = false;
        self.state
            .push_user_message(ConsoleMessage::info("License removed from this machine"));
    }
}

/// The verified summary card: grant rows + feature chips.
fn summary_card(ui: &mut egui::Ui, info: &LicenseInfo) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let frame = egui::Frame::none()
        .fill(c.bg_elevated)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius)
        .inner_margin(egui::Margin::symmetric(12.0, 10.0));
    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());
        row(ui, "Licensed to", &info.licensed_to, c.text);
        row(ui, "Tier", &info.tier, c.accent);
        if info.updates_expired {
            // Perpetual fallback: the key still activates; this build just
            // postdates its updates window.
            let label = format!("{} · updates ended", info.updates_until);
            row(ui, "Updates until", &label, c.warn);
        } else {
            row(ui, "Updates until", &info.updates_until, c.text);
        }
        row(ui, "License id", &info.license_id, c.text);
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
            for feature in &info.features {
                feature_chip(ui, feature);
            }
        });
    });
}

fn row(ui: &mut egui::Ui, key: &str, value: &str, value_color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        let (label_rect, _) = ui.allocate_exact_size(egui::vec2(110.0, 20.0), egui::Sense::hover());
        ui.painter().text(
            egui::pos2(label_rect.left(), label_rect.center().y),
            egui::Align2::LEFT_CENTER,
            key,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        );
        ui.label(
            egui::RichText::new(value)
                .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                .color(value_color),
        );
    });
}

fn feature_chip(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::none()
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border_strong))
        .rounding(t.radius)
        .inner_margin(egui::Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(label)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_dim),
            );
        });
}
