//! What plotting a data-browser quantity means: a waveform's tick is its
//! curve on the plot, an event net's or a bus's tick its lane. The browser
//! ticks rows and the strip takes rows dropped on it, so both reach the plot
//! through here.

use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use super::{AnalysisPresentationKey, ResultsParts, waveform_plotted};

/// Rows dragged out of the data browser: the quantities a drop on the plot
/// puts there, in the order the browser lists them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultQuantityDrag {
    pub keys: Vec<ResultBrowserSelectionKey>,
}

/// A dragged row's name: a waveform's own, an event net's or a bus's label.
pub(crate) fn drag_name(key: &ResultBrowserSelectionKey) -> &str {
    match key {
        ResultBrowserSelectionKey::Waveform(key) => key.source_name(),
        ResultBrowserSelectionKey::Artifact(key) => {
            let name = key.canonical_name();
            name.rsplit('/').next().unwrap_or(name)
        }
    }
}

/// The identity a declared bus's row carries, before its `name[msb:lsb]`
/// label: one per bus, so ticking one plots that bus alone.
pub(crate) const BUS_ROW_PREFIX: &str = "payload/transient-events/bus/";
/// The identity of the row of an event net the run keeps only as history,
/// before the `D(net)` or `E(net)` name its lane answers to.
pub(crate) const EVENT_NET_ROW_PREFIX: &str = "payload/transient-events/net/";

/// The strip models the result document last built, for a browser row that
/// takes its curve's colour from what the strip drew.
pub(crate) fn last_strip_models<'a>(
    state: &'a ResultsParts<'_>,
) -> Option<&'a [rspice_results_ui::waves::StripModel]> {
    state.results.models.last_built()
}

/// The colour the strip drew a waveform's curve in, so its browser row and the
/// Inspector's swatch read in the curve's hue: the strip gives curves their
/// palette places in the order they were plotted, not by column. Before the
/// strip has drawn the analysis, the colour the waveform stores, or the
/// palette place of its column.
pub(crate) fn plotted_signal_color(
    models: Option<&[rspice_results_ui::waves::StripModel]>,
    palette: &[egui::Color32],
    analysis: AnalysisPresentationKey,
    waveform_index: usize,
    stored: &str,
) -> egui::Color32 {
    models
        .and_then(|models| models.iter().find(|model| model.analysis_key == analysis))
        .and_then(|model| {
            model
                .traces
                .iter()
                .find(|trace| !trace.overlay && trace.waveform_index == waveform_index)
        })
        .map_or_else(
            || {
                rspice_results_ui::presentation::trace_color(
                    stored,
                    palette[waveform_index % palette.len()],
                )
            },
            |trace| trace.color,
        )
}

/// Whether a row's tick is on. A waveform's tick is whether it is on the plot:
/// there is no second record of it that the legend or the Inspector could
/// disagree with.
pub(crate) fn result_quantity_ticked(
    state: &ResultsParts<'_>,
    key: &ResultBrowserSelectionKey,
) -> bool {
    if let ResultBrowserSelectionKey::Artifact(artifact) = key
        && let Some(nets) = event_row_nets(state, artifact)
    {
        return !nets.is_empty()
            && nets
                .iter()
                .all(|net| super::event_net_plotted(state, artifact.analysis(), net));
    }
    match key {
        ResultBrowserSelectionKey::Waveform(key) => result_signal_ticked(state, key),
        ResultBrowserSelectionKey::Artifact(_) => state
            .results
            .session
            .checked_result_quantities
            .contains(key),
    }
}

pub fn set_result_quantity_ticked(
    state: &mut ResultsParts<'_>,
    key: &ResultBrowserSelectionKey,
    ticked: bool,
) {
    // An event net's row plots its lane; a bus row plots the bus, its
    // members, which the strip then draws as one bus lane.
    if let ResultBrowserSelectionKey::Artifact(artifact) = key
        && let Some(nets) = event_row_nets(state, artifact)
    {
        for net in &nets {
            super::set_event_net_plotted(state, artifact.analysis(), net, ticked);
        }
        if ticked && !nets.is_empty() {
            reveal_strip(state, artifact.analysis());
        }
        return;
    }
    match key {
        ResultBrowserSelectionKey::Waveform(key) => set_result_signal_plotted(state, key, ticked),
        ResultBrowserSelectionKey::Artifact(_) => {
            let checked = &mut state.results.session.checked_result_quantities;
            if ticked {
                checked.insert(key.clone());
            } else {
                checked.remove(key);
            }
        }
    }
}

/// Tick or untick every quantity of `keys`, as "Tick visible quantities", a
/// Shift range, "Plot all" and a saved set do: each signal is found in its
/// analysis's index of signals by name, where finding them one at a time
/// walked a million-column analysis once for each signal.
pub fn set_result_quantities_ticked<'k>(
    state: &mut ResultsParts<'_>,
    keys: impl IntoIterator<Item = &'k ResultBrowserSelectionKey>,
    ticked: bool,
) {
    let keys = keys.into_iter().collect::<Vec<_>>();
    let found = find_signals(
        &state.model.simulation.retained.runs,
        (
            &state.results.session.signal_indices,
            &state.model.simulation.retained.runs.revision(),
        ),
        &keys,
    );
    let mut gesture = Gesture::default();
    for (key, found) in keys.into_iter().zip(found) {
        match (key, found) {
            (ResultBrowserSelectionKey::Waveform(signal), Some(at))
                if tick_quick_signal(state, signal, at, ticked, &mut gesture) => {}
            (ResultBrowserSelectionKey::Waveform(signal), Some(at)) => {
                set_found_signal_plotted(state, signal, at, ticked);
            }
            (ResultBrowserSelectionKey::Waveform(_), None) => {}
            (ResultBrowserSelectionKey::Artifact(_), _) => {
                set_result_quantity_ticked(state, key, ticked);
            }
        }
    }
    gesture.finish(state);
}

/// What ticking many signals of the quick view at once leaves to do once
/// for them all: the strips to bring back, the signals put on last for the
/// browser's Recent scope, and the cached series to build again once a hide
/// is forgotten. The strip models follow the ticks by the edits they
/// record, as they follow a single tick.
#[derive(Default)]
struct Gesture<'k> {
    revealed: Vec<AnalysisPresentationKey>,
    recent: Vec<&'k SourceWaveformPresentationKey>,
    forgot: bool,
}

impl Gesture<'_> {
    fn finish(self, state: &mut ResultsParts<'_>) {
        for analysis in self.revealed {
            reveal_strip(state, analysis);
        }
        for key in self.recent {
            state.results.session.note_recent_signal(key.clone());
        }
        if self.forgot {
            state.results.session.cache.invalidate();
        }
    }
}

/// How many signals the browser's Recent scope lists: of those a gesture
/// puts on, the last so many are noted.
const RECENT_SIGNALS: usize = 24;

/// [`set_found_signal_plotted`] for a signal of the shown run drawn from
/// the quick view's ticks, as one of many a gesture ticks: what is done once
/// for them all is left to `gesture`. `false`, doing nothing, for any other
/// signal.
fn tick_quick_signal<'k>(
    state: &mut ResultsParts<'_>,
    key: &'k SourceWaveformPresentationKey,
    (run_index, analysis_index, waveform_index): (usize, usize, usize),
    plotted: bool,
    gesture: &mut Gesture<'k>,
) -> bool {
    if state.model.simulation.view.active_run_idx != Some(run_index)
        || state
            .results
            .session
            .persistent_pane_context
            .is_some_and(|context| context.analysis == key.analysis())
    {
        return false;
    }
    let Some(dataset_default) = state.model.simulation.retained.runs[run_index]
        .analyses
        .get(analysis_index)
        .and_then(|analysis| analysis.waveforms.get(waveform_index))
        .map(|waveform| waveform.visible)
    else {
        return true;
    };
    let session = &mut state.results.session;
    if session.waveform_visibility(key, dataset_default) != plotted {
        // Put on, a signal the dataset hides is shown; taken off, one it
        // hides is unplotted, not hidden in its pane's legend, and one it
        // shows is hidden.
        let set = match (plotted, dataset_default) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            _ => None,
        };
        session.waveform_visibility.set(key.clone(), set);
        gesture.forgot |= !plotted && !dataset_default;
        if plotted {
            if gesture.recent.len() == 2 * RECENT_SIGNALS {
                gesture.recent.drain(..RECENT_SIGNALS);
            }
            gesture.recent.push(key);
        }
    } else if !plotted && !dataset_default && session.waveform_visibility.get(key) == Some(&false) {
        session.waveform_visibility.set(key.clone(), None);
        gesture.forgot = true;
    }
    if plotted && gesture.revealed.last() != Some(&key.analysis()) {
        gesture.revealed.push(key.analysis());
    }
    true
}

/// Where each signal of `keys` is: its run, analysis and column, found as
/// [`SourceWaveformPresentationKey::resolve`] finds one (a name two columns
/// share finds neither), in each analysis's index, or one walk of an analysis
/// still being run.
fn find_signals(
    runs: &[crate::state::SimulationRun],
    (indices, revision): (
        &rspice_results_ui::signal_index::SignalIndices,
        &crate::state::RunHistoryRevision,
    ),
    keys: &[&ResultBrowserSelectionKey],
) -> Vec<Option<(usize, usize, usize)>> {
    use std::collections::HashMap;
    let mut found = vec![None; keys.len()];
    // Each analysis' signals, with where `keys` holds each: a gesture's keys
    // come mostly one analysis after another.
    let mut wanted: Vec<(AnalysisPresentationKey, Vec<(usize, &str)>)> = Vec::new();
    for (at, key) in keys.iter().enumerate() {
        if let ResultBrowserSelectionKey::Waveform(signal) = key {
            let analysis = signal.analysis();
            let name = (at, signal.source_name());
            match wanted.iter_mut().rev().find(|(held, _)| *held == analysis) {
                Some((_, names)) => names.push(name),
                None => wanted.push((analysis, vec![name])),
            }
        }
    }
    for (analysis_key, asked) in wanted {
        let Some((run_index, run)) = runs
            .iter()
            .enumerate()
            .find(|(_, run)| run.dataset_id == analysis_key.dataset_id())
        else {
            continue;
        };
        let Some((analysis_index, analysis)) = analysis_key.resolve(&run.data) else {
            continue;
        };
        if let Some(index) = indices.index(revision, analysis_key, &analysis.data) {
            let names: Vec<&str> = asked.iter().map(|&(_, name)| name).collect();
            let columns = index.columns(&analysis.data, &names);
            for (&(at, _), column) in asked.iter().zip(columns) {
                found[at] = column.map(|column| (run_index, analysis_index, column));
            }
            continue;
        }
        let mut names = HashMap::<&str, Vec<usize>>::with_capacity(asked.len());
        for (at, name) in asked {
            names.entry(name).or_default().push(at);
        }
        let mut columns = HashMap::<&str, Option<usize>>::with_capacity(names.len());
        for (column, waveform) in analysis.waveforms.iter().enumerate() {
            if names.contains_key(waveform.name.as_str()) {
                columns
                    .entry(waveform.name.as_str())
                    .and_modify(|at| *at = None)
                    .or_insert(Some(column));
            }
        }
        for (name, indices) in names {
            if let Some(Some(column)) = columns.get(name) {
                for index in indices {
                    found[index] = Some((run_index, analysis_index, *column));
                }
            }
        }
    }
    found
}

/// The event nets a row plots, by their `D()` or `E()` column names: an
/// event net's own, or a declared bus's members MSB first. `None` for a row
/// that names neither.
fn event_row_nets(
    state: &ResultsParts<'_>,
    artifact: &ResultArtifactPresentationKey,
) -> Option<Vec<String>> {
    let name = artifact.canonical_name();
    if let Some(net) = name.strip_prefix(EVENT_NET_ROW_PREFIX) {
        return Some(vec![net.to_owned()]);
    }
    let label = name.strip_prefix(BUS_ROW_PREFIX)?;
    let (_, _, analysis) = artifact.resolve(&state.model.simulation.retained.runs)?;
    let Some(crate::state::AnalysisResultPayload::TransientEvents { digital_buses, .. }) =
        analysis.result_payload.as_ref()
    else {
        return None;
    };
    let bus = digital_buses
        .iter()
        .find(|bus| format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb) == label)?;
    Some(bus.members.iter().map(|net| format!("D({net})")).collect())
}

/// The active run's analysis and waveform that a signal of another run
/// overlays: the strip draws a run over the curves of the same name.
fn active_counterpart(
    state: &ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
) -> Option<(usize, usize, SourceWaveformPresentationKey, String, bool)> {
    let simulation = &state.model.simulation;
    let (run_index, analysis_index, _, waveform) = key.resolve(&simulation.retained.runs)?;
    let run = &simulation.retained.runs[run_index];
    let analysis: &rspice_results::analysis_result::AnalysisResult<
        rspice_results::waveform::RetainedTrace,
    > = run.analyses[analysis_index].as_ref();
    let active = simulation.active_run()?;
    active
        .analyses
        .iter()
        .enumerate()
        .find_map(|(index, candidate)| {
            let matched =
                rspice_results_ui::waves::matching_overlay_analysis(candidate.as_ref(), run)?;
            if !std::ptr::eq(matched, analysis) {
                return None;
            }
            let (waveform_index, counterpart) = candidate
                .waveforms
                .iter()
                .enumerate()
                .find(|(_, counterpart)| counterpart.name == waveform.name)?;
            Some((
                index,
                waveform_index,
                SourceWaveformPresentationKey::new(
                    AnalysisPresentationKey::new(active.dataset_id, candidate),
                    counterpart.name.clone(),
                ),
                counterpart.name.clone(),
                counterpart.visible,
            ))
        })
}

/// A signal of the active run is ticked while it is on the plot; one of
/// another run while the strip overlays it.
fn result_signal_ticked(state: &ResultsParts<'_>, key: &SourceWaveformPresentationKey) -> bool {
    let simulation = &state.model.simulation;
    let Some((run_index, _, _, waveform)) = key.resolve(&simulation.retained.runs) else {
        return false;
    };
    if simulation.view.active_run_idx == Some(run_index) {
        return waveform_plotted(
            state.model,
            state.results,
            key,
            &waveform.name,
            waveform.visible,
        );
    }
    simulation.is_dataset_overlaid(simulation.retained.runs[run_index].dataset_id)
        && active_counterpart(state, key).is_some_and(|(.., key, name, default)| {
            waveform_plotted(state.model, state.results, &key, &name, default)
        })
}

/// Put a signal on the plot or take it off, at once. A signal of another run
/// goes on over the active run's curve of its name, and comes off with its
/// run: the strip overlays whole runs.
pub(crate) fn set_result_signal_plotted(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
    plotted: bool,
) {
    let Some((run_index, analysis_index, waveform_index, _)) =
        key.resolve(&state.model.simulation.retained.runs)
    else {
        return;
    };
    set_found_signal_plotted(
        state,
        key,
        (run_index, analysis_index, waveform_index),
        plotted,
    );
}

/// [`set_result_signal_plotted`] for a signal found at `(run, analysis,
/// column)` of the retained runs.
fn set_found_signal_plotted(
    state: &mut ResultsParts<'_>,
    key: &SourceWaveformPresentationKey,
    (run_index, analysis_index, waveform_index): (usize, usize, usize),
    plotted: bool,
) {
    if state.model.simulation.view.active_run_idx == Some(run_index) {
        let ticked = state.model.simulation.retained.runs[run_index]
            .analyses
            .get(analysis_index)
            .and_then(|analysis| analysis.waveforms.get(waveform_index))
            .is_some_and(|waveform| {
                waveform_plotted(
                    state.model,
                    state.results,
                    key,
                    &waveform.name,
                    waveform.visible,
                )
            });
        if ticked != plotted {
            super::toggle_visibility(state, analysis_index, waveform_index);
        }
        if plotted {
            reveal_strip(state, key.analysis());
        } else if let Some(waveform) = state.model.simulation.retained.runs[run_index]
            .analyses
            .get(analysis_index)
            .and_then(|analysis| analysis.waveforms.get(waveform_index))
        {
            // Taken off the plot here, a signal is unplotted, not hidden in
            // its pane's legend.
            let dataset_default = waveform.visible;
            state.results.forget_waveform_hide(key, dataset_default);
        }
        return;
    }
    let dataset = state.model.simulation.retained.runs[run_index].dataset_id;
    let overlaid = state.model.simulation.is_dataset_overlaid(dataset);
    if !plotted {
        if overlaid {
            state.model.simulation.toggle_dataset_overlay(dataset);
        }
        return;
    }
    let Some((analysis, waveform, counterpart, name, default)) = active_counterpart(state, key)
    else {
        state
            .workbench
            .prompt("The active result has no curve of this name to overlay.");
        return;
    };
    if !overlaid {
        state.model.simulation.toggle_dataset_overlay(dataset);
    }
    if !waveform_plotted(state.model, state.results, &counterpart, &name, default) {
        super::toggle_visibility(state, analysis, waveform);
    }
    reveal_strip(state, counterpart.analysis());
}

/// A strip closed or crowded out by another's maximizing comes back for a
/// curve just put on it.
fn reveal_strip(state: &mut ResultsParts<'_>, analysis: AnalysisPresentationKey) {
    let session = &mut state.results.session;
    session.hidden_strips.remove(&analysis);
    if session
        .maximized_strip
        .is_some_and(|maximized| maximized != analysis)
    {
        session.maximized_strip = None;
    }
}
