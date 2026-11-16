//! The Simulate and Verify docks, painted from seeded state.
//!
//! The Simulate inspector is painted in the three states its last run can be
//! in: complete, failed, and never run, and the Verify inspector beside them.

use egui::Vec2;

use crate::workbench::RSpiceApp;
use rspice_app_simulate::workbench::surfaces::verify::raster::seed_monte_carlo;
use rspice_app_state::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use rspice_app_state::state::VerificationPage;
use rspice_app_state::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisType, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;

const INSPECTOR_DEFAULT_WIDTH: f32 = 312.0;

/// The first analysis of the default plan: the one the inspector selects.
fn first_instance(app: &RSpiceApp) -> AnalysisInstanceId {
    app.state
        .model
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
    runs: &mut rspice_app_state::state::RunHistory,
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
        &mut app.state.model.simulation.retained.runs,
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
        &mut app.state.model.simulation.retained.runs,
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
    app.state.simulate.verification_page = page;
    app
}

/// The Verify fixtures' Monte Carlo run: forty trials of seed 0x1A2B against
/// a 40 dB gain floor, judged into the verification sheet.
fn verify_monte_carlo() -> RSpiceApp {
    let mut app = verify_app(VerificationPage::MonteCarlo);
    seed_monte_carlo(&mut app.state.model);
    app
}

fn verify_yield_empty() -> RSpiceApp {
    verify_app(VerificationPage::MonteCarlo)
}

fn inspector_context() -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
                .show(ui, |ui| {
                    rspice_app_workbench::workbench::docks::inspector::show(ui, app)
                });
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
    retain_run(
        &mut app.state.model.simulation.retained.runs,
        other,
        None,
        1.84,
    );
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
    app.state.model.simulation.retained.runs.push(older);
    app.state.model.simulation.view.active_run_idx = Some(1);

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
        app.state.model.simulation.active_run().map(|run| run.id),
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

/// With no trial selected, the inspector states the campaign the view's
/// statistics came from, and nothing the table, its summary or its detail
/// already show.
#[test]
fn the_verify_inspector_states_the_monte_carlo_run_without_its_dataset_identity() {
    let painted = simulate_text(&mut verify_monte_carlo());

    assert_states(
        &painted,
        &[
            "Monte Carlo",
            "40 / 40",
            "0x1A2B",
            "Pseudo-random",
            "12.4 s",
        ],
        &[
            "Run definition",
            "Wilson",
            "Dataset",
            "complete",
            "CI",
            "Worst samples",
            "Specification yields",
        ],
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
        VerificationPage::Checks,
        VerificationPage::Reference,
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
