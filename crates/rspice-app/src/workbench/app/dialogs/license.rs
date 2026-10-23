//! License key: enter, verify and activate a key, or read and remove the one
//! this machine holds.
//!
//! Three phases on one modal: entry (the key field and the line under it),
//! the same with the refusal in that line, and the verified grant on the
//! label/value grid. Verification is offline; Activate stores the key on this
//! machine, or in this browser, and nothing is sent anywhere.

use egui::Context;

use super::help::fact_grid;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::services::license::{self, LicenseInfo};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};

use crate::workbench::app::RSpiceApp;

/// Where the license-activation flow stands.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LicensePhase {
    /// Waiting for a key.
    #[default]
    Entry,
    /// Last verification failed; the message renders under the field.
    Error(String),
    /// Key verified — summary pane with the grant, primary = Activate.
    Verified(crate::services::license::LicenseInfo),
}

/// License activation dialog state.
#[derive(Debug, Clone, Default)]
pub struct LicenseDialogState {
    /// Whether the dialog is showing.
    pub open: bool,
    /// The pasted key text.
    pub text: String,
    /// Current phase of the flow.
    pub phase: LicensePhase,
}

/// Lines the key field shows: a whole key, wrapped, without scrolling.
const KEY_LINES: usize = 3;

impl RSpiceApp {
    /// Open the dialog: fresh entry, or the stored grant when already
    /// activated (primary becomes Close, secondary removes the license).
    pub(crate) fn open_license_dialog(&mut self) {
        let dialog = &mut self.state.cloud.license_dialog;
        dialog.open = true;
        dialog.text.clear();
        dialog.phase = match &self.state.cloud.license {
            Some(info) => LicensePhase::Verified(info.clone()),
            None => LicensePhase::Entry,
        };
    }

    pub(in crate::workbench) fn render_license_dialog(&mut self, ctx: &Context) {
        if !self.state.cloud.license_dialog.open {
            return;
        }
        let already_active = self.state.cloud.license.is_some()
            && matches!(
                self.state.cloud.license_dialog.phase,
                LicensePhase::Verified(_)
            )
            && self.state.cloud.license_dialog.text.is_empty();

        let phase = self.state.cloud.license_dialog.phase.clone();
        let empty = self.state.cloud.license_dialog.text.trim().is_empty();
        let (title, primary) = match &phase {
            LicensePhase::Entry | LicensePhase::Error(_) => ("Enter license key", "Verify"),
            LicensePhase::Verified(_) if already_active => ("License", "Close"),
            LicensePhase::Verified(_) => ("Enter license key", "Activate"),
        };
        // The footer says why Verify waits, how to give it the key from the
        // keyboard, and what Activate will do; a refusal is under the field.
        let hint = match &phase {
            LicensePhase::Entry if empty => Some("Paste a license key to verify it"),
            LicensePhase::Entry => Some("Ctrl+Enter verifies the key"),
            LicensePhase::Error(_) => None,
            LicensePhase::Verified(_) if already_active => None,
            LicensePhase::Verified(_) if cfg!(target_arch = "wasm32") => {
                Some("Activate keeps the key in this browser; it works offline")
            }
            LicensePhase::Verified(_) => {
                Some("Activate keeps the key on this machine; it works offline")
            }
        };
        let primary_enabled = !matches!(phase, LicensePhase::Entry) || !empty;
        let entering = matches!(phase, LicensePhase::Entry | LicensePhase::Error(_));

        // Ctrl+Enter verifies; bare Enter belongs to the key field, where a
        // line break pasted from an email is part of the key.
        let ctrl_enter = ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Enter));

        let state = &mut self.state;
        let mut dialog = Dialog::new(title, primary)
            .description(
                "Verify a license key on this machine, then activate it; nothing is sent anywhere.",
            )
            .size(DialogSize::Transaction)
            // Scroll-free: a reserved gutter would end the key field short
            // of the right edge the title and footer share.
            .overlay_scrollbar()
            .ghost("Cancel")
            .primary_enabled(primary_enabled)
            .primary_on_enter(!entering);
        if let Some(hint) = hint {
            dialog = dialog.hint(hint).hint_tone(DialogHintTone::Neutral);
        }
        if already_active {
            dialog = dialog.secondary("Remove license");
        }
        let mut edited = false;
        let choice = dialog.show(ctx, |ui| {
            fact_grid::show(ui, |rows| match &phase {
                LicensePhase::Entry | LicensePhase::Error(_) => {
                    let text = &mut state.cloud.license_dialog.text;
                    rows.block("Key", |ui, width| {
                        edited = ui
                            .add(
                                egui::TextEdit::multiline(text)
                                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                    .desired_rows(KEY_LINES)
                                    .desired_width(width)
                                    .hint_text("RSPICE-K1.\u{2026}"),
                            )
                            .changed();
                    });
                    match &phase {
                        LicensePhase::Error(message) => {
                            rows.note_toned(message, Tokens::get(ctx).color.err);
                        }
                        _ => rows.note(
                            "Paste the whole key from your purchase email; line breaks are fine.",
                        ),
                    }
                }
                LicensePhase::Verified(info) => grant_rows(rows, info),
            });
        });
        // A key edited after a refusal is a new key: the refusal was about
        // the old one.
        if edited && matches!(phase, LicensePhase::Error(_)) {
            self.state.cloud.license_dialog.phase = LicensePhase::Entry;
        }

        let choice = if choice == DialogChoice::None && ctrl_enter && primary_enabled {
            DialogChoice::Primary
        } else {
            choice
        };

        match choice {
            DialogChoice::None => {}
            DialogChoice::Cancelled | DialogChoice::Ghost => {
                self.state.cloud.license_dialog.open = false;
            }
            DialogChoice::Secondary if already_active => self.remove_license(),
            DialogChoice::Secondary => {}
            DialogChoice::Primary => match self.state.cloud.license_dialog.phase.clone() {
                LicensePhase::Entry | LicensePhase::Error(_) => self.verify_license_input(),
                LicensePhase::Verified(_) if already_active => {
                    self.state.cloud.license_dialog.open = false;
                }
                LicensePhase::Verified(info) => self.activate_license(info),
            },
        }
    }

    fn verify_license_input(&mut self) {
        let text = self.state.cloud.license_dialog.text.clone();
        self.state.cloud.license_dialog.phase = match license::parse_and_verify(&text) {
            Ok(info) => LicensePhase::Verified(info),
            Err(error) => LicensePhase::Error(error.message().to_owned()),
        };
    }

    fn activate_license(&mut self, info: LicenseInfo) {
        let key = self.state.cloud.license_dialog.text.trim().to_owned();
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(error) = license::store_key(&key) {
            self.state.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("The license is active for this session but could not be saved: {error}"),
            );
        }
        self.state.cloud.license_key = Some(key);
        let message = format!(
            "License activated: {} updates are available until {}.",
            info.tier, info.updates_until
        );
        self.state.cloud.license = Some(info);
        self.state.cloud.license_dialog.open = false;
        self.state
            .record(LogSeverity::Info, LogSource::Studio, message);
    }

    fn remove_license(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(path) = license::license_file_path() {
            let _ = std::fs::remove_file(path);
        }
        self.state.cloud.license_key = None;
        self.state.cloud.license = None;
        self.state.cloud.license_dialog.open = false;
        self.state.record(
            LogSeverity::Info,
            LogSource::Studio,
            "License removed from this machine.",
        );
    }
}

/// The grant a verified key carries, and where the key is kept.
fn grant_rows(rows: &mut fact_grid::Rows<'_>, info: &LicenseInfo) {
    rows.text("Licensed to", &info.licensed_to);
    rows.text("Tier", &info.tier);
    if info.updates_expired {
        // Perpetual fallback: the key still activates; this build just
        // postdates its updates window.
        rows.text("Updates", &format!("Ended {}", info.updates_until));
    } else {
        rows.text("Updates", &format!("Until {}", info.updates_until));
    }
    rows.mono("License id", &info.license_id);
    for (index, feature) in info.features.iter().enumerate() {
        rows.text(if index == 0 { "Features" } else { "" }, feature);
    }
    rows.mono("Stored in", &key_storage());
}

/// Where Activate keeps the key on this platform: the file it writes on the
/// desktop, this browser on the web.
fn key_storage() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        "This browser".to_owned()
    }
    #[cfg(not(target_arch = "wasm32"))]
    license::license_file_path().map_or_else(
        || "Nowhere: this system has no configuration folder".to_owned(),
        |path| path.display().to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every text the dialog published for a reader to hear. Generic over the
    /// app so that a test helper is not counted as a handler that takes the
    /// whole application.
    fn shown_texts<App: LicenseDialogHost>(app: &mut App) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut update = None;
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| app.render(ui.ctx()),
            );
            update = output.platform_output.accesskit_update;
        }
        update
            .expect("the frame published its accessibility tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| node.value().or_else(|| node.label()))
            .map(str::to_owned)
            .collect()
    }

    /// What draws the license dialog in a frame.
    trait LicenseDialogHost {
        fn render(&mut self, ctx: &egui::Context);
    }

    impl LicenseDialogHost for RSpiceApp {
        fn render(&mut self, ctx: &egui::Context) {
            self.render_license_dialog(ctx);
        }
    }

    /// A verified key says where Activate will keep it on this platform:
    /// the file it writes, not a Windows variable on every system.
    #[test]
    fn a_verified_key_names_where_this_platform_keeps_it() {
        let mut app = RSpiceApp::test_instance();
        app.open_license_dialog();
        app.state.cloud.license_dialog.text = license::SAMPLE_KEY.to_owned();
        app.verify_license_input();
        let shown = shown_texts(&mut app);
        assert!(
            !shown.iter().any(|text| text.contains("%APPDATA%")),
            "no platform's location is named by another's variable: {shown:#?}"
        );
        #[cfg(not(target_arch = "wasm32"))]
        {
            let file = license::license_file_path()
                .expect("the test host has a configuration folder")
                .display()
                .to_string();
            assert!(
                shown.iter().any(|text| text.contains(&file)),
                "the dialog names {file}: {shown:#?}"
            );
        }
    }

    /// The dialog a buyer pastes their key into offers no sample key to
    /// paste instead, and says nothing in signature jargon or with an em
    /// dash, while it waits for a key, refuses one, or shows its grant.
    #[test]
    fn the_license_dialog_offers_no_sample_key_and_speaks_plainly() {
        let mut app = RSpiceApp::test_instance();
        app.open_license_dialog();
        let mut seen = shown_texts(&mut app);
        app.state.cloud.license_dialog.text = "RSPICE-K1.TRUNCATED".to_owned();
        app.verify_license_input();
        seen.extend(shown_texts(&mut app));
        app.state.cloud.license_dialog.text = license::SAMPLE_KEY.to_owned();
        app.verify_license_input();
        seen.extend(shown_texts(&mut app));
        assert!(
            seen.iter().any(|text| text == "Verify") && seen.iter().any(|text| text == "Activate"),
            "the three phases were drawn: {seen:#?}"
        );
        for text in &seen {
            assert!(!text.contains("sample"), "{text:?} offers a sample key");
            assert!(
                !text.contains("ed25519"),
                "{text:?} names the signature scheme"
            );
            assert!(!text.contains('\u{2014}'), "{text:?} has an em dash");
        }
    }
}
