//! Source/cache selection and transactional actions for the waveform readout.
use super::*;
use rspice_results_ui::session::ResultViewerState;
use rspice_results_ui::waves::dock::{self, MarkerActions, ReadoutInput};
#[cfg(test)]
pub(super) use rspice_results_ui::waves::dock::{
    MARKER_ROW_H, READOUT_BODY_MAX_H, READOUT_HEADER_H, READOUT_MAX_H,
};
pub(super) use rspice_results_ui::waves::dock::{marker_color, marker_label};
#[cfg(test)]
pub(super) use rspice_results_ui::waves::readout::{
    MAX_READOUT_BRANCHES, READOUT_ABSENT, READOUT_ROW_H, ReadoutRow, measurement_values,
    readout_branch_note, readout_rows, trace_interval_statistics,
};

pub(super) fn on_screen_strips(state: &AppState) -> Vec<AnalysisPresentationKey> {
    let Some(run) = state.simulation.active_run() else {
        return Vec::new();
    };
    let results = &state.ui.results;
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
    state: &mut AppState,
    tokens: &Tokens,
    need_models: bool,
    render: impl FnOnce(&mut ResultViewerState, ReadoutInput<'_>) -> R,
) -> R {
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = need_models.then(|| {
        cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            tokens,
        )
    });
    let strips = on_screen_strips(state);
    render(
        &mut state.ui.results.session,
        ReadoutInput {
            models: models.as_deref().map_or(&[], Vec::as_slice),
            on_screen_strips: &strips,
            presentation: presentation.readout(),
            quantity_policy,
        },
    )
}

fn apply_marker_actions(state: &mut AppState, actions: MarkerActions) {
    if let Some(selector) = actions.remove {
        super::super::remove_marker(state, selector);
    }
    if let Some(selector) = actions.edit {
        super::marker_dialog::open(state, selector);
    }
}

pub fn readout_strip_height(state: &mut AppState) -> f32 {
    let session = &state.ui.results.session;
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

pub fn readout_strip(ui: &mut Ui, state: &mut AppState, height: f32) {
    let session = &state.ui.results.session;
    let need_models = (session.cursor_readout_active() && session.cursor_strip.is_some())
        || (!session.readout_collapsed
            && height > dock::READOUT_HEADER_H
            && on_screen_strips(state)
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

pub(crate) fn inline_cursor_readout(state: &mut AppState, tokens: &Tokens) -> Option<String> {
    let session = &state.ui.results.session;
    if !session.readout_collapsed || !session.cursor_readout_active() {
        return None;
    }
    with_readout(state, tokens, true, dock::inline_cursor_readout)
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    with_readout(state, &Tokens::get(ui.ctx()), true, |session, source| {
        dock::right_panel(ui, session, source)
    });
}

#[cfg(test)]
pub(super) fn readout_row_count(state: &mut AppState) -> usize {
    let need_models = state.ui.results.session.cursor_strip.is_some();
    with_readout(
        state,
        &Tokens::default(),
        need_models,
        dock::readout_row_count,
    )
}

#[cfg(test)]
pub(super) fn marker_body_height(state: &mut AppState) -> f32 {
    let session = &state.ui.results.session;
    let need_models = session.cursor_readout_active() && session.cursor_strip.is_some();
    with_readout(
        state,
        &Tokens::default(),
        need_models,
        dock::marker_body_height,
    )
}

#[cfg(test)]
pub(super) fn visible_markers(state: &AppState) -> Vec<MarkerView<'_>> {
    on_screen_strips(state)
        .into_iter()
        .flat_map(|analysis| state.ui.results.session.strip_markers(analysis))
        .collect()
}

#[cfg(test)]
pub(super) fn marker_section(ui: &mut Ui, state: &mut AppState) {
    let mut actions = MarkerActions::default();
    with_readout(state, &Tokens::get(ui.ctx()), true, |session, source| {
        dock::marker_section(ui, session, source, &mut actions)
    });
    apply_marker_actions(state, actions);
}
