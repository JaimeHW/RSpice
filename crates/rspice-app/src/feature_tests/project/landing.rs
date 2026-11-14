//! The landing page drawn with no project open, at every supported width.

use egui::{Rect, vec2};

use crate::workbench::RSpiceApp;
use crate::workbench::shell_host::project::landing::show;

#[test]
fn landing_renders_without_project_state_at_all_supported_widths() {
    for size in [vec2(1440.0, 900.0), vec2(820.0, 1180.0), vec2(390.0, 844.0)] {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state.close_project_session();
        app.state.platform.recovery_catalog = Default::default();

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );

        assert!(!output.shapes.is_empty(), "{}x{}", size.x, size.y);
        assert!(!app.state.platform.project_lifecycle.is_open());
    }
}
