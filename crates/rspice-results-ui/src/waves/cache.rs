//! Generation-bounded waveform projection cache over explicit presentation inputs.
use super::{FamilyTraceVisibilityKey, ProjectionOptions, StripModel, extent, stable_hash};
use crate::{selection::SourceWaveformPresentationKey, waveform::WaveformData};
use rspice_app_types::product::DatasetId;
use rspice_results::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    executed_deck::ExecutedDeckArchive, family_projection::SourceSampleSelection,
    result_presentation::ResultViewer, run::SimulationRun,
};
use rspice_ui_kit::tokens::Tokens;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Inputs not already covered by the host's retained-history invalidation.
pub struct ModelCacheInput<'a> {
    pub active_run_idx: Option<usize>,
    pub overlay_dataset_ids: &'a [DatasetId],
    pub viewer: ResultViewer,
    pub projection: ProjectionOptions<'a>,
    pub waveform_visibility: &'a HashMap<SourceWaveformPresentationKey, bool>,
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
    input.projection.complex_display.hash(&mut h);
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
pub fn apply_waveform_visibility<A: AsRef<AnalysisResult<WaveformData>>>(
    models: &mut [StripModel],
    active_run: Option<&SimulationRun<A>>,
    overrides: &HashMap<SourceWaveformPresentationKey, bool>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
) {
    let Some(run) = active_run else {
        return;
    };
    for model in models {
        let Some(analysis) = run.analyses.get(model.analysis_index).map(AsRef::as_ref) else {
            continue;
        };
        for trace in &mut model.traces {
            let Some(waveform) = analysis.waveforms.get(trace.waveform_index) else {
                trace.visible = false;
                continue;
            };
            let key = SourceWaveformPresentationKey::new(
                model.analysis_key,
                trace.source_waveform_name.clone(),
            );
            let source_visible = overrides.get(&key).copied().unwrap_or(waveform.visible);
            trace.visible = source_visible
                && trace
                    .family_visibility_key
                    .is_none_or(|key| !hidden_family_traces.contains(&key));
        }
    }
}
