//! License key: enter, verify and activate a key, or read and remove the one
//! this machine holds.
//!
//! Three phases on one modal: entry (the key field and the line under it),
//! the same with the refusal in that line, and the verified grant on the
//! label/value grid. Verification is offline; Activate stores the key on this
//! machine, or in this browser, and nothing is sent anywhere.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::services::license::{self, LicenseInfo};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};

use crate::workbench::app::dialogs::fact_grid;
use crate::workbench::app_state::Model;

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

/// Open the dialog: fresh entry, or `license`, the stored grant, when already
/// activated (primary becomes Close, secondary removes the license).
pub(crate) fn open_license_dialog(dialog: &mut LicenseDialogState, license: Option<&LicenseInfo>) {
    dialog.open = true;
    dialog.text.clear();
    dialog.phase = match license {
        Some(info) => LicensePhase::Verified(info.clone()),
        None => LicensePhase::Entry,
    };
}

/// What the license dialog reads and writes: its own inputs, the key and
/// the grant this machine holds, and the model's log it reports to.
pub(crate) struct LicenseParts<'a> {
    pub dialog: &'a mut LicenseDialogState,
    pub license_key: &'a mut Option<String>,
    pub license: &'a mut Option<LicenseInfo>,
    pub model: &'a mut Model,
}

/// Draw the license dialog for this frame, when it is open, and carry out
/// what the reader chose in it.
pub(crate) fn render_license_dialog(parts: &mut LicenseParts<'_>, ctx: &Context) {
    if !parts.dialog.open {
        return;
    }
    let already_active = parts.license.is_some()
        && matches!(parts.dialog.phase, LicensePhase::Verified(_))
        && parts.dialog.text.is_empty();

    let phase = parts.dialog.phase.clone();
    let empty = parts.dialog.text.trim().is_empty();
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

    let dialog_state = &mut *parts.dialog;
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
                let text = &mut dialog_state.text;
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
        parts.dialog.phase = LicensePhase::Entry;
    }

    let choice = if choice == DialogChoice::None && ctrl_enter && primary_enabled {
        DialogChoice::Primary
    } else {
        choice
    };

    match choice {
        DialogChoice::None => {}
        DialogChoice::Cancelled | DialogChoice::Ghost => {
            parts.dialog.open = false;
        }
        DialogChoice::Secondary if already_active => remove_license(parts),
        DialogChoice::Secondary => {}
        DialogChoice::Primary => match parts.dialog.phase.clone() {
            LicensePhase::Entry | LicensePhase::Error(_) => verify_license_input(parts.dialog),
            LicensePhase::Verified(_) if already_active => {
                parts.dialog.open = false;
            }
            LicensePhase::Verified(info) => activate_license(parts, info),
        },
    }
}

fn verify_license_input(dialog: &mut LicenseDialogState) {
    let text = dialog.text.clone();
    dialog.phase = match license::parse_and_verify(&text) {
        Ok(info) => LicensePhase::Verified(info),
        Err(error) => LicensePhase::Error(error.message().to_owned()),
    };
}

fn activate_license(parts: &mut LicenseParts<'_>, info: LicenseInfo) {
    let key = parts.dialog.text.trim().to_owned();
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(error) = license::store_key(&key) {
        parts.model.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!("The license is active for this session but could not be saved: {error}"),
        );
    }
    *parts.license_key = Some(key);
    let message = format!(
        "License activated: {} updates are available until {}.",
        info.tier, info.updates_until
    );
    *parts.license = Some(info);
    parts.dialog.open = false;
    parts
        .model
        .record(LogSeverity::Info, LogSource::Studio, message);
}

fn remove_license(parts: &mut LicenseParts<'_>) {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(path) = license::license_file_path() {
        let _ = std::fs::remove_file(path);
    }
    *parts.license_key = None;
    *parts.license = None;
    parts.dialog.open = false;
    parts.model.record(
        LogSeverity::Info,
        LogSource::Studio,
        "License removed from this machine.",
    );
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
    use crate::workbench::app::RSpiceApp;

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
            let cloud = &mut *self.state.cloud;
            render_license_dialog(
                &mut LicenseParts {
                    dialog: &mut cloud.license_dialog,
                    license_key: &mut cloud.license_key,
                    license: &mut cloud.license,
                    model: &mut self.state.model,
                },
                ctx,
            );
        }
    }

    /// A verified key says where Activate will keep it on this platform:
    /// the file it writes, not a Windows variable on every system.
    #[test]
    fn a_verified_key_names_where_this_platform_keeps_it() {
        let mut app = RSpiceApp::test_instance();
        let cloud = &mut *app.state.cloud;
        open_license_dialog(&mut cloud.license_dialog, cloud.license.as_ref());
        app.state.cloud.license_dialog.text = license::SAMPLE_KEY.to_owned();
        verify_license_input(&mut app.state.cloud.license_dialog);
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
        let cloud = &mut *app.state.cloud;
        open_license_dialog(&mut cloud.license_dialog, cloud.license.as_ref());
        let mut seen = shown_texts(&mut app);
        app.state.cloud.license_dialog.text = "RSPICE-K1.TRUNCATED".to_owned();
        verify_license_input(&mut app.state.cloud.license_dialog);
        seen.extend(shown_texts(&mut app));
        app.state.cloud.license_dialog.text = license::SAMPLE_KEY.to_owned();
        verify_license_input(&mut app.state.cloud.license_dialog);
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
