//! Run on a project whose simulation is its deck, from every workspace: the
//! run control names the deck, Run and Check and netlist take the deck and
//! never check the schematic the project does not have, and a deck whose
//! file is missing says so. A project with a design runs its plan.

use super::netlist_first::netlist_first_app;
use super::netlist_session::relaunch;
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_state::state::SimulationRunIntent;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::{CommandAvailability, ids};

/// The run control as `app` draws it, by its name and whether it is
/// unavailable; `None` on a workspace whose toolbar carries none, the
/// Project and Results workspaces, where F5 and Simulate > Run reach Run.
fn run_control(app: &mut RSpiceApp) -> Option<(String, bool)> {
    let ctx = themed_context();
    frame(&ctx, app, DESKTOP);
    let output = frame(&ctx, app, DESKTOP);
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
        .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.is_disabled())))
        .find(|(label, _)| label.starts_with("Run deck ") || label.starts_with("Run active"))
}

/// Whether the press started a run of the deck as written.
fn deck_run_started(app: &RSpiceApp) -> bool {
    let execution = &app.state.model.simulation.execution;
    execution.trigger_simulation && execution.run_intent == SimulationRunIntent::ManualDeck
}

/// Whether the press asked for the plan's run, whose first step checks the
/// schematic.
fn plan_run_asked(app: &mut RSpiceApp) -> bool {
    app.state.runs.preflight.take_run_and_queue_request()
        || app.state.runs.preflight.checking.is_some()
}

/// From every workspace but the Netlist one Run runs the deck as written,
/// and the schematic is never checked; where the toolbar draws the run
/// control, it names the deck.
#[test]
fn run_away_from_the_netlist_workspace_runs_the_deck() {
    for workspace in [
        Workspace::Project,
        Workspace::Results,
        Workspace::Design,
        Workspace::Simulate,
    ] {
        let mut app = netlist_first_app();
        app.state.workbench.activate(workspace);

        match run_control(&mut app) {
            Some((label, disabled)) => assert!(
                label.starts_with("Run deck front_end.sp") && !disabled,
                "{workspace:?}: Run names the deck it runs: {label}"
            ),
            None => assert!(
                matches!(workspace, Workspace::Project | Workspace::Results),
                "{workspace:?}: the toolbar draws the run control"
            ),
        }
        app.execute_command_key(ids::RUN_SIMULATION);

        assert!(deck_run_started(&app), "{workspace:?}: Run runs the deck");
        assert!(
            !plan_run_asked(&mut app),
            "{workspace:?}: the schematic is not checked"
        );
        assert_eq!(
            app.state.workbench.workspace, workspace,
            "Run stays where it was pressed"
        );
    }
}

/// A launch that reopens away from the Netlist workspace has the deck's
/// entry, not its text: Run opens it as the Netlist workspace does and runs
/// it. The Project workspace names nothing of the deck; Design and Simulate,
/// which draw the run control, open the deck to list it.
#[test]
fn run_away_from_the_netlist_workspace_opens_a_deck_the_launch_has_not_read() {
    let mut app = netlist_first_app();
    app.state.workbench.activate(Workspace::Project);
    let mut reopened = relaunch(&mut app);
    assert_eq!(reopened.state.workbench.workspace, Workspace::Project);
    assert!(
        reopened.state.code.netlist.open_deck.is_none(),
        "the Project workspace has not opened the deck"
    );

    reopened.execute_command_key(ids::RUN_SIMULATION);

    assert!(
        reopened.state.code.netlist.open_deck.is_some(),
        "Run opened the deck"
    );
    assert!(deck_run_started(&reopened), "Run runs the deck");
    assert!(
        !plan_run_asked(&mut reopened),
        "the schematic is not checked"
    );
}

/// Check and netlist from the Project workspace checks the deck as Run
/// would, and starts nothing.
#[test]
fn check_and_netlist_away_from_the_netlist_workspace_checks_the_deck() {
    let mut app = netlist_first_app();
    app.state.workbench.activate(Workspace::Project);
    assert_eq!(
        app.command_availability(ids::CHECK_AND_NETLIST),
        CommandAvailability::Available
    );

    app.execute_command_key(ids::CHECK_AND_NETLIST);

    let notice = app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(
        notice.starts_with("front_end.sp checked \u{b7} 0 blocking errors"),
        "the notice names the deck it checked: {notice}"
    );
    assert!(!deck_run_started(&app), "the deck does not run");
    assert!(!plan_run_asked(&mut app), "the schematic is not checked");
}

/// A deck whose file went between launches refuses Run from the Project
/// workspace with its missing reason, and Run waits until it is found.
#[test]
fn a_missing_deck_refuses_run_from_the_project_workspace_with_its_reason() {
    let mut app = netlist_first_app();
    let path = app
        .state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("the deck is open")
        .root
        .path
        .clone();
    app.state.workbench.activate(Workspace::Project);
    std::fs::remove_file(&path).expect("the deck's file goes");
    let mut reopened = relaunch(&mut app);

    reopened.execute_command_key(ids::RUN_SIMULATION);

    let notice = reopened.state.prompt_text().unwrap_or_default().to_owned();
    assert_eq!(
        notice,
        format!("front_end.sp is not at {}.", path.display()),
        "the refusal says where the deck was"
    );
    assert!(!deck_run_started(&reopened), "nothing runs");
    assert!(
        !plan_run_asked(&mut reopened),
        "the schematic is not checked"
    );
    assert_ne!(
        reopened.command_availability(ids::RUN_SIMULATION),
        CommandAvailability::Available,
        "Run waits until the deck is found"
    );
    reopened.state.workbench.activate(Workspace::Simulate);
    let (label, disabled) = run_control(&mut reopened).expect("Simulate draws the run control");
    assert!(
        label.starts_with("Run deck front_end.sp") && disabled,
        "Run names the missing deck and waits for it: {label}"
    );
}

/// A project with a design runs its plan from the Design workspace, whatever
/// deck the Netlist workspace holds.
#[test]
fn a_project_with_a_design_runs_its_plan() {
    let mut app = netlist_first_app();
    app.state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Resistor,
        rspice_app_state::state::Point::new(0, 0),
    );
    assert!(!app.state.is_netlist_first_without_schematic());
    app.state.workbench.activate(Workspace::Design);

    let (label, _) = run_control(&mut app).expect("Design draws the run control");
    assert!(
        label.starts_with("Run active"),
        "Run names the plan: {label}"
    );
    app.execute_command_key(ids::RUN_SIMULATION);

    assert!(!deck_run_started(&app), "the deck does not run");
    assert!(
        app.state.runs.preflight.take_run_and_queue_request(),
        "the plan's run is asked for"
    );
}
