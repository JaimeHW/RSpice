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

pub(super) fn report_figure_options(state: &AppState) -> Vec<ReportFigureOption> {
    let mut options = Vec::new();
    for (document_index, document) in state.workspace.visualization_documents.iter().enumerate() {
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

pub(super) fn open_insert_result_document(app: &mut RSpiceApp) {
    let options = report_figure_options(&app.state);
    if !report_mutation_allowed(&app.state) || options.is_empty() {
        return;
    }
    let first = &options[0];
    let source = &app.state.workspace.visualization_documents[first.document_index];
    let editor = &mut app.state.workbench.report_authoring;
    editor.insert_result_document_index = 0;
    editor.insert_result_caption = first.label.clone();
    editor.insert_result_alternative_text = format!(
        "Result figure {} at immutable visualization revision {}.",
        first.label,
        source.revision().get()
    );
    editor.insert_result_sizing = 0;
    editor.insert_result_frozen = false;
    editor.insert_result_document_open = true;
    editor.transaction_error = None;
}

pub(super) fn insert_result_document_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .report_authoring
        .insert_result_document_open
    {
        return;
    }
    const SIZING_LABELS: [&str; 3] = ["Fit width", "Fit page", "Natural size"];
    let figure_options = report_figure_options(&app.state);
    let source_options = figure_options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    let source_index = app
        .state
        .workbench
        .report_authoring
        .insert_result_document_index
        .min(source_options.len().saturating_sub(1));
    let caption = app
        .state
        .workbench
        .report_authoring
        .insert_result_caption
        .trim();
    let alternative_text = app
        .state
        .workbench
        .report_authoring
        .insert_result_alternative_text
        .trim();
    let valid = !source_options.is_empty()
        && !caption.is_empty()
        && caption.len() <= 2_048
        && !caption.chars().any(char::is_control)
        && !alternative_text.is_empty()
        && alternative_text.len() <= 8_192
        && !alternative_text
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t');
    let writable = report_mutation_allowed(&app.state);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · IMMUTABLE RESULT SOURCE",
        "Insert result document",
        "Insert result document",
    )
    .description(
        "Insert one exact visualization-document revision with all immutable dataset bindings retained for publication audit.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 132.0_f32.min(ui.available_width() * 0.34);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Result document"),
            );
            let current = source_options
                .get(source_index)
                .map_or("No retained result documents", String::as_str);
            if let Some(index) = select(
                ui,
                "report-insert-result-source",
                "Immutable result document",
                current,
                &source_options,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .insert_result_document_index = index;
                if let Some(option) = figure_options.get(index) {
                    let revision = app.state.workspace.visualization_documents
                        [option.document_index]
                        .revision()
                        .get();
                    app.state
                        .workbench
                        .report_authoring
                        .insert_result_caption = option.label.clone();
                    app.state
                        .workbench
                        .report_authoring
                        .insert_result_alternative_text = format!(
                        "Result figure {} at immutable visualization revision {revision}.",
                        option.label
                    );
                }
            }
        });
        ui.add_space(8.0);
        let focus = input_row(
            ui,
            "Caption",
            &mut app
                .state
                .workbench
                .report_authoring
                .insert_result_caption,
        );
        ui.add_space(8.0);
        input_row(
            ui,
            "Alternative text",
            &mut app
                .state
                .workbench
                .report_authoring
                .insert_result_alternative_text,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Sizing"),
            );
            let sizing_index = app
                .state
                .workbench
                .report_authoring
                .insert_result_sizing
                .min(SIZING_LABELS.len() - 1);
            let labels = SIZING_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-insert-result-sizing",
                "Result figure sizing",
                SIZING_LABELS[sizing_index],
                &labels,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .insert_result_sizing = index;
            }
        });
        ui.add_space(8.0);
        ui.checkbox(
            &mut app
                .state
                .workbench
                .report_authoring
                .insert_result_frozen,
            "Freeze self-contained source payload in this report revision",
        );
        ui.label(if app
            .state
            .workbench
            .report_authoring
            .insert_result_frozen
        {
            "The exact source snapshot and deterministic publication PNG are embedded and digest-authenticated."
        } else {
            "The block remains linked to one exact immutable visualization revision."
        });
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                report_mutation_block_reason(&app.state),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(focus.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => commit_insert_result_document(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state
                .workbench
                .report_authoring
                .insert_result_document_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

pub(super) fn commit_insert_result_document(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let option_index = app
        .state
        .workbench
        .report_authoring
        .insert_result_document_index;
    let figure_options = report_figure_options(&app.state);
    let Some(option) = figure_options.get(option_index).cloned() else {
        app.state.workbench.report_authoring.transaction_error =
            Some("The selected result figure no longer exists.".to_owned());
        return;
    };
    let Some(source) = app
        .state
        .workspace
        .visualization_documents
        .get(option.document_index)
        .cloned()
    else {
        app.state.workbench.report_authoring.transaction_error =
            Some("The selected result document no longer exists.".to_owned());
        return;
    };
    let caption = app
        .state
        .workbench
        .report_authoring
        .insert_result_caption
        .trim()
        .to_owned();
    let alternative_text = app
        .state
        .workbench
        .report_authoring
        .insert_result_alternative_text
        .trim()
        .to_owned();
    let sizing = match app.state.workbench.report_authoring.insert_result_sizing {
        1 => FigureSizing::FitPage,
        2 => FigureSizing::Natural,
        _ => FigureSizing::FitWidth,
    };
    let source_digest = match source.content_digest() {
        Ok(digest) => digest,
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error =
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
            app.state.workbench.report_authoring.transaction_error = Some(error.to_string());
            return;
        }
    };
    let reference = if app.state.workbench.report_authoring.insert_result_frozen {
        let raster = match render_visualization_report_figure(
            &source,
            &snapshot,
            option.page_id,
            option.pane_id,
            &VisualizationRasterProfile::default(),
        ) {
            Ok(raster) => raster,
            Err(error) => {
                app.state.workbench.report_authoring.transaction_error = Some(format!(
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
    let Some(page_id) =
        active_document(&app.state).and_then(|document| selected_page_id(&app.state, document))
    else {
        app.state.workbench.report_authoring.transaction_error =
            Some("Select a report page before inserting a result document.".to_owned());
        return;
    };
    let result = active_document_mut(&mut app.state).and_then(|document| {
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
                "Insert immutable result document into report page",
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(receipt) => {
            app.state.workbench.report_authoring.selected_report_block =
                receipt.created.iter().find_map(|entity| match entity {
                    ReportEntityRef::Block(id) => Some(*id),
                    _ => None,
                });
            app.state
                .workbench
                .report_authoring
                .insert_result_document_open = false;
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}
