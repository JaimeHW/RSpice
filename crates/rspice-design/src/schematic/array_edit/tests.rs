//! Array transform invariants.

use super::*;

#[test]
fn arbitrary_radial_angles_preserve_line_polygon_and_arc_control_geometry() {
    let transform = MemberTransform::Rotate {
        center: Point::origin(),
        member_index: 1,
        member_count: 3,
    };
    let geometries = [
        DocumentationShapeGeometry::Line {
            start: Point::new(30, 0),
            end: Point::new(30, 30),
        },
        DocumentationShapeGeometry::Polygon {
            points: vec![Point::new(40, 0), Point::new(60, 10), Point::new(45, 30)],
        },
        DocumentationShapeGeometry::Arc {
            start: Point::new(30, 0),
            through: Point::new(21, 21),
            end: Point::new(0, 30),
        },
    ];

    for (index, geometry) in geometries.into_iter().enumerate() {
        let expected_points = geometry
            .points()
            .into_iter()
            .map(|point| transform_point(point, transform).unwrap())
            .collect::<Vec<_>>();
        let transformed =
            transform_documentation_geometry(&geometry, transform, 100 + index as u64).unwrap();
        assert_eq!(transformed.kind(), geometry.kind());
        assert_eq!(transformed.points(), expected_points);
        transformed.validate().unwrap();
    }
}

#[test]
fn rectangle_and_callout_use_exact_integer_quarter_turns() {
    let transform = MemberTransform::Rotate {
        center: Point::origin(),
        member_index: 1,
        member_count: 4,
    };
    let rectangle = DocumentationShapeGeometry::Rectangle {
        first: Point::new(10, 0),
        opposite: Point::new(30, 20),
    };
    let callout = DocumentationShapeGeometry::Callout {
        tip: Point::new(20, 0),
        elbow: Point::new(10, 10),
        box_corner: Point::new(30, 30),
    };

    assert_eq!(
        transform_documentation_geometry(&rectangle, transform, 1).unwrap(),
        DocumentationShapeGeometry::Rectangle {
            first: Point::new(0, 10),
            opposite: Point::new(-20, 30),
        }
    );
    assert_eq!(
        transform_documentation_geometry(&callout, transform, 2).unwrap(),
        DocumentationShapeGeometry::Callout {
            tip: Point::new(0, 20),
            elbow: Point::new(-10, 10),
            box_corner: Point::new(-30, 30),
        }
    );
}

#[test]
fn axis_aligned_documentation_fails_closed_at_arbitrary_angles() {
    let transform = MemberTransform::Rotate {
        center: Point::origin(),
        member_index: 1,
        member_count: 3,
    };
    let rectangle = DocumentationShapeGeometry::Rectangle {
        first: Point::new(10, 0),
        opposite: Point::new(30, 20),
    };
    let callout = DocumentationShapeGeometry::Callout {
        tip: Point::new(20, 0),
        elbow: Point::new(10, 10),
        box_corner: Point::new(30, 30),
    };

    assert_eq!(
        transform_documentation_geometry(&rectangle, transform, 70),
        Err(SchematicArrayError::InvalidGeometry { object_id: 70 })
    );
    assert_eq!(
        transform_documentation_geometry(&callout, transform, 71),
        Err(SchematicArrayError::InvalidGeometry { object_id: 71 })
    );
}

#[test]
fn discarded_array_preserves_document_and_allocators() {
    use super::super::{component_type::ComponentType, history::SchematicSnapshot};

    let mut document = SchematicDocument::default();
    let mut component = Component::new(1, ComponentType::Resistor, Point::new(1, 0));
    component.name = "R1".to_owned();
    document.components.push(component);
    let mut identity = SchematicIdentity::with_cursor(7);
    identity.record_component_number("R", 1);
    let components = HashSet::from([1]);
    let empty = HashSet::new();
    let selection = || ArraySelection {
        objects: CopySelection {
            components: &components,
            wires: &empty,
            buses: &empty,
            bus_taps: &empty,
            net_labels: &empty,
            design_notes: &empty,
            documentation_shapes: &empty,
            probes: &empty,
        },
        junctions: std::iter::empty(),
        has_partial_objects: false,
        on_sheet: &every_sheet,
    };
    let plan = SchematicArrayPlan::parse(
        SchematicArrayKind::Linear,
        "2 × 1",
        "R1…R2",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    let terminals = |component: &Component| {
        component
            .terminal_positions()
            .into_iter()
            .map(|(name, point)| (name.to_owned(), point))
            .collect()
    };
    let forged = SchematicArrayPlan {
        kind: SchematicArrayKind::Linear,
        count: SchematicArrayCount::new(2, 2).unwrap(),
        naming: SchematicArrayNaming::parse("R1…R4").unwrap(),
        placement: SchematicArrayPlacement::Pitch(Point::new(100, 100)),
    };
    assert_eq!(
        ArraySource {
            document: &document,
            identity_cursor: identity.cursor(),
            selection: selection()
        }
        .preview_array_selection_resolved(&forged, terminals, Component::bounding_box),
        Err(SchematicArrayError::LinearCountRequiresOneAxis)
    );

    let before = SchematicSnapshot::capture(&document);

    let array = ArrayEdit::prepare(
        &mut document,
        &mut identity,
        selection(),
        &plan,
        terminals,
        Component::bounding_box,
    )
    .unwrap();
    assert_eq!(array.impact().components, 1);
    drop(array);

    assert!(before.is_equal_document(&document));
    assert_eq!(identity.cursor(), 7);
    let mut expected_identity = identity.clone();
    assert_eq!(
        expected_identity.generate_name(&document, ComponentType::Resistor),
        "R2"
    );

    let objects = ArrayEdit::prepare(
        &mut document,
        &mut identity,
        selection(),
        &plan,
        terminals,
        Component::bounding_box,
    )
    .unwrap()
    .commit();
    assert_eq!(objects.components, HashSet::from([1, 7]));
    assert_eq!(identity.cursor(), 8);
    assert_eq!(
        identity.generate_name(&document, ComponentType::Resistor),
        "R3"
    );
    assert_eq!(document.components.len(), 2);
    assert_eq!(document.components[1].name, "R2");
    assert_eq!(document.components[1].pos, Point::new(101, 0));
}

fn named_resistor(id: u64, name: &str, x: i32) -> Component {
    let mut component = Component::new(
        id,
        super::super::component_type::ComponentType::Resistor,
        Point::new(x, 0),
    );
    component.name = name.to_owned();
    component
}

fn naming_source<'a>(
    document: &'a SchematicDocument,
    components: &'a HashSet<u64>,
    labels: &'a HashSet<u64>,
    empty: &'a HashSet<u64>,
) -> ArraySource<'a, std::iter::Empty<Point>> {
    ArraySource {
        document,
        identity_cursor: 100,
        selection: ArraySelection {
            objects: CopySelection {
                components,
                wires: empty,
                buses: empty,
                bus_taps: empty,
                net_labels: labels,
                design_notes: empty,
                documentation_shapes: empty,
                probes: empty,
            },
            junctions: std::iter::empty(),
            has_partial_objects: false,
            on_sheet: &every_sheet,
        },
    }
}

#[test]
fn a_name_without_one_numeric_run_is_arrayed_with_a_numbered_suffix() {
    // A common-source stage {M1, Rload, Vdd, M1_2}; Rload_1 already exists.
    let mut document = SchematicDocument::default();
    document.components.extend([
        named_resistor(1, "M1", 0),
        named_resistor(2, "Rload", 100),
        named_resistor(3, "Vdd", 200),
        named_resistor(4, "M1_2", 300),
        named_resistor(5, "Rload_1", 400),
    ]);
    let components = HashSet::from([1, 2, 3, 4]);
    let empty = HashSet::new();
    let source = naming_source(&document, &components, &empty, &empty);

    let naming = source
        .default_array_naming(SchematicArrayCount::new(4, 1).unwrap())
        .unwrap();

    let names = |name: &str| {
        (0..4)
            .map(|member| naming.value_for_source(name, member).unwrap())
            .collect::<Vec<_>>()
    };
    assert_eq!(names("M1"), ["M1", "M2", "M3", "M4"]);
    assert_eq!(names("Vdd"), ["Vdd", "Vdd_1", "Vdd_2", "Vdd_3"]);
    assert_eq!(names("M1_2"), ["M1_2", "M1_2_1", "M1_2_2", "M1_2_3"]);
    assert_eq!(
        names("Rload"),
        ["Rload", "Rload_2", "Rload_4", "Rload_6"],
        "the generated names step over the existing Rload_1"
    );
    let shown = naming.to_string();
    assert_eq!(
        SchematicArrayNaming::parse(&shown)
            .unwrap()
            .normalized_for_members(4)
            .unwrap(),
        naming,
        "the clause the dialog shows reads back as the same naming: {shown}"
    );

    let terminals = |component: &Component| {
        component
            .terminal_positions()
            .into_iter()
            .map(|(name, point)| (name.to_owned(), point))
            .collect()
    };
    let placement = SchematicArrayPlacement::Pitch(Point::new(0, 100));
    let plan =
        SchematicArrayPlan::parse(SchematicArrayKind::Linear, "4 × 1", &shown, placement).unwrap();
    let preview = source
        .preview_array_selection_resolved(&plan, terminals, Component::bounding_box)
        .unwrap();
    let created: HashSet<&str> = preview
        .components()
        .iter()
        .map(|component| component.name.as_str())
        .collect();
    assert!(created.contains("Rload_2") && created.contains("Vdd_3"));

    // A typed range that would reuse an existing name is refused.
    let typed = shown.replace("Rload…Rload_6", "Rload…Rload_3");
    let plan =
        SchematicArrayPlan::parse(SchematicArrayKind::Linear, "4 × 1", &typed, placement).unwrap();
    assert_eq!(
        source.preview_array_selection_resolved(&plan, terminals, Component::bounding_box),
        Err(SchematicArrayError::NameCollision {
            name: "Rload_1".to_owned()
        })
    );
}

#[test]
fn an_angle_bracket_bit_with_a_digit_in_its_stem_is_arrayed() {
    let mut document = SchematicDocument::default();
    document
        .net_labels
        .push(NetLabel::new(1, Point::origin(), "A2D<3>"));
    let labels = HashSet::from([1]);
    let empty = HashSet::new();
    let source = naming_source(&document, &empty, &labels, &empty);

    let naming = source
        .default_array_naming(SchematicArrayCount::new(4, 1).unwrap())
        .unwrap();

    assert_eq!(naming.to_string(), "A2D<3>…A2D<6>");
    assert_eq!(
        naming.value_for_source("A2D<3>", 3).as_deref(),
        Some("A2D<6>")
    );
    // A bit spelled with brackets is not the sequence of one spelled with
    // angles, and a plan saved before bits carried their notation still
    // reads as the square-bracket bit it was.
    assert!(matches!(
        SchematicArrayNaming::parse("A2D<3>…A2D[6]"),
        Err(SchematicArrayError::MismatchedNamingEndpoints { .. })
    ));
    let saved: SchematicArrayNameAtom = serde_json::from_value(serde_json::json!({
        "kind": "indexed", "stem": "DATA", "index": 7, "minimum_width": null
    }))
    .unwrap();
    assert_eq!(saved.to_string(), "DATA[7]");
}
