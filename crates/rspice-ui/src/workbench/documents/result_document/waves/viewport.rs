//! The viewport a wave strip is being read through, and the gestures that
//! move it.
//!
//! Every pane of one analysis shares a single abscissa, so the X window is a
//! property of the strip rather than of any pane in it: a zoom taken from the
//! phase pane of a Bode pair has to move the magnitude pane with it, or the
//! two stop describing the same sweep. Y is the pane's own, because each pane
//! carries its own unit.
//!
//! The fraction arithmetic below is all done in the axis' own space —
//! logarithmic strips normalize through decades — so a window keeps its
//! geometric centre where a reader put it.

use super::*;

pub(super) fn scaled_range(
    range: (f64, f64),
    factor: f64,
    logarithmic: bool,
) -> Option<(f64, f64)> {
    if !factor.is_finite() || factor <= 0.0 || !range.0.is_finite() || !range.1.is_finite() {
        return None;
    }
    if logarithmic {
        if range.0 <= 0.0 || range.1 <= range.0 {
            return None;
        }
        let low = range.0.log10();
        let high = range.1.log10();
        let center = (low + high) * 0.5;
        let half_span = (high - low) * 0.5 * factor;
        return Some((
            10.0_f64.powf(center - half_span),
            10.0_f64.powf(center + half_span),
        ));
    }
    if range.1 <= range.0 {
        return None;
    }
    let center = (range.0 + range.1) * 0.5;
    let half_span = (range.1 - range.0) * 0.5 * factor;
    Some((center - half_span, center + half_span))
}

/// Magnify the active pane about the centre of what it is showing.
///
/// X is written through [`set_shared_x_view`], the same way the axis editor
/// and the shared strip's own gestures write it: the panes of one analysis
/// draw a single sweep, so a zoom taken from the phase pane of a Bode pair
/// that moved only that pane's ordinal left the magnitude pane describing a
/// different window — and the pinned ordinal it left behind kept reporting
/// the strip as zoomed after a fit.
///
/// Y is the pane's own, and a pane drawing decades has to be scaled in
/// decades: scaling a logarithmic pane's endpoints linearly moves its
/// geometric centre, so a zoom in and a zoom back out did not return.
pub(crate) fn zoom_active_pane(state: &mut AppState, t: &Tokens, factor: f64) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((model, ordinal, pane)) = active_pane(&models, &state.ui.results) else {
        return;
    };
    let pane_count = model.unit_panes().len();
    let logarithmic_y = pane_log_y(&state.ui.results, model, &pane);
    let current = state.ui.results.analysis_plot_view_pane(
        super::super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    let x = shared_x_view(&state.ui.results, model.analysis_key, pane_count)
        .or(model.x_range)
        .and_then(|range| scaled_range(range, factor, model.x_scale == XScale::Log10));
    let y = match current.y {
        Some(range) => Some(range),
        None => displayed_pane_auto_y(state, model, &pane, ordinal, t),
    }
    .and_then(|range| scaled_range(range, factor, logarithmic_y));
    if let Some(x) = x {
        set_shared_x_view(
            &mut state.ui.results,
            model.analysis_key,
            pane_count,
            Some(x),
        );
    }
    if let Some(y) = y {
        state
            .ui
            .results
            .analysis_plot_view_pane_mut(
                super::super::ResultViewer::Waves,
                model.analysis_key,
                ordinal,
            )
            .y = Some(y);
    }
}

/// Fit the active pane: its own Y returns to the data, and the strip's shared
/// abscissa returns with it.
///
/// The X window belongs to the strip rather than to this pane, so dropping it
/// on one ordinal alone left every sibling pane pinned to a window the reader
/// had just asked to leave — and any ordinal the strip no longer draws kept
/// the whole strip reading as zoomed, with no pane left to fit it from.
pub(crate) fn fit_active_pane(state: &mut AppState, t: &Tokens) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((model, ordinal, _)) = active_pane(&models, &state.ui.results) else {
        return;
    };
    state.ui.results.clear_analysis_plot_view_axis(
        super::super::ResultViewer::Waves,
        model.analysis_key,
        super::super::PaneAxis::X,
    );
    state
        .ui
        .results
        .analysis_plot_view_pane_mut(
            super::super::ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        )
        .y = None;
}

/// Drop every pinned viewport on the strip the active pane belongs to.
///
/// The instrument bar's fit button fits one pane; the workspace-level Fit
/// gesture fits the whole strip, because the panes of a strip share one X
/// domain and leaving the others pinned would make them disagree about the
/// window they show. With no active pane — nothing has been clicked yet —
/// every strip of the sheet fits, which is what "fit" means before the user
/// has singled one out.
pub(crate) fn fit_active_strip(state: &mut AppState) {
    let viewer = super::super::ResultViewer::Waves;
    match state.ui.results.active_wave_pane.as_ref() {
        Some(key) => {
            let analysis = key.analysis;
            state.ui.results.reset_analysis_plot_view(viewer, analysis);
        }
        None => state.ui.results.reset_all_analysis_plot_views(viewer),
    }
}

/// Whether the active pane is showing a viewport the user pinned rather than
/// the retained data's own extent.
///
/// The waveform stack keys its viewports by analysis, not by the legacy
/// `Global` plot index, so this is the only honest reading for these sheets.
pub(super) fn active_pane_is_pinned(tokens: &Tokens, state: &mut AppState) -> bool {
    let Some(key) = state.ui.results.active_wave_pane.clone() else {
        return false;
    };
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_key == key.analysis)
    else {
        return false;
    };
    let panes = model.unit_panes();
    (0..panes.len()).any(|ordinal| {
        state
            .ui
            .results
            .analysis_plot_view_pane(super::super::ResultViewer::Waves, key.analysis, ordinal)
            .is_zoomed()
    })
}

/// Move a placed cursor by a fraction of the visible X interval.
///
/// The nudge is expressed in the window the reader is looking at, not the
/// full retained sweep, so one keypress moves the same visible distance
/// however far the sheet is zoomed in.
pub(crate) fn nudge_cursor(state: &mut AppState, tokens: &Tokens, cursor_b: bool, steps: f64) {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        tokens,
    );
    let Some(strip) = state.ui.results.cursor_strip else {
        return;
    };
    let Some(model) = models.iter().find(|model| model.analysis_index == strip) else {
        return;
    };
    let Some(full) = model.x_range else {
        return;
    };
    let panes = model.unit_panes().len();
    let (x0, x1) = shared_x_view(&state.ui.results, model.analysis_key, panes).unwrap_or(full);
    let cursor = if cursor_b {
        &mut state.ui.results.cursors.b
    } else {
        &mut state.ui.results.cursors.a
    };
    let Some(position) = cursor else {
        return;
    };
    let moved = if model.x_scale == XScale::Log10 && *position > 0.0 && x0 > 0.0 && x1 > x0 {
        let decades = (x1.log10() - x0.log10()) * 0.01 * steps;
        10.0_f64.powf(position.log10() + decades)
    } else {
        *position + (x1 - x0) * 0.01 * steps
    };
    if moved.is_finite() {
        *position = moved.clamp(full.0.min(full.1), full.0.max(full.1));
    }
}

pub(super) fn shared_x_view(
    results: &ResultsState,
    analysis: AnalysisPresentationKey,
    pane_count: usize,
) -> Option<(f64, f64)> {
    (0..pane_count).find_map(|ordinal| {
        results
            .analysis_plot_view_pane(super::super::ResultViewer::Waves, analysis, ordinal)
            .x
    })
}

pub(super) fn set_shared_x_view(
    results: &mut ResultsState,
    analysis: AnalysisPresentationKey,
    pane_count: usize,
    range: Option<(f64, f64)>,
) {
    for ordinal in 0..pane_count {
        results
            .analysis_plot_view_pane_mut(super::super::ResultViewer::Waves, analysis, ordinal)
            .x = range;
    }
}

pub(super) fn shared_axis_viewport_fraction(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
) -> (f64, f64) {
    let start = scale.normalize(view.0, full.0, full.1).clamp(0.0, 1.0);
    let end = scale.normalize(view.1, full.0, full.1).clamp(0.0, 1.0);
    (start.min(end), start.max(end))
}

pub(super) fn panned_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    fraction_delta: f64,
) -> Option<(f64, f64)> {
    if !fraction_delta.is_finite() {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if !(width > 0.0 && width < 1.0) {
        return None;
    }
    let next_start = (start + fraction_delta).clamp(0.0, 1.0 - width);
    let next_end = next_start + width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

pub(super) fn zoomed_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    anchor_fraction: f64,
    factor: f64,
) -> Option<(f64, f64)> {
    if !anchor_fraction.is_finite() || !factor.is_finite() || factor <= 0.0 {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if width <= 0.0 {
        return None;
    }
    let anchor = anchor_fraction.clamp(0.0, 1.0);
    let relative = ((anchor - start) / width).clamp(0.0, 1.0);
    let next_width = (width * factor).clamp(1.0e-6, 1.0);
    let next_start = (anchor - relative * next_width).clamp(0.0, 1.0 - next_width);
    let next_end = next_start + next_width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// Resize the shared viewport by dragging one edge of the overview window.
///
/// The dragged edge follows the pointer and the opposite edge stays fixed,
/// so the gesture zooms and pans in one motion the way pulling a scrollbar
/// handle does.
pub(super) fn resized_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    move_start: bool,
    edge_fraction: f64,
) -> Option<(f64, f64)> {
    if !edge_fraction.is_finite() {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let edge = edge_fraction.clamp(0.0, 1.0);
    let (next_start, next_end) = if move_start {
        (edge.min(end - SHARED_X_MIN_WINDOW), end)
    } else {
        (start, edge.max(start + SHARED_X_MIN_WINDOW))
    };
    let next_start = next_start.clamp(0.0, 1.0 - SHARED_X_MIN_WINDOW);
    let next_end = next_end.clamp(next_start + SHARED_X_MIN_WINDOW, 1.0);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// The name the strip prints into the panes' left gutter beside the tick
/// values, following the mockup's axis vocabulary.
///
/// The tick values themselves are SI-prefixed bare numbers, so the gutter is
/// where the axis states its unit — once, for the whole row.
pub(super) fn shared_x_gutter_label(unit: &str) -> String {
    let name = match unit {
        "s" => "TIME",
        "Hz" => "FREQ",
        _ => "X",
    };
    if unit.is_empty() {
        name.to_owned()
    } else {
        format!("{name} · {unit}")
    }
}

pub(super) fn recentered_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    centre_fraction: f64,
) -> Option<(f64, f64)> {
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if !(width > 0.0 && width < 1.0 && centre_fraction.is_finite()) {
        return None;
    }
    let next_start = (centre_fraction.clamp(0.0, 1.0) - width * 0.5).clamp(0.0, 1.0 - width);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_start + width, full.0, full.1),
    ))
}
