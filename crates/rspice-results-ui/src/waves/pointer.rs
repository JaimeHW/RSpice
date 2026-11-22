//! Where the keys that act at the pointer put what they place, as ViVA's
//! bindkeys do: a cursor, a horizontal cursor, a marker or a note where the
//! pointer stands over a strip's plot. Asked from a menu or the palette,
//! with the pointer on no plot, they place it where the Cursors page's Add
//! does, on the strip the cursors follow.

use super::cursor_table::{
    MarkerSpot, add_horizontal, followed_model, marker_spot as page_marker_spot, shown_x_window,
    snapped, window_point,
};
use super::cursor_values::marker_x;
use super::dock::ReadoutInput;
use super::{StripModel, StripTrace, anchor_key};
use crate::session::{
    MarkerSelector, NoteAnchor, NoteDraft, PlotPointer, ReadingColumnId, ResultViewerState,
};
use rspice_results::result_presentation::{AnalysisPresentationKey, WavePanePresentationKey};
use rspice_ui_kit::plot::{CursorId, CursorPair, XScale};

/// What a key that acts at the pointer places or opens there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtPointer {
    CursorA,
    CursorB,
    HorizontalCursor,
    VerticalMarker,
    PointMarker,
    /// A point marker the Cursors table reads against a marker before it.
    DeltaMarker,
    Note,
    /// The properties of the marker nearest the pointer.
    Properties,
}

/// The strip the pointer stands over, if it is one of `source`'s, and
/// where on it.
fn pointed<'a>(
    state: &ResultViewerState,
    source: ReadoutInput<'a>,
) -> Option<(&'a StripModel, PlotPointer)> {
    let pointer = state.plot_pointer.clone()?;
    let model = source
        .models
        .iter()
        .find(|model| model.analysis_key == pointer.analysis)?;
    Some((model, pointer))
}

/// The strip the cursors follow, else the first on screen.
fn followed<'a>(state: &ResultViewerState, source: ReadoutInput<'a>) -> Option<&'a StripModel> {
    followed_model(state, source).map(|index| &source.models[index])
}

/// Put cursor `id` where the pointer stands, arming the cursors; A rides
/// the curve nearest the pointer. With the pointer on no plot, A goes a
/// third and B two thirds across the followed strip's window, as the
/// Cursors page's pair does. Says whether there was a strip to put it on.
pub fn place_cursor(state: &mut ResultViewerState, source: ReadoutInput<'_>, id: CursorId) -> bool {
    let (model, x, anchor) = match pointed(state, source) {
        Some((model, pointer)) => (model, pointer.x, Some(pointer.trace)),
        None => {
            let Some(model) = followed(state, source) else {
                return false;
            };
            let Some(window) = shown_x_window(state, model) else {
                return false;
            };
            let fraction = if id == CursorId::A {
                1.0 / 3.0
            } else {
                2.0 / 3.0
            };
            (model, window_point(model, window, fraction), None)
        }
    };
    state.arm_cursor_tool();
    if state.cursor_strip != Some(model.analysis_index) {
        state.cursors = CursorPair::default();
        state.cursor_a_anchor = None;
        state.cursor_strip = Some(model.analysis_index);
    }
    if id == CursorId::A
        && let Some(anchor) = anchor
    {
        state.cursor_a_anchor = anchor;
    }
    let x = snapped(state, model, state.cursor_a_anchor.as_ref(), x);
    state.cursors.set(id, x);
    state.stepping_cursor = id;
    true
}

/// Put a horizontal cursor at the pointer's height on the pane it is over:
/// H1 while it is free, else H2. With the pointer on no plot, as the
/// Cursors page adds one. Says whether one was placed.
pub fn place_horizontal(state: &mut ResultViewerState, source: ReadoutInput<'_>) -> bool {
    let Some((model, pointer)) = pointed(state, source) else {
        return add_horizontal(state, source);
    };
    let key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pointer.unit,
    };
    let slot = state
        .pane_horizontal_cursors(&key)
        .iter()
        .position(Option::is_none)
        .unwrap_or(1);
    state.set_horizontal_cursor(key.clone(), slot, pointer.y);
    state.active_wave_pane = Some(key);
    true
}

/// Where a marker goes: on the curve nearest the pointer at its X, else on
/// the first curve of the pane it is over. With the pointer on no plot,
/// where the Cursors page's Add puts one.
pub fn marker_spot(state: &ResultViewerState, source: ReadoutInput<'_>) -> Option<MarkerSpot> {
    let Some((model, pointer)) = pointed(state, source) else {
        return page_marker_spot(state, source);
    };
    let rides = |trace: &&StripTrace| !trace.overlay && !trace.is_expression();
    let trace = pointer
        .trace
        .as_ref()
        .and_then(|key| {
            model
                .traces
                .iter()
                .filter(rides)
                .find(|trace| anchor_key(model, trace) == *key)
        })
        .or_else(|| {
            let panes = model.unit_panes();
            let pane = panes.iter().find(|pane| pane.unit == pointer.unit)?;
            pane.traces
                .iter()
                .filter_map(|index| model.traces.get(*index))
                .find(rides)
        })?;
    let anchor = anchor_key(model, trace);
    Some(MarkerSpot {
        analysis: model.analysis_key,
        x: snapped(state, model, Some(&anchor), pointer.x),
        anchor,
        trace_name: trace.name.clone(),
        samples: trace.x.clone(),
    })
}

/// A note to write at the pointer, on the point of the data under it. With
/// the pointer on no plot, in the middle of the frame of the followed
/// strip's active pane, else its first.
pub fn note_draft(state: &ResultViewerState, source: ReadoutInput<'_>) -> Option<NoteDraft> {
    let (analysis, anchor) = match pointed(state, source) {
        Some((model, pointer)) => (
            model.analysis_key,
            NoteAnchor::Data {
                unit: pointer.unit,
                x: pointer.x,
                y: pointer.y,
            },
        ),
        None => {
            let model = followed(state, source)?;
            let panes = model.unit_panes();
            let pane = state
                .active_wave_pane
                .as_ref()
                .filter(|active| active.analysis == model.analysis_key)
                .and_then(|active| panes.iter().find(|pane| pane.unit == active.unit))
                .or_else(|| panes.first())?;
            (
                model.analysis_key,
                NoteAnchor::Frame {
                    unit: Some(pane.unit.to_owned()),
                    x: 0.5,
                    y: 0.5,
                },
            )
        }
    };
    Some(NoteDraft {
        analysis,
        note: None,
        anchor,
        text: String::new(),
    })
}

/// The marker nearest the pointer along X on the strip it is over, where
/// each is drawn. With the pointer on no plot, the newest marker of the
/// followed strip.
pub fn nearest_marker(
    state: &ResultViewerState,
    source: ReadoutInput<'_>,
) -> Option<MarkerSelector> {
    match pointed(state, source) {
        Some((model, pointer)) => {
            let along = |x: f64| match model.x_scale {
                XScale::Log10 if x > 0.0 && pointer.x > 0.0 => {
                    (x.log10() - pointer.x.log10()).abs()
                }
                _ => (x - pointer.x).abs(),
            };
            state
                .strip_markers(model.analysis_key)
                .map(|marker| {
                    let x = marker_x(model, state, marker).unwrap_or_else(|placed| placed);
                    (along(x), marker.selector())
                })
                .min_by(|(a, _), (b, _)| a.total_cmp(b))
                .map(|(_, selector)| selector)
        }
        None => newest_marker(state, followed(state, source)?.analysis_key, None),
    }
}

/// The newest marker of the strip of `analysis` other than `except`.
fn newest_marker(
    state: &ResultViewerState,
    analysis: AnalysisPresentationKey,
    except: Option<MarkerSelector>,
) -> Option<MarkerSelector> {
    state
        .strip_markers(analysis)
        .filter(|marker| Some(marker.selector()) != except)
        .max_by_key(|marker| marker.0.retained_id)
        .map(|marker| marker.selector())
}

/// After a delta marker `placed` on the strip of `analysis`: the Cursors
/// table reads it against a marker. A base that is a marker already stays;
/// otherwise the newest other marker of the strip becomes the base, or,
/// with none, the new one, for the next delta marker to read against.
pub fn read_delta_against_base(
    state: &mut ResultViewerState,
    analysis: AnalysisPresentationKey,
    placed: MarkerSelector,
) {
    if matches!(state.readout_base, Some(ReadingColumnId::Marker(_))) {
        return;
    }
    let base = newest_marker(state, analysis, Some(placed)).unwrap_or(placed);
    state.readout_base = Some(ReadingColumnId::Marker(base));
}
