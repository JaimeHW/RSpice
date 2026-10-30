//! Inserting an exact visualization figure into a project report.

use super::*;
use crate::results::report_document::{FigureSizing, PlotFigureBlock, ReportFigureSourceLocator};
use crate::results::visualization_document::{PageId, PaneId};
use crate::results::visualization_raster::{
    VisualizationRasterProfile, render_visualization_report_figure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReportFigureOption {
    document_index: usize,
    page_id: PageId,
    pane_id: PaneId,
    label: String,
}

pub(super) fn report_figure_options(model: &Model) -> Vec<ReportFigureOption> {
    let mut options = Vec::new();
    for (document_index, document) in model
        .workspace
        .content
        .visualization_documents
        .iter()
        .enumerate()
    {
        for page in document.pages() {
            let mut panes = document
                .panes()
                .iter()
                .filter(|pane| pane.page_id == page.id)
                .collect::<Vec<_>>();
            panes.sort_by_key(|pane| (pane.order, pane.id.get()));
            options.extend(panes.into_iter().map(|pane| ReportFigureOption {
                document_index,
                page_id: page.id,
                pane_id: pane.id,
                label: format!("{} · {} · {}", document.title(), page.title, pane.title),
            }));
        }
    }
    options
}

pub fn open_insert_result_document(cx: &mut Cx<'_, HardcopyFeature>) {
    let options = report_figure_options(cx.model);
    if !report_mutation_allowed(cx) || options.is_empty() {
        return;
    }
    let first = &options[0];
    let source = &cx.model.workspace.content.visualization_documents[first.document_index];
    let editor = &mut cx.state.report_authoring;
    editor.insert_result_document_index = 0;
    editor.insert_result_caption = first.label.clone();
    editor.insert_result_alternative_text = format!(
        "Result figure {} at revision {}.",
        first.label,
        source.revision().get()
    );
    editor.insert_result_sizing = 0;
    editor.insert_result_frozen = false;
    editor.insert_result_document_open = true;
    editor.transaction_error = None;
}

pub fn commit_insert_result_document(cx: &mut Cx<'_, HardcopyFeature>) {
    if !report_mutation_allowed(cx) {
        cx.state.report_authoring.transaction_error =
            Some(report_mutation_block_reason(cx).to_owned());
        return;
    }
    let option_index = cx.state.report_authoring.insert_result_document_index;
    let figure_options = report_figure_options(cx.model);
    let Some(option) = figure_options.get(option_index).cloned() else {
        cx.state.report_authoring.transaction_error =
            Some("The selected result figure no longer exists.".to_owned());
        return;
    };
    let Some(source) = cx
        .model
        .workspace
        .content
        .visualization_documents
        .get(option.document_index)
        .cloned()
    else {
        cx.state.report_authoring.transaction_error =
            Some("The selected result document no longer exists.".to_owned());
        return;
    };
    let caption = cx
        .state
        .report_authoring
        .insert_result_caption
        .trim()
        .to_owned();
    let alternative_text = cx
        .state
        .report_authoring
        .insert_result_alternative_text
        .trim()
        .to_owned();
    let sizing = match cx.state.report_authoring.insert_result_sizing {
        1 => FigureSizing::FitPage,
        2 => FigureSizing::Natural,
        _ => FigureSizing::FitWidth,
    };
    let source_digest = match source.content_digest() {
        Ok(digest) => digest,
        Err(error) => {
            cx.state.report_authoring.transaction_error =
                Some(format!("The result document is invalid: {error}"));
            return;
        }
    };
    let dataset_bindings = source
        .datasets()
        .iter()
        .map(|dataset| dataset.binding())
        .collect::<Vec<_>>();
    let snapshot = match ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: source.id(),
        },
        Some(source.revision()),
        source_digest,
        dataset_bindings,
    ) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            cx.state.report_authoring.transaction_error = Some(error.to_string());
            return;
        }
    };
    let reference = if cx.state.report_authoring.insert_result_frozen {
        let raster = match render_visualization_report_figure(
            &source,
            &snapshot,
            option.page_id,
            option.pane_id,
            &VisualizationRasterProfile::default(),
        ) {
            Ok(raster) => raster,
            Err(error) => {
                cx.state.report_authoring.transaction_error = Some(format!(
                    "The selected result figure cannot be frozen for publication: {error}"
                ));
                return;
            }
        };
        ReportReferenceMode::Frozen {
            snapshot,
            artifact: raster.artifact().clone(),
        }
    } else {
        ReportReferenceMode::Linked { snapshot }
    };
    let Some(page_id) = active_document(cx).and_then(|document| selected_page_id(cx, document))
    else {
        cx.state.report_authoring.transaction_error =
            Some("Select a report page before inserting a result document.".to_owned());
        return;
    };
    let result = active_document_mut(cx).and_then(|document| {
        let page = document
            .page(page_id)
            .ok_or_else(|| "The selected report page no longer exists.".to_owned())?;
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::AddBlockToPage {
                    page_id,
                    expected_page_revision: page.revision(),
                    kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                        caption,
                        alternative_text,
                        sizing,
                        source_locator: Some(ReportFigureSourceLocator {
                            page_id: option.page_id.get(),
                            pane_id: option.pane_id.get(),
                        }),
                        reference,
                    }),
                }],
                timestamp_unix_ms(),
                "rspice-local-session",
                "Insert result document into report page",
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(receipt) => {
            cx.state.report_authoring.selected_report_block =
                receipt.created.iter().find_map(|entity| match entity {
                    ReportEntityRef::Block(id) => Some(*id),
                    _ => None,
                });
            cx.state.report_authoring.insert_result_document_open = false;
            cx.state.report_authoring.transaction_error = None;
            cx.model.workspace.content.report_documents_dirty = true;
        }
        Err(error) => cx.state.report_authoring.transaction_error = Some(error),
    }
}

impl ReportFigureOption {
    pub(super) fn label(&self) -> &str {
        &self.label
    }
}

pub(super) fn select_result_document(
    cx: &mut Cx<'_, HardcopyFeature>,
    index: usize,
    figure: Option<&ReportFigureOption>,
) {
    cx.state.report_authoring.insert_result_document_index = index;
    if let Some(option) = figure {
        let revision = cx.model.workspace.content.visualization_documents[option.document_index]
            .revision()
            .get();
        cx.state.report_authoring.insert_result_caption = option.label.clone();
        cx.state.report_authoring.insert_result_alternative_text =
            format!("Result figure {} at revision {revision}.", option.label);
    }
}
