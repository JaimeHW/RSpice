//! What a plan's outputs keep when the plan runs, through the real store and
//! the real run path.

use egui::accesskit::Role;
use rspice_app_state::state::{SavedOutputMaterializationStatus, SimulationRun};

use crate::feature_tests::engineer::Engineer;

/// `20·log10(1/2)`: the gain of two equal resistors in series.
const HALF_IN_DB: f64 = -6.020_599_913_279_624;

/// An expression output runs under every save choice the Add output dialog
/// offers, the first of them its default, and keeps what Results plots:
/// `db(V(out)/V(in))` across a 1 kΩ/1 kΩ divider is 20·log10(1/2) at every
/// point. An output kept on demand is evaluated from the run when Results asks
/// for it; one kept only for a failed run is not written by a run that passed.
#[test]
fn an_expression_output_runs_under_every_save_choice() {
    let mut engineer = Engineer::new_project("gain");
    let e = &mut engineer;
    divider(e);
    add_expression(e, "gain_every", "db(V(out)/V(in))", None);
    add_expression(
        e,
        "gain_selected",
        "db(V(out)/V(in))",
        Some("Selected + final points"),
    );
    add_expression(
        e,
        "gain_demand",
        "db(V(out)/V(in))",
        Some("On demand from retained state"),
    );
    add_expression(
        e,
        "gain_failure",
        "db(V(out)/V(in))",
        Some("Failure diagnostics only"),
    );

    let run = e.run();
    for name in ["gain_every", "gain_selected"] {
        assert_half(&run, name);
    }

    let (analysis, receipt) = receipt_of(&run, "gain_demand");
    assert_eq!(
        run.analyses[analysis].saved_output_receipts[receipt].status,
        SavedOutputMaterializationStatus::Deferred
    );
    let analysis_id = run.analyses[analysis].id;
    e.replay
        .app
        .state
        .model
        .simulation
        .materialize_deferred_saved_output(run.run_id, analysis_id, receipt)
        .expect("Results evaluates the output kept on demand");
    let kept = e
        .replay
        .app
        .state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|kept| kept.run_id == run.run_id)
        .expect("the run is kept")
        .clone();
    assert_half(&kept, "gain_demand");

    let (analysis, receipt) = receipt_of(&run, "gain_failure");
    assert_eq!(
        run.analyses[analysis].saved_output_receipts[receipt].status,
        SavedOutputMaterializationStatus::SuppressedOnSuccess
    );
}

/// The analysis and receipt that hold the output `name` in `run`.
fn receipt_of(run: &SimulationRun, name: &str) -> (usize, usize) {
    run.analyses
        .iter()
        .enumerate()
        .find_map(|(analysis, data)| {
            data.saved_output_receipts
                .iter()
                .position(|receipt| receipt.name == name)
                .map(|receipt| (analysis, receipt))
        })
        .unwrap_or_else(|| panic!("the run holds no receipt for {name}"))
}

/// `name`'s waveform in `run` reads 20·log10(1/2) at every one of its points.
fn assert_half(run: &SimulationRun, name: &str) {
    let waves = || run.analyses.iter().flat_map(|analysis| &analysis.waveforms);
    let wave = waves().find(|wave| wave.name == name).unwrap_or_else(|| {
        panic!(
            "no {name} in the run: {:?}",
            waves().map(|wave| &wave.name).collect::<Vec<_>>()
        )
    });
    let values = wave.y.samples(..).collect::<Vec<f64>>();
    assert!(!values.is_empty(), "{name} holds no samples");
    for value in values {
        assert!(
            (value - HALF_IN_DB).abs() < 1e-6,
            "{name} reads {value} dB, not {HALF_IN_DB}"
        );
    }
}

/// A 1 V source across two 1 kΩ resistors, `in` to `out` to ground, and a
/// transient over it.
fn divider(e: &mut Engineer) {
    e.top_sheet();
    let pins = e.place(
        "voltage source",
        "Voltage Source",
        (100, 100),
        &[("Instance", "vin"), ("DC voltage, V", "1")],
    );
    e.connect(&pins, &["in", "0"]);
    let pins = e.place(
        "resistor",
        "Resistor",
        (250, 100),
        &[("Instance", "r1"), ("Resistance, \u{3a9}", "1k")],
    );
    e.connect(&pins, &["in", "out"]);
    let pins = e.place(
        "resistor",
        "Resistor",
        (400, 100),
        &[("Instance", "r2"), ("Resistance, \u{3a9}", "1k")],
    );
    e.connect(&pins, &["out", "0"]);
    e.transient("1u", "10u");
}

/// Outputs & expressions > Add output…: a derived expression named `name`
/// evaluating `expression`, under the save choice `policy` or the dialog's
/// default, and Add output.
fn add_expression(e: &mut Engineer, name: &str, expression: &str, policy: Option<&str>) {
    e.setup_page("Outputs & expressions");
    e.press("Add output\u{2026}", Role::Button);
    e.press("Output kind", Role::ComboBox);
    e.press("Derived expression", Role::ListBoxOption);
    e.press("Name", Role::TextInput);
    e.type_text(name);
    e.press("Source or expression", Role::MultilineTextInput);
    e.type_text(expression);
    if let Some(policy) = policy {
        e.press("Save policy", Role::ComboBox);
        e.press(policy, Role::ListBoxOption);
    }
    e.press("Add output", Role::Button);
    let model = &e.replay.app.state.model;
    let plan = model.sim_setup.analysis_plan.as_ref().expect("a plan").id();
    let added = model.workspace.content.plan_data(plan).is_some_and(|data| {
        data.saved_outputs
            .iter()
            .any(|output| output.name == name && output.source_expression == expression)
    });
    assert!(added, "{name} = {expression} is one of the plan's outputs");
}
