//! Schematic hardcopy resolution over borrowed design content and captured selection.

use super::*;
use rspice_app_types::product::ObjectRevision;
use rspice_design::resolved_symbol::{ResolvedSymbolIssueKind, ResolvedSymbolSource};
use rspice_design::schematic::{component_type::ComponentType, owned::Schematic};
use rspice_design::symbol::SymbolPin;
use rspice_design::symbol_resolver::SymbolResolver;
use rspice_design_model::design_management::{
    DesignSheet, DrawingSheetInheritance, DrawingSheetTitleFieldId, SheetCatalog, SheetId,
    validate_project_drawing_sheet_title_field_values,
};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

/// Durable objects selected for publication. Wire handles name their parent
/// wire; junctions retain the editor's position-based selection contract.
pub struct SchematicHardcopySelection<'a> {
    pub components: &'a HashSet<u64>,
    pub wires: HashSet<u64>,
    pub junctions: Vec<Point>,
    pub buses: &'a HashSet<u64>,
    pub bus_taps: &'a HashSet<u64>,
    pub net_labels: &'a HashSet<u64>,
    pub design_notes: &'a HashSet<u64>,
    pub documentation_shapes: &'a HashSet<u64>,
    pub has_probes: bool,
}

impl SchematicHardcopySelection<'_> {
    fn is_empty(&self) -> bool {
        self.components.is_empty()
            && self.wires.is_empty()
            && self.junctions.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.net_labels.is_empty()
            && self.design_notes.is_empty()
            && self.documentation_shapes.is_empty()
            && !self.has_probes
    }
}

pub struct SchematicHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub schematic: &'a Schematic,
    pub selection: Option<SchematicHardcopySelection<'a>>,
    pub expected_topology_version: u64,
    pub symbol_resolver: Option<&'a SymbolResolver<'a, Schematic>>,
    /// Optional governed multi-sheet partition. Absence means the legacy
    /// single-sheet document. When present, `sheet_id` must name an exact
    /// retained catalog sheet and only objects owned by that sheet resolve.
    pub sheet_catalog: Option<&'a SheetCatalog>,
    pub sheet_id: Option<SheetId>,
    /// Current project default used by the canvas for ungoverned documents
    /// and for governed sheets that follow the project default.
    pub project_default_drawing_sheet: Option<&'a SchematicSheetFormat>,
    /// Canonical project-owned values used by every sheet title block.
    pub project_title_block_field_values:
        Option<&'a std::collections::BTreeMap<DrawingSheetTitleFieldId, String>>,
    pub scope: HardcopyScope,
}

pub struct SchematicSheetSetHardcopySource<'a> {
    pub identity: HardcopySourceIdentity,
    pub schematic: &'a Schematic,
    pub expected_topology_version: u64,
    pub symbol_resolver: Option<&'a SymbolResolver<'a, Schematic>>,
    pub sheet_catalog: &'a SheetCatalog,
    pub project_default_drawing_sheet: &'a SchematicSheetFormat,
    pub project_title_block_field_values:
        &'a std::collections::BTreeMap<DrawingSheetTitleFieldId, String>,
}

/// Resolve every governed schematic sheet in exact catalog order. Each sheet
/// is filtered independently before its authority is pinned into the
/// aggregate, so an assignment can never leak into a neighboring page.
pub fn resolve_all_schematic_sheets(
    source: SchematicSheetSetHardcopySource<'_>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    validate_project_default_drawing_sheet(source.project_default_drawing_sheet)?;
    validate_project_drawing_sheet_title_field_values(source.project_title_block_field_values)
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
            selection: None,
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
        validate_project_drawing_sheet_title_field_values(project_values)
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
    let selection = source.selection.as_ref().filter(|_| selection_only);
    if selection_only && selection.is_none_or(SchematicHardcopySelection::is_empty) {
        return Err(HardcopySourceError::EmptySelection);
    }
    if selection.is_some_and(|selection| selection.has_probes) {
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
            && !selection
                .expect("validated selection")
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
    let selected_wire_ids = selection.map(|selection| &selection.wires);
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
                        || selection
                            .expect("validated selection")
                            .buses
                            .contains(&bus.id))
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
                        || selection
                            .expect("validated selection")
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
                        || selection
                            .expect("validated selection")
                            .junctions
                            .contains(&junction.pos))
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
                        || selection
                            .expect("validated selection")
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
                        || selection
                            .expect("validated selection")
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
                        || selection
                            .expect("validated selection")
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
    resolve_semantic_source(
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
        DrawingSheetInheritance::ProjectDefault => {
            project_default.with_target_sheet_title_fields(sheet_format)
        }
        DrawingSheetInheritance::Explicit | DrawingSheetInheritance::UserDefault => {
            sheet_format.clone()
        }
    }
}

fn validate_project_default_drawing_sheet(
    format: &SchematicSheetFormat,
) -> Result<(), HardcopySourceError> {
    format
        .validate()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    if format.inheritance != DrawingSheetInheritance::ProjectDefault {
        return Err(HardcopySourceError::InvalidSheetPartition(
            "project drawing-sheet default has non-default inheritance".to_owned(),
        ));
    }
    Ok(())
}

pub fn schematic_sheet_identity(
    base: &HardcopySourceIdentity,
    sheet: &DesignSheet,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_material = b"rspice-hardcopy-schematic-sheet-v1:".to_vec();
    identity_material.extend_from_slice(sheet.id().as_uuid().as_bytes());
    let mut identity = HardcopySourceIdentity::try_new(
        format!("{}:sheet:{}", base.source_key, sheet.id()),
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &base.document_id.as_uuid(),
            &identity_material,
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(sheet.revision())
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        compact_display(
            &format!("{} · {}", base.display_name, sheet.name()),
            "Schematic sheet",
        ),
    )?;
    identity.publication.clone_from(&base.publication);
    Ok(identity)
}

fn resolve_component_symbol(
    component: &Component,
    resolver: Option<&SymbolResolver<'_, Schematic>>,
) -> Result<(Option<SymbolDocument>, Option<SemanticSymbolSource>), HardcopySourceError> {
    if component.kind != ComponentType::CellInstance {
        return Ok((None, None));
    }
    let binding = component.library_cell.as_ref().ok_or_else(|| {
        HardcopySourceError::UnresolvedCellSymbol {
            component_id: component.id,
            reason: "cell instance has no library/cell/view binding".to_owned(),
        }
    })?;
    if binding.is_executable_builtin() {
        // Compiled catalog devices are not authored project masters. Preserve
        // the instance for deterministic catalog/fallback rendering.
        return Ok((None, None));
    }
    let resolver = resolver.ok_or_else(|| HardcopySourceError::UnresolvedCellSymbol {
        component_id: component.id,
        reason: "no symbol resolver was supplied for the active project snapshot".to_owned(),
    })?;
    let resolved = resolver.resolve_binding(binding).ok_or_else(|| {
        HardcopySourceError::UnresolvedCellSymbol {
            component_id: component.id,
            reason: format!(
                "no authored or generated symbol is retained for {}/{}",
                binding.library, binding.cell
            ),
        }
    })?;
    if resolved
        .issues()
        .iter()
        .any(|issue| issue.kind == ResolvedSymbolIssueKind::InvalidMetadata)
    {
        return Err(HardcopySourceError::InvalidAuthoredSymbol(component.id));
    }
    let source = match resolved.source() {
        ResolvedSymbolSource::Authored => SemanticSymbolSource::Authored,
        ResolvedSymbolSource::Generated => SemanticSymbolSource::Generated,
    };
    let mut printable_document = resolved.document().clone();
    // The reconciled pin contract, not orphan metadata, is what the
    // schematic renderer exposes. Freeze exactly that connectable set.
    printable_document.pins = resolved
        .connectable_pins()
        .map(|pin| SymbolPin::new(pin.name.clone(), pin.direction, Some(pin.offset)))
        .collect();
    Ok((Some(printable_document), Some(source)))
}

#[cfg(test)]
mod tests;
