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
    "19dbbe4c65d0114e04382aba96512d96942417d6ffb1d41b8f307f0e57f90ade",
    "3c5c57eb37fcf131d9e45c1f30afe3911a4c052342830875ab7b37e1ffbe69f3",
    "01308ed40222dfd018c6adc75be93967f735a94658637af6d381ec2feed9171d",
    "8857565d9f6c1f0c9b8f0878a320eccd4136a1c79a39ddb70d62c1b384398024",
    "8475b99ed02184ec59c745591031733bd9110dd94785353c15b4ddbdf915a31f",
    "3cd2aaeb7f63e347a926ce3d99e7ae76f5e27b5cfa47f49a2b618ba7dea6e0ea",
    "a24f2d88fa15ef2817f5e729666331008d5e4121953194e291bdbdf9f90166d0",
    "fd59c353fb7d9e33c04e6137adb3ab3b2fc1ee5238a7408d9099aa8b225efcfc",
    "467d590bfe15f0734ec14444ef36516986342237e93f4750e33e2df488c6f532",
    "9a4090c3e1a2deedd448cb11c673ddbd9a398112ca025db1d22c429fa0f30f97",
    "8b2b86650cf49eb40b7f89e8858723b4f97eb4421b1da64a5b427d5753771b23",
    "11c376446a544d8f460d1e3f7421aaf400fd6f8c967686ec6475a5c5404c7367",
    "ce2b845c8c1becc7507ba20e0f6ff84f86fde7646ebe5177194550005fa45ac7",
    "95de1a93406bc63f877221e6e3593cd1a3c6d66cfe1562c7449c4a175ea724e7",
    "fc9f41e882f53bc8a577386c2e8f8475190d541885df261a58a8ce91a51deb03",
    "fe038e7a95417d2e474cf99cccbbcc103f0f24fdc3ee40adbd04359aa8824624",
    "79efa9ec965ed743acc756ad624fb80823bcae77d9b1a72e33a79b51e48a81d1",
    "d91802b27b5739f9823d908e9dfc332bb3d041b4bef308d8ccabe1b31bb73075",
    "3ba9aba90369761c1b2723b881478b3e806fbe96f3de6e1d76ad1088c8fa5c13",
    "3c99f597f1699c64f0d27952582e3bd54a052e252021c352dd3ccf977ed0284b",
    "c648e7f5048cd4c3e608f1eeeb23e964192da7029f39787d7d72dea0fa7be270",
    "47c1009387a5a2267544bc010480b679489b3b342e103823ef2f5e6cc9f91881",
    "ec18a33c8d777aa1ab81f7924e6759cd13e2e3c3c99014029ac3ddf9ca3769c2",
    "01952e7611f2103d7f8ac40802add8643f52cc5491266bc27b14cba487b7620a",
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
