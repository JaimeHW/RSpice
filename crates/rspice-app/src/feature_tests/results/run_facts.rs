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

/// What Run details shows for `run` once the reader has it open in an
/// application that retains the run: every fact row, label and value, and
/// every line of words it paints.
struct Shown {
    rows: Vec<(String, String)>,
    words: Vec<String>,
}

fn painted_words(shapes: &[egui::epaint::ClippedShape]) -> Vec<String> {
    fn walk(shape: &egui::Shape, words: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => words.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, words);
                }
            }
            _ => {}
        }
    }
    let mut words = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut words);
    }
    words
}

fn run_details_shown(run: SimulationRun) -> Shown {
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
    let mut words = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            run_details::show(
                ui.ctx(),
                &mut app.state.results.run_details,
                &app.state.model,
            );
        });
        words = painted_words(&output.shapes);
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    Shown {
        rows: nodes
            .iter()
            .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
            .collect(),
        words,
    }
}

fn run_details_rows(run: SimulationRun) -> Vec<(String, String)> {
    run_details_shown(run).rows
}

fn names_a_row(shown: &Shown, name: &str) {
    assert!(
        shown.words.iter().any(|words| words == name),
        "Run details names no analysis row {name}; it paints {:?}",
        shown.words
    );
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

/// The name the divider's plan has.
fn divider_plan_name() -> String {
    divider().model.sim_setup.active_plan_name().to_string()
}

/// Add the design variable `name` at `value` to `state`'s plan.
fn add_design_variable(
    state: &mut rspice_app_runs::simulation::controller::test_host::RunTestHost,
    name: &str,
    value: &str,
) {
    let plan = state.model.sim_setup.stable_analysis_plan().unwrap().id();
    let content = &mut state.model.workspace.content;
    content.migrate_active_plan_data(plan);
    content.plan_data_mut(plan).unwrap().design_variables.push(
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

/// The divider's plan run at 85 °C, with two design variables and a
/// relative tolerance tighter than the default.
fn plan_run_at_85_c() -> SimulationRun {
    let mut state = divider();
    state
        .model
        .sim_setup
        .set_reference_pvt(rspice_vocabulary::product::ProcessCorner::TT, 85.0)
        .unwrap();
    state.model.sim_setup.options.reltol = 1e-4;
    for (name, value) in [("rload", "2k"), ("cl", "1p")] {
        add_design_variable(&mut state, name, value);
    }

    run_generated_batch(state, SimulationRunLifecycle::Completed)
}

/// The divider's plan run across three temperatures, given out of order.
fn temperature_sweep_run() -> SimulationRun {
    let mut state = divider();
    state.model.sim_setup.run_set.dimensions = vec![RunSetDimension::new(
        "fixture-temperature",
        RunSetDimensionKind::Temperature,
        &["125", "-40", "27"],
        1,
    )];
    run_generated_batch(state, SimulationRunLifecycle::Completed)
}

/// The divider's plan run with its design variable `rload` stepped across
/// two values.
fn design_variable_sweep_run() -> SimulationRun {
    let mut state = divider();
    add_design_variable(&mut state, "rload", "2k");
    let mut axis = RunSetDimension::new(
        "fixture-rload",
        RunSetDimensionKind::Parameter,
        &["1k", "2k"],
        1,
    );
    "design-variable:rload".clone_into(&mut axis.source);
    state.model.sim_setup.run_set.dimensions = vec![axis];
    run_generated_batch(state, SimulationRunLifecycle::Completed)
}

#[test]
fn a_plan_run_at_85_c_records_and_shows_its_variables_and_options() {
    let run = plan_run_at_85_c();
    let facts = run.facts.as_ref().expect("the run records what it set");
    assert_eq!(facts.temperatures, [85.0]);
    assert_eq!(facts.plan, divider_plan_name());

    let shown = run_details_shown(run);
    let rows = &shown.rows;
    shows(rows, "Temperature", "85 \u{b0}C");
    shows(rows, "rload", "2k");
    shows(rows, "cl", "1p");
    shows(rows, "RELTOL", "1e-4");
    assert!(
        !rows.iter().any(|(label, _)| label == "PIVTOL"),
        "an option at the product's own default is shown: {rows:?}"
    );
    assert!(version(rows).starts_with("RSpice "), "{rows:?}");
    // Source says what kind of run it was, Plan which plan ran it.
    shows(rows, "Source", "Simulation plan");
    shows(rows, "Plan", &divider_plan_name());
    // A run of one point names its analysis alone.
    names_a_row(&shown, "DC Operating Point");
}

#[test]
fn a_temperature_sweep_shows_its_range_and_names_each_row_by_its_temperature() {
    let run = temperature_sweep_run();
    let facts = run.facts.as_ref().expect("the run records what it set");
    assert_eq!(facts.temperatures, [-40.0, 27.0, 125.0]);

    let shown = run_details_shown(run);
    shows(
        &shown.rows,
        "Temperature",
        "\u{2212}40 \u{b0}C to 125 \u{b0}C, 3 temperatures",
    );
    for temperature in ["\u{2212}40", "27", "125"] {
        names_a_row(
            &shown,
            &format!("DC Operating Point \u{b7} {temperature} \u{b0}C"),
        );
    }
}

#[test]
fn a_design_variable_sweep_names_each_row_by_its_value() {
    let shown = run_details_shown(design_variable_sweep_run());
    shows(&shown.rows, "rload", "1k to 2k, 2 values");
    for value in ["1k", "2k"] {
        names_a_row(&shown, &format!("DC Operating Point \u{b7} rload={value}"));
    }
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
    // A deck no plan produced has no Plan row.
    shows(&rows, "Source", "Manual deck");
    assert!(
        !rows.iter().any(|(label, _)| label == "Plan"),
        "a manual deck's run names a plan: {rows:?}"
    );
}
