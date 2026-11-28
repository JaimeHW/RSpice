//! What a plan's outputs keep when the plan runs, through the real store and
//! the real run path.

use egui::accesskit::Role;
use rspice_app_state::state::{Point, SavedOutputMaterializationStatus, SimulationRun};

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

/// A run on the plan's default save keeps every node voltage of a design
/// with 200 nets, at every level of its hierarchy, and every source's
/// current: two instances of a 99-resistor ladder, each ending on a 0 V
/// source, in series across a 1 V supply.
#[test]
fn a_default_run_keeps_every_node_voltage_and_source_current_at_every_level() {
    let mut engineer = Engineer::new_project("ladders");
    let e = &mut engineer;
    e.schematic_cell("ladder");
    ladder(e, RUNG_NETS);
    e.pins("a", "Input", &[(100, 400)]);
    e.pins("b", "Output", &[(250, 400)]);
    e.symbol_from_schematic("ladder");

    e.top_sheet();
    let pins = e.place(
        "voltage source",
        "Voltage Source",
        (100, 100),
        &[("Instance", "vin"), ("DC voltage, V", "1")],
    );
    e.connect(&pins, &["in", "0"]);
    for (instance, at, nets) in [
        ("x1", (500, 100), ["in", "mid"]),
        ("x2", (900, 100), ["mid", "0"]),
    ] {
        let pins = e.place("ladder", "ladder", at, &[("Instance", instance)]);
        for (port, at) in &pins {
            let net = if port.eq_ignore_ascii_case("a") {
                nets[0]
            } else {
                nets[1]
            };
            if net == "0" {
                e.ground(*at);
            } else {
                e.label(*at, net);
            }
        }
    }
    e.transient("1u", "2u");

    let run = e.run();
    let names = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .map(|wave| wave.name.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>();
    let mut missing = Vec::new();
    let mut nodes = vec!["v(in)".to_owned(), "v(mid)".to_owned()];
    for instance in ["x1", "x2"] {
        nodes.extend((1..=RUNG_NETS).map(|net| format!("v({instance}.n{net})")));
    }
    assert_eq!(nodes.len(), 200);
    for node in &nodes {
        if !names.contains(node) {
            missing.push(node.clone());
        }
    }
    for current in ["i(vin)", "i(x1.vm)", "i(x2.vm)"] {
        if !names.contains(current) {
            missing.push(current.to_owned());
        }
    }
    assert!(
        missing.is_empty(),
        "the run lacks {} of the design's node voltages and source currents: {:?}; it holds {:?}",
        missing.len(),
        missing.iter().take(12).collect::<Vec<_>>(),
        {
            let mut held = names.iter().collect::<Vec<_>>();
            held.sort();
            held.into_iter().take(40).collect::<Vec<_>>()
        }
    );
}

/// Nets inside one ladder: `a`, then `n1` to `n99` joined by resistors, then
/// a 0 V source `vm` from `n99` to `b`.
const RUNG_NETS: usize = 99;

/// The sheet on show drawn as a ladder of `nets` internal nets: a resistor
/// between each pair of neighbouring nets from `a` to `n<nets>`, laid in
/// through the store's own editing calls, and a 0 V source `vm` on to `b`.
fn ladder(e: &mut Engineer, nets: usize) {
    use rspice_app_state::state::ComponentType;
    let net = |index: usize| {
        if index == 0 {
            "a".to_owned()
        } else {
            format!("n{index}")
        }
    };
    for index in 0..nets {
        let at = Point::new(
            1000 + 200 * i32::try_from(index % 10).unwrap(),
            100 + 200 * i32::try_from(index / 10).unwrap(),
        );
        let id = e
            .replay
            .app
            .state
            .model
            .schematic
            .add_component(ComponentType::Resistor, at);
        let part = e
            .replay
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .cloned()
            .expect("the resistor");
        let pins = e.pins_of(&part);
        assert_eq!(pins.len(), 2, "a resistor has two pins");
        let schematic = &mut e.replay.app.state.model.schematic;
        schematic.add_net_label(pins[0].1, net(index));
        schematic.add_net_label(pins[1].1, net(index + 1));
    }
    let pins = e.place(
        "voltage source",
        "Voltage Source",
        (100, 100),
        &[("Instance", "vm"), ("DC voltage, V", "0")],
    );
    e.connect(&pins, &[&net(nets), "b"]);
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
    add_output(e, "Derived expression", name, expression, policy);
}

/// An output of `kind` added from Outputs' Add output dialog.
fn add_output(e: &mut Engineer, kind: &str, name: &str, expression: &str, policy: Option<&str>) {
    e.setup_page("Outputs & expressions");
    e.press("Add output\u{2026}", Role::Button);
    e.press("Output kind", Role::ComboBox);
    e.press(kind, Role::ListBoxOption);
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

/// An output naming a net the design lacks is said on its row and by Check
/// before any run, and pressing the finding selects that row. An output
/// naming a net the design holds is neither.
#[test]
fn a_name_the_design_lacks_is_flagged_on_its_row_and_by_check() {
    use crate::workbench::diagnostic_routes::apply_remediation;
    use rspice_app_state::state::{PreflightRemediation, SimulationPage};

    let mut engineer = Engineer::new_project("names");
    let e = &mut engineer;
    divider(e);
    add_output(e, "Raw voltage / current", "typo", "V(outt)", None);
    add_output(e, "Raw voltage / current", "vout", "V(out)", None);

    let painted = e.painted();
    assert!(
        painted.contains("No net outt in top"),
        "the row says what the design lacks:\n{painted}"
    );
    assert!(!painted.contains("No net out in"), "{painted}");

    let report = rspice_app_runs::workbench::preflight::collect_report(&e.replay.app.state);
    let findings = report
        .advisories
        .iter()
        .filter(|advisory| advisory.about.0 == "unknown-output-signal")
        .collect::<Vec<_>>();
    assert!(
        matches!(findings.as_slice(), [finding] if finding.message.contains("No net outt in top")),
        "Check names the one output the design cannot answer: {findings:?}"
    );
    assert!(
        report.blockers.is_empty(),
        "a warning, not a refusal: {:?}",
        report.blockers
    );
    let remediation = findings[0]
        .remediation
        .clone()
        .expect("the finding has a row");
    assert_eq!(
        remediation,
        PreflightRemediation::SavedOutput {
            name: "typo".to_owned()
        }
    );
    e.replay.app.state.simulate.selected_saved_output = None;
    e.replay.app.state.simulate.simulation_page = SimulationPage::Save;
    apply_remediation(&mut e.replay.app, remediation);
    assert_eq!(
        e.replay.app.state.simulate.selected_saved_output.as_deref(),
        Some("typo")
    );
    assert_eq!(
        e.replay.app.state.simulate.simulation_page,
        SimulationPage::Outputs
    );
}

/// Lowering "Datasets kept" below the runs a plan holds asks first, naming
/// how many go, and Cancel changes nothing; the yes keeps the newest.
/// Raising it applies at once.
#[test]
fn lowering_datasets_kept_asks_before_it_deletes_runs() {
    let mut engineer = Engineer::new_project("kept");
    let e = &mut engineer;
    divider(e);
    for _ in 0..20 {
        e.run();
    }
    let state = |e: &Engineer| {
        let model = &e.replay.app.state.model;
        let plan = model.sim_setup.analysis_plan.as_ref().expect("a plan").id();
        (
            model
                .simulation
                .retained
                .runs
                .retained_plan_dataset_count(plan),
            model.sim_setup.save_policy.retained_dataset_limit,
        )
    };
    assert_eq!(state(e), (20, 20));

    let lower = |e: &mut Engineer, to: &str| {
        e.setup_page("Save, streaming & retention policy");
        e.press("Datasets kept", Role::ComboBox);
        e.press(to, Role::ListBoxOption);
    };
    lower(e, "5 datasets");
    let painted = e.painted();
    assert!(
        painted.contains("Keep 5 runs?")
            && painted.contains("15 older runs of this test will be deleted."),
        "lowering the limit asks first:\n{painted}"
    );
    assert_eq!(state(e), (20, 20), "nothing is deleted before the yes");
    e.press("Cancel", Role::Button);
    assert_eq!(state(e), (20, 20), "Cancel changes nothing");
    assert!(e.replay.app.state.simulate.retention_confirmation.is_none());

    lower(e, "5 datasets");
    e.press("Delete 15 runs", Role::Button);
    assert_eq!(state(e), (5, 5), "the yes keeps the newest five");

    lower(e, "50 datasets");
    assert!(
        e.replay.app.state.simulate.retention_confirmation.is_none(),
        "raising the limit asks nothing"
    );
    assert_eq!(state(e), (5, 50));
}
