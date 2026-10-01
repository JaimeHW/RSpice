//! Resolving one document kind to a printable source.
//!
//! Schematics and symbols each become a semantic page a different way,
//! but both obey the same rule: the page is built from the
//! saved document, not from what is currently on screen, and a document that
//! cannot supply the content is refused rather than printed blank. A blank
//! sheet is only ever produced where the schematic genuinely has one.

use std::collections::BTreeMap;

use super::*;

#[cfg(test)]
pub(super) fn resolve_blank_schematic_sheet(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_blank_schematic_sheet_with_format(identity, scope, None)
}

#[cfg(test)]
pub(crate) fn resolve_blank_schematic_sheet_with_format(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
    drawing_sheet: Option<&SchematicSheetFormat>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_blank_schematic_sheet_with_format_and_project_values(
        identity,
        scope,
        drawing_sheet,
        None,
    )
}

#[cfg(test)]
pub(super) fn resolve_blank_schematic_sheet_with_format_and_project_values(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
    drawing_sheet: Option<&SchematicSheetFormat>,
    project_title_block_field_values: Option<&BTreeMap<DrawingSheetTitleFieldId, String>>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if let Some(project_values) = project_title_block_field_values {
        crate::state::validate_project_drawing_sheet_title_field_values(project_values)
            .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    }
    if !matches!(scope, HardcopyScope::CurrentSheet) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let semantic = SemanticSchematic {
        view_path: identity.source_key.clone(),
        drawing_sheet: drawing_sheet.cloned(),
        drawing_sheet_title_values: drawing_sheet.map_or_else(BTreeMap::new, |format| {
            drawing_sheet_title_values(&identity, format, None, project_title_block_field_values)
        }),
        drawing_sheet_page_numbering: None,
        grid_pitch_units: 10,
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    let digest = canonical_digest(b"rspice-hardcopy-blank-schematic-sheet-v1", &semantic)?;
    let bounds = drawing_sheet.map_or_else(
        || {
            SemanticBounds::try_new(
                SemanticPoint::new(0, 0),
                SemanticPoint::new(
                    BLANK_SCHEMATIC_SHEET_WIDTH_UM,
                    BLANK_SCHEMATIC_SHEET_HEIGHT_UM,
                ),
            )
        },
        drawing_sheet_artwork_bounds,
    )?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        scope,
        HardcopySemanticDocument::Schematic(Box::new(semantic)),
        bounds,
    )
}

/// Resolve every governed schematic sheet in exact catalog order. Each sheet
/// is filtered independently before its authority is pinned into the
/// aggregate, so an assignment can never leak into a neighboring page.
pub fn resolve_all_schematic_sheets(
    source: SchematicSheetSetHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_project_default_drawing_sheet(source.project_default_drawing_sheet)?;
    crate::state::validate_project_drawing_sheet_title_field_values(
        source.project_title_block_field_values,
    )
    .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    source
        .sheet_catalog
        .validate()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    if source.sheet_catalog.sheets().is_empty() {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-sheets scope requires at least one governed sheet".to_owned(),
        ));
    }
    if source.schematic.topology_version() != source.expected_topology_version {
        return Err(HardcopySourceError::StaleSchematic {
            expected: source.expected_topology_version,
            actual: source.schematic.topology_version(),
        });
    }

    let aggregate_source_key = source.identity.source_key.clone();
    let mut resolved_sheets = Vec::with_capacity(source.sheet_catalog.sheets().len());
    for sheet in source.sheet_catalog.sheets() {
        let sheet_identity = schematic_sheet_identity(&source.identity, sheet)?;
        // Resolve empty sheets through the same governed path as populated
        // sheets. Besides eliminating a duplicate resolver, this preserves
        // catalog-aware automatic title values such as "3 / 3" and the
        // effective inherited project format on a deliberately blank page.
        resolved_sheets.push(resolve_schematic_source(SchematicHardcopySource {
            identity: sheet_identity,
            schematic: source.schematic,
            expected_topology_version: source.expected_topology_version,
            symbol_resolver: source.symbol_resolver,
            sheet_catalog: Some(source.sheet_catalog),
            sheet_id: Some(sheet.id()),
            project_default_drawing_sheet: Some(source.project_default_drawing_sheet),
            project_title_block_field_values: Some(source.project_title_block_field_values),
            scope: HardcopyScope::CurrentSheet,
        })?);
    }

    let members = resolved_sheets
        .iter()
        .map(source_set_member_from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let mut set_identity_material = b"rspice-hardcopy-all-schematic-sheets-v1:".to_vec();
    set_identity_material.extend_from_slice(source.identity.document_id.as_uuid().as_bytes());
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &source.identity.document_id.as_uuid(),
            &set_identity_material,
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(source.sheet_catalog.revision())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        compact_display(
            &format!("{} · All sheets", source.identity.display_name),
            "All schematic sheets",
        ),
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_sheets = resolved_sheets.into_iter();
    let mut resolved = resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_sheets.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })?;
    // `AllSheetsOrPanes` is a transient scope of the owning design
    // descriptor, not a separately persisted named set.
    resolved = resolved.with_source_key(aggregate_source_key)?;
    Ok(resolved)
}

pub fn resolve_schematic_source(
    source: SchematicHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if let Some(project_default) = source.project_default_drawing_sheet {
        validate_project_default_drawing_sheet(project_default)?;
    }
    if let Some(project_values) = source.project_title_block_field_values {
        crate::state::validate_project_drawing_sheet_title_field_values(project_values)
            .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    }
    if source.schematic.topology_version() != source.expected_topology_version {
        return Err(HardcopySourceError::StaleSchematic {
            expected: source.expected_topology_version,
            actual: source.schematic.topology_version(),
        });
    }
    if !matches!(
        &source.scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let selection_only = matches!(&source.scope, HardcopyScope::Selection);
    if selection_only && source.schematic.session.selection.is_empty() {
        return Err(HardcopySourceError::EmptySelection);
    }
    if selection_only && !source.schematic.session.selection.probes.is_empty() {
        return Err(HardcopySourceError::ProbeSelectionUnsupported);
    }
    let governed_sheet = match (source.sheet_catalog, source.sheet_id) {
        (Some(catalog), Some(sheet_id)) if matches!(source.scope, HardcopyScope::CurrentSheet) => {
            catalog
                .validate()
                .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
            if catalog.find(sheet_id).is_none() {
                return Err(HardcopySourceError::InvalidSheetPartition(format!(
                    "sheet {sheet_id} is not retained in the supplied catalog"
                )));
            }
            Some((catalog, sheet_id))
        }
        (None, None) => None,
        _ => {
            return Err(HardcopySourceError::InvalidSheetPartition(
                "sheet catalog and sheet identity must be supplied together only for CurrentSheet"
                    .to_owned(),
            ));
        }
    };
    let object_is_in_scope = |object_id: u64| {
        governed_sheet.is_none_or(|(catalog, sheet_id)| {
            // The canvas's existing governed-sheet contract assigns legacy
            // unassigned objects to the active sheet. Reusing it here makes
            // every object publish on exactly one sheet without leakage.
            catalog
                .sheet_for_object(object_id)
                .or(catalog.active_sheet_id())
                == Some(sheet_id)
        })
    };

    let drawing_sheet = governed_sheet
        .and_then(|(catalog, sheet_id)| catalog.find(sheet_id))
        .map(|sheet| {
            source.project_default_drawing_sheet.map_or_else(
                || sheet.page_format().clone(),
                |project_default| {
                    effective_governed_sheet_format(sheet.page_format(), project_default)
                },
            )
        })
        .or_else(|| {
            matches!(
                source.scope,
                HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
            )
            .then(|| source.project_default_drawing_sheet.cloned())
            .flatten()
        });
    let mut semantic = SemanticSchematic {
        view_path: governed_sheet.map_or_else(
            || source.identity.source_key.clone(),
            |(_, sheet_id)| format!("{}:sheet:{sheet_id}", source.identity.source_key),
        ),
        drawing_sheet_title_values: drawing_sheet.as_ref().map_or_else(BTreeMap::new, |format| {
            drawing_sheet_title_values(
                &source.identity,
                format,
                governed_sheet,
                source.project_title_block_field_values,
            )
        }),
        drawing_sheet_page_numbering: governed_sheet
            .map(|(catalog, _)| catalog.settings().page_numbering),
        drawing_sheet,
        grid_pitch_units: source.schematic.document().grid_size.max(1),
        components: Vec::new(),
        wires: Vec::new(),
        buses: Vec::new(),
        bus_taps: Vec::new(),
        junctions: Vec::new(),
        net_labels: Vec::new(),
        design_notes: Vec::new(),
        documentation_shapes: Vec::new(),
    };
    for component in &source.schematic.document().components {
        if !object_is_in_scope(component.id) {
            continue;
        }
        if selection_only
            && !source
                .schematic
                .session
                .selection
                .components
                .contains(&component.id)
        {
            continue;
        }
        let (resolved_symbol, symbol_source) =
            resolve_component_symbol(component, source.symbol_resolver)?;
        semantic.components.push(SemanticComponent {
            component: component.clone(),
            resolved_symbol,
            symbol_source,
        });
    }
    let selected_wire_ids = || {
        source
            .schematic
            .session
            .selection
            .wires
            .iter()
            .copied()
            .chain(
                source
                    .schematic
                    .session
                    .selection
                    .wire_segments
                    .iter()
                    .map(|selection| selection.wire_id),
            )
            .chain(
                source
                    .schematic
                    .session
                    .selection
                    .wire_vertices
                    .iter()
                    .map(|selection| selection.wire_id),
            )
            .collect::<std::collections::HashSet<_>>()
    };
    let selected_wire_ids = selection_only.then(selected_wire_ids);
    semantic.wires.extend(
        source
            .schematic
            .document()
            .wires
            .iter()
            .filter(|wire| {
                object_is_in_scope(wire.id)
                    && selected_wire_ids
                        .as_ref()
                        .is_none_or(|selected| selected.contains(&wire.id))
            })
            .cloned(),
    );
    semantic.buses.extend(
        source
            .schematic
            .document()
            .buses
            .iter()
            .filter(|bus| {
                object_is_in_scope(bus.id)
                    && (!selection_only
                        || source.schematic.session.selection.buses.contains(&bus.id))
            })
            .cloned(),
    );
    semantic.bus_taps.extend(
        source
            .schematic
            .document()
            .bus_taps
            .iter()
            .filter(|tap| {
                object_is_in_scope(tap.id)
                    && (!selection_only
                        || source
                            .schematic
                            .session
                            .selection
                            .bus_taps
                            .contains(&tap.id))
            })
            .cloned(),
    );
    semantic.junctions.extend(
        source
            .schematic
            .document()
            .junctions
            .iter()
            .filter(|junction| {
                object_is_in_scope(junction.id)
                    && (!selection_only
                        || source
                            .schematic
                            .session
                            .selection
                            .junctions
                            .iter()
                            .any(|selection| selection.pos == junction.pos))
            })
            .copied(),
    );
    semantic.net_labels.extend(
        source
            .schematic
            .document()
            .net_labels
            .iter()
            .filter(|label| {
                object_is_in_scope(label.id)
                    && (!selection_only
                        || source
                            .schematic
                            .session
                            .selection
                            .net_labels
                            .contains(&label.id))
            })
            .cloned(),
    );
    semantic.design_notes.extend(
        source
            .schematic
            .document()
            .design_notes
            .iter()
            .filter(|note| {
                object_is_in_scope(note.id)
                    && (!selection_only
                        || source
                            .schematic
                            .session
                            .selection
                            .design_notes
                            .contains(&note.id))
            })
            .cloned(),
    );
    semantic.documentation_shapes.extend(
        source
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .filter(|shape| {
                object_is_in_scope(shape.id)
                    && (!selection_only
                        || source
                            .schematic
                            .session
                            .selection
                            .documentation_shapes
                            .contains(&shape.id))
            })
            .cloned(),
    );
    let content_empty = semantic_is_empty(&semantic);
    if content_empty && (selection_only || semantic.drawing_sheet.is_none()) {
        return Err(HardcopySourceError::EmptyContent);
    }

    let bounds = if content_empty {
        drawing_sheet_artwork_bounds(
            semantic
                .drawing_sheet
                .as_ref()
                .expect("empty authored sheet was validated above"),
        )?
    } else {
        let content_bounds = schematic_bounds(&semantic)?;
        semantic
            .drawing_sheet
            .as_ref()
            .map_or(Ok(content_bounds), |format| {
                drawing_sheet_artwork_bounds(format)
                    .map(|sheet_bounds| union_bounds(content_bounds, sheet_bounds))
            })?
    };
    let digest = canonical_digest(b"rspice-hardcopy-schematic-v1", &semantic)?;
    finish_resolved(
        source.identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        source.scope,
        HardcopySemanticDocument::Schematic(Box::new(semantic)),
        bounds,
    )
}

fn effective_governed_sheet_format(
    sheet_format: &SchematicSheetFormat,
    project_default: &SchematicSheetFormat,
) -> SchematicSheetFormat {
    match sheet_format.inheritance {
        crate::state::DrawingSheetInheritance::ProjectDefault => {
            project_default.with_target_sheet_title_fields(sheet_format)
        }
        crate::state::DrawingSheetInheritance::Explicit
        | crate::state::DrawingSheetInheritance::UserDefault => sheet_format.clone(),
    }
}

fn validate_project_default_drawing_sheet(
    format: &SchematicSheetFormat,
) -> Result<(), HardcopySourceError> {
    format
        .validate()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    if format.inheritance != crate::state::DrawingSheetInheritance::ProjectDefault {
        return Err(HardcopySourceError::InvalidSheetPartition(
            "project drawing-sheet default has non-default inheritance".to_owned(),
        ));
    }
    Ok(())
}

pub fn resolve_symbol_source(
    source: SymbolHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(
        &source.scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(source.scope));
    }
    let document = if matches!(&source.scope, HardcopyScope::Selection) {
        let selection = source
            .selection
            .ok_or(HardcopySourceError::EmptySelection)?;
        selected_symbol_document(source.document, selection)?
    } else {
        source.document.clone()
    };
    resolve_symbol_document(source.identity, document, source.scope)
}
