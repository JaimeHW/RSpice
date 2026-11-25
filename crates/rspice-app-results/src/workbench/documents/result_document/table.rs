//! Retained table source/cache selection and shared cursor action ownership.

use super::{
    ResultsState, exact_result_artifact_text,
    frame_work::{self, DatasetWalk},
    waves::cached_models,
};
use crate::state::{AnalysisType, RunHistoryRevision, bus_notations};
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
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
    state: &mut ResultsParts<'_>,
    key: &super::ResultArtifactPresentationKey,
) -> Option<std::sync::Arc<ArtifactTextPlan>> {
    let source = (
        state.model.simulation.retained.runs.revision(),
        state.model.simulation.view.data_version,
    );
    if let Some(plan) = state.results.plans.artifact.as_ref()
        && plan.source == source
        && &plan.key == key
    {
        return Some(std::sync::Arc::clone(plan));
    }
    let text = exact_result_artifact_text(key, &state.model.simulation.retained.runs);
    let lines = text
        .as_ref()
        .map_or_else(|_| Vec::new(), |text| record_spans(text));
    let built = std::sync::Arc::new(ArtifactTextPlan {
        source,
        key: key.clone(),
        text,
        lines,
    });
    state.results.plans.artifact = Some(std::sync::Arc::clone(&built));
    Some(built)
}

/// Where each record of `text` starts and ends.
fn record_spans(text: &str) -> Vec<(usize, usize)> {
    text.lines()
        .map(|line| {
            // `lines` yields slices of `text`, so their offsets are
            // exactly where each record starts and ends.
            let start = line.as_ptr() as usize - text.as_ptr() as usize;
            (start, start + line.len())
        })
        .collect()
}

/// The table the sheet shows, in its words, for a printed page: the
/// evidence the reader selected, the operating point, or the samples the
/// controls select, chosen among as [`show`] chooses. Nothing where the
/// sheet shows no table.
pub(super) fn printed(state: ResultsReads<'_>) -> viewer::SheetPrint {
    let simulation = &state.model.simulation;
    if let Some(key) = state.results.session.selected_result_artifact.as_ref() {
        let source = (
            simulation.retained.runs.revision(),
            simulation.view.data_version,
        );
        let text = match state.results.plans.artifact.as_ref() {
            Some(plan) if plan.source == source && &plan.key == key => plan.text.clone(),
            _ => exact_result_artifact_text(key, &simulation.retained.runs),
        };
        return text.map_or_else(
            |_| viewer::SheetPrint::default(),
            |text| {
                viewer::SheetPrint::table(
                    None,
                    key.canonical_name().to_owned(),
                    viewer::printed_artifact(&text, &record_spans(&text)),
                )
            },
        );
    }
    if let Some(analysis) = simulation.active_analysis()
        && analysis.analysis_type == AnalysisType::DcOp
        && let Some(table) = viewer::printed_operating_point(&analysis.data)
    {
        return viewer::SheetPrint::table(Some(analysis.id), "Operating point".to_owned(), table);
    }
    let (Some(models), Some(analysis)) = (
        state.results.models.last_built(),
        super::shown_analysis(simulation),
    ) else {
        return viewer::SheetPrint::default();
    };
    let Some(model) = selected_model(models, simulation) else {
        return viewer::SheetPrint::default();
    };
    let presentation = state.presentation_policy();
    let session = &state.results.session;
    // Chosen columns belong to the analysis they were chosen in, as on the
    // sheet; another analysis prints its own signals.
    let fresh = viewer::TableView::default();
    let view = if session.table.analysis == Some(model.analysis_key()) {
        &session.table
    } else {
        &fresh
    };
    let on_strip = session.cursor_strip == Some(model.analysis_index());
    let notations = OnceCell::new();
    viewer::printed_samples(
        viewer::SampleTable {
            model,
            cursor_a: on_strip.then_some(session.cursors.a).flatten(),
            cursor_b: on_strip.then_some(session.cursors.b).flatten(),
            significant_digits: usize::from(presentation.displayed_significant_digits().get()),
            quantity_policy: state.model.preferences.quantity_presentation_policy(),
        },
        view,
        |name| {
            notations
                .get_or_init(|| bus_notations(&state.model.workspace, &state.model.schematic))
                .display(name)
        },
        || frame_work::note(DatasetWalk::TableCursorScan),
    )
    .map_or_else(viewer::SheetPrint::default, |(title, table)| {
        viewer::SheetPrint::table(Some(analysis.id), title, table)
    })
}

/// Whether the sheet shows a table to print, without reading its words:
/// what [`printed`] would find.
pub(super) fn is_printable(
    results: &ResultsState,
    simulation: &crate::state::SimulationState,
) -> bool {
    if let Some(key) = results.session.selected_result_artifact.as_ref() {
        return results
            .plans
            .artifact
            .as_ref()
            .is_some_and(|plan| &plan.key == key && plan.text.is_ok());
    }
    if simulation.active_analysis().is_some_and(|analysis| {
        analysis.analysis_type == AnalysisType::DcOp
            && viewer::operating_point_row_count(&analysis.data) > 0
    }) {
        return true;
    }
    results
        .models
        .last_built()
        .and_then(|models| selected_model(models, simulation))
        .is_some_and(|model| {
            let fresh = viewer::TableView::default();
            let view = if results.session.table.analysis == Some(model.analysis_key()) {
                &results.session.table
            } else {
                &fresh
            };
            viewer::shows_samples(model, view)
        })
}

fn show_operating_point_table(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let Some(analysis) = state.model.simulation.active_analysis() else {
        return false;
    };
    if analysis.analysis_type != AnalysisType::DcOp {
        return false;
    }
    let row_count = viewer::operating_point_row_count(&analysis.data);
    if row_count == 0 {
        return false;
    }
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    viewer::show_operating_point_table(ui, &analysis.data, row_count, &quantity_policy);
    true
}

fn show_selected_result_artifact_table(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let Some(key) = state.results.session.selected_result_artifact.clone() else {
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

/// The strip model of the analysis the data browser shows, which is the one
/// the table shows.
fn selected_model<'m>(
    models: &'m [rspice_results_ui::waves::StripModel],
    simulation: &crate::state::SimulationState,
) -> Option<&'m rspice_results_ui::waves::StripModel> {
    let analysis = super::shown_analysis(simulation)?;
    let key = super::AnalysisPresentationKey::new(simulation.active_run()?.dataset_id, analysis);
    models.iter().find(|model| model.analysis_key() == key)
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    if show_selected_result_artifact_table(ui, state) {
        return;
    }
    if show_operating_point_table(ui, state) {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let presentation = state.results_reads().presentation_policy();
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let models = cached_models(
        &state.model.simulation,
        state.results,
        presentation.complex_number_display(),
        &t,
    );
    if models.is_empty() {
        well_hint(ui, "No results yet — run a simulation");
        return;
    }

    let Some(model) = selected_model(&models, &state.model.simulation) else {
        well_hint(ui, "The selected analysis has no signals to tabulate");
        return;
    };
    // Chosen columns belong to the analysis they were chosen in; another
    // analysis starts from its own signals.
    let key = model.analysis_key();
    let view = &mut state.results.session.table;
    if view.analysis != Some(key) {
        view.columns.clear();
        view.analysis = Some(key);
    }
    let view = &state.results.session.table;

    // Around-cursor mode follows cursor A only when it belongs to this
    // analysis; a cursor on another strip is not a window into this one.
    let cursor = (state.results.session.cursor_strip == Some(model.analysis_index()))
        .then_some(state.results.session.cursors.a)
        .flatten();
    let cursor_b = (state.results.session.cursor_strip == Some(model.analysis_index()))
        .then_some(state.results.session.cursors.b)
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
                .get_or_init(|| bus_notations(&state.model.workspace, &state.model.schematic))
                .display(name)
        },
        || frame_work::note(DatasetWalk::TableCursorScan),
    ) else {
        return;
    };
    if let Some(sample) = response.clicked_sample {
        place_table_cursor(state.results, model.analysis_index(), sample);
    }
    state.results.session.table_status = Some(response.status);
}

pub fn inline_actions(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let t = Tokens::get(ui.ctx());
    if state
        .model
        .simulation
        .active_analysis()
        .is_some_and(|analysis| {
            analysis.analysis_type == AnalysisType::DcOp
                && viewer::operating_point_row_count(&analysis.data) > 0
        })
    {
        viewer::operating_point_controls(ui);
        return;
    }
    let presentation = state.results_reads().presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        state.results,
        presentation.complex_number_display(),
        &t,
    );
    let notations = OnceCell::new();
    viewer::inline_actions(
        ui,
        viewer::TableControls {
            model: selected_model(&models, &state.model.simulation),
            cursor_strip: state.results.session.cursor_strip,
            cursor_a: state.results.session.cursors.a,
            status: state.results.session.table_status.as_deref(),
        },
        &mut state.results.session.table,
        |name| {
            notations
                .get_or_init(|| bus_notations(&state.model.workspace, &state.model.schematic))
                .display(name)
        },
    );
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

        let mut state = ResultsTestHost::default();
        let run = state.model.simulation.start_run();
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
        state.model.simulation.complete_run();
        let original = artifact_text(&mut state.results_parts(), &key).unwrap();
        assert!(
            original
                .text
                .as_ref()
                .unwrap()
                .contains("0.3333333333333333")
        );
        let version = state.model.simulation.view.data_version;
        let mut replacement = state.model.simulation.clone();
        replacement.retained.runs[0].analyses[0].result_payload =
            Some(AnalysisResultPayload::ScalarMeasurements {
                values: std::collections::BTreeMap::from([
                    ("settling_time".to_owned(), 0.125),
                    ("peak".to_owned(), 42.0),
                ]),
            });
        state.model.simulation = crate::io::simulation_state_from_results(
            crate::io::capture_simulation_results(&replacement),
        )
        .unwrap();
        let restored = artifact_text(&mut state.results_parts(), &key).unwrap();
        assert_eq!(
            restored.text,
            exact_result_artifact_text(&key, &state.model.simulation.retained.runs)
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
        let payload = state.model.simulation.retained.runs[0].analyses[0]
            .result_payload
            .take();
        let missing = artifact_text(&mut state.results_parts(), &key).unwrap();
        assert!(missing.text.is_err());
        assert!(missing.lines.is_empty());
        state.model.simulation.retained.runs[0].analyses[0].result_payload = payload;
        let repaired = artifact_text(&mut state.results_parts(), &key).unwrap();
        assert_eq!(repaired.text, restored.text);
        assert!(std::sync::Arc::ptr_eq(
            &repaired,
            &artifact_text(&mut state.results_parts(), &key).unwrap()
        ));
        assert_eq!(state.model.simulation.view.data_version, version);
    }

    #[test]
    fn row_activation_places_a_and_b_at_exact_samples() {
        let mut state = ResultsTestHost::default();
        place_table_cursor(&mut state.results.results, 3, 1.25e-6);
        assert_eq!(state.results.results.session.cursor_strip, Some(3));
        assert_eq!(state.results.results.session.cursors.a, Some(1.25e-6));
        assert_eq!(state.results.results.session.cursors.b, None);

        place_table_cursor(&mut state.results.results, 3, 8.5e-6);
        assert_eq!(state.results.results.session.cursors.a, Some(1.25e-6));
        assert_eq!(state.results.results.session.cursors.b, Some(8.5e-6));

        // A third activation deletes nothing: both cursors stay.
        place_table_cursor(&mut state.results.results, 3, 4.0e-6);
        assert_eq!(state.results.results.session.cursors.a, Some(1.25e-6));
        assert_eq!(state.results.results.session.cursors.b, Some(8.5e-6));
    }

    /// The data browser picks the analysis the table shows; columns chosen
    /// in one analysis are not carried into the next.
    #[test]
    fn the_table_shows_the_analysis_the_data_browser_picked() {
        use crate::state::{RetainedTrace, SimulationRun};
        let tran = |id, name: &str, signal: &str| {
            AnalysisResult::new(id, AnalysisType::Transient, name).with_waveforms(vec![
                RetainedTrace::new(signal, vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
            ])
        };
        let mut run = SimulationRun::new(1);
        run.add_analysis(tran(1, "TRAN A", "V(a)"));
        run.add_analysis(tran(2, "TRAN B", "V(b)"));
        let keys =
            [0, 1].map(|index| AnalysisPresentationKey::new(run.dataset_id, &run.analyses[index]));
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let draw = |state: &mut ResultsTestHost| {
            state.quick_frame();
            let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
            });
        };

        assert!(state.model.simulation.select_analysis(1));
        draw(&mut state);
        assert_eq!(state.results.results.session.table.analysis, Some(keys[1]));

        state.results.results.session.table.columns = vec![super::super::TracePresentationKey {
            source_name: "V(b)".to_owned(),
            kind: 0,
            family_group: 0,
        }];
        assert!(state.model.simulation.select_analysis(0));
        draw(&mut state);
        assert_eq!(state.results.results.session.table.analysis, Some(keys[0]));
        assert!(state.results.results.session.table.columns.is_empty());
    }

    #[test]
    fn row_activation_rearms_and_rebinds_the_cursor_without_stale_b() {
        let mut state = ResultsTestHost::default();
        state.results.results.session.toggle_cursor_tool();
        assert!(!state.results.results.session.cursor_tool.is_armed());

        place_table_cursor(&mut state.results.results, 1, 1.0);
        place_table_cursor(&mut state.results.results, 1, 2.0);
        place_table_cursor(&mut state.results.results, 2, 9.0);

        assert!(state.results.results.session.cursor_tool.is_armed());
        assert_eq!(state.results.results.session.cursor_strip, Some(2));
        assert_eq!(state.results.results.session.cursors.a, Some(9.0));
        assert_eq!(state.results.results.session.cursors.b, None);
    }
}
