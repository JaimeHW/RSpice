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
/// expressions, the second routes them with what evaluation found. Returns
/// the second frame's accessible labels and painted text.
fn draw(state: &mut ResultsTestHost) -> (Vec<String>, Vec<String>) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut labels = Vec::new();
    let mut painted = Vec::new();
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
                    .filter_map(|(_, node)| node.label().or(node.value()).map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();
        painted = super::conversion::texts(&output.shapes)
            .into_iter()
            .map(|(text, _)| text)
            .collect();
    }
    (labels, painted)
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
    let (labels, painted) = draw(&mut state);
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
    // Each pane states its unit as its Y axis' caption; the one whose unit
    // cannot be stated says so there, and its reason is on the caption.
    assert!(
        painted.iter().any(|text| text == "no unit")
            && labels.iter().any(|label| label == "no unit"),
        "the unlabeled pane says it has no unit: {painted:?} {labels:?}"
    );
    assert!(painted.iter().any(|text| text == "W"), "{painted:?}");
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
pub(super) fn document_frame(
    state: &mut ResultsTestHost,
    document: crate::product::ResultDocumentId,
) {
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

/// Save the active analysis as a one-pane view.
pub(super) fn save_single_pane_view(
    state: &mut ResultsTestHost,
) -> crate::product::ResultDocumentId {
    let mut parts = state.results_parts();
    super::super::super::create_document::open(&mut parts);
    parts.create_result_document.layout_id =
        rspice_results::document_creation::ResultDocumentLayout::SinglePane
            .id()
            .to_owned();
    super::super::super::create_document::commit(&mut parts).expect("the view is saved")
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
    let document_id = save_single_pane_view(&mut state);
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

/// An expression the reader adds while a document pane is open is written to
/// that document. The expression editor commits inside the pane's frame,
/// between the pane's projection and its capture, so the test adds it there;
/// the quick view's own expressions are left as they were.
#[test]
fn an_expression_added_while_a_document_pane_is_open_is_written_to_the_document() {
    use super::super::super::persistent_document::{
        capture_pane_presentation, project_pane_presentation, projection,
    };
    let mut state = power_fixture();
    add(&mut state, "V(out)*I(vdd)");
    let analysis = AnalysisPresentationKey::new(
        state.model.simulation.retained.runs[0].dataset_id,
        &state.model.simulation.retained.runs[0].analyses[0],
    );
    let document_id = save_single_pane_view(&mut state);
    for _ in 0..2 {
        document_frame(&mut state, document_id);
    }
    let pane = projection(state.results_reads(), document_id)
        .expect("the document projects")
        .pages[0]
        .panes[0]
        .clone();
    let viewer = super::super::super::ResultViewer::from_viewer_document_id(&pane.viewer_id)
        .expect("the pane names a Results viewer");
    project_pane_presentation(&mut state.results_parts(), &pane, viewer)
        .expect("the pane projects onto its strip");
    add(&mut state, "V(out)-V(in)");
    state
        .results
        .results
        .session
        .expression_units
        .insert((analysis, "V(out)-V(in)".to_owned()), "mV".to_owned());
    capture_pane_presentation(&mut state.results_parts(), &pane, viewer);

    let held = |state: &ResultsTestHost| -> Vec<(String, Option<String>)> {
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project keeps the document")
            .expression_traces()
            .iter()
            .map(|trace| (trace.text.clone(), trace.unit.clone()))
            .collect()
    };
    let expected = vec![
        ("V(out)*I(vdd)".to_owned(), None),
        ("V(out)-V(in)".to_owned(), Some("mV".to_owned())),
    ];
    assert_eq!(
        held(&state),
        expected,
        "the document keeps the new expression"
    );
    // The next frame projects it back from the document and writes nothing.
    let settled = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .unwrap()
        .revision();
    document_frame(&mut state, document_id);
    assert_eq!(held(&state), expected);
    assert_eq!(
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .unwrap()
            .revision(),
        settled
    );
    // The quick view never had it.
    state.results.results.session.leave_persistent_document();
    assert_eq!(
        state.results.results.session.analysis_exprs[&analysis]
            .iter()
            .map(|expr| expr.text.as_str())
            .collect::<Vec<_>>(),
        ["V(out)*I(vdd)"]
    );
    assert!(
        !state
            .results
            .results
            .session
            .expression_units
            .contains_key(&(analysis, "V(out)-V(in)".to_owned()))
    );
}
