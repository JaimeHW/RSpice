//! Test-only resolution helpers for retained hardcopy source contracts.

use std::collections::BTreeMap;

use super::*;
use rspice_hardcopy::sources::resolve_semantic_source;

/// US Letter landscape, the page a blank sheet without a drawing format takes.
const BLANK_SCHEMATIC_SHEET_WIDTH_UM: i64 = 279_400;
const BLANK_SCHEMATIC_SHEET_HEIGHT_UM: i64 = 215_900;

/// A resolved schematic sheet with nothing authored on it, for tests that
/// need a document of a given identity and not any particular content.
pub(crate) fn resolve_blank_schematic_sheet(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_blank_schematic_sheet_with_format(identity, scope, None)
}

pub(crate) fn resolve_blank_schematic_sheet_with_format(
    identity: HardcopySourceIdentity,
    scope: HardcopyScope,
    drawing_sheet: Option<&SchematicSheetFormat>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(scope, HardcopyScope::CurrentSheet) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let semantic = SemanticSchematic {
        view_path: identity.source_key.clone(),
        drawing_sheet: drawing_sheet.cloned(),
        drawing_sheet_title_values: drawing_sheet.map_or_else(BTreeMap::new, |format| {
            drawing_sheet_title_values(&identity, format, None, None)
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
        authored_sheet_bounds,
    )?;
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        scope,
        HardcopySemanticDocument::Schematic(Box::new(semantic)),
        bounds,
    )
}
