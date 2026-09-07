//! Histogram readers must use one current population for bins and statistics.

use super::super::ActiveViewer;
use super::*;
use crate::state::{AnalysisResult, AnalysisType, SimulationRunLifecycle, SimulationRunProvenance};

fn state_with(variable: MonteCarloVariableMetadata) -> AppState {
    let count = variable.samples.len();
    let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 7,
            runs_requested: count,
            runs_completed: count,
            failures: 0,
            all_converged: true,
            variables: vec![variable],
            member_measurements: Vec::new(),
        },
    );
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.simulation.complete_run();
    state
}

fn paint(state: &mut AppState, panel: bool) -> String {
    fn collect(shape: &egui::Shape, text: &mut String) {
        match shape {
            egui::Shape::Text(value) => {
                text.push_str(value.galley.text());
                text.push('\n');
            }
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, text)),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                if panel {
                    right_panel(ui, state);
                } else {
                    show(ui, state);
                }
            });
        },
    );
    let mut text = String::new();
    for shape in &output.shapes {
        collect(&shape.shape, &mut text);
    }
    for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
        if let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive {
            assert!(
                mesh.vertices
                    .iter()
                    .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite())
            );
        }
    }
    text
}

#[test]
fn hist_source_restoration_opens_the_current_population_without_cached_bins() {
    use crate::io::project_io::ProjectSimulationResults;
    let original = state_with(super::tests::mc_variable("gain"));
    let mut state = AppState::default();
    state.simulation = ProjectSimulationResults::from_state(&original.simulation)
        .into_simulation_state()
        .unwrap();
    assert!(state.viewer_capability(ActiveViewer::Histogram).available);
    assert!(paint(&mut state, false).contains("gain"));
    assert!(paint(&mut state, true).contains("Exact samples"));
    let text = paint(&mut state, false);
    assert!(text.contains("gain"), "{text}");
}

#[test]
fn hist_selection_preserves_the_measurement_name_across_reordering_and_removal() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    state.analysis.histogram_state.selected = Some("gain".to_owned());
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    let mut other = variables[0].clone();
    other.name = "offset".to_owned();
    other.samples.fill(99.0);
    other.mean = 99.0;
    other.std_dev = 0.0;
    other.min = 99.0;
    other.max = 99.0;
    variables.insert(0, other);
    let histogram = active_histogram(&state).unwrap();
    assert_eq!(histogram.name, "gain");
    assert_eq!(histogram.data_max, 3.0);
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    variables.remove(1);
    assert!(active_histogram(&state).is_none());
    assert!(paint(&mut state, false).contains("selected measurement is unavailable"));
    state.analysis.histogram_state.selected = Some("offset".to_owned());
    assert_eq!(active_histogram(&state).unwrap().data_max, 99.0);
}

#[test]
fn hist_modes_refresh_projection_and_keep_point_density_unavailable() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    let count = hist_plan(&state, "gain");
    for mode in HistogramDisplayMode::ALL {
        state.analysis.histogram_state.mode = mode;
        let plan = hist_plan(&state, "gain");
        assert_eq!(plan.display.as_ref().unwrap().mode, mode);
        if mode != HistogramDisplayMode::Count {
            assert_ne!(plan.display_source, count.display_source);
        }
        let text = paint(&mut state, false);
        assert!(text.contains(mode.label()), "{text}");
    }
    let mut state = state_with(MonteCarloVariableMetadata {
        name: "constant".to_owned(),
        samples: vec![1.0; 3],
        mean: 1.0,
        std_dev: 0.0,
        min: 1.0,
        max: 1.0,
    });
    state.analysis.histogram_state.mode = HistogramDisplayMode::Pdf;
    assert!(paint(&mut state, false).contains("no finite probability density"));
    state.analysis.histogram_state.custom_range = true;
    state.analysis.histogram_state.custom_min = 0.0;
    state.analysis.histogram_state.custom_max = 2.0;
    assert!(active_histogram_display(&state).is_some());
}

#[test]
fn hist_empirical_cdf_reuses_a_large_sorted_population() {
    use super::super::frame_work::WorkCounts;
    let count = 100_000;
    let mut state = state_with(MonteCarloVariableMetadata {
        name: "gain".to_owned(),
        samples: (0..count).map(f64::from).collect(),
        mean: (f64::from(count) - 1.0) * 0.5,
        std_dev: (f64::from(count) * f64::from(count + 1) / 12.0).sqrt(),
        min: 0.0,
        max: f64::from(count - 1),
    });
    state.analysis.histogram_state.mode = HistogramDisplayMode::Cdf;
    paint(&mut state, false);
    let display = active_histogram_display(&state).unwrap();
    assert_eq!(display.cdf.as_ref().unwrap().x.len(), 200_000);
    let work = WorkCounts::reset();
    for _ in 0..3 {
        paint(&mut state, false);
        paint(&mut state, true);
        assert!(Arc::ptr_eq(
            &display,
            &active_histogram_display(&state).unwrap()
        ));
    }
    assert_eq!(work.since().total(), 0);
}

#[test]
fn hist_source_statistics_follow_nested_edits_and_active_analysis_changes() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    let mut other = state.simulation.runs[0].analyses[0].clone();
    other.id = 2;
    state.simulation.runs[0].add_analysis(other);
    let version = state.simulation.data_version;
    assert_eq!(hist_plan(&state, "gain").moments.unwrap().mean, 2.0);
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    let variable = &mut variables[0];
    variable.samples = vec![10.0, 20.0, 30.0];
    variable.mean = 20.0;
    variable.std_dev = 10.0;
    variable.min = 10.0;
    variable.max = 30.0;
    assert_eq!(state.simulation.data_version, version);
    assert_eq!(hist_plan(&state, "gain").moments.unwrap().mean, 20.0);
    state.simulation.active_analysis_idx = Some(1);
    assert_eq!(hist_plan(&state, "gain").moments.unwrap().mean, 2.0);
}

#[test]
fn hist_source_yield_replacement_cannot_reuse_a_wrapped_generation() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    state.simulation.runs[0].analyses[0].family_metadata = None;
    let provenance = super::tests::provenance(&state.simulation.runs[0]);
    let valid = super::tests::result("gain", 90.0);
    state
        .simulation
        .replace_yield_evidence(vec![valid.clone()], Some(provenance));
    let version = state.simulation.data_version;
    assert!(hist_plan(&state, "gain").yield_is_consistent);
    let mut invalid = valid;
    invalid.stats.mean += 1.0;
    state
        .simulation
        .replace_yield_evidence(vec![invalid], Some(provenance));
    state.simulation.data_version = version;
    assert!(!hist_plan(&state, "gain").yield_is_consistent);
}

#[test]
fn hist_source_yield_must_describe_the_displayed_monte_carlo_population() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    let provenance = super::tests::provenance(&state.simulation.runs[0]);
    let unrelated = super::tests::result("gain", 90.0);
    assert!(yield_result_is_consistent(&unrelated));
    state
        .simulation
        .replace_yield_evidence(vec![unrelated], Some(provenance));
    assert!(!hist_plan(&state, "gain").yield_is_consistent);
}

#[test]
fn hist_source_yield_provenance_cannot_name_a_different_run() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    let mut provenance = super::tests::provenance(&state.simulation.runs[0]);
    provenance.source_run_id = crate::state::SimulationRun::new(99).run_id;
    state
        .simulation
        .replace_yield_evidence(vec![super::tests::result("gain", 90.0)], Some(provenance));
    assert!(
        state
            .simulation
            .yield_results_for_active_dataset()
            .is_none()
    );
}

#[test]
fn hist_source_unchanged_large_histories_do_not_repeat_population_walks() {
    use super::super::frame_work::WorkCounts;
    let mut state = state_with(MonteCarloVariableMetadata {
        name: "gain".to_owned(),
        samples: vec![1.0; 100_000],
        mean: 1.0,
        std_dev: 0.0,
        min: 1.0,
        max: 1.0,
    });
    paint(&mut state, false);
    paint(&mut state, true);
    let work = WorkCounts::reset();
    for _ in 0..4 {
        assert!(state.viewer_capability(ActiveViewer::Histogram).available);
        assert!(active_histogram(&state).is_some());
        paint(&mut state, false);
        paint(&mut state, true);
    }
    let mut clone = state.clone();
    paint(&mut clone, false);
    paint(&mut clone, true);
    assert_eq!(work.since().total(), 0);
}

#[test]
fn hist_source_bins_follow_display_settings_and_repaired_evidence() {
    let mut state = state_with(super::tests::mc_variable("gain"));
    state.analysis.histogram_state.bin_count = 2;
    state.analysis.histogram_state.selected = Some("gain".to_owned());
    let first = active_histogram(&state).unwrap();
    assert_eq!(first.bins.len(), 2);
    assert_eq!(first.total_count, 3);
    assert!(Arc::ptr_eq(&first, &active_histogram(&state).unwrap()));
    state.analysis.histogram_state.bin_count = 3;
    let rebuilt = active_histogram(&state).unwrap();
    assert_eq!(
        rebuilt.bins.iter().map(|bin| bin.count).collect::<Vec<_>>(),
        vec![1, 1, 1]
    );
    assert!(!Arc::ptr_eq(&first, &rebuilt));
    state.analysis.histogram_state.custom_range = true;
    state.analysis.histogram_state.custom_min = 1.0;
    state.analysis.histogram_state.custom_max = 2.0;
    let cropped = active_histogram(&state).unwrap();
    assert_eq!(cropped.range(), (1.0, 2.0));
    assert_eq!(cropped.overflow, 1);
    state.analysis.histogram_state.custom_max = f64::NAN;
    assert!(active_histogram(&state).is_none());
    state.analysis.histogram_state.custom_range = false;
    state.simulation.runs[0].analyses[0].success = false;
    assert!(!histogram_is_available(&state));
    assert!(active_histogram(&state).is_none());
    state.simulation.runs[0].analyses[0].success = true;
    assert_eq!(active_histogram(&state).unwrap().total_count, 3);
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    variables[0].samples[0] = f64::NAN;
    assert!(active_histogram(&state).is_none());
    let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) = state.simulation.runs[0]
        .analyses[0]
        .family_metadata
        .as_mut()
    else {
        unreachable!()
    };
    variables[0].samples[0] = 1.0;
    assert_eq!(active_histogram(&state).unwrap().total_count, 3);
}

#[test]
fn hist_source_yield_requires_matching_seed_and_ordered_samples() {
    let result = super::tests::result("gain", 90.0);
    let mut state = state_with(MonteCarloVariableMetadata {
        name: "gain".to_owned(),
        samples: result.samples.clone(),
        mean: result.stats.mean,
        std_dev: result.stats.std_dev,
        min: result.stats.min,
        max: result.stats.max,
    });
    let provenance = super::tests::provenance(&state.simulation.runs[0]);
    state
        .simulation
        .replace_yield_evidence(vec![result.clone()], Some(provenance));
    assert!(hist_plan(&state, "gain").yield_is_consistent);
    let mut wrong_seed = provenance;
    wrong_seed.seed += 1;
    state
        .simulation
        .replace_yield_evidence(vec![result.clone()], Some(wrong_seed));
    assert!(!hist_plan(&state, "gain").yield_is_consistent);
    let mut reordered = result;
    reordered.samples.reverse();
    reordered.trail.reverse();
    assert!(yield_result_is_consistent(&reordered));
    state
        .simulation
        .replace_yield_evidence(vec![reordered], Some(provenance));
    assert!(!hist_plan(&state, "gain").yield_is_consistent);
}

#[test]
fn hist_source_moments_preserve_representable_tiny_and_large_statistics() {
    for scale in [1e-200, 1.0, 1e200] {
        let moments = moments_from_samples(&[scale, scale * 2.0, scale * 3.0]).unwrap();
        assert!(nearly_equal(moments.mean, scale * 2.0));
        assert!(nearly_equal(moments.std_dev, scale));
        assert!(!nearly_equal(moments.std_dev, 0.0));
    }
    let constant = moments_from_samples(&[f64::MAX; 4]).unwrap();
    assert_eq!(constant.mean, f64::MAX);
    assert_eq!(constant.std_dev, 0.0);
    let residual = moments_from_samples(&[1e100, 3.0, -1e100]).unwrap();
    assert!(nearly_equal(residual.mean, 1.0));
    assert!(nearly_equal(residual.std_dev, 1e100));
    let mut state = state_with(MonteCarloVariableMetadata {
        name: "large".to_owned(),
        samples: vec![1e308, 1.7e308],
        mean: 1.35e308,
        std_dev: 7e307 / 2.0_f64.sqrt(),
        min: 1e308,
        max: 1.7e308,
    });
    assert!(paint(&mut state, false).contains("large"));
    assert!(paint(&mut state, true).contains("Exact samples"));
}
