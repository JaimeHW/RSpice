//! The Simulate and Verify docks, rendered from seeded state for review.
//!
//! The Simulate inspector is rendered in the three states its last run can be
//! in: complete, failed, and never run. The Verify inspector and both
//! navigators are rendered beside them. PNGs and their measured geometry go to
//! `RSPICE_RASTER_DIR` (default: the system temp directory).

use egui::Vec2;

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun, SimulationRunLifecycle,
    SpecEntry, SpecPointScope,
};
use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;
use crate::workbench::state::{VerificationPage, Workspace};
use rspice_results::yield_analysis::{
    DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
};

const INSPECTOR_DEFAULT_WIDTH: f32 = 312.0;
const INSPECTOR_MIN_WIDTH: f32 = 278.0;
const NAVIGATOR_WIDTH: f32 = 260.0;

/// The first analysis of the default plan: the one the inspector selects.
fn first_instance(app: &RSpiceApp) -> AnalysisInstanceId {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .expect("the default plan")
        .instances()
        .first()
        .expect("the default plan holds a transient")
        .id()
}

/// Retain Run 12 of `instance`, finished in `elapsed` seconds, failed with
/// `error` when there is one.
fn retain_run(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
    error: Option<&str>,
    elapsed: f64,
) {
    let analysis = match error {
        None => AnalysisResult::new(1, AnalysisType::Transient, "Transient"),
        Some(error) => AnalysisResult::failed(1, AnalysisType::Transient, "Transient", error),
    }
    .with_provenance(
        AnalysisResultProvenance::new(
            instance,
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("test provenance is internally valid"),
    );
    let mut run = SimulationRun::new(12);
    run.add_analysis(analysis);
    let lifecycle = if error.is_some() {
        SimulationRunLifecycle::Failed
    } else {
        SimulationRunLifecycle::Completed
    };
    run.restore_lifecycle(lifecycle, elapsed)
        .expect("a finite elapsed time");
    app.state.simulation.retained.runs.insert(0, run);
}

fn simulate_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    app
}

fn simulate_last_run_ok() -> RSpiceApp {
    let mut app = simulate_app();
    let instance = first_instance(&app);
    retain_run(&mut app, instance, None, 1.84);
    app
}

fn simulate_last_run_failed() -> RSpiceApp {
    let mut app = simulate_app();
    let instance = first_instance(&app);
    retain_run(
        &mut app,
        instance,
        Some("Timestep too small at t = 1.25 µs"),
        0.42,
    );
    app
}

/// Forty Monte Carlo samples of a 40 dB gain floor, three of them under it.
fn verify_app(page: VerificationPage) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Verify);
    app.state.workbench.verification_page = page;
    app
}

fn verify_monte_carlo() -> RSpiceApp {
    let mut app = verify_app(VerificationPage::Yield);
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

fn verify_yield_empty() -> RSpiceApp {
    verify_app(VerificationPage::Yield)
}

fn verify_tuning() -> RSpiceApp {
    verify_app(VerificationPage::Tuning)
}

fn verify_corners() -> RSpiceApp {
    verify_app(VerificationPage::Corners)
}

/// One review panel: its file name and the app state it renders.
type ReviewPanel = (&'static str, fn() -> RSpiceApp);

fn review_directory() -> std::path::PathBuf {
    std::env::var("RSPICE_RASTER_DIR").map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from)
}

/// The Simulate and Verify inspectors as PNGs, with their measured geometry
/// beside each, at the dock's default and minimum widths.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_simulate_and_verify_inspectors_for_review() {
    let directory = review_directory();
    let panels: [ReviewPanel; 7] = [
        ("simulate-inspector-last-run-ok", simulate_last_run_ok),
        ("simulate-inspector-last-run-failed", simulate_last_run_failed),
        ("simulate-inspector-never-run", simulate_app),
        ("verify-inspector-yield-mc", verify_monte_carlo),
        ("verify-inspector-yield-empty", verify_yield_empty),
        ("verify-inspector-corners", verify_corners),
        ("verify-inspector-tuning", verify_tuning),
    ];
    for (name, fixture) in panels {
        for width in [INSPECTOR_DEFAULT_WIDTH, INSPECTOR_MIN_WIDTH] {
            let mut app = fixture();
            crate::ui::raster::write_panel_review(
                &directory,
                &format!("{name}-{width}"),
                crate::ui::Theme::default(),
                Vec2::new(width, 1200.0),
                10.0,
                |ui, _| {
                    let panel = Tokens::get(ui.ctx()).color.bg_panel;
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(panel))
                        .show(ui, |ui| super::super::show(ui, &mut app));
                },
            );
        }
    }
}

/// The Simulate and Verify navigators as PNGs, with their measured geometry.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_simulate_and_verify_navigators_for_review() {
    let directory = review_directory();
    let panels: [ReviewPanel; 3] = [
        ("simulate-navigator", simulate_app),
        ("verify-navigator-empty", verify_yield_empty),
        ("verify-navigator-mc", verify_monte_carlo),
    ];
    for (name, fixture) in panels {
        let mut app = fixture();
        crate::ui::raster::write_panel_review(
            &directory,
            name,
            crate::ui::Theme::default(),
            Vec2::new(NAVIGATOR_WIDTH, 1200.0),
            10.0,
            |ui, _| {
                let panel = Tokens::get(ui.ctx()).color.bg_panel;
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(panel))
                    .show(ui, |ui| super::super::super::navigator::show(ui, &mut app));
            },
        );
    }
}
