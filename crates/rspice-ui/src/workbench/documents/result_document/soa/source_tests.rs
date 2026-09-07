//! SOA readers must validate current evidence before reusing display facts.

use super::tests::{active_key, soa_state};
use super::*;
use crate::ui::plot::{DisplayDecimation, TraceView};
use std::sync::Arc;

fn plan(state: &mut AppState) -> Arc<SoaPlan> {
    let key = active_key(state);
    soa_plan(state, key).expect("valid active SOA evidence")
}

fn replace_stress(state: &mut AppState, samples: usize, peak: f64) {
    let mut donor = soa_state(1, samples, peak);
    let replacement = donor.simulation.runs[0].analyses.remove(0);
    let analysis = &mut state.simulation.runs[0].analyses[0];
    analysis.waveforms = replacement.waveforms;
    analysis.result_payload = replacement.result_payload;
    analysis.family_metadata = replacement.family_metadata;
}

fn envelope(state: &mut AppState) -> Arc<[[f64; 2]]> {
    let plan = plan(state);
    let facts = plan.facts(0).unwrap();
    let waveform = &state.simulation.runs[0].analyses[0].waveforms
        [facts.stress_waveform.expect("verified stress history")];
    state.ui.results.cache.series(
        DisplayDecimation::EnvelopeExtrema,
        facts.stress_cache_key,
        &waveform.x,
        &waveform.y,
        TraceView {
            x0: 0.0,
            x1: *waveform.x.last().unwrap(),
            y0: 0.0,
            y1: 4.0,
            x_scale: XScale::Linear,
            y_scale: XScale::Linear,
            columns: 16,
            rows: 100,
        },
        false,
        None,
    )
}

fn pick(state: &mut AppState) {
    state.ui.results.selected_soa_rule = Some(SoaRuleSelection {
        analysis: active_key(state),
        device_id: "M0000".to_owned(),
        parameter: SoaParameterEvidence::DrainSourceVoltage,
    });
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
    for shape in output.shapes {
        collect(&shape.shape, &mut text);
    }
    text
}

#[test]
fn soa_source_nested_edits_refresh_facts_without_a_version_bump() {
    let mut state = soa_state(1, 64, 3.0);
    let before = plan(&mut state);
    let version = state.simulation.data_version;
    replace_stress(&mut state, 64, 3.2);
    let after = plan(&mut state);
    assert_eq!(state.simulation.data_version, version);
    assert_ne!(
        before.facts(0).unwrap().interval_full,
        after.facts(0).unwrap().interval_full
    );
}

#[test]
fn soa_source_restoration_refreshes_the_actual_plot_before_a_frame() {
    use crate::io::project_io::ProjectSimulationResults;
    use crate::state::{SimulationRunLifecycle, SimulationRunProvenance};

    let mut state = soa_state(1, 64, 3.0);
    let run = &mut state.simulation.runs[0];
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    state.simulation.next_run_id = 1;
    let before = envelope(&mut state);
    let version = state.simulation.data_version;
    let key = active_key(&state);
    replace_stress(&mut state, 64, 3.2);
    let restored = ProjectSimulationResults::from_state(&state.simulation)
        .into_simulation_state()
        .unwrap();
    state.simulation = restored;
    state.simulation.data_version = version;
    assert_eq!(active_key(&state), key);
    let after = envelope(&mut state);
    assert_eq!(before.last().unwrap()[1], 3.0);
    assert_eq!(after.last().unwrap()[1], 3.2);
    assert!(!Arc::ptr_eq(&before, &after));
}

#[test]
fn soa_source_cached_analysis_must_still_be_active() {
    let mut state = soa_state(1, 32, 3.0);
    let second = soa_state(1, 32, 3.2).simulation.runs[0].analyses[0].clone();
    state.simulation.runs[0].add_analysis(second);
    let key = active_key(&state);
    plan(&mut state);
    state.simulation.active_analysis_idx = Some(1);
    assert!(soa_plan(&mut state, key).is_none());
}

#[test]
fn soa_source_rule_selection_survives_navigation_but_not_rule_removal() {
    let mut state = soa_state(2, 32, 3.0);
    let second = soa_state(1, 32, 3.2).simulation.runs[0].analyses[0].clone();
    state.simulation.runs[0].add_analysis(second);
    pick(&mut state);
    assert!(paint(&mut state, true).contains("SELECTED SOA RULE"));
    state.simulation.active_analysis_idx = Some(1);
    assert!(!paint(&mut state, true).contains("SELECTED SOA RULE"));
    assert!(state.ui.results.selected_soa_rule.is_some());
    state.simulation.active_analysis_idx = Some(0);
    let analysis = &mut state.simulation.runs[0].analyses[0];
    analysis.waveforms.reverse();
    let Some(AnalysisResultPayload::Soa { evaluations, .. }) = analysis.result_payload.as_mut()
    else {
        unreachable!()
    };
    evaluations.reverse();
    // Rules have a canonical retained order; waveform storage does not.
    assert!(analysis.validate_retained_evidence().is_err());
    assert!(!paint(&mut state, true).contains("SELECTED SOA RULE"));
    assert!(state.ui.results.selected_soa_rule.is_some());
    let analysis = &mut state.simulation.runs[0].analyses[0];
    let Some(AnalysisResultPayload::Soa { evaluations, .. }) = analysis.result_payload.as_mut()
    else {
        unreachable!()
    };
    evaluations.reverse();
    analysis.validate_retained_evidence().unwrap();
    let reordered = paint(&mut state, true);
    assert!(reordered.contains("SELECTED SOA RULE"), "{reordered}");
    assert!(reordered.contains("M0000"), "{reordered}");
    let Some(AnalysisResultPayload::Soa {
        evaluations,
        violations,
    }) = state.simulation.runs[0].analyses[0].result_payload.as_mut()
    else {
        unreachable!()
    };
    evaluations.retain(|rule| rule.device_id != "M0000");
    violations.retain(|event| event.device_id != "M0000");
    let removed = paint(&mut state, true);
    assert!(removed.contains("no longer retained"), "{removed}");
    assert!(state.ui.results.selected_soa_rule.is_none());
}

#[test]
fn soa_source_inspector_blocks_invalid_and_failed_evidence_until_repaired() {
    let mut state = soa_state(1, 32, 3.0);
    pick(&mut state);
    assert!(paint(&mut state, true).contains("SELECTED SOA RULE"));
    let valid = state.simulation.runs[0].analyses[0].clone();
    let Some(AnalysisResultPayload::Soa { evaluations, .. }) =
        state.simulation.runs[0].analyses[0].result_payload.as_mut()
    else {
        unreachable!()
    };
    evaluations[0].limit_value = f64::NAN;
    let invalid = paint(&mut state, true);
    assert!(!invalid.contains("SELECTED SOA RULE"), "{invalid}");
    assert!(state.ui.results.selected_soa_rule.is_some());
    state.simulation.runs[0].analyses[0] = valid;
    state.simulation.runs[0].analyses[0].success = false;
    let failed = paint(&mut state, true);
    assert!(!failed.contains("SELECTED SOA RULE"), "{failed}");
    assert!(state.ui.results.selected_soa_rule.is_some());
    state.simulation.runs[0].analyses[0].success = true;
    assert!(paint(&mut state, true).contains("SELECTED SOA RULE"));
}

#[test]
fn soa_source_unchanged_large_histories_and_clones_reuse_scans_and_envelopes() {
    use super::super::frame_work::WorkCounts;

    let mut original = soa_state(1, 100_000, 3.0);
    pick(&mut original);
    original.ui.results.soa_stress_trace_open = true;
    let before = envelope(&mut original);
    paint(&mut original, false);
    paint(&mut original, true);
    let mut fork = original.clone();
    let work = WorkCounts::reset();
    for state in [&mut original, &mut fork] {
        for _ in 0..4 {
            assert!(Arc::ptr_eq(&before, &envelope(state)));
            paint(state, false);
            paint(state, true);
        }
    }
    assert_eq!(work.since().total(), 0);
    replace_stress(&mut fork, 100_000, 3.2);
    assert_eq!(envelope(&mut fork).last().unwrap()[1], 3.2);
    assert!(Arc::ptr_eq(&before, &envelope(&mut original)));
}
