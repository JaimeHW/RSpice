use super::*;
use rspice_design::drc::DrcViolationType;
use rspice_design::schematic::{
    component::Component, component_type::ComponentType, document::SchematicDocument, wire::Wire,
};
use rspice_design_model::design_management::{
    SheetCatalog, SheetDefinition, SheetPortPolicy, SheetTemplate,
};

fn catalog_with_hidden_object(hidden_id: u64) -> SheetCatalog {
    let mut catalog = SheetCatalog::default();
    let mut sheets = Vec::new();
    for page in [1, 2] {
        let sheet = catalog
            .create_sheet(
                SheetDefinition {
                    name: format!("Sheet {page}"),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(page),
                },
                sheets.last().copied(),
            )
            .unwrap();
        sheets.push(sheet);
    }
    catalog
        .assign_objects(catalog.revision(), sheets[1], [hidden_id])
        .unwrap();
    catalog.set_active(sheets[0]).unwrap();
    catalog
}

#[test]
fn hidden_identity_anchor_is_suppressed_but_unowned_point_remains_visible() {
    let catalog = catalog_with_hidden_object(20);
    let mut document = SchematicDocument::default();
    document.components.push(Component::new(
        20,
        ComponentType::Resistor,
        Point::new(10, 10),
    ));
    document
        .wires
        .push(Wire::segment(20, Point::origin(), Point::new(20, 0)));

    let view = DesignView {
        document: &document,
        canvas_cache: None,
        sheet_catalog: Some(&catalog),
        review_markers: Default::default(),
    };
    let component = DrcViolation::new(
        1,
        DrcViolationType::UnconnectedPin,
        "hidden component",
        DrcLocation::Component {
            id: 20,
            name: "R1".to_owned(),
        },
    );
    let wire = DrcViolation::new(
        2,
        DrcViolationType::DanglingWire,
        "hidden wire",
        DrcLocation::Wire { id: 20 },
    );
    let point = DrcViolation::new(
        3,
        DrcViolationType::MissingGround,
        "unowned point",
        DrcLocation::Point { x: 4.0, y: 8.0 },
    );

    assert_eq!(anchor(&view, &component.location), None);
    assert_eq!(anchor(&view, &wire.location), None);
    assert_eq!(anchor(&view, &point.location), Some(Point::new(4, 8)));
}
