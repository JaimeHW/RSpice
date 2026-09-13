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
    "680124488a2140674ece7b787c27391f0a7b69958748dc39ed08f31e0bc306b3",
    "396d2104655f180c687af7c24dcc9dafa240ada52fb27db770b5759ab47244d0",
    "6477edc6a9c79bc920c85a6ff771a95cf5dd89fbd01a4bd539691067f00f5299",
    "dd85f6b8a1883e6466229cd5e1d41b17f3bd9f12e936414b38b485fbb2020a27",
    "a6474a91c64e20530285add305777bdc83b4efb164b73b0b42964cae0a84e660",
    "2bfa33deb52c911a5e210707d86e741fbf8d873e53cf8b3d615617eb62b691e5",
    "bd314df6163db05db841326a02d8e6c4346bec1bed7580cb748bc3052577e5d2",
    "7c5159789b141248e98ab634d924d1ad5f05609699dc0cf079e26c030211b59c",
    "56df4bc1bf084a160e9439c27ed2ba3ed9cac57c16988eb60f84df9ce73d7a1b",
    "a24a6b4bbc9d3b15af793a2dc524c4f556b67cfcae3117ca983e16de35bd23e8",
    "7651a0bfa39be605b87103276e1188f0e6c006f2e9ed0538fae548a971610b08",
    "908ce462123fcfb8d68abbcebb727291c07fc221f5f0000d8546895f209a7f04",
    "550c12d8d308554b36ae5d923cba70ca72c9c3c6ef3e4f5e8b698627a1a4a3fd",
    "a5a44f571a860b2924f2cc75e74ebb9ebfddd3aea5aee85254d0d024d66d7ba3",
    "a41e78a6cff7a75011509b391e2a97ea61cb8ab2d9c42371e64d3470fc897e02",
    "8f7cdfe047af2897041f2f62c6bdfbd77894b96f50d97557753232103ad82b39",
    "38b7c0e7f8f813394506d7da398e62e83f747cfe61bed251a65f947c28c534e4",
    "3fc34f6f7f9f1512e8a1f7b61f5ab101c471c1b8f63a4609f61a6db12ca68b2b",
    "3661538b949cd6a4b3847a60af2a45514e5aa6099b6ed92f2a925cee3b5488e3",
    "929237512999c2f8de8eadd6fe65cc11db4d3818337010f674d9037efcea511f",
    "24561f6e4ecb6a4f6bbf39061456502b70041d78147a09e46e4de7695dc38d48",
    "932dd6885fcfc6c77df007bf164f7bd1233c0ca36febf5f6c3c82ea95aeaa2c1",
    "b29414145c620e2e2528c5eacde0d8aedb68ccbb12eb3b7e5bce2a1d44ba543f",
    "03bb9fc547530a3f8f517074f3ae3598e2ce5c35fff3d9de33f08a838df86ff0",
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
