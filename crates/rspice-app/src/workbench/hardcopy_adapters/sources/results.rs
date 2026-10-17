//! Printable sources built from what a results viewer is currently showing.
//!
//! Quick-view plots are resolved from the retained result record and the
//! viewer's own declared presentation — never from a screenshot,
//! framebuffer, or transient viewer cache.  That is what lets a printed page
//! be reproduced exactly from the same inputs long after the window that
//! showed it has gone.

use super::*;

#[cfg(test)]
pub(super) fn active_quick_result(
    state: &AppState,
    viewer: ResultViewer,
) -> Result<RetainedQuickViewSource<'_, WaveformData>, HardcopySourceError> {
    let run = active_terminal_run(state)?;
    let analysis_index = quick_result_analysis_index(state, run, viewer).ok_or_else(|| {
        HardcopySourceError::UnretainedResult(format!(
            "no saved analysis has results for {}",
            viewer.label()
        ))
    })?;
    let analysis = run.analyses.get(analysis_index).ok_or_else(|| {
        HardcopySourceError::UnretainedResult(format!(
            "active analysis index {analysis_index} is not retained in dataset {}",
            run.dataset_id
        ))
    })?;
    RetainedQuickViewSource::try_new(run.as_ref(), analysis.id, |waveform: &WaveformData| {
        waveform.visible
    })
}

#[cfg(test)]
pub(super) fn active_terminal_run(state: &AppState) -> Result<&SimulationRun, HardcopySourceError> {
    let run = state.simulation.active_run().ok_or_else(|| {
        HardcopySourceError::UnretainedResult("no active result dataset is selected".to_owned())
    })?;
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "active dataset {} belongs to a non-terminal run",
            run.dataset_id
        )));
    }
    Ok(run)
}

/// Resolve the exact result document currently selected in the ordinary
/// Results workspace. Specialized viewers read their durable analysis model
/// directly; table viewers read the selected immutable simulation result.
#[cfg(test)]
pub(crate) fn resolve_results_quick_view_source(
    source: ResultsQuickViewHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let presentation = capture_results_quick_view_presentation(source.state)?;
    if presentation.viewer() == ResultViewer::Manifest {
        let run = active_terminal_run(source.state)?;
        return resolve_results_manifest_source(
            source.source_key,
            source.project_id,
            source.scope,
            run.as_ref(),
        );
    }
    let active = active_quick_result(source.state, presentation.viewer())?;
    resolve_results_quick_view_parts(
        source.source_key,
        source.project_id,
        source.scope,
        &active,
        &presentation,
    )
}
