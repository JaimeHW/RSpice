//! Generation-bounded waveform projection cache over explicit presentation inputs.
use super::{FamilyTraceVisibilityKey, ProjectionOptions, StripModel, extent, stable_hash};
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    executed_deck::ExecutedDeckArchive,
    family_projection::SourceSampleSelection,
    result_presentation::{ResultViewer, SourceWaveformPresentationKey},
    run::SimulationRun,
    waveform::RetainedTrace,
};
use rspice_ui_kit::tokens::Tokens;
use rspice_vocabulary::product::DatasetId;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Inputs not already covered by the host's retained-history invalidation.
pub struct ModelCacheInput<'a> {
    pub active_run_idx: Option<usize>,
    pub overlay_dataset_ids: &'a [DatasetId],
    pub viewer: ResultViewer,
    pub projection: ProjectionOptions<'a>,
    pub waveform_visibility: &'a HashMap<SourceWaveformPresentationKey, bool>,
    /// A digest of what routes the reader's expressions to panes: their
    /// text, visibility and declared units, and which failed to evaluate.
    pub expressions: u64,
    /// The X scales the reader chose, by strip.
    pub x_scales: &'a HashMap<
        rspice_results::result_presentation::AnalysisPresentationKey,
        rspice_ui_kit::plot::XScale,
    >,
}

/// Frame cache for the strip models. Building them clones every trace name
/// and walks all overlay runs, and both the center view and the right panel
/// ask for them each frame. Retained history is tracked by the host;
/// this cache also owns the executed-deck identity and presentation inputs.
#[derive(Default, Clone)]
pub struct ModelsCache {
    generation: u64,
    cached: Option<CachedModels>,
}

#[derive(Clone)]
struct CachedModels {
    fingerprint: u64,
    decks: ExecutedDeckArchive,
    models: Arc<Vec<StripModel>>,
}

impl ModelsCache {
    /// Which generation of strip models is currently held. Anything derived
    /// from the models keys on this rather than on the data version alone:
    /// hiding a trace changes the models without changing the dataset.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Keep the generation across presentation invalidation so dependent
    /// envelopes cannot mistake a rebuilt model for an earlier one.
    pub fn invalidate(&mut self) {
        self.cached = None;
    }

    /// The strip models last built, for a reader that has to agree with what
    /// the strip drew, such as a browser row taking its curve's colour,
    /// without building them itself.
    pub fn last_built(&self) -> Option<&[StripModel]> {
        self.cached.as_ref().map(|cached| cached.models.as_slice())
    }
}

impl std::fmt::Debug for ModelsCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelsCache(..)")
    }
}

fn models_fingerprint(input: &ModelCacheInput<'_>, t: &Tokens) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    input.active_run_idx.hash(&mut h);
    input.overlay_dataset_ids.hash(&mut h);
    input.viewer.hash(&mut h);
    input.projection.phase_continuous.hash(&mut h);
    input.expressions.hash(&mut h);
    let mut x_scales = input
        .x_scales
        .iter()
        .map(|(analysis, scale)| {
            (
                analysis.order_key(),
                *scale == rspice_ui_kit::plot::XScale::Log10,
            )
        })
        .collect::<Vec<_>>();
    x_scales.sort_unstable();
    x_scales.hash(&mut h);
    input.projection.complex_display.hash(&mut h);
    input.projection.ac.hash(&mut h);
    input
        .projection
        .selection
        .map(SourceSampleSelection::fingerprint)
        .hash(&mut h);
    let mut hidden = input
        .projection
        .hidden_family_traces
        .iter()
        .copied()
        .collect::<Vec<_>>();
    hidden.sort_unstable_by_key(stable_hash);
    hidden.hash(&mut h);
    let mut visibility = input
        .waveform_visibility
        .iter()
        .map(|(key, visible)| (stable_hash(key), *visible))
        .collect::<Vec<_>>();
    visibility.sort_unstable();
    visibility.hash(&mut h);
    for color in &t.color.traces {
        color.to_array().hash(&mut h);
    }
    h.finish()
}

impl ModelsCache {
    /// Reuse the current projection or lazily build one from host-selected sources.
    pub fn get_or_build(
        &mut self,
        input: ModelCacheInput<'_>,
        executed_decks: &ExecutedDeckArchive,
        tokens: &Tokens,
        envelopes: &mut extent::FamilyEnvelopeCache,
        build: impl FnOnce(
            ProjectionOptions<'_>,
            &HashMap<SourceWaveformPresentationKey, bool>,
        ) -> Vec<StripModel>,
    ) -> Arc<Vec<StripModel>> {
        let fp = models_fingerprint(&input, tokens);
        if let Some(cached) = &self.cached
            && cached.fingerprint == fp
            && cached.decks.shares_content_with(executed_decks)
        {
            return Arc::clone(&cached.models);
        }
        let mut built = build(input.projection, input.waveform_visibility);
        // The reader's X scale replaces the analysis' own, unless a
        // logarithmic axis cannot show the strip's domain.
        for model in &mut built {
            if let Some(scale) = input.x_scales.get(&model.analysis_key)
                && (*scale == rspice_ui_kit::plot::XScale::Linear
                    || super::stack::log_x_refusal(model).is_none())
            {
                model.x_scale = *scale;
            }
        }
        // Only now is it settled which traces the strip draws, so only now can
        // its shared X extent be resolved.
        extent::resolve_x_ranges(&mut built);
        built.retain(|model| match input.viewer {
            ResultViewer::DcSweep => model.analysis_type == AnalysisType::DcSweep,
            ResultViewer::Waves => model.analysis_type.is_time_domain(),
            ResultViewer::Bode => {
                model.analysis_type.is_bode_response()
                    || model.analysis_type.is_raw_frequency_curve()
            }
            ResultViewer::NoiseContrib => matches!(
                model.analysis_type,
                AnalysisType::Noise | AnalysisType::Hbnoise | AnalysisType::Qpnoise
            ),
            _ => true,
        });
        let models = Arc::new(built);
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 {
            *envelopes = extent::FamilyEnvelopeCache::default();
        }
        self.cached = Some(CachedModels {
            fingerprint: fp,
            decks: executed_decks.clone(),
            models: Arc::clone(&models),
        });
        models
    }
}

/// Apply quick-view presentation overrides after constructing the immutable
/// dataset projection. Family visibility remains the more specific gate, so
/// revealing a source never accidentally reveals a hidden family member.
///
/// With the reader's visibility known, each curve then takes its place in
/// the canvas `palette`'s cycle (see [`rank_palette_slots`]).
pub fn apply_waveform_visibility<A: AsRef<AnalysisResult<RetainedTrace>>>(
    models: &mut [StripModel],
    active_run: Option<&SimulationRun<A>>,
    overrides: &HashMap<SourceWaveformPresentationKey, bool>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
    palette: &[egui::Color32],
) {
    let Some(run) = active_run else {
        return;
    };
    for model in models {
        let Some(analysis) = run.analyses.get(model.analysis_index).map(AsRef::as_ref) else {
            continue;
        };
        let mut source_visible = Vec::with_capacity(model.traces.len());
        for trace in &mut model.traces {
            let Some(waveform) = analysis.waveforms.get(trace.waveform_index) else {
                trace.visible = false;
                source_visible.push(false);
                continue;
            };
            let key = SourceWaveformPresentationKey::new(
                model.analysis_key,
                trace.source_waveform_name.clone(),
            );
            let shown = overrides.get(&key).copied().unwrap_or(waveform.visible);
            source_visible.push(shown);
            trace.visible = shown
                && trace
                    .family_visibility_key
                    .is_none_or(|key| !hidden_family_traces.contains(&key));
        }
        let first_lane_slot = rank_palette_slots(model, analysis, &source_visible, palette);
        for lane in &mut model.lane_traces {
            let key = SourceWaveformPresentationKey::new(
                model.analysis_key,
                lane.source_waveform_name.clone(),
            );
            // A net with no column is off the plot until the reader plots it.
            let dataset_default = lane
                .waveform_index
                .and_then(|index| analysis.waveforms.get(index))
                .is_some_and(|waveform| waveform.visible);
            lane.visible = overrides.get(&key).copied().unwrap_or(dataset_default);
        }
        rank_lane_hues(model, analysis, first_lane_slot, palette);
    }
}

/// Give the lanes of `model` hues from the palette's cycle in turn after the
/// curves the strip shows: each real lane, which is a curve in its unit's
/// pane, and then the logic and bus rows ([`StripModel::lane_hues`]).
///
/// A lane's hue was its place among the analysis' columns, so whether it
/// repeated a plotted curve's hue was chance: on the SAR review strip EOC
/// drew in exactly the green of B3's curve. Ranked after the shown curves,
/// no lane shares a hue with one until the curves and lanes together
/// outnumber the palette.
/// As with curves, the real lanes the dataset plots rank first and keep that
/// rank whatever the reader hides, then the ones the reader added; a column
/// that stores a colour keeps it.
fn rank_lane_hues(
    model: &mut StripModel,
    analysis: &AnalysisResult<RetainedTrace>,
    first_slot: usize,
    palette: &[egui::Color32],
) {
    use super::LaneTraceKind;
    if palette.is_empty() {
        return;
    }
    let column = |lane: &super::LaneTrace| {
        lane.waveform_index
            .and_then(|index| analysis.waveforms.get(index))
    };
    let dataset = |lane: &super::LaneTrace| column(lane).is_some_and(|waveform| waveform.visible);
    let real = |lane: &super::LaneTrace| lane.kind == LaneTraceKind::Real;
    let plotted = model
        .lane_traces
        .iter()
        .filter(|lane| real(lane) && dataset(lane))
        .count();
    let added = model
        .lane_traces
        .iter()
        .filter(|lane| real(lane) && !dataset(lane) && lane.visible)
        .count();
    let (mut plotted_rank, mut added_rank, mut spare_rank) = (0, plotted, plotted + added);
    for lane in model.lane_traces.iter_mut().filter(|lane| real(lane)) {
        let counter = if dataset(lane) {
            &mut plotted_rank
        } else if lane.visible {
            &mut added_rank
        } else {
            &mut spare_rank
        };
        let slot = first_slot + *counter;
        *counter += 1;
        let stored = column(lane).map_or("", |waveform| waveform.color.as_str());
        lane.color = crate::presentation::trace_color(stored, palette[slot % palette.len()]);
    }
    let rows_from = first_slot + plotted + added;
    model.lane_hues = (0..palette.len())
        .map(|k| palette[(rows_from + k) % palette.len()])
        .collect();
}

/// Give each curve of `model` its place in the palette's cycle, and the hue
/// of that place unless its waveform stores a colour of its own.
///
/// The curves the dataset plots rank first, in column order, and keep that
/// rank whatever the reader hides, so hiding one recolours none of the
/// others. A curve the reader adds takes the next place after them, as a
/// waveform viewer gives a newly plotted signal the next free colour, so
/// the curves on the strip take distinct hues before any repeats one with a
/// dash. A real and an imaginary part rank apart; a family's members share
/// their source's place; an overlay run's curve takes the place of the
/// active run's curve it is merged with. `source_visible` is each trace's
/// source visibility with the reader's overrides applied. Returns the first
/// place after the shown curves'.
fn rank_palette_slots(
    model: &mut StripModel,
    analysis: &AnalysisResult<RetainedTrace>,
    source_visible: &[bool],
    palette: &[egui::Color32],
) -> usize {
    use super::TraceKind;
    if palette.is_empty() {
        return 0;
    }
    let ranked = |trace: &super::StripTrace| trace.palette_slot.is_some() && !trace.overlay;
    let mut sources: Vec<((usize, TraceKind), bool, bool)> = Vec::new();
    for (trace, shown) in model.traces.iter().zip(source_visible) {
        if !ranked(trace) {
            continue;
        }
        let key = (trace.waveform_index, trace.kind);
        let dataset = analysis
            .waveforms
            .get(trace.waveform_index)
            .is_some_and(|waveform| waveform.visible);
        match sources.iter_mut().find(|(source, ..)| *source == key) {
            Some((_, _, any_shown)) => *any_shown |= *shown,
            None => sources.push((key, dataset, *shown)),
        }
    }
    let plotted = sources.iter().filter(|(_, dataset, _)| *dataset).count();
    let added = sources
        .iter()
        .filter(|(_, dataset, shown)| !dataset && *shown)
        .count();
    let (mut plotted_rank, mut added_rank, mut spare_rank) = (0, plotted, plotted + added);
    let slots: Vec<((usize, TraceKind), usize)> = sources
        .iter()
        .map(|(key, dataset, shown)| {
            let counter = if *dataset {
                &mut plotted_rank
            } else if *shown {
                &mut added_rank
            } else {
                &mut spare_rank
            };
            *counter += 1;
            (*key, *counter - 1)
        })
        .collect();
    let mut merged = HashMap::new();
    for trace in model.traces.iter_mut().filter(|trace| ranked(trace)) {
        let key = (trace.waveform_index, trace.kind);
        let Some(&(_, slot)) = slots.iter().find(|(source, _)| *source == key) else {
            continue;
        };
        let hue = palette[slot % palette.len()];
        let stored = analysis
            .waveforms
            .get(trace.waveform_index)
            .filter(|_| trace.kind != TraceKind::Imaginary)
            .map_or("", |waveform| waveform.color.as_str());
        trace.palette_slot = Some(slot);
        trace.signal_color = crate::presentation::trace_color(stored, hue);
        trace.color = trace.family_style.map_or(trace.signal_color, |style| {
            super::projection::family_color(style, trace.signal_color)
        });
        merged.insert(
            (trace.source_waveform_name.clone(), trace.name.clone()),
            (slot, trace.signal_color, trace.color),
        );
    }
    for trace in model.traces.iter_mut().filter(|trace| trace.overlay) {
        if let Some(&(slot, signal_color, color)) =
            merged.get(&(trace.source_waveform_name.clone(), trace.name.clone()))
        {
            trace.palette_slot = Some(slot);
            trace.signal_color = signal_color;
            trace.color = color;
        }
    }
    plotted + added
}
