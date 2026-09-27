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
        expected_identity.generate_name(ComponentType::Resistor),
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
    assert_eq!(identity.generate_name(ComponentType::Resistor), "R3");
    assert_eq!(document.components.len(), 2);
    assert_eq!(document.components[1].name, "R2");
    assert_eq!(document.components[1].pos, Point::new(101, 0));
}
