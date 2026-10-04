//! Selected sensitivity evidence and history-bound presentation cache.

use super::AnalysisPresentationKey;
use super::frame_work::{self, DatasetWalk};
use crate::state::{AnalysisResultPayload, RunHistoryRevision};
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::sensitivity as viewer;
use std::sync::Arc;
use viewer::{ActiveSensitivity, SensitivityView};
mod mismatch;
mod study;
pub(super) use mismatch::MismatchPlan;
pub(super) use study::{StudyPlan, domain_bar};

fn active_sensitivity(state: &AppState) -> ActiveSensitivity<'_> {
    let Some(analysis) = state.simulation.active_analysis() else {
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
    })
}
pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
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
    display: viewer::SensitivityPlan,
}
fn sensitivity_plan(state: &mut AppState) -> Option<Arc<SensitivityPlan>> {
    let source = (
        state.simulation.retained.runs.revision(),
        state.simulation.view.data_version,
    );
    let run = state.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.simulation.active_analysis()?);
    if let Some(plan) = state.ui.results.plans.sensitivity.as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
    {
        return Some(Arc::clone(plan));
    }
    let ActiveSensitivity::Ready(view) = active_sensitivity(state) else {
        return None;
    };
    frame_work::note(DatasetWalk::SensitivityRank);
    let built = Arc::new(SensitivityPlan {
        source,
        analysis: analysis_key,
        display: viewer::SensitivityPlan::new(view.rows),
    });
    state.ui.results.plans.sensitivity = Some(Arc::clone(&built));
    Some(built)
}
pub fn show(ui: &mut Ui, state: &mut AppState) {
    if study::serves_active_analysis(state) {
        study::show(ui, state);
        return;
    }
    if mismatch::serves_active_analysis(state) {
        mismatch::show(ui, state);
        return;
    }
    let plan = sensitivity_plan(state);
    viewer::show(
        ui,
        active_sensitivity(state),
        plan.as_deref().map(|plan| &plan.display),
    );
}
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    if study::serves_active_analysis(state) {
        study::right_panel(ui, state);
        return;
    }
    if mismatch::serves_active_analysis(state) {
        mismatch::right_panel(ui, state);
        return;
    }
    let plan = sensitivity_plan(state);
    viewer::right_panel(
        ui,
        active_sensitivity(state),
        plan.as_deref().map(|plan| &plan.display),
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
    fn state_with_analyses(analyses: Vec<AnalysisResult>) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        for analysis in analyses {
            run.add_analysis(analysis);
        }
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }
    fn ranked_state(parameters: usize) -> AppState {
        let rows = (0..parameters)
            .map(|index| SensitivityResultRow {
                parameter: format!("p{index:05}"),
                raw: (index as f64).into(),
                normalized: ((index as f64).sin()).into(),
            })
            .collect();
        let mut state = state_with_analyses(vec![sensitivity_result(1, "SENS", rows)]);
        assert!(state.simulation.select_analysis(0));
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
            active_sensitivity(&state),
            ActiveSensitivity::Missing
        ));
        assert!(state.simulation.select_analysis(1));
        let ActiveSensitivity::Ready(view) = active_sensitivity(&state) else {
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
            active_sensitivity(&state),
            ActiveSensitivity::Invalid
        ));
    }
    #[test]
    fn a_sensitivity_result_recorded_before_filters_is_labelled_design_parameters() {
        let mut state = state_with_analyses(vec![sensitivity_result(
            1,
            "SENS",
            vec![SensitivityResultRow {
                parameter: "GAIN".to_owned(),
                raw: 2.0.into(),
                normalized: 0.5.into(),
            }],
        )]);
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
                egui::CentralPanel::default().show(ctx, |ui| right_panel(ui, &mut state));
            },
        );
        let tree = output
            .platform_output
            .accesskit_update
            .expect("the sensitivity panel publishes an accessibility tree");
        assert!(
            tree.nodes.iter().any(|(_, node)| {
                node.value().is_some_and(|text| {
                    text == "Variables: design parameters · recorded before filters"
                })
            }),
            "{tree:?}"
        );
    }
    #[test]
    fn retained_view_source_sensitivity_rebuilds_shortened_rows_and_rejects_invalid_evidence() {
        let mut state = ranked_state(64);
        let original = sensitivity_plan(&mut state).unwrap();
        let version = state.simulation.view.data_version;
        state.simulation.retained.runs[0].analyses[0] = sensitivity_result(
            1,
            "SENS",
            vec![SensitivityResultRow {
                parameter: "only".to_owned(),
                raw: (1.0).into(),
                normalized: (-2.0).into(),
            }],
        );
        let changed = sensitivity_plan(&mut state).unwrap();
        assert_eq!(changed.analysis, original.analysis);
        assert_eq!(changed.display.order(), [0]);
        assert_eq!(changed.display.offsets().rows(), 1);
        assert_eq!(changed.display.max_magnitude(), 2.0);
        assert_eq!(original.display.order().len(), 64);

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, &mut state);
                right_panel(ui, &mut state);
            });
        });
        state.simulation.retained.runs[0].analyses[0].success = false;
        assert!(sensitivity_plan(&mut state).is_none());
        state.simulation.retained.runs[0].analyses[0].success = true;
        assert_eq!(sensitivity_plan(&mut state).unwrap().display.order(), [0]);
        assert_eq!(state.simulation.view.data_version, version);
    }
    #[test]
    fn the_ranking_is_sorted_once_per_dataset_generation() {
        let mut state = ranked_state(64);
        let first = sensitivity_plan(&mut state).expect("a ranked sensitivity plan");
        let again = sensitivity_plan(&mut state).expect("a ranked sensitivity plan");
        assert!(Arc::ptr_eq(&first, &again));

        let ActiveSensitivity::Ready(view) = active_sensitivity(&state) else {
            panic!("the fixture retains a sensitivity payload");
        };
        assert_eq!(
            first.display.order(),
            viewer::SensitivityPlan::new(view.rows).order()
        );
        assert_eq!(
            first.display.max_magnitude(),
            view.rows
                .iter()
                .filter_map(|row| row.normalized.value().map(f64::abs))
                .fold(0.0_f64, f64::max)
        );

        state.simulation.retained.runs[0].analyses[0].result_payload =
            Some(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![SensitivityResultRow {
                    parameter: "only".to_owned(),
                    raw: (1.0).into(),
                    normalized: (2.0).into(),
                }],
            });
        state.simulation.view.data_version = state.simulation.view.data_version.wrapping_add(1);

        let after = sensitivity_plan(&mut state).expect("a ranked sensitivity plan");
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
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state));
            },
        );
        let tree = output.platform_output.accesskit_update.unwrap();
        assert!(tree.nodes.iter().any(|(_, node)| node.value().is_some_and(|label|
            label.contains("Unranked, parameter gain, normalized sensitivity Unavailable (zero output), raw sensitivity 0.00000000000000000e0"))), "{tree:?}");
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
                egui::CentralPanel::default().show(ctx, |ui| right_panel(ui, &mut state));
            },
        );
        let drawn = output
            .platform_output
            .accesskit_update
            .expect("the sensitivity panel publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| {
                // A Label-role node carries its text in `value`, not `label`.
                node.value().is_some_and(|text| text.starts_with("p0"))
            })
            .count();
        assert!(drawn > 0, "the panel listed no parameters at all");
        assert!(
            drawn < 400,
            "the panel listed {drawn} of 4000 ranked parameters for a 900 px viewport"
        );
    }
    #[test]
    fn the_chart_row_offsets_are_built_once_beside_the_ranking_they_address() {
        let mut state = ranked_state(2_000);
        let plan = sensitivity_plan(&mut state).expect("a retained sensitivity result");

        assert_eq!(plan.display.offsets().rows(), plan.display.order().len());
        assert!(
            (plan.display.offsets().total_height() - 60_000.0).abs() < 1.0e-3,
            "the plan's extent is {}",
            plan.display.offsets().total_height()
        );
        assert!(
            std::sync::Arc::ptr_eq(
                &plan,
                &sensitivity_plan(&mut state).expect("the memo is served")
            ),
            "a second frame rebuilt the plan the offsets travel in"
        );
    }
}
