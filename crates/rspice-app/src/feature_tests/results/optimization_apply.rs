//! "Apply best to design" on the optimization sheet: the best candidate is
//! written into the plan's design variables, and Undo takes it back.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::apply_best_to_design;
use rspice_app_results::workbench::documents::result_document::test_fixtures::optimization_analysis;
use rspice_app_state::state::{
    DesignVariable, DesignVariableQuantity, DesignVariableSweepEligibility, SimulationRun,
    SimulationRunLifecycle,
};

/// The test instance's plan, holding `variables` as width variables at
/// 1 µ, and showing an optimization whose best width is 1.5 µ.
fn app_with(variables: &[&str]) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .id();
    for &name in variables {
        let variable = DesignVariable::new(
            name,
            "1u",
            DesignVariableQuantity::Dimensionless,
            "channel width",
            None,
            DesignVariableSweepEligibility::FixedParameter,
        )
        .expect("valid design variable");
        app.state
            .model
            .workspace
            .content
            .add_design_variable(plan, variable)
            .expect("the plan accepts it");
    }
    let mut run = SimulationRun::new(1);
    run.add_analysis(optimization_analysis());
    run.lifecycle = SimulationRunLifecycle::Completed;
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app
}

fn expression_of(app: &RSpiceApp, name: &str) -> String {
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("a stable plan")
        .id();
    app.state
        .model
        .workspace
        .content
        .plan_data(plan)
        .expect("the plan's data")
        .design_variables
        .iter()
        .find(|variable| variable.name == name)
        .expect("the variable")
        .expression
        .clone()
}

#[test]
fn apply_best_writes_the_best_candidate_into_the_design_and_undo_takes_it_back() {
    let mut app = app_with(&["W"]);

    apply_best_to_design(&mut app.state.results_parts()).expect("the best candidate applies");
    // The optimizer's own number, read back exactly: 1.5e-6, not 1.5 µ
    // rounded to the reader's digits.
    assert_eq!(expression_of(&app, "W"), "1.5e-6");
    assert_eq!(
        apply_best_to_design(&mut app.state.results_parts()),
        Err("The design already holds the best candidate".to_owned())
    );

    assert!(app.state.model.can_undo_simulation_setup());
    assert!(matches!(
        app.state.model.undo_simulation_setup(),
        Some(Ok(_))
    ));
    assert_eq!(expression_of(&app, "W"), "1u");
}

#[test]
fn a_best_candidate_the_plan_cannot_hold_writes_nothing() {
    let mut app = app_with(&["L"]);

    assert_eq!(
        apply_best_to_design(&mut app.state.results_parts()),
        Err("The active plan has no design variable named w".to_owned())
    );
    assert_eq!(expression_of(&app, "L"), "1u");
    assert!(!app.state.model.can_undo_simulation_setup());
}
