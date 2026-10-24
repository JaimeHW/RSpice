//! Capture the live Results sheet's reading through its authoritative UI accessors.

use super::*;
use crate::workbench::documents::result_document::{AnalysisPresentationKey, MarkerKind};
use crate::workbench::preferences::CursorInterpolation;
use rspice_results::result_presentation::SourceWaveformPresentationKey;

pub(super) fn capture_quick_view_overlays(
    state: &AppState,
    run: &SimulationRun,
) -> Result<RetainedQuickViewOverlays, HardcopySourceError> {
    let strips = run
        .analyses
        .iter()
        .map(|analysis| {
            capture_quick_view_overlay(state, run, analysis).map(|overlay| (analysis.id, overlay))
        })
        .collect::<Result<Vec<_>, _>>()?;
    RetainedQuickViewOverlays::try_new(strips)
}

/// Freeze the reading state of the strip a quick-view page will show.
fn capture_quick_view_overlay(
    state: &AppState,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<RetainedQuickViewOverlay, HardcopySourceError> {
    let results = &state.results.results;
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let visible_traces = analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            results.session.waveform_visibility(
                &SourceWaveformPresentationKey::new(analysis_key, &waveform.name),
                waveform.visible,
            )
        })
        .map(|waveform| waveform.name.clone())
        .collect();
    // One read path with the screen: `strip_markers` unions the quick and
    // document stores and verifies the overlay against the live pane
    // context, so a stale projection cannot put another document's
    // markers on this page.
    let markers = results
        .session
        .strip_markers(analysis_key)
        .map(|marker| RetainedQuickMarker {
            label: rspice_results_ui::waves::dock::marker_label(marker),
            kind: marker.kind(),
            x: marker.x(),
            trace_name: (marker.kind() != MarkerKind::Spec).then(|| marker.trace_name().to_owned()),
        })
        .collect();
    RetainedQuickViewOverlay::try_new(
        Some(visible_traces),
        results.session.cursors.a,
        results.session.cursors.b,
        markers,
        match state
            .model
            .preferences
            .result_presentation_policy()
            .cursor_interpolation()
        {
            CursorInterpolation::MonotoneCubicWhereValid => {
                RetainedCursorInterpolation::MonotoneCubic
            }
            CursorInterpolation::Linear => RetainedCursorInterpolation::Linear,
            CursorInterpolation::NearestAcceptedPoint => RetainedCursorInterpolation::Nearest,
        },
        crate::workbench::documents::result_document::captured_viewport(state, analysis_key)
            .map(|(x, y)| RetainedQuickViewport { x, y }),
    )
}
