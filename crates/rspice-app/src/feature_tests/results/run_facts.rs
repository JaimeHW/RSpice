//! A run records what it set the engine to as it starts, and Run details
//! states it: the temperature, the design variables, the options and the
//! simulator, as a run log states them up front.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::docks::inspector::run_details;
use rspice_app_runs::simulation::controller::test_execution::{
    pvt, run_generated_batch, run_manual_batch,
};
use rspice_app_state::simulation::plan::{AnalysisDraft, AnalysisKind};
use rspice_app_state::state::SimulationRun;
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation_contract::design_variable::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableScope,
    DesignVariableSweepEligibility,
};
use rspice_simulation_contract::design_variable_quantity::DesignVariableQuantity;
use rspice_simulation_contract::run_set::{RunSetDimension, RunSetDimensionKind};

/// The divider fixture with an operating point its only analysis.
fn divider() -> rspice_app_runs::simulation::controller::test_host::RunTestHost {
    let mut state = pvt::divider();
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    let draft = AnalysisDraft::for_kind(AnalysisKind::OperatingPoint);
    let (analysis, _) = plan.insert(draft.kind()).unwrap();
    plan.edit(analysis, |target| *target = draft).unwrap();
    state
}

/// Every row Run details shows for `run`, label and value, once the reader
/// has it open in an application that retains the run.
fn run_details_rows(run: SimulationRun) -> Vec<(String, String)> {
    let mut app = RSpiceApp::test_instance();
    let run_id = run.id;
    app.state.model.simulation.retained.runs.push(run);
    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        run_id,
    );
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            run_details::show(
                ui.ctx(),
                &mut app.state.results.run_details,
                &app.state.model,
            );
        });
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    nodes
        .iter()
        .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
        .collect()
}

fn shows(rows: &[(String, String)], label: &str, value: &str) {
    assert!(
        rows.iter()
            .any(|(shown, words)| shown == label && words == value),
        "Run details does not show {label}: {value}; it shows {rows:?}"
    );
}

fn version(rows: &[(String, String)]) -> &str {
    rows.iter()
        .find(|(label, _)| label == "Version")
        .map_or("", |(_, value)| value.as_str())
}

#[test]
fn a_plan_run_at_85_c_records_and_shows_its_variables_and_options() {
    let mut state = divider();
    state
        .model
        .sim_setup
        .set_reference_pvt(rspice_vocabulary::product::ProcessCorner::TT, 85.0)
        .unwrap();
    state.model.sim_setup.options.reltol = 1e-4;
    let plan = state.model.sim_setup.stable_analysis_plan().unwrap().id();
    let content = &mut state.model.workspace.content;
    content.migrate_active_plan_data(plan);
    let payload = content.plan_data_mut(plan).unwrap();
    for (name, value) in [("rload", "2k"), ("cl", "1p")] {
        payload.design_variables.push(
            DesignVariable::new(
                name.to_owned(),
                value.to_owned(),
                DesignVariableQuantity::Dimensionless,
                DesignVariableScope::Testbench,
                String::new(),
                None,
                DesignVariableSweepEligibility::NestedSweepAndOptimization,
                DesignVariableOverridePolicy::ExplicitTestLocalOverride,
            )
            .unwrap(),
        );
    }

    let run = run_generated_batch(state, SimulationRunLifecycle::Completed);
    let facts = run.facts.as_ref().expect("the run records what it set");
    assert_eq!(facts.temperatures, [85.0]);

    let rows = run_details_rows(run);
    shows(&rows, "Temperature", "85 \u{b0}C");
    shows(&rows, "rload", "2k");
    shows(&rows, "cl", "1p");
    shows(&rows, "RELTOL", "1e-4");
    assert!(
        !rows.iter().any(|(label, _)| label == "PIVTOL"),
        "an option at the product's own default is shown: {rows:?}"
    );
    assert!(version(&rows).starts_with("RSpice "), "{rows:?}");
}

#[test]
fn a_temperature_sweep_shows_its_range() {
    let mut state = divider();
    state.model.sim_setup.run_set.dimensions = vec![RunSetDimension::new(
        "fixture-temperature",
        RunSetDimensionKind::Temperature,
        &["125", "-40", "27"],
        1,
    )];

    let run = run_generated_batch(state, SimulationRunLifecycle::Completed);
    let facts = run.facts.as_ref().expect("the run records what it set");
    assert_eq!(facts.temperatures, [-40.0, 27.0, 125.0]);

    let rows = run_details_rows(run);
    shows(
        &rows,
        "Temperature",
        "-40 \u{b0}C to 125 \u{b0}C, 3 temperatures",
    );
}

#[test]
fn a_manual_deck_shows_what_its_deck_set() {
    let run = run_manual_batch(concat!(
        "* divider\n",
        "V1 in 0 1\n",
        "R1 in out {rtop}\n",
        "R2 out 0 1k\n",
        ".param rtop=3k gain=2\n",
        ".options reltol=1e-4 abstol=1e-10\n",
        ".temp 50\n",
        ".op\n",
        ".end\n",
    ));

    let rows = run_details_rows(run);
    shows(&rows, "Temperature", "50 \u{b0}C");
    shows(&rows, "rtop", "3k");
    shows(&rows, "gain", "2");
    shows(&rows, "RELTOL", "1e-4");
    shows(&rows, "ABSTOL", "1e-10");
    assert!(version(&rows).starts_with("RSpice "), "{rows:?}");
}
