//! Deterministic visual regressions for every plan-manager route and gated
//! viewport.
//!
//! The contract tests next door read what the manager paints and how wide it
//! came out; nothing in them sees the result. This runs the real dialog through
//! [`crate::ui::raster`] — headless, software-filled, no GPU, no window, and no
//! dependence on the wasm build.
//!
//! Every route at every gated viewport — the same eight modes and the same
//! three shapes the contract tests gate, taken from
//! [`tests::GATED_VIEWPORTS`](super::tests::GATED_VIEWPORTS) rather than
//! restated, so a render is of the arrangement that was actually asserted
//! about. Each canvas is exactly its viewport: the product rule is that the
//! whole surface fits without scrolling, and a taller canvas would let a
//! surface that overflows the reader's screen look complete here.
//!
//! Read the renders for layout, not for wording — the rasterizer samples the
//! font atlas without filtering, and its own header says what that costs. A
//! glyph read off one of these images is not a defect report; check the string
//! in `tests.rs`, which reads the galley rather than the pixels.
//!
//! The tests compare reviewed fingerprints and never write files.

// Native-only, like the render harnesses in `tests.rs` it shares fixtures with:
// it rasterizes to a file for a person to open.
#![cfg(not(target_arch = "wasm32"))]

use egui::Vec2;

use crate::ui::raster::Canvas;
use crate::workbench::state::SimulationPlanManagerMode;

/// Route-major, then viewport-major, matching `EVERY_ROUTE × GATED_VIEWPORTS`.
const PLAN_MANAGER_FINGERPRINTS: [&str; 24] = [
    "fb0983fa5c2c016315ee7411daf59115e63af4e8dc4208e29acc9b481494a7a7",
    "7a9e3b6ed88e50a099e6c3676b91e83384182d0fddb912d137cd61fe146db181",
    "a7d240473e8264b634a5034ce1bf615080fe08b47adcadd44a243d277785faac",
    "95fee073a627ec01c513444ffa5dd1b07940ca823bb7215b2152b6681862f1d5",
    "35bb879487efd0d94f54fe28b477147fe965c334462c662f5d84d942fa6ced1e",
    "e0b518d87d31cd727b70b82fda4fffe42c9acb802f31e6eea7d41b2876b786bb",
    "a24f2d88fa15ef2817f5e729666331008d5e4121953194e291bdbdf9f90166d0",
    "fd59c353fb7d9e33c04e6137adb3ab3b2fc1ee5238a7408d9099aa8b225efcfc",
    "467d590bfe15f0734ec14444ef36516986342237e93f4750e33e2df488c6f532",
    "5185b3838c43c04daf50044ec693b0433a8a33d4a13313cda33f447e920e61f0",
    "46e449c015d4d6fc8e508d4b84bf32d29a5359ea19fd0636b9f5dc5272693962",
    "32409351f229a6c60bbe5f2bbeb8e6d88cb1fae71bfab5c0c9d6e7402c040a1f",
    "a23175b87e1596cd3515cee54eb8a7a7beb0757a37bbcdbadb40aa035a08750f",
    "090f97cc696c58cbef52ebcf69fbc99c717ea2cb55fb1bc1badf68f31c4c3043",
    "309522b64ce7d4851d2840c29111ed64c1ec4ba3376d5ba7587f168cd143f742",
    "fe038e7a95417d2e474cf99cccbbcc103f0f24fdc3ee40adbd04359aa8824624",
    "79efa9ec965ed743acc756ad624fb80823bcae77d9b1a72e33a79b51e48a81d1",
    "d91802b27b5739f9823d908e9dfc332bb3d041b4bef308d8ccabe1b31bb73075",
    "aa01c920fea19456ef8d469294d3c981f5fc933c8b8583004be9ff986d9f72d8",
    "3f4316d22d7de5c8edadde20c5f22082492864a8e881efab76083218a36aa622",
    "81cf945d1f5dde6c88bac57b56c36487d8d8cb2d857448c3d2ba9b1502a97275",
    "77839de4fd64c4804b043a7dd36923ddafc9b091af7d43939ebe1122da9e1c31",
    "58961f40439dc1e726f7d2543792df36f046abf3cd34ce903564a384429cecc6",
    "c02f922f49db7d1929f43f325faa5e10e26ce7bfa4912abb9fb2485cf48b1232",
];

/// Render one route at one gated viewport and rasterize it.
///
/// The fixture and the draft are the contract tests' own — a catalog with one
/// plan of each lifecycle state, and the entry state
/// [`route_draft`](super::tests::route_draft) documents each route is reachable
/// in. A render seeded any other way would show an arrangement no reader can
/// get to, which is worse than no render at all.
fn raster(mode: SimulationPlanManagerMode, screen: Vec2) -> Canvas {
    let (mut app, active, available, _) = super::tests::app_with_every_lifecycle_state();
    let draft = super::tests::route_draft(&app, mode, active, available);

    // The dialog owns the draft for the frame and re-arms it in
    // `simulation_workflow`; each pass hands it a fresh one so the rasterizer's
    // three passes render the same state rather than accumulating it.
    //
    // The application is captured rather than taken as a parameter on purpose.
    // `tests/module_layering.rs` counts every whole-application mutable
    // parameter in the crate, test-only files included, and a render harness is
    // not a reason for that ceiling to move.
    crate::ui::raster::render(screen, |ui, _| {
        super::plan_manager_dialog(ui, &mut app, draft.clone());
    })
}

/// A route's name in a file name: its variant, lowercased.
///
/// Taken from `Debug` rather than authored beside the enum, so a renamed
/// variant renames its renders instead of leaving a file named after a mode
/// that no longer exists.
fn file_stem(mode: SimulationPlanManagerMode, viewport: &str) -> String {
    format!(
        "plan-manager-{}-{}",
        format!("{mode:?}").to_ascii_lowercase(),
        viewport.replace(' ', "-")
    )
}

fn regression_height(canvas: &Canvas) -> usize {
    canvas.content_height().max(1)
}

fn baseline_cases() -> impl Iterator<Item = (usize, SimulationPlanManagerMode, &'static str, Vec2)>
{
    super::tests::EVERY_ROUTE
        .into_iter()
        .flat_map(|mode| {
            super::tests::GATED_VIEWPORTS
                .into_iter()
                .map(move |(viewport, screen)| (mode, viewport, screen))
        })
        .enumerate()
        .map(|(index, (mode, viewport, screen))| (index, mode, viewport, screen))
}

#[test]
fn plan_manager_visual_baselines_cover_the_route_viewport_product_and_are_unique() {
    assert_eq!(
        PLAN_MANAGER_FINGERPRINTS.len(),
        super::tests::EVERY_ROUTE.len() * super::tests::GATED_VIEWPORTS.len(),
        "the visual baseline table must cover every route at every gated viewport"
    );
    let names = baseline_cases()
        .map(|(_, mode, viewport, _)| file_stem(mode, viewport))
        .collect::<std::collections::HashSet<_>>();
    let fingerprints = PLAN_MANAGER_FINGERPRINTS
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        names.len(),
        PLAN_MANAGER_FINGERPRINTS.len(),
        "duplicate plan-manager visual baseline identity"
    );
    assert_eq!(
        fingerprints.len(),
        PLAN_MANAGER_FINGERPRINTS.len(),
        "each route/viewport pair must have an independently reviewed fingerprint"
    );
}

#[test]
fn every_plan_manager_route_matches_its_reviewed_gated_viewport_baselines() {
    for (index, mode, viewport, screen) in baseline_cases() {
        let canvas = raster(mode, screen);
        canvas.assert_regression(
            &file_stem(mode, viewport),
            regression_height(&canvas),
            PLAN_MANAGER_FINGERPRINTS[index],
        );
    }
}
