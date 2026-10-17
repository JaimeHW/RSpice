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
    document.components.push(Component::new(
        21,
        ComponentType::Resistor,
        Point::new(40, 40),
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

    // A terminal is anchored at the pin rather than at the symbol origin, and
    // it is suppressed with the component that owns it.
    let hidden_terminal = DrcViolation::new(
        4,
        DrcViolationType::UnconnectedPin,
        "hidden terminal",
        DrcLocation::Terminal {
            component_id: 20,
            component_name: "R1".to_owned(),
            pin_name: "1".to_owned(),
            point: Point::new(10, 20),
        },
    );
    let visible_terminal = DrcViolation::new(
        5,
        DrcViolationType::UndrivenInput,
        "visible terminal",
        DrcLocation::Terminal {
            component_id: 21,
            component_name: "R2".to_owned(),
            pin_name: "1".to_owned(),
            point: Point::new(44, 48),
        },
    );
    let inside = DrcViolation::new(
        6,
        DrcViolationType::UnconnectedPin,
        "a finding in another cellview",
        DrcLocation::InsideInstance {
            path: rspice_app_types::hierarchy_path::InstancePath::parse("/X1").expect("path"),
            view: rspice_design_model::cell_view::CellViewRef::new("work", "amp", "schematic"),
            inner: Box::new(DrcLocation::Point { x: 4.0, y: 8.0 }),
        },
    );

    assert_eq!(anchor(&view, &component.location), None);
    assert_eq!(anchor(&view, &wire.location), None);
    assert_eq!(anchor(&view, &point.location), Some(Point::new(4, 8)));
    assert_eq!(anchor(&view, &hidden_terminal.location), None);
    assert_eq!(
        anchor(&view, &visible_terminal.location),
        Some(Point::new(44, 48))
    );
    assert_eq!(anchor(&view, &inside.location), None);
}

/// What follows a finding is told the sheet its object is drawn on, so it can
/// bring that sheet forward first; the canvas, which draws only the sheet in
/// view, is given no place for an object it does not draw.
#[test]
fn an_object_on_another_sheet_is_placed_on_its_own_sheet() {
    let catalog = catalog_with_hidden_object(20);
    let second = catalog
        .sheet_for_object(20)
        .expect("assigned to the second sheet");
    let mut document = SchematicDocument::default();
    for (id, at) in [(20, Point::new(10, 10)), (21, Point::new(40, 40))] {
        document
            .components
            .push(Component::new(id, ComponentType::Resistor, at));
    }
    let view = DesignView {
        document: &document,
        canvas_cache: None,
        sheet_catalog: Some(&catalog),
        review_markers: Default::default(),
    };
    let hidden = DrcLocation::Component {
        id: 20,
        name: "R1".to_owned(),
    };
    let unassigned = DrcLocation::Component {
        id: 21,
        name: "R2".to_owned(),
    };

    assert_eq!(
        placement(&view, &hidden),
        Some(Placement {
            point: Point::new(10, 10),
            sheet: Some(second),
        })
    );
    assert_eq!(anchor(&view, &hidden), None);
    assert_eq!(
        placement(&view, &unassigned),
        Some(Placement {
            point: Point::new(40, 40),
            sheet: None,
        }),
        "an object no sheet claims is drawn on the sheet in view"
    );
    assert_eq!(anchor(&view, &unassigned), Some(Point::new(40, 40)));
}

/// The checks read the projected document, which lays the second sheet a
/// stride along. A terminal found there is badged where the second sheet
/// draws it, not a million grid units off the canvas.
#[test]
fn a_terminal_on_a_later_sheet_is_anchored_where_that_sheet_draws_it() {
    let mut catalog = catalog_with_hidden_object(30);
    let second = catalog
        .sheet_for_object(30)
        .expect("assigned to the second sheet");
    catalog.set_active(second).unwrap();
    let mut document = SchematicDocument::default();
    document.components.push(Component::new(
        30,
        ComponentType::Resistor,
        Point::new(10, 10),
    ));
    let view = DesignView {
        document: &document,
        canvas_cache: None,
        sheet_catalog: Some(&catalog),
        review_markers: Default::default(),
    };

    let terminal = DrcLocation::Terminal {
        component_id: 30,
        component_name: "R1".to_owned(),
        pin_name: "1".to_owned(),
        point: Point::new(1_000_010, 20),
    };

    assert_eq!(anchor(&view, &terminal), Some(Point::new(10, 20)));
    // One drawing with no sheet catalog is its own projection.
    let unsheeted = DesignView {
        sheet_catalog: None,
        ..view
    };
    assert_eq!(
        live_terminal_point(unsheeted.sheet_catalog, 30, Point::new(10, 20)),
        Point::new(10, 20)
    );
}

/// A finding located at a bare point — the missing-ground anchor, a bus
/// diagnostic — has no object to ask for its sheet. The projection laid the
/// point on the second sheet, so it is badged where that sheet draws it, and
/// only while that sheet is in view.
#[test]
fn a_point_on_a_later_sheet_is_anchored_where_that_sheet_draws_it() {
    let mut catalog = catalog_with_hidden_object(30);
    let first = catalog
        .active_sheet_id()
        .expect("the first sheet is active");
    let second = catalog
        .sheet_for_object(30)
        .expect("assigned to the second sheet");
    let document = SchematicDocument::default();
    let point = DrcLocation::Point {
        x: 1_000_040.0,
        y: 60.0,
    };

    fn view<'a>(document: &'a SchematicDocument, catalog: &'a SheetCatalog) -> DesignView<'a> {
        DesignView {
            document,
            canvas_cache: None,
            sheet_catalog: Some(catalog),
            review_markers: Default::default(),
        }
    }

    catalog.set_active(second).unwrap();
    assert_eq!(
        anchor(&view(&document, &catalog), &point),
        Some(Point::new(40, 60))
    );

    catalog.set_active(first).unwrap();
    assert_eq!(anchor(&view(&document, &catalog), &point), None);
    assert_eq!(
        anchor(
            &view(&document, &catalog),
            &DrcLocation::Point { x: 40.0, y: 60.0 }
        ),
        Some(Point::new(40, 60))
    );
}
