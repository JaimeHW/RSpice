//! Exact source-set authentication and aggregate placement.

use super::*;

pub fn resolve_hardcopy_source_set_with(
    source_set: &HardcopySourceSet,
    mut resolve_member: impl FnMut(
        &HardcopySourceSetMember,
    ) -> Result<ResolvedHardcopyDocument, HardcopySourceError>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    source_set.validate()?;

    let mut children = Vec::with_capacity(source_set.members().len());
    let mut next_y_um = 0_i64;
    let mut maximum_width_um = 0_i64;
    for (index, member) in source_set.members().iter().enumerate() {
        let resolved = resolve_member(member)?;
        validate_source_set_member_authority(source_set, member, &resolved)?;
        if matches!(
            resolved.semantic_document(),
            HardcopySemanticDocument::Aggregate(_)
        ) {
            return Err(HardcopySourceError::InvalidSourceSet(
                "nested semantic aggregates are not supported".to_owned(),
            ));
        }

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
        let ResolvedHardcopyDocument {
            source_key,
            authority,
            semantic_document,
            ..
        } = resolved;
        children.push(SemanticAggregateChild {
            ordinal,
            source_key,
            display_name: authority.display_name().to_owned(),
            document_id: authority.document_id(),
            revision: authority.revision(),
            content_digest: authority.content_digest(),
            local_bounds: bounds,
            placement_origin: SemanticPoint::new(0, next_y_um),
            page_break_before: index != 0,
            publication_page_label: None,
            document: Box::new(semantic_document),
        });
        next_y_um = next_y_um
            .checked_add(height)
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        if index + 1 != source_set.members().len() {
            next_y_um = next_y_um
                .checked_add(REPORT_PAGE_GAP_UM)
                .ok_or(HardcopySourceError::CoordinateOverflow)?;
        }
    }

    // A per-print-set schematic is numbered by the exact ordered schematic
    // subset in this authenticated aggregate, not by its full source catalog.
    // The override belongs to the aggregate projection and therefore does not
    // mutate or misrepresent the retained child source digest.
    let per_print_set_count = children
        .iter()
        .filter(|child| {
            matches!(
                child.document.as_ref(),
                HardcopySemanticDocument::Schematic(schematic)
                    if schematic.drawing_sheet_page_numbering
                        == Some(rspice_design_model::design_management::SheetPageNumbering::PerPrintSet)
            )
        })
        .count();
    if per_print_set_count > 0 {
        let mut page = 0_usize;
        for child in &mut children {
            if matches!(
                child.document.as_ref(),
                HardcopySemanticDocument::Schematic(schematic)
                    if schematic.drawing_sheet_page_numbering
                        == Some(rspice_design_model::design_management::SheetPageNumbering::PerPrintSet)
            ) {
                page += 1;
                child.publication_page_label = Some(format!("{page} of {per_print_set_count}"));
            }
        }
    }

    let aggregate = SemanticAggregate {
        source_set_digest: source_set.definition_digest(),
        children,
    };
    let semantic_document = HardcopySemanticDocument::Aggregate(aggregate);
    let content_digest = canonical_digest(
        b"rspice-hardcopy-source-set-aggregate-v1",
        &(source_set.definition_digest(), &semantic_document),
    )?;
    let bounds = SemanticBounds::try_new(
        SemanticPoint::new(0, 0),
        SemanticPoint::new(maximum_width_um, next_y_um),
    )?;
    finish_resolved(
        HardcopySourceIdentity::try_new(
            source_set.source_key(),
            source_set.document_id(),
            source_set.revision(),
            source_set.name(),
        )?,
        content_digest,
        source_set.document_kind(),
        source_set.scope().clone(),
        semantic_document,
        bounds,
    )
}

fn validate_source_set_member_authority(
    source_set: &HardcopySourceSet,
    expected: &HardcopySourceSetMember,
    actual: &ResolvedHardcopyDocument,
) -> Result<(), HardcopySourceError> {
    let authority = actual.authority();
    let exact = actual.source_key() == expected.source_key()
        && authority.display_name() == expected.display_name()
        && authority.document_id() == expected.document_id()
        && authority.revision() == expected.revision()
        && authority.content_digest() == expected.content_digest()
        && authority.scope() == expected.scope();
    if !exact {
        return Err(HardcopySourceError::StaleSourceSetMember {
            source_key: expected.source_key().to_owned(),
        });
    }
    if source_set.document_kind() != HardcopyDocumentKind::EngineeringDocument
        && authority.document_kind() != source_set.document_kind()
    {
        return Err(HardcopySourceError::InvalidSourceSet(format!(
            "member {} has kind {:?}, expected {:?}",
            expected.source_key(),
            authority.document_kind(),
            source_set.document_kind()
        )));
    }
    Ok(())
}
