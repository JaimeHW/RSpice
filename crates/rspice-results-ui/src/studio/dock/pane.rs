//! Retained source selection and pane placement controls.
use super::super::widgets::dock_intro;
use egui::Ui;
use rspice_app_types::product::{DatasetId, short_identity as short_dataset};
use rspice_results::{
    result_presentation::ResultViewer, studio_presentation::VisualizationPanePlacement,
};

pub struct PaneDraft<'a> {
    pub viewer: &'a mut ResultViewer,
    pub dataset: &'a mut Option<DatasetId>,
    pub analysis: &'a mut Option<u64>,
    pub placement: &'a mut VisualizationPanePlacement,
    pub page_title: &'a mut String,
}
pub struct DatasetRow {
    pub id: DatasetId,
    pub label: String,
    pub first_analysis: Option<u64>,
}
pub struct AnalysisRow {
    pub id: u64,
    pub label: String,
    pub kind: &'static str,
}
pub trait PaneHost {
    fn prepare(&mut self) -> (Option<DatasetId>, Option<u64>);
    fn availability(
        &self,
        viewer: ResultViewer,
        dataset: Option<DatasetId>,
        analysis: Option<u64>,
    ) -> Result<ResultViewer, String>;
    fn draft(&mut self) -> PaneDraft<'_>;
    fn dataset_label(&self, dataset: DatasetId) -> Option<&str>;
    fn datasets(&self) -> Vec<DatasetRow>;
    fn selected_analysis(&self) -> Option<(u64, &str)>;
    fn analyses(&self, dataset: Option<DatasetId>) -> Vec<AnalysisRow>;
    fn create(&mut self) -> bool;
}

const NATIVE_VIEWERS: [ResultViewer; 12] = [
    ResultViewer::Waves,
    ResultViewer::DcSweep,
    ResultViewer::Bode,
    ResultViewer::Fft,
    ResultViewer::HarmonicBalance,
    ResultViewer::PhaseNoise,
    ResultViewer::Eye,
    ResultViewer::Hist,
    ResultViewer::Contribution,
    ResultViewer::Specs,
    ResultViewer::Smith,
    ResultViewer::PoleZero,
];

pub fn show(ui: &mut Ui, host: &mut impl PaneHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · WORKSHEET LAYOUT",
        "Create a compatible viewer pane without disturbing existing link groups.",
    );
    let (draft_dataset, draft_analysis) = host.prepare();
    let options = NATIVE_VIEWERS.map(|viewer| {
        (
            viewer,
            host.availability(viewer, draft_dataset, draft_analysis),
        )
    });
    egui::ComboBox::from_label("Viewer")
        .selected_text(host.draft().viewer.label())
        .show_ui(ui, |ui| {
            for (viewer, availability) in &options {
                let response = ui.add_enabled_ui(availability.is_ok(), |ui| {
                    ui.selectable_value(host.draft().viewer, *viewer, viewer.label())
                });
                if let Err(reason) = availability {
                    response.response.on_hover_text(reason);
                }
            }
        });

    let selected_dataset_text = draft_dataset
        .and_then(|dataset| host.dataset_label(dataset).map(|label| (dataset, label)))
        .map_or_else(
            || "Select retained dataset".to_owned(),
            |(dataset, label)| format!("{} · {}", label, short_dataset(dataset)),
        );
    egui::ComboBox::from_label("Dataset")
        .selected_text(selected_dataset_text)
        .show_ui(ui, |ui| {
            for DatasetRow {
                id: dataset_id,
                label,
                first_analysis,
            } in host.datasets()
            {
                if ui
                    .selectable_value(
                        host.draft().dataset,
                        Some(dataset_id),
                        format!("{} · {}", label, short_dataset(dataset_id)),
                    )
                    .clicked()
                {
                    *host.draft().analysis = first_analysis;
                }
            }
        });

    let draft_dataset = *host.draft().dataset;
    let selected_analysis_text = host.selected_analysis().map_or_else(
        || "Select retained analysis".to_owned(),
        |(id, label)| format!("{label} · {id}"),
    );
    ui.add_enabled_ui(draft_dataset.is_some(), |ui| {
        egui::ComboBox::from_label("Analysis")
            .selected_text(selected_analysis_text)
            .show_ui(ui, |ui| {
                for AnalysisRow {
                    id: analysis_id,
                    label,
                    kind,
                } in host.analyses(draft_dataset)
                {
                    ui.selectable_value(
                        host.draft().analysis,
                        Some(analysis_id),
                        format!("{label} · {kind} · {analysis_id}"),
                    );
                }
            });
    });

    let placement = *host.draft().placement;
    egui::ComboBox::from_label("Placement")
        .selected_text(placement.label())
        .show_ui(ui, |ui| {
            for placement in VisualizationPanePlacement::ALL {
                ui.selectable_value(host.draft().placement, placement, placement.label());
            }
        });
    if *host.draft().placement == VisualizationPanePlacement::NewWorksheetPage {
        ui.label("New page title");
        ui.text_edit_singleline(host.draft().page_title);
    }

    let selected_viewer = *host.draft().viewer;
    let selected_compatibility = options
        .iter()
        .find_map(|(viewer, availability)| (*viewer == selected_viewer).then_some(availability));
    let page_valid = *host.draft().placement != VisualizationPanePlacement::NewWorksheetPage
        || !host.draft().page_title.trim().is_empty();
    let enabled = selected_compatibility.is_some_and(Result::is_ok) && page_valid;
    ui.add_space(10.0);
    let add = ui
        .add_enabled(enabled, egui::Button::new("Add pane"))
        .on_disabled_hover_text(
            selected_compatibility
                .and_then(|result| result.as_ref().err())
                .map_or(
                    "A retained compatible result analysis is required",
                    String::as_str,
                ),
        )
        .clicked();
    if add {
        return host.create();
    }
    add
}
