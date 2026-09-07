//! Cached waveform projections, keyed by retained sources and presentation.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::super::ResultViewer;
use super::{
    AnalysisType, ComplexNumberDisplay, FamilyTraceVisibilityKey, ResultsState, SimulationState,
    SourceSampleSelection, SourceWaveformPresentationKey, StripModel, Tokens,
    apply_waveform_visibility, build_models, extent, stable_hash,
};

/// Frame cache for the strip models. Building them clones every trace name
/// and walks all overlay runs, and both the center view and the right panel
/// ask for them each frame. Retained history is tracked by ResultsState;
/// this cache also owns the executed-deck identity and presentation inputs.
#[derive(Default, Clone)]
pub(in crate::workbench::documents::result_document) struct ModelsCache {
    generation: u64,
    cached: Option<CachedModels>,
}

#[derive(Clone)]
struct CachedModels {
    fingerprint: u64,
    decks: crate::state::ExecutedDeckArchive,
    models: Arc<Vec<StripModel>>,
}

impl ModelsCache {
    /// Which generation of strip models is currently held. Anything derived
    /// from the models keys on this rather than on the data version alone:
    /// hiding a trace changes the models without changing the dataset.
    pub(in crate::workbench::documents::result_document) fn generation(&self) -> u64 {
        self.generation
    }

    /// Keep the generation across presentation invalidation so dependent
    /// envelopes cannot mistake a rebuilt model for an earlier one.
    pub(in crate::workbench::documents::result_document) fn invalidate(&mut self) {
        self.cached = None;
    }
}

impl std::fmt::Debug for ModelsCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelsCache(..)")
    }
}

/// Presentation inputs not owned by retained history or executed decks.
/// Metadata and sample changes invalidate through the retained history owner,
/// so unchanged frames need not walk every analysis and waveform descriptor.
fn models_fingerprint(
    simulation: &SimulationState,
    viewer: ResultViewer,
    phase_continuous: bool,
    complex_display: ComplexNumberDisplay,
    selection: Option<&SourceSampleSelection>,
    hidden_family_traces: &HashSet<FamilyTraceVisibilityKey>,
    waveform_visibility: &HashMap<SourceWaveformPresentationKey, bool>,
    t: &Tokens,
) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    simulation.active_run_idx.hash(&mut h);
    simulation.overlay_dataset_ids.hash(&mut h);
    viewer.hash(&mut h);
    phase_continuous.hash(&mut h);
    complex_display.hash(&mut h);
    selection
        .map(SourceSampleSelection::fingerprint)
        .hash(&mut h);
    let mut hidden = hidden_family_traces.iter().copied().collect::<Vec<_>>();
    hidden.sort_unstable_by_key(stable_hash);
    hidden.hash(&mut h);
    let mut visibility = waveform_visibility
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

/// Fingerprint-cached [`build_models`]; the returned handle is cheap to
/// clone and stays valid across later state borrows.
pub(in crate::workbench::documents::result_document) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    results.synchronize_wave_caches(simulation);
    results.reconcile_expression_projection(simulation);
    let fp = models_fingerprint(
        simulation,
        results.viewer,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
        &results.hidden_family_traces,
        &results.waveform_visibility,
        t,
    );
    if let Some(cached) = &results.models.cached
        && cached.fingerprint == fp
        && cached.decks.shares_content_with(&simulation.executed_decks)
    {
        return Arc::clone(&cached.models);
    }
    let mut built = build_models(
        simulation,
        &mut results.derived,
        t,
        results.phase_continuous,
        complex_display,
        results.sample_selection.as_ref(),
        &results.hidden_family_traces,
    );
    apply_waveform_visibility(
        &mut built,
        simulation,
        &results.waveform_visibility,
        &results.hidden_family_traces,
    );
    // Only now is it settled which traces the strip draws, so only now can
    // its shared X extent be resolved.
    extent::resolve_x_ranges(&mut built);
    built.retain(|model| match results.viewer {
        ResultViewer::DcSweep => model.analysis_type == AnalysisType::DcSweep,
        ResultViewer::Waves => model.analysis_type.is_time_domain(),
        ResultViewer::Bode => {
            model.analysis_type.is_bode_response() || model.analysis_type.is_raw_frequency_curve()
        }
        ResultViewer::NoiseContrib => matches!(
            model.analysis_type,
            AnalysisType::Noise | AnalysisType::Hbnoise
        ),
        _ => true,
    });
    let models = Arc::new(built);
    results.models.generation = results.models.generation.wrapping_add(1);
    if results.models.generation == 0 {
        results.plans.envelopes = extent::FamilyEnvelopeCache::default();
    }
    results.models.cached = Some(CachedModels {
        fingerprint: fp,
        decks: simulation.executed_decks.clone(),
        models: Arc::clone(&models),
    });
    models
}
