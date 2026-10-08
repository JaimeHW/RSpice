//! Stretching.
//!
//! Moving one edge or vertex of an object while the rest stays put — the
//! operation that resizes a shape or reroutes one leg of a wire without
//! detaching either end.

use super::super::*;
use rspice_design::schematic::stretch;
use rspice_schematic_editor::session::stretch as editor_stretch;

impl SchematicState {
    /// Whether the current selection contains at least one live stretch handle.
    pub fn has_live_stretch_selection(&self) -> bool {
        editor_stretch::has_live_selection(self.design.document(), &self.session.editor.selection)
    }

    /// Resolve an unambiguous default handle from the current frozen selection.
    /// Pointer-driven callers may instead construct an exact target and validate
    /// it with [`Self::is_stretch_target_eligible`].
    pub fn default_stretch_target(&self) -> Option<StretchTarget> {
        editor_stretch::default_target(self.design.document(), &self.session.editor.selection)
    }

    /// Prove that an exact live handle belongs to the current frozen selection.
    pub fn is_stretch_target_eligible(&self, target: StretchTarget) -> bool {
        editor_stretch::target_is_eligible(
            self.design.document(),
            &self.session.editor.selection,
            target,
        )
    }

    /// Stretch one exact selected segment or typed shape control point.
    ///
    /// Every electrical edit is built and validated on a candidate first. An
    /// error therefore leaves geometry, taps, connections, dirty state, and the
    /// topology epoch unchanged.
    pub fn stretch_target(
        &mut self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
    ) -> Result<bool, StretchSelectionError> {
        self.stretch_target_resolved(
            delta,
            target,
            policy,
            |component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(|(_, point)| point)
                    .collect()
            },
            Component::bounding_box,
        )
    }

    /// Build the exact validated preview candidate using durable core terminal
    /// geometry. `Ok(None)` is the same clean no-op contract as commit.
    pub fn preview_stretch_target(
        &self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
    ) -> Result<Option<SchematicDocument>, StretchSelectionError> {
        self.preview_stretch_target_resolved(
            delta,
            target,
            policy,
            |component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(|(_, point)| point)
                    .collect()
            },
            Component::bounding_box,
        )
    }

    /// Preserve editor eligibility and error precedence before document editing.
    fn stretch_request_is_selected(
        &self,
        delta: Point,
        target: StretchTarget,
    ) -> Result<bool, StretchSelectionError> {
        if self.session.read_only || delta == Point::origin() {
            return Ok(false);
        }
        if !self.session.editor.selection.probes.is_empty() {
            return Err(StretchSelectionError::ProbeSelectionUnsupported);
        }
        if !stretch::target_is_live(self.design.document(), target) {
            return Err(StretchSelectionError::StaleTarget);
        }
        Ok(editor_stretch::selection_authorizes_target(
            &self.session.editor.selection,
            target,
        ))
    }

    /// Stretch using caller-resolved component geometry.
    pub fn stretch_target_resolved(
        &mut self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<bool, StretchSelectionError> {
        if !self.stretch_request_is_selected(delta, target)? {
            return Ok(false);
        }
        let changed = self.design.stretch_target_resolved(
            delta,
            target,
            policy,
            terminal_points_for,
            component_bounds_for,
        )?;
        if changed {
            self.session.is_dirty = true;
        }
        Ok(changed)
    }

    /// Build the validated document preview without cloning editor state.
    pub fn preview_stretch_target_resolved(
        &self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<Option<SchematicDocument>, StretchSelectionError> {
        if !self.stretch_request_is_selected(delta, target)? {
            return Ok(None);
        }
        stretch::preview_stretch_target_resolved(
            self.design.document(),
            delta,
            target,
            policy,
            terminal_points_for,
            component_bounds_for,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::DocumentationShapeGeometry;
    use super::*;
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
        DocumentationShape, Junction, NetLabel, SchematicSnapshot, Wire, WireConnection,
    };

    fn wire_target(wire_id: u64, segment_index: usize) -> StretchTarget {
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        }
    }

    fn bus_target(bus_id: u64, segment_index: usize) -> StretchTarget {
        StretchTarget::BusSegment {
            bus_id,
            segment_index,
        }
    }

    fn u_wire(id: u64) -> Wire {
        Wire::new(
            id,
            vec![
                Point::new(0, 0),
                Point::new(0, 10),
                Point::new(20, 10),
                Point::new(20, 0),
            ],
        )
    }

    fn selected_u_wire() -> SchematicState {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires.push(u_wire(1));
        state
            .session
            .editor
            .selection
            .select_only_wire_segment(1, 1);
        state
    }

    #[test]
    fn orthogonal_policy_labels_and_order_match_the_mockup() {
        assert_eq!(
            StretchOrthogonalPolicy::default().label(),
            "Preserve orthogonal"
        );
        assert_eq!(
            StretchOrthogonalPolicy::ALL.map(StretchOrthogonalPolicy::label),
            ["Preserve orthogonal", "Allow diagonal"]
        );

        let state = selected_u_wire();
        assert!(state.has_live_stretch_selection());
        assert_eq!(state.default_stretch_target(), Some(wire_target(1, 1)));
        assert!(state.is_stretch_target_eligible(wire_target(1, 1)));
    }

    #[test]
    fn exact_vertex_selection_resolves_an_incident_segment() {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires.push(u_wire(1));
        state.session.editor.selection.select_only_wire_vertex(1, 2);
        assert_eq!(state.default_stretch_target(), Some(wire_target(1, 2)));
        assert!(state.is_stretch_target_eligible(wire_target(1, 1)));
        assert!(state.is_stretch_target_eligible(wire_target(1, 2)));
    }

    #[test]
    fn stretches_horizontal_interior_segment_and_keeps_outer_anchors_fixed() {
        let mut state = selected_u_wire();
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(true)
        );
        assert_eq!(
            state.design.document().wires[0].points,
            [
                Point::new(0, 0),
                Point::new(0, 15),
                Point::new(20, 15),
                Point::new(20, 0),
            ]
        );
        assert!(state.design.document().wires[0].is_orthogonal());
    }

    #[test]
    fn stretches_vertical_interior_segment_orthogonally() {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires.push(Wire::new(
            1,
            vec![
                Point::new(0, 0),
                Point::new(10, 0),
                Point::new(10, 20),
                Point::new(0, 20),
            ],
        ));
        state
            .session
            .editor
            .selection
            .select_only_wire_segment(1, 1);
        state
            .stretch_target(
                Point::new(5, 0),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(
            state.design.document().wires[0].points[1],
            Point::new(15, 0)
        );
        assert_eq!(
            state.design.document().wires[0].points[2],
            Point::new(15, 20)
        );
        assert_eq!(state.design.document().wires[0].points[0], Point::new(0, 0));
        assert_eq!(
            state.design.document().wires[0].points[3],
            Point::new(0, 20)
        );
    }

    #[test]
    fn first_last_and_deep_interior_segments_do_not_self_reject() {
        let cases = [
            (
                vec![
                    Point::new(0, 0),
                    Point::new(10, 0),
                    Point::new(10, 20),
                    Point::new(20, 20),
                ],
                0,
                Point::new(0, 5),
            ),
            (
                vec![
                    Point::new(0, 20),
                    Point::new(10, 20),
                    Point::new(10, 0),
                    Point::new(20, 0),
                ],
                2,
                Point::new(0, 5),
            ),
            (
                vec![
                    Point::new(-20, 0),
                    Point::new(-20, 20),
                    Point::new(0, 20),
                    Point::new(0, 10),
                    Point::new(20, 10),
                    Point::new(20, 20),
                    Point::new(40, 20),
                    Point::new(40, 0),
                ],
                3,
                Point::new(0, 5),
            ),
        ];
        for (points, segment_index, delta) in cases {
            let mut state = SchematicState::default();
            state
                .design
                .document_mut_for_test()
                .wires
                .push(Wire::new(1, points));
            state
                .session
                .editor
                .selection
                .select_only_wire_segment(1, segment_index);
            assert_eq!(
                state.stretch_target(
                    delta,
                    wire_target(1, segment_index),
                    StretchOrthogonalPolicy::PreserveOrthogonal,
                ),
                Ok(true)
            );
        }
    }

    #[test]
    fn preserve_orthogonal_rejects_parallel_or_nonorthogonal_contracts() {
        let mut state = selected_u_wire();
        assert_eq!(
            state.stretch_target(
                Point::new(5, 0),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::PerpendicularDeltaRequired)
        );

        let mut diagonal = SchematicState::default();
        diagonal
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::new(2, vec![Point::new(0, 0), Point::new(10, 10)]));
        diagonal.session.editor.selection.select_only_wire(2);
        assert_eq!(
            diagonal.stretch_target(
                Point::new(0, 5),
                wire_target(2, 0),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::NonOrthogonalSource { object_id: 2 })
        );
    }

    #[test]
    fn allow_diagonal_preserves_exact_requested_geometry() {
        let mut state = selected_u_wire();
        state
            .stretch_target(
                Point::new(5, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::AllowDiagonal,
            )
            .unwrap();
        assert_eq!(
            state.design.document().wires[0].points[1],
            Point::new(5, 15)
        );
        assert_eq!(
            state.design.document().wires[0].points[2],
            Point::new(25, 15)
        );
        assert!(!state.design.document().wires[0].is_orthogonal());
        assert!(state.design.document().wires[0].contains_point(Point::new(15, 15)));
    }

    #[test]
    fn preview_is_nonmutating_and_is_the_exact_commit_candidate() {
        let mut state = selected_u_wire();
        let before = SchematicSnapshot::capture(state.design.document());
        let preview = state
            .preview_stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap()
            .unwrap();
        assert!(before.is_equal(&SchematicSnapshot::capture(state.design.document())));
        state
            .stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(
            state.design.document().wires[0].points,
            preview.wires[0].points
        );
        assert_eq!(state.design.document().bus_taps, preview.bus_taps);
    }

    #[test]
    fn diagonal_contains_point_is_overflow_safe() {
        let wire = Wire::segment(
            1,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
        );
        assert!(wire.contains_point(Point::origin()));
        assert!(!wire.contains_point(Point::new(0, 1)));
    }

    #[test]
    fn component_connection_record_blocks_a_moved_endpoint() {
        let mut state = selected_u_wire();
        state
            .design
            .document_mut_for_test()
            .connections
            .push(WireConnection::new(1, 1, 44, "OUT"));
        assert!(matches!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::ConnectedTerminal {
                component_id: 44,
                ..
            })
        ));
    }

    #[test]
    fn geometric_component_terminal_on_source_segment_blocks_stretch() {
        let mut state = SchematicState::default();
        let component = Component::new(20, ComponentType::Resistor, Point::origin());
        let terminal = component.terminal_positions()[0].1;
        state
            .design
            .document_mut_for_test()
            .components
            .push(component);
        state
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(
                1,
                terminal,
                Point::new(terminal.x, terminal.y + 20),
            ));
        state.session.editor.selection.select_only_wire(1);
        assert!(matches!(
            state.stretch_target(
                Point::new(5, 0),
                wire_target(1, 0),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::ConnectedTerminal {
                component_id: 20,
                ..
            })
        ));
    }

    #[test]
    fn unrelated_wire_bus_and_junction_anchors_fail_closed() {
        for kind in 0..3 {
            let mut state = selected_u_wire();
            match kind {
                0 => state
                    .design
                    .document_mut_for_test()
                    .wires
                    .push(Wire::segment(2, Point::new(10, 10), Point::new(10, 30))),
                1 => {
                    state.design.document_mut_for_test().buses.push(
                        Bus::segment(2, Point::new(10, 10), Point::new(10, 30), None).unwrap(),
                    )
                }
                _ => state
                    .design
                    .document_mut_for_test()
                    .junctions
                    .push(Junction::new(2, Point::new(10, 10))),
            }
            assert!(matches!(
                state.stretch_target(
                    Point::new(0, 5),
                    wire_target(1, 1),
                    StretchOrthogonalPolicy::PreserveOrthogonal,
                ),
                Err(StretchSelectionError::FixedAnchor { .. })
            ));
        }
    }

    #[test]
    fn existing_and_new_net_label_anchors_are_rejected_atomically() {
        let mut existing = selected_u_wire();
        existing
            .design
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(70, Point::new(10, 10), "SENSE"));
        let before = SchematicSnapshot::capture(existing.design.document());
        assert_eq!(
            existing.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::NetLabelAnchor {
                label_id: 70,
                point: Point::new(10, 10),
            })
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(existing.design.document())));

        let mut new_contact = selected_u_wire();
        new_contact
            .design
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(71, Point::new(10, 15), "OTHER"));
        let before = SchematicSnapshot::capture(new_contact.design.document());
        assert_eq!(
            new_contact.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::NetLabelAnchor {
                label_id: 71,
                point: Point::new(10, 15),
            })
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(new_contact.design.document())));
    }

    fn declared_bus(id: u64, points: Vec<Point>) -> Bus {
        Bus::new(
            id,
            points,
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap()
    }

    #[test]
    fn tap_on_stretched_wire_segment_follows_and_keeps_source_fixed() {
        let mut state = selected_u_wire();
        let bus = declared_bus(5, vec![Point::new(0, -10), Point::new(20, -10)]);
        let tap = BusTap::new(
            6,
            &bus,
            Point::new(10, -10),
            Point::new(10, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        state.design.document_mut_for_test().buses.push(bus);
        state.design.document_mut_for_test().bus_taps.push(tap);
        state
            .stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(
            state.design.document().bus_taps[0].bus_point,
            Point::new(10, -10)
        );
        assert_eq!(
            state.design.document().bus_taps[0].connection_point,
            Point::new(10, 15)
        );
    }

    #[test]
    fn tap_stretch_rejects_when_the_two_ends_would_collapse() {
        let mut state = selected_u_wire();
        let bus = declared_bus(5, vec![Point::new(0, 15), Point::new(20, 15)]);
        let tap = BusTap::new(
            6,
            &bus,
            Point::new(10, 15),
            Point::new(10, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        state.design.document_mut_for_test().buses.push(bus);
        state.design.document_mut_for_test().bus_taps.push(tap);
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::InvalidTapAttachment { tap_id: 6 })
        );
    }

    #[test]
    fn bus_segment_stretches_and_its_source_tap_follows() {
        let mut state = SchematicState::default();
        let bus = declared_bus(
            5,
            vec![
                Point::new(0, 0),
                Point::new(0, 10),
                Point::new(20, 10),
                Point::new(20, 0),
            ],
        );
        let tap = BusTap::new(
            6,
            &bus,
            Point::new(10, 10),
            Point::new(10, 30),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        state
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(9, Point::new(0, 30), Point::new(20, 30)));
        state.design.document_mut_for_test().buses.push(bus);
        state.design.document_mut_for_test().bus_taps.push(tap);
        state.session.editor.selection.select_bus(5);
        state
            .stretch_target(
                Point::new(0, 5),
                bus_target(5, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(
            state.design.document().buses[0].points[1],
            Point::new(0, 15)
        );
        assert_eq!(
            state.design.document().buses[0].points[2],
            Point::new(20, 15)
        );
        assert_eq!(
            state.design.document().bus_taps[0].bus_point,
            Point::new(10, 15)
        );
        assert_eq!(
            state.design.document().bus_taps[0].connection_point,
            Point::new(10, 30)
        );
    }

    fn shape_geometries() -> Vec<(DocumentationShapeGeometry, usize, Point)> {
        vec![
            (
                DocumentationShapeGeometry::Rectangle {
                    first: Point::new(0, 0),
                    opposite: Point::new(10, 10),
                },
                0,
                Point::new(-1, 0),
            ),
            (
                DocumentationShapeGeometry::Line {
                    start: Point::new(0, 0),
                    end: Point::new(10, 0),
                },
                1,
                Point::new(11, 0),
            ),
            (
                DocumentationShapeGeometry::Polygon {
                    points: vec![Point::new(0, 0), Point::new(10, 0), Point::new(0, 10)],
                },
                1,
                Point::new(11, 0),
            ),
            (
                DocumentationShapeGeometry::Arc {
                    start: Point::new(10, 0),
                    through: Point::new(0, -10),
                    end: Point::new(0, 10),
                },
                0,
                Point::new(9, 0),
            ),
            (
                DocumentationShapeGeometry::Callout {
                    tip: Point::new(0, 0),
                    elbow: Point::new(10, 10),
                    box_corner: Point::new(20, 20),
                },
                0,
                Point::new(-1, 0),
            ),
        ]
    }

    #[test]
    fn every_typed_documentation_shape_control_point_can_stretch() {
        for (geometry, point_index, expected) in shape_geometries() {
            let mut state = SchematicState::default();
            state
                .design
                .document_mut_for_test()
                .documentation_shapes
                .push(DocumentationShape::new(7, geometry).unwrap());
            state.session.editor.selection.select_documentation_shape(7);
            let before_topology = state.topology_version();
            state
                .stretch_target(
                    Point::new(if point_index == 0 { -1 } else { 1 }, 0),
                    StretchTarget::DocumentationShapePoint {
                        shape_id: 7,
                        point_index,
                    },
                    StretchOrthogonalPolicy::PreserveOrthogonal,
                )
                .unwrap();
            assert_eq!(
                state.design.document().documentation_shapes[0]
                    .geometry
                    .points()[point_index],
                expected
            );
            assert_eq!(state.topology_version(), before_topology);
            assert!(state.session.is_dirty);
        }
    }

    #[test]
    fn invalid_shape_control_point_is_atomic() {
        let mut state = SchematicState::default();
        state
            .design
            .document_mut_for_test()
            .documentation_shapes
            .push(
                DocumentationShape::new(
                    7,
                    DocumentationShapeGeometry::Line {
                        start: Point::new(0, 0),
                        end: Point::new(10, 0),
                    },
                )
                .unwrap(),
            );
        state.session.editor.selection.select_documentation_shape(7);
        let before = SchematicSnapshot::capture(state.design.document());
        assert_eq!(
            state.stretch_target(
                Point::new(10, 0),
                StretchTarget::DocumentationShapePoint {
                    shape_id: 7,
                    point_index: 0,
                },
                StretchOrthogonalPolicy::AllowDiagonal,
            ),
            Err(StretchSelectionError::InvalidDocumentationGeometry { shape_id: 7 })
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(state.design.document())));
    }

    #[test]
    fn stale_target_and_coordinate_overflow_are_rejected_atomically() {
        let mut state = selected_u_wire();
        let before = SchematicSnapshot::capture(state.design.document());
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 99),
                StretchOrthogonalPolicy::AllowDiagonal,
            ),
            Err(StretchSelectionError::StaleTarget)
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(state.design.document())));

        state.design.document_mut_for_test().wires[0].points[1].y = i32::MAX;
        state.design.document_mut_for_test().wires[0].points[2].y = i32::MAX;
        let before = SchematicSnapshot::capture(state.design.document());
        assert_eq!(
            state.stretch_target(
                Point::new(0, 1),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::CoordinateOverflow)
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(state.design.document())));
    }

    #[test]
    fn read_only_zero_delta_and_unselected_target_are_clean_noops() {
        let mut state = selected_u_wire();
        let baseline = SchematicSnapshot::capture(state.design.document());
        state.session.read_only = true;
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(false)
        );
        state.session.read_only = false;
        assert_eq!(
            state.stretch_target(
                Point::origin(),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(false)
        );
        state.session.editor.selection.clear();
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(false)
        );
        assert!(baseline.is_equal(&SchematicSnapshot::capture(state.design.document())));
    }

    #[test]
    fn new_overlap_and_endpoint_contact_are_rejected() {
        let mut overlap = selected_u_wire();
        overlap
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(2, Point::new(5, 15), Point::new(15, 15)));
        assert!(matches!(
            overlap.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::ConductorOverlap { .. })
        ));

        let mut contact = selected_u_wire();
        contact
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(2, Point::new(10, 15), Point::new(10, 25)));
        assert!(matches!(
            contact.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::UnintendedConductorContact { .. })
        ));
    }

    #[test]
    fn new_component_terminal_contact_is_rejected() {
        let component = Component::new(20, ComponentType::Resistor, Point::new(10, 15));
        let terminal = component.terminal_positions()[0].1;
        let mut state = SchematicState::default();
        state
            .design
            .document_mut_for_test()
            .components
            .push(component);
        state.design.document_mut_for_test().wires.push(Wire::new(
            1,
            vec![
                Point::new(terminal.x - 5, terminal.y - 10),
                Point::new(terminal.x - 5, terminal.y - 5),
                Point::new(terminal.x + 5, terminal.y - 5),
                Point::new(terminal.x + 5, terminal.y - 10),
            ],
        ));
        state
            .session
            .editor
            .selection
            .select_only_wire_segment(1, 1);
        assert!(matches!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::UnintendedTerminalContact {
                component_id: 20,
                ..
            }) | Err(StretchSelectionError::ComponentBodyEntry {
                component_id: 20,
                ..
            })
        ));
    }

    #[test]
    fn caller_resolved_authored_terminal_blocks_source_and_new_contacts() {
        let mut source_contact = selected_u_wire();
        source_contact
            .design
            .document_mut_for_test()
            .components
            .push(Component::new(
                20,
                ComponentType::CellInstance,
                Point::new(100, 100),
            ));
        assert!(matches!(
            source_contact.stretch_target_resolved(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
                |component| {
                    (component.id == 20)
                        .then_some(vec![Point::new(10, 10)])
                        .unwrap_or_default()
                },
                Component::bounding_box,
            ),
            Err(StretchSelectionError::ConnectedTerminal {
                component_id: 20,
                ..
            })
        ));

        let mut new_contact = selected_u_wire();
        new_contact
            .design
            .document_mut_for_test()
            .components
            .push(Component::new(
                20,
                ComponentType::CellInstance,
                Point::new(100, 100),
            ));
        assert!(matches!(
            new_contact.stretch_target_resolved(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
                |component| {
                    (component.id == 20)
                        .then_some(vec![Point::new(10, 15)])
                        .unwrap_or_default()
                },
                Component::bounding_box,
            ),
            Err(StretchSelectionError::UnintendedTerminalContact {
                component_id: 20,
                ..
            })
        ));
    }

    #[test]
    fn caller_resolved_authored_body_bounds_block_new_entry() {
        let mut state = selected_u_wire();
        state
            .design
            .document_mut_for_test()
            .components
            .push(Component::new(
                20,
                ComponentType::CellInstance,
                Point::new(100, 100),
            ));
        assert_eq!(
            state.stretch_target_resolved(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
                |_| Vec::new(),
                |component| {
                    if component.id == 20 {
                        (5, 14, 15, 18)
                    } else {
                        component.bounding_box()
                    }
                },
            ),
            Err(StretchSelectionError::ComponentBodyEntry {
                object_id: 1,
                component_id: 20,
            })
        );
    }

    #[test]
    fn successful_electrical_stretch_bumps_topology_once_and_preserves_connections() {
        let mut state = selected_u_wire();
        state
            .design
            .document_mut_for_test()
            .connections
            .push(WireConnection::new(1, 0, 50, "IN"));
        let connections = state.design.document().connections.clone();
        let before = state.topology_version();
        state
            .stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(state.topology_version(), before.wrapping_add(1));
        assert_eq!(state.design.document().connections, connections);
        assert!(state.session.is_dirty);
    }
}
