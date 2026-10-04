//! Comparison source snapshots and immediate receipt execution/commit.
use super::*;
use rspice_results_ui::studio::dock::comparison::{
    self as presentation, ComparisonDraft, ComparisonHost,
};

pub(super) fn comparison_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::show(
        ui,
        &mut Host {
            app,
            active_dataset: None,
            compatible_datasets: Vec::new(),
        },
    )
}
struct Host<'a> {
    app: &'a mut RSpiceApp,
    active_dataset: Option<DatasetId>,
    compatible_datasets: Vec<DatasetId>,
}
impl ComparisonHost for Host<'_> {
    fn prepare(&mut self) -> bool {
        let app = &mut self.app;
        let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
        let comparison_data_version = app.state.simulation.view.data_version;
        if app
            .state
            .workbench
            .visualization_studio
            .draft_comparison_data_version
            != comparison_data_version
        {
            let compatible_datasets = active_comparison_dataset_ids(&app.state);
            let studio = &mut app.state.workbench.visualization_studio;
            studio.draft_comparison_candidates = compatible_datasets;
            studio.draft_comparison_data_version = comparison_data_version;
        }
        let compatible_datasets = app
            .state
            .workbench
            .visualization_studio
            .draft_comparison_candidates
            .clone();
        if !app
            .state
            .workbench
            .visualization_studio
            .draft_comparison_dataset
            .is_some_and(|dataset| compatible_datasets.contains(&dataset))
        {
            app.state
                .workbench
                .visualization_studio
                .draft_comparison_dataset = compatible_datasets.first().copied();
        }
        self.active_dataset = active_dataset;
        self.compatible_datasets = compatible_datasets;
        active_dataset.is_some()
    }
    fn candidate(&self) -> Option<(DatasetId, &str)> {
        self.app
            .state
            .simulation
            .active_run()
            .map(|run| (run.dataset_id, run.label.as_str()))
    }
    fn selected_label(&self) -> Option<&str> {
        self.app
            .state
            .workbench
            .visualization_studio
            .draft_comparison_dataset
            .and_then(|dataset| {
                self.app
                    .state
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .find(|run| run.dataset_id == dataset)
                    .map(|run| run.label.as_str())
            })
    }
    fn baselines(
        &mut self,
    ) -> (
        &mut Option<DatasetId>,
        impl Iterator<Item = (DatasetId, &str)>,
    ) {
        let active_dataset = self.active_dataset;
        let compatible_datasets = &self.compatible_datasets;
        let state = &mut self.app.state;
        (
            &mut state
                .workbench
                .visualization_studio
                .draft_comparison_dataset,
            state
                .simulation
                .retained
                .runs
                .iter()
                .filter(move |run| {
                    Some(run.dataset_id) != active_dataset
                        && compatible_datasets.contains(&run.dataset_id)
                })
                .map(|run| (run.dataset_id, run.label.as_str())),
        )
    }
    fn draft(&mut self) -> ComparisonDraft<'_> {
        let s = &mut self.app.state.workbench.visualization_studio;
        ComparisonDraft {
            dataset: &mut s.draft_comparison_dataset,
            alignment: &mut s.draft_comparison_alignment,
            signal: &mut s.draft_comparison_alignment_signal,
            threshold: &mut s.draft_comparison_threshold,
            maximum_lag_samples: &mut s.draft_comparison_maximum_lag_samples,
            absolute_tolerance: &mut s.draft_comparison_absolute_tolerance,
            relative_tolerance: &mut s.draft_comparison_relative_tolerance,
            difference_trace: &mut s.draft_comparison_difference_trace,
        }
    }
    fn signals(&self) -> Vec<String> {
        let app = &self.app;
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_dataset
            .and_then(|baseline| comparison_signal_names_for_baseline(&app.state, baseline).ok())
            .unwrap_or_default()
    }
    fn create(&mut self) -> bool {
        let app = &mut self.app;
        match execute_comparison_draft_with_differences(app) {
            Ok(execution) => {
                let rows = execution.receipt.rows_compared;
                let disposition = execution.receipt.disposition;
                let difference_count = execution.difference_traces.len();
                let result = commit_comparison_execution(app, execution);
                if report_visualization_commit(app, result) {
                    app.state.push_user_message(ConsoleMessage::info(format!(
                    "Recorded comparison receipt for {rows} row(s) and {difference_count} retained difference trace set(s): {disposition:?}."
                )));
                    return true;
                }
                false
            }
            Err(error) => {
                app.state.push_user_message(ConsoleMessage::error(error));
                false
            }
        }
    }
}
