//! A run and what follows it, through whole application frames.

use super::veriloga_flow_tests::User;
use super::*;
use crate::workbench::ResultViewer;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::{ConsolePage, Workspace};
use crate::workbench::tools::examples_browser::create_example_project;
use rspice_app_state::workbench::examples::EXAMPLES;

/// An example project with a short transient, the session settled on it.
fn example_project() -> User {
    let mut app = RSpiceApp::test_instance();
    create_example_project(&mut app.state, EXAMPLES[0]).expect("the example opens as a project");
    app.state.model.sim_setup.tran.step = "1n".to_owned();
    app.state.model.sim_setup.tran.stop = "10u".to_owned();
    User::new(app)
}

fn run_has_ended(app: &RSpiceApp, before: usize) -> bool {
    let simulation = &app.state.model.simulation;
    simulation.retained.runs.len() > before
        && !simulation.execution.has_active_execution()
        && !simulation.execution.trigger_simulation
}

fn newest_run(user: &User) -> u64 {
    user.app
        .state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .map(|run| run.id)
        .max()
        .expect("a run is retained")
}

fn active_run(user: &User) -> Option<u64> {
    user.app
        .state
        .model
        .simulation
        .active_run()
        .map(|run| run.id)
}

/// Press Run as the toolbar does and frame until the run has ended.
fn run_to_its_end(user: &mut User) {
    let before = user.app.state.model.simulation.retained.runs.len();
    assert!(user.app.command_key_enabled(SimulateCommand::Run.key()));
    user.app.execute_command_key(SimulateCommand::Run.key());
    user.wait_for("the run", |app| {
        let simulation = &app.state.model.simulation;
        simulation.retained.runs.len() > before
            && !simulation.execution.has_active_execution()
            && !simulation.execution.trigger_simulation
    });
    user.settle();
}

/// The Inspector's Open results link, clicked as a hand does, while the
/// notice of the run's end is still up: the pointer arrives, rests, presses
/// and lets go, on the frames a window delivers them.
#[test]
fn open_results_on_the_inspector_opens_that_run_in_results() {
    let mut user = example_project();
    run_to_its_end(&mut user);
    run_to_its_end(&mut user);
    let simulation = &mut user.app.state.model.simulation;
    let newest = simulation
        .retained
        .runs
        .iter()
        .map(|run| run.id)
        .max()
        .expect("the runs are retained");
    assert!(simulation.select_run_by_sequence(newest - 1));
    user.app.state.workbench.activate(Workspace::Simulate);
    user.settle();
    assert!(!user.app.state.workbench.toasts.activity().is_empty());

    let link = user.expect("Open results").center();
    user.frame(vec![egui::Event::PointerMoved(link)]);
    user.settle();
    user.press(link, egui::PointerButton::Primary);
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app
            .state
            .model
            .simulation
            .active_run()
            .map(|run| run.id),
        Some(newest)
    );
}

/// From Design, Simulation or Results, a run leaves the reader where they are
/// while it runs, and when it completes it opens in Results on Waves, whatever
/// viewer Results last showed.
#[test]
fn a_finished_run_opens_in_results_on_waves_from_any_page() {
    for workspace in [Workspace::Design, Workspace::Simulate, Workspace::Results] {
        let mut user = example_project();
        user.app.state.workbench.activate(workspace);
        user.app.state.results.results.session.viewer = ResultViewer::Op;
        user.settle();

        user.app.execute_command_key(SimulateCommand::Run.key());
        user.wait_for("the run", |app| {
            let ended = run_has_ended(app, 0);
            if !ended {
                assert_eq!(app.state.workbench.workspace, workspace);
            }
            ended
        });
        user.settle();

        assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
        assert_eq!(
            user.app.state.results.results.session.viewer,
            ResultViewer::Waves,
            "pressed on {workspace:?}"
        );
        assert_eq!(active_run(&user), Some(newest_run(&user)));
    }
}

/// A reader who went elsewhere while the run ran is left there, with the
/// finished run one click away on the status bar.
#[test]
fn a_run_that_ends_after_the_reader_moved_on_waits_on_the_status_bar() {
    let mut user = example_project();
    user.app.execute_command_key(SimulateCommand::Run.key());
    user.frame(Vec::new());
    user.app.state.workbench.activate(Workspace::Project);
    user.wait_for("the run", |app| run_has_ended(app, 0));
    user.settle();
    assert_eq!(user.app.state.workbench.workspace, Workspace::Project);

    user.click("Run 1 complete");

    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
    assert_eq!(active_run(&user), Some(1));
    assert_eq!(
        user.find("Run 1 complete"),
        None,
        "the offer has been taken"
    );
}

/// A run the checks refuse says why where it can be read, and the reader
/// stays on their page.
#[test]
fn a_refused_run_shows_its_problem_and_changes_no_page() {
    let mut user = example_project();
    // Two labels the deck folds into one node: an error the checks refuse.
    let schematic = &mut user.app.state.model.schematic;
    schematic.add_net_label(crate::state::Point::new(600, 600), "sense".to_owned());
    schematic.add_net_label(crate::state::Point::new(600, 700), "SENSE".to_owned());
    user.app.state.sync_active_schematic_to_workspace();
    user.settle();
    let workspace = user.app.state.workbench.workspace;

    user.app.execute_command_key(SimulateCommand::Run.key());
    user.wait_for("the checks to refuse the run", |app| {
        app.state
            .runs
            .preflight
            .report
            .as_ref()
            .is_some_and(|report| !report.blockers.is_empty())
    });
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, workspace);
    assert!(user.app.state.workbench.console_visible);
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Problems);
    assert!(user.app.state.model.simulation.retained.runs.is_empty());
    assert_eq!(user.app.state.workbench.run_press_route, None);
}

/// A failed run brings its own account up in the Console and leaves the
/// reader on their page, with nothing offered to open.
#[test]
fn a_failed_run_shows_its_problem_and_changes_no_page() {
    use crate::diagnostics::{LogAnchor, LogSeverity, LogSource};
    use crate::state::SimulationRunLifecycle as Lifecycle;

    let mut user = example_project();
    let workspace = user.app.state.workbench.workspace;
    user.app.state.workbench.run_press_route = Some(user.app.state.workbench.current_route());
    let run = user.app.state.model.simulation.start_run();
    run.lifecycle = Lifecycle::Running;
    user.frame(Vec::new());
    let run = &mut user.app.state.model.simulation.retained.runs[0];
    run.success = false;
    run.finish_lifecycle(Lifecycle::Failed)
        .expect("a running run can fail");
    let line = user.app.state.model.log_buffer.log_anchored(
        LogSeverity::Error,
        LogSource::Run,
        "Run 1 failed: timestep too small",
        Some(LogAnchor::ResultRun { run_sequence: 1 }),
    );
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, workspace);
    assert!(user.app.state.workbench.console_visible);
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Console);
    assert!(
        user.app
            .state
            .workbench
            .console_view
            .selection
            .is_some_and(|selection| selection.contains(line)),
        "the Console opens on the line that says what happened"
    );
    assert_eq!(user.app.state.runs.run_watch.finished_run, None);
}

/// A run with no curve to draw opens where its result can be read.
#[test]
fn an_operating_point_run_opens_on_its_operating_point() {
    let mut user = example_project();
    let plan = user
        .app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a plan");
    let ids = plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>();
    for id in ids {
        plan.set_enabled(id, false).expect("the analysis exists");
    }
    plan.insert(crate::simulation::plan::AnalysisKind::OperatingPoint)
        .expect("an operating point can be planned");
    user.app
        .state
        .model
        .sim_setup
        .refresh_legacy_analysis_projections();
    user.settle();

    run_to_its_end(&mut user);

    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Op
    );
}
