//! Family selection is an input to calculations, not a slice of their output.

use super::*;

fn fixture(indices: Vec<usize>, text: &str) -> (AppState, Vec<StripModel>) {
    let mut state = AppState::default();
    state
        .simulation
        .start_run()
        .add_analysis(family_analysis(vec![10.0, 10.0, 1.0, 1.0, 7.0, 7.0]));
    state.simulation.complete_run();
    let run = state.simulation.active_run().unwrap();
    let analysis = &run.analyses[0];
    let manifest = FamilyManifest::from_analysis(analysis).unwrap().unwrap();
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let selection = SourceSampleSelection::new(run.dataset_id, analysis.id, indices)
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();
    state.ui.results.set_sample_selection(Some(selection));
    state
        .ui
        .results
        .add_expression_trace(&state.simulation, key, text.to_owned())
        .unwrap();
    let models = build_models(
        &state.simulation,
        &mut DerivedSeries::default(),
        &Tokens::default(),
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        state.ui.results.sample_selection.as_ref(),
        &HashSet::new(),
    );
    (state, models)
}

#[test]
fn excluded_family_members_cannot_change_a_selected_maximum() {
    let (mut state, models) = fixture(vec![2, 3], "max(V(out))");
    let curves = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(curves.len(), 1);
    assert_eq!(curves[0].x.as_slice(), &[3.0, 4.0]);
    assert_eq!(curves[0].y.as_slice(), &[1.0, 1.0]);
}

#[test]
fn reductions_are_evaluated_independently_for_each_selected_member() {
    let (mut state, models) = fixture((0..6).collect(), "max(V(out))");
    let curves = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(curves.len(), 3);
    for (member, expected) in [("SS", 10.0), ("TT", 1.0), ("FF", 7.0)] {
        let curve = curves
            .iter()
            .find(|curve| curve.label.contains(member))
            .unwrap();
        assert_eq!(curve.y.as_slice(), &[expected, expected], "{member}");
    }
}

#[test]
fn stateful_expressions_restart_at_each_member_and_use_the_displayed_axis() {
    let (mut state, models) = fixture((0..6).collect(), "integ(V(out))");
    let curves = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(curves.len(), 3);
    for (member, expected, axis) in [
        ("SS", 10.0, [1.0, 2.0]),
        ("TT", 1.0, [3.0, 4.0]),
        ("FF", 7.0, [5.0, 6.0]),
    ] {
        let curve = curves
            .iter()
            .find(|curve| curve.label.contains(member))
            .unwrap();
        assert_eq!(curve.x.as_slice(), &axis);
        assert_eq!(curve.y.as_slice(), &[0.0, expected], "{member}");
    }

    let (mut state, models) = fixture(vec![2, 3], "xval(V(out))");
    let curves = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(curves[0].y.as_slice(), &[3.0, 4.0]);
}

#[test]
fn empty_family_selection_is_an_error_instead_of_a_successful_empty_trace() {
    let (state, _) = fixture(vec![], "max(V(out))");
    let error = evaluate_expression(
        &state.simulation,
        0,
        "max(V(out))",
        state.ui.results.sample_selection.as_ref(),
    )
    .unwrap_err();
    assert!(error.contains("no samples"), "{error}");
}

#[test]
fn changing_scope_refreshes_calculations_while_visibility_does_not() {
    let (mut state, models) = fixture(vec![2, 3], "max(V(out))");
    let first = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(first[0].y.as_slice(), &[1.0, 1.0]);
    let version = state.simulation.data_version;
    let run = state.simulation.active_run().unwrap();
    let digest = run.dataset_content_digest();
    let manifest = FamilyManifest::from_analysis(&run.analyses[0])
        .unwrap()
        .unwrap();
    let next = SourceSampleSelection::new(run.dataset_id, 41, vec![0, 1])
        .unwrap()
        .with_family_presentation(&manifest, &family_policy())
        .unwrap();
    state.ui.results.set_sample_selection(Some(next));
    let second = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(second[0].x.as_slice(), &[1.0, 2.0]);
    assert_eq!(second[0].y.as_slice(), &[10.0, 10.0]);
    toggle_visibility(&mut state, 0, 0);
    let hidden = resolve_strip_exprs(&mut state, &models[0], &Tokens::default());
    assert_eq!(hidden[0].y, second[0].y);
    assert_eq!(state.simulation.data_version, version);
    assert_eq!(
        state
            .simulation
            .active_run()
            .unwrap()
            .dataset_content_digest(),
        digest
    );
}

#[test]
fn restored_expression_and_typed_family_filter_keep_the_selected_measurement() {
    let (state, models) = fixture(vec![2, 3], "max(V(out))");
    let mut policy = family_policy();
    policy.filter = Some(FamilyFilterExpression {
        source: "process = TT".to_owned(),
        predicate: FamilyPredicate::Compare {
            dimension: FamilyDimension::new("process", ValueType::Text).unwrap(),
            operator: FamilyComparisonOperator::Equal,
            value: TypedValue::Text("TT".to_owned()),
        },
    });
    let bytes = serde_json::to_vec(&(
        state.ui.results.project_presentation(&state.simulation),
        policy,
    ))
    .unwrap();
    let (presentation, policy): (
        crate::state::result_presentation::ResultPresentation,
        FamilyPresentationPolicy,
    ) = serde_json::from_slice(&bytes).unwrap();
    let mut reopened = AppState::default();
    reopened.simulation = state.simulation.clone();
    super::super::super::restore_presentation(&mut reopened, presentation);
    let run = reopened.simulation.active_run().unwrap();
    let manifest = FamilyManifest::from_analysis(&run.analyses[0])
        .unwrap()
        .unwrap();
    let indices = manifest
        .matching_source_indices_for_filter(policy.filter.as_ref())
        .unwrap();
    let selection = SourceSampleSelection::new(run.dataset_id, 41, indices)
        .unwrap()
        .with_family_presentation(&manifest, &policy)
        .unwrap();
    reopened.ui.results.set_sample_selection(Some(selection));
    let curves = resolve_strip_exprs(&mut reopened, &models[0], &Tokens::default());
    assert_eq!(curves.len(), 1);
    assert_eq!(curves[0].x.as_slice(), &[3.0, 4.0]);
    assert_eq!(curves[0].y.as_slice(), &[1.0, 1.0]);
}

#[test]
fn family_editor_displays_calculation_scope_inside_a_narrow_pane() {
    let (mut state, models) = fixture(vec![2, 3], "max(V(out))");
    state.ui.results.expr_editor = Some(ExprEditor {
        analysis: models[0].analysis_key,
        text: "max(V(out))".to_owned(),
        error: None,
        want_focus: false,
    });
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(350.0, 200.0));
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(viewport),
            ..Default::default()
        },
        |root| {
            egui::CentralPanel::default().show(root, |ui| {
                expr_editor_row(ui, &mut state, models[0].analysis_key, 0);
            });
        },
    );
    fn scope_bounds(shape: &egui::epaint::Shape, found: &mut Vec<egui::Rect>) {
        match shape {
            egui::epaint::Shape::Text(text) if text.galley.job.text.starts_with("Scope:") => {
                assert!(text.galley.job.text.contains("each selected curve"));
                found.push(text.galley.rect.translate(text.pos.to_vec2()));
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    scope_bounds(shape, found);
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    for shape in output.shapes {
        scope_bounds(&shape.shape, &mut found);
    }
    assert_eq!(found.len(), 1);
    assert!(viewport.contains_rect(found[0]));
    assert!(found[0].top() >= Tokens::get(&ctx).metrics.ctl_h + 10.0);
}
