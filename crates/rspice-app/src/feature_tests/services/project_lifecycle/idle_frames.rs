//! An idle frame asks whether the project holds unsaved work and captures
//! nothing: the answer is held for the working state until it changes.

use super::*;
use crate::workbench::RSpiceApp;
use crate::workbench::app::FrameSimulationInput;
use rspice_app_state::workbench::workspace::Workspace;

/// One whole application frame at 1600 x 900, as the shell paints it.
fn frame(ctx: &egui::Context, app: &mut RSpiceApp) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 900.0),
            )),
            ..Default::default()
        },
        |ui| {
            let ctx = ui.ctx().clone();
            let before = FrameSimulationInput::capture(&app.state);
            app.prepare_frame(&ctx);
            app.pump_live_session();
            app.render_frame_chrome(ui);
            app.render_secondary_application_windows(&ctx);
            let _ = app.state.refresh_live_checks(std::time::Instant::now());
            app.render_frame_dialogs(&ctx);
            app.end_frame(&before);
        },
    );
}

fn captures() -> usize {
    rspice_project::registry::PROJECT_CAPTURES.with(std::cell::Cell::get)
}

fn stamps(app: &RSpiceApp) -> String {
    let model = &app.state.model;
    format!(
        "workspace {} schematic {} libraries {} plan {} models {:?}",
        model.workspace.content.stamp(),
        model.schematic.editor_ref().design.content_stamp(),
        model.library_manager.content_stamp(),
        model.sim_setup.setup.content_stamp(),
        model.model_library_manager.content_stamp(),
    )
}

/// A saved project with a resistor on its sheet.
fn saved_project() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(2, 4));
    app.state.sync_active_schematic_to_workspace();
    let baseline = snapshot(&app.state).unwrap();
    app.state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    app
}

#[test]
fn idle_frames_in_every_workspace_capture_nothing() {
    let mut app = saved_project();
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    for workspace in Workspace::ALL {
        app.state.workbench.activate(workspace);
        for _ in 0..3 {
            frame(&ctx, &mut app);
        }
        let before = captures();
        let stamps_before = stamps(&app);
        for _ in 0..10 {
            frame(&ctx, &mut app);
        }
        assert_eq!(
            captures() - before,
            0,
            "{workspace:?}: idle frames captured the whole project; stamps {stamps_before} -> {}",
            stamps(&app)
        );
    }
}
