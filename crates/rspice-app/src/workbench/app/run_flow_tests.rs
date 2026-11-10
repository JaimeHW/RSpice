//! A run and what follows it, through whole application frames.

use super::veriloga_flow_tests::User;
use super::*;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::Workspace;
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
