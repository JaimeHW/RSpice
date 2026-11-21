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
use crate::ui::plot::floored_span;

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
pub(crate) fn zoom_active_pane(state: &mut ResultsParts<'_>, t: &Tokens, factor: f64) {
    let presentation = state.results_reads().presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        state.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((model, ordinal, pane)) = active_pane(&models, &state.results.session) else {
        return;
    };
    let pane_count = model.unit_panes().len();
    let logarithmic_y = pane_log_y(&state.results.session, model, &pane);
    let current = state.results.session.analysis_plot_view_pane(
        super::super::ResultViewer::Waves,
        model.analysis_key,
        ordinal,
    );
    let x = shared_x_view(&state.results.session, model.analysis_key, pane_count)
        .or(model.x_range)
        .and_then(|range| scaled_range(range, factor, model.x_scale == XScale::Log10))
        .and_then(|range| floored_span(range, model.x_scale));
    let y = match current.y {
        Some(range) => Some(range),
        None => displayed_pane_auto_y(state, model, &pane, ordinal, t),
    }
    .and_then(|range| scaled_range(range, factor, logarithmic_y))
    .and_then(|range| {
        floored_span(
            range,
            if logarithmic_y {
                XScale::Log10
            } else {
                XScale::Linear
            },
        )
    });
    if let Some(x) = x {
        set_shared_x_view(
            &mut state.results.session,
            model.analysis_key,
            pane_count,
            Some(x),
        );
    }
    if let Some(y) = y {
        state
            .results
            .session
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
pub(crate) fn fit_active_pane(state: &mut ResultsParts<'_>, t: &Tokens) {
    let presentation = state.results_reads().presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        state.results,
        presentation.complex_number_display(),
        t,
    );
    let Some((model, ordinal, _)) = active_pane(&models, &state.results.session) else {
        return;
    };
    state.results.session.clear_analysis_plot_view_axis(
        super::super::ResultViewer::Waves,
        model.analysis_key,
        super::super::PaneAxis::X,
    );
    state
        .results
        .session
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
pub(crate) fn fit_active_strip(state: &mut ResultsParts<'_>) {
    let viewer = super::super::ResultViewer::Waves;
    match state.results.session.active_wave_pane.as_ref() {
        Some(key) => {
            let analysis = key.analysis;
            state
                .results
                .session
                .reset_analysis_plot_view(viewer, analysis);
        }
        None => state.results.session.reset_all_analysis_plot_views(viewer),
    }
}

/// Fit Y to the window in view. Every pane of the active pane's strip lets go
/// of the Y range it was pinned to, so each refits to what the X window
/// shows, and the window itself stays where the reader put it. With no
/// active pane every strip of the sheet fits, the XY strips with them.
pub(crate) fn fit_y_to_visible_x(state: &mut ResultsParts<'_>) {
    let viewer = super::super::ResultViewer::Waves;
    let session = &mut state.results.session;
    match session.active_wave_pane.as_ref().map(|key| key.analysis) {
        Some(analysis) => {
            session.clear_analysis_plot_view_axis(viewer, analysis, super::super::PaneAxis::Y);
        }
        None => {
            session.clear_every_analysis_plot_view_axis(viewer, super::super::PaneAxis::Y);
            for strip in session.xy_strips.iter_mut() {
                strip.y_view = None;
            }
        }
    }
}

/// A cursor jump whose search walks a stored trace on a Results job; the cursor moves when it
/// answers.
#[derive(Debug, Clone)]
pub(crate) struct PendingJump {
    pub(crate) search: rspice_results_ui::waves::navigation::JumpSearch,
    pub(crate) job: crate::workbench::results_job::ResultsJob<Result<f64, String>>,
}

/// Move the stepping cursor to `jump` on the trace it reads: the selected
/// trace, else the one cursor A was anchored to, else the active pane's.
///
/// The arrow keys step it sample by sample, Shift+arrow from peak to peak
/// and Alt+arrow from crossing to crossing; the Results commands move it to
/// the maximum, the minimum, a peak or a crossing.
///
/// A step to a neighbouring sample, and any jump on a trace in memory, lands now: `Some` of
/// where. A peak, crossing or extreme of a stored trace is searched on a Results job, since the
/// search reads the trace from its start; `None`, and [`poll_cursor_jump`] moves the cursor
/// when the job answers. A jump asked while another searches replaces it.
pub(crate) fn jump_cursor(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
    jump: rspice_results_ui::waves::navigation::CursorJump,
) -> Result<Option<f64>, String> {
    use rspice_results_ui::waves::navigation::{land_jump, prepare_jump};
    let presentation = state.results_reads().presentation_policy();
    let models = cached_models(
        &state.model.simulation,
        state.results,
        presentation.complex_number_display(),
        tokens,
    );
    if let Some(running) = state.results.cursor_jump.take() {
        running.job.cancel();
    }
    let search = prepare_jump(&state.results.session, &models, jump)?;
    if !search.walks_store() {
        let x = search.run()?;
        land_jump(&mut state.results.session, &models, &search, x);
        return Ok(Some(x));
    }
    let work = search.clone();
    let job = crate::workbench::results_job::ResultsJob::start("cursor-jump", move |job| {
        job.step(&format!("Searching {}", work.trace_name()), 0);
        Ok(work.run())
    });
    state.results.cursor_jump = Some(PendingJump { search, job });
    // A job that ran as it started, as the browser's do, answers now.
    poll_cursor_jump(state, tokens);
    Ok(None)
}

/// Move the cursor a jump's search found, once its job has answered, or say on the status bar
/// why it found nothing. Whether a search is still running.
pub(crate) fn poll_cursor_jump(state: &mut ResultsParts<'_>, tokens: &Tokens) -> bool {
    use crate::workbench::results_job::JobOutcome;
    let Some(pending) = state.results.cursor_jump.as_ref() else {
        return false;
    };
    if !pending.job.is_finished() {
        return true;
    }
    let Some(pending) = state.results.cursor_jump.take() else {
        return false;
    };
    match pending.job.take_outcome() {
        Some(JobOutcome::Done(Ok(x))) => {
            let presentation = state.results_reads().presentation_policy();
            let models = cached_models(
                &state.model.simulation,
                state.results,
                presentation.complex_number_display(),
                tokens,
            );
            rspice_results_ui::waves::navigation::land_jump(
                &mut state.results.session,
                &models,
                &pending.search,
                x,
            );
        }
        Some(JobOutcome::Done(Err(refusal))) => state.prompt(format!("{refusal}.")),
        Some(JobOutcome::Failed(reason)) => state.prompt(format!(
            "{} could not be searched: {reason}.",
            pending.search.trace_name()
        )),
        Some(JobOutcome::Cancelled) | None => {}
    }
    false
}

/// Wait for a cursor jump's search and land it as [`poll_cursor_jump`] does: the search's
/// answer, or `None` when none was running.
#[cfg(test)]
pub(crate) fn settle_cursor_jump(
    state: &mut ResultsParts<'_>,
    tokens: &Tokens,
) -> Option<Result<f64, String>> {
    use crate::workbench::results_job::JobOutcome;
    let pending = state.results.cursor_jump.take()?;
    assert!(
        pending.job.wait(std::time::Duration::from_secs(60)),
        "the search answers"
    );
    let answer = match pending.job.take_outcome() {
        Some(JobOutcome::Done(answer)) => answer,
        outcome => panic!("the search ended {outcome:?}"),
    };
    if let Ok(x) = answer {
        let presentation = state.results_reads().presentation_policy();
        let models = cached_models(
            &state.model.simulation,
            state.results,
            presentation.complex_number_display(),
            tokens,
        );
        rspice_results_ui::waves::navigation::land_jump(
            &mut state.results.session,
            &models,
            &pending.search,
            x,
        );
    }
    Some(answer)
}

/// Whether a cursor jump has a waveform sheet to work on, or why not.
pub(crate) fn cursor_jump_availability(
    state: super::super::ResultsReads<'_>,
) -> Result<(), &'static str> {
    if !state.model.simulation.retained.has_results() {
        return Err("no result dataset is available");
    }
    if !rspice_results::result_presentation::viewer_uses_wave_stack(state.results.session.viewer) {
        return Err("open a waveform sheet in Results");
    }
    Ok(())
}
