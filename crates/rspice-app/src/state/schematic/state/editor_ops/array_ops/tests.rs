//! Tests for array and bus generation.
//!
//! Generation is budgeted and checked before it allocates, and generated
//! names, bounds, and terminal ownership are asserted to stay canonical -
//! a rejected array must leave the schematic exactly as it was.

use super::*;
use crate::state::{
    BusDeclaration, BusTapOrientation, ComponentType, DesignNoteKind, DocumentationShapeGeometry,
};

fn plan(
    kind: SchematicArrayKind,
    count: &str,
    naming: &str,
    placement: SchematicArrayPlacement,
) -> SchematicArrayPlan {
    SchematicArrayPlan::parse(kind, count, naming, placement).unwrap()
}

fn selected_resistor() -> SchematicState {
    let mut state = SchematicState::default();
    let id = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    state.selection.select_only_component(id);
    state.clear_undo_history();
    state.is_dirty = false;
    state
}

#[test]
fn preview_is_exact_and_does_not_mutate_any_live_runtime_state() {
    let state = selected_resistor();
    let before = state.clone();
    let plan = plan(
        SchematicArrayKind::Linear,
        "4 × 1",
        "R1…R4",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    );

    let preview = state.preview_array_selection(&plan).unwrap();

    assert_eq!(preview.impact.members, 4);
    assert_eq!(preview.impact.replicas, 3);
    assert_eq!(preview.components.len(), 3);
    assert_eq!(
        preview
            .components
            .iter()
            .map(|component| (component.name.clone(), component.pos))
            .collect::<Vec<_>>(),
        [
            ("R2".to_owned(), Point::new(100, 0)),
            ("R3".to_owned(), Point::new(200, 0)),
            ("R4".to_owned(), Point::new(300, 0)),
        ]
    );
    assert_eq!(state.components, before.components);
    assert_eq!(state.selection, before.selection);
    assert_eq!(state.clipboard.count(), before.clipboard.count());
    assert_eq!(state.is_dirty, before.is_dirty);
    assert_eq!(state.topology_version(), before.topology_version());
    assert!(!state.can_undo());
}

#[test]
fn linear_commit_matches_preview_preserves_clipboard_and_undoes_once() {
    let mut state = selected_resistor();
    state
        .clipboard
        .net_labels
        .push(NetLabel::new(9_999, Point::new(5, 5), "clipboard_guard"));
    let clipboard = state.clipboard.clone();
    let topology = state.topology_version();
    let plan = plan(
        SchematicArrayKind::Linear,
        "3 × 1",
        "R1…R3",
        SchematicArrayPlacement::Pitch(Point::new(80, 0)),
    );
    let preview = state.preview_array_selection(&plan).unwrap();

    let impact = state.array_selection(&plan).unwrap();

    assert_eq!(impact, preview.impact);
    assert_eq!(&state.components[1..], preview.components.as_slice());
    assert_eq!(state.clipboard.components, clipboard.components);
    assert_eq!(state.clipboard.net_labels, clipboard.net_labels);
    assert_eq!(state.topology_version(), topology.wrapping_add(1));
    assert_eq!(state.undo_description(), Some("create array"));
    assert!(state.undo());
    assert_eq!(state.components.len(), 1);
    assert!(
        !state.can_undo(),
        "array commit must create exactly one undo step"
    );
}

#[test]
fn rectangular_members_are_row_major_with_independent_axis_pitch() {
    let mut state = selected_resistor();
    let plan = plan(
        SchematicArrayKind::Rectangular,
        "2 × 2",
        "R1…R4",
        SchematicArrayPlacement::Pitch(Point::new(100, 60)),
    );

    let preview = state.preview_array_selection(&plan).unwrap();
    assert_eq!(
        preview
            .components
            .iter()
            .map(|component| (component.name.as_str(), component.pos))
            .collect::<Vec<_>>(),
        [
            ("R2", Point::new(100, 0)),
            ("R3", Point::new(0, 60)),
            ("R4", Point::new(100, 60)),
        ]
    );
    state.array_selection(&plan).unwrap();
    assert_eq!(state.components.len(), 4);
}

#[test]
fn radial_documentation_rotates_exact_geometry_without_topology_change() {
    let mut state = SchematicState::default();
    let note = DesignNote::new(
        40,
        Point::new(10, 0),
        DesignNoteKind::PlainText,
        "radial note",
    )
    .unwrap();
    let shape = DocumentationShape::new(
        41,
        DocumentationShapeGeometry::Line {
            start: Point::new(20, 0),
            end: Point::new(30, 0),
        },
    )
    .unwrap();
    state.design_notes.push(note);
    state.documentation_shapes.push(shape);
    state.selection.select_design_note(40);
    state.selection.select_documentation_shape(41);
    state.recalculate_runtime_state();
    state.clear_undo_history();
    state.is_dirty = false;
    let topology = state.topology_version();
    let naming = state
        .default_array_naming(SchematicArrayCount::parse("4 × 1").unwrap())
        .unwrap();
    assert!(naming.is_empty());
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::RadialDocumentation,
        SchematicArrayCount::parse("4 × 1").unwrap(),
        naming,
        SchematicArrayPlacement::Center(Point::origin()),
    )
    .unwrap();

    let preview = state.preview_array_selection(&plan).unwrap();
    assert_eq!(
        preview
            .design_notes
            .iter()
            .map(|note| note.pos)
            .collect::<Vec<_>>(),
        [Point::new(0, 10), Point::new(-10, 0), Point::new(0, -10)]
    );
    state.array_selection(&plan).unwrap();
    assert_eq!(state.topology_version(), topology);
    assert_eq!(state.design_notes.len(), 4);
    assert_eq!(state.documentation_shapes.len(), 4);
    assert!(state.undo());
    assert_eq!(state.design_notes.len(), 1);
    assert_eq!(state.documentation_shapes.len(), 1);
}

#[test]
fn collision_and_radial_electrical_rejections_are_atomic_true_no_ops() {
    let mut state = selected_resistor();
    let before = state.clone();
    let colliding = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "R1…R2",
        SchematicArrayPlacement::Pitch(Point::new(1, 0)),
    );
    assert!(matches!(
        state.array_selection(&colliding),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));
    let radial = plan(
        SchematicArrayKind::RadialDocumentation,
        "2 × 1",
        "R1…R2",
        SchematicArrayPlacement::Center(Point::new(100, 100)),
    );
    assert!(matches!(
        state.array_selection(&radial),
        Err(SchematicArrayError::RadialDocumentationOnly { .. })
    ));
    assert_eq!(state.components, before.components);
    assert_eq!(state.is_dirty, before.is_dirty);
    assert_eq!(state.topology_version(), before.topology_version());
    assert!(!state.can_undo());
}

#[test]
fn resolved_internal_wire_connections_are_remapped_per_replica() {
    let mut state = SchematicState::default();
    let left = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let right = state.add_component(ComponentType::Resistor, Point::new(20, 0));
    state.components[0].name = "R1".to_owned();
    state.components[1].name = "R10".to_owned();
    let wire_id = state.next_id();
    state
        .wires
        .push(Wire::segment(wire_id, Point::new(0, 0), Point::new(20, 0)));
    state.selection.select_component(left);
    state.selection.select_component(right);
    state.clear_undo_history();
    let plan = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "R1…R2 · R10…R11",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    );
    let terminals = |component: &Component| vec![("P".to_owned(), component.pos)];
    let bounds = |component: &Component| {
        (
            component.pos.x - 2,
            component.pos.y - 2,
            component.pos.x + 2,
            component.pos.y + 2,
        )
    };

    let preview = state
        .preview_array_selection_resolved(&plan, terminals, bounds)
        .unwrap();
    assert_eq!(preview.wires.len(), 1);
    assert_eq!(preview.connections.len(), 2);
    let generated_wire_id = preview.wires[0].id;
    assert!(
        preview
            .connections
            .iter()
            .all(|connection| connection.wire_id == generated_wire_id)
    );
    state
        .array_selection_resolved(&plan, terminals, bounds)
        .unwrap();
    assert_eq!(state.connections, preview.connections);
}

#[test]
fn bus_and_tap_ownership_and_scalar_slice_are_remapped_together() {
    let declaration = BusDeclaration::parse("DATA[3:0]").unwrap();
    let bus = Bus::segment(70, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap();
    let tap = BusTap::new(
        71,
        &bus,
        Point::new(10, 0),
        Point::new(10, 10),
        BusSlice::parse("DATA[0]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    let mut state = SchematicState::default();
    state.buses.push(bus);
    state.bus_taps.push(tap);
    state.recalculate_runtime_state();
    state.clear_undo_history();
    state.selection.select_only_bus_tap(71);
    let tap_naming = state
        .default_array_naming(SchematicArrayCount::parse("2 × 1").unwrap())
        .unwrap();
    assert_eq!(tap_naming.to_string(), "DATA[0]…DATA[1]");
    let tap_plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        SchematicArrayCount::parse("2 × 1").unwrap(),
        tap_naming,
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    assert_eq!(
        state.preview_array_selection(&tap_plan).unwrap().bus_taps[0]
            .slice
            .to_string(),
        "DATA[1]"
    );

    state.selection.select_only_bus(70);
    assert_eq!(
        state
            .default_array_naming(SchematicArrayCount::parse("2 × 1").unwrap())
            .unwrap()
            .to_string(),
        ""
    );
    let plan = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    );

    let preview = state.preview_array_selection(&plan).unwrap();
    assert_eq!(preview.buses.len(), 1);
    assert_eq!(preview.bus_taps.len(), 1);
    assert_eq!(preview.bus_taps[0].bus_id, preview.buses[0].id);
    assert_eq!(preview.bus_taps[0].slice.to_string(), "DATA[0]");
    state.array_selection(&plan).unwrap();
    assert_eq!(
        state.bus_taps.last().unwrap().bus_id,
        state.buses.last().unwrap().id
    );
}

#[test]
fn naming_collision_partial_stale_read_only_and_overflow_fail_closed() {
    let mut state = selected_resistor();
    state.components.push(
        Component::new(900, ComponentType::Capacitor, Point::new(500, 0))
            .with_name_value("R2", "1p"),
    );
    let count = SchematicArrayCount::parse("2 × 1").unwrap();
    let collision_free = state.default_array_naming(count).unwrap();
    assert_ne!(
        collision_free.value_for_source("R1", 1).as_deref(),
        Some("R2")
    );

    state.selection.select_wire_segment(123, 0);
    assert!(!state.has_live_array_selection());
    state.selection.wire_segments.clear();
    state.selection.select_component(999_999);
    let stale_plan = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "R1…R2",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    );
    assert!(matches!(
        state.preview_array_selection(&stale_plan),
        Err(SchematicArrayError::StaleSelection { .. })
    ));
    state.selection.components.remove(&999_999);
    state.read_only = true;
    assert!(matches!(
        state.array_selection(&stale_plan),
        Err(SchematicArrayError::ReadOnly)
    ));
    assert!(matches!(
        SchematicArrayCount::parse("4294967295 × 4294967295"),
        Err(SchematicArrayError::CountOverflow)
            | Err(SchematicArrayError::CountExceedsLimit { .. })
    ));
    let overflow = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "R1…R2",
        SchematicArrayPlacement::Pitch(Point::new(i32::MAX, 0)),
    );
    let mut overflow_state = selected_resistor();
    overflow_state.components[0].pos = Point::new(1, 0);
    assert!(matches!(
        overflow_state.preview_array_selection(&overflow),
        Err(SchematicArrayError::CoordinateOverflow)
    ));

    let forged = SchematicArrayPlan {
        kind: SchematicArrayKind::Linear,
        count: SchematicArrayCount::new(2, 2).unwrap(),
        naming: SchematicArrayNaming::parse("R1…R4").unwrap(),
        placement: SchematicArrayPlacement::Pitch(Point::new(100, 100)),
    };
    assert_eq!(
        overflow_state.preview_array_selection(&forged),
        Err(SchematicArrayError::LinearCountRequiresOneAxis)
    );
}

#[test]
fn strict_source_eligibility_rejects_mixed_stale_and_malformed_objects() {
    let mut stale = selected_resistor();
    assert_eq!(stale.validate_array_source_selection(), Ok(()));
    stale.selection.select_component(999_999);
    assert!(matches!(
        stale.validate_array_source_selection(),
        Err(SchematicArrayError::StaleSelection { object_id: 999_999 })
    ));

    stale.selection.clear();
    stale.selection.select_component(999_999);
    assert!(matches!(
        stale.validate_array_source_selection(),
        Err(SchematicArrayError::StaleSelection { object_id: 999_999 })
    ));

    let mut malformed = selected_resistor();
    let malformed_wire_id = malformed.next_id();
    malformed.wires.push(Wire::segment(
        malformed_wire_id,
        Point::new(20, 0),
        Point::new(20, 0),
    ));
    malformed.selection.select_wire(malformed_wire_id);
    assert!(matches!(
        malformed.validate_array_source_selection(),
        Err(SchematicArrayError::InvalidGeometry { object_id })
            if object_id == malformed_wire_id
    ));

    malformed.wires[0].points[1] = Point::new(30, 0);
    malformed.connections.push(WireConnection::new(
        malformed_wire_id,
        99,
        malformed.components[0].id,
        "A",
    ));
    assert!(matches!(
        malformed.validate_array_source_selection(),
        Err(SchematicArrayError::InvalidConnection(wire_id))
            if wire_id == malformed_wire_id
    ));

    let mut implicit = SchematicState::default();
    let left = implicit.add_component(ComponentType::Resistor, Point::new(0, 0));
    let right = implicit.add_component(ComponentType::Resistor, Point::new(20, 0));
    let implicit_wire_id = implicit.next_id();
    implicit.wires.push(Wire::segment(
        implicit_wire_id,
        Point::new(10, 0),
        Point::new(10, 0),
    ));
    implicit
        .connections
        .push(WireConnection::new(implicit_wire_id, 0, left, "A"));
    implicit
        .connections
        .push(WireConnection::new(implicit_wire_id, 1, right, "B"));
    implicit.selection.select_component(left);
    implicit.selection.select_component(right);
    assert!(matches!(
        implicit.validate_array_source_selection(),
        Err(SchematicArrayError::InvalidGeometry { object_id })
            if object_id == implicit_wire_id
    ));
}

#[test]
fn default_group_naming_interleaves_same_prefix_without_collisions() {
    let mut state = SchematicState::default();
    let first = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let second = state.add_component(ComponentType::Resistor, Point::new(0, 30));
    state.selection.select_component(first);
    state.selection.select_component(second);
    let count = SchematicArrayCount::parse("8 × 1").unwrap();
    let naming = state.default_array_naming(count).unwrap();
    assert_eq!(naming.to_string(), "R1…R15 · R2…R16");
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        naming,
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    let preview = state.preview_array_selection(&plan).unwrap();
    let names: HashSet<_> = preview
        .components
        .iter()
        .map(|component| component.name.as_str())
        .collect();
    assert_eq!(names.len(), 14);
    assert!(names.contains("R15"));
    assert!(names.contains("R16"));

    let mut sparse = SchematicState::default();
    let first = sparse.add_component(ComponentType::Resistor, Point::new(0, 0));
    let second = sparse.add_component(ComponentType::Resistor, Point::new(0, 30));
    sparse.components[1].name = "R5".to_owned();
    sparse.selection.select_component(first);
    sparse.selection.select_component(second);
    let sparse_naming = sparse.default_array_naming(count).unwrap();
    let sparse_plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        sparse_naming,
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    let sparse_names: HashSet<_> = sparse
        .preview_array_selection(&sparse_plan)
        .unwrap()
        .components
        .into_iter()
        .map(|component| component.name)
        .collect();
    assert_eq!(sparse_names.len(), 14);

    let mut occupied = selected_resistor();
    occupied.components.push(
        Component::new(800, ComponentType::Capacitor, Point::new(500, 0))
            .with_name_value("R3", "1p"),
    );
    let occupied_naming = occupied
        .default_array_naming(SchematicArrayCount::parse("4 × 1").unwrap())
        .unwrap();
    assert!(
        (0..4)
            .all(|member| occupied_naming.value_for_source("R1", member).as_deref() != Some("R3"))
    );
}

#[test]
fn default_bus_index_naming_interleaves_selected_scalar_labels() {
    let mut state = SchematicState::default();
    state
        .net_labels
        .push(NetLabel::new(60, Point::new(0, 0), "DATA[0]"));
    state
        .net_labels
        .push(NetLabel::new(61, Point::new(0, 20), "DATA[1]"));
    state.selection.select_net_label(60);
    state.selection.select_net_label(61);
    state.recalculate_runtime_state();
    let count = SchematicArrayCount::parse("4 × 1").unwrap();
    let naming = state.default_array_naming(count).unwrap();
    assert_eq!(naming.to_string(), "DATA[0]…DATA[6] · DATA[1]…DATA[7]");
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        naming,
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    let preview = state.preview_array_selection(&plan).unwrap();
    let names: HashSet<_> = preview
        .net_labels
        .iter()
        .map(|label| label.name.as_str())
        .collect();
    assert_eq!(names.len(), 6);
    assert!(names.contains("DATA[6]"));
    assert!(names.contains("DATA[7]"));
}

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
fn radial_zero_radius_and_quantized_overlap_are_atomic_rejections() {
    let mut zero_radius = SchematicState::default();
    zero_radius
        .design_notes
        .push(DesignNote::new(80, Point::origin(), DesignNoteKind::PlainText, "centered").unwrap());
    zero_radius.selection.select_design_note(80);
    zero_radius.recalculate_runtime_state();
    zero_radius.clear_undo_history();
    zero_radius.is_dirty = false;
    let zero_before = zero_radius.clone();
    let zero_count = SchematicArrayCount::new(4, 1).unwrap();
    let zero_plan = SchematicArrayPlan::new(
        SchematicArrayKind::RadialDocumentation,
        zero_count,
        zero_radius.default_array_naming(zero_count).unwrap(),
        SchematicArrayPlacement::Center(Point::origin()),
    )
    .unwrap();

    assert!(matches!(
        zero_radius.array_selection(&zero_plan),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));
    assert_eq!(zero_radius.design_notes, zero_before.design_notes);
    assert_eq!(zero_radius.is_dirty, zero_before.is_dirty);
    assert!(!zero_radius.can_undo());

    let mut quantized = SchematicState::default();
    quantized.design_notes.push(
        DesignNote::new(
            81,
            Point::new(1, 0),
            DesignNoteKind::PlainText,
            "small radius",
        )
        .unwrap(),
    );
    quantized.selection.select_design_note(81);
    quantized.recalculate_runtime_state();
    quantized.clear_undo_history();
    quantized.is_dirty = false;
    let quantized_before = quantized.clone();
    let quantized_count = SchematicArrayCount::new(360, 1).unwrap();
    let quantized_plan = SchematicArrayPlan::new(
        SchematicArrayKind::RadialDocumentation,
        quantized_count,
        quantized.default_array_naming(quantized_count).unwrap(),
        SchematicArrayPlacement::Center(Point::origin()),
    )
    .unwrap();

    assert!(matches!(
        quantized.array_selection(&quantized_plan),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));
    assert_eq!(quantized.design_notes, quantized_before.design_notes);
    assert_eq!(quantized.is_dirty, quantized_before.is_dirty);
    assert!(!quantized.can_undo());
}

#[test]
fn symmetric_radial_shape_overlap_is_rejected_even_when_points_reverse() {
    let mut state = SchematicState::default();
    state.documentation_shapes.push(
        DocumentationShape::new(
            90,
            DocumentationShapeGeometry::Line {
                start: Point::new(-10, 0),
                end: Point::new(10, 0),
            },
        )
        .unwrap(),
    );
    state.selection.select_documentation_shape(90);
    state.recalculate_runtime_state();
    state.clear_undo_history();
    state.is_dirty = false;
    let before = state.clone();
    let count = SchematicArrayCount::new(2, 1).unwrap();
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::RadialDocumentation,
        count,
        state.default_array_naming(count).unwrap(),
        SchematicArrayPlacement::Center(Point::origin()),
    )
    .unwrap();

    assert!(matches!(
        state.array_selection(&plan),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));
    assert_eq!(state.documentation_shapes, before.documentation_shapes);
    assert_eq!(state.is_dirty, before.is_dirty);
    assert!(!state.can_undo());
}

#[test]
fn snapped_durable_connections_capture_implicit_wire_and_close_selection_symmetrically() {
    let mut state = SchematicState::default();
    let left = state.add_component(ComponentType::Resistor, Point::new(0, 0));
    let right = state.add_component(ComponentType::Resistor, Point::new(20, 0));
    let wire_id = state.next_id();
    state
        .wires
        .push(Wire::segment(wire_id, Point::new(1, 0), Point::new(19, 0)));
    state
        .connections
        .push(WireConnection::new(wire_id, 0, left, "A"));
    state
        .connections
        .push(WireConnection::new(wire_id, 1, right, "B"));
    state.selection.select_component(left);
    state.selection.select_component(right);
    state.clear_undo_history();
    let count = SchematicArrayCount::new(2, 1).unwrap();
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        state.default_array_naming(count).unwrap(),
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    let terminals = |component: &Component| vec![("resolved".to_owned(), component.pos)];
    let bounds = |component: &Component| {
        (
            component.pos.x - 2,
            component.pos.y - 2,
            component.pos.x + 2,
            component.pos.y + 2,
        )
    };

    let preview = state
        .preview_array_selection_resolved(&plan, terminals, bounds)
        .unwrap();
    assert_eq!(preview.wires.len(), 1);
    assert_eq!(preview.connections.len(), 2);
    assert_eq!(preview.connections[0].terminal_name, "A");
    assert_eq!(preview.connections[1].terminal_name, "B");
    assert!(preview.selection.has_wire(wire_id));
    assert!(preview.selection.has_wire(preview.wires[0].id));
    state
        .array_selection_resolved(&plan, terminals, bounds)
        .unwrap();
    assert_eq!(state.connections.len(), 4);
    assert!(state.selection.has_wire(wire_id));
    assert!(state.selection.has_wire(preview.wires[0].id));
}

#[test]
fn explicitly_selected_wire_preserves_durable_terminal_ownership_without_duplicates() {
    let mut state = SchematicState::default();
    let component_id = state.add_component(ComponentType::Resistor, Point::origin());
    let wire_id = state.next_id();
    state
        .wires
        .push(Wire::segment(wire_id, Point::origin(), Point::new(20, 0)));
    state.connections.push(WireConnection::new(
        wire_id,
        0,
        component_id,
        "durable-terminal",
    ));
    state.selection.select_component(component_id);
    state.selection.select_wire(wire_id);
    state.clear_undo_history();
    let count = SchematicArrayCount::new(2, 1).unwrap();
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        state.default_array_naming(count).unwrap(),
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    let terminals = |component: &Component| vec![("resolved-terminal".to_owned(), component.pos)];
    let bounds = |component: &Component| {
        (
            component.pos.x - 2,
            component.pos.y - 2,
            component.pos.x + 2,
            component.pos.y + 2,
        )
    };

    let preview = state
        .preview_array_selection_resolved(&plan, terminals, bounds)
        .unwrap();
    assert_eq!(preview.connections.len(), 1);
    assert_eq!(preview.connections[0].terminal_name, "durable-terminal");
    assert!(preview.selection.has_wire(wire_id));
    assert!(preview.selection.has_wire(preview.wires[0].id));
}

#[test]
fn resolved_authored_bounds_enforce_symmetric_body_conductor_and_anchor_collisions() {
    let bounds = |component: &Component| {
        (
            component.pos.x - 20,
            component.pos.y - 20,
            component.pos.x + 20,
            component.pos.y + 20,
        )
    };
    let terminals = |_component: &Component| Vec::<(String, Point)>::new();

    let mut component_case = selected_resistor();
    component_case
        .wires
        .push(Wire::segment(700, Point::new(90, 0), Point::new(110, 0)));
    let count = SchematicArrayCount::new(2, 1).unwrap();
    let component_plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        component_case.default_array_naming(count).unwrap(),
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    assert!(matches!(
        component_case.preview_array_selection_resolved(&component_plan, terminals, bounds),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));

    let mut conductor_case = SchematicState::default();
    conductor_case
        .wires
        .push(Wire::segment(710, Point::new(-5, 0), Point::new(5, 0)));
    conductor_case.selection.select_only_wire(710);
    conductor_case.components.push(
        Component::new(711, ComponentType::Capacitor, Point::new(100, 0))
            .with_name_value("C1", "1p"),
    );
    let conductor_plan = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    );
    assert!(matches!(
        conductor_case.preview_array_selection_resolved(&conductor_plan, terminals, bounds),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));

    let mut anchor_case = SchematicState::default();
    anchor_case
        .net_labels
        .push(NetLabel::new(720, Point::origin(), "SIGNAL"));
    anchor_case.selection.select_net_label(720);
    anchor_case.components.push(
        Component::new(721, ComponentType::Capacitor, Point::new(100, 0))
            .with_name_value("C1", "1p"),
    );
    let anchor_plan = plan(
        SchematicArrayKind::Linear,
        "2 × 1",
        "",
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    );
    assert!(matches!(
        anchor_case.preview_array_selection_resolved(&anchor_plan, terminals, bounds),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));

    let mut body_case = selected_resistor();
    body_case
        .net_labels
        .push(NetLabel::new(730, Point::new(100, 0), "SIGNAL"));
    let body_plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        body_case.default_array_naming(count).unwrap(),
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    assert!(matches!(
        body_case.preview_array_selection_resolved(&body_plan, terminals, bounds),
        Err(SchematicArrayError::GeometryCollision { .. })
    ));
}

#[test]
fn typed_reference_defaults_support_suffixes_and_zero_padding() {
    let mut state = SchematicState::default();
    state.components.push(
        Component::new(501, ComponentType::CellInstance, Point::new(0, 0))
            .with_name_value("X3A", "cell"),
    );
    state.components.push(
        Component::new(502, ComponentType::CellInstance, Point::new(0, 30))
            .with_name_value("X03B", "cell"),
    );
    state.selection.select_component(501);
    state.selection.select_component(502);
    state.recalculate_runtime_state();
    let count = SchematicArrayCount::new(8, 1).unwrap();
    let naming = state.default_array_naming(count).unwrap();
    assert_eq!(naming.value_for_source("X3A", 7).as_deref(), Some("X10A"));
    assert_eq!(naming.value_for_source("X03B", 1).as_deref(), Some("X04B"));
    assert_eq!(naming.value_for_source("X03B", 7).as_deref(), Some("X10B"));
}

#[test]
fn duplicate_scalar_bus_sources_share_one_canonical_naming_range() {
    let mut state = SchematicState::default();
    state
        .net_labels
        .push(NetLabel::new(601, Point::new(0, 0), "DATA[0]"));
    state
        .net_labels
        .push(NetLabel::new(602, Point::new(0, 20), "data[0]"));
    state.selection.select_net_label(601);
    state.selection.select_net_label(602);
    state.recalculate_runtime_state();
    let count = SchematicArrayCount::new(4, 1).unwrap();
    let naming = state.default_array_naming(count).unwrap();
    assert_eq!(naming.len(), 1);
    assert_eq!(
        naming.value_for_source("DATA[0]", 3).as_deref(),
        Some("DATA[3]")
    );
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        naming,
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    assert_eq!(
        state
            .preview_array_selection(&plan)
            .unwrap()
            .net_labels
            .len(),
        6
    );
}

#[test]
fn generated_object_and_segment_budgets_reject_before_allocation() {
    let mut object_heavy = SchematicState::default();
    for index in 0..17u64 {
        object_heavy.design_notes.push(
            DesignNote::new(
                800 + index,
                Point::new(index as i32 * 10, 0),
                DesignNoteKind::PlainText,
                format!("note {index}"),
            )
            .unwrap(),
        );
        object_heavy.selection.select_design_note(800 + index);
    }
    object_heavy.recalculate_runtime_state();
    let count = SchematicArrayCount::new(4_096, 1).unwrap();
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        SchematicArrayNaming::default(),
        SchematicArrayPlacement::Pitch(Point::new(100, 0)),
    )
    .unwrap();
    assert!(matches!(
        object_heavy.preview_array_selection(&plan),
        Err(SchematicArrayError::GeneratedObjectBudgetExceeded { .. })
    ));

    let mut segment_heavy = SchematicState::default();
    segment_heavy.wires.push(Wire::new(
        900,
        (0..35).map(|index| Point::new(index * 10, 0)).collect(),
    ));
    segment_heavy.selection.select_only_wire(900);
    segment_heavy.recalculate_runtime_state();
    assert!(matches!(
        segment_heavy.preview_array_selection(&plan),
        Err(SchematicArrayError::GeneratedSegmentBudgetExceeded { .. })
    ));
}
