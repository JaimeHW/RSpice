//! The license dialog drawn over a whole application's state: where a
//! verified key is kept.

use crate::workbench::app::RSpiceApp;
use rspice_app_cloud::services::license;
use rspice_app_cloud::workbench::app::dialogs::license::*;

/// Every text the dialog published for a reader to hear. Generic over the
/// app so that a test helper is not counted as a handler that takes the
/// whole application.
fn shown_texts<App: LicenseDialogHost>(app: &mut App) -> Vec<String> {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
