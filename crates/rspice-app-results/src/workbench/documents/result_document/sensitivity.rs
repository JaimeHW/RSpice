//! Selected sensitivity evidence and history-bound presentation cache.

use super::frame_work::{self, DatasetWalk};
use super::{AnalysisPresentationKey, ResultsParts, ResultsReads};
use crate::state::{AnalysisResultPayload, RunHistoryRevision};
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use egui::Ui;
use rspice_results_ui::sensitivity as viewer;
use std::sync::Arc;
use viewer::{ActiveSensitivity, SensitivityView};
mod mismatch;
mod study;
pub(super) use mismatch::MismatchPlan;
pub(super) use study::StudyPlan;

fn active_sensitivity(state: ResultsReads<'_>) -> ActiveSensitivity<'_> {
    let Some(analysis) = state.model.simulation.active_analysis() else {
        return ActiveSensitivity::Missing;
    };
    let Some(
        payload @ AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        },
    ) = analysis.result_payload.as_ref()
    else {
        return ActiveSensitivity::Missing;
    };

    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return ActiveSensitivity::Invalid;
    }

    ActiveSensitivity::Ready(SensitivityView {
        analysis_label: analysis.label.as_str(),
        output,
        result_mode: *result_mode,
        rows,
        quantity_policy: state.model.preferences.quantity_presentation_policy(),
    })
}

/// The sheet's bar: a frequency study's read point, then the filter that
/// narrows the chart and its table to the rows a name matches.
pub(in crate::workbench::documents::result_document) fn domain_bar(
    ui: &mut Ui,
    context: &mut super::SheetContext<'_>,
) -> bool {
    let study = context.simulation.active_analysis().and_then(|analysis| {
        match analysis.result_payload.as_ref() {
            Some(AnalysisResultPayload::SensitivityStudy { evidence }) => Some(evidence.as_ref()),
            _ => None,
        }
    });
    viewer::domain_bar(
        ui,
        study.map(|evidence| (evidence, &context.policy)),
        &mut context.results.session.study,
    )
}
pub(super) fn active_payload_is_valid(state: ResultsReads<'_>) -> bool {
    if study::serves_active_analysis(state) {
        return study::active_payload_is_valid(state);
    }
    if mismatch::serves_active_analysis(state) {
        return mismatch::active_payload_is_valid(state);
    }
    matches!(active_sensitivity(state), ActiveSensitivity::Ready(_))
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct SensitivityPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    /// The filter the listed rows were kept by.
    filter: String,
    display: viewer::SensitivityPlan,
}
fn sensitivity_plan(state: &mut ResultsParts<'_>) -> Option<Arc<SensitivityPlan>> {
    let source = (
        state.model.simulation.retained.runs.revision(),
        state.model.simulation.view.data_version,
    );
    let run = state.model.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.model.simulation.active_analysis()?);
    let filter = state.results.session.study.filter.as_str();
    if let Some(plan) = state.results.plans.sensitivity.as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
        && plan.filter == filter
    {
        return Some(Arc::clone(plan));
    }
    let ActiveSensitivity::Ready(view) = active_sensitivity(state.results_reads()) else {
        return None;
    };
    frame_work::note(DatasetWalk::SensitivityRank);
    let built = Arc::new(SensitivityPlan {
        source,
        analysis: analysis_key,
        filter: filter.to_owned(),
        display: viewer::SensitivityPlan::new(view.rows, filter),
    });
    state.results.plans.sensitivity = Some(Arc::clone(&built));
    Some(built)
}
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    if study::serves_active_analysis(state.results_reads()) {
        study::show(ui, state);
        return;
    }
    if mismatch::serves_active_analysis(state.results_reads()) {
        mismatch::show(ui, state);
        return;
    }
    let plan = sensitivity_plan(state);
    viewer::show(
        ui,
        active_sensitivity(state.results_reads()),
        plan.as_deref().map(|plan| &plan.display),
        significant_digits(state.results_reads()),
    );
}

/// The reader's significant digits, which a chart's values are stated at.
fn significant_digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    if study::serves_active_analysis(state.results_reads()) {
        study::right_panel(ui, state);
        return;
    }
    if mismatch::serves_active_analysis(state.results_reads()) {
        mismatch::right_panel(ui, state);
        return;
    }
    let plan = sensitivity_plan(state);
    viewer::right_panel(
        ui,
        active_sensitivity(state.results_reads()),
        plan.as_deref().map(|plan| &plan.display),
        significant_digits(state.results_reads()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use rspice_results::sensitivity::{
        SensitivityResultMode, SensitivityResultRow, SensitivityUnavailability, SensitivityValue,
    };
    fn sensitivity_result(id: u64, label: &str, rows: Vec<SensitivityResultRow>) -> AnalysisResult {
        AnalysisResult::new(id, AnalysisType::Sensitivity, label).with_result_payload(
            AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows,
            },
        )
    }
    fn state_with_analyses(analyses: Vec<AnalysisResult>) -> ResultsTestHost {
        let mut state = ResultsTestHost::default();
        let mut run = SimulationRun::new(1);
        for analysis in analyses {
            run.add_analysis(analysis);
        }
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state
    }
    fn ranked_state(parameters: usize) -> ResultsTestHost {
        let rows = (0..parameters)
            .map(|index| SensitivityResultRow {
                parameter: format!("p{index:05}"),
                raw: (index as f64).into(),
                normalized: ((index as f64).sin()).into(),
            })
            .collect();
        let mut state = state_with_analyses(vec![sensitivity_result(1, "SENS", rows)]);
        assert!(state.model.simulation.select_analysis(0));
        state
    }
    #[test]
    fn selection_reads_only_the_active_analysis() {
        let rows = vec![SensitivityResultRow {
            parameter: "r1".to_owned(),
            raw: (1.0).into(),
            normalized: (0.25).into(),
        }];
        let mut state = state_with_analyses(vec![
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN"),
            sensitivity_result(2, "SENS", rows),
        ]);

        assert!(matches!(
            active_sensitivity(state.results_reads()),
            ActiveSensitivity::Missing
        ));
        assert!(state.model.simulation.select_analysis(1));
        let ActiveSensitivity::Ready(view) = active_sensitivity(state.results_reads()) else {
            panic!("the selected sensitivity payload should be available");
        };
        assert_eq!(view.analysis_label, "SENS");
        assert_eq!(view.output, "V(out)");
        assert_eq!(view.rows.len(), 1);
    }
    #[test]
    fn invalid_payload_is_rejected_fail_closed() {
        let mut invalid = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS");
        invalid.result_payload = Some(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: vec![SensitivityResultRow {
                parameter: "r1".to_owned(),
                raw: (f64::NAN).into(),
                normalized: (1.0).into(),
            }],
        });
        let state = state_with_analyses(vec![invalid]);

        assert!(matches!(
            active_sensitivity(state.results_reads()),
            ActiveSensitivity::Invalid
        ));
    }
    #[test]
    fn retained_view_source_sensitivity_rebuilds_shortened_rows_and_rejects_invalid_evidence() {
        let mut state = ranked_state(64);
        let original = sensitivity_plan(&mut state.results_parts()).unwrap();
        let version = state.model.simulation.view.data_version;
        state.model.simulation.retained.runs[0].analyses[0] = sensitivity_result(
            1,
            "SENS",
            vec![SensitivityResultRow {
                parameter: "only".to_owned(),
                raw: (1.0).into(),
                normalized: (-2.0).into(),
            }],
        );
        let changed = sensitivity_plan(&mut state.results_parts()).unwrap();
        assert_eq!(changed.analysis, original.analysis);
        assert_eq!(changed.display.order(), [0]);
        assert_eq!(changed.display.offsets().rows(), 1);
        assert_eq!(changed.display.max_magnitude(), 2.0);
        assert_eq!(original.display.order().len(), 64);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, &mut state.results_parts());
                right_panel(ui, &mut state.results_parts());
            });
        });
        state.model.simulation.retained.runs[0].analyses[0].success = false;
        assert!(sensitivity_plan(&mut state.results_parts()).is_none());
        state.model.simulation.retained.runs[0].analyses[0].success = true;
        assert_eq!(
            sensitivity_plan(&mut state.results_parts())
                .unwrap()
                .display
                .order(),
            [0]
        );
        assert_eq!(state.model.simulation.view.data_version, version);
    }
    #[test]
    fn the_ranking_is_sorted_once_per_dataset_generation() {
        let mut state = ranked_state(64);
        let first =
            sensitivity_plan(&mut state.results_parts()).expect("a ranked sensitivity plan");
        let again =
            sensitivity_plan(&mut state.results_parts()).expect("a ranked sensitivity plan");
        assert!(Arc::ptr_eq(&first, &again));

        let ActiveSensitivity::Ready(view) = active_sensitivity(state.results_reads()) else {
            panic!("the fixture retains a sensitivity payload");
        };
        assert_eq!(
            first.display.order(),
            viewer::SensitivityPlan::new(view.rows, "").order()
        );
        assert_eq!(
            first.display.max_magnitude(),
            view.rows
                .iter()
                .filter_map(|row| row.normalized.value().map(f64::abs))
                .fold(0.0_f64, f64::max)
        );

        state.model.simulation.retained.runs[0].analyses[0].result_payload =
            Some(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![SensitivityResultRow {
                    parameter: "only".to_owned(),
                    raw: (1.0).into(),
                    normalized: (2.0).into(),
                }],
            });
        state.model.simulation.view.data_version =
            state.model.simulation.view.data_version.wrapping_add(1);

        let after =
            sensitivity_plan(&mut state.results_parts()).expect("a ranked sensitivity plan");
        assert_eq!(after.display.order(), [0]);
        assert_eq!(after.display.max_magnitude(), 2.0);
    }
    #[test]
    fn unavailable_sensitivity_renders_as_unranked_with_accessible_reason() {
        let mut state = state_with_analyses(vec![sensitivity_result(
            1,
            "SENS",
            vec![SensitivityResultRow {
                parameter: "gain".to_owned(),
                raw: 0.0.into(),
                normalized: SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput),
            }],
        )]);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
            },
        );
        let tree = output.platform_output.accesskit_update.unwrap();
        assert!(tree.nodes.iter().any(|(_, node)| node.value().is_some_and(|label|
            label.contains("Unranked, parameter gain, normalized sensitivity Unavailable (zero output), raw sensitivity 0"))), "{tree:?}");
    }
    #[test]
    fn the_panel_lists_only_the_rows_its_viewport_shows() {
        let mut state = ranked_state(4_000);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(520.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| right_panel(ui, &mut state.results_parts()));
            },
        );
        // The table paints only the rows its viewport shows; count the
        // names it painted.
        fn painted(shape: &egui::Shape, prefix: &str) -> usize {
            match shape {
                egui::Shape::Text(text) => usize::from(text.galley.job.text.starts_with(prefix)),
                egui::Shape::Vec(inner) => inner.iter().map(|shape| painted(shape, prefix)).sum(),
                _ => 0,
            }
        }
        let drawn: usize = output
            .shapes
            .iter()
            .map(|clipped| painted(&clipped.shape, "p0"))
            .sum();
        assert!(drawn > 0, "the panel listed no parameters at all");
        assert!(
            drawn < 400,
            "the panel listed {drawn} of 4000 ranked parameters for a 900 px viewport"
        );
    }
}
