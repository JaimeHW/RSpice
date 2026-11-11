//! The toolbar's plan chip raises the plan manager through the simulate
//! feature's command, and owns no dialog of its own.

use crate::workbench::RSpiceApp;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::Workspace;

/// The chip's whole purpose is the plan manager. Navigating to Simulation
/// Studio instead left the manager reachable only from a button inside it.
#[test]
fn the_plan_chip_owns_no_dialog_construction_of_its_own() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.workspace = Workspace::Design;

    app.execute_command_key(SimulateCommand::ManagePlans.key());

    assert!(matches!(
        app.state.simulate.simulation_workflow,
        Some(crate::workbench::state::SimulationWorkflowDialog::PlanManager(_))
    ));
    assert_ne!(app.state.workbench.workspace, Workspace::Simulate);
}
