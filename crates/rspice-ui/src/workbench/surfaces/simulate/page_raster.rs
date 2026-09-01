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
        fingerprint: "293b4c4af45a6af9eaf800ae3ef775c220c66d02c5cc906bbc814dd7c7823ecd",
    },
    StudioBaseline {
        name: "studio-excitations",
        page: SimulationPage::Excitations,
        fingerprint: "130884c32e9441b618358c0623b563c794d257aea2a20f86fda029157b613c97",
    },
    StudioBaseline {
        name: "studio-variables",
        page: SimulationPage::Variables,
        fingerprint: "a5a43bf7609649d230c12ab5e78afd498cad27c01e19e7932d6283382dee2e05",
    },
    StudioBaseline {
        name: "studio-outputs",
        page: SimulationPage::Outputs,
        fingerprint: "9026ed061e491ad77a57adfa8a6745df39302029c3bcc025fd3aa9c94b989de0",
    },
    StudioBaseline {
        name: "studio-specifications",
        page: SimulationPage::Specifications,
        fingerprint: "aa3e73e6c11b9f29249ffffb891ea536a4684489b6613f40b074a4b6148d899a",
    },
    StudioBaseline {
        name: "studio-run-set",
        page: SimulationPage::RunSet,
        fingerprint: "f5ffe09b9eb517a4e1db9f2bdc5bcc16898049f40adf92f99a33bb8c38045f04",
    },
    StudioBaseline {
        name: "studio-models",
        page: SimulationPage::Models,
        fingerprint: "e54b9c7a27a6816e439df19a25f5500143844c9fc06cc9e60824fa74435b3119",
    },
    StudioBaseline {
        name: "studio-solver",
        page: SimulationPage::Solver,
        fingerprint: "081f972432fdad09bed457b0f59efcd4a0d9eb32911cd006d26d2967cfefb3e5",
    },
    StudioBaseline {
        name: "studio-save",
        page: SimulationPage::Save,
        fingerprint: "cb780291a143be66782c050dd7ab6099a2211aef946bf8537ab4ea112478f424",
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
    for baseline in STUDIO_BASELINES {
        let canvas = raster(baseline.page, APPROVED_PAGE_WIDTH, |_| {});
        eprintln!(
            "{} {}x{} {}",
            baseline.name,
            canvas.width(),
            regression_height(&canvas),
            canvas.regression_fingerprint(regression_height(&canvas))
        );
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
