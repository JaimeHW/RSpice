//! A probe placed on the sheet before any run saves its signal in the active
//! test's plan, and that change is a step of the setup's own history: Undo on
//! a Simulate page takes it back, as it takes back an edit made there.

use crate::workbench::RSpiceApp;
use rspice_app_design::schematic::view::gesture_commits::click_probe;
use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
use rspice_app_state::state::Point;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// What the active plan saves for the next run, as the engine spells it.
fn saved_outputs(app: &RSpiceApp) -> Vec<String> {
    let model = &app.state.model;
    let plan = model
        .sim_setup
        .stable_analysis_plan()
        .expect("the plan resolves")
        .id();
    model
        .workspace
        .content
        .plan_data(plan)
        .map(|payload| {
            payload
                .saved_outputs
                .iter()
                .map(|output| output.source_expression.clone())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn a_probe_placed_before_any_run_is_a_setup_step_undone_from_simulate() {
    let host = state_with_current_clean_drc();
    let mut app = RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    // The net PROBED, drawn away from everything else on the sheet.
    let (a, b) = (Point::new(2000, 2000), Point::new(2040, 2000));
    app.state.model.schematic.add_wire(vec![a, b]);
    app.state
        .model
        .schematic
        .add_net_label(a, "PROBED".to_owned());
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Design);
    assert!(app.state.model.simulation.active_run().is_none());
    let before = saved_outputs(&app);

    click_probe(&mut app.state.canvas_frame(), Point::new(2020, 2000));
    app.after_contribution();
    let probed = saved_outputs(&app);
    assert_eq!(
        probed.len(),
        before.len() + 1,
        "the probe saves its net for the next run: {probed:?}"
    );

    app.state.workbench.activate(Workspace::Simulate);
    assert!(
        app.command_key_enabled(ids::UNDO),
        "Undo is offered on a Simulate page"
    );
    app.execute_command_key(ids::UNDO);
    assert_eq!(
        saved_outputs(&app),
        before,
        "Undo there takes the probe's output back"
    );
    app.execute_command_key(ids::REDO);
    assert_eq!(saved_outputs(&app), probed, "and Redo saves it again");
}
