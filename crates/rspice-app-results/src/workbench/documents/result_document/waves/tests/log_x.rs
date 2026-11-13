//! The shared X axis switches between linear and logarithmic: a startup
//! transient read over decades of time, a frequency sweep read linearly. A
//! domain a logarithmic axis cannot show refuses it with the reason.

use super::*;
use rspice_results_ui::waves::stack::log_x_refusal;

fn strip_host(kind: AnalysisType, x: Vec<f64>) -> ResultsTestHost {
    let y: Vec<f64> = x.iter().map(|x| x * 2.0).collect();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, kind, "Analysis")
            .with_waveforms(vec![RetainedTrace::new("V(out)", x, y, "#fff")]),
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

fn model(state: &mut ResultsTestHost) -> Arc<Vec<StripModel>> {
    let presentation = state.model.preferences.result_presentation_policy();
    cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    )
}

fn key(state: &ResultsTestHost) -> AnalysisPresentationKey {
    AnalysisPresentationKey::new(
        state.model.simulation.retained.runs[0].dataset_id,
        &state.model.simulation.retained.runs[0].analyses[0],
    )
}

/// Draw the waves view and return the labels it publishes.
fn frame(state: &mut ResultsTestHost) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            show(ui, &mut state.results_parts());
        });
    });
    output
        .platform_output
        .accesskit_update
        .map(|update| {
            update
                .nodes
                .iter()
                .filter_map(|(_, node)| node.label().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn a_startup_transient_reads_over_decades_of_time_from_its_first_positive_sample() {
    let x = vec![0.0, 1.0e-9, 1.0e-8, 1.0e-7, 1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3];
    let mut state = strip_host(AnalysisType::Transient, x);
    assert_eq!(model(&mut state)[0].x_scale, XScale::Linear);
    assert!(
        frame(&mut state).iter().any(|label| label == "X scale LIN"),
        "the axis offers its scale"
    );
    assert_eq!(log_x_refusal(&model(&mut state)[0]), None);

    let analysis = key(&state);
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    let models = model(&mut state);
    let strip = &models[0];
    assert_eq!(strip.x_scale, XScale::Log10);
    // t = 0 has no place on a logarithmic axis; the axis starts at the
    // first positive sample.
    assert_eq!(strip.x_range, Some((1.0e-9, 1.0e-3)));
    // Ticks at each decade, as the reference viewers place them.
    let axis = rspice_results_ui::waves::navigation::model_x_axis(
        strip,
        1.0e-9,
        1.0e-3,
        Default::default(),
    );
    let ticks: Vec<f64> = axis.ticks.iter().map(|(value, _)| *value).collect();
    assert_eq!(ticks.len(), 7, "{ticks:?}");
    for (tick, decade) in ticks.iter().zip(-9..=-3) {
        assert!(
            (tick / 10f64.powi(decade) - 1.0).abs() < 1.0e-12,
            "{tick} is not 1e{decade}"
        );
    }
    drop(models);
    assert!(frame(&mut state).iter().any(|label| label == "X scale LOG"));
    // The choice is the project's.
    let saved = state
        .results
        .results
        .project_presentation(&state.model.simulation);
    assert_eq!(
        saved.x_scales,
        [rspice_results::result_presentation::StripXScale {
            analysis,
            logarithmic: true
        }]
    );
    state.results.results.session.x_scales.clear();
    crate::workbench::documents::result_document::restore_presentation(
        &mut state.results_parts(),
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap(),
    );
    assert_eq!(model(&mut state)[0].x_scale, XScale::Log10);
}

#[test]
fn a_frequency_sweep_reads_linearly_when_the_reader_asks() {
    let x = vec![1.0e3, 2.0e3, 3.0e3, 4.0e3];
    let mut state = strip_host(AnalysisType::Ac, x);
    state.results.results.session.viewer = crate::workbench::ResultViewer::Bode;
    assert_eq!(model(&mut state)[0].x_scale, XScale::Log10);
    let analysis = key(&state);
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Linear);
    assert_eq!(model(&mut state)[0].x_scale, XScale::Linear);
}

#[test]
fn a_signed_sweep_refuses_a_logarithmic_axis_and_says_why() {
    let x = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    let mut state = strip_host(AnalysisType::DcSweep, x);
    state.results.results.session.viewer = crate::workbench::ResultViewer::DcSweep;
    let reason = log_x_refusal(&model(&mut state)[0]).expect("a signed sweep is refused");
    assert!(reason.contains("reaches -1"), "{reason}");
    assert!(reason.contains("positive values only"), "{reason}");
    // A choice that reaches the strip anyway, from a file, is not applied.
    let analysis = key(&state);
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    assert_eq!(model(&mut state)[0].x_scale, XScale::Linear);
    assert!(frame(&mut state).iter().any(|label| label == "X scale LIN"));
}
