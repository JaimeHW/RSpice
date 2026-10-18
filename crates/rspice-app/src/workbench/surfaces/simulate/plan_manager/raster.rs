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
//! The default tests compare reviewed fingerprints and never write files. The
//! ignored review tools print replacement fingerprints or write PNGs to
//! `RSPICE_RASTER_DIR` (default: the system temp directory).

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
    "1d98b28655ce3cc8aa3fc8b34149edd0b9040dca6677cbf55ff0b146280f439f",
    "8639a76a51f39da8f59f73e3da49b97896a13d5476e86751ad83f4b87af91c25",
    "157ac6863c5cde705f36d1579e22f945524864c4456116a3373b9eda09d57bb9",
    "a24f2d88fa15ef2817f5e729666331008d5e4121953194e291bdbdf9f90166d0",
    "fd59c353fb7d9e33c04e6137adb3ab3b2fc1ee5238a7408d9099aa8b225efcfc",
    "467d590bfe15f0734ec14444ef36516986342237e93f4750e33e2df488c6f532",
    "5185b3838c43c04daf50044ec693b0433a8a33d4a13313cda33f447e920e61f0",
    "46e449c015d4d6fc8e508d4b84bf32d29a5359ea19fd0636b9f5dc5272693962",
    "32409351f229a6c60bbe5f2bbeb8e6d88cb1fae71bfab5c0c9d6e7402c040a1f",
    "c35b77ed417c19bda2b562e3a83f293f6c7ec55d58eb24fa8eb10f350823af1d",
    "22939f359343ffe0cac62880041c7a80f236df3964046ae803419a34eb4b4db4",
    "bee832d83c8dee0e2c7aa684026520de1117cc30ea9180789d8ec7787cc79287",
    "fe038e7a95417d2e474cf99cccbbcc103f0f24fdc3ee40adbd04359aa8824624",
    "79efa9ec965ed743acc756ad624fb80823bcae77d9b1a72e33a79b51e48a81d1",
    "d91802b27b5739f9823d908e9dfc332bb3d041b4bef308d8ccabe1b31bb73075",
    "7cdc53e8f253ac2d93332d6ec0391d99f2ec7e5ceadb28927af31f34d472423a",
    "a37a06e612aed8182bde68315057afe5968f94a3eeb08c96f466672b646a1f57",
    "e5eec721e798c0d2839cafae02546e58d4f0c31e1b4e46f836c0dac8c273fb0b",
    "31be96b0f91ee078b58c2c280c3f25c0f526fe4d7d588d6ee5d11f43767249e5",
    "348bfd086036131d2780864bda05e6e36f04ac57d87c7e25cbaa5a2d89b21c46",
    "0c4c38fd52253fac684e8e8fe40ca53b91cd059d8f57f2fbfc31594e8eb20b37",
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

#[test]
#[ignore = "prints source-ready visual fingerprints after explicit review"]
fn print_plan_manager_visual_fingerprints_for_review() {
    use std::io::Write as _;

    let mut stdout = std::io::stdout().lock();
    for (_, mode, viewport, screen) in baseline_cases() {
        let canvas = raster(mode, screen);
        let height = regression_height(&canvas);
        writeln!(
            stdout,
            "{} {}x{} {}",
            file_stem(mode, viewport),
            canvas.width(),
            height,
            canvas.regression_fingerprint(height)
        )
        .expect("write visual fingerprint");
    }
}

/// Write every route at every gated viewport to PNGs so the design can be
/// reviewed.
///
/// Twenty-four images: eight modes at three shapes. The list of modes is
/// [`tests::EVERY_ROUTE`](super::tests::EVERY_ROUTE), which the coverage claim
/// beside it holds to the mode enum, so a ninth route arrives here through the
/// same edit that dispatches it rather than whenever someone remembers this
/// file.
///
/// Read them for layout, not for wording — the module header says why.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_route_at_every_gated_viewport() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report_output = stderr.lock();

    for mode in super::tests::EVERY_ROUTE {
        for (viewport, screen) in super::tests::GATED_VIEWPORTS {
            let canvas = raster(mode, screen);
            // Cropped to the last row anything painted on, so a surface shorter
            // than its viewport is not reported as acres of empty space. A
            // surface *taller* than its viewport cannot hide here: the canvas
            // is the viewport, so the overflow is simply cut off, and the fit
            // gates are what prove there is none.
            let height = regression_height(&canvas);
            let bytes = canvas.png(height);
            let path = directory.join(format!("{}.png", file_stem(mode, viewport)));
            std::fs::write(&path, &bytes).expect("write plan-manager render");
            writeln!(
                report_output,
                "{} {}x{} {} bytes",
                path.display(),
                canvas.width(),
                height,
                bytes.len()
            )
            .expect("write raster qualification report");
        }
    }
}
