//! Commands gated on the executor, and on the verdicts of what it ran.
//!
//! Ownership is the shared invariant. The executor owns the active run, so
//! stopping follows the target's cancellation capability and clearing results
//! cannot remove the run underneath it. A verification route opens its page,
//! and stepping through violations
//! keeps advancing rather than restarting once a jump has changed workspace.

use super::*;

#[test]
fn tuning_command_opens_the_transactional_sandbox() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    app.state.workbench.verification_page = VerificationPage::Yield;
    let command = Command::VerificationPage(VerificationPage::Tuning);

    assert!(command.is_enabled(&app));
    assert_eq!(command.availability(&app), CommandAvailability::Available);
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Verify);
    assert_eq!(
        app.state.workbench.verification_page,
        VerificationPage::Tuning
    );
}

#[test]
fn clear_results_cannot_remove_the_executor_owned_run() {
    let mut app = RSpiceApp::test_instance();
    let run = app.state.simulation.start_run();
    run.mark_running().unwrap();
    let identity = run.execution_identity().unwrap();
    app.state.simulation.execution.active_execution = Some(identity);
    app.state.simulation.execution.is_running = true;

    assert!(!Command::ClearResults.is_enabled(&app));
    assert_eq!(
        Command::ClearResults.availability(&app),
        CommandAvailability::Disabled("an active simulation execution still owns result history")
    );

    Command::ClearResults.execute(&mut app);

    assert!(
        app.state
            .simulation
            .retained
            .run_by_stable_id(identity.run_id)
            .is_some()
    );
}

#[test]
fn stop_command_follows_the_execution_target_capability() {
    let mut simulation = crate::state::SimulationState::default();
    assert!(!stop_simulation_enabled(&simulation));
    let identity = simulation
        .start_run()
        .execution_identity()
        .expect("current run has execution identity");
    simulation.execution.active_execution = Some(identity);
    simulation.execution.is_running = false;
    assert_eq!(
        stop_simulation_enabled(&simulation),
        crate::simulation::execution::execution_target_supports_cancellation()
    );

    simulation.request_abort_active_run().unwrap();
    assert!(!stop_simulation_enabled(&simulation));
}

#[test]
fn run_controls_follow_stable_execution_ownership_through_cancellation() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let identity = app
        .state
        .simulation
        .start_run()
        .execution_identity()
        .expect("current run has execution identity");
    app.state.simulation.execution.active_execution = Some(identity);
    app.state.simulation.execution.is_running = false;

    assert!(!Command::RunSimulation.is_enabled(&app));
    assert!(Command::StopSimulation.is_enabled(&app));

    app.state.simulation.request_abort_active_run().unwrap();
    assert!(!Command::StopSimulation.is_enabled(&app));
    assert_eq!(
        Command::StopSimulation.availability(&app),
        CommandAvailability::Disabled("simulation cancellation is already in progress")
    );
}

#[test]
fn repeated_violation_navigation_keeps_advancing_after_jump_to_design() {
    use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    let mut result = DrcResult::new();
    for (id, x) in [(1, 10.0), (2, 20.0)] {
        result.add_violation(DrcViolation::new(
            id,
            DrcViolationType::DanglingWire,
            format!("anchored finding {id}"),
            DrcLocation::Point { x, y: 0.0 },
        ));
    }
    result.completed = true;
    // Published through the canonical owner, which is what gives each finding
    // the identity the tour and the Problems grid share.
    app.state
        .publish_active_design_check_result(result)
        .expect("the fixture publishes a current receipt");
    app.state.refresh_active_design_check_projection();
    app.state.workbench.activate(Workspace::Verify);

    let subject = app.state.workspace.content.active_schematic_reference();
    let identities = app.state.design_rule_finding_identities(&subject);
    assert_eq!(identities.len(), 2, "both findings were published");
    for expected in identities {
        Command::NextViolation.execute(&mut app);
        assert_eq!(app.state.workbench.workspace, Workspace::Design);
        assert_eq!(
            app.state.diagnostics.current_finding(),
            Some(expected),
            "advancing the tour moves the one current finding",
        );
        assert!(app.state.schematic.session.editor.center_request.is_some());
    }
}
