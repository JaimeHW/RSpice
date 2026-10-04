//! Selected mismatch evidence and history-bound presentation cache.

use super::super::AnalysisPresentationKey;
use super::super::frame_work::{self, DatasetWalk};
use crate::state::{AnalysisResultPayload, RunHistoryRevision};
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::sensitivity::mismatch as viewer;
use std::sync::Arc;
use viewer::{ActiveMismatch, MismatchView};

pub(super) fn active_mismatch(state: &AppState) -> ActiveMismatch<'_> {
    let Some(analysis) = state.simulation.active_analysis() else {
        return ActiveMismatch::Missing;
    };
    let Some(payload @ AnalysisResultPayload::DcMismatch { evidence }) =
        analysis.result_payload.as_ref()
    else {
        return ActiveMismatch::Missing;
    };
    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return ActiveMismatch::Invalid;
    }
    ActiveMismatch::Ready(MismatchView {
        analysis_label: analysis.label.as_str(),
        evidence: evidence.as_ref(),
    })
}
pub(super) fn serves_active_analysis(state: &AppState) -> bool {
    state.simulation.active_analysis().is_some_and(|analysis| {
        matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::DcMismatch { .. })
        )
    })
}
pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    matches!(active_mismatch(state), ActiveMismatch::Ready(_))
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::workbench::documents::result_document) struct MismatchPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    display: viewer::MismatchPlan,
}
fn mismatch_plan(state: &mut AppState) -> Option<Arc<MismatchPlan>> {
    let source = (
        state.simulation.retained.runs.revision(),
        state.simulation.view.data_version,
    );
    let run = state.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.simulation.active_analysis()?);
    if let Some(plan) = state.ui.results.plans.mismatch.as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
    {
        return Some(Arc::clone(plan));
    }
    let ActiveMismatch::Ready(view) = active_mismatch(state) else {
        return None;
    };
    frame_work::note(DatasetWalk::MismatchCumulative);
    let built = Arc::new(MismatchPlan {
        source,
        analysis: analysis_key,
        display: viewer::MismatchPlan::new(view.evidence),
    });
    state.ui.results.plans.mismatch = Some(Arc::clone(&built));
    Some(built)
}
pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let plan = mismatch_plan(state);
    viewer::show(
        ui,
        active_mismatch(state),
        plan.as_deref().map(|plan| &plan.display),
    );
}
pub(super) fn right_panel(ui: &mut Ui, state: &AppState) {
    viewer::right_panel(ui, active_mismatch(state));
}

#[cfg(test)]
mod tests {
    use super::super::super::frame_work::WorkCounts;
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use rspice_results::dc_mismatch::DcMismatchEvidence;
    use viewer::evidence_fixture as evidence;
    fn state_with(evidence: DcMismatchEvidence) -> AppState {
        let analysis = AnalysisResult::new(1, AnalysisType::DcMismatch, "DCMATCH")
            .with_result_payload(AnalysisResultPayload::DcMismatch {
                evidence: Arc::new(evidence),
            });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }
    #[test]
    fn the_contribution_sheet_lists_dc_mismatch_contributors_in_the_engine_order() {
        let mut state = state_with(evidence());
        assert!(serves_active_analysis(&state));
        assert!(active_payload_is_valid(&state));
        let plan = mismatch_plan(&mut state).expect("a valid payload has a plan");
        let cumulative = plan.display.cumulative();
        assert_eq!(cumulative.len(), 2);
        assert!((cumulative[0] - 0.6).abs() < 1.0e-12, "{cumulative:?}");
        assert!((cumulative[1] - 0.8).abs() < 1.0e-12, "{cumulative:?}");
        assert!((plan.display.max_contribution() - 2.0e-3).abs() < 1.0e-18);

        let ActiveMismatch::Ready(view) = active_mismatch(&state) else {
            panic!("the active analysis carries the payload");
        };
        let order: Vec<&str> = view
            .evidence
            .contributors
            .iter()
            .map(|row| row.instance.as_str())
            .collect();
        assert_eq!(order, ["R1", "R2"], "the engine's order, never re-sorted");
    }
    #[test]
    fn an_idle_frame_does_not_rebuild_the_mismatch_plan() {
        let mut state = state_with(evidence());
        let baseline = WorkCounts::reset();
        mismatch_plan(&mut state).expect("the first look builds the plan");
        assert_eq!(baseline.since().get(DatasetWalk::MismatchCumulative), 1);

        let baseline = WorkCounts::reset();
        mismatch_plan(&mut state).expect("the memo answers the second look");
        mismatch_plan(&mut state).expect("and the third");
        assert_eq!(baseline.since().get(DatasetWalk::MismatchCumulative), 0);
    }
    #[test]
    fn the_contribution_sheet_still_serves_a_sensitivity_result_unchanged() {
        let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: crate::state::SensitivityResultMode::Dc,
                rows: vec![crate::state::SensitivityResultRow {
                    parameter: "r1".to_owned(),
                    raw: (1.0).into(),
                    normalized: (0.25).into(),
                }],
            });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));

        assert!(!serves_active_analysis(&state));
        assert!(super::super::active_payload_is_valid(&state));
        assert!(matches!(active_mismatch(&state), ActiveMismatch::Missing));
    }
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_the_dc_mismatch_sheet() {
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for width in [1000.0_f32, 1600.0] {
            for normalized in [true, false] {
                let mut evidence = evidence();
                evidence.normalized_contributions = normalized;
                let mut state = state_with(evidence);
                let canvas =
                    crate::ui::raster::render(egui::vec2(width, 420.0), |ui, background| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::NONE.fill(background))
                            .show(ui, |ui| show(ui, &mut state));
                    });
                let basis = if normalized { "share" } else { "absolute" };
                std::fs::write(
                    directory.join(format!("dc-mismatch-sheet-{width}-{basis}.png")),
                    canvas.png(420),
                )
                .expect("write sheet render");
            }
        }
    }
}
