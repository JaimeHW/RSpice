//! Review renders of the Verify pages.
//!
//! Each page is rendered twice: on a project that has run nothing, and on one
//! holding a Monte Carlo run of a gain specification with three failing
//! samples, so the empty states and the populated tables can both be looked
//! at. The PNGs go to `RSPICE_RASTER_DIR` (default: the system temp directory).

use egui::vec2;

use crate::state::{SimulationRun, SimulationRunLifecycle, SpecEntry, SpecPointScope};
use crate::workbench::RSpiceApp;
use crate::workbench::state::{VerificationPage, Workspace};
use rspice_results::yield_analysis::{
    DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
};

const PAGE_WIDTH: f32 = 1100.0;
const PAGE_HEIGHT: f32 = 1400.0;

/// Forty Monte Carlo samples of a 40 dB gain floor, three of them under it.
pub(super) fn monte_carlo_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.content.specs.push(SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(40.0),
        max: None,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    });
    let samples = (0..40)
        .map(|index| match index {
            7 => 39.2,
            19 => 39.7,
            31 => 38.9,
            _ => 40.4 + f64::from(index % 9) * 0.35,
        })
        .collect::<Vec<f64>>();
    let trail = samples
        .iter()
        .map(|value| *value >= 40.0)
        .collect::<Vec<_>>();
    let pass_count = trail.iter().filter(|passes| **passes).count();
    let result = YieldResult {
        spec: YieldSpec::lower("gain", 40.0, "dB"),
        total_runs: samples.len(),
        pass_count,
        fail_count: samples.len() - pass_count,
        yield_percent: pass_count as f64 / samples.len() as f64 * 100.0,
        stats: DistributionStats::default(),
        samples,
        trail,
    };
    let mut run = SimulationRun::new(3);
    run.restore_lifecycle(SimulationRunLifecycle::Completed, 12.4)
        .expect("a finite elapsed time");
    let provenance = YieldAnalysisProvenance {
        source_run_id: run.run_id,
        source_dataset_id: run.dataset_id,
        seed: 0x1A2B,
        runs_requested: 40,
        runs_completed: 40,
        sampling_mode: MonteCarloSamplingMode::PseudoRandom,
    };
    app.state.simulation.retained.runs = vec![run].into();
    app.state.simulation.view.active_run_idx = Some(0);
    app.state
        .simulation
        .replace_yield_evidence(vec![result], Some(provenance));
    app
}

#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_verify_page() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for page in VerificationPage::NAVIGATION {
        for (state, fixture) in [
            ("empty", RSpiceApp::test_instance as fn() -> RSpiceApp),
            ("mc", monte_carlo_app),
        ] {
            let mut app = fixture();
            app.state.workbench.activate(Workspace::Verify);
            app.state.workbench.verification_page = page;
            let canvas =
                crate::ui::raster::render(vec2(PAGE_WIDTH, PAGE_HEIGHT), |ui, background| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(background))
                        .show(ui, |ui| super::show(ui, &mut app));
                });
            let height = canvas.content_height().max(1);
            let path = directory.join(format!("verify-{page:?}-{state}.png").to_lowercase());
            std::fs::write(&path, canvas.png(height)).expect("write page render");
            writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                .expect("write raster report");
        }
    }
}
