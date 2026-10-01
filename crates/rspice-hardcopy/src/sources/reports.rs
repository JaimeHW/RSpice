//! Report reference authentication and frozen figure preparation.

use super::*;
use rspice_results::report_document::{
    ReportDocument, ReportReferenceCurrentness, ReportReferenceInventory, ReportReferenceMode,
};

pub struct ReportHardcopySource<'a> {
    pub source_key: String,
    pub document: &'a ReportDocument,
    /// Exact retained live-source inventory for linked references. Frozen
    /// blocks authenticate from their embedded artifact and do not require it.
    pub reference_inventory: Option<&'a ReportReferenceInventory>,
    pub scope: HardcopyScope,
}

pub fn resolve_report_source(
    source: ReportHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_label("source key", &source.source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &source.scope,
        HardcopyScope::CompleteReport | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    source
        .document
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    let record = source
        .document
        .revision_record(source.document.id(), source.document.revision())
        .ok_or_else(|| HardcopySourceError::UnretainedReportRevision(source.document.revision()))?;
    if record.snapshot().pages().is_empty() {
        return Err(HardcopySourceError::EmptyContent);
    }
    let mut authenticated_references = Vec::new();
    let mut figure_blocks = Vec::new();
    let mut contains_linked_reference = false;
    for block in record
        .snapshot()
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
    {
        let Some(reference) = block.kind().reference() else {
            continue;
        };
        if let ReportBlockKind::PlotFigure(figure) = block.kind() {
            figure_blocks.push((block.id(), figure.clone()));
        }
        contains_linked_reference |= matches!(reference, ReportReferenceMode::Linked { .. });
        authenticated_references.push(SemanticReportReference {
            block_id: block.id(),
            reference: reference.clone(),
        });
    }
    let empty_inventory = ReportReferenceInventory::default();
    let inventory = match (contains_linked_reference, source.reference_inventory) {
        (true, None) => {
            return Err(HardcopySourceError::ReportReferenceInventoryRequired);
        }
        (_, Some(inventory)) => inventory,
        (false, None) => &empty_inventory,
    };
    inventory
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    let audit = source
        .document
        .audit_references(inventory)
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    if audit.entries.len() != authenticated_references.len() {
        return Err(HardcopySourceError::InvalidReportSource(
            "reference audit does not cover every referenced report block".to_owned(),
        ));
    }
    for entry in &audit.entries {
        if !matches!(
            entry.currentness,
            ReportReferenceCurrentness::Current | ReportReferenceCurrentness::Frozen
        ) {
            return Err(HardcopySourceError::UnauthenticatedReportReference {
                block_id: entry.block_id,
                currentness: entry.currentness,
            });
        }
    }
    let mut figures = Vec::with_capacity(figure_blocks.len());
    for (block_id, figure) in figure_blocks {
        let artifact = match &figure.reference {
            ReportReferenceMode::Frozen { artifact, .. } => artifact,
            ReportReferenceMode::Linked { .. } => {
                let matches = inventory
                    .figure_artifacts
                    .iter()
                    .filter(|artifact| artifact.block_id() == block_id)
                    .collect::<Vec<_>>();
                let [resolved] = matches.as_slice() else {
                    return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
                        block_id,
                        kind: "linked plot figure",
                        reason: if matches.is_empty() {
                            "the exact linked figure payload is absent from the retained source inventory"
                                .to_owned()
                        } else {
                            "the retained source inventory contains ambiguous linked figure payloads"
                                .to_owned()
                        },
                    });
                };
                if resolved.source() != figure.reference.snapshot()
                    || figure.source_locator.as_ref() != Some(resolved.source_locator())
                {
                    return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
                        block_id,
                        kind: "linked plot figure",
                        reason: "the resolved figure payload does not match the block source snapshot and page/pane locator"
                            .to_owned(),
                    });
                }
                resolved.artifact()
            }
        };
        let (width_pixels, height_pixels) = validate_frozen_report_png(block_id, artifact)?;
        figures.push(SemanticReportFigure {
            block_id,
            artifact_digest: artifact.content_digest(),
            media_type: artifact.media_type().to_owned(),
            payload: artifact.payload().to_vec(),
            width_pixels,
            height_pixels,
            caption: figure.caption,
            alternative_text: figure.alternative_text,
            sizing: figure.sizing,
        });
    }
    let page_count = i64::try_from(record.snapshot().pages().len())
        .map_err(|_| HardcopySourceError::CoordinateOverflow)?;
    let height = REPORT_PAGE_HEIGHT_UM
        .checked_mul(page_count)
        .and_then(|value| {
            REPORT_PAGE_GAP_UM
                .checked_mul(page_count.saturating_sub(1))
                .and_then(|gaps| value.checked_add(gaps))
        })
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let identity = HardcopySourceIdentity::try_new(
        source.source_key,
        HardcopyDocumentId::try_from_uuid(source.document.id().as_uuid())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        source.document.revision(),
        source.document.title(),
    )?;
    finish_resolved(
        identity,
        record.snapshot_digest(),
        HardcopyDocumentKind::Report,
        source.scope,
        HardcopySemanticDocument::Report(SemanticReport {
            pages: record.snapshot().pages().to_vec(),
            authenticated_references,
            figures,
        }),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, height),
        )?,
    )
}
