//! A strip built from the reader's ticks, changed by an edit of them.

use super::{
    AnalysisPresentationKey, ProjectionContext, ProjectionOptions, StripModel, StripTrace,
    append_column_trace, column_kind,
};
use crate::derived::DerivedSeries;
use crate::session::WaveformLook;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::result_presentation::SourceWaveformPresentationKey;
use rspice_results::run::SimulationRun;
use rspice_results::visualization_document::TraceColor;
use rspice_results::waveform::RetainedTrace;
use std::collections::{HashMap, HashSet};

/// What following an edit of the reader's ticks reads besides the strips.
pub struct Retick<'a, A> {
    /// The run the strips draw.
    pub run: &'a SimulationRun<A>,
    pub options: ProjectionOptions<'a>,
    /// The colours the view gives its traces, by signal.
    pub colors: &'a HashMap<SourceWaveformPresentationKey, TraceColor>,
    /// The styles and names the view gives its traces, by signal.
    pub looks: &'a HashMap<SourceWaveformPresentationKey, WaveformLook>,
    /// The canvas palette the curves take their places in.
    pub palette: &'a [egui::Color32],
}

/// Change `models`, built from the reader's ticks, by the ticks of the
/// signals `edited` since: a strip gains the curves of the signals put on
/// the plot and loses those taken off it, keeps every other curve as it was,
/// and gives its curves their places in the palette again. A strip no edit
/// adds a curve to or takes one from is left as it was.
///
/// The strips changed, by their places in `models`; `None`, with `models`
/// as they were, when an edit touches a strip only a build follows: one of
/// families, event lanes, complex parts or overlaid runs, or of an analysis
/// no strip shows.
pub fn retick<A: AsRef<AnalysisResult<RetainedTrace>>>(
    models: &mut [StripModel],
    edited: &[&SourceWaveformPresentationKey],
    derived: &mut DerivedSeries,
    with: &Retick<'_, A>,
) -> Option<Vec<usize>> {
    let lookup = with.options.signals?;
    // The edited signals of each strip's analysis, by the strip's place.
    let mut edits: HashMap<usize, Vec<&SourceWaveformPresentationKey>> = HashMap::new();
    for &key in edited {
        if key.analysis().dataset_id() != with.run.dataset_id {
            // Another run's ticks: the strips draw this run's.
            continue;
        }
        let place = models
            .iter()
            .position(|model| model.analysis_key == key.analysis())?;
        if !models[place].retickable {
            return None;
        }
        edits.entry(place).or_default().push(key);
    }
    // What each edit adds and takes away, worked out before any strip is
    // changed, so a refusal leaves every strip as it was.
    let mut changes = Vec::new();
    for (place, keys) in edits {
        let model = &models[place];
        let analysis = with.run.analyses.get(model.analysis_index)?.as_ref();
        let index = lookup
            .indices
            .index(lookup.revision, model.analysis_key, analysis)?;
        let mut added = Vec::new();
        let mut taken = HashSet::new();
        // The signals' columns found together; a name no column or more
        // than one has is looked for column by column.
        let names: Vec<&str> = keys.iter().map(|key| key.source_name()).collect();
        let found = index.columns(analysis, &names);
        for (key, found) in keys.into_iter().zip(found) {
            let mut follow = |column: usize| {
                let waveform = &analysis.waveforms[column];
                let shown = with
                    .options
                    .waveform_visibility
                    .get(key)
                    .copied()
                    .unwrap_or(waveform.visible);
                let held = model
                    .traces
                    .binary_search_by_key(&column, |trace| trace.waveform_index)
                    .is_ok();
                if shown && !held {
                    added.push(column);
                } else if !shown && held {
                    taken.insert(column);
                }
            };
            match found {
                Some(column) => follow(column),
                None => index
                    .columns_named(analysis, key.source_name())
                    .for_each(follow),
            }
        }
        if added.is_empty() && taken.is_empty() {
            continue;
        }
        added.sort_unstable();
        added.dedup();
        let context = ProjectionContext {
            analysis_key: model.analysis_key,
            run_id: with.run.id,
            selection_key: 0,
            sample_selection: None,
            hidden_family_traces: with.options.hidden_family_traces,
            phase_continuous: with.options.phase_continuous,
        };
        let mut fresh = Vec::with_capacity(added.len());
        for column in added {
            let kind = column_kind(analysis, &analysis.waveforms[column], with.options.ac);
            let before = fresh.len();
            // Placed in the palette once the strip is ranked below.
            append_column_trace(
                &mut fresh,
                derived,
                &context,
                analysis,
                (column, kind),
                (egui::Color32::TRANSPARENT, None),
            );
            if fresh.len() != before + 1 {
                // A column drawn as other than one curve is placed in the
                // palette by a build, which counts its curves by source.
                return None;
            }
        }
        changes.push((place, fresh, taken));
    }
    derived.forget_build_axes();
    let mut touched = Vec::with_capacity(changes.len());
    for (place, fresh, taken) in changes {
        let model = &mut models[place];
        let analysis = with.run.analyses[model.analysis_index].as_ref();
        if !taken.is_empty() {
            model
                .traces
                .retain(|trace| !taken.contains(&trace.waveform_index));
        }
        merge_by_column(&mut model.traces, fresh);
        model.signal_trace_count = model.traces.len();
        let first_lane_slot = rank_curves(model, analysis, with);
        crate::waves::cache::rank_lane_hues(
            model,
            analysis,
            first_lane_slot,
            with.colors,
            with.palette,
        );
        model.forget_trace_panes();
        model.x_bounds = None;
        model.grid_ascending = crate::waves::grid_is_ascending(model);
        touched.push(place);
    }
    touched.sort_unstable();
    Some(touched)
}

/// Merge `fresh`, in column order, into `traces`, in column order.
fn merge_by_column(traces: &mut Vec<StripTrace>, fresh: Vec<StripTrace>) {
    if fresh.is_empty() {
        return;
    }
    if traces
        .last()
        .is_none_or(|last| last.waveform_index < fresh[0].waveform_index)
    {
        traces.extend(fresh);
        return;
    }
    let held = std::mem::take(traces);
    traces.reserve(held.len() + fresh.len());
    let mut held = held.into_iter().peekable();
    let mut fresh = fresh.into_iter().peekable();
    loop {
        let next = match (held.peek(), fresh.peek()) {
            (Some(a), Some(b)) if a.waveform_index <= b.waveform_index => held.next(),
            (Some(_), Some(_)) | (None, Some(_)) => fresh.next(),
            (Some(_), None) => held.next(),
            (None, None) => break,
        };
        traces.extend(next);
    }
}

/// Give each curve of `model` its place in the palette's cycle as a build
/// does: a curve the dataset plots keeps its rank among them, and the others
/// follow in column order. Only a curve whose place moved, or that was just
/// added, takes its colour again. Returns the first place after the curves'.
fn rank_curves<A>(
    model: &mut StripModel,
    analysis: &AnalysisResult<RetainedTrace>,
    with: &Retick<'_, A>,
) -> usize {
    let mut added = model.dataset_plotted;
    if with.palette.is_empty() {
        return 0;
    }
    let analysis_key: AnalysisPresentationKey = model.analysis_key;
    let ranks = &model.dataset_ranks;
    for trace in &mut model.traces {
        let column = analysis.waveforms.get(trace.waveform_index);
        let dataset = column
            .is_some_and(|waveform| waveform.visible)
            .then(|| ranks.get(&(trace.waveform_index, trace.kind)).copied())
            .flatten();
        let slot = dataset.unwrap_or_else(|| {
            added += 1;
            added - 1
        });
        if trace.palette_slot == Some(slot) {
            continue;
        }
        let fresh = trace.palette_slot.is_none();
        trace.palette_slot = Some(slot);
        trace.signal_color = crate::waves::cache::drawn_color(
            with.colors,
            analysis_key,
            &trace.source_waveform_name,
            column.map_or("", |waveform| waveform.color.as_str()),
            with.palette[slot % with.palette.len()],
        );
        trace.color = trace.signal_color;
        if fresh {
            crate::waves::cache::give_look(trace, analysis_key, with.looks);
        }
    }
    added
}
