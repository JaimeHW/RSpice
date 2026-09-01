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
    "facc35cf91aa2da0ae6301260d0b46a177e50750cf78aee43c94a988cd0c1cd6",
    "51701dd4c9086d9124a4996ddbaa136404cb45fb7c0017dea09504d4c6ae0f9d",
    "0922bf4538d0c2c5551d9445aaf1e1e7e7202c651dff4a6836968ecf63771327",
    "01e1c06b1efa34fea89732f52e938fdb36dbea5dc003a6a726010d339eae709b",
    "57979422637a18b1d409c217ffaf44fdbd54d6e0034a40af48b467568231c8c3",
    "bc9b9be1ad5d76d5e696adda0897518ab18d17538cf9614b2cbf6e15af4e11c4",
    "8b91fd21f965fb6d93653b6468aa8a2862632dffe06d3628b53da0ebfbe1ef9b",
    "f3429815b5923096b6e58a9566040b4b08bbb7fa5c755ee0f651df0d312c0258",
    "aee8af2e16d6393a88d38ef4f4af48e6be0f1506c7d77077e40586c79ca59c12",
    "b7834551e8550fd9ff92cb219e58730bc34bd62eca4d4e379dc7512922c40c2f",
    "8cad911b7228c827039a7a3029ed53aaaf5162e94ab9da1acc94944b09537d68",
    "47bcd878b97315893a921bf8bfdd0811eed136d3ba91321579e45bc2314cf023",
    "e07bbb85bb0e8a89a9f4c57e673fa76a4489e5143343a5c8fa622cf23a5b08bc",
    "d101b1f873319a20b2862f0e1f84c845e1be32abce9187014a9443db8fdf6fc3",
    "ee1f2899c0ed509c43f1d37dc1575c819d72dd030f6d9e47d76a49c1d976aaf9",
    "a39722910ce62cc72ffa6078ce85bb0b5732e85842dd85b68f942956da26bd87",
    "4504751c10e297a3b50462801cb0b9327a3d4bd9e7d030fa7dc1063b24073b7d",
    "c4fcb11b90d4681d8b6e4a043178f0e72ef2d3450890aa4ce83f306c00d08fe1",
    "90fa3e543e703ec2622dcac642f71084665ff6d8342e59da506ebb1f9cf8ab4c",
    "a4a4c362759a162351ea3709fdf1d1b8c8c93eaf5234fd7af30270da71617cce",
    "543d622c16535086427cefc4e2ec6e07d79ae5d38a53eb5768719c00d2209a89",
    "05dbf1508e047d6aec46f7695105d835cf8c469bb813eeb390faf09f411bb421",
    "24a7dcca9bbb3112f2a2a9567f04491102608a64fb8fffbc41da4c69df68e242",
    "f108a45c899634b163f3f2a8f4bb19106ae1442957abf2e82eee2503256f6f95",
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
    for (_, mode, viewport, screen) in baseline_cases() {
        let canvas = raster(mode, screen);
        let height = regression_height(&canvas);
        eprintln!(
            "{} {}x{} {}",
            file_stem(mode, viewport),
            canvas.width(),
            height,
            canvas.regression_fingerprint(height)
        );
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
