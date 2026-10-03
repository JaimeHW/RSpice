//! Hardcopy rendering is owned by the headless service.

pub use rspice_hardcopy::render::*;

#[cfg(test)]
use rspice_hardcopy::sources::ResolvedHardcopyDocument;

#[cfg(test)]
pub(crate) fn source_metadata(
    source: &ResolvedHardcopyDocument,
    creator: impl Into<String>,
) -> Result<HardcopySceneMetadata, HardcopyRenderError> {
    let mut metadata = HardcopySceneMetadata::try_new(source.authority().display_name(), creator)?;
    metadata.set_provenance_lines(vec![format!(
        "source {} · document {} · revision {} · digest {}",
        source.source_key(),
        source.authority().document_id(),
        source.authority().revision().get(),
        source.authority().content_digest()
    )])?;
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::super::sources::{
        HardcopySemanticDocument, SchematicHardcopySource, resolve_hardcopy_source_set_with,
        resolve_schematic_source, schematic_sheet_identity, source_set_member_from_resolved,
    };
    use super::*;
    use crate::product::ObjectRevision;
    use crate::state::{
        DrawingSheetTitleFieldId, SchematicState, SheetCatalog, SheetDefinition,
        SheetPageNumbering, SheetPortPolicy, SheetTemplate,
    };
    use rspice_hardcopy_contract::sources::{HardcopySourceIdentity, HardcopySourceSet};
    use rspice_hardcopy_contract::{
        HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope, SchematicHardcopySetup,
    };

    #[test]
    fn named_print_set_projects_per_set_sheet_numbers_without_mutating_child_authority() {
        let sheet = |name: &str| SheetDefinition {
            name: name.to_owned(),
            template: SheetTemplate::AnalogSchematic,
            port_policy: SheetPortPolicy::TypedOffSheetPorts,
            explicit_page_number: None,
        };
        let schematic = SchematicState::default();
        let mut catalog = SheetCatalog::default();
        let first = catalog.create_sheet(sheet("First"), None).unwrap();
        let second = catalog.create_sheet(sheet("Second"), Some(first)).unwrap();
        let third = catalog.create_sheet(sheet("Third"), Some(second)).unwrap();
        let mut settings = catalog.settings().clone();
        settings.page_numbering = SheetPageNumbering::PerPrintSet;
        catalog.set_settings(catalog.revision(), settings).unwrap();
        let project_settings = crate::state::DrawingSheetProjectSettings::default();
        let base_identity = HardcopySourceIdentity::try_new(
            "named-set-schematic",
            HardcopyDocumentId::new(),
            ObjectRevision::INITIAL,
            "Active document",
        )
        .unwrap();
        let mut selected = [second, third]
            .into_iter()
            .map(|sheet_id| {
                resolve_schematic_source(SchematicHardcopySource {
                    identity: schematic_sheet_identity(
                        &base_identity,
                        catalog.find(sheet_id).unwrap(),
                    )
                    .unwrap(),
                    schematic: schematic.editor_ref().design,
                    selection: None,
                    expected_topology_version: schematic.topology_version(),
                    symbol_resolver: None,
                    sheet_catalog: Some(&catalog),
                    sheet_id: Some(sheet_id),
                    project_default_drawing_sheet: Some(&project_settings.default_format),
                    project_title_block_field_values: Some(
                        &project_settings.title_block_field_values,
                    ),
                    scope: HardcopyScope::CurrentSheet,
                })
                .unwrap()
            })
            .collect::<Vec<_>>();
        for (resolved, expected) in selected.iter().zip(["2 of 3", "3 of 3"]) {
            let HardcopySemanticDocument::Schematic(schematic) = resolved.semantic_document()
            else {
                panic!("expected governed schematic sheet")
            };
            assert_eq!(
                schematic
                    .drawing_sheet_title_values
                    .get(&DrawingSheetTitleFieldId::Page)
                    .map(String::as_str),
                Some(expected),
                "the retained child keeps its catalog-relative source semantics"
            );
        }
        let members = selected
            .iter()
            .map(source_set_member_from_resolved)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let source_set = HardcopySourceSet::try_new(
            HardcopyDocumentId::new(),
            ObjectRevision::INITIAL,
            "Release subset",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::NamedPrintSet("Release subset".to_owned()),
            members,
        )
        .unwrap();
        let mut selected = selected.drain(..);
        let aggregate = resolve_hardcopy_source_set_with(&source_set, |_| {
            Ok(selected.next().expect("one exact retained set member"))
        })
        .unwrap();
        let HardcopySemanticDocument::Aggregate(semantic) = aggregate.semantic_document() else {
            panic!("expected named aggregate")
        };
        assert_eq!(
            semantic
                .children
                .iter()
                .map(|child| child.publication_page_label.as_deref())
                .collect::<Vec<_>>(),
            [Some("1 of 2"), Some("2 of 2")]
        );

        let scene = scene_from_resolved(
            &aggregate,
            aggregate.default_print_mapping(),
            SchematicHardcopySetup::default(),
            source_metadata(&aggregate, "RSpice test").unwrap(),
        )
        .unwrap();
        let printed_text = scene
            .primitives()
            .iter()
            .filter_map(|primitive| match primitive {
                ScenePrimitive::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(printed_text.contains(&"1 of 2"));
        assert!(printed_text.contains(&"2 of 2"));
        assert!(!printed_text.contains(&"2 of 3"));
        assert!(!printed_text.contains(&"3 of 3"));
    }
}
