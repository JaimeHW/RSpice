//! A run records what it set the engine to as it starts, and Run details
//! states it: the temperature, the design variables, the options and the
//! simulator, as a run log states them up front.

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::docks::inspector::run_details;
use rspice_app_runs::simulation::controller::test_execution::{
    pvt, run_generated_batch, run_manual_batch, run_manual_batch_at,
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
    run_details_shown_keeping(run, None)
}

/// [`run_details_shown`] with its Identifiers unfolded, in an application
/// that keeps `deck` as the deck the run executed.
fn run_details_shown_keeping(run: SimulationRun, deck: Option<&str>) -> Shown {
    let mut app = RSpiceApp::test_instance();
    let run_id = run.id;
    app.state.model.simulation.retained.runs.push(run);
    if let Some(deck) = deck {
        use rspice_results::executed_deck::{ExecutedDeck, ExecutedDeckPoint};
        app.state
            .model
            .simulation
            .retained
            .executed_decks
            .retain(ExecutedDeck {
                run_id,
                points: vec![ExecutedDeckPoint::new("op", deck.into(), Vec::new())],
            });
    }
    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        run_id,
    );
    app.state
        .results
        .run_details
        .as_mut()
        .expect("Run details is open")
        .show_identifiers();
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

fn value<'a>(rows: &'a [(String, String)], label: &str) -> &'a str {
    rows.iter()
        .find(|(shown, _)| shown == label)
        .map_or("", |(_, value)| value.as_str())
}

fn shows_no(rows: &[(String, String)], label: &str) {
    assert!(
        !rows.iter().any(|(shown, _)| shown == label),
        "Run details shows {label}; it shows {rows:?}"
    );
}

/// Where in the dialog `label`'s row stands, top to bottom.
fn place(rows: &[(String, String)], label: &str) -> usize {
    rows.iter()
        .position(|(shown, _)| shown == label)
        .unwrap_or_else(|| panic!("Run details shows no {label}; it shows {rows:?}"))
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

/// The divider's plan run across two supplies and two temperatures.
fn pvt_run() -> SimulationRun {
    let state = pvt::run_set_divider(
        AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        &["-40", "125"],
    );
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
    assert!(value(rows, "Simulator").starts_with("RSpice "), "{rows:?}");
    // A plan's run is named by its plan, which says what kind of run it was.
    shows(rows, "Plan", &divider_plan_name());
    // What goes without saying is not said: the kind of run beside its plan,
    // the default configuration, the check its source passed, and the engine
    // of the computer it ran on.
    for label in ["Source", "Source check", "Configuration", "Engine"] {
        shows_no(rows, label);
    }
    // The revision it ran is one of its identifiers, among them, which no
    // longer name the job or the source check's digest.
    assert!(place(rows, "Project revision") > place(rows, "Run ID"));
    for label in ["Job ID", "Source-check digest"] {
        shows_no(rows, label);
    }
    // Its duration reads in plain units, with nothing finer than a
    // millisecond.
    let duration = value(rows, "Duration");
    assert!(
        duration.ends_with(" ms") || duration.ends_with(" s") || duration == "Under 1 ms",
        "{duration}"
    );
    assert!(!duration.starts_with("0."), "{duration}");
    // A plan's run read no deck file; the dialog holds no deck for it.
    names_a_row(&shown, "Not kept");
    // A run of one point names its analysis alone, in a section of its own.
    names_a_row(&shown, "Analyses");
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

    let shown = run_details_shown(run);
    let rows = &shown.rows;
    shows(rows, "Temperature", "50 \u{b0}C");
    // Its `.temp` is the one temperature the summary states, and not a
    // sweep of one point among its analyses.
    assert!(
        !shown
            .words
            .iter()
            .any(|words| words.starts_with("Parametric Sweep")),
        "{:?}",
        shown.words
    );
    shows(rows, "rtop", "3k");
    shows(rows, "gain", "2");
    shows(rows, "RELTOL", "1e-4");
    shows(rows, "ABSTOL", "1e-10");
    assert!(value(rows, "Simulator").starts_with("RSpice "), "{rows:?}");
    // A deck no plan produced says what it was, and has no Plan row.
    shows(rows, "Source", "Manual deck");
    shows_no(rows, "Plan");
}

#[test]
fn a_deck_run_names_its_deck_by_its_file_and_its_whole_path_in_the_hover() {
    let source = "* divider
V1 in 0 1
R1 in out 3k
R2 out 0 1k
.op
.end
";
    let path = std::env::temp_dir()
        .join("rspice-run-facts")
        .join("divider.cir");
    let run = run_manual_batch_at(source, &path, SimulationRunLifecycle::Completed);
    let whole = path.display().to_string();
    assert_eq!(
        run.facts.as_ref().map(|facts| facts.deck.as_str()),
        Some(whole.as_str())
    );

    // Kept, the deck is named by its file, and the hover says where it is.
    let shown = run_details_shown_keeping(run.clone(), Some(source));
    names_a_row(&shown, "divider.cir");
    shows(&shown.rows, "Deck", &whole);

    // Let go, it says so, and the hover says why.
    let shown = run_details_shown(run);
    names_a_row(&shown, "divider.cir, not kept");
    assert!(
        value(&shown.rows, "Deck").starts_with(&format!("{whole}; not kept")),
        "{:?}",
        shown.rows
    );
}

#[test]
fn a_pvt_run_states_its_supplies_and_names_each_row_by_its_point() {
    let run = pvt_run();
    let facts = run.facts.as_ref().expect("the run records what it set");
    assert_eq!(facts.supplies, [1.62, 1.8]);

    let shown = run_details_shown(run);
    shows(&shown.rows, "Supply", "1.62 V to 1.8 V, 2 supplies");
    // One corner is no range of corners.
    shows_no(&shown.rows, "Process");
    for point in [
        "1.62 V, \u{2212}40",
        "1.62 V, 125",
        "1.8 V, \u{2212}40",
        "1.8 V, 125",
    ] {
        names_a_row(
            &shown,
            &format!("DC Operating Point \u{b7} {point} \u{b0}C"),
        );
    }
}
