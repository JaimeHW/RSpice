//! Pure design document capture for hardcopy publication.

use std::collections::BTreeMap;

use rspice_design_model::design_management::{
    DrawingSheetScale, DrawingSheetTitleFieldId, SheetCatalog, SheetId,
};

use super::*;

pub fn drawing_sheet_title_values(
    identity: &HardcopySourceIdentity,
    format: &SchematicSheetFormat,
    governed: Option<(&SheetCatalog, SheetId)>,
    project_title_block_field_values: Option<&BTreeMap<DrawingSheetTitleFieldId, String>>,
) -> BTreeMap<DrawingSheetTitleFieldId, String> {
    let mut values = BTreeMap::new();
    let source_view = identity.publication.as_ref().map_or_else(
        || identity.display_name.as_str(),
        |publication| publication.document_path.as_str(),
    );
    let project = identity
        .publication
        .as_ref()
        .map_or("Project", |publication| publication.project_name.as_str());
    let (sheet_title, page) = governed.map_or_else(
        || (identity.display_name.clone(), "1 of 1".to_owned()),
        |(catalog, sheet_id)| {
            let title = catalog.find(sheet_id).map_or_else(
                || identity.display_name.clone(),
                |sheet| sheet.name().to_owned(),
            );
            let (page, count) = catalog
                .page_number_and_count(sheet_id)
                .unwrap_or((1, u32::try_from(catalog.sheets().len()).unwrap_or(1)));
            (title, format!("{page} of {count}"))
        },
    );
    values.insert(DrawingSheetTitleFieldId::Project, project.to_owned());
    values.insert(DrawingSheetTitleFieldId::CellView, source_view.to_owned());
    values.insert(DrawingSheetTitleFieldId::SheetTitle, sheet_title);
    values.insert(DrawingSheetTitleFieldId::Page, page);
    values.insert(
        DrawingSheetTitleFieldId::Revision,
        identity
            .publication
            .as_ref()
            .and_then(|publication| publication.revision_label.clone())
            .unwrap_or_else(|| identity.revision.get().to_string()),
    );
    values.insert(
        DrawingSheetTitleFieldId::Format,
        format!(
            "{} · {}",
            format.authored_size.label(),
            format.orientation.label().to_lowercase()
        ),
    );
    values.insert(
        DrawingSheetTitleFieldId::Scale,
        match format.title_block.scale {
            DrawingSheetScale::NotToScale => "NTS".to_owned(),
            DrawingSheetScale::Ratio {
                drawing_units,
                reality_units,
            } => format!("{drawing_units}:{reality_units}"),
        },
    );
    values.insert(
        DrawingSheetTitleFieldId::Date,
        identity
            .publication
            .as_ref()
            .and_then(|publication| publication.publication_date_utc.clone())
            .unwrap_or_else(|| "UNRELEASED".to_owned()),
    );
    if let Some(project_values) = project_title_block_field_values {
        for id in DrawingSheetTitleFieldId::PROJECT_OWNED {
            if let Some(value) = project_values.get(&id) {
                values.insert(id, value.clone());
            }
        }
    }
    values
}

/// Freeze a symbol document after the caller has captured its requested selection.
pub fn resolve_symbol_document(
    identity: HardcopySourceIdentity,
    document: SymbolDocument,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    if !matches!(
        &scope,
        HardcopyScope::Selection | HardcopyScope::CurrentSheet | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let bounds = symbol_bounds(&document)?;
    let digest = canonical_digest(b"rspice-hardcopy-symbol-v1", &document)?;
    finish_resolved(
        identity,
        digest,
        HardcopyDocumentKind::SchematicOrSymbol,
        scope,
        HardcopySemanticDocument::Symbol(document),
        bounds,
    )
}
