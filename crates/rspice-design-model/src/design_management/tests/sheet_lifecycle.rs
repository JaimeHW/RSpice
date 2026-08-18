//! Sheet rename, delete, and move lifecycle tests.
//!
//! These cases pin the transactional lifecycle of a sheet — the operations a
//! navigator offers — without growing the broader design-management test
//! module.

use super::*;

fn sheet_title(sheet: &DesignSheet) -> &str {
    &sheet
        .page_format()
        .title_block
        .fields
        .get(&DrawingSheetTitleFieldId::SheetTitle)
        .expect("an authored sheet always owns a title field")
        .value
}

fn boundary_port(
    net_name: &str,
    source: SheetId,
    source_wire: u64,
    point: Point,
    destination: SheetId,
    destination_component: u64,
) -> CrossSheetPortDefinition {
    CrossSheetPortDefinition {
        net_name: net_name.to_owned(),
        first: CrossSheetPortEndpoint {
            sheet_id: source,
            anchor: CrossSheetPortAnchor::WirePoint {
                wire_id: source_wire,
                point,
            },
        },
        second: CrossSheetPortEndpoint {
            sheet_id: destination,
            anchor: CrossSheetPortAnchor::ComponentTerminal {
                component_id: destination_component,
                terminal_name: net_name.to_owned(),
            },
        },
        direction: CrossSheetPortDirection::Output,
        signal_type: CrossSheetSignalType::Analog,
        discipline: CrossSheetDiscipline::Electrical,
    }
}

fn port_id(catalog: &SheetCatalog, net_name: &str) -> CrossSheetPortId {
    catalog
        .cross_sheet_ports()
        .iter()
        .find(|port| port.definition().net_name == net_name)
        .expect("the port was created by an explicit boundary resolution")
        .id()
}

#[test]
fn sheet_rename_moves_the_name_and_the_sheet_owned_title_together() {
    let mut catalog = SheetCatalog::default();
    let input = catalog.create_sheet(sheet("Input", 1), None).unwrap();
    let bias = catalog.create_sheet(sheet("Bias", 2), Some(input)).unwrap();
    let sheet_revision = catalog.find(input).unwrap().revision();
    let catalog_revision = catalog.revision();

    assert!(matches!(
        catalog.rename_sheet(input, sheet_revision + 1, "AFE core".to_owned()),
        Err(DesignManagementError::RevisionConflict { .. })
    ));
    assert!(matches!(
        catalog.rename_sheet(input, sheet_revision, "  bias  ".to_owned()),
        Err(DesignManagementError::DuplicateName {
            domain: "sheet",
            ..
        })
    ));
    assert_eq!(catalog.find(input).unwrap().name(), "Input");
    assert_eq!(sheet_title(catalog.find(input).unwrap()), "Input");
    assert_eq!(catalog.revision(), catalog_revision);

    let committed = catalog
        .rename_sheet(input, sheet_revision, "AFE core".to_owned())
        .unwrap();
    assert_eq!(committed, sheet_revision + 1);
    assert_eq!(catalog.revision(), catalog_revision + 1);
    let renamed = catalog.find(input).unwrap();
    assert_eq!(renamed.name(), "AFE core");
    assert_eq!(sheet_title(renamed), "AFE core");
    assert_eq!(catalog.find(bias).unwrap().revision(), 1);
    catalog.validate().unwrap();

    assert!(matches!(
        catalog.rename_sheet(input, committed, "AFE core".to_owned()),
        Err(DesignManagementError::NoChanges("sheet name"))
    ));
}

#[test]
fn sheet_delete_refuses_the_last_sheet_and_a_populated_one() {
    let mut catalog = SheetCatalog::default();
    let top = catalog.create_sheet(sheet("Top", 1), None).unwrap();
    assert!(matches!(
        catalog.delete_sheet(
            catalog.revision(),
            top,
            SheetDeleteResolution::BlockIfNotEmpty
        ),
        Err(DesignManagementError::LastSheetRemoval)
    ));

    let bias = catalog.create_sheet(sheet("Bias", 2), Some(top)).unwrap();
    catalog
        .assign_objects(catalog.revision(), top, [1, 2])
        .unwrap();
    let revision = catalog.revision();
    assert!(matches!(
        catalog.delete_sheet(revision, top, SheetDeleteResolution::BlockIfNotEmpty),
        Err(DesignManagementError::SheetNotEmpty { sheet: subject, objects: 2 }) if subject == top
    ));
    assert!(matches!(
        catalog.delete_sheet(revision + 1, bias, SheetDeleteResolution::BlockIfNotEmpty),
        Err(DesignManagementError::RevisionConflict { .. })
    ));
    assert_eq!(catalog.revision(), revision);
    assert_eq!(catalog.sheets().len(), 2);

    let receipt = catalog
        .delete_sheet(revision, bias, SheetDeleteResolution::BlockIfNotEmpty)
        .unwrap();
    assert_eq!(receipt.removed_sheet_id, bias);
    assert!(receipt.moved_object_ids.is_empty());
    assert!(receipt.deleted_object_ids.is_empty());
    assert!(receipt.removed_cross_sheet_ports.is_empty());
    assert_eq!(receipt.next_active_sheet_id, Some(top));
    assert_eq!(receipt.catalog_revision, catalog.revision());
    assert_ne!(receipt.semantic_digest, empty_digest());
    assert_eq!(catalog.sheets().len(), 1);
    catalog.validate().unwrap();
}

#[test]
fn sheet_delete_moves_objects_and_re_anchors_or_drops_cross_sheet_ports() {
    let mut catalog = SheetCatalog::default();
    let bias = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let afe = catalog.create_sheet(sheet("AFE", 2), Some(bias)).unwrap();
    let output = catalog.create_sheet(sheet("Output", 3), Some(afe)).unwrap();
    catalog
        .assign_objects(catalog.revision(), bias, [1, 2, 3])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), afe, [4])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), output, [5])
        .unwrap();
    catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![3],
            destination_sheet_id: afe,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![boundary_port("VREF", bias, 1, Point::new(4, 4), afe, 4)],
            },
        })
        .unwrap();
    catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![2],
            destination_sheet_id: output,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![boundary_port("VBIAS", bias, 1, Point::new(9, 4), output, 5)],
            },
        })
        .unwrap();
    let vref = port_id(&catalog, "VREF");
    let vbias = port_id(&catalog, "VBIAS");
    assert_eq!(catalog.active_sheet_id(), Some(bias));
    assert_eq!(catalog.objects_on_sheet(bias).collect::<Vec<_>>(), vec![1]);

    let receipt = catalog
        .delete_sheet(
            catalog.revision(),
            bias,
            SheetDeleteResolution::MoveObjectsTo(afe),
        )
        .unwrap();
    assert_eq!(receipt.moved_object_ids, vec![1]);
    assert!(receipt.deleted_object_ids.is_empty());
    assert_eq!(receipt.removed_cross_sheet_ports, vec![vref]);
    assert_eq!(receipt.next_active_sheet_id, Some(afe));
    assert_eq!(catalog.active_sheet_id(), Some(afe));
    assert_eq!(catalog.sheet_for_object(1), Some(afe));

    let retained = catalog.cross_sheet_ports();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].id(), vbias);
    assert_eq!(retained[0].definition().first.sheet_id, afe);
    assert_eq!(retained[0].definition().second.sheet_id, output);
    assert_eq!(retained[0].revision(), 2);
    assert_eq!(catalog.page_number_and_count(afe), Some((1, 2)));
    assert_eq!(catalog.page_number_and_count(output), Some((2, 2)));
    catalog.validate().unwrap();

    assert!(matches!(
        catalog.delete_sheet(
            catalog.revision(),
            afe,
            SheetDeleteResolution::MoveObjectsTo(afe)
        ),
        Err(DesignManagementError::MissingReference {
            domain: "destination sheet",
            ..
        })
    ));
}

#[test]
fn sheet_delete_hands_back_the_objects_the_schematic_must_remove() {
    let mut catalog = SheetCatalog::default();
    let bias = catalog.create_sheet(sheet("Bias", 1), None).unwrap();
    let afe = catalog.create_sheet(sheet("AFE", 2), Some(bias)).unwrap();
    catalog
        .assign_objects(catalog.revision(), bias, [1, 2])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), afe, [3])
        .unwrap();
    catalog
        .move_selection(MoveSelectionRequest {
            expected_catalog_revision: catalog.revision(),
            object_ids: vec![2],
            destination_sheet_id: afe,
            boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                ports: vec![boundary_port("VREF", bias, 1, Point::new(4, 4), afe, 3)],
            },
        })
        .unwrap();
    let vref = port_id(&catalog, "VREF");

    let receipt = catalog
        .delete_sheet(
            catalog.revision(),
            bias,
            SheetDeleteResolution::DeleteObjects,
        )
        .unwrap();
    assert_eq!(receipt.deleted_object_ids, vec![1]);
    assert!(receipt.moved_object_ids.is_empty());
    assert_eq!(receipt.removed_cross_sheet_ports, vec![vref]);
    assert_eq!(catalog.sheet_for_object(1), None);
    assert_eq!(catalog.sheet_for_object(2), Some(afe));
    assert!(catalog.cross_sheet_ports().is_empty());
    assert_eq!(catalog.page_number_and_count(afe), Some((1, 1)));
    catalog.validate().unwrap();
}

#[test]
fn sheet_delete_reactivates_a_neighbor_and_renumbers_the_print_set() {
    let mut catalog = SheetCatalog::default();
    let top = catalog.create_sheet(sheet("Top", 1), None).unwrap();
    let middle = catalog.create_sheet(sheet("Middle", 2), Some(top)).unwrap();
    let last = catalog
        .create_sheet(sheet("Last", 3), Some(middle))
        .unwrap();
    catalog.set_active(last).unwrap();

    let receipt = catalog
        .delete_sheet(
            catalog.revision(),
            last,
            SheetDeleteResolution::BlockIfNotEmpty,
        )
        .unwrap();
    assert_eq!(receipt.next_active_sheet_id, Some(middle));
    assert_eq!(catalog.active_sheet_id(), Some(middle));
    assert_eq!(catalog.find(middle).unwrap().revision(), 1);

    catalog.set_active(top).unwrap();
    let receipt = catalog
        .delete_sheet(
            catalog.revision(),
            top,
            SheetDeleteResolution::BlockIfNotEmpty,
        )
        .unwrap();
    assert_eq!(receipt.next_active_sheet_id, Some(middle));
    assert_eq!(catalog.page_number_and_count(middle), Some((1, 1)));
    assert_eq!(catalog.find(middle).unwrap().revision(), 2);
    catalog.validate().unwrap();
}

#[test]
fn moving_one_sheet_commits_the_same_transaction_as_the_equivalent_reorder() {
    let mut catalog = SheetCatalog::default();
    let top = catalog.create_sheet(sheet("Top", 1), None).unwrap();
    let middle = catalog.create_sheet(sheet("Middle", 2), Some(top)).unwrap();
    let last = catalog
        .create_sheet(sheet("Last", 3), Some(middle))
        .unwrap();
    let mut reordered = catalog.clone();
    let revision = catalog.revision();

    let moved_revision = catalog
        .move_sheet(
            revision,
            last,
            0,
            ReorderPageNumbering::UpdatePrintPageNumbers,
        )
        .unwrap();
    let reordered_revision = reordered
        .reorder(
            revision,
            vec![last, top, middle],
            ReorderPageNumbering::UpdatePrintPageNumbers,
            ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
        )
        .unwrap();
    assert_eq!(moved_revision, reordered_revision);
    assert_eq!(catalog, reordered);
    assert_eq!(
        catalog
            .sheets()
            .iter()
            .map(DesignSheet::id)
            .collect::<Vec<_>>(),
        vec![last, top, middle]
    );

    assert!(matches!(
        catalog.move_sheet(
            catalog.revision(),
            last,
            3,
            ReorderPageNumbering::UpdatePrintPageNumbers
        ),
        Err(DesignManagementError::InvalidSheetOrder)
    ));
    assert!(matches!(
        catalog.move_sheet(
            revision,
            last,
            2,
            ReorderPageNumbering::UpdatePrintPageNumbers
        ),
        Err(DesignManagementError::RevisionConflict { .. })
    ));
}

#[test]
fn sheet_neighbours_stop_at_the_ends_of_the_catalog() {
    let mut catalog = SheetCatalog::default();
    let top = catalog.create_sheet(sheet("Top", 1), None).unwrap();
    let bias = catalog.create_sheet(sheet("Bias", 2), Some(top)).unwrap();

    assert_eq!(catalog.previous_sheet(top), None);
    assert_eq!(catalog.next_sheet(top), Some(bias));
    assert_eq!(catalog.previous_sheet(bias), Some(top));
    assert_eq!(catalog.next_sheet(bias), None);
    assert_eq!(catalog.next_sheet(SheetId::new()), None);
    assert_eq!(catalog.previous_sheet(SheetId::new()), None);
}
