//! What the reader left a settings-keeping sheet showing, per authored
//! analysis: the Smith chart's markers and the network matrix's form and
//! format.
//!
//! They are the view's, as the readout pins are. Before the quick view
//! draws, each pane's settings are taken into the session, and what the
//! reader changed is written back through the view's one writer afterwards,
//! where undo can step it back. They name the authored analysis, not one
//! solve of it, so the next run of the plan opens with the same markers and
//! the same form.

use std::collections::HashMap;

use rspice_results::result_presentation::AnalysisPresentationSource;

use super::{AnalysisPresentationKey, ResultViewer, ResultsParts, ResultsReads};
use crate::results::visualization_document::{
    DocumentEdit, PaneId, SheetSettingValue, VisualizationDocument,
};

/// Every sheet's settings, by the authored analysis and the sheet.
pub(super) type Settings = HashMap<(AnalysisPresentationSource, ResultViewer), SheetSettingValue>;

/// The authored analysis the sheets draw: the shown run's active one.
fn shown(state: ResultsReads<'_>) -> Option<AnalysisPresentationSource> {
    let simulation = &state.model.simulation;
    let run = simulation.active_run()?;
    let analysis = simulation.active_analysis()?;
    Some(AnalysisPresentationKey::new(run.dataset_id, analysis).authored())
}

/// What `sheet` was left showing for the analysis shown.
pub(super) fn kept(state: ResultsReads<'_>, sheet: ResultViewer) -> Option<SheetSettingValue> {
    let source = shown(state)?;
    state
        .results
        .session
        .sheet_settings
        .get(&(source, sheet))
        .cloned()
}

/// Keep what the reader left `sheet` showing, or with `None` forget it.
pub(super) fn keep(
    state: &mut ResultsParts<'_>,
    sheet: ResultViewer,
    setting: Option<SheetSettingValue>,
) {
    let Some(source) = shown(state.results_reads()) else {
        return;
    };
    let settings = &mut state.results.session.sheet_settings;
    match setting {
        Some(setting) => {
            settings.insert((source, sheet), setting);
        }
        None => {
            settings.remove(&(source, sheet));
        }
    }
}

/// Take `pane_id`'s settings in `document` into the session for `key`.
pub(super) fn project(
    settings: &mut Settings,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
) {
    let source = key.authored();
    settings.retain(|(held, _), _| *held != source);
    settings.extend(
        document
            .sheet_settings()
            .iter()
            .filter(|setting| setting.pane_id == pane_id)
            .map(|setting| ((source, setting.setting.sheet()), setting.setting.clone())),
    );
}

/// The session's settings for the analyses `keys` name.
pub(super) fn scratch(settings: &Settings, keys: &[(AnalysisPresentationKey, PaneId)]) -> Settings {
    settings
        .iter()
        .filter(|((source, _), _)| keys.iter().any(|(key, _)| key.authored() == *source))
        .map(|(held, setting)| (*held, setting.clone()))
        .collect()
}

/// The edits that make `pane_id` in `document` hold the session's settings
/// for `key`.
pub(super) fn edits_for(
    settings: &Settings,
    document: &VisualizationDocument,
    key: AnalysisPresentationKey,
    pane_id: PaneId,
) -> Vec<DocumentEdit> {
    let source = key.authored();
    let held: HashMap<ResultViewer, &SheetSettingValue> = document
        .sheet_settings()
        .iter()
        .filter(|setting| setting.pane_id == pane_id)
        .map(|setting| (setting.setting.sheet(), &setting.setting))
        .collect();
    let wanted: HashMap<ResultViewer, &SheetSettingValue> = settings
        .iter()
        .filter(|((held, _), _)| *held == source)
        .map(|((_, sheet), setting)| (*sheet, setting))
        .collect();
    let set = wanted
        .iter()
        .filter(|(sheet, setting)| held.get(*sheet) != Some(*setting))
        .map(|(sheet, setting)| DocumentEdit::SetSheetSetting {
            pane_id,
            sheet: *sheet,
            setting: Some((*setting).clone()),
        });
    let forgotten = held
        .keys()
        .filter(|sheet| !wanted.contains_key(*sheet))
        .map(|sheet| DocumentEdit::SetSheetSetting {
            pane_id,
            sheet: *sheet,
            setting: None,
        });
    set.chain(forgotten).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
    use crate::results::visualization_document::{
        NetworkMatrixForm, NetworkMatrixFormat, SmithMarker,
    };
    use crate::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisType, RetainedTrace, SimulationRun,
    };
    use crate::workbench::documents::result_document::test_fixtures::{
        completed_prepared_run, digest,
    };
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;
    use crate::workbench::documents::result_document::view_edits;

    /// A finished run of `plan` solving the S-parameter `analysis`.
    fn plan_run(plan: SimulationPlanId, analysis: AnalysisInstanceId) -> SimulationRun {
        let mut run = completed_prepared_run(plan, ObjectRevision::INITIAL, digest(0x50));
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::SParameter, "SP")
                .with_waveforms(vec![RetainedTrace::new(
                    "|S11|",
                    vec![1.0e9, 2.0e9],
                    vec![0.5, 0.4],
                    "#ffbd2e",
                )])
                .with_provenance(
                    AnalysisResultProvenance::new(
                        analysis,
                        ObjectRevision::INITIAL,
                        digest(0x51),
                        Vec::new(),
                    )
                    .expect("analysis provenance"),
                ),
        );
        run
    }

    /// The Smith chart's markers and the network matrix's form and format
    /// are the view's: the next run of the plan, and the project reopened,
    /// show them again, and undo steps a change of form back.
    #[test]
    fn the_smith_markers_and_the_matrix_form_are_kept_by_the_view() {
        let (plan, analysis) = (SimulationPlanId::new(), AnalysisInstanceId::new());
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![plan_run(plan, analysis)].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        let markers = SheetSettingValue::Smith {
            markers: vec![
                SmithMarker {
                    trace: "S11".to_owned(),
                    frequency: 1.0e9,
                },
                SmithMarker {
                    trace: "S22".to_owned(),
                    frequency: 2.0e9,
                },
            ],
        };
        let impedance = SheetSettingValue::NetworkMatrix {
            form: NetworkMatrixForm::Z,
            format: NetworkMatrixFormat::DecibelAngle,
        };
        keep(
            &mut state.results_parts(),
            ResultViewer::Smith,
            Some(markers.clone()),
        );
        keep(
            &mut state.results_parts(),
            ResultViewer::NetworkMatrix,
            Some(impedance.clone()),
        );
        state.quick_frame();

        // The session forgets them, as a restart does; then the plan runs
        // again.
        super::super::working_view::release(&mut state.results_parts());
        state.results.results.session.sheet_settings.clear();
        state
            .model
            .simulation
            .retained
            .runs
            .insert(0, plan_run(plan, analysis));
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        let reads = state.results_reads();
        assert_eq!(kept(reads, ResultViewer::Smith), Some(markers.clone()));
        assert_eq!(
            kept(reads, ResultViewer::NetworkMatrix),
            Some(impedance.clone())
        );

        let mut reopened = state.reopened();
        reopened.quick_frame();
        assert_eq!(
            kept(reopened.results_reads(), ResultViewer::Smith),
            Some(markers),
            "the markers come back with the project"
        );

        keep(
            &mut state.results_parts(),
            ResultViewer::NetworkMatrix,
            Some(SheetSettingValue::NetworkMatrix {
                form: NetworkMatrixForm::S,
                format: NetworkMatrixFormat::RealImaginary,
            }),
        );
        state.quick_frame();
        let view = state.working_view().expect("the run's view").id();
        view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
            .expect("the change of form is undone");
        state.quick_frame();
        assert_eq!(
            kept(state.results_reads(), ResultViewer::NetworkMatrix),
            Some(impedance)
        );
    }
}
