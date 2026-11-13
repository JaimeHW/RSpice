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
    if state.model.simulation.view.active_run_idx == Some(run_index) {
        if result_signal_ticked(state, key) != plotted {
            super::toggle_visibility(state, analysis_index, waveform_index);
        }
        if plotted {
            reveal_strip(state, key.analysis());
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
