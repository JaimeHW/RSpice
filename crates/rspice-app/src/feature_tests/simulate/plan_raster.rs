//! The plan manager's visual regression over the whole application: every
//! route at every gated viewport matches its reviewed fingerprint.

// Native-only, like the render harness it reads.
#![cfg(not(target_arch = "wasm32"))]

use super::plan_manager::{app_with_every_lifecycle_state, route_draft};
use crate::ui::raster::Canvas;
use crate::workbench::state::SimulationPlanManagerMode;
use crate::workbench::surfaces::simulate::plan_manager::plan_manager_dialog;
use crate::workbench::surfaces::simulate::plan_manager::raster::{
    PLAN_MANAGER_FINGERPRINTS, baseline_cases, file_stem,
};
use egui::Vec2;

/// Render one route at one gated viewport and rasterize it.
///
/// The fixture and the draft are the contract tests' own — a catalog with one
/// plan of each lifecycle state, and the entry state
/// [`route_draft`] documents each route is reachable
/// in. A render seeded any other way would show an arrangement no reader can
/// get to, which is worse than no render at all.
fn raster(mode: SimulationPlanManagerMode, screen: Vec2) -> Canvas {
    let (mut app, active, available, _) = app_with_every_lifecycle_state();
    let draft = route_draft(&app, mode, active, available);

    // The dialog owns the draft for the frame and re-arms it in
    // `simulation_workflow`; each pass hands it a fresh one so the rasterizer's
    // three passes render the same state rather than accumulating it.
    //
    // The application is captured rather than taken as a parameter on purpose.
    // `tests/module_layering.rs` counts every whole-application mutable
    // parameter in the crate, test-only files included, and a render harness is
    // not a reason for that ceiling to move.
    crate::ui::raster::render(screen, |ui, _| {
        plan_manager_dialog(
            ui,
            &mut crate::workbench::features::cx(&mut app),
            draft.clone(),
        );
    })
}

fn regression_height(canvas: &Canvas) -> usize {
    canvas.content_height().max(1)
}

#[test]
fn every_plan_manager_route_matches_its_reviewed_gated_viewport_baselines() {
    // Every changed route at once, so one run names each fingerprint a
    // reviewed change replaces.
    let changed = baseline_cases()
        .filter_map(|(index, mode, viewport, screen)| {
            let canvas = raster(mode, screen);
            let actual = canvas.regression_fingerprint(regression_height(&canvas));
            (actual != PLAN_MANAGER_FINGERPRINTS[index])
                .then(|| format!("{index} {}: {actual}", file_stem(mode, viewport)))
        })
        .collect::<Vec<_>>();
    assert!(
        changed.is_empty(),
        "visual regression; review the renders before accepting the new fingerprints:\n{}",
        changed.join("\n")
    );
}
