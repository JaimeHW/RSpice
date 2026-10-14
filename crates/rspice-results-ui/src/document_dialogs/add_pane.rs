//! Add a pane to the open result document.

use egui::Context;
use rspice_app_types::product::DatasetId;
use rspice_results::result_presentation::ResultViewer;
use rspice_ui_kit::widgets::{CommandForm, DialogChoice};

use super::labels_and_selection;

/// The source and sheet of the pane being added. The pane goes after the
/// selected pane, in the page's own layout.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddPaneDraft {
    pub dataset: Option<DatasetId>,
    pub analysis: Option<u64>,
    pub viewer: Option<ResultViewer>,
}

/// What the form offers, resolved by the host against the retained datasets.
pub struct AddPaneChoices {
    /// The selected pane the new one follows.
    pub anchor: String,
    pub datasets: Vec<(DatasetId, String)>,
    pub analyses: Vec<(u64, String)>,
    pub viewers: Vec<(ResultViewer, String)>,
    /// Why Add is refused, when it is.
    pub refusal: Option<String>,
}

pub fn show(ctx: &Context, draft: &mut AddPaneDraft, choices: &AddPaneChoices) -> DialogChoice {
    let refusal = choices.refusal.as_deref();
    CommandForm::new("Add pane", "Add")
        .context(Some(choices.anchor.as_str()))
        .describe("Add a result pane to this document after the selected pane.")
        .primary_enabled(refusal.is_none())
        .status(refusal.map_or(Ok(None), Err))
        .show(ctx, |rows| {
            let (labels, current) = labels_and_selection(&choices.datasets, draft.dataset);
            if let Some(index) =
                rows.select("Dataset", "rspice.add-pane.dataset", &current, &labels)
            {
                draft.dataset = Some(choices.datasets[index].0);
                draft.analysis = None;
            }
            let (labels, current) = labels_and_selection(&choices.analyses, draft.analysis);
            if let Some(index) =
                rows.select("Analysis", "rspice.add-pane.analysis", &current, &labels)
            {
                draft.analysis = Some(choices.analyses[index].0);
            }
            let (labels, current) = labels_and_selection(&choices.viewers, draft.viewer);
            if let Some(index) = rows.select("Viewer", "rspice.add-pane.viewer", &current, &labels)
            {
                draft.viewer = Some(choices.viewers[index].0);
            }
            None
        })
}
