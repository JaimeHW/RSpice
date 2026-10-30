//! Source/cache selection and transactional actions for the waveform readout.
use super::*;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use rspice_results_ui::session::ResultViewerState;
use rspice_results_ui::waves::dock::{self, MarkerActions, ReadoutInput};
#[cfg(test)]
pub(super) use rspice_results_ui::waves::dock::{MARKER_ROW_H, READOUT_HEADER_H, READOUT_MAX_H};
#[cfg(test)]
pub(super) use rspice_results_ui::waves::readout::{
    MAX_READOUT_BRANCHES, READOUT_ABSENT, READOUT_ROW_H, ReadoutRow, measurement_values,
    readout_branch_note, readout_rows, trace_interval_statistics,
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
    let models = need_models.then(|| {
        cached_models(
            &state.model.simulation,
            state.results,
            presentation.complex_number_display(),
            tokens,
        )
    });
    let strips = on_screen_strips(state.results_reads());
    render(
        &mut state.results.session,
        ReadoutInput {
            models: models.as_deref().map_or(&[], Vec::as_slice),
            on_screen_strips: &strips,
            presentation: presentation.readout(),
            quantity_policy,
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

pub fn readout_strip_height(state: &mut ResultsParts<'_>) -> f32 {
    let session = &state.results.session;
    let need_models = !session.readout_collapsed
        && session.cursor_readout_active()
        && session.cursor_strip.is_some();
    with_readout(
        state,
        &Tokens::default(),
        need_models,
        dock::readout_strip_height,
    )
}

pub fn readout_strip(ui: &mut Ui, state: &mut ResultsParts<'_>, height: f32) {
    let session = &state.results.session;
    let need_models = (session.cursor_readout_active() && session.cursor_strip.is_some())
        || (!session.readout_collapsed
            && height > dock::READOUT_HEADER_H
            && on_screen_strips(state.results_reads())
                .into_iter()
                .any(|key| session.strip_markers(key).next().is_some()));
    let actions = with_readout(
        state,
        &Tokens::get(ui.ctx()),
        need_models,
        |session, source| dock::readout_strip(ui, session, source, height),
    );
    apply_marker_actions(state, actions);
}

pub(crate) fn inline_cursor_readout(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
) -> Option<String> {
    let session = &state.results.session;
    if !session.readout_collapsed || !session.cursor_readout_active() {
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
pub(super) fn marker_body_height(state: &mut ResultsTestHost) -> f32 {
    let session = &state.results.results.session;
    let need_models = session.cursor_readout_active() && session.cursor_strip.is_some();
    with_readout(
        &mut state.results_parts(),
        &Tokens::default(),
        need_models,
        dock::marker_body_height,
    )
}

#[cfg(test)]
pub(super) fn visible_markers(state: &ResultsTestHost) -> Vec<MarkerView<'_>> {
    on_screen_strips(state.results_reads())
        .into_iter()
        .flat_map(|analysis| state.results.results.session.strip_markers(analysis))
        .collect()
}

#[cfg(test)]
pub(super) fn marker_section(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let mut actions = MarkerActions::default();
    with_readout(state, &Tokens::get(ui.ctx()), true, |session, source| {
        dock::marker_section(ui, session, source, &mut actions)
    });
    apply_marker_actions(state, actions);
}
