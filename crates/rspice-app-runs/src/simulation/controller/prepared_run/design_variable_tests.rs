//! A plan's design variables reach the run as the expressions they are: the
//! deck carries each as a `.param` card, the engine resolves one variable
//! from another, and a variable with no value stops the run naming why.

use super::*;
use crate::simulation::controller::test_execution::{pvt, run_generated_batch};
use crate::simulation::controller::test_host::RunTestHost;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::SimulationRun;
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
    DesignVariableSweepEligibility,
};
use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;

/// The divider with its source at `{VCM}` and its upper resistor at
/// `{W2*125e6}`, so `V(out)` is VCM/2 exactly when W2 is 8u, with one
/// operating point to run.
fn divider_reading_variables() -> RunTestHost {
    let mut state = pvt::divider();
    for component in &mut state.model.schematic.document_mut_for_test().components {
        match component.name.as_str() {
            "VDD" => {
                "V1".clone_into(&mut component.name);
                "dc={VCM}".clone_into(&mut component.params);
            }
            "R1" => "{W2*125e6}".clone_into(&mut component.value),
            _ => {}
        }
    }
    let plan = state
        .model
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a plan");
    let (analysis, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("an operating point");
    plan.edit(analysis, |draft| {
        *draft = AnalysisDraft::for_kind(AnalysisKind::OperatingPoint);
    })
    .expect("its settings");
    state
}

/// Add `name = expression` to the plan through the workspace store.
fn add(state: &mut RunTestHost, name: &str, expression: &str, quantity: DesignVariableQuantity) {
    let plan = state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("a plan")
        .id();
    let variable = DesignVariable::new(
        name,
        expression,
        quantity,
        DesignVariableScope::Testbench,
        "",
        None,
        DesignVariableSweepEligibility::NestedSweepAndOptimization,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("a well-formed variable");
    state
        .model
        .workspace
        .content
        .add_design_variable(plan, variable)
        .expect("the store takes the variable");
}

/// Every task deck the run would dispatch, as the controller prepares it.
fn dispatched_decks(state: &mut RunTestHost) -> Vec<String> {
    state.model.simulation.execution.run_intent = SimulationRunIntent::SimulateRunSet;
    state.sync_active_schematic_to_workspace();
    let mut controller = SimulationController::new();
    let snapshot =
        SimulationController::build_prepared_snapshot(&*state, SimulationRunIntent::SimulateRunSet)
            .expect("the plan prepares");
    controller
        .retain_prepared_from(&*state, snapshot)
        .expect("the snapshot is current");
    controller
        .consume_snapshot_for_dispatch(&*state)
        .expect("the run dispatches")
        .into_tasks()
        .into_iter()
        .map(|task| task.executable_netlist().to_string())
        .collect()
}

/// Why the run refuses to start.
fn refusal(state: &mut RunTestHost) -> String {
    state.model.simulation.execution.run_intent = SimulationRunIntent::SimulateRunSet;
    state.sync_active_schematic_to_workspace();
    SimulationController::build_prepared_snapshot(&*state, SimulationRunIntent::SimulateRunSet)
        .err()
        .expect("the run is refused")
        .to_string()
}

fn first_out(run: &SimulationRun) -> f64 {
    run.analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find(|wave| wave.name.eq_ignore_ascii_case("V(out)"))
        .expect("V(out) in the run")
        .y
        .sample(0)
        .expect("a first sample")
}

fn close(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() <= expected.abs() * 1e-9
}

/// `W2 = 2*W1` with `W1 = 4u` and `VCM = VDD/2` with `VDD = 1.0`, entered
/// before the variables they name: the engine reads W2 = 8u and VCM = 0.5,
/// and the operating point it solves with them is VCM/2 at `out`.
#[test]
fn a_variable_naming_another_reaches_the_engine_resolved() {
    let mut state = divider_reading_variables();
    add(
        &mut state,
        "W2",
        "2*W1",
        DesignVariableQuantity::Dimensionless,
    );
    add(&mut state, "VCM", "VDD/2", DesignVariableQuantity::Voltage);
    add(
        &mut state,
        "W1",
        "4u",
        DesignVariableQuantity::Dimensionless,
    );
    add(&mut state, "VDD", "1.0", DesignVariableQuantity::Voltage);

    let decks = dispatched_decks(&mut state);
    assert!(!decks.is_empty());
    for deck in &decks {
        assert!(deck.contains(".param W2={2*W1}"), "{deck}");
        assert!(deck.contains(".param VCM={VDD/2}"), "{deck}");
        let parsed = rspice_core::Netlist::parse(deck).expect("the deck parses");
        assert!(close(parsed.params.get("W2").expect("W2"), 8e-6));
        assert_eq!(parsed.params.get("VCM"), Some(0.5));
    }

    let run = run_generated_batch(state, SimulationRunLifecycle::Completed);
    assert!(close(first_out(&run), 0.25), "{}", first_out(&run));
}

/// Two variables that name each other, or one that names a variable the plan
/// does not have, stop the run, and the refusal names them.
#[test]
fn a_loop_or_an_unknown_name_stops_the_run_naming_it() {
    let mut state = divider_reading_variables();
    add(
        &mut state,
        "W1",
        "W2/2",
        DesignVariableQuantity::Dimensionless,
    );
    add(
        &mut state,
        "W2",
        "2*W1",
        DesignVariableQuantity::Dimensionless,
    );
    add(&mut state, "VCM", "VDD/2", DesignVariableQuantity::Voltage);
    let refused = refusal(&mut state);
    assert!(refused.contains("W1 and W2 name each other"), "{refused}");
    assert!(refused.contains("VDD is not defined"), "{refused}");
}
