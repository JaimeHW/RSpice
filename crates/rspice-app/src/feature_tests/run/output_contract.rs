//! A live complex preview, read by the calculator the results viewer evaluates
//! with; and the samples a default run keeps.

use egui::accesskit::Role;
use rspice_simulation::output_contract::PreparedSavedOutput;

use rspice_app_results::analysis::calculator;
use rspice_app_runs::simulation::output_contract::fixtures::{ac, output};
use rspice_app_runs::simulation::output_contract::materialize_live_saved_outputs;
use rspice_app_state::product::AnalysisInstanceId;
use rspice_app_state::state::{
    AnalysisResult, ComplexExpressionPolicy, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, RetainedTrace,
    SavedOutputKind, SavedOutputStreaming, SimulationRun,
};

use crate::feature_tests::engineer::Engineer;

#[test]
fn complex_live_outputs_keep_aligned_components_and_full_precision_source_knots() {
    let mut output = output(
        SavedOutputKind::DerivedExpression,
        "Complex preview",
        "complex(V(out),V(out))",
    );
    output.streaming = SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation;
    let contract = PreparedSavedOutput::prepare(&output, AnalysisInstanceId::new(), &ac())
        .unwrap()
        .unwrap();
    let x = (0..10_000)
        .map(|index| 1e-4 + index as f64 * 1e-18)
        .collect::<Vec<_>>();
    let y = (0..x.len())
        .map(|index| 1e-120 * (index as f64 / 17.0).sin())
        .collect::<Vec<_>>();
    let source = AnalysisResult::live_transient_partial(
        1,
        rspice_app_state::state::AnalysisType::Transient,
        "TRAN",
    )
    .with_waveforms(vec![RetainedTrace::new(
        "out",
        x.clone(),
        y.clone(),
        "#fff",
    )]);
    let previews = materialize_live_saved_outputs(&source, &[contract]);
    assert_eq!(previews.len(), 1);
    let preview = &previews[0];
    assert!(preview.x.len() <= DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES);
    assert!(preview.x.windows(2).all(|pair| pair[0] < pair[1]));
    let complex = preview.complex.as_ref().unwrap();
    assert_eq!(preview.x.len(), complex.real.len());
    assert_eq!(preview.x.len(), complex.imag.len());
    for (index, &point) in preview.x.iter().enumerate() {
        let source_index = x
            .binary_search_by(|candidate| candidate.total_cmp(&point))
            .unwrap();
        assert_eq!(complex.real[index], y[source_index]);
        assert_eq!(complex.imag[index], y[source_index]);
    }
    assert!(calculator::waveform_value(preview, ComplexExpressionPolicy::Rectangular).is_ok());
}

/// V(out)'s samples in `run`.
fn out(run: &SimulationRun) -> (Vec<f64>, Vec<f64>) {
    let waves = || run.analyses.iter().flat_map(|analysis| &analysis.waveforms);
    let wave = waves()
        .find(|wave| wave.name.eq_ignore_ascii_case("V(out)"))
        .unwrap_or_else(|| {
            panic!(
                "no V(out) in the run: {:?}",
                waves().map(|wave| &wave.name).collect::<Vec<_>>()
            )
        });
    (wave.x.to_vec(), wave.y.to_vec())
}

/// A run on the plan's default output selection keeps every point its
/// transient accepted, as Save all does: an RC driven through a 1 ns edge
/// gives V(out) the same samples both ways, many more than the 1 us print
/// step's eleven.
#[test]
fn a_default_run_keeps_every_point_its_transient_accepted() {
    let mut engineer = Engineer::new_project("edge");
    let e = &mut engineer;
    edge(e);
    let automatic = out(&e.run());

    save_all(e);
    let everything = out(&e.run());
    assert!(
        automatic.0.len() > 11,
        "V(out) keeps {} samples",
        automatic.0.len()
    );
    assert_eq!(automatic, everything, "V(out) as Save all keeps it");
}

/// A Save all run names each node voltage `V(<node>)`, as a default run does:
/// the net selected on the schematic cross-probes to its trace, and the
/// canvas badges its transient voltage. Under the engine's bare `OUT` neither
/// found it.
#[test]
fn a_save_all_run_cross_probes_and_badges_its_nodes() {
    use rspice_app_state::state::SchematicAnnotationVisibility;
    use rspice_app_workbench::workbench::framework::feature::CodePort as _;

    let mut engineer = Engineer::new_project("save-all-names");
    let e = &mut engineer;
    edge(e);
    save_all(e);
    let run = e.run();
    let names = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .map(|wave| wave.name.as_str())
        .collect::<Vec<_>>();
    assert!(
        names.iter().any(|name| name.eq_ignore_ascii_case("V(out)")),
        "{names:?}"
    );
    assert!(
        !names.iter().any(|name| name.eq_ignore_ascii_case("out")),
        "{names:?}"
    );

    let state = &mut e.replay.app.state;
    let label = state
        .model
        .schematic
        .document()
        .net_labels
        .iter()
        .find(|label| label.name == "out")
        .expect("the out label")
        .id;
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_net_label(label);
    let projection = rspice_app_design::workbench::cross_probe::synchronize_schematic_cross_probe(
        &state.model,
        &mut state.design.schematic_cross_probe,
        state.code.shown_netlist_document_key(),
    )
    .expect("the selection changed");
    let trace = projection
        .trace
        .expect("the out net cross-probes to a trace");
    assert!(
        trace.source_name().eq_ignore_ascii_case("V(out)"),
        "{}",
        trace.source_name()
    );

    state.design.schematic_visibility.annotations =
        SchematicAnnotationVisibility::TransientVoltages;
    let badges = rspice_app_design::schematic::view::scene::operating_point_annotations(
        &state.canvas_facts(),
    );
    assert!(
        badges.iter().any(|badge| badge.key == "out"),
        "the out net has a transient voltage badge: {:?}",
        badges
            .iter()
            .map(|badge| (&badge.key, &badge.lines))
            .collect::<Vec<_>>()
    );
}

/// An RC driven through a 1 ns edge, `in` to `out`, and a transient over it.
fn edge(e: &mut Engineer) {
    e.top_sheet();
    let pins = e.place(
        "pulse",
        "Pulse Voltage Source",
        (100, 100),
        &[
            ("Instance", "vin"),
            ("Value", "0"),
            ("Pulsed value (V2), V", "1"),
            ("Delay time (TD), s", "1u"),
            ("Rise time (TR), s", "1n"),
            ("Fall time (TF), s", "1n"),
            ("Pulse width (PW), s", "4u"),
            ("Period (PER), s", "10u"),
        ],
    );
    e.connect(&pins, &["in", "0"]);
    let pins = e.place(
        "resistor",
        "Resistor",
        (250, 100),
        &[("Instance", "r1"), ("Value", "1k")],
    );
    e.connect(&pins, &["in", "out"]);
    let pins = e.place(
        "capacitor",
        "Capacitor",
        (400, 100),
        &[("Instance", "c1"), ("Value", "10p")],
    );
    e.connect(&pins, &["out", "0"]);
    e.transient("1u", "10u");
}

/// The plan's output selection set to Save all on its Save page.
fn save_all(e: &mut Engineer) {
    e.setup_page("Save, streaming & retention policy");
    e.press("Plan output selection mode", Role::ComboBox);
    e.press("Save all", Role::ListBoxOption);
}
