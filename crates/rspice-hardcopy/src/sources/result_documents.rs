//! Run-level publications over retained analysis evidence and frozen specifications.

use super::*;
use rspice_app_types::product::{ObjectRevision, ProjectId};
use rspice_hardcopy_contract::sources::MAX_HARDCOPY_SOURCE_SET_MEMBERS;
use rspice_results::{
    analysis_result::AnalysisResult,
    manifest::ManifestViewModel,
    result_digest::ResultDigestEncoding,
    run::SimulationRun,
    specification::SpecEntry,
    specification::report::{resolved_specifications, result_rows},
    waveform::RetainedWaveform,
};
use uuid::Uuid;

fn require_terminal_run<A>(run: &SimulationRun<A>) -> Result<(), HardcopySourceError> {
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "dataset {} belongs to a non-terminal run",
            run.dataset_id,
        )));
    }
    Ok(())
}

pub(super) fn validate_result_specifications(
    specs: &[SpecEntry],
) -> Result<(), HardcopySourceError> {
    if specs.len() > 10_000 {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared specification count exceeds the governed limit".to_owned(),
        ));
    }
    for (index, spec) in specs.iter().enumerate() {
        spec.validate().map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "prepared specification {index} is invalid: {error}"
            ))
        })?;
    }
    Ok(())
}

/// Resolve every analysis strip retained in a stacked Results view. Each
/// strip remains an independently authenticated page group so publication
/// cannot flatten incompatible axes or silently omit background analyses.
pub fn resolve_results_quick_view_stack<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    run: &SimulationRun<A>,
    presentation: &ResultsQuickViewPresentation,
    is_visible: fn(&W) -> bool,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    require_terminal_run(run)?;
    presentation.validate()?;
    if !rspice_results::result_presentation::viewer_uses_wave_stack(presentation.viewer())
        || run.analyses.len() < 2
        || run.analyses.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS
    {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "prepared stacked Results view has an invalid analysis count or viewer".to_owned(),
        ));
    }
    let mut resolved_strips = Vec::with_capacity(run.analyses.len());
    for analysis in run.analyses.iter().map(AsRef::as_ref) {
        if !analysis.success {
            return Err(HardcopySourceError::UnretainedResult(format!(
                "displayed analysis {} is unsuccessful",
                analysis.id
            )));
        }
        resolved_strips.push(resolve_results_quick_view_parts(
            format!("{source_key}:analysis:{}", analysis.id),
            project_id,
            HardcopyScope::ActivePlotDocument,
            &RetainedQuickViewSource::try_new(run, analysis.id, is_visible)?,
            presentation,
        )?);
    }

    let source_set_digest = canonical_digest(
        b"rspice-hardcopy-results-analysis-stack-set-v1",
        &(
            run.dataset_id,
            presentation.viewer(),
            run.analyses
                .iter()
                .map(AsRef::as_ref)
                .map(|analysis| {
                    (
                        analysis.id,
                        analysis
                            .result_data_ref()
                            .digest(ResultDigestEncoding::CURRENT),
                    )
                })
                .collect::<Vec<_>>(),
        ),
    )?;
    let mut children = Vec::with_capacity(resolved_strips.len());
    let mut next_y_um = 0_i64;
    let mut maximum_width_um = 0_i64;
    let strip_count = resolved_strips.len();
    for (index, resolved) in resolved_strips.into_iter().enumerate() {
        let bounds = resolved.bounds();
        let width = bounds
            .maximum
            .x_um
            .checked_sub(bounds.minimum.x_um)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let height = bounds
            .maximum
            .y_um
            .checked_sub(bounds.minimum.y_um)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        maximum_width_um = maximum_width_um.max(width);
        let ordinal = u32::try_from(index).map_err(|_| HardcopySourceError::CoordinateOverflow)?;
        children.push(resolved.into_aggregate_child(
            ordinal,
            SemanticPoint::new(0, next_y_um),
            Some(format!("{} of {strip_count}", index + 1)),
        ));
        next_y_um = next_y_um
            .checked_add(height)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        if index + 1 != strip_count {
            next_y_um = next_y_um
                .checked_add(REPORT_PAGE_GAP_UM)
                .ok_or(HardcopySourceError::CoordinateOverflow)?;
        }
    }
    let semantic_document = HardcopySemanticDocument::Aggregate(SemanticAggregate {
        source_set_digest,
        children,
    });
    let content_digest = canonical_digest(
        b"rspice-hardcopy-results-analysis-stack-v1",
        &(
            run.dataset_id,
            run.run_id,
            run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT),
            presentation.viewer(),
            &semantic_document,
        ),
    )?;
    resolve_semantic_source(
        results_stack_identity(&source_key, project_id, run, presentation.viewer())?,
        content_digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(maximum_width_um, next_y_um),
        )?,
    )
}

pub fn resolve_results_manifest_source<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    run: &SimulationRun<A>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    require_terminal_run(run)?;
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }

    let manifest = ManifestViewModel::from_run(run, |analysis| analysis.as_ref().is_live_partial());
    let semantic_document =
        HardcopySemanticDocument::ResultSummary(Box::new(semantic_manifest_summary(&manifest)));
    let dataset_digest = run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT);
    let digest = canonical_digest(
        b"rspice-hardcopy-results-manifest-v1",
        &(
            run.dataset_id,
            run.run_id,
            dataset_digest,
            &semantic_document,
        ),
    )?;
    let identity = results_manifest_identity(&source_key, project_id, run)?;

    resolve_semantic_source(
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

pub fn resolve_results_specs_source<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    source_key: String,
    project_id: ProjectId,
    scope: HardcopyScope,
    run: &SimulationRun<A>,
    specs: &[SpecEntry],
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    require_terminal_run(run)?;
    validate_result_specifications(specs)?;
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let table = specifications_table(run, specs);
    if table.rows.is_empty() {
        return Err(HardcopySourceError::MissingViewerEvidence("specifications"));
    }
    let semantic_document =
        HardcopySemanticDocument::ResultSummary(Box::new(SemanticResultSummary {
            viewer: ResultViewer::Specs,
            title: table.title.clone(),
            tables: vec![table],
            payload: None,
        }));
    let content_digest = canonical_digest(
        b"rspice-hardcopy-results-specifications-v1",
        &(
            run.dataset_id,
            run.run_id,
            run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT),
            specs,
            &semantic_document,
        ),
    )?;
    resolve_semantic_source(
        results_specs_identity(&source_key, project_id, run, specs)?,
        content_digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        semantic_document,
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    )
}

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

pub fn results_manifest_identity<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    source_key: &str,
    project_id: ProjectId,
    run: &SimulationRun<A>,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = Vec::with_capacity(80);
    identity_name.extend_from_slice(b"rspice-results-manifest-v1");
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run.run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(
        run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT)
            .as_bytes(),
    );
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results · {} · Manifest", run.label),
    )
}

pub fn results_specs_identity<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    source_key: &str,
    project_id: ProjectId,
    run: &SimulationRun<A>,
    specs: &[SpecEntry],
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let specs_digest = canonical_digest(b"rspice-results-specifications-v1", &specs)?;
    let mut identity_name = Vec::with_capacity(112);
    identity_name.extend_from_slice(b"rspice-results-specifications-v1");
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(run.run_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(
        run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT)
            .as_bytes(),
    );
    identity_name.extend_from_slice(specs_digest.as_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results · {} · Specifications", run.label),
    )
}

pub fn results_stack_identity<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    source_key: &str,
    project_id: ProjectId,
    run: &SimulationRun<A>,
    viewer: ResultViewer,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let stack_digest = canonical_digest(
        b"rspice-results-analysis-stack-identity-v1",
        &(
            run.dataset_id,
            viewer,
            run.analyses
                .iter()
                .map(AsRef::as_ref)
                .map(|analysis| {
                    (
                        analysis.id,
                        analysis
                            .result_data_ref()
                            .digest(ResultDigestEncoding::CURRENT),
                    )
                })
                .collect::<Vec<_>>(),
        ),
    )?;
    let mut identity_name = Vec::with_capacity(96);
    identity_name.extend_from_slice(b"rspice-results-analysis-stack-v1");
    identity_name.extend_from_slice(run.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(stack_digest.as_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::INITIAL,
        format!("Results · {} · {}", run.label, viewer.label()),
    )
}

/// Renderer-neutral table projection shared with semantic hardcopy. This is
/// deliberately run-level: the sheet judges every retained source, not only
/// whichever analysis happens to be selected globally.
///
/// `workspace_specs` is the legacy fallback only, exactly as in
/// the CSV projection: what is printed is the contract the run froze.
fn specifications_table<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &SimulationRun<A>,
    workspace_specs: &[SpecEntry],
) -> SemanticTable {
    let specs = resolved_specifications(run, workspace_specs);
    let result_rows = result_rows(run, &specs);
    let rows = result_rows
        .iter()
        .map(|row| {
            let spec = specs
                .iter()
                .find(|spec| spec.measurement.eq_ignore_ascii_case(&row.measurement));
            let bounds = spec.map_or_else(
                || "not specified".to_owned(),
                |spec| match (spec.min, spec.max) {
                    (Some(minimum), Some(maximum)) => {
                        format!(
                            "{} <= value <= {}",
                            exact_value(minimum),
                            exact_value(maximum)
                        )
                    }
                    (Some(minimum), None) => format!("value >= {}", exact_value(minimum)),
                    (None, Some(maximum)) => format!("value <= {}", exact_value(maximum)),
                    (None, None) => "tracked without bounds".to_owned(),
                },
            );
            let scope = spec
                .map(|spec| {
                    serde_json::to_string(&spec.scope).unwrap_or_else(|_| "unavailable".to_owned())
                })
                .unwrap_or_default();
            vec![
                row.measurement.clone(),
                row.expression.clone(),
                row.value.map_or_else(String::new, exact_value),
                bounds,
                row.margin.map_or_else(String::new, exact_value),
                row.unit.clone(),
                scope,
                row.worst_corner.clone().unwrap_or_default(),
                row.status.label().to_owned(),
                row.detail.clone(),
            ]
        })
        .collect();
    SemanticTable {
        title: format!("Specifications · {}", run.label),
        columns: [
            "Measurement",
            "Expression",
            "Value",
            "Bounds",
            "Margin",
            "Unit",
            "Scope",
            "Worst corner",
            "Status",
            "Detail",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        rows,
    }
}

fn exact_value(value: f64) -> String {
    format!("{value:.17e}")
}
