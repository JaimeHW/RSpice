//! Moving the stepping cursor to a feature of the trace it reads: the
//! maximum, the minimum, the next or previous peak, crossing or sample.

use super::super::{StripModel, StripTrace, UnitPane, anchor_key};
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
            .and_then(|active| {
                let panes = model.unit_panes();
                let unit = panes
                    .iter()
                    .find(|pane| pane.unit == active.unit)
                    .map_or(active.unit.as_str(), UnitPane::quantity_unit);
                candidates().find(|trace| model.trace_unit(trace) == unit)
            })
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
        let (low, high) = finite_span(trace);
        (low + high) * 0.5
    })
}

/// The least and greatest finite value of `trace`: a stored trace's from the store's
/// summaries, never its column.
fn finite_span(trace: &StripTrace) -> (f64, f64) {
    if let Some(extent) = trace.y.extent_over(&trace.x, None) {
        return (extent.min, extent.max);
    }
    trace
        .y
        .in_memory()
        .unwrap_or_default()
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        })
}

/// Move the stepping cursor to `jump` on the trace [`jump_trace`] picks, searching now.
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
    let search = prepare_jump(session, models, jump)?;
    let x = search.run()?;
    land_jump(session, models, &search, x);
    Ok(x)
}

/// A cursor jump's search, owned whole so it can run off the frame: the trace's columns, where
/// it starts, what it looks for, and the cursor it moves.
#[derive(Clone)]
pub struct JumpSearch {
    jump: CursorJump,
    feature: Feature,
    from: f64,
    excursion: f64,
    x: rspice_results::waveform::SharedWaveformValues,
    y: rspice_results::waveform::SharedWaveformValues,
    trace_name: String,
    strip: usize,
    anchor: rspice_results::result_presentation::WaveformPresentationKey,
    cursor: CursorId,
    /// The strip and the cursor's place when the search began.
    began: (Option<usize>, Option<f64>),
}

impl std::fmt::Debug for JumpSearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JumpSearch")
            .field("jump", &self.jump)
            .field("from", &self.from)
            .field("trace", &self.trace_name)
            .finish_non_exhaustive()
    }
}

impl JumpSearch {
    /// Whether the search walks a stored column: every jump but a step to the next or previous
    /// sample reads the trace from its start, so over a stored trace it runs off the frame.
    #[must_use]
    pub fn walks_store(&self) -> bool {
        !matches!(self.jump, CursorJump::Sample(_)) && (self.x.is_stored() || self.y.is_stored())
    }

    /// The trace searched, as the status bar names it.
    #[must_use]
    pub fn trace_name(&self) -> &str {
        &self.trace_name
    }

    /// Search the trace: where the cursor lands, or why nothing was found. A stored trace is
    /// read a block at a time, never held whole.
    pub fn run(&self) -> Result<f64, String> {
        let direction = match self.jump {
            CursorJump::Peak(direction)
            | CursorJump::Crossing(direction)
            | CursorJump::Sample(direction) => Some(direction),
            CursorJump::Maximum | CursorJump::Minimum => None,
        };
        let (x, y) = (
            feature_search::WindowedCurve::new(&self.x),
            feature_search::WindowedCurve::new(&self.y),
        );
        feature_search::find(&x, &y, self.from, self.feature, self.excursion)
            .map(|found| found.x)
            .map_err(|refusal| match refusal {
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
                    format!(
                        "{} has no {which}{}{side}",
                        self.trace_name,
                        self.jump.noun()
                    )
                }
                refusal => format!("{}: {refusal}", self.trace_name),
            })
    }
}

/// The search [`jump_cursor`] makes for `jump`, ready to run here or off the frame.
pub fn prepare_jump(
    session: &ResultViewerState,
    models: &[StripModel],
    jump: CursorJump,
) -> Result<JumpSearch, String> {
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
    let (low, high) = finite_span(trace);
    Ok(JumpSearch {
        jump,
        feature,
        from,
        excursion: feature_search::excursion_of_span(low, high),
        x: trace.x.clone(),
        y: trace.y.clone(),
        trace_name: trace.name.clone(),
        strip: model.analysis_index,
        anchor: anchor_key(model, trace),
        cursor: id,
        began: (session.cursor_strip, session.cursors.get(id)),
    })
}

/// Move the cursor `search` was made for to `x`, its answer, onto its strip in `models`, armed
/// and scrolled into view. A search that the reader overtook, by moving the cursor, choosing
/// another strip or another stepping cursor while it ran, lands nothing: `false`.
pub fn land_jump(
    session: &mut ResultViewerState,
    models: &[StripModel],
    search: &JumpSearch,
    x: f64,
) -> bool {
    let id = search.cursor;
    let overtaken = session.stepping_cursor != id
        || (session.cursor_strip, session.cursors.get(id)) != search.began;
    let Some(model) = models
        .iter()
        .find(|model| model.analysis_index == search.strip)
        .filter(|_| !overtaken)
    else {
        return false;
    };
    if session.cursor_strip != Some(search.strip) {
        session.clear_cursors();
        session.cursor_strip = Some(search.strip);
        session.stepping_cursor = id;
    }
    session.arm_cursor_tool();
    session.cursors.set(id, x);
    if id == CursorId::A {
        session.cursor_a_anchor = Some(search.anchor.clone());
    }
    follow(session, model, x);
    true
}

/// Where a step along `model`'s strip starts: the stepping cursor, else the
/// other cursor, when the cursors are on that strip.
#[must_use]
pub fn stepping_from(session: &ResultViewerState, model: &StripModel) -> Option<f64> {
    let id = session.stepping_cursor;
    (session.cursor_strip == Some(model.analysis_index))
        .then(|| session.cursors.get(id).or(session.cursors.get(id.other())))
        .flatten()
}

/// Move the stepping cursor to `x` on `model`'s strip, found by a search of
/// something other than one trace, as [`jump_cursor`] moves it: onto the
/// strip, armed, and scrolled into view.
pub fn place_stepping_cursor(session: &mut ResultViewerState, model: &StripModel, x: f64) {
    let id = session.stepping_cursor;
    if session.cursor_strip != Some(model.analysis_index) {
        session.clear_cursors();
        session.cursor_strip = Some(model.analysis_index);
        session.stepping_cursor = id;
    }
    session.arm_cursor_tool();
    session.cursors.set(id, x);
    follow(session, model, x);
}

/// Scroll the strip so `x` is in view, keeping the zoom.
fn follow(session: &mut ResultViewerState, model: &StripModel, x: f64) {
    let Some(full) = model.x_range else {
        return;
    };
    let Some(view) = shared_x_view(session, model.analysis_key) else {
        return;
    };
    if x >= view.0.min(view.1) && x <= view.0.max(view.1) {
        return;
    }
    let (centre, _) = shared_axis_viewport_fraction(model.x_scale, full, (x, x));
    if let Some(range) = recentered_shared_x_view(model.x_scale, full, view, centre) {
        set_shared_x_view(session, model.analysis_key, Some(range));
    }
}
