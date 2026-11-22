//! Run on a project whose simulation is its deck, from every workspace: the
//! run control names the deck, Run and Check and netlist take the deck and
//! never check the schematic the project does not have, and a deck whose
//! file is missing says so. A project with a design runs its plan.

use egui::accesskit::Role;

use super::netlist_first::netlist_first_app;
use super::netlist_session::relaunch;
use crate::feature_tests::replay::Replay;
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::state::{SimulationPage, SimulationRunIntent};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::{CommandAvailability, ids};
use rspice_app_workbench::workbench::framework::host::Host;

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

/// Run's menu names what Run runs, as the run control does: Run deck, not
/// the active plan, and no plan options under it. Every list of the
/// commands names it the same, and what would open or edit a plan is off
/// and says why.
#[test]
fn the_run_menu_reads_run_deck_and_offers_nothing_of_a_plan() {
    let mut replay = Replay::of(netlist_first_app());
    replay.app.state.workbench.activate(Workspace::Simulate);
    replay.frames(Vec::new());

    let menu = replay.click("More run commands", Role::Button);
    let entries: Vec<&str> = menu
        .iter()
        .filter(|control| control.role == Role::Button)
        .map(|control| control.label.as_str())
        .collect();
    assert!(
        entries
            .iter()
            .any(|label| *label == "Run deck" || label.starts_with("Run deck (")),
        "the menu's first entry runs the deck: {entries:?}"
    );
    assert!(
        entries
            .iter()
            .any(|label| label.starts_with("Check and netlist")),
        "the deck is checked as Run would check it: {entries:?}"
    );
    for absent in ["Run active plan", "Global solver & convergence"] {
        assert!(
            !entries.iter().any(|label| label.starts_with(absent)),
            "{absent:?} in the run menu: {entries:?}"
        );
    }

    assert_eq!(
        Host::label(&replay.app, SimulateCommand::Run.key()),
        "Run deck"
    );
    for plan in [
        SimulateCommand::ManagePlans,
        SimulateCommand::AddAnalysis,
        SimulateCommand::Options,
        SimulateCommand::Page(SimulationPage::Analyses),
    ] {
        assert_eq!(
            replay.app.command_availability(plan.key()),
            CommandAvailability::Disabled("this project runs its deck, not a simulation plan"),
            "{plan:?}"
        );
    }
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
    assert_eq!(
        Host::label(&app, SimulateCommand::Run.key()),
        "Run active plan"
    );
    app.execute_command_key(ids::RUN_SIMULATION);

    assert!(!deck_run_started(&app), "the deck does not run");
    assert!(
        app.state.runs.preflight.take_run_and_queue_request(),
        "the plan's run is asked for"
    );
}
