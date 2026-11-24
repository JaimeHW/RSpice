//! A tick changes the strip of its signal by the curve it puts on the plot
//! or takes off, leaves every other strip as it was, and draws what a build
//! from the same ticks draws.

use super::*;
use crate::session::{VisibilityOverrides, WaveformLook};
use crate::signal_index::SignalIndices;
use crate::waves::cache::{ModelCacheInput, ModelsCache, ModelsMaker, apply_waveform_visibility};
use crate::waves::extent::FamilyEnvelopeCache;
use rspice_results::result_presentation::{AnalysisPresentationKey, ResultViewer};
use rspice_results::run::ExecutionTarget;
use rspice_results::visualization_document::TraceColor;
use rspice_vocabulary::source_revision::SourceRevision;

type Run = SimulationRun<AnalysisResult<RetainedTrace>>;

/// A run of two transients of `signals` node voltages each, the dataset
/// plotting the first of each, with no colour of their own.
fn two_transients(signals: usize) -> Run {
    let time = vec![0.0, 1.0e-9, 2.0e-9];
    let analysis = |id| {
        let waveforms = (0..signals)
            .map(|index| {
                let mut trace = RetainedTrace::new(
                    format!("V(n{index})"),
                    time.clone(),
                    vec![index as f64; 3],
                    "",
                )
                .with_unit("V");
                trace.visible = index == 0;
                trace
            })
            .collect();
        AnalysisResult::new(id, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(waveforms)
    };
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses.push(analysis(1));
    run.analyses.push(analysis(2));
    run
}

/// What the host keeps between frames: the strip models and what they are
/// made from, and how many times they were built afresh.
struct Host<'r> {
    run: &'r Run,
    cache: ModelsCache,
    derived: DerivedSeries,
    decks: ExecutedDeckArchive,
    indices: SignalIndices,
    revision: SourceRevision,
    envelopes: FamilyEnvelopeCache,
    colors: HashMap<SourceWaveformPresentationKey, TraceColor>,
    looks: HashMap<SourceWaveformPresentationKey, WaveformLook>,
    builds: usize,
}

impl<'r> Host<'r> {
    fn new(run: &'r Run) -> Self {
        Self {
            run,
            cache: ModelsCache::default(),
            derived: DerivedSeries::default(),
            decks: ExecutedDeckArchive::default(),
            indices: SignalIndices::default(),
            revision: SourceRevision::default(),
            envelopes: FamilyEnvelopeCache::default(),
            colors: HashMap::new(),
            looks: HashMap::new(),
            builds: 0,
        }
    }

    /// The strip models of a frame with `ticks`, as the host asks for them.
    fn models(&mut self, ticks: &VisibilityOverrides) -> Arc<Vec<StripModel>> {
        let tokens = Tokens::default();
        let hidden = HashSet::new();
        let (x_scales, strips) = (HashMap::new(), HashMap::new());
        let run = self.run;
        let (colors, looks) = (&self.colors, &self.looks);
        let builds = &mut self.builds;
        let decks = &self.decks;
        self.cache.get_or_build(
            ModelCacheInput {
                active_run_idx: Some(0),
                overlay_dataset_ids: &[],
                viewer: ResultViewer::Waves,
                projection: options(ticks, &hidden, &self.indices, &self.revision),
                waveform_visibility: ticks,
                waveform_colors: colors,
                waveform_looks: looks,
                expressions: 0,
                x_scales: &x_scales,
                strips: &strips,
                derived: &mut self.derived,
            },
            decks,
            &tokens,
            &mut self.envelopes,
            ModelsMaker {
                build:
                    |derived: &mut DerivedSeries,
                     projection: ProjectionOptions<'_>,
                     visibility: &HashMap<SourceWaveformPresentationKey, bool>| {
                        *builds += 1;
                        built(run, decks, derived, projection, visibility, colors)
                    },
                retick: |derived: &mut DerivedSeries,
                         models: &mut [StripModel],
                         edited: &[&SourceWaveformPresentationKey],
                         projection: ProjectionOptions<'_>| {
                    super::retick(
                        models,
                        edited,
                        derived,
                        &Retick {
                            run,
                            options: projection,
                            colors,
                            looks,
                            palette: &tokens.color.traces,
                        },
                    )
                },
            },
        )
    }

    /// The strip models a build from `ticks` makes, for comparison.
    fn built_afresh(&self, ticks: &VisibilityOverrides) -> Vec<StripModel> {
        let hidden = HashSet::new();
        let mut built = built(
            self.run,
            &self.decks,
            &mut DerivedSeries::default(),
            options(ticks, &hidden, &self.indices, &self.revision),
            ticks,
            &self.colors,
        );
        for model in &mut built {
            model.x_bounds = crate::waves::stack::visible_x_bounds(model);
        }
        crate::waves::extent::resolve_x_ranges(&mut built);
        built
    }
}

fn options<'a>(
    ticks: &'a VisibilityOverrides,
    hidden: &'a HashSet<FamilyTraceVisibilityKey>,
    indices: &'a SignalIndices,
    revision: &'a SourceRevision,
) -> ProjectionOptions<'a> {
    ProjectionOptions {
        phase_continuous: false,
        complex_display: Default::default(),
        ac: Default::default(),
        selection: None,
        hidden_family_traces: hidden,
        waveform_visibility: ticks,
        signals: Some(SignalLookup { indices, revision }),
    }
}

/// The strip models a build of `run` makes, as the host builds them.
fn built(
    run: &Run,
    decks: &ExecutedDeckArchive,
    derived: &mut DerivedSeries,
    projection: ProjectionOptions<'_>,
    visibility: &HashMap<SourceWaveformPresentationKey, bool>,
    colors: &HashMap<SourceWaveformPresentationKey, TraceColor>,
) -> Vec<StripModel> {
    let tokens = Tokens::default();
    let mut built =
        super::super::build_models(&[run], decks, derived, &tokens, projection, |_| None);
    apply_waveform_visibility(
        &mut built,
        Some(run),
        visibility,
        colors,
        projection.hidden_family_traces,
        &tokens.color.traces,
    );
    built
}

/// What a strip draws: each curve's signal, palette place, colour and
/// whether it is drawn, and its X extent.
fn drawn(model: &StripModel) -> (Vec<(String, Option<usize>, egui::Color32, bool)>, String) {
    let curves = model
        .traces
        .iter()
        .map(|trace| {
            (
                trace.source_waveform_name.clone(),
                trace.palette_slot,
                trace.color,
                trace.visible,
            )
        })
        .collect();
    (curves, format!("{:?}", model.x_bounds))
}

fn strip(models: &[StripModel], analysis: AnalysisPresentationKey) -> &StripModel {
    models
        .iter()
        .find(|model| model.analysis_key == analysis)
        .expect("the analysis has a strip")
}

#[test]
fn a_tick_in_one_strip_leaves_the_other_strip_as_it_was() {
    let run = two_transients(1_000);
    let first = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let second = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1]);
    let signal = |analysis, name: &str| SourceWaveformPresentationKey::new(analysis, name);
    let mut host = Host::new(&run);
    let mut ticks = VisibilityOverrides::default();
    ticks.insert(signal(first, "V(n5)"), true);
    let (untouched, before) = {
        let models = host.models(&ticks);
        (
            strip(&models, first).revision(),
            strip(&models, second).revision(),
        )
    };
    assert_eq!(host.builds, 1);

    // Ticks in the second strip: one signal on the plot, the dataset's own off it.
    ticks.insert(signal(second, "V(n742)"), true);
    ticks.insert(signal(second, "V(n7)"), true);
    ticks.insert(signal(second, "V(n0)"), false);
    let models = host.models(&ticks);
    assert_eq!(host.builds, 1, "a tick builds no strip afresh");
    assert_eq!(
        strip(&models, first).revision(),
        untouched,
        "the strip no tick touched is the one built before"
    );
    assert_ne!(strip(&models, second).revision(), before);
    let afresh = host.built_afresh(&ticks);
    for analysis in [first, second] {
        assert_eq!(
            drawn(strip(&models, analysis)),
            drawn(strip(&afresh, analysis)),
            "a strip changed by ticks draws what a build from them draws"
        );
    }
    let names: Vec<_> = strip(&models, second)
        .traces
        .iter()
        .map(|trace| trace.source_waveform_name.as_str())
        .collect();
    assert_eq!(names, ["V(n7)", "V(n742)"]);
    drop(models);

    // Taking a curve off and putting the dataset's own back on, the same.
    ticks.remove(&signal(second, "V(n7)"));
    ticks.remove(&signal(second, "V(n0)"));
    let models = host.models(&ticks);
    assert_eq!(host.builds, 1, "an untick builds no strip afresh");
    assert_eq!(strip(&models, first).revision(), untouched);
    let afresh = host.built_afresh(&ticks);
    assert_eq!(drawn(strip(&models, second)), drawn(strip(&afresh, second)));
}

#[test]
fn ticks_edited_past_what_is_known_build_the_strips_afresh() {
    let run = two_transients(10);
    let first = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let mut host = Host::new(&run);
    let mut ticks = VisibilityOverrides::default();
    drop(host.models(&ticks));
    // A mutable borrow of the ticks does not say which it edits.
    std::ops::DerefMut::deref_mut(&mut ticks)
        .insert(SourceWaveformPresentationKey::new(first, "V(n3)"), true);
    let models = host.models(&ticks);
    assert_eq!(host.builds, 2, "edits no longer known are built from");
    let names: Vec<_> = strip(&models, first)
        .traces
        .iter()
        .map(|trace| trace.source_waveform_name.as_str())
        .collect();
    assert_eq!(names, ["V(n0)", "V(n3)"]);
}
