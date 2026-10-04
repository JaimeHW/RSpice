//! Retained-history invalidation and source selection for waveform projections.
use super::{
    ComplexNumberDisplay, ResultsState, SimulationState, StripModel, Tokens, build_models,
};
use rspice_results_ui::waves::ProjectionOptions;
use rspice_results_ui::waves::cache::{ModelCacheInput, apply_waveform_visibility};
use std::sync::Arc;

pub(in crate::workbench::documents::result_document) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    complex_display: ComplexNumberDisplay,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    results.synchronize_wave_caches(simulation);
    results.reconcile_expression_projection(simulation);
    let session = &mut results.session;
    results.models.get_or_build(
        ModelCacheInput {
            active_run_idx: simulation.view.active_run_idx,
            overlay_dataset_ids: &simulation.view.overlay_dataset_ids,
            viewer: session.viewer,
            projection: ProjectionOptions {
                phase_continuous: session.phase_continuous,
                complex_display,
                selection: session.sample_selection.as_ref(),
                hidden_family_traces: &session.hidden_family_traces,
            },
            waveform_visibility: &session.waveform_visibility,
        },
        &simulation.retained.executed_decks,
        t,
        &mut results.plans.envelopes,
        |projection, visibility| {
            let mut built = build_models(
                simulation,
                &mut session.derived,
                t,
                projection.phase_continuous,
                projection.complex_display,
                projection.selection,
                projection.hidden_family_traces,
            );
            apply_waveform_visibility(
                &mut built,
                simulation.active_run().map(AsRef::as_ref),
                visibility,
                projection.hidden_family_traces,
            );
            built
        },
    )
}
