//! Cross-feature netlist checks for the shipped example schematic.
//!
//! The workbench owns the authored example; its netlist assertions belong here
//! rather than making the lower simulation module depend on the workbench.

use super::load_example;
use crate::simulation::netlist_gen::generate_netlist;
use crate::state::{
    DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry, Point,
    SchematicState,
};

/// A label sitting on a wire names that net in the generated netlist.
#[test]
fn net_label_names_the_node() {
    let mut state = SchematicState::default();
    load_example("RC Lowpass Filter", &mut state);

    let result = generate_netlist(&state);
    assert!(
        result.nets.contains_key("out"),
        "the \"out\" label should name its net; got nets {:?}",
        result.nets.keys().collect::<Vec<_>>()
    );
    // The named node appears in instance lines (R1 ... out ...).
    assert!(
        result
            .netlist
            .lines()
            .any(|l| { l.starts_with('R') && l.split_whitespace().any(|tok| tok == "out") }),
        "instance lines should reference the labeled node:\n{}",
        result.netlist
    );
}

#[test]
fn design_notes_never_change_generated_spice_or_connectivity() {
    let mut baseline = SchematicState::default();
    load_example("RC Lowpass Filter", &mut baseline);
    let expected = generate_netlist(&baseline);
    let mut documented = baseline;
    documented.design_notes.push(
        DesignNote::new(
            90_001,
            Point::new(25, 30),
            DesignNoteKind::PropertyDisplay,
            "${component_count} components",
        )
        .unwrap(),
    );

    let actual = generate_netlist(&documented);
    assert_eq!(actual.netlist, expected.netlist);
    assert_eq!(actual.nets.len(), expected.nets.len());
    for (name, expected_points) in &expected.nets {
        let mut expected_points = expected_points.clone();
        expected_points.sort_by_key(|point| (point.x, point.y));
        let mut actual_points = actual.nets.get(name).cloned().unwrap_or_default();
        actual_points.sort_by_key(|point| (point.x, point.y));
        assert_eq!(actual_points, expected_points, "net {name}");
    }
    assert_eq!(actual.warnings, expected.warnings);
    assert_eq!(actual.errors, expected.errors);
}

#[test]
fn all_documentation_shapes_leave_spice_and_connectivity_byte_for_byte_unchanged() {
    let mut baseline = SchematicState::default();
    load_example("RC Lowpass Filter", &mut baseline);
    let expected = generate_netlist(&baseline);
    let mut documented = baseline;
    documented.documentation_shapes = [
        DocumentationShapeGeometry::Rectangle {
            first: Point::new(-80, -40),
            opposite: Point::new(20, 30),
        },
        DocumentationShapeGeometry::Line {
            start: Point::new(-25, 70),
            end: Point::new(65, 105),
        },
        DocumentationShapeGeometry::Polygon {
            points: vec![
                Point::new(100, -20),
                Point::new(170, 10),
                Point::new(145, 85),
                Point::new(80, 45),
            ],
        },
        DocumentationShapeGeometry::Arc {
            start: Point::new(-100, 160),
            through: Point::new(-50, 110),
            end: Point::new(0, 160),
        },
        DocumentationShapeGeometry::Callout {
            tip: Point::new(90, 150),
            elbow: Point::new(130, 125),
            box_corner: Point::new(230, 190),
        },
    ]
    .into_iter()
    .enumerate()
    .map(|(index, geometry)| {
        DocumentationShape::new(90_100 + index as u64, geometry)
            .expect("documentation shape fixture is valid")
    })
    .collect();

    let actual = generate_netlist(&documented);
    assert_eq!(actual.netlist.as_bytes(), expected.netlist.as_bytes());
    assert_eq!(actual.nets.len(), expected.nets.len());
    for (name, expected_points) in &expected.nets {
        let mut expected_points = expected_points.clone();
        expected_points.sort_by_key(|point| (point.x, point.y));
        let mut actual_points = actual
            .nets
            .get(name)
            .unwrap_or_else(|| panic!("missing generated net {name}"))
            .clone();
        actual_points.sort_by_key(|point| (point.x, point.y));
        assert_eq!(actual_points, expected_points, "net {name}");
    }
    assert_eq!(actual.point_to_net, expected.point_to_net);
    assert_eq!(actual.net_segments.len(), expected.net_segments.len());
    for (name, expected_segments) in &expected.net_segments {
        let canonicalize = |segments: &[(Point, Point)]| {
            let mut segments: Vec<_> = segments
                .iter()
                .map(|&(first, second)| {
                    if (first.x, first.y) <= (second.x, second.y) {
                        (first, second)
                    } else {
                        (second, first)
                    }
                })
                .collect();
            segments.sort_by_key(|(first, second)| (first.x, first.y, second.x, second.y));
            segments
        };
        let actual_segments = actual
            .net_segments
            .get(name)
            .unwrap_or_else(|| panic!("missing generated net segments for {name}"));
        assert_eq!(
            canonicalize(actual_segments),
            canonicalize(expected_segments),
            "net segments for {name}"
        );
    }
    assert_eq!(actual.warnings, expected.warnings);
    assert_eq!(actual.errors, expected.errors);
}
