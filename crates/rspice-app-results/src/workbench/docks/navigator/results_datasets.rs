//! The data browser's Datasets tab: the saved result documents, then every
//! retained run, with the analyses of the one shown under it. Like the
//! Signals tree, it draws only the rows in view, so a long run history
//! costs a frame what one screen of it does.

use egui::{ScrollArea, Ui};

use super::super::super::commands::result_navigation::open_run_log;
use super::super::super::design_system::WorkbenchIcon;
use super::results::{ResultRun, locate_results, result_dataset_row, run_mark, run_row_id};
use super::results_actions::select_result_analysis_by_key;
use super::results_documents::{
    document_row, document_row_actions, documents_head_actions, group_expanded,
};
use super::results_rows::{
    RESULT_BROWSER_RENDER_WINDOW_ROWS, documents_head, result_browser_manifest_row,
    result_browser_row_height,
};
use crate::product::ResultDocumentId;
use crate::ui::tokens::Tokens;
use crate::workbench::chrome::document_bar::results::{ListedDocument, show_run};
use crate::workbench::docks::RunDetailsDialogState;
use crate::workbench::documents::result_document::ResultsParts;
use rspice_app_workbench::workbench::docks::navigator::muted;

/// Rows drawn past each edge of the view, so a row the keyboard steps to
/// is already there to take the focus.
const OVERSCAN_ROWS: usize = 2;

/// One row of the Datasets tab, by its place in the frame's lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatasetRow {
    Documents { expanded: bool },
    Document(usize),
    Run(usize),
    Analysis { run: usize, analysis: usize },
}

/// The tab's rows: the documents group while there are documents, then each
/// run, with its analyses under it while it is the run shown, or under
/// every run while a query filters them.
fn dataset_rows(
    ctx: &egui::Context,
    documents: usize,
    runs: &[ResultRun],
    shown_run: Option<usize>,
    filtering: bool,
) -> Vec<DatasetRow> {
    let mut rows = Vec::with_capacity(documents + runs.len() + 1);
    if documents > 0 {
        let expanded = filtering || group_expanded(ctx);
        rows.push(DatasetRow::Documents { expanded });
        if expanded {
            rows.extend((0..documents).map(DatasetRow::Document));
        }
    }
    for (at, run) in runs.iter().enumerate() {
        rows.push(DatasetRow::Run(at));
        if filtering || shown_run == Some(run.run_index) {
            rows.extend(
                (0..run.analyses.len()).map(|analysis| DatasetRow::Analysis { run: at, analysis }),
            );
        }
    }
    rows
}

/// The rows from `start` to `end` that a view from `top` to `bottom`
/// shows, given each row's top and, last, the list's height.
fn rows_in_view(tops: &[f32], top: f32, bottom: f32) -> (usize, usize) {
    let rows = tops.len().saturating_sub(1);
    let first = tops.partition_point(|row| *row <= top).saturating_sub(1);
    let end = tops.partition_point(|row| *row < bottom).min(rows);
    (
        first.saturating_sub(OVERSCAN_ROWS).min(end),
        end.saturating_add(OVERSCAN_ROWS).min(rows),
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "the tab reads the frame's documents, runs and the one shown"
)]
pub(super) fn show_datasets(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_details: &mut Option<Box<RunDetailsDialogState>>,
    documents: &[ListedDocument],
    shown_document: Option<ResultDocumentId>,
    runs: &[ResultRun],
    active_run: Option<usize>,
    active_analysis: Option<usize>,
    filtering: bool,
) {
    if runs.is_empty() {
        muted(
            ui,
            if state.model.simulation.retained.runs.is_empty() {
                "No results yet \u{2014} run a simulation"
            } else {
                "No dataset or analysis matches this filter."
            },
        );
    }
    // One row reads as what Results shows: a saved document's while one is
    // shown, though its pane reads a run, and else the run's.
    let shown_run = active_run.filter(|_| shown_document.is_none());
    let rows = dataset_rows(ui.ctx(), documents.len(), runs, shown_run, filtering);
    let t = Tokens::get(ui.ctx());
    let (row_height, run_height) = (result_browser_row_height(&t), t.metrics.row_h);
    let height = |row: &DatasetRow| match row {
        DatasetRow::Run(_) => run_height,
        _ => row_height,
    };
    let tops = std::iter::once(0.0)
        .chain(rows.iter().scan(0.0, |top, row| {
            *top += height(row);
            Some(*top)
        }))
        .collect::<Vec<f32>>();
    // The list's height is the dock's, and a window of rows where the dock
    // sets no bound, as the Signals tree's is.
    let list_height = ui
        .available_height()
        .max(row_height)
        .min(row_height * RESULT_BROWSER_RENDER_WINDOW_ROWS as f32);
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .auto_shrink([false, true])
        .max_height(list_height)
        .show_viewport(ui, |ui, viewport| {
            // Rows abut, as the tree's do: each row's height is its stride.
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_height(tops.last().copied().unwrap_or_default());
            let (start, end) = rows_in_view(&tops, viewport.min.y, viewport.max.y);
            let top = ui.max_rect().top();
            let drawn = egui::Rect::from_x_y_ranges(
                ui.max_rect().x_range(),
                top + tops[start]..=top + tops[end],
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(drawn), |ui| {
                ui.skip_ahead_auto_ids(start);
                for row in &rows[start..end] {
                    match *row {
                        DatasetRow::Documents { expanded } => {
                            let head = documents_head(ui, documents.len(), expanded);
                            documents_head_actions(&head, expanded, filtering);
                        }
                        DatasetRow::Document(document) => {
                            let document = &documents[document];
                            let shown = shown_document == Some(document.id);
                            let response = document_row(ui, document, shown);
                            document_row_actions(state, document, shown, &response);
                        }
                        DatasetRow::Run(at) => {
                            run_row(ui, state, run_details, runs, at, shown_run);
                        }
                        DatasetRow::Analysis { run, analysis } => {
                            let run = &runs[run];
                            let analysis = &run.analyses[analysis];
                            let active = shown_run == Some(run.run_index)
                                && active_analysis == Some(analysis.analysis_index);
                            if result_browser_manifest_row(
                                ui,
                                analysis.short_label,
                                &analysis.label,
                                &analysis.domain,
                                active,
                                analysis.success,
                            )
                            .clicked()
                            {
                                select_result_analysis_by_key(state, analysis.presentation_key);
                            }
                        }
                    }
                }
            });
        });
}

/// The row of the run at `run_at` in `runs`: a click or Up and Down show a
/// run, its overlay toggle lays it over the sheet, and its menu holds what
/// else a run offers.
fn run_row(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    run_details: &mut Option<Box<RunDetailsDialogState>>,
    runs: &[ResultRun],
    run_at: usize,
    shown_run: Option<usize>,
) {
    let run = &runs[run_at];
    let run_active = shown_run == Some(run.run_index);
    let mark = run_mark(state, run);
    let retained = state.model.simulation.retained.runs.get(run.run_index);
    // A run whose results store no folder holds, or holds damaged, says so
    // before anything about its data.
    let unread = retained.and_then(crate::state::SimulationRun::unread_results);
    let checking = retained.and_then(crate::state::SimulationRun::checking_results);
    let pinned = retained.is_some_and(|retained| retained.retention().is_pinned());
    let run_id = retained.map(|retained| retained.id);
    let checking_meta;
    let run_meta = if let Some((done, total)) = checking {
        checking_meta = match done.saturating_mul(100).checked_div(total) {
            Some(percent) => format!("checking {percent}%"),
            None => "checking".to_owned(),
        };
        checking_meta.as_str()
    } else if unread.is_some() {
        "results not found"
    } else {
        ""
    };
    let overlaid = state.model.simulation.is_dataset_overlaid(run.dataset_id);
    let pinned_meta;
    let run_meta = match (pinned, run_meta.is_empty()) {
        (false, _) => run_meta,
        (true, true) => "pinned",
        (true, false) => {
            pinned_meta = format!("pinned \u{b7} {run_meta}");
            pinned_meta.as_str()
        }
    };
    let responses = result_dataset_row(
        ui,
        run_row_id(run.dataset_id),
        if run.success {
            WorkbenchIcon::Success
        } else {
            WorkbenchIcon::Warning
        },
        &run.label,
        run_active,
        run_meta,
        overlaid,
        mark.as_ref().map(|mark| mark.width(ui)),
    );
    if let (Some(mark), Some(slot)) = (&mark, responses.mark) {
        let asked = mark.show_in(ui, slot, run.dataset_id);
        mark.apply(state, &asked);
    }
    if responses.selection.gained_focus() {
        responses.selection.scroll_to_me(None);
    }
    // Up and Down on a run's row step to the run above or below it and show
    // that one, as a click on it would.
    let stepped = responses
        .selection
        .has_focus()
        .then(|| {
            ui.input_mut(|input| {
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                    run_at.checked_sub(1)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                    Some(run_at + 1)
                } else {
                    None
                }
            })
        })
        .flatten()
        .and_then(|at| runs.get(at));
    let picked = if let Some(next) = stepped {
        ui.memory_mut(|memory| memory.request_focus(run_row_id(next.dataset_id)));
        Some((next.dataset_id, &next.label))
    } else if responses.selection.clicked() {
        responses.selection.request_focus();
        Some((run.dataset_id, &run.label))
    } else {
        None
    };
    if let Some((dataset, label)) = picked {
        // A plan's run is shown through the plan's view, which holds on it
        // unless it is the plan's newest.
        if let Err(error) = show_run(state, dataset) {
            state
                .workbench
                .prompt(format!("Could not show {label}: {error}"));
        }
        state.results.session.selected_trace = None;
        state.results.session.selected_result_artifact = None;
    }
    if let Some(reason) = &unread {
        responses.selection.clone().on_hover_text(format!(
            "The results of {} cannot be read: {reason}",
            run.label
        ));
    }
    let mut details = false;
    let mut log = false;
    let mut locate = false;
    responses.selection.context_menu(|ui| {
        super::results_runs::run_menu(ui, state, run.dataset_id);
        ui.separator();
        if unread.is_some() && ui.button("Locate results\u{2026}").clicked() {
            locate = true;
            ui.close();
        }
        if ui.button("Run details\u{2026}").clicked() {
            details = true;
            ui.close();
        }
        if ui.button("Open run log").clicked() {
            log = true;
            ui.close();
        }
    });
    if locate {
        locate_results(state);
    }
    if let Some(run_id) = run_id {
        if details {
            super::super::inspector::run_details::open(
                run_details,
                &state.model.simulation,
                run_id,
            );
        }
        if log {
            open_run_log(state.model, state.workbench, run_id);
        }
    }
    if responses.overlay.is_some_and(|response| response.clicked()) {
        let enabled = state
            .model
            .simulation
            .toggle_dataset_overlay(run.dataset_id);
        state.workbench.prompt(if enabled {
            format!("Overlaying {} on the active result sheet.", run.label)
        } else {
            format!("Removed {} from the active result sheet.", run.label)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Twenty rows of 22 px and a 100 px view from 220 px: rows 10 to 14 are
    /// in view, and two more each side are drawn.
    #[test]
    fn a_view_draws_its_rows_and_two_past_each_edge() {
        let tops = (0..=20).map(|row| row as f32 * 22.0).collect::<Vec<_>>();
        assert_eq!(rows_in_view(&tops, 220.0, 320.0), (8, 17));
        assert_eq!(rows_in_view(&tops, 0.0, 100.0), (0, 7));
        assert_eq!(rows_in_view(&tops, 400.0, 500.0), (16, 20));
        assert_eq!(rows_in_view(&[0.0], 0.0, 100.0), (0, 0));
    }
}
