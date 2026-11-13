//! The strip's logic and bus lanes as the Results workspace hosts them: what
//! they draw from, what the reader's requests change, and where selecting a
//! lane cross-probes.
//!
//! Which lanes exist follows the plotted `D()` columns, so plotting or
//! removing a digital net from the browser adds or removes its lane through
//! the same visibility a value trace has. The session's layout holds the
//! order, each bus's radix and expansion, the reader's buses and the
//! selection; over a project result document it is the document's, projected
//! in before the frame and kept after it.
use super::*;
use crate::ui::plot::CursorPair;
use rspice_results_ui::waves::lanes::{self, EventLaneLayout};
use rspice_results_ui::waves::logic::{self, LaneProbe, LaneStackInput};
use rspice_results_ui::waves::stack::EventLaneFrame;

/// Height of the lanes `model`'s strip shows above its panes.
pub(super) fn lane_height(state: &ResultsParts<'_>, model: &StripModel) -> f32 {
    let layout = state.results.session.event_lanes.get(&model.analysis_key);
    lanes::stack_height(&lanes::lane_specs(model, layout))
}

/// Draw `model`'s lanes on the panes' time axis and apply what the reader
/// asked of them.
pub(super) fn show_lanes(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    frame: EventLaneFrame,
) {
    let Some(history) = model.event_lanes.clone() else {
        return;
    };
    let digits = usize::from(
        state
            .results_reads()
            .presentation_policy()
            .readout()
            .displayed_significant_digits()
            .get(),
    );
    let session = &mut state.results.session;
    let mut layout = session
        .event_lanes
        .get(&model.analysis_key)
        .cloned()
        .unwrap_or_default();
    let specs = lanes::lane_specs(model, Some(&layout));
    lanes::reconcile(&mut layout, &specs);
    let user = lanes::user_bus_lanes(&mut session.derived, model, &specs);
    let rows = lanes::lane_rows(&history, &specs, &user);
    let selected = lanes::selected_rows(&layout, &specs);
    let owns_cursors = session.cursor_strip == Some(model.analysis_index);
    let cursors = if owns_cursors {
        session.cursors
    } else {
        CursorPair::default()
    };
    let response = logic::show_lanes(
        ui,
        &LaneStackInput {
            rows: &rows,
            x_range: frame.x_range,
            left_margin: frame.left_margin,
            name_width: frame.name_width,
            right_margin: frame.right_margin,
            selected: &selected,
            cursor: cursors.a,
            cursor_b: cursors.b,
            significant_digits: digits,
            // An assertion that reads one digital net shades its violations
            // on that net's lane too.
            violations: model
                .event_markers
                .as_deref()
                .map_or(&[], |markers| markers.assertions.as_slice()),
            // The retained events do not yet carry where an HDL net was
            // declared or what drives it, so those probes have nowhere to go.
            source_probes: false,
        },
    );
    if let Some(width) = response.resize_name {
        session.lane_name_width = Some(rspice_results_ui::waves::stack::clamp_name_width(width));
    }
    let requests = lanes::apply_response(&mut layout, &specs, response);
    store_layout(session, model.analysis_key, layout);
    if let Some(t) = requests.cursor_to {
        step_cursor(session, model.analysis_index, t);
    }
    if let Some(LaneProbe::SelectNet { signal }) = requests.probe {
        select_lane_net(state, model, &signal);
    }
    if let Some(text) = requests.add_expression {
        super::expressions::add_strip_expression(state, model.analysis_key, text);
    }
}

/// Keep `layout` for the strip, writing only when it changed so an idle frame
/// leaves the session alone.
fn store_layout(
    session: &mut rspice_results_ui::session::ResultViewerState,
    analysis: AnalysisPresentationKey,
    layout: EventLaneLayout,
) {
    if session.event_lanes.get(&analysis) != Some(&layout) {
        session.event_lanes.insert(analysis, layout);
    }
}

/// Move the cursor placed last (B once placed, A before) to `t`, taking the
/// cursors to this strip if another held them.
pub(super) fn step_cursor(
    session: &mut rspice_results_ui::session::ResultViewerState,
    analysis_index: usize,
    t: f64,
) {
    if session.cursor_strip != Some(analysis_index) {
        session.cursors = CursorPair::default();
        session.cursor_strip = Some(analysis_index);
    }
    if session.cursors.b.is_some() {
        session.cursors.b = Some(t);
    } else {
        session.cursors.a = Some(t);
        session.cursor_a_anchor = None;
    }
}

/// Selecting a lane selects its `D()` column as the inspected trace and
/// highlights its net on the schematic, as the schematic's own cross-probe
/// does. A run with no current drawing, a deck run among them, has nothing
/// to highlight, and selecting the lane is all that happens.
fn select_lane_net(state: &mut ResultsParts<'_>, model: &StripModel, signal: &str) {
    if model
        .lane_traces
        .iter()
        .any(|lane| lane.source_waveform_name == signal)
    {
        state.results.session.selected_trace = Some(SelectedResultTrace::from_identity(
            model.analysis_key,
            signal.to_owned(),
        ));
    }
    if state.model.result_mapping_is_current() {
        let _ = state.model.select_signal_conductor(signal);
    }
}

/// Whether the event net `name`, its `D()` or `E()` column's name, is on the
/// active run's plot of `analysis`.
pub fn event_net_plotted(
    state: &ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    name: &str,
) -> bool {
    let Some(default) = event_net_default(state, analysis, name) else {
        return false;
    };
    let key = SourceWaveformPresentationKey::new(analysis, name.to_owned());
    waveform_plotted(state.model, state.results, &key, name, default)
}

/// Put the event net `name` on the active run's plot of `analysis`, or take
/// it off. The net need not be a column: a native mixed run keeps its nets
/// only as event history, and the lane answers to the column's name.
pub fn set_event_net_plotted(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    name: &str,
    plotted: bool,
) {
    let Some(default) = event_net_default(state, analysis, name) else {
        return;
    };
    if event_net_plotted(state, analysis, name) != plotted {
        let key = SourceWaveformPresentationKey::new(analysis, name.to_owned());
        toggle_source_visibility(state, key, name.to_owned(), default);
    }
}

/// Whether the event net `name` of `analysis` on the active run is plotted
/// before the reader says otherwise: as its column is, and off when the run
/// keeps it only as history. `None` when the run records no such net.
fn event_net_default(
    state: &ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    name: &str,
) -> Option<bool> {
    let run = state.model.simulation.active_run()?;
    let (_, retained) = analysis.resolve(run)?;
    if let Some(column) = retained
        .waveforms
        .iter()
        .find(|waveform| waveform.name == name)
    {
        return Some(column.visible);
    }
    let crate::state::AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
        ..
    } = retained.result_payload.as_ref()?
    else {
        return None;
    };
    let recorded = |prefix: &str| {
        name.strip_prefix(prefix)
            .and_then(|rest| rest.strip_suffix(')'))
    };
    let known = recorded("D(")
        .is_some_and(|net| digital_traces.iter().any(|trace| trace.node_name == net))
        || recorded("E(").is_some_and(|net| real_traces.iter().any(|trace| trace.node_name == net));
    known.then_some(false)
}

/// Put the event nets `columns` of `analysis` on the waveform plot and
/// cursor A on `time` there, as the Events sheet's plot action asks: a
/// digital net becomes a lane, a bus a bus lane once its members are all
/// plotted, a real net a step trace. The Waves sheet comes forward on it.
pub(in crate::workbench::documents::result_document) fn plot_event_lanes(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    columns: &[String],
    time: f64,
) {
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    let Some((analysis_index, _)) = analysis.resolve(run) else {
        return;
    };
    for name in columns {
        set_event_net_plotted(state, analysis, name, true);
    }
    let session = &mut state.results.session;
    session.hidden_strips.remove(&analysis);
    if session
        .maximized_strip
        .is_some_and(|maximized| maximized != analysis)
    {
        session.maximized_strip = None;
    }
    session.cursor_strip = Some(analysis_index);
    session.cursors = CursorPair {
        a: Some(time),
        b: None,
    };
    session.cursor_a_anchor = None;
    session.viewer = rspice_results::result_presentation::ResultViewer::Waves;
}
