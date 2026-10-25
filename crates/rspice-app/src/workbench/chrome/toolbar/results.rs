//! The Results workspace's toolbar, and whether the document it shows is
//! still current.

use crate::product::{ContentDigest, ObjectRevision};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{WorkbenchIcon, labeled_icon_button_sized};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::Workspace;

use super::{
    context_separator, explicit_label_width, take_projected_tool_slot, toolbar_icon_command,
};

pub(in crate::workbench) fn results_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    let historical = results_document_is_historical(app);
    if results_tracking_button(ui, app, layout) {
        context_separator(ui, layout);
    }
    toolbar_icon_command(
        ui,
        app,
        Command::ToggleLinkedCursors.key(),
        WorkbenchIcon::Target,
        layout,
    );
    context_separator(ui, layout);
    if !historical {
        toolbar_icon_command(
            ui,
            app,
            Command::WaveformCalculator.key(),
            WorkbenchIcon::Sliders,
            layout,
        );
        context_separator(ui, layout);
    }
    toolbar_icon_command(
        ui,
        app,
        Command::CompareResultDatasets.key(),
        WorkbenchIcon::Compare,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ImportResultDataset.key(),
        WorkbenchIcon::Folder,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::ExportWaveformsCsv.key(),
        WorkbenchIcon::Export,
        layout,
    );
    toolbar_icon_command(ui, app, ids::PRINT_HARDCOPY, WorkbenchIcon::File, layout);
}

fn results_tracking_button(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) -> bool {
    use crate::results::visualization_document::{
        DocumentEdit, ResultDocumentTracking, ResultDocumentTrackingMode,
    };
    use crate::workbench::state::WorkspaceDocumentId;

    let Some(WorkspaceDocumentId::VisualizationDocument(document_id)) = app
        .state
        .workbench
        .documents
        .active(Workspace::Results)
        .cloned()
    else {
        return false;
    };
    if results_document_is_historical(app) {
        if take_projected_tool_slot(ui, layout) {
            labeled_icon_button_sized(
                ui,
                WorkbenchIcon::History,
                "Historical",
                false,
                explicit_label_width("Historical"),
                layout.toolbar_control_height,
            )
            .on_hover_text(
                "This document still shows the results it was made from, but no completed run matches the current design and its netlist.",
            );
        }
        return true;
    }
    let Some((tracking, revision)) = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .map(|document| (document.tracking(), document.revision()))
    else {
        return false;
    };
    let (Some(plan_id), Some(analysis_id)) =
        (tracking.simulation_plan_id, tracking.authored_analysis_id)
    else {
        // A migrated or externally imported historical document cannot
        // truthfully claim Latest until exact plan and authored-analysis
        // authority exists.
        return false;
    };
    if !take_projected_tool_slot(ui, layout) {
        return true;
    }

    let pinned = tracking.mode == ResultDocumentTrackingMode::Pinned;
    let label = if pinned { "Pinned" } else { "Latest" };
    let tooltip = if pinned {
        "Pinned to this run's results; a rerun does not replace them."
    } else {
        "Tracking latest: this document follows the newest run of its plan and analysis."
    };
    let response = labeled_icon_button_sized(
        ui,
        WorkbenchIcon::Pin,
        label,
        pinned,
        explicit_label_width(label),
        layout.toolbar_control_height,
    );
    let response = response.on_hover_text(tooltip);
    if response.clicked() {
        let next_mode = if pinned {
            ResultDocumentTrackingMode::Latest
        } else {
            ResultDocumentTrackingMode::Pinned
        };
        let next = ResultDocumentTracking::for_plan(next_mode, plan_id, analysis_id);
        let result = app
            .state
            .model
            .workspace
            .content
            .transact_visualization_document(
                document_id,
                revision,
                vec![DocumentEdit::SetTracking(next)],
            )
            .map(|_| ())
            .map_err(|error| error.to_string());
        match result {
            Ok(()) => app.state.prompt(if pinned {
                "Result document now follows the latest plan run."
            } else {
                "Result document is pinned to its current dataset."
            }),
            Err(error) => app.state.prompt(error),
        }
    }
    true
}

fn current_result_source_digest(app: &RSpiceApp) -> Option<ContentDigest> {
    let netlist = &app.state.model.netlist_artifacts;
    (netlist.generation_error.is_none()
        && netlist.generated_input_digest.is_some()
        && netlist.generated_input_digest == netlist.current_generation_input_digest
        && !app
            .state
            .model
            .simulation
            .source
            .netlist_content
            .trim()
            .is_empty())
    .then(|| crate::state::content_digest(&app.state.model.simulation.source.netlist_content))
}

pub(crate) fn run_has_current_success_authority(
    run: &crate::state::SimulationRun,
    project_revision: ObjectRevision,
    source_digest: Option<ContentDigest>,
) -> bool {
    run.lifecycle == crate::state::SimulationRunLifecycle::Completed
        && run.success
        && run.prepared_receipt().is_some_and(|receipt| {
            receipt.project_revision() == project_revision
                && source_digest.is_some_and(|digest| receipt.source_content_digest() == digest)
        })
}

pub(crate) fn results_document_is_historical(app: &RSpiceApp) -> bool {
    use crate::workbench::state::WorkspaceDocumentId;

    let project_revision = app.state.model.workspace.content.project.revision();
    let source_digest = current_result_source_digest(app);
    match app.state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset_id)) => app
            .state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .filter(|run| run_has_current_success_authority(run, project_revision, source_digest))
            .max_by_key(|run| run.id)
            .is_none_or(|latest| latest.dataset_id != *dataset_id),
        Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => {
            let Some(document) = app
                .state
                .model
                .workspace
                .content
                .visualization_document(*document_id)
            else {
                return true;
            };
            let Some(binding) = document
                .panes()
                .iter()
                .filter_map(|pane| pane.binding)
                .next()
            else {
                return true;
            };
            let tracking = document.tracking();
            let (Some(plan_id), Some(analysis_id)) =
                (tracking.simulation_plan_id, tracking.authored_analysis_id)
            else {
                return true;
            };
            app.state
                .model.simulation
                .retained.runs
                .iter()
                .filter(|run| {
                    run_has_current_success_authority(run, project_revision, source_digest)
                        && run
                            .prepared_receipt()
                            .and_then(|receipt| receipt.simulation_plan_id())
                            == Some(plan_id)
                        && run.analyses.iter().any(|analysis| {
                            analysis.success
                                && crate::workbench::documents::result_document::analysis_matches_authored_source(
                                    analysis,
                                    analysis_id,
                                )
                        })
                })
                .max_by_key(|run| run.id)
                .is_none_or(|latest| latest.dataset_id != binding.dataset.dataset_id)
        }
        _ => false,
    }
}
