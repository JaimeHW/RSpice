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
    runs: &mut crate::state::RunHistory,
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
    runs.insert(0, run);
}

fn simulate_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    app
}

fn simulate_last_run_ok() -> RSpiceApp {
    let mut app = simulate_app();
    let instance = first_instance(&app);
    retain_run(
        &mut app.state.simulation.retained.runs,
        instance,
        None,
        1.84,
    );
    app
}

fn simulate_last_run_failed() -> RSpiceApp {
    let mut app = simulate_app();
    let instance = first_instance(&app);
    retain_run(
        &mut app.state.simulation.retained.runs,
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

fn inspector_context() -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// One frame of the inspector dock at its default width, fed `events`: every
/// run of text it painted, with where it landed.
fn simulate_pass(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::Shape::Text(text) if !text.galley.text().trim().is_empty() => out.push((
                text.galley.text().to_owned(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => {}
        }
    }
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(INSPECTOR_DEFAULT_WIDTH, 800.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| super::super::show(ui, app));
        },
    );
    let mut painted = Vec::new();
    for shape in &output.shapes {
        walk(&shape.shape, &mut painted);
    }
    painted
}

fn simulate_text(app: &mut RSpiceApp) -> Vec<String> {
    let ctx = inspector_context();
    simulate_pass(&ctx, app, Vec::new());
    simulate_pass(&ctx, app, Vec::new())
        .into_iter()
        .map(|(text, _)| text)
        .collect()
}

/// What the old inspector stated about an analysis, none of which is the
/// last run: the setup is the Analyses page's, and the identity is internal.
const SETUP_ROWS: [&str; 10] = [
    "Name",
    "Type",
    "Instance",
    "Configuration",
    "Run set",
    "Revision",
    "Prerequisites",
    "Configuration valid",
    "Execution context",
    "Engine status",
];

fn assert_no_setup_rows(painted: &[String]) {
    for row in SETUP_ROWS {
        assert!(
            !painted.iter().any(|text| text == row),
            "the Simulate inspector still states {row:?}: {painted:?}"
        );
    }
}

#[test]
fn a_completed_last_run_states_its_status_runtime_and_results_link() {
    let mut app = simulate_last_run_ok();
    let painted = simulate_text(&mut app);

    for expected in ["Status", "Complete", "Runtime", "1.84 s", "Open results"] {
        assert!(
            painted.iter().any(|text| text == expected),
            "{expected:?} is missing: {painted:?}"
        );
    }
    assert_no_setup_rows(&painted);
}

#[test]
fn a_failed_last_run_states_the_failure_and_the_engine_message() {
    let mut app = simulate_last_run_failed();
    let painted = simulate_text(&mut app);

    for expected in [
        "Failed",
        "Timestep too small at t = 1.25 µs",
        "Runtime",
        "420 ms",
    ] {
        assert!(
            painted.iter().any(|text| text == expected),
            "{expected:?} is missing: {painted:?}"
        );
    }
    assert!(!painted.iter().any(|text| text == "Complete"));
    assert_no_setup_rows(&painted);
}

#[test]
fn an_analysis_never_run_says_so_and_offers_no_results() {
    let mut app = simulate_app();
    let painted = simulate_text(&mut app);

    assert!(
        painted.iter().any(|text| text == "Not run yet."),
        "{painted:?}"
    );
    for absent in ["Status", "Runtime", "Open results"] {
        assert!(
            !painted.iter().any(|text| text == absent),
            "{absent:?} is stated for a run that never happened: {painted:?}"
        );
    }
    assert_no_setup_rows(&painted);
}

#[test]
fn a_run_of_another_analysis_is_not_this_ones_last_run() {
    let mut app = simulate_app();
    let other = AnalysisInstanceId::new();
    assert_ne!(other, first_instance(&app));
    retain_run(&mut app.state.simulation.retained.runs, other, None, 1.84);
    let painted = simulate_text(&mut app);

    assert!(
        painted.iter().any(|text| text == "Not run yet."),
        "{painted:?}"
    );
}

#[test]
fn open_results_selects_the_last_run_in_the_results_workspace() {
    let mut app = simulate_last_run_ok();
    let mut older = SimulationRun::new(11);
    older
        .restore_lifecycle(SimulationRunLifecycle::Completed, 0.5)
        .expect("a finite elapsed time");
    app.state.simulation.retained.runs.push(older);
    app.state.simulation.view.active_run_idx = Some(1);

    let ctx = inspector_context();
    simulate_pass(&ctx, &mut app, Vec::new());
    let link = simulate_pass(&ctx, &mut app, Vec::new())
        .into_iter()
        .find(|(text, _)| text == "Open results")
        .map(|(_, rect)| rect.center())
        .expect("the results link is painted");
    simulate_pass(
        &ctx,
        &mut app,
        vec![
            egui::Event::PointerMoved(link),
            egui::Event::PointerButton {
                pos: link,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ],
    );
    simulate_pass(
        &ctx,
        &mut app,
        vec![egui::Event::PointerButton {
            pos: link,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.simulation.active_run().map(|run| run.id),
        Some(12)
    );
}

fn assert_states(painted: &[String], present: &[&str], absent: &[&str]) {
    for expected in present {
        assert!(
            painted.iter().any(|text| text == expected),
            "{expected:?} is missing: {painted:?}"
        );
    }
    for gone in absent {
        assert!(
            !painted.iter().any(|text| text.contains(gone)),
            "{gone:?} is still stated: {painted:?}"
        );
    }
}

#[test]
fn the_verify_inspector_states_the_monte_carlo_run_without_its_dataset_identity() {
    let painted = simulate_text(&mut verify_monte_carlo());

    assert_states(
        &painted,
        &[
            "Monte Carlo",
            "40 / 40",
            "95% CI 80.14–97.42%",
            "12.4 s",
            "Worst samples",
            "observed 38.90 dB",
            "Specification yields",
        ],
        &["Run definition", "Wilson", "Dataset", "complete"],
    );
}

#[test]
fn the_verify_inspector_says_once_that_no_monte_carlo_ran() {
    let painted = simulate_text(&mut verify_yield_empty());

    assert_states(
        &painted,
        &["Monte Carlo", "No Monte Carlo run"],
        &[
            "Unavailable",
            "Seed",
            "Worst samples",
            "Specification yields",
            "No active",
        ],
    );
}

#[test]
fn the_verify_inspector_restates_nothing_the_other_pages_show() {
    for page in [
        VerificationPage::Corners,
        VerificationPage::Optimization,
        VerificationPage::DeviceSafety,
        VerificationPage::Regression,
    ] {
        let painted = simulate_text(&mut verify_app(page));
        assert_states(
            &painted,
            &[],
            &[
                "details",
                "Points",
                "Signals",
                "Execution",
                "State",
                "Geometry",
                "Retained runs",
                "Baseline",
                "Verdict",
            ],
        );
    }
}

#[test]
fn the_tuning_inspector_states_only_uncommitted_and_invalid_values() {
    let painted = simulate_text(&mut verify_tuning());

    assert_states(
        &painted,
        &["Tuning"],
        &[
            "Parameter tuning",
            "committed baseline",
            "Design variables",
            "Provisional changes",
            "Validation",
            "Commit contract",
        ],
    );
}

/// One review panel: its file name and the app state it renders.
type ReviewPanel = (&'static str, fn() -> RSpiceApp);

fn review_directory() -> std::path::PathBuf {
    std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from)
}

/// The Simulate and Verify inspectors as PNGs, with their measured geometry
/// beside each, at the dock's default and minimum widths.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_simulate_and_verify_inspectors_for_review() {
    let directory = review_directory();
    let panels: [ReviewPanel; 7] = [
        ("simulate-inspector-last-run-ok", simulate_last_run_ok),
        (
            "simulate-inspector-last-run-failed",
            simulate_last_run_failed,
        ),
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
