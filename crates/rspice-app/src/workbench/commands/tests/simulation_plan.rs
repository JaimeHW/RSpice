//! Command tests for the simulation plan: the one command that manages the
//! catalog, and the eight routes onto the plan's own records.
//!
//! These belong together because they are the same claim made twice. A plan
//! owns its analyses, variables, outputs, specifications, run set, model
//! closure, solver options and save policy, and each of those is reachable only
//! through a command with a stable identity — including the choice of *which*
//! plan the routes then edit. A route reachable only by clicking a navigator
//! row cannot be bound to a shortcut or driven from automation, and a plan
//! manager reachable only from one surface's title button is the same defect
//! one level up.

use super::*;
use crate::state::command_catalog::ShortcutContext;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::CommandFlags;
use crate::workbench::simulate::commands::SimulateCommand;

/// Every Simulation Studio setup route is addressable, the way the Project,
/// Verify and Models pages are. A route reachable only by clicking the
/// navigator tree cannot be bound to a shortcut or driven from automation.
#[test]
fn every_simulation_setup_route_has_one_discoverable_command_with_a_stable_identity() {
    let expected = [
        (SimulationPage::Analyses, "simulation-analyses"),
        (SimulationPage::Excitations, "simulation-excitations"),
        (SimulationPage::Variables, "simulation-variables"),
        (SimulationPage::Outputs, "simulation-outputs"),
        (SimulationPage::Specifications, "simulation-specifications"),
        (SimulationPage::RunSet, "simulation-run-set"),
        (SimulationPage::Models, "simulation-models"),
        (SimulationPage::Solver, "simulation-solver"),
        (SimulationPage::Save, "simulation-save-policy"),
    ];
    assert_eq!(SimulationPage::NAVIGATION.len(), expected.len());

    let registry = command_registry();
    for (page, stable_id) in expected {
        let command = SimulateCommand::Page(page);
        assert!(
            registry.order().contains(&command.key()),
            "setup route is absent from the command registry: {page:?}"
        );
        assert_eq!(command.key().as_str(), stable_id);
        assert_eq!(registry.resolve(stable_id), Some(command.key()));
        assert_eq!(SimulateCommand::from_key(command.key()), Some(command));
        assert!(
            command
                .declared_spec()
                .flags
                .contains(CommandFlags::REQUIRES_OPEN_PROJECT),
            "setup route bypasses the open-project boundary: {page:?}"
        );
    }
}

#[test]
fn simulation_route_commands_activate_the_simulate_workspace_and_exact_route() {
    for page in SimulationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        app.state.workbench.workspace = Workspace::Results;
        app.state.simulate.simulation_page = SimulationPage::Analyses;

        app.execute_command_key(SimulateCommand::Page(page).key());

        assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
        assert_eq!(app.state.simulate.simulation_page, page);
    }
}

/// The Simulation Plan Manager was reachable only from a button on the
/// Simulation Studio title row: absent from the palette, the shortcut map and
/// every route a global chrome control could take. One command owns it now.
#[test]
fn managing_simulation_plans_opens_the_plan_manager_on_the_active_plan() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let plan_id = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    app.state
        .model
        .sim_setup
        .rename_plan(plan_id, "Regression envelope")
        .expect("rename the active plan");

    let manage = SimulateCommand::ManagePlans.key();
    assert_eq!(
        app.command_availability(manage),
        CommandAvailability::Available
    );
    app.execute_command_key(manage);

    let Some(crate::workbench::state::SimulationWorkflowDialog::PlanManager(draft)) =
        app.state.simulate.simulation_workflow.as_ref()
    else {
        panic!("the plan manager did not open");
    };
    assert_eq!(draft.selected_plan_id, plan_id);
    assert_eq!(draft.name, "Regression envelope");
    assert_eq!(
        draft.mode,
        crate::workbench::state::SimulationPlanManagerMode::Browse
    );
}

/// A project whose plan never migrated to stable analysis identity has nothing
/// for the manager to select, so the global chip that owns this command has to
/// go inert and say why rather than open an empty dialog.
#[test]
fn managing_simulation_plans_is_disabled_without_a_stable_analysis_plan() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.model.sim_setup.analysis_plan = None;

    let manage = SimulateCommand::ManagePlans.key();
    assert!(!app.command_key_enabled(manage));
    assert_eq!(
        app.command_availability(manage),
        CommandAvailability::Disabled(
            "the active simulation plan has no stable analysis identity to manage"
        )
    );

    app.execute_command_key(manage);

    assert!(app.state.simulate.simulation_workflow.is_none());
}

/// The chip lives in the context toolbar, which is chrome in every workspace.
/// A Simulation-workspace shortcut context would withdraw the keyboard route
/// from most of the places the chip itself is still clickable.
#[test]
fn managing_simulation_plans_is_a_global_route_not_a_simulation_workspace_one() {
    let spec = SimulateCommand::ManagePlans.declared_spec();
    assert_eq!(spec.context, ShortcutContext::Global);
    assert_eq!(spec.key.as_str(), "manage-simulation-plans");
    assert_eq!(spec.label, "Manage simulation plans\u{2026}");
    assert_eq!(spec.group, "Simulate");
    let registry = command_registry();
    assert!(registry.order().contains(&spec.key));
    assert!(spec.flags.contains(CommandFlags::PALETTE_VISIBLE));
    assert_eq!(registry.resolve("manage-simulation-plans"), Some(spec.key));
}
