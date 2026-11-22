//! Run-level publications over retained analysis evidence and frozen specifications.

use super::*;
use rspice_hardcopy_contract::sources::MAX_HARDCOPY_SOURCE_SET_MEMBERS;
use rspice_results::{
    analysis_result::AnalysisResult, result_digest::ResultDigestEncoding, run::SimulationRun,
    waveform::RetainedWaveform,
};
use rspice_vocabulary::product::{ObjectRevision, ProjectId};
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
