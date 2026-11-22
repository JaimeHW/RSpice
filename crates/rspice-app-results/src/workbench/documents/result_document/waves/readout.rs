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

/// The dock's Cursors page: the strip it follows on its toolbar band, then
/// what the cursors and markers read, or how to place one.
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
            let followed = dock::followed_strip(session, source);
            console::toolbar(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_min_height(Tokens::get(ui.ctx()).metrics.ctl_h);
                    if let Some(strip) = followed {
                        ui.label(
                            egui::RichText::new(strip)
                                .font(rspice_ui_kit::theme::mono(
                                    rspice_ui_kit::tokens::FS_1,
                                    rspice_ui_kit::theme::FontWeight::Regular,
                                ))
                                .color(Tokens::get(ui.ctx()).color.text_dim),
                        );
                    }
                });
            });
            dock::cursors_page(ui, session, source)
        },
    );
    match shown {
        Some(actions) => apply_marker_actions(state, actions),
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

#[cfg(test)]
pub(super) fn marker_section(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let mut actions = MarkerActions::default();
    with_readout(state, &Tokens::get(ui.ctx()), true, |session, source| {
        dock::marker_section(ui, session, source, &mut actions)
    });
    apply_marker_actions(state, actions);
}
