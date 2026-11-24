//! Generation-bounded waveform projection cache over explicit presentation inputs.
use super::{FamilyTraceVisibilityKey, ProjectionOptions, StripModel, extent, stable_hash};
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    executed_deck::ExecutedDeckArchive,
    family_projection::SourceSampleSelection,
    result_presentation::{ResultViewer, SourceWaveformPresentationKey},
    run::SimulationRun,
    visualization_document::TraceColor,
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
    pub waveform_visibility: &'a crate::session::VisibilityOverrides,
    /// The colours the view gives its traces, by signal.
    pub waveform_colors: &'a HashMap<SourceWaveformPresentationKey, TraceColor>,
    /// The styles and names the view gives its traces, by signal.
    pub waveform_looks: &'a HashMap<SourceWaveformPresentationKey, crate::session::WaveformLook>,
    /// A digest of what routes the reader's expressions to panes: their
    /// text, visibility and declared units, and which failed to evaluate.
    pub expressions: u64,
    /// The X scales the reader chose, by strip.
    pub x_scales: &'a HashMap<
        rspice_results::result_presentation::AnalysisPresentationKey,
        rspice_ui_kit::plot::XScale,
    >,
    /// The strips the reader arranged each analysis into.
    pub strips: &'a HashMap<
        rspice_results::result_presentation::AnalysisPresentationKey,
        Vec<rspice_results::visualization_document::Strip>,
    >,
    /// The series derived from the run's, which a projection reads and adds to.
    pub derived: &'a mut crate::derived::DerivedSeries,
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
    /// The reader's ticks the models were built from, compared by revision:
    /// hashing a hundred thousand ticks every frame cost more than the frame.
    visibility: rspice_vocabulary::source_revision::SourceRevision,
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
    let mut strips = input
        .strips
        .iter()
        .map(|(analysis, strips)| (analysis.order_key(), format!("{strips:?}")))
        .collect::<Vec<_>>();
    strips.sort_unstable();
    strips.hash(&mut h);
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
    let mut colors = input
        .waveform_colors
        .iter()
        .map(|(key, color)| (stable_hash(key), color.rgb()))
        .collect::<Vec<_>>();
    colors.sort_unstable();
    colors.hash(&mut h);
    let mut looks = input
        .waveform_looks
        .iter()
        .map(|(key, look)| (stable_hash(key), stable_hash(look)))
        .collect::<Vec<_>>();
    looks.sort_unstable();
    looks.hash(&mut h);
    for color in &t.color.traces {
        color.to_array().hash(&mut h);
    }
    h.finish()
}

/// How the host makes the strip models: `build` projects every strip
/// afresh; `retick` changes the strips built before by the reader's ticks of
/// the signals edited since, answering which strips it changed, or `None`
/// when only a build follows the edits (see [`super::retick`]).
pub struct ModelsMaker<B, R> {
    pub build: B,
    pub retick: R,
}

impl ModelsCache {
    /// Reuse the current projection, change it by the reader's ticks edited
    /// since it was built, or build one from host-selected sources.
    pub fn get_or_build<B, R>(
        &mut self,
        mut input: ModelCacheInput<'_>,
        executed_decks: &ExecutedDeckArchive,
        tokens: &Tokens,
        envelopes: &mut extent::FamilyEnvelopeCache,
        maker: ModelsMaker<B, R>,
    ) -> Arc<Vec<StripModel>>
    where
        B: FnOnce(
            &mut crate::derived::DerivedSeries,
            ProjectionOptions<'_>,
            &HashMap<SourceWaveformPresentationKey, bool>,
        ) -> Vec<StripModel>,
        R: FnOnce(
            &mut crate::derived::DerivedSeries,
            &mut [StripModel],
            &[&SourceWaveformPresentationKey],
            ProjectionOptions<'_>,
        ) -> Option<Vec<usize>>,
    {
        let fp = models_fingerprint(&input, tokens);
        let visibility = input.waveform_visibility.revision();
        if let Some(cached) = self.cached.take_if(|cached| {
            cached.fingerprint == fp && cached.decks.shares_content_with(executed_decks)
        }) {
            if cached.visibility == visibility {
                let models = Arc::clone(&cached.models);
                self.cached = Some(cached);
                return models;
            }
            if let Some(models) = self.follow_ticks(cached, &mut input, maker.retick, envelopes) {
                return models;
            }
        }
        let mut built = (maker.build)(input.derived, input.projection, input.waveform_visibility);
        // The reader's X scale replaces the analysis' own, unless a
        // logarithmic axis cannot show the strip's domain.
        for model in &mut built {
            model.x_bounds = super::stack::visible_x_bounds(model);
            if let Some(scale) = input.x_scales.get(&model.analysis_key)
                && (*scale == rspice_ui_kit::plot::XScale::Linear
                    || super::stack::log_x_refusal(model).is_none())
            {
                model.x_scale = *scale;
            }
            if let Some(strips) = input.strips.get(&model.analysis_key) {
                model.arrangement.clone_from(strips);
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
            ResultViewer::SParameters => model.analysis_type.is_network_response(),
            ResultViewer::NoiseContrib => match model.analysis_type {
                AnalysisType::Noise | AnalysisType::Hbnoise | AnalysisType::Qpnoise => true,
                // A periodic noise run referred to the carrier's phase keeps
                // decibels, and is the Phase Noise sheet's.
                AnalysisType::Pnoise => model
                    .traces
                    .iter()
                    .any(|trace| trace.kind == super::TraceKind::NoiseDensity),
                _ => false,
            },
            _ => true,
        });
        let models = Arc::new(built);
        self.next_generation(envelopes);
        self.cached = Some(CachedModels {
            fingerprint: fp,
            visibility,
            decks: executed_decks.clone(),
            models: Arc::clone(&models),
        });
        models
    }

    /// The strips of `cached`, changed by the reader's ticks edited since
    /// they were built: a tick changes the strip of its signal by the curves
    /// it adds or takes away, and leaves every other strip as it was. `None`
    /// when only a build follows: the edits are no longer known, a strip is
    /// shared with a reader off the frame, or an edited strip is one the
    /// reader arranged or put on a logarithmic X, which its curves decide.
    fn follow_ticks<'a>(
        &mut self,
        cached: CachedModels,
        input: &mut ModelCacheInput<'a>,
        retick: impl FnOnce(
            &mut crate::derived::DerivedSeries,
            &mut [StripModel],
            &[&SourceWaveformPresentationKey],
            ProjectionOptions<'_>,
        ) -> Option<Vec<usize>>,
        envelopes: &mut extent::FamilyEnvelopeCache,
    ) -> Option<Arc<Vec<StripModel>>> {
        let ticks: &'a crate::session::VisibilityOverrides = input.waveform_visibility;
        let edited: Vec<&SourceWaveformPresentationKey> =
            ticks.edits_since(&cached.visibility)?.collect();
        if edited.iter().any(|key| {
            input.strips.contains_key(&key.analysis())
                || input.x_scales.get(&key.analysis()) == Some(&rspice_ui_kit::plot::XScale::Log10)
        }) {
            return None;
        }
        let mut models = Arc::try_unwrap(cached.models).ok()?;
        let touched = retick(&mut *input.derived, &mut models, &edited, input.projection)?;
        for &place in &touched {
            let model = &mut models[place];
            model.x_bounds = super::stack::visible_x_bounds(model);
            extent::resolve_x_ranges(std::slice::from_mut(model));
        }
        let models = Arc::new(models);
        self.next_generation(envelopes);
        self.cached = Some(CachedModels {
            fingerprint: cached.fingerprint,
            visibility: ticks.revision(),
            decks: cached.decks,
            models: Arc::clone(&models),
        });
        Some(models)
    }

    /// Count a new generation of strip models.
    fn next_generation(&mut self, envelopes: &mut extent::FamilyEnvelopeCache) {
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 {
            *envelopes = extent::FamilyEnvelopeCache::default();
        }
    }
}

/// Apply quick-view presentation overrides after constructing the immutable
/// dataset projection. Family visibility remains the more specific gate, so
/// revealing a source never accidentally reveals a hidden family member.
///
/// With the reader's visibility known, each curve then takes its place in
/// the canvas `palette`'s cycle (see [`rank_palette_slots`]), and the colour
/// the view gives it in `colors`, when it gives one.
pub fn apply_waveform_visibility<A: AsRef<AnalysisResult<RetainedTrace>>>(
    models: &mut [StripModel],
    active_run: Option<&SimulationRun<A>>,
    overrides: &HashMap<SourceWaveformPresentationKey, bool>,
    colors: &HashMap<SourceWaveformPresentationKey, TraceColor>,
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
        // The model holds the curves of the signals on the plot alone, so a
        // curve is drawn unless the reader hid its member of a family.
        let mut source_visible = Vec::with_capacity(model.traces.len());
        for trace in &mut model.traces {
            let shown = analysis.waveforms.get(trace.waveform_index).is_some();
            source_visible.push(shown);
            let family_shown = trace
                .family_visibility_key
                .is_none_or(|key| !hidden_family_traces.contains(&key));
            trace.visible = shown && family_shown;
        }
        model.forget_trace_panes();
        let first_lane_slot = rank_palette_slots(model, analysis, &source_visible, colors, palette);
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
        rank_lane_hues(model, analysis, first_lane_slot, colors, palette);
    }
}

/// Give each curve of `models` the style and name the view gives its
/// signal in `looks`. A family member keeps its member's tag after the name.
pub fn apply_waveform_looks(
    models: &mut [StripModel],
    looks: &HashMap<SourceWaveformPresentationKey, crate::session::WaveformLook>,
) {
    if looks.is_empty() {
        return;
    }
    for model in models {
        for trace in &mut model.traces {
            give_look(trace, model.analysis_key, looks);
        }
    }
}

/// Give `trace`, a curve of the analysis `analysis`, the style and name the
/// view gives its signal in `looks`.
pub(crate) fn give_look(
    trace: &mut super::StripTrace,
    analysis: rspice_results::result_presentation::AnalysisPresentationKey,
    looks: &HashMap<SourceWaveformPresentationKey, crate::session::WaveformLook>,
) {
    if looks.is_empty() {
        return;
    }
    let Some(look) = looks.get(&SourceWaveformPresentationKey::new(
        analysis,
        trace.source_waveform_name.clone(),
    )) else {
        return;
    };
    trace.style = look.style;
    if let Some(name) = look.name.as_deref() {
        trace.name = match trace.name.strip_prefix(trace.base_name.as_str()) {
            Some(tag) if !tag.is_empty() => format!("{name}{tag}"),
            _ => name.to_owned(),
        };
    }
}

/// The colour a curve or real lane of `analysis` reading `signal` draws in:
/// the one the view gives it, else the one its run's column stores, else
/// `hue`, its palette place.
pub(crate) fn drawn_color(
    colors: &HashMap<SourceWaveformPresentationKey, TraceColor>,
    analysis: rspice_results::result_presentation::AnalysisPresentationKey,
    signal: &str,
    stored: &str,
    hue: egui::Color32,
) -> egui::Color32 {
    colors
        .get(&SourceWaveformPresentationKey::new(analysis, signal))
        .map_or_else(
            || crate::presentation::trace_color(stored, hue),
            |color| crate::presentation::chosen_trace_color(*color),
        )
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
/// that stores a colour keeps it, and a colour the view gives one wins.
pub(crate) fn rank_lane_hues(
    model: &mut StripModel,
    analysis: &AnalysisResult<RetainedTrace>,
    first_slot: usize,
    colors: &HashMap<SourceWaveformPresentationKey, TraceColor>,
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
    let analysis_key = model.analysis_key;
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
        lane.color = drawn_color(
            colors,
            analysis_key,
            &lane.source_waveform_name,
            stored,
            palette[slot % palette.len()],
        );
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
    colors: &HashMap<SourceWaveformPresentationKey, TraceColor>,
    palette: &[egui::Color32],
) -> usize {
    use super::TraceKind;
    if palette.is_empty() {
        return 0;
    }
    let ranked = |trace: &super::StripTrace| trace.palette_slot.is_some() && !trace.overlay;
    // Each source once, in the order its first curve comes, found by its key:
    // a strip can draw a million curves, so none is found by searching the
    // others.
    let mut sources: Vec<((usize, TraceKind), bool, bool)> = Vec::new();
    let mut source_at = HashMap::<(usize, TraceKind), usize>::new();
    for (trace, shown) in model.traces.iter().zip(source_visible) {
        if !ranked(trace) {
            continue;
        }
        let key = (trace.waveform_index, trace.kind);
        let dataset = analysis
            .waveforms
            .get(trace.waveform_index)
            .is_some_and(|waveform| waveform.visible);
        match source_at.entry(key) {
            std::collections::hash_map::Entry::Occupied(at) => sources[*at.get()].2 |= *shown,
            std::collections::hash_map::Entry::Vacant(at) => {
                at.insert(sources.len());
                sources.push((key, dataset, *shown));
            }
        }
    }
    // The curves the dataset plots keep the places they were counted to as
    // the model was built, the ones the reader took off the plot among them.
    let plotted = model.dataset_plotted;
    let added = sources
        .iter()
        .filter(|(key, dataset, shown)| {
            !(*dataset && model.dataset_ranks.contains_key(key)) && *shown
        })
        .count();
    let (mut added_rank, mut spare_rank) = (plotted, plotted + added);
    let slots: HashMap<(usize, TraceKind), usize> = sources
        .iter()
        .map(|(key, dataset, shown)| {
            if *dataset && let Some(&rank) = model.dataset_ranks.get(key) {
                return (*key, rank);
            }
            let counter = if *shown {
                &mut added_rank
            } else {
                &mut spare_rank
            };
            *counter += 1;
            (*key, *counter - 1)
        })
        .collect();
    let mut merged = HashMap::new();
    let analysis_key = model.analysis_key;
    for trace in model.traces.iter_mut().filter(|trace| ranked(trace)) {
        let key = (trace.waveform_index, trace.kind);
        let Some(&slot) = slots.get(&key) else {
            continue;
        };
        let hue = palette[slot % palette.len()];
        let stored = analysis
            .waveforms
            .get(trace.waveform_index)
            .filter(|_| trace.kind != TraceKind::Imaginary)
            .map_or("", |waveform| waveform.color.as_str());
        trace.palette_slot = Some(slot);
        // An imaginary part keeps its own palette place, as it does against
        // a colour the run stores.
        trace.signal_color = if trace.kind == TraceKind::Imaginary {
            hue
        } else {
            drawn_color(
                colors,
                analysis_key,
                &trace.source_waveform_name,
                stored,
                hue,
            )
        };
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
