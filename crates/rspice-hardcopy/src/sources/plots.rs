//! Cartesian source projection from canonical retained visualization documents.

use super::*;

pub fn resolve_plot_source(
    source: PlotHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    validate_label("display name", &source.display_name, DISPLAY_NAME_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    if source.scene.traces().is_empty() {
        return Err(HardcopySourceError::UnretainedResult(
            "the active pane has no retained visible trace samples".to_owned(),
        ));
    }
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let x_range = source.scene.x_range();
    let y_range = source.scene.y_range();
    let x_span = x_range.maximum - x_range.minimum;
    let y_span = y_range.maximum - y_range.minimum;
    if !x_span.is_finite() || !y_span.is_finite() || x_span <= 0.0 || y_span <= 0.0 {
        return Err(HardcopySourceError::InvalidResultRange);
    }
    let mut traces = Vec::with_capacity(source.scene.traces().len());
    for trace in source.scene.traces() {
        if trace.points().is_empty() {
            return Err(HardcopySourceError::UnretainedResult(format!(
                "visible trace `{}` has no retained samples",
                trace.label()
            )));
        }
        let source_points = trace
            .points()
            .iter()
            .map(|point| (point.x(), point.y()))
            .collect::<Vec<_>>();
        let paths = clipped_plot_paths(
            &source_points,
            x_range.minimum,
            x_range.maximum,
            y_range.minimum,
            y_range.maximum,
            plot_width,
            plot_height,
        )?;
        traces.push(SemanticPlotTrace {
            trace_id: trace.trace_id().get(),
            label: trace.label().to_owned(),
            paths,
            source_samples: source_points
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
        });
    }
    let semantic = SemanticPlot {
        viewer: ResultViewer::Waves,
        page_id: source.scene.page_id().get(),
        pane_id: source.scene.pane_id().get(),
        // The Cartesian raster scene resolves only linear axes today; a
        // logarithmic pane is refused upstream by `resolve_cartesian_line_scene`.
        x_scale: AxisScale::Linear,
        y_scale: AxisScale::Linear,
        axis_ticks: Vec::new(),
        traces,
        cursors: source
            .scene
            .cursors()
            .iter()
            .filter_map(|cursor| canonical_cursor_semantics(source.scene, cursor))
            .collect(),
        markers: Vec::new(),
        annotations: Vec::new(),
        captions: Vec::new(),
    };
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.scene.document_id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.scene.revision(),
        source.display_name,
    )?;
    finish_resolved(
        identity,
        source.scene.source_digest(),
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::Plot(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
    )
}

/// Resolve an active Visualization Studio pane without depending on a window,
/// screenshot, framebuffer, or transient viewer cache.
pub fn resolve_visualization_pane_source(
    source: VisualizationPaneHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let scene = resolve_cartesian_line_scene(
        source.document,
        source.reference,
        source.page_id,
        source.pane_id,
    )
    .map_err(map_visualization_error)?;
    let mut resolved = resolve_plot_source(PlotHardcopySource {
        source_key: source.source_key,
        display_name: source.display_name,
        scene: &scene,
        scope: source.scope,
    })?;
    let HardcopySemanticDocument::Plot(plot) = &mut resolved.semantic_document else {
        unreachable!("plot source resolver always returns plot semantics")
    };
    plot.markers = source
        .document
        .markers()
        .iter()
        .filter(|marker| marker.pane_id == source.pane_id)
        .map(|marker| canonical_marker_semantics(&scene, marker))
        .collect();
    plot.annotations = source
        .document
        .annotations()
        .iter()
        .filter(|annotation| annotation.pane_id == source.pane_id)
        .map(|annotation| canonical_annotation_semantics(&scene, annotation))
        .collect();
    let content_digest = canonical_digest(
        b"rspice-hardcopy-visualization-pane-v2",
        &(scene.source_digest(), &resolved.semantic_document),
    )?;
    let document_id = resolved.authority.document_id();
    let revision = resolved.authority.revision();
    let display_name = resolved.authority.display_name().to_owned();
    let document_kind = resolved.authority.document_kind();
    let scope = resolved.authority.scope().clone();
    resolved.authority = ActiveHardcopySource::try_new(
        document_id,
        revision,
        content_digest,
        display_name,
        document_kind,
        scope,
    )
    .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?;
    resolved.default_print_mapping = default_print_mapping(&resolved.semantic_document)?;
    Ok(resolved)
}

fn map_visualization_error(error: VisualizationRasterError) -> HardcopySourceError {
    match error {
        error @ (VisualizationRasterError::PageNotFound(_)
        | VisualizationRasterError::PaneNotFound(_)
        | VisualizationRasterError::DatasetNotFound(_)
        | VisualizationRasterError::EmptyTrace(_)
        | VisualizationRasterError::NoVisibleTraces) => {
            HardcopySourceError::UnretainedResult(error.to_string())
        }
        error => HardcopySourceError::InvalidVisualizationSource(error.to_string()),
    }
}
