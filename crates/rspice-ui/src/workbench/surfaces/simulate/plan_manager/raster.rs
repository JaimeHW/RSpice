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
    "763ba3a316496c3feb26cfe834e7e3897b3d788465b76fb4a6e651a953adb406",
    "7a3d70e446f3cbda6dc702e823b1edc071b30c4163b8d55200522270f3fbf7fa",
    "ce639539b85b92930b210e7f2bc61ded5c7d24a5868fc90529c08696fd824182",
    "dd85f6b8a1883e6466229cd5e1d41b17f3bd9f12e936414b38b485fbb2020a27",
    "07c732c00775f1791cc5911befa712c13609f6f66f1d90421422dec4353998b2",
    "24a08397707c497b3d4b2762b1e51a11404463762e9dafd8c19f35da9955761e",
    "bd314df6163db05db841326a02d8e6c4346bec1bed7580cb748bc3052577e5d2",
    "5bf6f50cb58784ff663b2f727850d9cdf75bba1061260a2242f4fe94d027a88a",
    "6823651099620974bbc6177dc69a00d76032ae3e2fd84b6d7e167804b07b1cc6",
    "a24a6b4bbc9d3b15af793a2dc524c4f556b67cfcae3117ca983e16de35bd23e8",
    "7651a0bfa39be605b87103276e1188f0e6c006f2e9ed0538fae548a971610b08",
    "f225978ce2339310b3c6fb8d12db55036a2b9a2acd7eff0b2f19780ba91d5da5",
    "550c12d8d308554b36ae5d923cba70ca72c9c3c6ef3e4f5e8b698627a1a4a3fd",
    "a037233afe68f6fda0f8c677f5d5a9111494622927abac264b8c017f88199fc9",
    "3ec9e7ae70db95b398e92052abd6ef80a4e3e1e867a9e6364ce6df2ef87a748c",
    "8f7cdfe047af2897041f2f62c6bdfbd77894b96f50d97557753232103ad82b39",
    "b3bd06da384189e1989e05d3ae3776f96386d71a0e1c327c923990303258ebef",
    "684585a5d65b5113cbadfa99322445b8d87418e0c61f84c6889de46041d37d1b",
    "3661538b949cd6a4b3847a60af2a45514e5aa6099b6ed92f2a925cee3b5488e3",
    "6d5566176f71b1b79e15a956d0053eeb5e93a0866ac9f939115825c009556d9c",
    "063e139fa4f8ca7a7ae8959c9372a03711a8aaa398e80a33efdc4651ea67006f",
    "932dd6885fcfc6c77df007bf164f7bd1233c0ca36febf5f6c3c82ea95aeaa2c1",
    "a14fea0c0c4d7bf3387496d407fa0017f6bf158f83bff656443772d04bb839ad",
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
