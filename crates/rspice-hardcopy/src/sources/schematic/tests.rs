use super::*;
use rspice_design::library::{Cell, Library, LibraryCatalog, View, ViewType};
use rspice_design::schematic::{
    component::LibraryCellInstance, document::SchematicDocument, wire::Wire,
};
use rspice_design_model::design_management::{SheetDefinition, SheetPortPolicy, SheetTemplate};
use rspice_design_model::port::PortDirection;
use rspice_hardcopy_contract::sources::HardcopyPublicationIdentity;
use std::collections::HashMap;

fn identity(key: &str) -> HardcopySourceIdentity {
    HardcopySourceIdentity::try_new(
        key,
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Active document",
    )
    .unwrap()
}

fn sheet_definition(name: &str) -> SheetDefinition {
    SheetDefinition {
        name: name.to_owned(),
        template: SheetTemplate::AnalogSchematic,
        port_policy: SheetPortPolicy::TypedOffSheetPorts,
        explicit_page_number: None,
    }
}

#[test]
fn governed_current_sheet_never_leaks_and_all_sheets_preserve_catalog_order() {
    let mut document = SchematicDocument::default();
    document
        .wires
        .push(Wire::segment(11, Point::new(0, 0), Point::new(20, 0)));
    document
        .wires
        .push(Wire::segment(22, Point::new(100, 0), Point::new(120, 0)));
    let schematic = Schematic::from_document(document);
    let mut catalog = SheetCatalog::default();
    let first_id = catalog
        .create_sheet(sheet_definition("Input"), None)
        .unwrap();
    let second_id = catalog
        .create_sheet(sheet_definition("Output"), Some(first_id))
        .unwrap();
    let empty_id = catalog
        .create_sheet(sheet_definition("Reserved"), Some(second_id))
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), first_id, [11])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), second_id, [22])
        .unwrap();

    let mut project_settings =
        rspice_design_model::design_management::DrawingSheetProjectSettings {
            default_format: SchematicSheetFormat::from_standard(
                rspice_design_model::design_management::DrawingSheetStandard::IsoA3,
                rspice_design_model::SchematicPageOrientation::Landscape,
            )
            .try_update(|draft| {
                draft.inheritance =
                    rspice_design_model::design_management::DrawingSheetInheritance::ProjectDefault;
            })
            .unwrap(),
            ..Default::default()
        };
    let inherited_format = catalog
        .find(second_id)
        .unwrap()
        .page_format()
        .try_update(|draft| {
            draft.inheritance =
                rspice_design_model::design_management::DrawingSheetInheritance::ProjectDefault;
        })
        .unwrap();
    catalog
        .update_sheet_page_format(
            second_id,
            catalog.find(second_id).unwrap().revision(),
            inherited_format,
        )
        .unwrap();
    project_settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Organization,
        "RSpice Engineering".to_owned(),
    );
    let base_identity = identity("governed-schematic")
        .with_publication(
            HardcopyPublicationIdentity::try_new(
                "Precision Instruments",
                "analog/top/schematic",
                Some("A".to_owned()),
                Some("2026-08-04".to_owned()),
            )
            .unwrap(),
        )
        .unwrap();
    let second = resolve_schematic_source(SchematicHardcopySource {
        identity: schematic_sheet_identity(&base_identity, catalog.find(second_id).unwrap())
            .unwrap(),
        schematic: &schematic,
        selection: None,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: Some(&catalog),
        sheet_id: Some(second_id),
        project_default_drawing_sheet: Some(&project_settings.default_format),
        project_title_block_field_values: Some(&project_settings.title_block_field_values),
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();
    let HardcopySemanticDocument::Schematic(second_semantic) = second.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(
        second_semantic
            .wires
            .iter()
            .map(|wire| wire.id)
            .collect::<Vec<_>>(),
        [22]
    );
    let expected_inherited_format = project_settings
        .default_format
        .with_target_sheet_title_fields(catalog.find(second_id).unwrap().page_format());
    assert_eq!(
        second_semantic.drawing_sheet.as_ref(),
        Some(&expected_inherited_format)
    );
    assert_eq!(
        second_semantic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::Organization)
            .map(String::as_str),
        Some("RSpice Engineering")
    );
    assert_eq!(
        second_semantic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::Project)
            .map(String::as_str),
        Some("Precision Instruments")
    );
    assert_eq!(
        second_semantic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::CellView)
            .map(String::as_str),
        Some("analog/top/schematic")
    );
    assert_eq!(
        second_semantic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::Date)
            .map(String::as_str),
        Some("2026-08-04")
    );
    assert_eq!(
        second_semantic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::Revision)
            .map(String::as_str),
        Some("A")
    );

    let all = resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
        identity: base_identity,
        schematic: &schematic,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: &catalog,
        project_default_drawing_sheet: &project_settings.default_format,
        project_title_block_field_values: &project_settings.title_block_field_values,
    })
    .unwrap();
    let HardcopySemanticDocument::Aggregate(aggregate) = all.semantic_document() else {
        panic!("expected aggregate")
    };
    assert_eq!(aggregate.children.len(), 3);
    assert_eq!(
        aggregate
            .children
            .iter()
            .map(|child| child.source_key.clone())
            .collect::<Vec<_>>(),
        [
            format!("governed-schematic:sheet:{first_id}"),
            format!("governed-schematic:sheet:{second_id}"),
            format!("governed-schematic:sheet:{empty_id}"),
        ]
    );
    for (index, expected_wire) in [Some(11), Some(22), None].into_iter().enumerate() {
        let HardcopySemanticDocument::Schematic(sheet) =
            aggregate.children[index].document.as_ref()
        else {
            panic!("expected schematic child")
        };
        assert_eq!(
            sheet.wires.first().map(|wire| wire.id),
            expected_wire,
            "sheet {index} must contain only its own assigned wire"
        );
        let stored_format = catalog.sheets()[index].page_format();
        let effective_format = if stored_format.inheritance
            == rspice_design_model::design_management::DrawingSheetInheritance::ProjectDefault
        {
            project_settings
                .default_format
                .with_target_sheet_title_fields(stored_format)
        } else {
            stored_format.clone()
        };
        let expected_format_label = format!(
            "{} · {}",
            effective_format.authored_size.label(),
            effective_format.orientation.label().to_lowercase()
        );
        assert_eq!(sheet.drawing_sheet.as_ref(), Some(&effective_format));
        assert_eq!(
            sheet
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Page)
                .map(String::as_str),
            Some(match index {
                0 => "1 of 3",
                1 => "2 of 3",
                _ => "3 of 3",
            })
        );
        assert_eq!(
            sheet
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Format)
                .map(String::as_str),
            Some(expected_format_label.as_str())
        );
        assert_eq!(
            sheet
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Organization)
                .map(String::as_str),
            Some("RSpice Engineering")
        );
        assert_eq!(aggregate.children[index].page_break_before, index != 0);
    }
    assert_eq!(
        aggregate.children[2]
            .local_bounds
            .content_extent()
            .unwrap()
            .width()
            .micrometres(),
        297_000
    );
    assert_eq!(
        aggregate.children[2]
            .local_bounds
            .content_extent()
            .unwrap()
            .height()
            .micrometres(),
        210_000
    );
    assert_eq!(
        all.hardcopy_sections_for_setup(rspice_hardcopy_contract::SchematicHardcopySetup::default())
            .unwrap()
            .len(),
        3
    );

    let worker_bytes = all.worker_snapshot_json().unwrap();
    let round_trip = ResolvedHardcopyDocument::from_worker_snapshot_json(&worker_bytes).unwrap();
    assert_eq!(round_trip, all);

    let mut tampered: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
    tampered["source_key"] = serde_json::Value::String("tampered-source".to_owned());
    assert!(matches!(
        ResolvedHardcopyDocument::from_worker_snapshot_json(
            &serde_json::to_vec(&tampered).unwrap()
        ),
        Err(HardcopySourceError::InvalidWorkerSnapshot(_))
    ));
    let mut unknown: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
    unknown["unexpected"] = serde_json::Value::Bool(true);
    assert!(matches!(
        ResolvedHardcopyDocument::from_worker_snapshot_json(&serde_json::to_vec(&unknown).unwrap()),
        Err(HardcopySourceError::InvalidWorkerSnapshot(_))
    ));
}

#[test]
fn authored_cell_symbol_is_frozen_into_the_semantic_source() {
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-20, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 9,
        }],
        ..SymbolDocument::default()
    };
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document.store_in_view(&mut symbol_view).unwrap();
    let mut cell = Cell::new("amp");
    cell.add_view(symbol_view);
    let mut library = Library::new("work");
    library.add_cell(cell);
    let mut libraries = LibraryCatalog::default();
    libraries.add_library(library);
    let buffers = HashMap::new();
    let resolver = SymbolResolver::new(&libraries, &buffers);

    let mut schematic_document = SchematicDocument::default();
    schematic_document.components.push(
        Component::new(7, ComponentType::CellInstance, Point::new(20, 30))
            .with_library_cell(LibraryCellInstance::new("work", "amp", "symbol")),
    );
    let schematic = Schematic::from_document(schematic_document);
    let resolved = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        selection: None,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: Some(&resolver),
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();
    let HardcopySemanticDocument::Schematic(scene) = resolved.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(
        scene.components[0].resolved_symbol.as_ref(),
        Some(&document)
    );
    assert_eq!(
        scene.components[0].symbol_source,
        Some(SemanticSymbolSource::Authored)
    );
}

#[test]
fn stale_schematic_authority_is_rejected_before_digesting() {
    let schematic = Schematic::default();
    let error = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        selection: None,
        expected_topology_version: schematic.topology_version() + 1,
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap_err();
    assert!(matches!(error, HardcopySourceError::StaleSchematic { .. }));
}
