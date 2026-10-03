//! Selected study evidence and history-bound presentation cache.

use super::super::AnalysisPresentationKey;
use super::super::frame_work::{self, DatasetWalk};
use crate::state::{AnalysisResultPayload, RunHistoryRevision};
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::sensitivity::study as viewer;
use std::sync::Arc;
use viewer::{ActiveStudy, StudyView};

pub(super) fn active_study(state: &AppState) -> ActiveStudy<'_> {
    let Some(analysis) = state.simulation.active_analysis() else {
        return ActiveStudy::Missing;
    };
    let Some(payload @ AnalysisResultPayload::SensitivityStudy { evidence }) =
        analysis.result_payload.as_ref()
    else {
        return ActiveStudy::Missing;
    };
    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return ActiveStudy::Invalid;
    }
    ActiveStudy::Ready(StudyView {
        analysis_label: analysis.label.as_str(),
        evidence: evidence.as_ref(),
    })
}
pub(super) fn serves_active_analysis(state: &AppState) -> bool {
    state.simulation.active_analysis().is_some_and(|analysis| {
        matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::SensitivityStudy { .. })
        )
    })
}
pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    matches!(active_study(state), ActiveStudy::Ready(_))
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::workbench::documents::result_document) struct StudyPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    display: viewer::StudyPlan,
}
fn study_plan(state: &mut AppState) -> Option<Arc<StudyPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let run = state.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.simulation.active_analysis()?);
    let ActiveStudy::Ready(view) = active_study(state) else {
        return None;
    };
    let index = state.ui.results.study.read_index(view.evidence);
    if let Some(plan) = state.ui.results.plans.study.as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
        && plan.display.index() == index
    {
        return Some(Arc::clone(plan));
    }
    frame_work::note(DatasetWalk::SensitivityRank);
    let built = Arc::new(StudyPlan {
        source,
        analysis: analysis_key,
        display: viewer::StudyPlan::new(view.evidence, index),
    });
    state.ui.results.plans.study = Some(Arc::clone(&built));
    Some(built)
}
pub(in crate::workbench::documents::result_document) fn domain_bar(
    ui: &mut Ui,
    context: &mut super::super::SheetContext<'_>,
) -> bool {
    let Some(analysis) = context.simulation.active_analysis() else {
        return false;
    };
    let Some(AnalysisResultPayload::SensitivityStudy { evidence }) =
        analysis.result_payload.as_ref()
    else {
        return false;
    };
    viewer::domain_bar(ui, evidence, &mut context.results.study)
}
pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let plan = study_plan(state);
    viewer::show(
        ui,
        active_study(state),
        plan.as_deref().map(|plan| &plan.display),
    );
}
pub(super) fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let plan = study_plan(state);
    viewer::right_panel(
        ui,
        active_study(state),
        plan.as_deref().map(|plan| &plan.display),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use rspice_results::sensitivity::{
        SensitivityBasisEvidence, SensitivityStudyEvidence, SensitivityStudyRow, SensitivityValue,
    };
    use viewer::swept_evidence;
    pub(super) fn state_with(evidence: SensitivityStudyEvidence) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS").with_result_payload(
                AnalysisResultPayload::SensitivityStudy {
                    evidence: Arc::new(evidence),
                },
            ),
        );
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));
        state
    }
    #[test]
    fn a_study_is_ranked_once_per_dataset_generation() {
        let mut state = state_with(swept_evidence());
        let first = study_plan(&mut state).expect("a retained study ranks");
        let again = study_plan(&mut state).expect("the memo is served");
        assert!(Arc::ptr_eq(&first, &again));
        // Read at the sweep's first frequency, where R1 has the larger
        // magnitude even though PARAM:GAIN dominates further up the band.
        assert_eq!(first.display.order(), [1, 0]);
        assert_eq!(first.display.max_magnitude(), 1.0);
        assert_eq!(first.display.offsets().rows(), 2);
    }
    #[test]
    fn the_contribution_sheet_ranks_a_sweep_at_the_selected_frequency() {
        let mut state = state_with(swept_evidence());
        assert_eq!(study_plan(&mut state).unwrap().display.order(), [1, 0]);

        // At the middle frequency PARAM:GAIN dominates the band.
        state.ui.results.study.frequency_index = 1;
        let middle = study_plan(&mut state).expect("the ranking follows the reader");
        assert_eq!(middle.display.index(), 1);
        assert_eq!(middle.display.order(), [0, 1]);
        assert_eq!(middle.display.max_magnitude(), 9.0);

        // A selection past the end of a shorter study is clamped on read, and
        // the stored index is left exactly as the reader set it.
        state.ui.results.study.frequency_index = 97;
        let clamped = study_plan(&mut state).expect("a clamped selection still ranks");
        assert_eq!(clamped.display.index(), 2);
        assert_eq!(state.ui.results.study.frequency_index, 97);
    }
    #[test]
    fn only_the_active_analysis_and_only_a_valid_payload_is_drawn() {
        let mut state = state_with(swept_evidence());
        assert!(serves_active_analysis(&state));
        assert!(active_payload_is_valid(&state));

        state.simulation.runs[0].analyses[0].success = false;
        assert!(serves_active_analysis(&state), "the payload is still there");
        assert!(matches!(active_study(&state), ActiveStudy::Invalid));
        assert!(study_plan(&mut state).is_none());
    }
    #[test]
    fn the_panel_lists_only_the_rows_its_viewport_shows() {
        let rows = (0..2_000)
            .map(|index| SensitivityStudyRow {
                parameter: format!("R{index:05}"),
                nominal_value: 1000.0,
                raw: vec![SensitivityValue::Available(index as f64)],
                normalized: vec![SensitivityValue::Available((index as f64).sin())],
                phase: Vec::new(),
            })
            .collect();
        let mut state = state_with(SensitivityStudyEvidence {
            output: "V(OUT)".to_owned(),
            filter: String::new(),
            basis: SensitivityBasisEvidence::Dc { output: 1.0 },
            rows,
        });
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
            .expect("the study panel publishes an accessibility tree")
            .nodes
            .iter()
            // A Label-role node carries its text in `value`, not `label`.
            .filter(|(_, node)| node.value().is_some_and(|text| text.starts_with("R0")))
            .count();
        assert!(drawn > 0, "the panel listed no variables at all");
        assert!(
            drawn < 400,
            "the panel listed {drawn} of 2000 ranked variables for a 900 px viewport"
        );
    }
    #[test]
    fn a_swept_study_paints_without_borrowing_its_own_plan_twice() {
        let mut state = state_with(swept_evidence());
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 700.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(ui, &mut state);
                    right_panel(ui, &mut state);
                });
            },
        );
    }
}
