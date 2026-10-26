//! The license dialog drawn over a whole application's state: where a
//! verified key is kept, and the plain words it speaks in.

use crate::services::license;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::license::*;

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
