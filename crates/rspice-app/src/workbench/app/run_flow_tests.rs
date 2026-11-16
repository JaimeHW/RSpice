//! A run and what follows it, through whole application frames.

use super::veriloga_flow_tests::User;
use super::*;
use crate::workbench::session::project_transactions::create_example_project;
use rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_results::workbench::documents::result_document::waveform_plotted;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::workbench::examples::EXAMPLES;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::ConsolePage;
use rspice_results::result_presentation::SourceWaveformPresentationKey;

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
/// The status bar's run segment, by the name it is spoken by.
fn run_segment(user: &User) -> Option<String> {
    user.named
        .iter()
        .map(|(name, _)| name)
        .find(|name| name.contains("Run 1 complete \u{b7}"))
        .cloned()
}

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

/// A reader who went elsewhere while the run ran is left there. The run's
/// end is announced in the window's corner with the way to its results, and
/// the status bar's run segment states it and opens its details.
#[test]
fn a_run_that_ends_after_the_reader_moved_on_is_announced_in_the_corner() {
    let mut user = example_project();
    user.app.execute_command_key(SimulateCommand::Run.key());
    user.frame(Vec::new());
    user.app.state.workbench.activate(Workspace::Project);
    user.wait_for("the run", |app| run_has_ended(app, 0));
    user.settle();
    assert_eq!(user.app.state.workbench.workspace, Workspace::Project);

    let segment = run_segment(&user).expect("the run segment states the outcome");
    user.click(&segment);
    assert_eq!(
        user.app
            .state
            .results
            .run_details
            .as_ref()
            .map(|details| details.run_id),
        Some(1),
        "the run segment opens Run 1's details"
    );
    user.app.state.results.run_details = None;
    user.settle();

    let offer = user
        .named
        .iter()
        .find(|(name, _)| name.starts_with("Open in Results"))
        .map(|(name, _)| name.clone())
        .expect("the corner notice offers the run's results");
    user.click(&offer);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
    assert_eq!(active_run(&user), Some(1));
}

/// A reader who stayed on the page they pressed Run on is taken to the
/// results, and nothing floats over them: the run segment states the
/// outcome, and Open in Results is not offered where the results are.
#[test]
fn a_run_whose_results_are_on_screen_raises_no_notice() {
    let mut user = example_project();
    run_to_its_end(&mut user);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.starts_with("Open in Results")),
        "{:?}",
        user.named.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    let segment = run_segment(&user).expect("the run segment states the outcome");
    let elapsed = rspice_app_state::quantity::format_elapsed(
        user.app.state.model.simulation.retained.runs[0].elapsed_time,
    );
    assert!(
        segment.ends_with(&format!("Run 1 complete \u{b7} 1 warning \u{b7} {elapsed}")),
        "{segment}"
    );
    let kept = user.app.state.workbench.toasts.activity();
    assert_eq!(
        kept[0].title(),
        "Run 1 complete with 1 warning",
        "the bell keeps it"
    );
    assert_eq!(kept[0].action(), None);
}

/// A run the checks refuse says why where it can be read, and the reader
/// stays on their page.
#[test]
fn a_refused_run_shows_its_problem_and_changes_no_page() {
    let mut user = example_project();
    // Two labels the deck folds into one node: an error the checks refuse.
    let schematic = &mut user.app.state.model.schematic;
    schematic.add_net_label(
        rspice_app_state::state::Point::new(600, 600),
        "sense".to_owned(),
    );
    schematic.add_net_label(
        rspice_app_state::state::Point::new(600, 700),
        "SENSE".to_owned(),
    );
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
    use rspice_app_state::diagnostics::{LogAnchor, LogSeverity, LogSource};
    use rspice_app_state::state::SimulationRunLifecycle as Lifecycle;

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
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.starts_with("Open in Results")),
        "nothing is offered to open"
    );
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
    plan.insert(rspice_app_state::simulation::plan::AnalysisKind::OperatingPoint)
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

/// The Results page on Waves after a run, as a completed run opens it.
fn results_after_a_run() -> User {
    let mut user = example_project();
    run_to_its_end(&mut user);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    user.app.state.workbench.navigator_visible = true;
    user.app.state.workbench.inspector_visible = true;
    user.settle();
    user
}

/// The presentation key of the active run's signal `name`, and whether the
/// run drew it by default.
fn signal(user: &User, name: &str) -> (SourceWaveformPresentationKey, bool) {
    let run = user
        .app
        .state
        .model
        .simulation
        .active_run()
        .expect("a run is open");
    run.analyses
        .iter()
        .find_map(|analysis| {
            let waveform = analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == name)?;
            Some((
                SourceWaveformPresentationKey::new(
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    name,
                ),
                waveform.visible,
            ))
        })
        .expect("the run holds the signal")
}

/// Whether the active run's signal `name` is on the plot.
fn plotted(user: &User, name: &str) -> bool {
    let (key, default) = signal(user, name);
    waveform_plotted(
        &user.app.state.model,
        &user.app.state.results.results,
        &key,
        name,
        default,
    )
}

/// A signal of the active run that is not on the plot.
fn unplotted_signal(user: &User) -> String {
    let run = user
        .app
        .state
        .model
        .simulation
        .active_run()
        .expect("a run is open");
    let names = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            analysis
                .waveforms
                .iter()
                .map(|waveform| waveform.name.clone())
        })
        .collect::<Vec<_>>();
    names
        .into_iter()
        .find(|name| !plotted(user, name))
        .expect("a signal is off the plot")
}

fn selected_signal(user: &User) -> Option<String> {
    user.app
        .state
        .results
        .results
        .valid_selected_trace(&user.app.state.model.simulation)
        .map(|selected| selected.source_name().to_owned())
}

/// A tick plots its signal at once and unticking removes it, and the row's
/// tick, the strip's legend and the Inspector say the same thing throughout.
#[test]
fn ticking_a_signal_plots_it_and_unticking_removes_it() {
    let mut user = results_after_a_run();
    let name = unplotted_signal(&user);
    let tick = format!("Plot {name}");
    let chip = format!("Toggle {name} visibility");
    assert_eq!(user.ticked(&tick), Some(false));

    user.click(&tick);
    assert!(plotted(&user, &name), "a tick plots at once");
    assert_eq!(user.ticked(&tick), Some(true));
    assert_eq!(user.ticked(&chip), Some(true), "the legend shows it");
    assert_eq!(selected_signal(&user), None, "a tick selects nothing");

    user.click(&tick);
    assert!(!plotted(&user, &name), "unticking removes it");
    assert_eq!(user.ticked(&tick), Some(false));
    assert_ne!(user.ticked(&chip), Some(true), "and the legend lets it go");
}

/// A click on a row selects it for the Inspector and plots nothing; a double
/// click plots it and shows its statistics. The Inspector's button toggles
/// the same membership the tick shows.
#[test]
fn a_click_selects_a_signal_and_a_double_click_plots_and_inspects_it() {
    let mut user = results_after_a_run();
    let name = unplotted_signal(&user);
    let tick = format!("Plot {name}");
    // On the row's name, just right of its tick: the Inspector's list of
    // saved outputs prints the same name.
    let tick_rect = user.expect(&tick);
    let row = egui::pos2(tick_rect.right() + 24.0, tick_rect.center().y);

    user.press(row, egui::PointerButton::Primary);
    user.settle();
    assert_eq!(selected_signal(&user).as_deref(), Some(name.as_str()));
    assert!(!plotted(&user, &name), "a click plots nothing");
    assert!(user.find("Show trace").is_some());

    // Past the double-click interval, so the next two presses are their own.
    for _ in 0..30 {
        user.frame(Vec::new());
    }
    user.press(row, egui::PointerButton::Primary);
    user.press(row, egui::PointerButton::Primary);
    user.settle();
    assert!(plotted(&user, &name), "a double click plots it");
    assert_eq!(selected_signal(&user).as_deref(), Some(name.as_str()));
    assert!(user.app.state.workbench.inspector_visible);
    assert!(user.find("Mean").is_some(), "with its statistics");
    assert_eq!(user.ticked(&tick), Some(true));

    user.click("Hide trace");
    assert!(
        !plotted(&user, &name),
        "the Inspector's button takes it off"
    );
    assert_eq!(user.ticked(&tick), Some(false), "and the tick follows");
    user.click("Show trace");
    assert!(plotted(&user, &name), "and puts it back");
    assert_eq!(user.ticked(&tick), Some(true));
}
