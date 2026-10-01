//! Resolving one document kind to a printable source.
//!
//! Schematics and symbols each become a semantic page a different way,
//! but both obey the same rule: the page is built from the
//! saved document, not from what is currently on screen, and a document that
//! cannot supply the content is refused rather than printed blank. A blank
//! sheet is only ever produced where the schematic genuinely has one.

#[cfg(test)]
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
