//! Printable sources built from what a results viewer is currently showing.
//!
//! Studio panes and quick-view plots are resolved from the retained result
//! record and the pane's own declared presentation — never from a screenshot,
//! framebuffer, or transient viewer cache.  That is what lets a printed page
//! be reproduced exactly from the same inputs long after the window that
//! showed it has gone.

use super::*;
#[cfg(test)]
use crate::workbench::documents::result_document::manifest::ManifestViewModel;

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

pub(crate) fn resolve_visualization_document_source(
    source_key: String,
    project_id: ProjectId,
    document: &VisualizationDocument,
    page_id: PageId,
    pane_id: PaneId,
    all_panes: bool,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let reference = visualization_document_reference(document)?;
    if !all_panes {
        return resolve_visualization_pane_source(VisualizationPaneHardcopySource {
            source_key,
            display_name: document.title().to_owned(),
            document,
            reference: &reference,
            page_id,
            pane_id,
            scope,
        });
    }
    if !matches!(scope, HardcopyScope::AllSheetsOrPanes) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let mut ordered = Vec::new();
    for page in document.pages() {
        let mut panes = document
            .panes()
            .iter()
            .filter(|pane| pane.page_id == page.id)
            .collect::<Vec<_>>();
        panes.sort_by_key(|pane| (pane.order, pane.id.get()));
        ordered.extend(panes.into_iter().map(|pane| (page, pane)));
    }
    if ordered.is_empty() {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-panes result-document hardcopy requires at least one retained pane".to_owned(),
        ));
    }
    let mut resolved_panes = Vec::with_capacity(ordered.len());
    for (page, pane) in ordered {
        resolved_panes.push(resolve_visualization_pane_source(
            VisualizationPaneHardcopySource {
                source_key: visualization_document_pane_source_key(
                    project_id,
                    document.id(),
                    pane.id,
                ),
                display_name: format!("{} · {} · {}", document.title(), page.title, pane.title),
                document,
                reference: &reference,
                page_id: page.id,
                pane_id: pane.id,
                scope: HardcopyScope::ActivePlotDocument,
            },
        )?);
    }
    let members = resolved_panes
        .iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(document.id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        document.revision(),
        document.title(),
        HardcopyDocumentKind::PlotOrWorksheet,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_panes = resolved_panes.into_iter();
    let mut resolved = resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_panes.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })?;
    resolved.source_key = source_key;
    Ok(resolved)
}

pub(crate) fn resolve_all_studio_panes(
    project_id: ProjectId,
    studio: &VisualizationStudioState,
    simulation: &SimulationState,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if studio.panes.is_empty() {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-panes scope requires at least one retained pane".to_owned(),
        ));
    }
    let mut resolved_panes = Vec::with_capacity(studio.panes.len());
    for pane in &studio.panes {
        resolved_panes.push(resolve_active_studio_pane_source(
            ActiveStudioPaneHardcopySource {
                source_key: format!(
                    "project:{}:visualization-pane:{}",
                    project_id.as_uuid(),
                    pane.id
                ),
                project_id,
                studio,
                simulation,
                pane_id: pane.id,
                scope: HardcopyScope::ActivePlotDocument,
            },
        )?);
    }
    let members = resolved_panes
        .iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &project_id.as_uuid(),
            b"rspice-hardcopy-all-visualization-panes-v1",
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(studio.revision)
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        "All visualization panes",
        HardcopyDocumentKind::PlotOrWorksheet,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_panes = resolved_panes.into_iter();
    resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_panes.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })
}

/// Resolve the exact active Visualization Studio pane directly from its
/// retained simulation dataset. This closes the application integration gap
/// without manufacturing a report reference or consulting the currently
/// rendered plot widget.
pub(crate) fn resolve_active_studio_pane_source(
    source: ActiveStudioPaneHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let pane_id = source.pane_id;
    let panes = source
        .studio
        .panes
        .iter()
        .filter(|pane| pane.id == pane_id)
        .collect::<Vec<_>>();
    let [pane] = panes.as_slice() else {
        return if panes.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "active pane {pane_id} is not retained"
            )))
        } else {
            Err(HardcopySourceError::AmbiguousActiveSource(format!(
                "visualization pane {pane_id}"
            )))
        };
    };
    if is_curve_viewer(pane.viewer) && source.studio.family_policies.contains_key(&pane.id) {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "active family presentation requires its exact resolved family slice".to_owned(),
        ));
    }
    if is_curve_viewer(pane.viewer)
        && source.studio.autoscale == VisualizationAutoscale::SpecificationBounds
    {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "specification-bound autoscale requires the active project specification authority"
                .to_owned(),
        ));
    }

    let runs = source
        .simulation
        .runs
        .iter()
        .filter(|run| run.dataset_id == pane.dataset_id)
        .collect::<Vec<_>>();
    let [run] = runs.as_slice() else {
        return if runs.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "dataset {} is not retained",
                pane.dataset_id
            )))
        } else {
            Err(HardcopySourceError::AmbiguousRetainedDataset(
                pane.dataset_id.to_string(),
            ))
        };
    };
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "dataset {} belongs to a non-terminal run",
            pane.dataset_id
        )));
    }
    let analyses = run
        .analyses
        .iter()
        .filter(|analysis| analysis.id == pane.analysis_sequence)
        .collect::<Vec<_>>();
    let [analysis] = analyses.as_slice() else {
        return if analyses.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "analysis {} is not retained in dataset {}",
                pane.analysis_sequence, pane.dataset_id
            )))
        } else {
            Err(HardcopySourceError::AmbiguousRetainedAnalysis(
                pane.analysis_sequence,
            ))
        };
    };
    if !analysis.success {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "analysis {} did not complete successfully",
            analysis.id
        )));
    }
    if !is_curve_viewer(pane.viewer) {
        return resolve_studio_result_summary(source, pane, run.run_id, analysis);
    }
    let visible = analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .collect::<Vec<_>>();
    if visible.is_empty() {
        return Err(HardcopySourceError::UnretainedResult(
            "the active pane has no visible retained waveform".to_owned(),
        ));
    }
    for waveform in &visible {
        if waveform.x.is_empty()
            || waveform.x.len() != waveform.y.len()
            || waveform
                .x
                .iter()
                .chain(waveform.y.iter())
                .any(|value| !value.is_finite())
        {
            return Err(HardcopySourceError::InvalidRetainedWaveform(
                waveform.name.clone(),
            ));
        }
    }

    let source_x_minimum = visible
        .iter()
        .flat_map(|waveform| waveform.x.iter().copied())
        .min_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no X samples".to_owned()))?;
    let source_x_maximum = visible
        .iter()
        .flat_map(|waveform| waveform.x.iter().copied())
        .max_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no X samples".to_owned()))?;
    let (x_minimum, x_maximum) = source
        .studio
        .pane_x_ranges
        .get(&pane.id)
        .copied()
        .filter(|(minimum, maximum)| {
            minimum.is_finite() && maximum.is_finite() && minimum < maximum
        })
        .unwrap_or_else(|| nondegenerate_range(source_x_minimum, source_x_maximum));
    let source_y_minimum = visible
        .iter()
        .flat_map(|waveform| waveform.y.iter().copied())
        .min_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no Y samples".to_owned()))?;
    let source_y_maximum = visible
        .iter()
        .flat_map(|waveform| waveform.y.iter().copied())
        .max_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no Y samples".to_owned()))?;
    let (mut y_minimum, mut y_maximum) = nondegenerate_range(source_y_minimum, source_y_maximum);
    if source.studio.autoscale == VisualizationAutoscale::RobustVisible {
        let padding = ((y_maximum - y_minimum) * 0.05).max(f64::EPSILON);
        y_minimum -= padding;
        y_maximum += padding;
    }

    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let mut traces = Vec::with_capacity(visible.len());
    let mut trace_ids = std::collections::HashSet::new();
    for waveform in &visible {
        let trace_id = stable_trace_id(pane.dataset_id, analysis.id, &waveform.name);
        if !trace_ids.insert(trace_id) {
            return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
        }
        let source_points = waveform
            .x
            .iter()
            .copied()
            .zip(waveform.y.iter().copied())
            .collect::<Vec<_>>();
        traces.push(SemanticPlotTrace {
            trace_id,
            label: waveform.name.clone(),
            paths: clipped_plot_paths(
                &source_points,
                x_minimum,
                x_maximum,
                y_minimum,
                y_maximum,
                plot_width,
                plot_height,
            )?,
            source_samples: source_points
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
        });
    }
    let markers = source
        .studio
        .markers
        .iter()
        .filter(|marker| {
            marker.dataset_id == pane.dataset_id
                && marker.analysis_sequence == pane.analysis_sequence
        })
        .map(|marker| {
            Ok(SemanticPlotMarker {
                marker_id: marker.id,
                label: marker.label.clone(),
                trace_id: Some(stable_trace_id(
                    marker.dataset_id,
                    marker.analysis_sequence,
                    &marker.waveform_name,
                )),
                source_x_bits: Some(marker.x.to_bits()),
                source_y_bits: Some(marker.y.to_bits()),
                position: Some(map_plot_point(
                    marker.x.clamp(x_minimum, x_maximum),
                    marker.y.clamp(y_minimum, y_maximum),
                    x_minimum,
                    y_minimum,
                    x_maximum - x_minimum,
                    y_maximum - y_minimum,
                    plot_width,
                    plot_height,
                )?),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let annotations = source
        .studio
        .annotations
        .iter()
        .filter(|annotation| {
            annotation.dataset_id == pane.dataset_id
                && annotation.analysis_sequence == pane.analysis_sequence
        })
        .map(|annotation| {
            Ok(SemanticPlotAnnotation {
                annotation_id: annotation.id,
                text: annotation.text.clone(),
                trace_id: None,
                source_x_bits: Some(annotation.x.to_bits()),
                source_y_bits: None,
                position: Some(map_plot_point(
                    annotation.x.clamp(x_minimum, x_maximum),
                    y_maximum,
                    x_minimum,
                    y_minimum,
                    x_maximum - x_minimum,
                    y_maximum - y_minimum,
                    plot_width,
                    plot_height,
                )?),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let semantic = SemanticPlot {
        viewer: pane.viewer,
        page_id: stable_page_id(&pane.page),
        pane_id: pane.id,
        traces,
        markers,
        annotations,
    };
    let digest = studio_pane_digest(
        source.studio,
        pane,
        run.run_id,
        analysis.id,
        &visible,
        &source
            .studio
            .markers
            .iter()
            .filter(|marker| {
                marker.dataset_id == pane.dataset_id
                    && marker.analysis_sequence == pane.analysis_sequence
            })
            .collect::<Vec<_>>(),
        &source
            .studio
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.dataset_id == pane.dataset_id
                    && annotation.analysis_sequence == pane.analysis_sequence
            })
            .collect::<Vec<_>>(),
    )?;
    let identity =
        studio_source_identity(&source.source_key, source.project_id, source.studio, pane)?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::Plot(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
    )
}

/// Resolve the exact result document currently selected in the ordinary
/// Results workspace. Specialized viewers read their durable analysis model
/// directly; table viewers read the selected immutable simulation result.
#[cfg(test)]
pub(crate) fn resolve_results_quick_view_source(
    source: ResultsQuickViewHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let presentation = ResultsQuickViewPresentation::from_state(source.state);
    if presentation.viewer == ResultViewer::Manifest {
        let run = active_terminal_run(source.state)?;
        return resolve_results_manifest_source(
            source.source_key,
            source.project_id,
            source.scope,
            run,
        );
    }
    let active = active_quick_result(source.state)?;
    resolve_results_quick_view_parts(
        source.source_key,
        source.project_id,
        source.scope,
        active,
        &presentation,
    )
}

pub(super) fn resolve_results_quick_view_parts(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    active: ActiveQuickResult<'_>,
    presentation: &ResultsQuickViewPresentation,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let viewer = presentation.viewer;
    let semantic_document = match viewer {
        ResultViewer::Waves | ResultViewer::Bode => {
            HardcopySemanticDocument::Plot(quick_waveform_plot(active, viewer)?)
        }
        ResultViewer::Fft => HardcopySemanticDocument::Plot(quick_fft_plot(presentation, active)?),
        ResultViewer::Eye => HardcopySemanticDocument::Plot(quick_eye_plot(presentation, active)?),
        ResultViewer::Hist => {
            HardcopySemanticDocument::Plot(quick_histogram_plot(presentation, active)?)
        }
        ResultViewer::Nyquist => {
            HardcopySemanticDocument::Plot(quick_complex_plot(active, ResultViewer::Nyquist)?)
        }
        ResultViewer::Smith => {
            HardcopySemanticDocument::Plot(quick_complex_plot(active, ResultViewer::Smith)?)
        }
        ResultViewer::Op
        | ResultViewer::NoiseContrib
        | ResultViewer::Contribution
        | ResultViewer::TransferFunction
        | ResultViewer::Specs
        | ResultViewer::Table
        | ResultViewer::PoleZero => HardcopySemanticDocument::ResultSummary(
            semantic_result_summary(viewer, active.analysis)?,
        ),
        ResultViewer::Manifest => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                "dataset-native Manifest must resolve from its owning run".to_owned(),
            ));
        }
    };
    let digest = canonical_digest(
        b"rspice-hardcopy-results-quick-view-v2",
        &(
            active.run.dataset_id,
            active.run.run_id,
            active.analysis.id,
            active.analysis.result_data_digest(),
            viewer,
            &semantic_document,
        ),
    )?;
    let identity =
        results_quick_view_identity(&source_key, project_id, viewer, active.run, active.analysis)?;
    let bounds = match &semantic_document {
        HardcopySemanticDocument::Plot(_) => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
        _ => SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    };
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        bounds,
    )
}

#[cfg(test)]
fn resolve_results_manifest_source(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    run: &SimulationRun,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }

    let manifest = ManifestViewModel::from_run(run);
    let semantic_document =
        HardcopySemanticDocument::ResultSummary(semantic_manifest_summary(&manifest));
    let dataset_digest = run.dataset_content_digest();
    let digest = canonical_digest(
        b"rspice-hardcopy-results-manifest-v1",
        &(
            run.dataset_id,
            run.run_id,
            dataset_digest,
            &semantic_document,
        ),
    )?;
    let mut identity_name = Vec::with_capacity(80);
    identity_name.extend_from_slice(b"rspice-results-manifest-v1");
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run.run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(dataset_digest.as_bytes());
    let identity = HardcopySourceIdentity::try_new(
        source_key.clone(),
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results · {} · Manifest", run.label),
    )?;

    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    )
}

#[cfg(test)]
fn semantic_manifest_summary(manifest: &ManifestViewModel) -> SemanticResultSummary {
    let inventory = SemanticTable {
        title: format!("Frozen analysis inventory · {}", manifest.run_label),
        columns: vec![
            "Analysis".to_owned(),
            "Expansion".to_owned(),
            "Tasks".to_owned(),
            "Domain axis".to_owned(),
            "Stored values".to_owned(),
            "Precision".to_owned(),
            "Eligibility".to_owned(),
        ],
        rows: manifest
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.analysis.clone(),
                    row.expansion.clone(),
                    row.tasks.clone(),
                    row.domain_axis.clone(),
                    row.stored_values.clone(),
                    row.precision.clone(),
                    row.eligibility.clone(),
                ]
            })
            .collect(),
    };
    let mut tables = vec![
        inventory,
        SemanticTable {
            title: "Dataset identity".to_owned(),
            columns: vec!["Field".to_owned(), "Value".to_owned()],
            rows: vec![
                vec!["Dataset".to_owned(), manifest.dataset_id.clone()],
                vec!["Content digest".to_owned(), manifest.dataset_digest.clone()],
                vec!["Run".to_owned(), manifest.run_id.clone()],
                vec!["Run sequence".to_owned(), manifest.run_sequence.clone()],
                vec!["Lifecycle".to_owned(), manifest.lifecycle.clone()],
                vec![
                    "Execution target".to_owned(),
                    manifest.execution_target.clone(),
                ],
                vec!["Duration".to_owned(), manifest.elapsed_time.clone()],
            ],
        },
        SemanticTable {
            title: "Integrity and eligibility".to_owned(),
            columns: vec!["Field".to_owned(), "Value".to_owned()],
            rows: vec![
                vec!["Receipt".to_owned(), manifest.integrity.clone()],
                vec!["Qualification".to_owned(), manifest.qualification.clone()],
                vec!["Frozen tasks".to_owned(), manifest.task_count.to_string()],
                vec![
                    "Retained results".to_owned(),
                    manifest.retained_result_count.to_string(),
                ],
            ],
        },
    ];

    if let Some(authority) = &manifest.authority {
        tables.push(SemanticTable {
            title: "Prepared source authority".to_owned(),
            columns: vec!["Field".to_owned(), "Value".to_owned()],
            rows: vec![
                vec!["Source domain".to_owned(), authority.source_domain.clone()],
                vec![
                    "Simulation plan".to_owned(),
                    authority
                        .simulation_plan_id
                        .clone()
                        .unwrap_or_else(|| "manual deck · no simulation plan".to_owned()),
                ],
                vec![
                    "Project revision".to_owned(),
                    authority.project_revision.clone(),
                ],
                vec![
                    "Prepared snapshot".to_owned(),
                    authority.prepared_snapshot_digest.clone(),
                ],
                vec![
                    "Source content".to_owned(),
                    authority.source_content_digest.clone(),
                ],
                vec!["Source check".to_owned(), authority.source_check.clone()],
                vec![
                    "Check digest".to_owned(),
                    authority.source_check_digest.clone(),
                ],
            ],
        });
        if !authority.model_sources.is_empty() {
            tables.push(SemanticTable {
                title: "Model source digests".to_owned(),
                columns: vec!["Model source".to_owned(), "Content digest".to_owned()],
                rows: authority
                    .model_sources
                    .iter()
                    .map(|(name, digest)| vec![name.clone(), digest.clone()])
                    .collect(),
            });
        }
    }

    SemanticResultSummary {
        viewer: ResultViewer::Manifest,
        title: format!("Frozen analysis inventory · {}", manifest.run_label),
        tables,
        payload: None,
    }
}

#[derive(Debug)]
pub(super) struct QuickResultSeries {
    pub(super) identity: String,
    pub(super) label: String,
    pub(super) points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ActiveQuickResult<'a> {
    pub(super) run: &'a SimulationRun,
    pub(super) analysis: &'a AnalysisResult,
}

#[cfg(test)]
pub(super) fn active_quick_result(
    state: &AppState,
) -> Result<ActiveQuickResult<'_>, HardcopySourceError> {
    let run = active_terminal_run(state)?;
    let analysis_index = state.simulation.active_analysis_idx.ok_or_else(|| {
        HardcopySourceError::UnretainedResult(
            "no active analysis is selected in the active terminal dataset".to_owned(),
        )
    })?;
    let analysis = run.analyses.get(analysis_index).ok_or_else(|| {
        HardcopySourceError::UnretainedResult(format!(
            "active analysis index {analysis_index} is not retained in dataset {}",
            run.dataset_id
        ))
    })?;
    if !analysis.success {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "active analysis {} did not complete successfully",
            analysis.id
        )));
    }
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    Ok(ActiveQuickResult { run, analysis })
}

pub(super) fn quick_waveform_plot(
    active: ActiveQuickResult<'_>,
    viewer: ResultViewer,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .map(|waveform| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: waveform
                .x
                .iter()
                .copied()
                .zip(waveform.y.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(viewer, "Results", 0, series)
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

pub(super) fn quick_fft_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft.selected_source.as_deref(),
        "FFT source waveform",
    )?;
    let input = crate::analysis::fft::prepare_fft_input_with_options(
        &waveform.name,
        &waveform.x,
        &waveform.y,
        presentation.fft.input_options_for_waveform(&waveform.x),
    )
    .ok_or(HardcopySourceError::MissingViewerEvidence(
        "FFT source waveform",
    ))?;
    let data = crate::analysis::fft::data::FftData::from_time_domain_with_normalization(
        &format!("FFT({})", waveform.name),
        &input.samples,
        input.sample_rate,
        presentation.fft.window,
        presentation.fft.normalization,
    );
    if data.points.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence("FFT spectrum"));
    }
    quick_plot_from_series(
        ResultViewer::Fft,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:fft:{}",
                active.run.dataset_id,
                active.run.run_id,
                active.analysis.id,
                waveform.name,
                data.fft_size
            ),
            label: data.name.clone(),
            points: data
                .points
                .iter()
                .map(|point| (point.frequency, point.magnitude))
                .collect(),
        }],
    )
}

pub(super) fn quick_eye_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let waveform = selected_retained_waveform(
        active,
        presentation.fft.selected_source.as_deref(),
        "eye source waveform",
    )?;
    let bit_period = retained_eye_bit_period(&waveform.x, &waveform.y)?;
    let data = crate::analysis::eye_diagram::EyeDataBuilder::new()
        .bit_period(bit_period)
        .ui_count(2)
        .skip_initial(2)
        .build(&waveform.x, &waveform.y);
    if data.traces.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence("eye diagram"));
    }
    let series = data
        .traces
        .iter()
        .enumerate()
        .map(|(index, trace)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:eye:{}:{index}",
                active.run.dataset_id,
                active.run.run_id,
                active.analysis.id,
                waveform.name,
                bit_period.to_bits()
            ),
            label: format!("Eye trace {}", index + 1),
            points: trace
                .time
                .iter()
                .copied()
                .zip(trace.amplitude.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(ResultViewer::Eye, "Results", 0, series)
}

pub(super) fn quick_histogram_plot(
    presentation: &ResultsQuickViewPresentation,
    active: ActiveQuickResult<'_>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let AnalysisResultFamilyMetadata::MonteCarlo { variables, .. } =
        active.analysis.family_metadata.as_ref().ok_or(
            HardcopySourceError::MissingViewerEvidence("Monte Carlo family metadata"),
        )?
    else {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo family metadata",
        ));
    };
    let variable = variables.get(presentation.histogram_selected).ok_or(
        HardcopySourceError::MissingViewerEvidence("selected Monte Carlo variable"),
    )?;
    if variable.samples.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "Monte Carlo samples",
        ));
    }
    let mut builder = crate::analysis::HistogramBuilder::new()
        .name(&variable.name)
        .bin_count(presentation.histogram_bin_count.clamp(1, 1000));
    if presentation.histogram_custom_range {
        let minimum = presentation.histogram_custom_min;
        let maximum = presentation.histogram_custom_max;
        if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
            return Err(HardcopySourceError::InvalidResultRange);
        }
        builder = builder.range(minimum, maximum);
    }
    let histogram = builder.build(&variable.samples);
    let ordinates = match presentation.histogram_mode {
        crate::analysis::HistogramDisplayMode::Count => histogram
            .bins
            .iter()
            .map(|bin| bin.count as f64)
            .collect::<Vec<_>>(),
        crate::analysis::HistogramDisplayMode::Pdf => histogram.pdf(),
        crate::analysis::HistogramDisplayMode::Cdf => histogram.cdf(),
        crate::analysis::HistogramDisplayMode::Percent => histogram
            .bins
            .iter()
            .map(|bin| {
                if histogram.total_count == 0 {
                    0.0
                } else {
                    bin.count as f64 * 100.0 / histogram.total_count as f64
                }
            })
            .collect(),
    };
    quick_plot_from_series(
        ResultViewer::Hist,
        "Results",
        0,
        vec![QuickResultSeries {
            identity: format!(
                "{}:{}:{}:monte-carlo:{}",
                active.run.dataset_id, active.run.run_id, active.analysis.id, variable.name
            ),
            label: histogram.name.clone(),
            points: histogram
                .bins
                .iter()
                .zip(ordinates)
                .map(|(bin, ordinate)| (bin.center(), ordinate))
                .collect(),
        }],
    )
}

pub(super) fn quick_complex_plot(
    active: ActiveQuickResult<'_>,
    viewer: ResultViewer,
) -> Result<SemanticPlot, HardcopySourceError> {
    let series = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| waveform.visible)
        .filter_map(|waveform| waveform.complex.as_ref().map(|complex| (waveform, complex)))
        .map(|(waveform, complex)| QuickResultSeries {
            identity: format!(
                "{}:{}:{}:{}:complex",
                active.run.dataset_id, active.run.run_id, active.analysis.id, waveform.name
            ),
            label: waveform.name.clone(),
            points: complex
                .real
                .iter()
                .copied()
                .zip(complex.imag.iter().copied())
                .collect(),
        })
        .collect();
    quick_plot_from_series(viewer, "Results", 0, series)
}

pub(super) fn selected_retained_waveform<'a>(
    active: ActiveQuickResult<'a>,
    preferred_name: Option<&str>,
    evidence: &'static str,
) -> Result<&'a WaveformData, HardcopySourceError> {
    let mut candidates = active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            waveform.x.len().min(waveform.y.len()) >= crate::analysis::fft::MIN_FFT_SAMPLES
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.name.cmp(&right.name));
    let selected = preferred_name
        .and_then(|name| {
            candidates
                .iter()
                .copied()
                .find(|waveform| waveform.name == name || waveform.name.eq_ignore_ascii_case(name))
                .or_else(|| {
                    let preferred_core = derived_waveform_source_core(name);
                    candidates.iter().copied().find(|waveform| {
                        derived_waveform_source_core(&waveform.name) == preferred_core
                    })
                })
        })
        .or_else(|| candidates.first().copied())
        .ok_or(HardcopySourceError::MissingViewerEvidence(evidence))?;
    let sample_count = selected.x.len().min(selected.y.len());
    if selected
        .x
        .iter()
        .take(sample_count)
        .chain(selected.y.iter().take(sample_count))
        .any(|value| !value.is_finite())
    {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            selected.name.clone(),
        ));
    }
    Ok(selected)
}

pub(super) fn derived_waveform_source_core(name: &str) -> String {
    let trimmed = name.trim().trim_matches('|');
    trimmed
        .strip_prefix("V(")
        .and_then(|value| value.strip_suffix(')'))
        .or_else(|| {
            trimmed
                .strip_prefix("I(")
                .and_then(|value| value.strip_suffix(')'))
        })
        .unwrap_or(trimmed)
        .trim()
        .to_ascii_lowercase()
}

pub(super) fn retained_eye_bit_period(
    time: &[f64],
    values: &[f64],
) -> Result<f64, HardcopySourceError> {
    let sample_count = time.len().min(values.len());
    if sample_count < 8 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let minimum = values
        .iter()
        .take(sample_count)
        .copied()
        .filter(|value| value.is_finite())
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))?;
    let maximum = values
        .iter()
        .take(sample_count)
        .copied()
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))?;
    if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let threshold = (minimum + maximum) * 0.5;
    let edges = crate::analysis::eye_diagram::find_edges(
        &time[..sample_count],
        &values[..sample_count],
        threshold,
    );
    if edges.len() < 3 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let mut rising_times = edges
        .iter()
        .filter(|edge| edge.rising && edge.time.is_finite())
        .map(|edge| edge.time)
        .collect::<Vec<_>>();
    rising_times.sort_by(f64::total_cmp);
    let edge_times = if rising_times.len() >= 3 {
        rising_times
    } else {
        let mut all = edges
            .iter()
            .map(|edge| edge.time)
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        all.sort_by(f64::total_cmp);
        all
    };
    if edge_times.len() < 3 {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    let mut intervals = edge_times
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|interval| interval.is_finite() && *interval > 0.0)
        .collect::<Vec<_>>();
    if intervals.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ));
    }
    intervals.sort_by(f64::total_cmp);
    let period = intervals[intervals.len() / 2];
    if period.is_finite() && period > 0.0 {
        Ok(period)
    } else {
        Err(HardcopySourceError::MissingViewerEvidence(
            "eye transition timing",
        ))
    }
}

pub(super) fn quick_plot_from_series(
    viewer: ResultViewer,
    page: &str,
    pane_id: u64,
    series: Vec<QuickResultSeries>,
) -> Result<SemanticPlot, HardcopySourceError> {
    if series.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence(
            "visible plot series",
        ));
    }
    if series.iter().any(|series| {
        series.points.is_empty()
            || series
                .points
                .iter()
                .any(|(x, y)| !x.is_finite() || !y.is_finite())
    }) {
        return Err(HardcopySourceError::InvalidRetainedWaveform(
            "active viewer series".to_owned(),
        ));
    }
    let x_minimum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.0))
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let x_maximum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.0))
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let y_minimum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.1))
        .min_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let y_maximum = series
        .iter()
        .flat_map(|series| series.points.iter().map(|point| point.1))
        .max_by(f64::total_cmp)
        .ok_or(HardcopySourceError::InvalidResultRange)?;
    let (x_minimum, x_maximum) = nondegenerate_range(x_minimum, x_maximum);
    let (y_minimum, y_maximum) = nondegenerate_range(y_minimum, y_maximum);
    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let mut trace_ids = std::collections::HashSet::new();
    let traces = series
        .into_iter()
        .enumerate()
        .map(|(index, series)| {
            let trace_id = stable_quick_trace_id(viewer, index, &series.identity);
            if !trace_ids.insert(trace_id) {
                return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
            }
            Ok(SemanticPlotTrace {
                trace_id,
                label: series.label,
                paths: clipped_plot_paths(
                    &series.points,
                    x_minimum,
                    x_maximum,
                    y_minimum,
                    y_maximum,
                    plot_width,
                    plot_height,
                )?,
                source_samples: series
                    .points
                    .iter()
                    .map(|(x, y)| (x.to_bits(), y.to_bits()))
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    Ok(SemanticPlot {
        viewer,
        page_id: stable_page_id(page),
        pane_id,
        traces,
        markers: Vec::new(),
        annotations: Vec::new(),
    })
}

pub(super) fn results_quick_view_identity(
    source_key: &str,
    project_id: ProjectId,
    viewer: ResultViewer,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = source_key.as_bytes().to_vec();
    identity_name.extend_from_slice(viewer.label().as_bytes());
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run.run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&analysis.id.to_be_bytes());
    identity_name.extend_from_slice(analysis.result_data_digest().as_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results · {}", viewer.label()),
    )
}

pub(super) fn stable_quick_trace_id(viewer: ResultViewer, index: usize, identity: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-hardcopy-results-trace-v1");
    hasher.update(viewer.label().as_bytes());
    hasher.update((index as u64).to_be_bytes());
    hasher.update(identity.as_bytes());
    let bytes: [u8; 8] = hasher.finalize()[..8]
        .try_into()
        .expect("SHA-256 prefix has fixed length");
    u64::from_be_bytes(bytes)
}

pub(super) const fn is_curve_viewer(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::Bode
            | ResultViewer::Fft
            | ResultViewer::Eye
            | ResultViewer::Hist
            | ResultViewer::Nyquist
            | ResultViewer::Smith
    )
}

pub(super) fn resolve_studio_result_summary(
    source: ActiveStudioPaneHardcopySource<'_>,
    pane: &StudioPane,
    run_id: RunId,
    analysis: &AnalysisResult,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let summary = semantic_result_summary(pane.viewer, analysis)?;
    let digest = canonical_digest(
        b"rspice-hardcopy-studio-result-summary-v1",
        &(source.studio.revision, pane, run_id, analysis.id, &summary),
    )?;
    let identity =
        studio_source_identity(&source.source_key, source.project_id, source.studio, pane)?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        source.scope,
        HardcopySemanticDocument::ResultSummary(summary),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    )
}

pub(super) fn semantic_result_summary(
    viewer: ResultViewer,
    analysis: &AnalysisResult,
) -> Result<SemanticResultSummary, HardcopySourceError> {
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    let mut tables = Vec::new();
    match viewer {
        ResultViewer::Op => {
            if let Some(operating_point) = &analysis.dc_op {
                for (title, values) in [
                    ("Node voltages", &operating_point.node_voltages),
                    ("Branch currents", &operating_point.branch_currents),
                    ("Device power", &operating_point.power_dissipation),
                ] {
                    if !values.is_empty() {
                        tables.push(SemanticTable {
                            title: title.to_owned(),
                            columns: vec![
                                "Quantity".to_owned(),
                                "Value".to_owned(),
                                "Unit".to_owned(),
                            ],
                            rows: values
                                .iter()
                                .map(|value| {
                                    vec![
                                        value.name.clone(),
                                        exact_number(value.value),
                                        value.unit.clone(),
                                    ]
                                })
                                .collect(),
                        });
                    }
                }
            }
            if tables.is_empty()
                && !matches!(
                    analysis.result_payload.as_ref(),
                    Some(AnalysisResultPayload::OperatingPoint { .. })
                )
            {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "operating point",
                ));
            }
        }
        ResultViewer::NoiseContrib => {
            let summary = analysis.noise_summary.as_ref().ok_or(
                HardcopySourceError::MissingViewerEvidence("noise contributor summary"),
            )?;
            tables.push(SemanticTable {
                title: format!(
                    "Noise contributors · {} Hz to {} Hz",
                    exact_number(summary.band.0),
                    exact_number(summary.band.1)
                ),
                columns: vec![
                    "Device".to_owned(),
                    "Mechanism".to_owned(),
                    "Power (V²)".to_owned(),
                    "Share (%)".to_owned(),
                ],
                rows: summary
                    .rows
                    .iter()
                    .map(|row| {
                        vec![
                            row.device.clone(),
                            row.mechanism.clone(),
                            exact_number(row.power),
                            exact_number(row.share_pct),
                        ]
                    })
                    .collect(),
            });
            tables.push(SemanticTable {
                title: "Integrated totals".to_owned(),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec![
                        "Output referred RMS".to_owned(),
                        summary
                            .total_rms
                            .map_or_else(|| "not retained".to_owned(), exact_number),
                    ],
                    vec![
                        "Input referred RMS".to_owned(),
                        summary
                            .input_rms
                            .map_or_else(|| "not retained".to_owned(), exact_number),
                    ],
                ],
            });
        }
        ResultViewer::Contribution => {
            let Some(AnalysisResultPayload::Sensitivity { output, rows, .. }) =
                &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("sensitivity"));
            };
            tables.push(SemanticTable {
                title: format!("Sensitivity of {output}"),
                columns: vec![
                    "Parameter".to_owned(),
                    "Raw".to_owned(),
                    "Normalized".to_owned(),
                ],
                rows: rows
                    .iter()
                    .map(|row| {
                        vec![
                            row.parameter.clone(),
                            exact_number(row.raw),
                            exact_number(row.normalized),
                        ]
                    })
                    .collect(),
            });
        }
        ResultViewer::TransferFunction => {
            let Some(AnalysisResultPayload::TransferFunction {
                input_source,
                output_expression,
                gain,
                input_resistance,
                output_resistance,
                ..
            }) = &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "transfer function",
                ));
            };
            tables.push(SemanticTable {
                title: format!("{output_expression} / {input_source}"),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec!["Gain".to_owned(), format_optional_scalar(*gain)],
                    vec![
                        "Input resistance".to_owned(),
                        format_optional_scalar(*input_resistance),
                    ],
                    vec![
                        "Output resistance".to_owned(),
                        format_optional_scalar(*output_resistance),
                    ],
                ],
            });
        }
        ResultViewer::Specs => {
            if !analysis.measurements.is_empty() {
                tables.push(SemanticTable {
                    title: "Measurements and specifications".to_owned(),
                    columns: vec![
                        "Measurement".to_owned(),
                        "Value".to_owned(),
                        "Expected".to_owned(),
                        "Tolerance".to_owned(),
                        "Status".to_owned(),
                    ],
                    rows: analysis
                        .measurements
                        .iter()
                        .map(|measurement| {
                            vec![
                                measurement.name.clone(),
                                measurement
                                    .value
                                    .map_or_else(|| "not available".to_owned(), exact_number),
                                measurement
                                    .expected
                                    .map_or_else(|| "—".to_owned(), exact_number),
                                measurement
                                    .tolerance
                                    .map_or_else(|| "—".to_owned(), exact_number),
                                if measurement.passed { "pass" } else { "fail" }.to_owned(),
                            ]
                        })
                        .collect(),
                });
            } else if let Some(AnalysisResultPayload::ScalarMeasurements { values }) =
                &analysis.result_payload
            {
                tables.push(SemanticTable {
                    title: "Scalar measurements".to_owned(),
                    columns: vec!["Measurement".to_owned(), "Value".to_owned()],
                    rows: values
                        .iter()
                        .map(|(name, value)| vec![name.clone(), exact_number(*value)])
                        .collect(),
                });
            } else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "measurement/specification",
                ));
            }
        }
        ResultViewer::PoleZero => {
            let Some(AnalysisResultPayload::PoleZero { poles, zeros, gain }) =
                &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("pole-zero"));
            };
            let mut rows = Vec::with_capacity(poles.len() + zeros.len());
            rows.extend(poles.iter().enumerate().map(|(index, value)| {
                vec![
                    format!("P{}", index + 1),
                    exact_number(value.real),
                    exact_number(value.imaginary),
                ]
            }));
            rows.extend(zeros.iter().enumerate().map(|(index, value)| {
                vec![
                    format!("Z{}", index + 1),
                    exact_number(value.real),
                    exact_number(value.imaginary),
                ]
            }));
            tables.push(SemanticTable {
                title: format!("Pole-zero roots · gain {}", exact_number(*gain)),
                columns: vec!["Root".to_owned(), "Real".to_owned(), "Imaginary".to_owned()],
                rows,
            });
        }
        ResultViewer::Manifest => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                "dataset-native Manifest cannot be derived from one analysis".to_owned(),
            ));
        }
        viewer if is_curve_viewer(viewer) => unreachable!("curve viewers resolve as plots"),
        viewer => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                viewer.label().to_owned(),
            ));
        }
    }
    Ok(SemanticResultSummary {
        viewer,
        title: analysis.label.clone(),
        tables,
        payload: analysis.result_payload.clone(),
    })
}

pub(super) fn studio_source_identity(
    source_key: &str,
    project_id: ProjectId,
    studio: &VisualizationStudioState,
    pane: &StudioPane,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = Vec::with_capacity(24);
    identity_name.extend_from_slice(pane.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&pane.id.to_be_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(studio.revision)
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        format!("{} · {}", pane.page, pane.viewer.label()),
    )
}

pub(super) fn exact_number(value: f64) -> String {
    format!("{value:.17e}")
}

pub(super) fn format_optional_scalar(
    scalar: Option<crate::state::TransferFunctionScalarEvidence>,
) -> String {
    scalar.map_or_else(
        || "not requested".to_owned(),
        |value| match value {
            crate::state::TransferFunctionScalarEvidence::Finite(value) => exact_number(value),
            crate::state::TransferFunctionScalarEvidence::PositiveInfinity => {
                "+infinity".to_owned()
            }
            crate::state::TransferFunctionScalarEvidence::NegativeInfinity => {
                "-infinity".to_owned()
            }
        },
    )
}

pub(super) fn map_visualization_error(error: VisualizationRasterError) -> HardcopySourceError {
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
