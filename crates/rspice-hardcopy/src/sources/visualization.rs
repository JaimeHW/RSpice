//! Printable pages resolved from a canonical visualization document.

use super::*;
use rspice_app_types::product::{ProjectId, ResultDocumentId};
use rspice_results::report_document::ReportSourceId;

pub fn resolve_visualization_document_source(
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
        .map(source_set_member_from_resolved)
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
    resolved = resolved.with_source_key(source_key)?;
    Ok(resolved)
}

pub fn visualization_document_pane_source_key(
    project_id: ProjectId,
    document_id: ResultDocumentId,
    pane_id: PaneId,
) -> String {
    format!(
        "project:{}:result-document:{}:pane:{}",
        project_id.as_uuid(),
        document_id,
        pane_id.get()
    )
}

fn visualization_document_reference(
    document: &VisualizationDocument,
) -> Result<ReportReferenceSnapshot, HardcopySourceError> {
    let content_digest = document
        .content_digest()
        .map_err(|error| HardcopySourceError::InvalidVisualizationSource(error.to_string()))?;
    ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: document.id(),
        },
        Some(document.revision()),
        content_digest,
        document
            .datasets()
            .iter()
            .map(|dataset| dataset.binding())
            .collect(),
    )
    .map_err(|error| HardcopySourceError::InvalidVisualizationSource(error.to_string()))
}
