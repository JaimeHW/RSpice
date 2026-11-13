//! An expression is drawn on the axis of the unit unit algebra states for it,
//! or of the unit the reader declares; one whose unit cannot be stated has an
//! unlabeled pane of its own and never borrows another unit's axis.

use super::*;

fn power_fixture() -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let x = vec![0.0, 1.0e-3, 2.0e-3];
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            RetainedTrace::new("V(out)", x.clone(), vec![0.0, 2.0, 4.0], "#fff"),
            RetainedTrace::new("V(in)", x.clone(), vec![1.0, 1.0, 1.0], "#fff"),
            RetainedTrace::new("I(vdd)", x, vec![-1.0e-3, -2.0e-3, -3.0e-3], "#fff"),
        ]),
    );
    state.model.simulation.complete_run();
    state
}

fn add(state: &mut ResultsTestHost, text: &str) {
    let analysis = AnalysisPresentationKey::new(
        state.model.simulation.retained.runs[0].dataset_id,
        &state.model.simulation.retained.runs[0].analyses[0],
    );
    state
        .results
        .results
        .add_expression_trace(&state.model.simulation, analysis, text.to_owned())
        .expect("the strip accepts an expression");
}

/// Draw the waves view twice, as the app does: the first frame evaluates the
/// expressions, the second routes them with what evaluation found.
fn draw(state: &mut ResultsTestHost) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut labels = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, &mut state.results_parts());
            });
        });
        labels = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| node.label().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();
    }
    labels
}

/// Each pane's unit, its signal traces and its expression slots.
fn panes(state: &mut ResultsTestHost) -> Vec<(String, bool, Vec<usize>)> {
    let presentation = state.model.preferences.result_presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    models[0]
        .unit_panes()
        .iter()
        .map(|pane| {
            (
                pane.axis_unit().to_owned(),
                pane.unlabeled,
                pane.expressions.clone(),
            )
        })
        .collect()
}

#[test]
fn each_expression_is_drawn_on_the_axis_of_its_own_unit() {
    let mut state = power_fixture();
    for text in [
        "V(out)-V(in)",
        "V(out)*I(vdd)",
        "V(out)/V(in)",
        "V(out)+I(vdd)",
    ] {
        add(&mut state, text);
    }
    let labels = draw(&mut state);
    assert_eq!(
        panes(&mut state),
        [
            // Volts join the volts pane, watts and a ratio open their own,
            // and volts added to amperes has a pane of its own with no unit.
            ("V".to_owned(), false, vec![0]),
            ("A".to_owned(), false, vec![]),
            ("W".to_owned(), false, vec![1]),
            (String::new(), false, vec![2]),
            (String::new(), true, vec![3]),
        ]
    );
    assert!(
        labels.iter().any(|label| label == "no unit"),
        "the unlabeled pane says it has no unit: {labels:?}"
    );
    assert!(labels.iter().any(|label| label == "W"), "{labels:?}");
}

#[test]
fn a_declared_unit_replaces_the_inferred_one_and_a_hidden_expression_opens_no_pane() {
    let mut state = power_fixture();
    add(&mut state, "V(out)*I(vdd)");
    add(&mut state, "V(out)/V(in)");
    let analysis = AnalysisPresentationKey::new(
        state.model.simulation.retained.runs[0].dataset_id,
        &state.model.simulation.retained.runs[0].analyses[0],
    );
    state
        .results
        .results
        .session
        .expression_units
        .insert((analysis, "V(out)*I(vdd)".to_owned()), "A".to_owned());
    state
        .results
        .results
        .toggle_expression_visibility_by_key(
            &state.model.simulation,
            &rspice_results_ui::selection::ResultExpressionPresentationKey::new(
                analysis,
                "V(out)/V(in)".to_owned(),
            ),
        )
        .expect("the expression is on the strip");
    draw(&mut state);
    assert_eq!(
        panes(&mut state),
        [
            ("V".to_owned(), false, vec![]),
            ("A".to_owned(), false, vec![0]),
        ]
    );
}

#[test]
fn an_expression_that_fails_to_evaluate_opens_no_pane() {
    let mut state = power_fixture();
    add(&mut state, "V(missing)*I(vdd)");
    draw(&mut state);
    assert_eq!(
        panes(&mut state),
        [
            ("V".to_owned(), false, vec![]),
            ("A".to_owned(), false, vec![]),
        ]
    );
}

#[test]
fn a_declared_unit_survives_a_save_and_reload() {
    let mut state = power_fixture();
    add(&mut state, "V(out)*I(vdd)");
    let analysis = AnalysisPresentationKey::new(
        state.model.simulation.retained.runs[0].dataset_id,
        &state.model.simulation.retained.runs[0].analyses[0],
    );
    state
        .results
        .results
        .session
        .expression_units
        .insert((analysis, "V(out)*I(vdd)".to_owned()), "mW".to_owned());
    let saved = state
        .results
        .results
        .project_presentation(&state.model.simulation);
    assert_eq!(saved.expression_groups[0].units.len(), 1);
    let wire = serde_json::to_string(&saved).unwrap();
    state.results.results.session.expression_units.clear();
    crate::workbench::documents::result_document::restore_presentation(
        &mut state.results_parts(),
        serde_json::from_str(&wire).unwrap(),
    );
    assert_eq!(
        state
            .results
            .results
            .session
            .expression_units
            .get(&(analysis, "V(out)*I(vdd)".to_owned()))
            .map(String::as_str),
        Some("mW")
    );
}
