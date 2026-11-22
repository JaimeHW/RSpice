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

pub(super) fn with_readout<R>(
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
    if let Some((spot, from)) = actions.offset {
        add_offset_marker(state, spot, from);
    }
}

/// Add a marker 3 below `from`'s level on its trace, where `from` stands,
/// and open it for its amount to be typed.
pub(super) fn add_offset_marker(
    state: &mut ResultsParts<'_>,
    spot: rspice_results_ui::waves::cursor_table::MarkerSpot,
    from: super::super::MarkerSelector,
) -> Option<super::super::MarkerSelector> {
    let placement = super::super::MarkerPlacement {
        analysis: spot.analysis,
        anchor: spot.anchor,
        trace_name: spot.trace_name,
        x: spot.x,
        samples: &spot.samples,
    };
    let selector = super::super::place_marker(state, placement)?;
    let kind =
        super::super::MarkerKind::Offset(rspice_results::result_presentation::MarkerOffset {
            from: from.marker_id.get(),
            by: rspice_results::result_presentation::CrossingLevel(-3.0),
        });
    if let Err(error) = super::super::commit_marker_edit(state, selector, "", kind) {
        state.prompt(format!("Could not make the offset marker: {error}"));
        return None;
    }
    super::marker_dialog::open(state, selector);
    Some(selector)
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
    use rspice_results_ui::waves::cursor_table::CursorTable;
    let shown = with_readout(state, &Tokens::get(ui.ctx()), true, |session, source| {
        // With no strip to read the page is one line, with no band over it.
        if source.models.is_empty() {
            return None;
        }
        let tables = if dock::page_has_readings(session, source) {
            dock::cursor_tables(session, source, &Tokens::get(ui.ctx()))
        } else {
            Vec::new()
        };
        let outcome = console::toolbar(ui, |ui| cursors_toolbar(ui, session, &tables));
        if outcome.copied {
            ui.ctx().copy_text(
                tables
                    .iter()
                    .map(CursorTable::tsv)
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            );
        }
        let placed = outcome
            .add
            .and_then(|choice| add_reading(session, source, choice));
        if outcome.add.is_some() {
            ui.ctx().request_repaint();
        }
        let actions = if tables.is_empty() {
            None
        } else {
            dock::cursors_page(ui, session, source, &tables)
        };
        Some((actions, outcome.copied, placed))
    });
    let Some((actions, copied, placed)) = shown else {
        empty_cursors_page(ui);
        return;
    };
    if copied {
        state.prompt("Copied the cursor table");
    }
    if let Some((spot, vertical)) = placed {
        add_marker(state, spot, vertical);
    }
    match actions {
        Some(actions) => apply_marker_actions(state, actions),
        None => empty_cursors_page(ui),
    }
}

/// Add what the page's Add list names at `choice`: a cursor pair or a
/// horizontal cursor at once, or where a marker goes, and whether it is a
/// vertical one, for the host to place.
fn add_reading(
    session: &mut ResultViewerState,
    source: ReadoutInput<'_>,
    choice: usize,
) -> Option<(rspice_results_ui::waves::cursor_table::MarkerSpot, bool)> {
    use rspice_results_ui::waves::cursor_table;
    match cursor_table::ADD_CHOICES.get(choice).copied()? {
        "Delta pair" => {
            cursor_table::add_delta_pair(session, source);
            None
        }
        "Horizontal cursor" => {
            cursor_table::add_horizontal(session, source);
            None
        }
        choice => cursor_table::marker_spot(session, source)
            .map(|spot| (spot, choice == "Vertical marker")),
    }
}

/// Add from the Cursors page, as picking `choice` in its Add list does.
#[cfg(test)]
pub(super) fn add_from_cursors_page(state: &mut ResultsParts<'_>, choice: &str) {
    let index = rspice_results_ui::waves::cursor_table::ADD_CHOICES
        .iter()
        .position(|name| *name == choice)
        .expect("an Add choice");
    if let Some((spot, vertical)) =
        with_readout(state, &Tokens::default(), true, |session, source| {
            add_reading(session, source, index)
        })
    {
        add_marker(state, spot, vertical);
    }
}

/// The Cursors page's tables, as it would draw them now.
#[cfg(test)]
pub(super) fn cursor_tables(
    state: &mut ResultsParts<'_>,
) -> Vec<rspice_results_ui::waves::cursor_table::CursorTable> {
    with_readout(state, &Tokens::default(), true, |session, source| {
        dock::cursor_tables(session, source, &Tokens::default())
    })
}

/// The page with nothing to read: one line saying how to place a cursor.
fn empty_cursors_page(ui: &mut Ui) {
    ui.add_space(console::CONSOLE_BODY_PADDING_TOP);
    if console::prose_fits(ui) {
        console::padded(ui, |ui| {
            console::muted(
                ui,
                "No cursors or markers. Add one, or switch on A|B in the sheet's bar and click a plot to place A, then B.",
            );
        });
    }
}

/// Place the marker Add asked for: a vertical marker as it is, a point
/// marker with its dialog open, as a click on the plot places one.
fn add_marker(
    state: &mut ResultsParts<'_>,
    spot: rspice_results_ui::waves::cursor_table::MarkerSpot,
    vertical: bool,
) {
    let placement = super::super::MarkerPlacement {
        analysis: spot.analysis,
        anchor: spot.anchor,
        trace_name: spot.trace_name,
        x: spot.x,
        samples: &spot.samples,
    };
    let Some(selector) = super::super::place_marker(state, placement) else {
        return;
    };
    if vertical {
        if let Err(error) = super::super::commit_marker_edit(
            state,
            selector,
            "",
            super::super::MarkerKind::Vertical,
        ) {
            state.prompt(format!("Could not make the marker vertical: {error}"));
        }
    } else {
        super::marker_dialog::open(state, selector);
    }
}

/// What the reader asked of the Cursors page's toolbar this frame.
#[derive(Default)]
struct ToolbarOutcome {
    copied: bool,
    /// The Add choice picked, by its place in the list.
    add: Option<usize>,
}

/// The Cursors page's controls: Add, the base every difference is read
/// against and Snap, then Copy at the right.
fn cursors_toolbar(
    ui: &mut Ui,
    session: &mut ResultViewerState,
    tables: &[rspice_results_ui::waves::cursor_table::CursorTable],
) -> ToolbarOutcome {
    use rspice_ui_kit::widgets::{
        chip_width, compact_chip, compact_select_mono_width, compact_select_mono_with_response,
    };
    let mut outcome = ToolbarOutcome::default();
    ui.horizontal(|ui| {
        ui.set_min_height(Tokens::get(ui.ctx()).metrics.ctl_h);
        ui.spacing_mut().item_spacing.x = 6.0;
        // Add reads as its own name, and a pick places what it names.
        let choices: Vec<String> = rspice_results_ui::waves::cursor_table::ADD_CHOICES
            .iter()
            .map(|choice| (*choice).to_owned())
            .collect();
        let width = compact_select_mono_width(ui, &choices);
        outcome.add = compact_select_mono_with_response(
            ui,
            "results.cursors.add",
            "Add a cursor or marker",
            "Add",
            &choices,
            width,
        )
        .picked;
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
        if compact_chip(ui, "Snap", session.snap_to_samples, chip_width(ui, "Snap"))
            .on_hover_text(
                "Stand the cursors and the markers placed from here on retained samples \
                 rather than between them",
            )
            .clicked()
        {
            session.snap_to_samples = !session.snap_to_samples;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !tables.is_empty()
                && compact_chip(ui, "Copy", false, chip_width(ui, "Copy"))
                    .on_hover_text(
                        "Copy the table as tab-separated text, as a spreadsheet pastes it",
                    )
                    .clicked()
            {
                outcome.copied = true;
            }
        });
    });
    outcome
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
