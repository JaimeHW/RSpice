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
    "2c3fcbe7f6b32ed221016fd3e2d5f082151cc1756fc1b7cc5497047edf579a20",
    "6ed5271bfd312e0d670e0c5fce4b104ffc530c710cc42cb108c3c00fe224a209",
    "b657f49ea25d0cc2f00d1a6aaebb40779bb8a5a19b13bc1668e6cd5a1d474357",
    "95fee073a627ec01c513444ffa5dd1b07940ca823bb7215b2152b6681862f1d5",
    "35bb879487efd0d94f54fe28b477147fe965c334462c662f5d84d942fa6ced1e",
    "e0b518d87d31cd727b70b82fda4fffe42c9acb802f31e6eea7d41b2876b786bb",
    "a24f2d88fa15ef2817f5e729666331008d5e4121953194e291bdbdf9f90166d0",
    "fd59c353fb7d9e33c04e6137adb3ab3b2fc1ee5238a7408d9099aa8b225efcfc",
    "467d590bfe15f0734ec14444ef36516986342237e93f4750e33e2df488c6f532",
    "9a4090c3e1a2deedd448cb11c673ddbd9a398112ca025db1d22c429fa0f30f97",
    "8b2b86650cf49eb40b7f89e8858723b4f97eb4421b1da64a5b427d5753771b23",
    "11c376446a544d8f460d1e3f7421aaf400fd6f8c967686ec6475a5c5404c7367",
    "054cc80cc3b61d721f0f3b306fb948118812d9a2af99487da552df7842200903",
    "c16dcf0f3d7ce7e67242b7ae1182d4ed5b835e3a628235693a923a027d49a05b",
    "6f80a9a6a12756c1a8edb918fe7bbd42b2920188b9766ae32d935e50b83dfe2c",
    "fe038e7a95417d2e474cf99cccbbcc103f0f24fdc3ee40adbd04359aa8824624",
    "79efa9ec965ed743acc756ad624fb80823bcae77d9b1a72e33a79b51e48a81d1",
    "d91802b27b5739f9823d908e9dfc332bb3d041b4bef308d8ccabe1b31bb73075",
    "aa01c920fea19456ef8d469294d3c981f5fc933c8b8583004be9ff986d9f72d8",
    "3f4316d22d7de5c8edadde20c5f22082492864a8e881efab76083218a36aa622",
    "81cf945d1f5dde6c88bac57b56c36487d8d8cb2d857448c3d2ba9b1502a97275",
    "bf2acef77b1ee0a8242459010e3edcce14c13477560103ce5911f02115d22264",
    "489658f3c90927e048d5bdb69e70b815346b75a7b6717d1ea0d358ee46621311",
    "ae5a3c61e5168618149dc4bc355d291ad7f648d677265462e63151dfe6842abe",
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
