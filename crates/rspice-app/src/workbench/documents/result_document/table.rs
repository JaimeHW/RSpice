//! Retained table source/cache selection and shared cursor action ownership.

use super::{
    ResultsState, exact_result_artifact_text,
    frame_work::{self, DatasetWalk},
    waves::cached_models,
};
use crate::{
    schematic::bus_notations,
    state::{AnalysisType, RunHistoryRevision},
    workbench::AppState,
};
use egui::Ui;
use rspice_results_ui::{presentation::well_hint, table as viewer};
use rspice_ui_kit::tokens::Tokens;
use std::cell::OnceCell;

#[derive(Debug)]
pub(super) struct ArtifactTextPlan {
    source: (RunHistoryRevision, u64),
    key: super::ResultArtifactPresentationKey,
    text: Result<String, String>,
    lines: Vec<(usize, usize)>,
}

fn artifact_text(
    state: &mut AppState,
    key: &super::ResultArtifactPresentationKey,
) -> Option<std::sync::Arc<ArtifactTextPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    if let Some(plan) = state.ui.results.plans.artifact.as_ref()
        && plan.source == source
        && &plan.key == key
    {
        return Some(std::sync::Arc::clone(plan));
    }
    let text = exact_result_artifact_text(key, &state.simulation.runs);
    let lines = text.as_ref().map_or_else(
        |_| Vec::new(),
        |text| {
            text.lines()
                .map(|line| {
                    // `lines` yields slices of `text`, so their offsets are
                    // exactly where each record starts and ends.
                    let start = line.as_ptr() as usize - text.as_ptr() as usize;
                    (start, start + line.len())
                })
                .collect()
        },
    );
    let built = std::sync::Arc::new(ArtifactTextPlan {
        source,
        key: key.clone(),
        text,
        lines,
    });
    state.ui.results.plans.artifact = Some(std::sync::Arc::clone(&built));
    Some(built)
}

fn show_operating_point_table(ui: &mut Ui, state: &mut AppState) -> bool {
    let Some(analysis) = state.simulation.active_analysis() else {
        return false;
    };
    if analysis.analysis_type != AnalysisType::DcOp {
        return false;
    }
    let row_count = viewer::operating_point_row_count(&analysis.data);
    if row_count == 0 {
        return false;
    }
    viewer::show_operating_point_table(ui, &analysis.data, row_count);
    true
}

fn show_selected_result_artifact_table(ui: &mut Ui, state: &mut AppState) -> bool {
    let Some(key) = state.ui.results.session.selected_result_artifact.clone() else {
        return false;
    };
    let Some(plan) = artifact_text(state, &key) else {
        return false;
    };
    viewer::show_artifact(ui, key.canonical_name(), &plan.text, &plan.lines);
    true
}

fn place_table_cursor(results: &mut ResultsState, analysis_index: usize, x: f64) {
    if !results.session.cursor_tool.is_armed() {
        results.session.toggle_cursor_tool();
    }
    if results.session.cursor_strip != Some(analysis_index) {
        results.session.clear_cursors();
    }
    let placing_a = results.session.cursor_a_is_next();
    results.session.cursor_strip = Some(analysis_index);
    results.session.cursors.place(x);
    if placing_a {
        // A table row has no unique trace anchor; retaining the plot's
        // previous anchor would make marker placement claim the wrong
        // waveform identity.
        results.session.cursor_a_anchor = None;
    }
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    if show_selected_result_artifact_table(ui, state) {
        return;
    }
    if show_operating_point_table(ui, state) {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    if models.is_empty() {
        well_hint(ui, "No results yet — run a simulation to fill the table");
        return;
    }

    let view = &state.ui.results.session.table;
    let Some(model) = models
        .iter()
        .find(|model| Some(model.analysis_key()) == view.analysis)
        .or_else(|| models.first())
    else {
        well_hint(ui, "The active run has no tabular analyses");
        return;
    };

    // Around-cursor mode follows cursor A only when it belongs to this
    // analysis; a cursor on another strip is not a window into this one.
    let cursor = (state.ui.results.session.cursor_strip == Some(model.analysis_index()))
        .then_some(state.ui.results.session.cursors.a)
        .flatten();
    let cursor_b = (state.ui.results.session.cursor_strip == Some(model.analysis_index()))
        .then_some(state.ui.results.session.cursors.b)
        .flatten();
    let notations = OnceCell::new();
    let Some(response) = viewer::show_samples(
        ui,
        viewer::SampleTable {
            model,
            cursor_a: cursor,
            cursor_b,
            significant_digits,
            quantity_policy,
        },
        view,
        |name| {
            notations
                .get_or_init(|| bus_notations(&state.workspace, &state.schematic))
                .display(name)
        },
        || frame_work::note(DatasetWalk::TableCursorScan),
    ) else {
        return;
    };
    if let Some(sample) = response.clicked_sample {
        place_table_cursor(&mut state.ui.results, model.analysis_index(), sample);
    }
    state.ui.results.session.table.analysis = Some(model.analysis_key());
    state.ui.results.session.table_status = Some(response.status);
}

pub fn inline_actions(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    if state.simulation.active_analysis().is_some_and(|analysis| {
        analysis.analysis_type == AnalysisType::DcOp
            && viewer::operating_point_row_count(&analysis.data) > 0
    }) {
        viewer::operating_point_controls(ui);
        return;
    }
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let notations = OnceCell::new();
    viewer::inline_actions(
        ui,
        viewer::TableControls {
            models: &models,
            cursor_strip: state.ui.results.session.cursor_strip,
            cursor_a: state.ui.results.session.cursors.a,
            status: state.ui.results.session.table_status.as_deref(),
        },
        &mut state.ui.results.session.table,
        |name| {
            notations
                .get_or_init(|| bus_notations(&state.workspace, &state.schematic))
                .display(name)
        },
    );
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    if let Some(analysis) = state.simulation.active_analysis()
        && analysis.analysis_type == AnalysisType::DcOp
        && viewer::operating_point_row_count(&analysis.data) > 0
    {
        viewer::operating_point_panel(ui, &analysis.data);
        return;
    }
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &t,
    );
    let analysis_key = state.ui.results.session.table.analysis;
    let Some(model) = models
        .iter()
        .find(|model| Some(model.analysis_key()) == analysis_key)
        .or_else(|| models.first())
    else {
        return;
    };
    viewer::sample_panel(ui, model);
}

#[cfg(test)]
mod tests {
    use super::super::AnalysisPresentationKey;
    use super::*;
    use crate::state::{AnalysisResult, AnalysisResultPayload};
    #[test]
    fn retained_view_source_artifact_tracks_restoration_errors_and_repair() {
        use super::super::ResultArtifactPresentationKey;

        use crate::state::{SimulationRunLifecycle, SimulationRunProvenance};

        let mut state = AppState::default();
        let run = state.simulation.start_run();
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
                AnalysisResultPayload::ScalarMeasurements {
                    values: std::collections::BTreeMap::from([(
                        "settling_time".to_owned(),
                        1.0 / 3.0,
                    )]),
                },
            ),
        );
        let key = ResultArtifactPresentationKey::new(
            AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
            "payload/scalar-measurements",
        );
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .unwrap();
        run.mark_running().unwrap();
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        state.simulation.complete_run();
        let original = artifact_text(&mut state, &key).unwrap();
        assert!(
            original
                .text
                .as_ref()
                .unwrap()
                .contains("0.3333333333333333")
        );
        let version = state.simulation.data_version;
        let mut replacement = state.simulation.clone();
        replacement.runs[0].analyses[0].result_payload =
            Some(AnalysisResultPayload::ScalarMeasurements {
                values: std::collections::BTreeMap::from([
                    ("settling_time".to_owned(), 0.125),
                    ("peak".to_owned(), 42.0),
                ]),
            });
        state.simulation = crate::io::simulation_state_from_results(
            crate::io::capture_simulation_results(&replacement),
        )
        .unwrap();
        let restored = artifact_text(&mut state, &key).unwrap();
        assert_eq!(
            restored.text,
            exact_result_artifact_text(&key, &state.simulation.runs)
        );
        assert_ne!(restored.text, original.text);
        let text = restored.text.as_ref().unwrap();
        assert!(text.contains("0.125") && text.contains("42.0"));
        assert_eq!(
            restored
                .lines
                .iter()
                .map(|(start, end)| &text[*start..*end])
                .collect::<Vec<_>>(),
            text.lines().collect::<Vec<_>>()
        );
        let payload = state.simulation.runs[0].analyses[0].result_payload.take();
        let missing = artifact_text(&mut state, &key).unwrap();
        assert!(missing.text.is_err());
        assert!(missing.lines.is_empty());
        state.simulation.runs[0].analyses[0].result_payload = payload;
        let repaired = artifact_text(&mut state, &key).unwrap();
        assert_eq!(repaired.text, restored.text);
        assert!(std::sync::Arc::ptr_eq(
            &repaired,
            &artifact_text(&mut state, &key).unwrap()
        ));
        assert_eq!(state.simulation.data_version, version);
    }

    #[test]
    fn row_activation_places_a_and_b_at_exact_samples() {
        let mut state = AppState::default();
        place_table_cursor(&mut state.ui.results, 3, 1.25e-6);
        assert_eq!(state.ui.results.session.cursor_strip, Some(3));
        assert_eq!(state.ui.results.session.cursors.a, Some(1.25e-6));
        assert_eq!(state.ui.results.session.cursors.b, None);

        place_table_cursor(&mut state.ui.results, 3, 8.5e-6);
        assert_eq!(state.ui.results.session.cursors.a, Some(1.25e-6));
        assert_eq!(state.ui.results.session.cursors.b, Some(8.5e-6));

        place_table_cursor(&mut state.ui.results, 3, 4.0e-6);
        assert_eq!(state.ui.results.session.cursors.a, Some(4.0e-6));
        assert_eq!(state.ui.results.session.cursors.b, None);
    }

    #[test]
    fn row_activation_rearms_and_rebinds_the_cursor_without_stale_b() {
        let mut state = AppState::default();
        state.ui.results.session.toggle_cursor_tool();
        assert!(!state.ui.results.session.cursor_tool.is_armed());

        place_table_cursor(&mut state.ui.results, 1, 1.0);
        place_table_cursor(&mut state.ui.results, 1, 2.0);
        place_table_cursor(&mut state.ui.results, 2, 9.0);

        assert!(state.ui.results.session.cursor_tool.is_armed());
        assert_eq!(state.ui.results.session.cursor_strip, Some(2));
        assert_eq!(state.ui.results.session.cursors.a, Some(9.0));
        assert_eq!(state.ui.results.session.cursors.b, None);
    }
}
