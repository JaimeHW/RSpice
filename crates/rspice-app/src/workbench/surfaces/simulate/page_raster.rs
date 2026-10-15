//! Deterministic visual regressions for every Simulation Studio setup page.
//!
//! The contract tests next door read the text a page paints; nothing in them
//! sees where it lands. This composes one page into a full-width panel and
//! hands it to [`crate::ui::raster`], which runs it headless and rasterizes in
//! software — no GPU, no window, and no dependence on the wasm build, which is
//! where every other way of looking at these pages currently goes through.
//!
//! Read the renders for layout, not for wording; the rasterizer's own header
//! says why.
//!
//! The default tests compare reviewed fingerprints and never write files. The
//! ignored review tools print replacement fingerprints or write PNGs to
//! `RSPICE_RASTER_DIR` (default: the system temp directory).

use egui::vec2;

use crate::ui::raster::Canvas;
use crate::workbench::RSpiceApp;
use crate::workbench::state::SimulationPage;

const APPROVED_PAGE_WIDTH: f32 = 1100.0;
#[derive(Clone, Copy)]
struct StudioBaseline {
    name: &'static str,
    page: SimulationPage,
    fingerprint: &'static str,
}

const STUDIO_BASELINES: [StudioBaseline; 9] = [
    StudioBaseline {
        name: "studio-analyses",
        page: SimulationPage::Analyses,
        fingerprint: "2c75a139aa9add63febb93b85b8014b8b314a539888a491fb17740eddaf8ad16",
    },
    StudioBaseline {
        name: "studio-excitations",
        page: SimulationPage::Excitations,
        fingerprint: "767b1e0232f37765f205cae0f6e02e0732a2943b5431d77addfd001ca8228067",
    },
    StudioBaseline {
        name: "studio-variables",
        page: SimulationPage::Variables,
        fingerprint: "83651f197ecfc70fbf460ac6700e2e54d390690ede8c0bf586b056c8c27fdd71",
    },
    StudioBaseline {
        name: "studio-outputs",
        page: SimulationPage::Outputs,
        fingerprint: "b56873cb5338b1c24f5a9cc6069332c7b33ce51c1b650b054ff13d21d40ee540",
    },
    StudioBaseline {
        name: "studio-specifications",
        page: SimulationPage::Specifications,
        fingerprint: "7ed4bdb93a52f88acbbf7435bc69e2434f326667210a0788c8f318706e561d7c",
    },
    StudioBaseline {
        name: "studio-run-set",
        page: SimulationPage::RunSet,
        fingerprint: "e4ac360cb75dc228c8bf6b81c3f74c69eed3ecdde8b298e15915139b920e633e",
    },
    StudioBaseline {
        name: "studio-models",
        page: SimulationPage::Models,
        fingerprint: "5bd2d15f048095971bf1cf30964045781e6ebedac175b3a0020aefa682939a71",
    },
    StudioBaseline {
        name: "studio-solver",
        page: SimulationPage::Solver,
        fingerprint: "e22eab4055adcd77aa7ef84190a0fe265b2788876465ca6b06f291d2a2e2ec46",
    },
    StudioBaseline {
        name: "studio-save",
        page: SimulationPage::Save,
        fingerprint: "960de4e65dbbbdbefe34a6495430a418fb3b02c9bc7e4d7bf2e283693f4e6016",
    },
];

/// Width of the page column in the shell at a 1680 px window. Override with
/// `RSPICE_RASTER_WIDTH` to find the width a page stops overflowing at.
fn page_width() -> f32 {
    std::env::var("RSPICE_RASTER_WIDTH")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1100.0)
}
/// Tall enough that no studio page is cut off; the render is cropped back to
/// the content afterwards.
const PAGE_HEIGHT: f32 = 2600.0;

/// Render one page and rasterize it.
fn raster(page: SimulationPage, width: f32, seed: impl FnOnce(&mut RSpiceApp)) -> Canvas {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.simulation_page = page;
    seed(&mut app);

    crate::ui::raster::render(vec2(width, PAGE_HEIGHT), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| super::show(ui, &mut app));
    })
}

fn regression_height(canvas: &Canvas) -> usize {
    canvas.content_height().max(1)
}

#[test]
fn studio_visual_baselines_cover_every_page_once_and_are_unique() {
    assert_eq!(STUDIO_BASELINES.len(), SimulationPage::NAVIGATION.len());
    assert_eq!(
        STUDIO_BASELINES.map(|baseline| baseline.page),
        SimulationPage::NAVIGATION,
        "the visual baseline table must follow the complete Studio navigation catalog"
    );
    let names = STUDIO_BASELINES
        .iter()
        .map(|baseline| baseline.name)
        .collect::<std::collections::HashSet<_>>();
    let fingerprints = STUDIO_BASELINES
        .iter()
        .map(|baseline| baseline.fingerprint)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        names.len(),
        STUDIO_BASELINES.len(),
        "duplicate Studio baseline name"
    );
    assert_eq!(
        fingerprints.len(),
        STUDIO_BASELINES.len(),
        "Studio pages must have independently reviewed fingerprints"
    );
}

#[test]
fn every_studio_page_matches_its_reviewed_visual_baseline() {
    for baseline in STUDIO_BASELINES {
        let canvas = raster(baseline.page, APPROVED_PAGE_WIDTH, |_| {});
        canvas.assert_regression(
            baseline.name,
            regression_height(&canvas),
            baseline.fingerprint,
        );
    }
}

#[test]
#[ignore = "prints source-ready visual fingerprints after explicit review"]
fn print_studio_visual_fingerprints_for_review() {
    use std::io::Write as _;

    let mut stdout = std::io::stdout().lock();
    for baseline in STUDIO_BASELINES {
        let canvas = raster(baseline.page, APPROVED_PAGE_WIDTH, |_| {});
        writeln!(
            stdout,
            "{} {}x{} {}",
            baseline.name,
            canvas.width(),
            regression_height(&canvas),
            canvas.regression_fingerprint(regression_height(&canvas))
        )
        .expect("write visual fingerprint");
    }
}

/// Write one form per analysis kind that owns advanced options, so the fields
/// they add under the analysis's own parameters can be reviewed.
///
/// [`render_every_studio_page`] renders the Analyses route on the default plan,
/// which is one transient — and the option fields differ per kind, because what
/// an analysis owns differs per kind. The operating point is the one that
/// cannot be seen any other way: it is not in the default plan, and its
/// Convergence field is the one the homotopy chooser above it decides.
///
/// PSS is here because it carries the largest authored parameter set in the
/// catalogue — every control the engine's `.PSS` card holds — and two of its
/// fields change enablement with the autonomous switch, which is a thing to
/// look at rather than to assert.
///
/// DC mismatch is here for the note under its switches. The form states a
/// prerequisite the project has to satisfy elsewhere — the design's own
/// Spectre statistics block — and full-width prose beside typed fields is
/// exactly the kind of thing that reads well at one width and wraps badly at
/// another.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_analysis_option_form() {
    use std::io::Write as _;

    use crate::simulation::plan::AnalysisKind;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report_output = stderr.lock();
    for kind in [
        AnalysisKind::Transient,
        AnalysisKind::OperatingPoint,
        AnalysisKind::DcSweep,
        AnalysisKind::HarmonicBalance,
        AnalysisKind::Pss,
        AnalysisKind::DcMismatch,
    ] {
        let canvas = raster(SimulationPage::Analyses, page_width(), |app| {
            let Ok(plan) = app.state.sim_setup.stable_analysis_plan_mut() else {
                return;
            };
            let instance = plan
                .instances()
                .iter()
                .find(|instance| instance.kind() == kind)
                .map(|instance| instance.id())
                .or_else(|| plan.insert(kind).ok().map(|(id, _)| id));
            app.state.workbench.active_analysis_instance = instance;
        });
        let height = regression_height(&canvas);
        let path = directory.join(format!("options-{kind:?}.png").to_lowercase());
        std::fs::write(&path, canvas.png(height)).expect("write page render");
        writeln!(
            report_output,
            "{} {}x{}",
            path.display(),
            canvas.width(),
            height
        )
        .expect("write raster qualification report");
    }
}

/// Write every studio page to a PNG so its design can be reviewed.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_studio_page() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report_output = stderr.lock();

    for page in SimulationPage::NAVIGATION {
        let canvas = raster(page, page_width(), |_| {});
        let height = regression_height(&canvas);
        let bytes = canvas.png(height);
        let path = directory.join(format!("studio-{:?}.png", page).to_lowercase());
        std::fs::write(&path, &bytes).expect("write page render");
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
