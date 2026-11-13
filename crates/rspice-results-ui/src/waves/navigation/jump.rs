//! Moving the stepping cursor to a feature of the trace it reads: the
//! maximum, the minimum, the next or previous peak, crossing or sample.

use super::super::{StripModel, StripTrace, anchor_key};
use super::{recentered_shared_x_view, shared_axis_viewport_fraction};
use crate::session::ResultViewerState;
use crate::waves::stack::{set_shared_x_view, shared_x_view};
use rspice_results::feature_search::{self, Direction, Feature, SearchRefusal};
use rspice_results::result_presentation::WavePanePresentationKey;
use rspice_ui_kit::plot::CursorId;

/// Where a jump moves the stepping cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorJump {
    Maximum,
    Minimum,
    Peak(Direction),
    Crossing(Direction),
    Sample(Direction),
}

impl CursorJump {
    /// What the jump looks for, as a refusal names it.
    fn noun(self) -> &'static str {
        match self {
            Self::Maximum => "maximum",
            Self::Minimum => "minimum",
            Self::Peak(_) => "peak",
            Self::Crossing(_) => "crossing",
            Self::Sample(_) => "sample",
        }
    }
}

/// The trace a jump searches, on `model`: the selected trace, else the one
/// cursor A was anchored to, else the first trace of the active pane, else
/// the strip's first. Only a visible trace of the active run is searched; an
/// overlay run is a comparison, not the curve being measured.
pub fn jump_trace<'a>(
    session: &ResultViewerState,
    model: &'a StripModel,
) -> Option<&'a StripTrace> {
    let candidates = || {
        model
            .traces
            .iter()
            .filter(|trace| trace.visible && !trace.overlay)
    };
    let selected = session
        .selected_trace
        .as_ref()
        .filter(|selected| selected.analysis_key() == model.analysis_key)
        .and_then(|selected| {
            candidates().find(|trace| trace.source_waveform_name == selected.source_name())
        });
    let anchored = || {
        session
            .cursor_a_anchor
            .as_ref()
            .and_then(|anchor| candidates().find(|trace| anchor_key(model, trace) == *anchor))
    };
    let active = || {
        session
            .active_wave_pane
            .as_ref()
            .filter(|pane| pane.analysis == model.analysis_key)
            .and_then(|pane| candidates().find(|trace| model.trace_unit(trace) == pane.unit))
    };
    selected
        .or_else(anchored)
        .or_else(active)
        .or_else(|| candidates().next())
}

/// The level a crossing jump on `trace` looks for: H1 of the trace's pane,
/// else H2, else the trace's mid level, `(min + max) / 2`, as the
/// calculator's periodic measures default to.
pub fn crossing_level(session: &ResultViewerState, model: &StripModel, trace: &StripTrace) -> f64 {
    let pane = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: model.trace_unit(trace).to_owned(),
    };
    let [h1, h2] = session.pane_horizontal_cursors(&pane);
    h1.or(h2).unwrap_or_else(|| {
        let (low, high) = trace
            .y
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
                (low.min(value), high.max(value))
            });
        (low + high) * 0.5
    })
}

/// Move the stepping cursor to `jump` on the trace [`jump_trace`] picks.
///
/// The strip is the cursor's own, or, before any cursor is placed, the
/// active pane's, else the first. A jump starts from the stepping cursor, or
/// from the other cursor when the stepping one is not placed yet, so "B to
/// the next crossing" with only A placed puts B on the crossing after A. The
/// strip scrolls to keep the cursor in view. A jump that finds nothing moves
/// nothing and says why.
pub fn jump_cursor(
    session: &mut ResultViewerState,
    models: &[StripModel],
    jump: CursorJump,
) -> Result<f64, String> {
    let model = session
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))
        .or_else(|| {
            session.active_wave_pane.as_ref().and_then(|pane| {
                models
                    .iter()
                    .find(|model| model.analysis_key == pane.analysis)
            })
        })
        .or_else(|| models.iter().find(|model| model.x_range.is_some()))
        .ok_or("there is no waveform strip to move a cursor on")?;
    let trace = jump_trace(session, model)
        .ok_or_else(|| format!("{} has no visible trace to search", model.table_label()))?;
    let same_strip = session.cursor_strip == Some(model.analysis_index);
    let id = session.stepping_cursor;
    let from = same_strip
        .then(|| session.cursors.get(id).or(session.cursors.get(id.other())))
        .flatten();
    let direction = match jump {
        CursorJump::Peak(direction)
        | CursorJump::Crossing(direction)
        | CursorJump::Sample(direction) => Some(direction),
        CursorJump::Maximum | CursorJump::Minimum => None,
    };
    let from = from.unwrap_or(match direction {
        Some(Direction::Previous) => f64::INFINITY,
        _ => f64::NEG_INFINITY,
    });
    let feature = match jump {
        CursorJump::Maximum => Feature::Maximum,
        CursorJump::Minimum => Feature::Minimum,
        CursorJump::Peak(direction) => Feature::Peak(direction),
        CursorJump::Sample(direction) => Feature::Sample(direction),
        CursorJump::Crossing(direction) => Feature::Crossing {
            level: crossing_level(session, model, trace),
            direction,
        },
    };
    let excursion = feature_search::default_excursion(&trace.y);
    let found =
        feature_search::find(&trace.x, &trace.y, from, feature, excursion).map_err(|refusal| {
            match refusal {
                SearchRefusal::NotFound => {
                    let side = match direction {
                        Some(Direction::Next) => " after the cursor",
                        Some(Direction::Previous) => " before the cursor",
                        None => "",
                    };
                    let which = match direction {
                        Some(Direction::Next) => "next ",
                        Some(Direction::Previous) => "previous ",
                        None => "",
                    };
                    format!("{} has no {which}{}{side}", trace.name, jump.noun())
                }
                refusal => format!("{}: {refusal}", trace.name),
            }
        })?;

    if !same_strip {
        session.clear_cursors();
        session.cursor_strip = Some(model.analysis_index);
        session.stepping_cursor = id;
    }
    session.arm_cursor_tool();
    session.cursors.set(id, found.x);
    if id == CursorId::A {
        session.cursor_a_anchor = Some(anchor_key(model, trace));
    }
    follow(session, model, found.x);
    Ok(found.x)
}

/// Scroll the strip so `x` is in view, keeping the zoom.
fn follow(session: &mut ResultViewerState, model: &StripModel, x: f64) {
    let Some(full) = model.x_range else {
        return;
    };
    let panes = model.unit_panes().len();
    let Some(view) = shared_x_view(session, model.analysis_key, panes) else {
        return;
    };
    if x >= view.0.min(view.1) && x <= view.0.max(view.1) {
        return;
    }
    let (centre, _) = shared_axis_viewport_fraction(model.x_scale, full, (x, x));
    if let Some(range) = recentered_shared_x_view(model.x_scale, full, view, centre) {
        set_shared_x_view(session, model.analysis_key, panes, Some(range));
    }
}
