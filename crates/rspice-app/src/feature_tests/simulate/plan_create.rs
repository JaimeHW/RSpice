//! The plan manager's Create route drawn over the whole application: every
//! fact it states paints whole at every gated viewport.

use crate::workbench::RSpiceApp;
use crate::workbench::state::{SimulationPlanManagerDraft, SimulationPlanManagerMode};
use crate::workbench::surfaces::simulate::plan_manager::plan_manager_dialog;
use egui::Rect;

/// Every fact this route paints lands on screen *whole* at every gated
/// viewport.
///
/// The shell's route fit gate proves nothing is clipped. It cannot see a
/// truncated value: a label elided to `Corner char…` sits entirely inside its
/// clip rect and passes. Elision is how a property row keeps its geometry, so
/// nothing here may be long enough to need it — which makes the absence of an
/// ellipsis in the paint the exact statement of that.
///
/// Every string this checks is this route's own: its title,
/// description and button labels, its setting rows, its controls and its two
/// statement groups. There is no list of expected labels, deliberately —
/// rewording a row is covered without editing this test.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_stated_fact_paints_whole_at_every_gated_viewport() {
    for (name, screen) in [
        ("desktop", egui::Vec2::new(1024.0, 640.0)),
        ("edge-to-edge", egui::Vec2::new(820.0, 640.0)),
        ("portrait", egui::Vec2::new(560.0, 900.0)),
    ] {
        let elided = elided_text_at(screen);
        assert!(
            elided.is_empty(),
            "at {name} the create route painted elided values: {elided:?}"
        );
    }
}

/// Every string the real dialog painted at one viewport that had to be
/// shortened to fit the box it was painted in.
#[cfg(not(target_arch = "wasm32"))]
fn elided_text_at(screen: egui::Vec2) -> Vec<String> {
    let mut app = RSpiceApp::test_instance();
    let active = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture has a stable plan")
        .id();
    let name = app.state.model.sim_setup.active_plan_name().to_string();
    let mut draft = SimulationPlanManagerDraft::new(active, name);
    draft.mode = SimulationPlanManagerMode::Create;
    draft.name = "New simulation plan".to_owned();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // The dialog owns the draft for a frame and the shell re-arms it, so each
    // pass is handed its own copy and the three passes render one state
    // rather than accumulating it.
    let mut pass = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                ..Default::default()
            },
            |ctx| {
                plan_manager_dialog(
                    ctx,
                    &mut crate::workbench::features::cx(&mut app),
                    draft.clone(),
                )
            },
        )
    };
    let _ = pass();
    let _ = pass();
    let output = pass();
    output
        .shapes
        .iter()
        .filter_map(|shape| match &shape.shape {
            egui::epaint::Shape::Text(text) => Some(text.galley.job.text.clone()),
            _ => None,
        })
        .filter(|painted| painted.contains('\u{2026}'))
        .collect()
}
