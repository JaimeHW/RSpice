//! Studio source validation, metric preparation, and navigation integration.

use super::*;
use rspice_results_ui::studio::chrome::{self as presentation, StatusCounts};

pub(super) fn workspace_header(ui: &mut Ui, app: &mut RSpiceApp) {
    let configuration = visualization_configuration_status(&app.state);
    if presentation::workspace_header(
        ui,
        configuration.as_ref().map(|_| ()).map_err(String::as_str),
        app.state.workbench.previous_route().is_some(),
    ) {
        let _ = app
            .state
            .workbench
            .navigate_back(RouteTransitionSource::User);
    }
}

pub(super) fn visualization_configuration_status(state: &AppState) -> Result<(), String> {
    let studio = &state.workbench.visualization_studio;
    studio.validate_presentation()?;
    if studio.panes.is_empty() {
        return Err("No visualization pane is bound to an immutable result dataset".to_owned());
    }
    for pane in &studio.panes {
        let run = state
            .simulation
            .runs
            .iter()
            .find(|run| run.dataset_id == pane.dataset_id)
            .ok_or_else(|| format!("Pane {:02} references an unavailable dataset", pane.id))?;
        let analysis = run
            .analyses
            .iter()
            .find(|analysis| analysis.id == pane.analysis_sequence)
            .ok_or_else(|| format!("Pane {:02} references an unavailable analysis", pane.id))?;
        let definition = viewer_document(&pane.viewer_document_id)
            .ok_or_else(|| format!("Pane {:02} references an unknown viewer", pane.id))?;
        // A pane retains the exact sheet as well as the shared catalog
        // document. Validate that forward identity before consulting the
        // catalog's necessarily lossy inverse (Specs, OP, and Table all use
        // `viewer-table`).
        if pane.viewer.viewer_document_id() != Some(definition.id) {
            return Err(format!(
                "Pane {:02} has no exact renderer for its retained viewer",
                pane.id
            ));
        }
        // Through the workspace memos. This runs once per pane, on every
        // frame, from the Studio's own header: resolving it directly walked
        // every retained sample of every waveform behind every bound pane
        // for a reader who was not touching anything.
        let exact_sheet_compatible =
            result_document::view_context::analysis_supports_viewer_memoized(
                state,
                pane.dataset_id,
                pane.viewer,
                analysis,
            );
        let analysis_ids = [analysis_manifest_id(analysis.analysis_type)];
        match viewer_compatibility(
            definition.id,
            ViewerCapabilities {
                analysis_ids: &analysis_ids,
                external_capabilities: &[],
            },
        ) {
            ViewerCompatibility::Compatible => {}
            // A shared catalog document cannot encode the analysis contract
            // of every forward-mapped sheet. The exact Rust renderer is
            // authoritative when it accepts the retained evidence.
            ViewerCompatibility::MissingAnalysis { .. } if exact_sheet_compatible => {}
            ViewerCompatibility::MissingAnalysis { .. } => {
                return Err(format!(
                    "Pane {:02} viewer is incompatible with its retained analysis",
                    pane.id
                ));
            }
            ViewerCompatibility::MissingExternalCapability { capability_id } => {
                return Err(format!(
                    "Pane {:02} requires unavailable capability {capability_id}",
                    pane.id
                ));
            }
            ViewerCompatibility::UnknownDocument => {
                return Err(format!("Pane {:02} viewer is not registered", pane.id));
            }
        }
        // A retained pane names both its sheet and its viewer document, so what
        // has to hold is that the two agree — read forwards, from the sheet.
        // Read backwards it would not: three sheets render `viewer-table`, and
        // the inverse can only name one of them, so a retained Specs or OP pane
        // would be rejected as having no renderer.
        if pane.viewer.viewer_document_id() != Some(definition.id) {
            return Err(format!(
                "Pane {:02} has no exact renderer for its retained viewer",
                pane.id
            ));
        }
        if !exact_sheet_compatible {
            return Err(format!(
                "Pane {:02} viewer is incompatible with its retained analysis",
                pane.id
            ));
        }
        if pane.viewer == ResultViewer::PoleZero && retained_pole_zero_payload(analysis).is_none() {
            return Err(format!(
                "Pane {:02} has no valid retained pole-zero payload",
                pane.id
            ));
        }
        if pane.viewer == ResultViewer::Contribution
            && retained_sensitivity_payload(analysis).is_none()
        {
            return Err(format!(
                "Pane {:02} has no valid retained sensitivity payload",
                pane.id
            ));
        }
    }
    for annotation in &studio.annotations {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == annotation.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == annotation.analysis_sequence)
        }) {
            return Err(format!(
                "Annotation {:02} references an unavailable source",
                annotation.id
            ));
        }
    }
    for marker in &studio.markers {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == marker.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == marker.analysis_sequence)
        }) {
            return Err(format!(
                "Marker {:02} references an unavailable source",
                marker.id
            ));
        }
    }
    for measurement in &studio.measurements {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == measurement.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == measurement.analysis_sequence)
        }) {
            return Err(format!(
                "Measurement {:02} references an unavailable source",
                measurement.id
            ));
        }
    }
    Ok(())
}

pub(super) fn status_strip(ui: &mut Ui, app: &RSpiceApp) {
    let studio = &app.state.workbench.visualization_studio;
    let bound_datasets = studio
        .panes
        .iter()
        .map(|pane| pane.dataset_id)
        .collect::<HashSet<_>>();
    let dataset_count = bound_datasets.len();
    let pane_count = studio.panes.len();
    let linked_groups = studio
        .panes
        .iter()
        .flat_map(|pane| [pane.x_link, pane.cursor_group])
        .flatten()
        .collect::<HashSet<_>>()
        .len();
    let expression_count: usize = app.state.ui.results.exprs.values().map(Vec::len).sum();
    let samples: usize = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| bound_datasets.contains(&run.dataset_id))
        .flat_map(|run| &run.analyses)
        .flat_map(|analysis| &analysis.waveforms)
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum();
    presentation::status_strip(
        ui,
        StatusCounts {
            dataset_count,
            pane_count,
            linked_groups,
            expression_count,
            samples,
            revision: studio.revision,
        },
        app.state.workbench.coarse_pointer,
    );
}

#[cfg(test)]
mod tests {
    /// The header resolves every pane's compatibility through the workspace
    /// memos, never by walking the datasets behind them.
    ///
    /// `visualization_configuration_status` runs on every frame, once per
    /// bound pane, and it called the unmemoized predicate — which reads every
    /// sample of every waveform in the analysis, and every structural gate
    /// besides. The two spellings differ by one identifier, so a source guard
    /// is what keeps the cheap one in place: nothing about calling
    /// `analysis_supports_viewer` here fails a test on its own.
    #[test]
    fn the_header_never_resolves_pane_compatibility_by_walking_the_dataset() {
        let shipped = crate::source_guard::production_source(include_str!("chrome.rs"));
        let offenders = shipped
            .lines()
            .enumerate()
            .filter(|(_, line)| {
                line.contains("analysis_supports_viewer")
                    && !line.contains("analysis_supports_viewer_memoized")
            })
            .map(|(index, line)| format!("{}: {}", index + 1, line.trim()))
            .collect::<Vec<_>>();

        assert!(
            offenders.is_empty(),
            "the Studio header must resolve pane compatibility through \
             `analysis_supports_viewer_memoized`:\n{}",
            offenders.join("\n")
        );
    }
}
