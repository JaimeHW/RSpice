//! Source/cache selection and transactional actions for the waveform readout.
use super::*;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::state::Workspace;
use rspice_app_workbench::workbench::docks::console;
use rspice_app_workbench::workbench::state::workbench_session::{ConsolePage, WorkbenchSession};
use rspice_results_ui::session::ResultViewerState;
use rspice_results_ui::waves::dock::{self, MarkerActions, ReadoutInput};
#[cfg(test)]
pub(super) use rspice_results_ui::waves::readout::{
    MAX_READOUT_BRANCHES, READOUT_ABSENT, ReadoutRow, measurement_values, readout_branch_note,
    readout_rows, trace_interval_statistics,
};

pub(super) fn on_screen_strips(state: ResultsReads<'_>) -> Vec<AnalysisPresentationKey> {
    let Some(run) = state.model.simulation.active_run() else {
        return Vec::new();
    };
    let results = &state.results;
    let present = |key: AnalysisPresentationKey| key.resolve(run).is_some();
    match results.session.maximized_strip {
        Some(max_key) if present(max_key) => vec![max_key],
        _ => run
            .analyses
            .iter()
            .map(|analysis| AnalysisPresentationKey::new(run.dataset_id, analysis))
            .filter(|key| !results.session.hidden_strips.contains(key))
            .collect(),
    }
}

fn with_readout<R>(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
    need_models: bool,
    render: impl FnOnce(&mut ResultViewerState, ReadoutInput<'_>) -> R,
) -> R {
    let presentation = state.results_reads().presentation_policy();
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let models = if need_models {
        cached_models(
            &state.model.simulation,
            state.results,
            presentation.complex_number_display(),
            tokens,
        )
    } else {
        std::sync::Arc::default()
    };
    let models_generation = state.results.models.generation();
    let strips = on_screen_strips(state.results_reads());
    // Design notation changes labels only; the rows' keys stay the result's.
    // It is read the first time a row is named, so a frame that names none
    // does not walk the drawings.
    let notations = std::cell::OnceCell::new();
    let (workspace, schematic) = (&state.model.workspace, &state.model.schematic);
    render(
        &mut state.results.session,
        ReadoutInput {
            models: &models,
            models_generation,
            on_screen_strips: &strips,
            presentation: presentation.readout(),
            quantity_policy,
            display_name: &|name| {
                notations
                    .get_or_init(|| bus_notations(workspace, schematic))
                    .display(name)
            },
        },
    )
}

fn apply_marker_actions(state: &mut ResultsParts<'_>, actions: MarkerActions) {
    if let Some(moved) = actions.moved {
        super::super::move_marker(state, moved);
    }
    if let Some(selector) = actions.remove {
        super::super::remove_marker(state, selector);
    }
    if let Some(selector) = actions.edit {
        super::marker_dialog::open(state, selector);
    }
}

/// Whether the dock offers its Cursors page: while Results shows a sheet
/// that reads cursors and markers.
pub fn offers_cursors(state: ResultsReads<'_>) -> bool {
    state.workbench.workspace == Workspace::Results
        && super::super::sheet_has_cursors(state.results.session.viewer)
}

/// Whether the dock shows its Cursors page now.
fn cursors_page_shown(workbench: &WorkbenchSession) -> bool {
    (workbench.console_visible || workbench.console_maximized)
        && !workbench.focus_mode
        && workbench.console_page == ConsolePage::Cursors
}

/// The dock's Cursors page: its controls on the toolbar band, then what
/// the cursors and markers read, or how to place one.
pub fn cursors_page(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let session = &state.results.session;
    let need_models = (session.cursor_readout_active() && session.cursor_strip.is_some())
        || on_screen_strips(state.results_reads())
            .into_iter()
            .any(|key| session.strip_markers(key).next().is_some());
    let shown = with_readout(
        state,
        &Tokens::get(ui.ctx()),
        need_models,
        |session, source| {
            // With nothing to read the page is one line, with no band over it.
            if !dock::page_has_readings(session, source) {
                return None;
            }
            let tables = dock::cursor_tables(session, source, &Tokens::get(ui.ctx()));
            let copied = console::toolbar(ui, |ui| cursors_toolbar(ui, session, &tables));
            if copied {
                ui.ctx().copy_text(
                    tables
                        .iter()
                        .map(rspice_results_ui::waves::cursor_table::CursorTable::tsv)
                        .collect::<Vec<_>>()
                        .join("\n\n"),
                );
            }
            dock::cursors_page(ui, session, source, &tables).map(|actions| (actions, copied))
        },
    );
    match shown {
        Some((actions, copied)) => {
            if copied {
                state.prompt("Copied the cursor table");
            }
            apply_marker_actions(state, actions);
        }
        None => {
            ui.add_space(console::CONSOLE_BODY_PADDING_TOP);
            if console::prose_fits(ui) {
                console::padded(ui, |ui| {
                    console::muted(
                        ui,
                        "No cursors or markers. Switch on A|B in the sheet's bar and click a plot to place A, then B.",
                    );
                });
            }
        }
    }
}

/// The Cursors page's controls: the base every difference is read
/// against, and Copy at the right. Says whether Copy was pressed.
fn cursors_toolbar(
    ui: &mut Ui,
    session: &mut ResultViewerState,
    tables: &[rspice_results_ui::waves::cursor_table::CursorTable],
) -> bool {
    use rspice_ui_kit::widgets::{
        chip_width, compact_chip, compact_select_mono_width, compact_select_mono_with_response,
    };
    let mut copied = false;
    ui.horizontal(|ui| {
        ui.set_min_height(Tokens::get(ui.ctx()).metrics.ctl_h);
        ui.spacing_mut().item_spacing.x = 6.0;
        // Every column of every table can be the base; a table without it
        // reads against its own first column.
        let columns: Vec<_> = tables
            .iter()
            .flat_map(|table| table.columns.iter().map(|column| (column.id, &column.name)))
            .collect();
        if columns.len() > 1 {
            let options: Vec<String> = columns
                .iter()
                .map(|(_, name)| format!("Base {name}"))
                .collect();
            let base = tables
                .first()
                .map(|table| table.columns[table.base].id)
                .and_then(|id| columns.iter().position(|(column, _)| *column == id))
                .unwrap_or(0);
            let width = compact_select_mono_width(ui, &options);
            let picked = compact_select_mono_with_response(
                ui,
                "results.cursors.base",
                "The column the differences are read against",
                &options[base],
                &options,
                width,
            )
            .picked;
            if let Some(picked) = picked {
                session.readout_base = Some(columns[picked].0);
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !tables.is_empty()
                && compact_chip(ui, "Copy", false, chip_width(ui, "Copy"))
                    .on_hover_text(
                        "Copy the table as tab-separated text, as a spreadsheet pastes it",
                    )
                    .clicked()
            {
                copied = true;
            }
        });
    });
    copied
}

pub(crate) fn inline_cursor_readout(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
) -> Option<String> {
    if cursors_page_shown(state.workbench) || !state.results.session.cursor_readout_active() {
        return None;
    }
    with_readout(state, tokens, true, dock::inline_cursor_readout)
}

pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    with_readout(state, &Tokens::get(ui.ctx()), true, |session, source| {
        dock::right_panel(ui, session, source)
    });
}

#[cfg(test)]
pub(super) fn cursors_page_reads(state: &mut ResultsTestHost) -> bool {
    with_readout(
        &mut state.results_parts(),
        &Tokens::default(),
        true,
        dock::page_has_readings,
    )
}

#[cfg(test)]
pub(super) fn readout_row_count(state: &mut ResultsTestHost) -> usize {
    let need_models = state.results.results.session.cursor_strip.is_some();
    with_readout(
        &mut state.results_parts(),
        &Tokens::default(),
        need_models,
        dock::readout_row_count,
    )
}
