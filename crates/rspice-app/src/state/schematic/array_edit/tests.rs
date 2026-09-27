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
