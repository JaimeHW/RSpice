//! An expression is drawn on the axis of the unit unit algebra states for it,
//! or of the unit the reader declares; one whose unit cannot be stated has an
//! unlabeled pane of its own and never borrows another unit's axis.

use super::*;

fn power_fixture() -> ResultsTestHost {
    let x = vec![0.0, 1.0e-3, 2.0e-3];
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            RetainedTrace::new("V(out)", x.clone(), vec![0.0, 2.0, 4.0], "#fff"),
            RetainedTrace::new("V(in)", x.clone(), vec![1.0, 1.0, 1.0], "#fff"),
            RetainedTrace::new("I(vdd)", x, vec![-1.0e-3, -2.0e-3, -3.0e-3], "#fff"),
        ]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy provenance");
    run.restore_lifecycle(crate::state::SimulationRunLifecycle::LegacyUnknown, 0.0)
        .expect("legacy lifecycle");
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
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

/// One frame of a project result document.
fn document_frame(state: &mut ResultsTestHost, document: crate::product::ResultDocumentId) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1200.0, 800.0),
        )),
        ..Default::default()
    };
    let mut controller = crate::simulation::SimulationController::new();
    let _ = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            super::super::super::persistent_document::show(
                ui,
                &mut state.results_parts(),
                &mut controller,
                None,
                document,
            );
        });
    });
}

/// A saved view keeps its expressions, as text and the pane's dataset, with
/// the units declared for them, and the quick view gets its own back.
#[test]
fn a_saved_view_keeps_its_expressions_and_their_declared_units() {
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
        .insert((analysis, "V(out)*I(vdd)".to_owned()), "mW".to_owned());
    let document_id = {
        let mut parts = state.results_parts();
        super::super::super::create_document::open(&mut parts);
        parts.create_result_document.layout_id =
            rspice_results::document_creation::ResultDocumentLayout::SinglePane
                .id()
                .to_owned();
        super::super::super::create_document::commit(&mut parts).expect("the view is saved")
    };
    for _ in 0..2 {
        document_frame(&mut state, document_id);
    }
    let saved = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("the project keeps the document")
        .clone();
    let held: Vec<_> = saved
        .expression_traces()
        .iter()
        .map(|trace| {
            (
                trace.text.as_str(),
                trace.unit.as_deref(),
                trace.datasets.len(),
            )
        })
        .collect();
    assert_eq!(
        held,
        [("V(out)*I(vdd)", Some("mW"), 1), ("V(out)/V(in)", None, 1)]
    );
    let settled = saved.revision();
    document_frame(&mut state, document_id);
    assert_eq!(
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .unwrap()
            .revision(),
        settled,
        "an idle frame writes nothing"
    );
    // The quick view keeps its own expressions while the view shows the
    // document's, and has them back when the view is left.
    state
        .results
        .results
        .session
        .analysis_exprs
        .get_mut(&analysis)
        .unwrap()
        .pop();
    state.results.results.session.leave_persistent_document();
    assert_eq!(
        state.results.results.session.analysis_exprs[&analysis].len(),
        2,
        "the quick view's expressions come back"
    );

    let json = serde_json::to_string(&saved).expect("the document writes out");
    let mut fresh = ResultsTestHost::default();
    fresh.model.simulation.retained.runs = state.model.simulation.retained.runs.clone();
    fresh.model.simulation.view.active_run_idx = Some(0);
    fresh.model.simulation.view.active_analysis_idx = Some(0);
    let reopened: rspice_results::visualization_document::VisualizationDocument =
        serde_json::from_str(&json).expect("it reads back");
    let reopened_id = fresh
        .model
        .workspace
        .content
        .insert_visualization_document(reopened)
        .expect("the project takes the document back");
    document_frame(&mut fresh, reopened_id);
    document_frame(&mut fresh, reopened_id);
    let session = &fresh.results.results.session;
    assert_eq!(
        session.analysis_exprs[&analysis]
            .iter()
            .map(|expr| expr.text.as_str())
            .collect::<Vec<_>>(),
        ["V(out)*I(vdd)", "V(out)/V(in)"]
    );
    assert_eq!(
        session
            .expression_units
            .get(&(analysis, "V(out)*I(vdd)".to_owned()))
            .map(String::as_str),
        Some("mW")
    );
}
