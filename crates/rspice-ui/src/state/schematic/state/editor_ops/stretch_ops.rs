//! Stretching.
//!
//! Moving one edge or vertex of an object while the rest stays put — the
//! operation that resizes a shape or reroutes one leg of a wire without
//! detaching either end.

use super::super::super::{BusTargetKind, DocumentationShapeGeometry, WireSegment};
use super::super::*;

impl SchematicState {
    /// Whether the current selection contains at least one live stretch handle.
    pub fn has_live_stretch_selection(&self) -> bool {
        self.wires.iter().any(|wire| {
            self.selection.has_wire(wire.id) && wire.segment_count() != 0
                || self.selection.wire_segments.iter().any(|selected| {
                    selected.wire_id == wire.id && selected.segment_index < wire.segment_count()
                })
                || self.selection.wire_vertices.iter().any(|selected| {
                    selected.wire_id == wire.id
                        && selected.vertex_index < wire.vertex_count()
                        && wire.segment_count() != 0
                })
        }) || self
            .buses
            .iter()
            .any(|bus| self.selection.has_bus(bus.id) && bus.points.len() >= 2)
            || self.documentation_shapes.iter().any(|shape| {
                self.selection.has_documentation_shape(shape.id)
                    && documentation_shape_point_count(&shape.geometry) != 0
            })
    }

    /// Resolve an unambiguous default handle from the current frozen selection.
    /// Pointer-driven callers may instead construct an exact target and validate
    /// it with [`Self::is_stretch_target_eligible`].
    pub fn default_stretch_target(&self) -> Option<StretchTarget> {
        let mut targets = Vec::new();
        for selected in &self.selection.wire_segments {
            push_unique_target(
                &mut targets,
                StretchTarget::WireSegment {
                    wire_id: selected.wire_id,
                    segment_index: selected.segment_index,
                },
                self,
            );
        }
        for selected in &self.selection.wire_vertices {
            let Some(wire) = self.wires.iter().find(|wire| wire.id == selected.wire_id) else {
                continue;
            };
            let segment_index = if selected.vertex_index < wire.segment_count() {
                selected.vertex_index
            } else if selected.vertex_index != 0 && selected.vertex_index - 1 < wire.segment_count()
            {
                selected.vertex_index - 1
            } else {
                continue;
            };
            push_unique_target(
                &mut targets,
                StretchTarget::WireSegment {
                    wire_id: wire.id,
                    segment_index,
                },
                self,
            );
        }
        for &wire_id in &self.selection.wires {
            push_unique_target(
                &mut targets,
                StretchTarget::WireSegment {
                    wire_id,
                    segment_index: 0,
                },
                self,
            );
        }
        for &bus_id in &self.selection.buses {
            push_unique_target(
                &mut targets,
                StretchTarget::BusSegment {
                    bus_id,
                    segment_index: 0,
                },
                self,
            );
        }
        for &shape_id in &self.selection.documentation_shapes {
            push_unique_target(
                &mut targets,
                StretchTarget::DocumentationShapePoint {
                    shape_id,
                    point_index: 0,
                },
                self,
            );
        }
        (targets.len() == 1).then(|| targets[0])
    }

    /// Prove that an exact live handle belongs to the current frozen selection.
    pub fn is_stretch_target_eligible(&self, target: StretchTarget) -> bool {
        target_is_live(self, target) && selection_authorizes_target(self, target)
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
    ) -> Result<Option<SchematicState>, StretchSelectionError> {
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

    /// Stretch using caller-resolved component terminal geometry.
    ///
    /// The application supplies authored library-symbol pin positions through
    /// this boundary. Core callers retain the durable generated/primitive
    /// terminal geometry through [`Self::stretch_target`].
    pub fn stretch_target_resolved(
        &mut self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<bool, StretchSelectionError> {
        let Some(candidate) = self.preview_stretch_target_resolved(
            delta,
            target,
            policy,
            terminal_points_for,
            component_bounds_for,
        )?
        else {
            return Ok(false);
        };
        self.commit_stretch_candidate(candidate, target)?;
        Ok(true)
    }

    /// Build and validate the exact candidate rendered by a stretch preview.
    ///
    /// This is the single candidate construction/validation authority used by
    /// commit. It performs at most one full schematic clone; callers must not
    /// pre-clone the state before invoking it.
    pub fn preview_stretch_target_resolved(
        &self,
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        mut component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<Option<SchematicState>, StretchSelectionError> {
        if self.read_only || delta == Point::origin() {
            return Ok(None);
        }
        if !self.selection.probes.is_empty() {
            return Err(StretchSelectionError::ProbeSelectionUnsupported);
        }
        if !target_is_live(self, target) {
            return Err(StretchSelectionError::StaleTarget);
        }
        if !selection_authorizes_target(self, target) {
            return Ok(None);
        }
        let terminal_points_by_component = self
            .components
            .iter()
            .map(|component| (component.id, terminal_points_for(component)))
            .collect::<std::collections::HashMap<_, _>>();
        let component_bounds_by_component = self
            .components
            .iter()
            .map(|component| (component.id, component_bounds_for(component)))
            .collect::<std::collections::HashMap<_, _>>();

        let candidate = match target {
            StretchTarget::DocumentationShapePoint {
                shape_id,
                point_index,
            } => self.documentation_shape_stretch_candidate(shape_id, point_index, delta)?,
            StretchTarget::WireSegment {
                wire_id,
                segment_index,
            } => self.conductor_stretch_candidate(
                delta,
                ConductorTarget::Wire(wire_id),
                segment_index,
                policy,
                &terminal_points_by_component,
                &component_bounds_by_component,
            )?,
            StretchTarget::BusSegment {
                bus_id,
                segment_index,
            } => self.conductor_stretch_candidate(
                delta,
                ConductorTarget::Bus(bus_id),
                segment_index,
                policy,
                &terminal_points_by_component,
                &component_bounds_by_component,
            )?,
        };
        Ok(Some(candidate))
    }

    fn documentation_shape_stretch_candidate(
        &self,
        shape_id: u64,
        point_index: usize,
        delta: Point,
    ) -> Result<SchematicState, StretchSelectionError> {
        let mut candidate = self.clone();
        let shape = candidate
            .documentation_shapes
            .iter_mut()
            .find(|shape| shape.id == shape_id)
            .ok_or(StretchSelectionError::StaleTarget)?;
        let point = documentation_shape_point_mut(&mut shape.geometry, point_index)
            .ok_or(StretchSelectionError::StaleTarget)?;
        *point = checked_offset(*point, delta)?;
        if shape.validate().is_err() {
            return Err(StretchSelectionError::InvalidDocumentationGeometry { shape_id });
        }
        Ok(candidate)
    }

    fn conductor_stretch_candidate(
        &self,
        delta: Point,
        target: ConductorTarget,
        segment_index: usize,
        policy: StretchOrthogonalPolicy,
        terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
        component_bounds_by_component: &std::collections::HashMap<u64, (i32, i32, i32, i32)>,
    ) -> Result<SchematicState, StretchSelectionError> {
        let source_points = conductor_points(self, target)
            .ok_or(StretchSelectionError::StaleTarget)?
            .to_vec();
        let source_segment = source_points
            .get(segment_index..=segment_index + 1)
            .filter(|points| points.len() == 2)
            .map(|points| WireSegment::new(points[0], points[1]))
            .ok_or(StretchSelectionError::StaleTarget)?;
        let object_id = target.object_id();
        if source_segment.is_zero_length() {
            return Err(StretchSelectionError::DegenerateGeometry { object_id });
        }

        let affected_indices = affected_segment_indices(source_points.len(), segment_index);
        let source_affected = indexed_segments(&source_points, &affected_indices);
        validate_orthogonal_policy(object_id, source_segment, &source_affected, delta, policy)?;
        reject_source_anchors(
            self,
            target,
            segment_index,
            source_segment,
            terminal_points_by_component,
        )?;

        let mut candidate = self.clone();
        {
            let points = conductor_points_mut(&mut candidate, target)
                .ok_or(StretchSelectionError::StaleTarget)?;
            points[segment_index] = checked_offset(points[segment_index], delta)?;
            points[segment_index + 1] = checked_offset(points[segment_index + 1], delta)?;
        }
        translate_attached_taps(self, &mut candidate, target, source_segment, delta)?;

        let candidate_points =
            conductor_points(&candidate, target).ok_or(StretchSelectionError::StaleTarget)?;
        let candidate_affected = indexed_segments(candidate_points, &affected_indices);
        if candidate_affected
            .iter()
            .any(|(_, segment)| segment.is_zero_length())
        {
            return Err(StretchSelectionError::DegenerateGeometry { object_id });
        }
        if policy == StretchOrthogonalPolicy::PreserveOrthogonal
            && candidate_affected
                .iter()
                .any(|(_, segment)| !segment.is_orthogonal())
        {
            return Err(StretchSelectionError::NonOrthogonalSource { object_id });
        }
        match target {
            ConductorTarget::Wire(_) => {}
            ConductorTarget::Bus(_) => {
                let bus = candidate
                    .buses
                    .iter()
                    .find(|bus| bus.id == object_id)
                    .ok_or(StretchSelectionError::StaleTarget)?;
                if bus.validate().is_err() {
                    return Err(StretchSelectionError::DegenerateGeometry { object_id });
                }
            }
        }
        validate_all_tap_attachments(&candidate)?;
        validate_new_conductor_conflicts(
            self,
            &candidate,
            target,
            segment_index,
            &affected_indices,
        )?;
        validate_new_terminal_and_body_contacts(
            &candidate,
            target,
            &source_affected,
            &candidate_affected,
            terminal_points_by_component,
            component_bounds_by_component,
        )?;

        Ok(candidate)
    }

    fn commit_stretch_candidate(
        &mut self,
        candidate: SchematicState,
        target: StretchTarget,
    ) -> Result<(), StretchSelectionError> {
        match target {
            StretchTarget::WireSegment { wire_id, .. } => {
                let points = candidate
                    .wires
                    .iter()
                    .find(|wire| wire.id == wire_id)
                    .map(|wire| wire.points.clone())
                    .ok_or(StretchSelectionError::StaleTarget)?;
                self.wires
                    .iter_mut()
                    .find(|wire| wire.id == wire_id)
                    .ok_or(StretchSelectionError::StaleTarget)?
                    .points = points;
                self.bus_taps = candidate.bus_taps;
                self.bump_topology_version();
            }
            StretchTarget::BusSegment { bus_id, .. } => {
                let points = candidate
                    .buses
                    .iter()
                    .find(|bus| bus.id == bus_id)
                    .map(|bus| bus.points.clone())
                    .ok_or(StretchSelectionError::StaleTarget)?;
                self.buses
                    .iter_mut()
                    .find(|bus| bus.id == bus_id)
                    .ok_or(StretchSelectionError::StaleTarget)?
                    .points = points;
                self.bus_taps = candidate.bus_taps;
                self.bump_topology_version();
            }
            StretchTarget::DocumentationShapePoint { shape_id, .. } => {
                let geometry = candidate
                    .documentation_shapes
                    .iter()
                    .find(|shape| shape.id == shape_id)
                    .map(|shape| shape.geometry.clone())
                    .ok_or(StretchSelectionError::StaleTarget)?;
                self.documentation_shapes
                    .iter_mut()
                    .find(|shape| shape.id == shape_id)
                    .ok_or(StretchSelectionError::StaleTarget)?
                    .geometry = geometry;
            }
        }
        self.is_dirty = true;
        Ok(())
    }
}

fn push_unique_target(
    targets: &mut Vec<StretchTarget>,
    target: StretchTarget,
    state: &SchematicState,
) {
    if state.is_stretch_target_eligible(target) && !targets.contains(&target) {
        targets.push(target);
    }
}

fn target_is_live(state: &SchematicState, target: StretchTarget) -> bool {
    match target {
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        } => state
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .is_some_and(|wire| segment_index < wire.segment_count()),
        StretchTarget::BusSegment {
            bus_id,
            segment_index,
        } => state
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .is_some_and(|bus| segment_index < bus.points.len().saturating_sub(1)),
        StretchTarget::DocumentationShapePoint {
            shape_id,
            point_index,
        } => state
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == shape_id)
            .is_some_and(|shape| point_index < documentation_shape_point_count(&shape.geometry)),
    }
}

fn selection_authorizes_target(state: &SchematicState, target: StretchTarget) -> bool {
    match target {
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        } => {
            state.selection.has_wire(wire_id)
                || state.selection.has_wire_segment(wire_id, segment_index)
                || state.selection.has_wire_vertex(wire_id, segment_index)
                || state.selection.has_wire_vertex(wire_id, segment_index + 1)
        }
        StretchTarget::BusSegment { bus_id, .. } => state.selection.has_bus(bus_id),
        StretchTarget::DocumentationShapePoint { shape_id, .. } => {
            state.selection.has_documentation_shape(shape_id)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConductorTarget {
    Wire(u64),
    Bus(u64),
}

impl ConductorTarget {
    const fn object_id(self) -> u64 {
        match self {
            Self::Wire(id) | Self::Bus(id) => id,
        }
    }
}

fn conductor_points(state: &SchematicState, target: ConductorTarget) -> Option<&[Point]> {
    match target {
        ConductorTarget::Wire(id) => state
            .wires
            .iter()
            .find(|wire| wire.id == id)
            .map(|wire| wire.points.as_slice()),
        ConductorTarget::Bus(id) => state
            .buses
            .iter()
            .find(|bus| bus.id == id)
            .map(|bus| bus.points.as_slice()),
    }
}

fn conductor_points_mut(
    state: &mut SchematicState,
    target: ConductorTarget,
) -> Option<&mut Vec<Point>> {
    match target {
        ConductorTarget::Wire(id) => state
            .wires
            .iter_mut()
            .find(|wire| wire.id == id)
            .map(|wire| &mut wire.points),
        ConductorTarget::Bus(id) => state
            .buses
            .iter_mut()
            .find(|bus| bus.id == id)
            .map(|bus| &mut bus.points),
    }
}

fn affected_segment_indices(point_count: usize, selected: usize) -> Vec<usize> {
    let mut indices = Vec::with_capacity(3);
    if selected != 0 {
        indices.push(selected - 1);
    }
    indices.push(selected);
    if selected + 1 < point_count.saturating_sub(1) {
        indices.push(selected + 1);
    }
    indices
}

fn indexed_segments(points: &[Point], indices: &[usize]) -> Vec<(usize, WireSegment)> {
    indices
        .iter()
        .filter_map(|&index| {
            points
                .get(index..=index + 1)
                .filter(|points| points.len() == 2)
                .map(|points| (index, WireSegment::new(points[0], points[1])))
        })
        .collect()
}

fn checked_offset(point: Point, delta: Point) -> Result<Point, StretchSelectionError> {
    Ok(Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(StretchSelectionError::CoordinateOverflow)?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(StretchSelectionError::CoordinateOverflow)?,
    ))
}

fn validate_orthogonal_policy(
    object_id: u64,
    selected: WireSegment,
    affected: &[(usize, WireSegment)],
    delta: Point,
    policy: StretchOrthogonalPolicy,
) -> Result<(), StretchSelectionError> {
    if policy == StretchOrthogonalPolicy::AllowDiagonal {
        return Ok(());
    }
    if affected.iter().any(|(_, segment)| !segment.is_orthogonal()) {
        return Err(StretchSelectionError::NonOrthogonalSource { object_id });
    }
    let perpendicular = if selected.is_horizontal() {
        delta.x == 0 && delta.y != 0
    } else if selected.is_vertical() {
        delta.y == 0 && delta.x != 0
    } else {
        false
    };
    if !perpendicular {
        return Err(StretchSelectionError::PerpendicularDeltaRequired);
    }
    Ok(())
}

fn reject_source_anchors(
    state: &SchematicState,
    target: ConductorTarget,
    segment_index: usize,
    selected: WireSegment,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
) -> Result<(), StretchSelectionError> {
    if let ConductorTarget::Wire(wire_id) = target
        && state.connections.iter().any(|connection| {
            connection.wire_id == wire_id
                && (connection.point_index == segment_index
                    || connection.point_index == segment_index + 1)
        })
    {
        let connection = state
            .connections
            .iter()
            .find(|connection| {
                connection.wire_id == wire_id
                    && (connection.point_index == segment_index
                        || connection.point_index == segment_index + 1)
            })
            .expect("matching connection was just proven");
        let point = conductor_points(state, target)
            .and_then(|points| points.get(connection.point_index))
            .copied()
            .unwrap_or(selected.start);
        return Err(StretchSelectionError::ConnectedTerminal {
            component_id: connection.component_id,
            point,
        });
    }

    for component in &state.components {
        if let Some(point) = terminal_points_by_component
            .get(&component.id)
            .into_iter()
            .flatten()
            .copied()
            .find(|point| selected.contains_point(*point))
        {
            return Err(StretchSelectionError::ConnectedTerminal {
                component_id: component.id,
                point,
            });
        }
    }
    if let Some(junction) = state
        .junctions
        .iter()
        .find(|junction| selected.contains_point(junction.pos))
    {
        return Err(StretchSelectionError::FixedAnchor {
            point: junction.pos,
        });
    }
    if let Some(label) = state
        .net_labels
        .iter()
        .find(|label| selected.contains_point(label.pos))
    {
        return Err(StretchSelectionError::NetLabelAnchor {
            label_id: label.id,
            point: label.pos,
        });
    }

    for wire in &state.wires {
        if target == ConductorTarget::Wire(wire.id) {
            continue;
        }
        if let Some(point) = unrelated_anchor_point(&wire.points, selected) {
            return Err(StretchSelectionError::FixedAnchor { point });
        }
    }
    for bus in &state.buses {
        if target == ConductorTarget::Bus(bus.id) {
            continue;
        }
        if let Some(point) = unrelated_anchor_point(&bus.points, selected) {
            return Err(StretchSelectionError::FixedAnchor { point });
        }
    }
    Ok(())
}

fn unrelated_anchor_point(points: &[Point], selected: WireSegment) -> Option<Point> {
    points
        .iter()
        .copied()
        .find(|point| selected.contains_point(*point))
        .or_else(|| {
            [selected.start, selected.end]
                .into_iter()
                .find(|point| polyline_contains_point(points, *point))
        })
}

fn translate_attached_taps(
    original: &SchematicState,
    candidate: &mut SchematicState,
    target: ConductorTarget,
    selected: WireSegment,
    delta: Point,
) -> Result<(), StretchSelectionError> {
    for (index, old_tap) in original.bus_taps.iter().enumerate() {
        let source_moves = matches!(target, ConductorTarget::Bus(bus_id) if old_tap.bus_id == bus_id)
            && selected.contains_point(old_tap.bus_point);
        let target_moves = match target {
            ConductorTarget::Wire(_) => {
                old_tap.target_kind() == BusTargetKind::Wire
                    && selected.contains_point(old_tap.connection_point)
            }
            ConductorTarget::Bus(bus_id) => {
                old_tap.bus_id != bus_id
                    && old_tap.target_kind() == BusTargetKind::Bus
                    && selected.contains_point(old_tap.connection_point)
            }
        };
        if source_moves {
            candidate.bus_taps[index].bus_point = checked_offset(old_tap.bus_point, delta)?;
        }
        if target_moves {
            candidate.bus_taps[index].connection_point =
                checked_offset(old_tap.connection_point, delta)?;
        }
    }
    Ok(())
}

fn validate_all_tap_attachments(state: &SchematicState) -> Result<(), StretchSelectionError> {
    for tap in &state.bus_taps {
        let source_valid = state
            .buses
            .iter()
            .find(|bus| bus.id == tap.bus_id)
            .is_some_and(|bus| tap.validate_against_bus(bus).is_ok());
        let target_valid = match tap.target_kind() {
            BusTargetKind::Wire => state
                .wires
                .iter()
                .any(|wire| wire.contains_point(tap.connection_point)),
            BusTargetKind::Bus => state
                .buses
                .iter()
                .filter(|bus| bus.id != tap.bus_id)
                .any(|bus| bus.contains_point(tap.connection_point)),
        };
        if !source_valid || !target_valid {
            return Err(StretchSelectionError::InvalidTapAttachment { tap_id: tap.id });
        }
    }
    Ok(())
}

fn validate_new_conductor_conflicts(
    original: &SchematicState,
    candidate: &SchematicState,
    target: ConductorTarget,
    selected_index: usize,
    affected_indices: &[usize],
) -> Result<(), StretchSelectionError> {
    let object_id = target.object_id();
    let original_points = conductor_points(original, target).expect("target was preflighted");
    let candidate_points = conductor_points(candidate, target).expect("candidate preserves target");
    let original_affected = indexed_segments(original_points, affected_indices);
    let candidate_affected = indexed_segments(candidate_points, affected_indices);
    let old_moved = [
        original_points[selected_index],
        original_points[selected_index + 1],
    ];
    let new_moved = [
        candidate_points[selected_index],
        candidate_points[selected_index + 1],
    ];

    for left in 0..candidate_affected.len() {
        for right in left + 1..candidate_affected.len() {
            if candidate_affected[left]
                .0
                .abs_diff(candidate_affected[right].0)
                <= 1
            {
                continue;
            }
            let overlaps =
                positive_length_overlap(candidate_affected[left].1, candidate_affected[right].1);
            let existed =
                positive_length_overlap(original_affected[left].1, original_affected[right].1);
            if overlaps && !existed {
                return Err(StretchSelectionError::ConductorOverlap {
                    object_id,
                    other_id: object_id,
                });
            }
        }
    }

    for wire in &candidate.wires {
        let same = target == ConductorTarget::Wire(wire.id);
        let obstacle_indices = obstacle_indices(wire.points.len(), same, affected_indices);
        if obstacle_indices.is_empty() {
            continue;
        }
        let old_wire = original
            .wires
            .iter()
            .find(|old| old.id == wire.id)
            .expect("candidate preserves wire identities");
        validate_against_obstacle(
            object_id,
            wire.id,
            &original_affected,
            &candidate_affected,
            old_moved,
            new_moved,
            &old_wire.points,
            &wire.points,
            &obstacle_indices,
        )?;
    }
    for bus in &candidate.buses {
        let same = target == ConductorTarget::Bus(bus.id);
        let obstacle_indices = obstacle_indices(bus.points.len(), same, affected_indices);
        if obstacle_indices.is_empty() {
            continue;
        }
        let old_bus = original
            .buses
            .iter()
            .find(|old| old.id == bus.id)
            .expect("candidate preserves bus identities");
        validate_against_obstacle(
            object_id,
            bus.id,
            &original_affected,
            &candidate_affected,
            old_moved,
            new_moved,
            &old_bus.points,
            &bus.points,
            &obstacle_indices,
        )?;
    }

    for junction in &candidate.junctions {
        let new_contact = candidate_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(junction.pos));
        let old_contact = original_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(junction.pos));
        if new_contact != old_contact {
            return Err(StretchSelectionError::FixedAnchor {
                point: junction.pos,
            });
        }
    }
    for label in &candidate.net_labels {
        let new_contact = candidate_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(label.pos));
        let old_contact = original_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(label.pos));
        if new_contact != old_contact {
            return Err(StretchSelectionError::NetLabelAnchor {
                label_id: label.id,
                point: label.pos,
            });
        }
    }
    Ok(())
}

fn validate_against_obstacle(
    object_id: u64,
    other_id: u64,
    original_affected: &[(usize, WireSegment)],
    candidate_affected: &[(usize, WireSegment)],
    old_moved: [Point; 2],
    new_moved: [Point; 2],
    original_obstacle_points: &[Point],
    candidate_obstacle_points: &[Point],
    obstacle_indices: &[usize],
) -> Result<(), StretchSelectionError> {
    let original_obstacles = indexed_segments(original_obstacle_points, obstacle_indices);
    let candidate_obstacles = indexed_segments(candidate_obstacle_points, obstacle_indices);
    for (affected_index, (_, candidate_segment)) in candidate_affected.iter().enumerate() {
        for (obstacle_index, (_, candidate_obstacle)) in candidate_obstacles.iter().enumerate() {
            if positive_length_overlap(*candidate_segment, *candidate_obstacle)
                && !positive_length_overlap(
                    original_affected[affected_index].1,
                    original_obstacles[obstacle_index].1,
                )
            {
                return Err(StretchSelectionError::ConductorOverlap {
                    object_id,
                    other_id,
                });
            }
        }
    }

    let obstacle_point_indices = obstacle_indices
        .iter()
        .flat_map(|index| [*index, *index + 1])
        .collect::<std::collections::HashSet<_>>();
    for point_index in obstacle_point_indices {
        let Some(&point) = candidate_obstacle_points.get(point_index) else {
            continue;
        };
        let new_contact = candidate_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(point));
        let old_contact = original_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(point));
        if new_contact && !old_contact {
            return Err(StretchSelectionError::UnintendedConductorContact {
                object_id,
                other_id,
            });
        }
    }
    for index in 0..2 {
        let new_contact = candidate_obstacles
            .iter()
            .any(|(_, segment)| segment.contains_point(new_moved[index]));
        let old_contact = original_obstacles
            .iter()
            .any(|(_, segment)| segment.contains_point(old_moved[index]));
        if new_contact && !old_contact {
            return Err(StretchSelectionError::UnintendedConductorContact {
                object_id,
                other_id,
            });
        }
    }
    Ok(())
}

fn obstacle_indices(
    point_count: usize,
    same_target: bool,
    affected_indices: &[usize],
) -> Vec<usize> {
    (0..point_count.saturating_sub(1))
        .filter(|index| !same_target || !affected_indices.contains(index))
        .collect()
}

fn validate_new_terminal_and_body_contacts(
    candidate: &SchematicState,
    target: ConductorTarget,
    original_affected: &[(usize, WireSegment)],
    candidate_affected: &[(usize, WireSegment)],
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
    component_bounds_by_component: &std::collections::HashMap<u64, (i32, i32, i32, i32)>,
) -> Result<(), StretchSelectionError> {
    let object_id = target.object_id();
    for component in &candidate.components {
        let terminals = terminal_points_by_component
            .get(&component.id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        for &terminal in terminals {
            let new_contact = candidate_affected
                .iter()
                .any(|(_, segment)| segment.contains_point(terminal));
            let old_contact = original_affected
                .iter()
                .any(|(_, segment)| segment.contains_point(terminal));
            if new_contact && !old_contact {
                return Err(StretchSelectionError::UnintendedTerminalContact {
                    object_id,
                    component_id: component.id,
                });
            }
        }
        let bounds = extended_component_bounds(
            component_bounds_by_component
                .get(&component.id)
                .copied()
                .unwrap_or_else(|| component.bounding_box()),
            terminals,
        );
        let enters_now = candidate_affected
            .iter()
            .any(|(_, segment)| segment_enters_open_rect(*segment, bounds));
        let entered_before = original_affected
            .iter()
            .any(|(_, segment)| segment_enters_open_rect(*segment, bounds));
        if enters_now && !entered_before {
            return Err(StretchSelectionError::ComponentBodyEntry {
                object_id,
                component_id: component.id,
            });
        }
    }
    Ok(())
}

fn extended_component_bounds(
    (mut min_x, mut min_y, mut max_x, mut max_y): (i32, i32, i32, i32),
    terminals: &[Point],
) -> (i32, i32, i32, i32) {
    for terminal in terminals {
        min_x = min_x.min(terminal.x);
        min_y = min_y.min(terminal.y);
        max_x = max_x.max(terminal.x);
        max_y = max_y.max(terminal.y);
    }
    (min_x, min_y, max_x, max_y)
}

fn positive_length_overlap(left: WireSegment, right: WireSegment) -> bool {
    if left.is_zero_length() || right.is_zero_length() {
        return false;
    }
    let left_dx = i128::from(left.end.x) - i128::from(left.start.x);
    let left_dy = i128::from(left.end.y) - i128::from(left.start.y);
    let right_dx = i128::from(right.end.x) - i128::from(right.start.x);
    let right_dy = i128::from(right.end.y) - i128::from(right.start.y);
    if left_dx * right_dy != left_dy * right_dx {
        return false;
    }
    let offset_x = i128::from(right.start.x) - i128::from(left.start.x);
    let offset_y = i128::from(right.start.y) - i128::from(left.start.y);
    if left_dx * offset_y != left_dy * offset_x {
        return false;
    }
    if left_dx != 0 {
        i128::from(left.start.x.min(left.end.x)).max(i128::from(right.start.x.min(right.end.x)))
            < i128::from(left.start.x.max(left.end.x))
                .min(i128::from(right.start.x.max(right.end.x)))
    } else {
        i128::from(left.start.y.min(left.end.y)).max(i128::from(right.start.y.min(right.end.y)))
            < i128::from(left.start.y.max(left.end.y))
                .min(i128::from(right.start.y.max(right.end.y)))
    }
}

fn polyline_contains_point(points: &[Point], point: Point) -> bool {
    points
        .windows(2)
        .any(|pair| WireSegment::new(pair[0], pair[1]).contains_point(point))
}

fn segment_enters_open_rect(segment: WireSegment, bounds: (i32, i32, i32, i32)) -> bool {
    let (min_x, min_y, max_x, max_y) = bounds;
    if min_x >= max_x || min_y >= max_y {
        return false;
    }
    let start_x = f64::from(segment.start.x);
    let start_y = f64::from(segment.start.y);
    let dx = f64::from(segment.end.x) - start_x;
    let dy = f64::from(segment.end.y) - start_y;
    let mut enter: f64 = 0.0;
    let mut exit: f64 = 1.0;
    for (origin, direction, lower, upper) in [
        (start_x, dx, f64::from(min_x), f64::from(max_x)),
        (start_y, dy, f64::from(min_y), f64::from(max_y)),
    ] {
        if direction == 0.0 {
            if origin <= lower || origin >= upper {
                return false;
            }
            continue;
        }
        let first = (lower - origin) / direction;
        let second = (upper - origin) / direction;
        enter = enter.max(first.min(second));
        exit = exit.min(first.max(second));
        if enter > exit {
            return false;
        }
    }
    let sample = ((enter.max(0.0) + exit.min(1.0)) / 2.0).clamp(0.0, 1.0);
    let x = start_x + sample * dx;
    let y = start_y + sample * dy;
    x > f64::from(min_x) && x < f64::from(max_x) && y > f64::from(min_y) && y < f64::from(max_y)
}

fn documentation_shape_point_count(geometry: &DocumentationShapeGeometry) -> usize {
    match geometry {
        DocumentationShapeGeometry::Rectangle { .. } | DocumentationShapeGeometry::Line { .. } => 2,
        DocumentationShapeGeometry::Polygon { points } => points.len(),
        DocumentationShapeGeometry::Arc { .. } | DocumentationShapeGeometry::Callout { .. } => 3,
    }
}

fn documentation_shape_point_mut(
    geometry: &mut DocumentationShapeGeometry,
    point_index: usize,
) -> Option<&mut Point> {
    match geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => match point_index {
            0 => Some(first),
            1 => Some(opposite),
            _ => None,
        },
        DocumentationShapeGeometry::Line { start, end } => match point_index {
            0 => Some(start),
            1 => Some(end),
            _ => None,
        },
        DocumentationShapeGeometry::Polygon { points } => points.get_mut(point_index),
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => match point_index {
            0 => Some(start),
            1 => Some(through),
            2 => Some(end),
            _ => None,
        },
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => match point_index {
            0 => Some(tip),
            1 => Some(elbow),
            2 => Some(box_corner),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
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
        state.wires.push(u_wire(1));
        state.selection.select_only_wire_segment(1, 1);
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
        state.wires.push(u_wire(1));
        state.selection.select_only_wire_vertex(1, 2);
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
            state.wires[0].points,
            [
                Point::new(0, 0),
                Point::new(0, 15),
                Point::new(20, 15),
                Point::new(20, 0),
            ]
        );
        assert!(state.wires[0].is_orthogonal());
    }

    #[test]
    fn stretches_vertical_interior_segment_orthogonally() {
        let mut state = SchematicState::default();
        state.wires.push(Wire::new(
            1,
            vec![
                Point::new(0, 0),
                Point::new(10, 0),
                Point::new(10, 20),
                Point::new(0, 20),
            ],
        ));
        state.selection.select_only_wire_segment(1, 1);
        state
            .stretch_target(
                Point::new(5, 0),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(state.wires[0].points[1], Point::new(15, 0));
        assert_eq!(state.wires[0].points[2], Point::new(15, 20));
        assert_eq!(state.wires[0].points[0], Point::new(0, 0));
        assert_eq!(state.wires[0].points[3], Point::new(0, 20));
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
            state.wires.push(Wire::new(1, points));
            state.selection.select_only_wire_segment(1, segment_index);
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
            .wires
            .push(Wire::new(2, vec![Point::new(0, 0), Point::new(10, 10)]));
        diagonal.selection.select_only_wire(2);
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
        assert_eq!(state.wires[0].points[1], Point::new(5, 15));
        assert_eq!(state.wires[0].points[2], Point::new(25, 15));
        assert!(!state.wires[0].is_orthogonal());
        assert!(state.wires[0].contains_point(Point::new(15, 15)));
    }

    #[test]
    fn preview_is_nonmutating_and_is_the_exact_commit_candidate() {
        let mut state = selected_u_wire();
        let before = SchematicSnapshot::capture(&state);
        let preview = state
            .preview_stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap()
            .unwrap();
        assert!(before.is_equal(&SchematicSnapshot::capture(&state)));
        state
            .stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(state.wires[0].points, preview.wires[0].points);
        assert_eq!(state.bus_taps, preview.bus_taps);
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
        state.connections.push(WireConnection::new(1, 1, 44, "OUT"));
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
        state.components.push(component);
        state.wires.push(Wire::segment(
            1,
            terminal,
            Point::new(terminal.x, terminal.y + 20),
        ));
        state.selection.select_only_wire(1);
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
                    .wires
                    .push(Wire::segment(2, Point::new(10, 10), Point::new(10, 30))),
                1 => state
                    .buses
                    .push(Bus::segment(2, Point::new(10, 10), Point::new(10, 30), None).unwrap()),
                _ => state.junctions.push(Junction::new(2, Point::new(10, 10))),
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
            .net_labels
            .push(NetLabel::new(70, Point::new(10, 10), "SENSE"));
        let before = SchematicSnapshot::capture(&existing);
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
        assert!(before.is_equal(&SchematicSnapshot::capture(&existing)));

        let mut new_contact = selected_u_wire();
        new_contact
            .net_labels
            .push(NetLabel::new(71, Point::new(10, 15), "OTHER"));
        let before = SchematicSnapshot::capture(&new_contact);
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
        assert!(before.is_equal(&SchematicSnapshot::capture(&new_contact)));
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
        state.buses.push(bus);
        state.bus_taps.push(tap);
        state
            .stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(state.bus_taps[0].bus_point, Point::new(10, -10));
        assert_eq!(state.bus_taps[0].connection_point, Point::new(10, 15));
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
        state.buses.push(bus);
        state.bus_taps.push(tap);
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
            .wires
            .push(Wire::segment(9, Point::new(0, 30), Point::new(20, 30)));
        state.buses.push(bus);
        state.bus_taps.push(tap);
        state.selection.select_bus(5);
        state
            .stretch_target(
                Point::new(0, 5),
                bus_target(5, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(state.buses[0].points[1], Point::new(0, 15));
        assert_eq!(state.buses[0].points[2], Point::new(20, 15));
        assert_eq!(state.bus_taps[0].bus_point, Point::new(10, 15));
        assert_eq!(state.bus_taps[0].connection_point, Point::new(10, 30));
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
                .documentation_shapes
                .push(DocumentationShape::new(7, geometry).unwrap());
            state.selection.select_documentation_shape(7);
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
                state.documentation_shapes[0].geometry.points()[point_index],
                expected
            );
            assert_eq!(state.topology_version(), before_topology);
            assert!(state.is_dirty);
        }
    }

    #[test]
    fn invalid_shape_control_point_is_atomic() {
        let mut state = SchematicState::default();
        state.documentation_shapes.push(
            DocumentationShape::new(
                7,
                DocumentationShapeGeometry::Line {
                    start: Point::new(0, 0),
                    end: Point::new(10, 0),
                },
            )
            .unwrap(),
        );
        state.selection.select_documentation_shape(7);
        let before = SchematicSnapshot::capture(&state);
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
        assert!(before.is_equal(&SchematicSnapshot::capture(&state)));
    }

    #[test]
    fn stale_target_and_coordinate_overflow_are_rejected_atomically() {
        let mut state = selected_u_wire();
        let before = SchematicSnapshot::capture(&state);
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 99),
                StretchOrthogonalPolicy::AllowDiagonal,
            ),
            Err(StretchSelectionError::StaleTarget)
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(&state)));

        state.wires[0].points[1].y = i32::MAX;
        state.wires[0].points[2].y = i32::MAX;
        let before = SchematicSnapshot::capture(&state);
        assert_eq!(
            state.stretch_target(
                Point::new(0, 1),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Err(StretchSelectionError::CoordinateOverflow)
        );
        assert!(before.is_equal(&SchematicSnapshot::capture(&state)));
    }

    #[test]
    fn read_only_zero_delta_and_unselected_target_are_clean_noops() {
        let mut state = selected_u_wire();
        let baseline = SchematicSnapshot::capture(&state);
        state.read_only = true;
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(false)
        );
        state.read_only = false;
        assert_eq!(
            state.stretch_target(
                Point::origin(),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(false)
        );
        state.selection.clear();
        assert_eq!(
            state.stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            ),
            Ok(false)
        );
        assert!(baseline.is_equal(&SchematicSnapshot::capture(&state)));
    }

    #[test]
    fn new_overlap_and_endpoint_contact_are_rejected() {
        let mut overlap = selected_u_wire();
        overlap
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
        state.components.push(component);
        state.wires.push(Wire::new(
            1,
            vec![
                Point::new(terminal.x - 5, terminal.y - 10),
                Point::new(terminal.x - 5, terminal.y - 5),
                Point::new(terminal.x + 5, terminal.y - 5),
                Point::new(terminal.x + 5, terminal.y - 10),
            ],
        ));
        state.selection.select_only_wire_segment(1, 1);
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
        source_contact.components.push(Component::new(
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
        new_contact.components.push(Component::new(
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
        state.components.push(Component::new(
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
        state.connections.push(WireConnection::new(1, 0, 50, "IN"));
        let connections = state.connections.clone();
        let before = state.topology_version();
        state
            .stretch_target(
                Point::new(0, 5),
                wire_target(1, 1),
                StretchOrthogonalPolicy::PreserveOrthogonal,
            )
            .unwrap();
        assert_eq!(state.topology_version(), before.wrapping_add(1));
        assert_eq!(state.connections, connections);
        assert!(state.is_dirty);
    }
}
