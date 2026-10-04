//! Retained source qualification and authoritative pane creation.
use super::*;
use rspice_results_ui::studio::dock::pane::{
    self as presentation, AnalysisRow, DatasetRow, PaneDraft, PaneHost,
};

pub(super) fn add_pane_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::show(ui, &mut Host(app))
}
struct Host<'a>(&'a mut RSpiceApp);
impl PaneHost for Host<'_> {
    fn prepare(&mut self) -> (Option<DatasetId>, Option<u64>) {
        normalize_add_pane_draft(&mut self.0.state);
        let draft = &self.0.state.workbench.visualization_studio;
        (draft.draft_dataset_id, draft.draft_analysis_sequence)
    }
    fn availability(
        &self,
        viewer: ResultViewer,
        draft_dataset: Option<DatasetId>,
        draft_analysis: Option<u64>,
    ) -> Result<ResultViewer, String> {
        let app = &self.0;
        let definition = viewer.viewer_document_id().and_then(viewer_document);
        definition
            .ok_or_else(|| "Viewer document is not registered".to_owned())
            .and_then(|definition| {
                resolved_viewer_availability_for_binding(
                    &app.state,
                    definition,
                    draft_dataset,
                    draft_analysis,
                )
            })
    }
    fn draft(&mut self) -> PaneDraft<'_> {
        let s = &mut self.0.state.workbench.visualization_studio;
        PaneDraft {
            viewer: &mut s.draft_viewer,
            dataset: &mut s.draft_dataset_id,
            analysis: &mut s.draft_analysis_sequence,
            placement: &mut s.draft_pane_placement,
            page_title: &mut s.draft_page_title,
        }
    }
    fn dataset_label(&self, dataset: DatasetId) -> Option<&str> {
        self.0
            .state
            .simulation
            .retained
            .runs
            .iter()
            .find(|run| run.dataset_id == dataset)
            .map(|run| run.label.as_str())
    }
    fn datasets(&self) -> Vec<DatasetRow> {
        self.0
            .state
            .simulation
            .retained
            .runs
            .iter()
            .map(|run| DatasetRow {
                id: run.dataset_id,
                label: run.label.clone(),
                first_analysis: run.analyses.first().map(|a| a.id),
            })
            .collect()
    }
    fn selected_analysis(&self) -> Option<(u64, &str)> {
        selected_draft_analysis(&self.0.state)
            .map(|analysis| (analysis.id, analysis.label.as_str()))
    }
    fn analyses(&self, draft_dataset: Option<DatasetId>) -> Vec<AnalysisRow> {
        draft_dataset
            .and_then(|dataset_id| {
                self.0
                    .state
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .find(|run| run.dataset_id == dataset_id)
            })
            .map(|run| {
                run.analyses
                    .iter()
                    .map(|analysis| AnalysisRow {
                        id: analysis.id,
                        label: analysis.label.clone(),
                        kind: analysis_manifest_id(analysis.analysis_type),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
    fn create(&mut self) -> bool {
        let app = &mut self.0;
        let viewer = app.state.workbench.visualization_studio.draft_viewer;
        let Some(document_id) = viewer.viewer_document_id() else {
            app.state.push_user_message(ConsoleMessage::error(
                "Dataset-native result projections cannot be added as Visualization Studio panes",
            ));
            return false;
        };
        let dataset_id = app
            .state
            .workbench
            .visualization_studio
            .draft_dataset_id
            .expect("enabled add-pane action has a retained dataset");
        let analysis_sequence = app
            .state
            .workbench
            .visualization_studio
            .draft_analysis_sequence
            .expect("enabled add-pane action has a retained analysis");
        let placement = app
            .state
            .workbench
            .visualization_studio
            .draft_pane_placement;
        let page_title = app
            .state
            .workbench
            .visualization_studio
            .draft_page_title
            .trim()
            .to_owned();
        add_viewer_pane_bound(
            app,
            document_id,
            viewer,
            dataset_id,
            analysis_sequence,
            placement,
            page_title,
        );
        true
    }
}
