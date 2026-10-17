//! Validated stretch previews and document edits.

use super::{
    bus::BusTargetKind, component::Component, document::SchematicDocument,
    documentation_shape::DocumentationShapeGeometry, wire::WireSegment,
};
use rspice_design_model::Point;
use serde::{Deserialize, Serialize};

/// Geometry policy applied while stretching one selected schematic segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StretchOrthogonalPolicy {
    /// Retain orthogonal source and affected geometry. The requested motion
    /// must be perpendicular to the selected segment.
    #[default]
    PreserveOrthogonal,
    /// Permit diagonal adjacent segments while retaining every fixed anchor.
    AllowDiagonal,
}

impl StretchOrthogonalPolicy {
    pub const ALL: [Self; 2] = [Self::PreserveOrthogonal, Self::AllowDiagonal];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PreserveOrthogonal => "Preserve orthogonal",
            Self::AllowDiagonal => "Allow diagonal",
        }
    }
}

/// Stable identity of the exact geometry handle stretched by one command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StretchTarget {
    WireSegment {
        wire_id: u64,
        segment_index: usize,
    },
    BusSegment {
        bus_id: u64,
        segment_index: usize,
    },
    /// One typed control point of a documentation/parameterized shape.
    DocumentationShapePoint {
        shape_id: u64,
        point_index: usize,
    },
}

/// A stretch was rejected before any document mutation occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StretchSelectionError {
    ProbeSelectionUnsupported,
    StaleTarget,
    CoordinateOverflow,
    DegenerateGeometry { object_id: u64 },
    NonOrthogonalSource { object_id: u64 },
    PerpendicularDeltaRequired,
    FixedAnchor { point: Point },
    NetLabelAnchor { label_id: u64, point: Point },
    ConnectedTerminal { component_id: u64, point: Point },
    InvalidTapAttachment { tap_id: u64 },
    ConductorOverlap { object_id: u64, other_id: u64 },
    UnintendedConductorContact { object_id: u64, other_id: u64 },
    UnintendedTerminalContact { object_id: u64, component_id: u64 },
    ComponentBodyEntry { object_id: u64, component_id: u64 },
    InvalidDocumentationGeometry { shape_id: u64 },
}

impl std::fmt::Display for StretchSelectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProbeSelectionUnsupported => formatter.write_str(
                "Probe markers cannot be stretched; move the retained probe marker instead.",
            ),
            Self::StaleTarget => formatter
                .write_str("The selected stretch handle no longer exists in the active schematic."),
            Self::CoordinateOverflow => {
                formatter.write_str("The requested stretch exceeds the schematic coordinate range.")
            }
            Self::DegenerateGeometry { object_id } => write!(
                formatter,
                "Stretching object {object_id} would create a zero-length or invalid segment."
            ),
            Self::NonOrthogonalSource { object_id } => write!(
                formatter,
                "Object {object_id} has non-orthogonal affected geometry; choose Allow diagonal to stretch it."
            ),
            Self::PerpendicularDeltaRequired => formatter.write_str(
                "Preserve orthogonal requires motion perpendicular to the selected segment.",
            ),
            Self::FixedAnchor { point } => write!(
                formatter,
                "The selected segment contains a fixed conductor or junction anchor at ({}, {}).",
                point.x, point.y
            ),
            Self::NetLabelAnchor { label_id, point } => write!(
                formatter,
                "Net label {label_id} is an electrical anchor at ({}, {}); stretch cannot move away from or under it.",
                point.x, point.y
            ),
            Self::ConnectedTerminal {
                component_id,
                point,
            } => write!(
                formatter,
                "Component {component_id} owns a connected terminal at ({}, {}); its anchor cannot be stretched.",
                point.x, point.y
            ),
            Self::InvalidTapAttachment { tap_id } => write!(
                formatter,
                "Bus tap {tap_id} cannot retain both its declared source and destination attachments."
            ),
            Self::ConductorOverlap {
                object_id,
                other_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would overlap conductor {other_id}."
            ),
            Self::UnintendedConductorContact {
                object_id,
                other_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would create an unintended contact with conductor {other_id}."
            ),
            Self::UnintendedTerminalContact {
                object_id,
                component_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would contact a terminal on component {component_id}."
            ),
            Self::ComponentBodyEntry {
                object_id,
                component_id,
            } => write!(
                formatter,
                "Stretching object {object_id} would route through component {component_id}."
            ),
            Self::InvalidDocumentationGeometry { shape_id } => write!(
                formatter,
                "Moving that control point would make documentation shape {shape_id} invalid."
            ),
        }
    }
}

impl std::error::Error for StretchSelectionError {}

/// Stretch using caller-resolved component terminal geometry.
///
/// The application supplies authored library-symbol pin positions through
/// this boundary. Core callers retain the durable generated/primitive
/// terminal geometry through [`preview_stretch_target_resolved`].
pub fn stretch_target_resolved(
    document: &mut SchematicDocument,
    delta: Point,
    target: StretchTarget,
    policy: StretchOrthogonalPolicy,
    terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
) -> Result<bool, StretchSelectionError> {
    let Some(candidate) = preview_stretch_target_resolved(
        document,
        delta,
        target,
        policy,
        terminal_points_for,
        component_bounds_for,
    )?
    else {
        return Ok(false);
    };
    commit_stretch_candidate(document, candidate, target)?;
    Ok(true)
}

/// Build and validate the exact candidate rendered by a stretch preview.
///
/// This is the single candidate construction/validation authority used by
/// commit. It performs at most one document clone; callers must not
/// pre-clone the document before invoking it.
pub fn preview_stretch_target_resolved(
    document: &SchematicDocument,
    delta: Point,
    target: StretchTarget,
    policy: StretchOrthogonalPolicy,
    mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    mut component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
) -> Result<Option<SchematicDocument>, StretchSelectionError> {
    if delta == Point::origin() {
        return Ok(None);
    }
    if !target_is_live(document, target) {
        return Err(StretchSelectionError::StaleTarget);
    }
    let terminal_points_by_component = document
        .components
        .iter()
        .map(|component| (component.id, terminal_points_for(component)))
        .collect::<std::collections::HashMap<_, _>>();
    let component_bounds_by_component = document
        .components
        .iter()
        .map(|component| (component.id, component_bounds_for(component)))
        .collect::<std::collections::HashMap<_, _>>();

    let candidate = match target {
        StretchTarget::DocumentationShapePoint {
            shape_id,
            point_index,
        } => documentation_shape_stretch_candidate(document, shape_id, point_index, delta)?,
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        } => conductor_stretch_candidate(
            document,
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
        } => conductor_stretch_candidate(
            document,
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
    document: &SchematicDocument,
    shape_id: u64,
    point_index: usize,
    delta: Point,
) -> Result<SchematicDocument, StretchSelectionError> {
    let mut candidate = document.clone();
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
    document: &SchematicDocument,
    delta: Point,
    target: ConductorTarget,
    segment_index: usize,
    policy: StretchOrthogonalPolicy,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
    component_bounds_by_component: &std::collections::HashMap<u64, (i32, i32, i32, i32)>,
) -> Result<SchematicDocument, StretchSelectionError> {
    let source_points = conductor_points(document, target)
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
    let carried = carried_ends(document, target, source_segment);
    reject_source_anchors(
        document,
        target,
        segment_index,
        source_segment,
        terminal_points_by_component,
        &carried,
    )?;

    let mut candidate = document.clone();
    {
        let points = conductor_points_mut(&mut candidate, target)
            .ok_or(StretchSelectionError::StaleTarget)?;
        points[segment_index] = checked_offset(points[segment_index], delta)?;
        points[segment_index + 1] = checked_offset(points[segment_index + 1], delta)?;
    }
    translate_attached_taps(document, &mut candidate, target, source_segment, delta)?;
    let carried_segments = carry_neighbour_ends(&mut candidate, &carried, delta, policy)?;

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
        document,
        &candidate,
        target,
        segment_index,
        &affected_indices,
        &carried,
    )?;
    validate_new_terminal_and_body_contacts(
        &candidate,
        target,
        &source_affected,
        &candidate_affected,
        terminal_points_by_component,
        component_bounds_by_component,
    )?;
    for segment in &carried_segments {
        validate_carried_segment(
            document,
            &candidate,
            *segment,
            terminal_points_by_component,
            component_bounds_by_component,
        )?;
    }

    Ok(candidate)
}

/// A wire that ends on a corner of the stretched segment and touches it
/// nowhere else. The wire tool stores each segment of a route as its own
/// wire, so the segments either side of the stretched one are other wires:
/// they follow the corner instead of pinning it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CarriedEnd {
    wire_index: usize,
    /// The corner the wire's first vertex sits on, if it does.
    first: Option<Point>,
    /// The corner the wire's last vertex sits on, if it does.
    last: Option<Point>,
}

fn carried_ends(
    state: &SchematicDocument,
    target: ConductorTarget,
    selected: WireSegment,
) -> Vec<CarriedEnd> {
    let ConductorTarget::Wire(target_id) = target else {
        return Vec::new();
    };
    let corners = [selected.start, selected.end];
    let mut carried = Vec::new();
    for (wire_index, wire) in state.wires.iter().enumerate() {
        let count = wire.points.len();
        if wire.id == target_id || count < 2 {
            continue;
        }
        let first = corners.contains(&wire.points[0]);
        let last = corners.contains(&wire.points[count - 1]);
        if !first && !last {
            continue;
        }
        let shared = |index: usize| (index == 0 && first) || (index == count - 1 && last);
        let touches_elsewhere = wire
            .points
            .iter()
            .enumerate()
            .any(|(index, point)| !shared(index) && selected.contains_point(*point))
            || corners.iter().any(|corner| {
                let at_shared_end = (first && wire.points[0] == *corner)
                    || (last && wire.points[count - 1] == *corner);
                !at_shared_end && polyline_contains_point(&wire.points, *corner)
            });
        if !touches_elsewhere {
            carried.push(CarriedEnd {
                wire_index,
                first: first.then_some(wire.points[0]),
                last: last.then_some(wire.points[count - 1]),
            });
        }
    }
    carried
}

fn carries_wire(carried: &[CarriedEnd], wire_index: usize) -> bool {
    carried.iter().any(|end| end.wire_index == wire_index)
}

fn carries_corner(carried: &[CarriedEnd], point: Point) -> bool {
    carried
        .iter()
        .any(|end| end.first == Some(point) || end.last == Some(point))
}

/// The part of a carried wire the stretch lays anew, against what it
/// replaces: the end segment itself, or, where the end turns at its old
/// corner, the new leg from that corner.
#[derive(Debug, Clone, Copy)]
struct CarriedSegment {
    wire_index: usize,
    before: WireSegment,
    after: WireSegment,
}

/// Move each carried end with its corner. Under Preserve orthogonal an end
/// whose segment would slant keeps its old corner as a vertex and turns
/// there to follow; the pin connections of a wire that gains a vertex at
/// its start move up by one. A dot on a carried corner moves with it.
fn carry_neighbour_ends(
    candidate: &mut SchematicDocument,
    carried: &[CarriedEnd],
    delta: Point,
    policy: StretchOrthogonalPolicy,
) -> Result<Vec<CarriedSegment>, StretchSelectionError> {
    let mut laid = Vec::with_capacity(carried.len());
    for end in carried {
        let wire = &mut candidate.wires[end.wire_index];
        let object_id = wire.id;
        if let ([start, finish], Some(_), Some(_)) = (wire.points.as_slice(), end.first, end.last) {
            // A single segment between the two corners travels with them.
            let before = WireSegment::new(*start, *finish);
            let after = WireSegment::new(
                checked_offset(*start, delta)?,
                checked_offset(*finish, delta)?,
            );
            wire.points = vec![after.start, after.end];
            laid.push(CarriedSegment {
                wire_index: end.wire_index,
                before,
                after,
            });
            continue;
        }
        for (first, corner) in [(true, end.first), (false, end.last)] {
            let Some(corner) = corner else {
                continue;
            };
            let moved = checked_offset(corner, delta)?;
            let last = wire.points.len() - 1;
            let (end_index, neighbour_index) = if first { (0, 1) } else { (last, last - 1) };
            let neighbour = wire.points[neighbour_index];
            let before = WireSegment::new(neighbour, corner);
            let after = WireSegment::new(neighbour, moved);
            let squares = policy == StretchOrthogonalPolicy::PreserveOrthogonal;
            if squares && !before.is_orthogonal() {
                return Err(StretchSelectionError::NonOrthogonalSource { object_id });
            }
            if squares && !after.is_orthogonal() {
                if first {
                    wire.points.insert(0, moved);
                    for connection in candidate
                        .connections
                        .iter_mut()
                        .filter(|connection| connection.wire_id == object_id)
                    {
                        connection.point_index += 1;
                    }
                } else {
                    wire.points.push(moved);
                }
                laid.push(CarriedSegment {
                    wire_index: end.wire_index,
                    before: WireSegment::new(corner, corner),
                    after: WireSegment::new(corner, moved),
                });
            } else {
                if after.is_zero_length() {
                    return Err(StretchSelectionError::DegenerateGeometry { object_id });
                }
                wire.points[end_index] = moved;
                laid.push(CarriedSegment {
                    wire_index: end.wire_index,
                    before,
                    after,
                });
            }
        }
    }
    for junction in &mut candidate.junctions {
        if carries_corner(carried, junction.pos) {
            junction.pos = checked_offset(junction.pos, delta)?;
        }
    }
    Ok(laid)
}

/// The newly laid part of a carried wire may touch only what its old part
/// touched: no new pin, body, dot, label or conductor contact, and no
/// conductor doubled along it.
fn validate_carried_segment(
    original: &SchematicDocument,
    candidate: &SchematicDocument,
    segment: CarriedSegment,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
    component_bounds_by_component: &std::collections::HashMap<u64, (i32, i32, i32, i32)>,
) -> Result<(), StretchSelectionError> {
    let CarriedSegment {
        wire_index,
        before,
        after,
    } = segment;
    let object_id = candidate.wires[wire_index].id;
    for component in &candidate.components {
        let terminals = terminal_points_by_component
            .get(&component.id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        if terminals
            .iter()
            .any(|terminal| after.contains_point(*terminal) && !before.contains_point(*terminal))
        {
            return Err(StretchSelectionError::UnintendedTerminalContact {
                object_id,
                component_id: component.id,
            });
        }
        let bounds = extended_component_bounds(
            component_bounds_by_component
                .get(&component.id)
                .copied()
                .unwrap_or_else(|| component.bounding_box()),
            terminals,
        );
        if segment_enters_open_rect(after, bounds) && !segment_enters_open_rect(before, bounds) {
            return Err(StretchSelectionError::ComponentBodyEntry {
                object_id,
                component_id: component.id,
            });
        }
    }
    for (old, new) in original.junctions.iter().zip(&candidate.junctions) {
        if after.contains_point(new.pos) != before.contains_point(old.pos) {
            return Err(StretchSelectionError::FixedAnchor { point: new.pos });
        }
    }
    for label in &candidate.net_labels {
        if after.contains_point(label.pos) != before.contains_point(label.pos) {
            return Err(StretchSelectionError::NetLabelAnchor {
                label_id: label.id,
                point: label.pos,
            });
        }
    }
    let touches = |segment: WireSegment, points: &[Point]| {
        points.iter().any(|point| segment.contains_point(*point))
            || polyline_contains_point(points, segment.start)
            || polyline_contains_point(points, segment.end)
    };
    let overlaps = |segment: WireSegment, points: &[Point]| {
        points
            .windows(2)
            .any(|pair| positive_length_overlap(segment, WireSegment::new(pair[0], pair[1])))
    };
    let others = original
        .wires
        .iter()
        .zip(&candidate.wires)
        .enumerate()
        .filter(|(index, _)| *index != wire_index)
        .map(|(_, (old, new))| (new.id, old.points.as_slice(), new.points.as_slice()))
        .chain(
            original
                .buses
                .iter()
                .zip(&candidate.buses)
                .map(|(old, new)| (new.id, old.points.as_slice(), new.points.as_slice())),
        );
    for (other_id, old_points, new_points) in others {
        if overlaps(after, new_points) && !overlaps(before, old_points) {
            return Err(StretchSelectionError::ConductorOverlap {
                object_id,
                other_id,
            });
        }
        if touches(after, new_points) && !touches(before, old_points) {
            return Err(StretchSelectionError::UnintendedConductorContact {
                object_id,
                other_id,
            });
        }
    }
    Ok(())
}

fn commit_stretch_candidate(
    document: &mut SchematicDocument,
    candidate: SchematicDocument,
    target: StretchTarget,
) -> Result<(), StretchSelectionError> {
    match target {
        StretchTarget::WireSegment { wire_id, .. } => {
            if !candidate.wires.iter().any(|wire| wire.id == wire_id) {
                return Err(StretchSelectionError::StaleTarget);
            }
            // The candidate is this document with the segment, the
            // neighbours that follow its corners and their dots moved.
            document.wires = candidate.wires;
            document.junctions = candidate.junctions;
            document.connections = candidate.connections;
            document.bus_taps = candidate.bus_taps;
        }
        StretchTarget::BusSegment { bus_id, .. } => {
            let points = candidate
                .buses
                .iter()
                .find(|bus| bus.id == bus_id)
                .map(|bus| bus.points.clone())
                .ok_or(StretchSelectionError::StaleTarget)?;
            document
                .buses
                .iter_mut()
                .find(|bus| bus.id == bus_id)
                .ok_or(StretchSelectionError::StaleTarget)?
                .points = points;
            document.bus_taps = candidate.bus_taps;
        }
        StretchTarget::DocumentationShapePoint { shape_id, .. } => {
            let geometry = candidate
                .documentation_shapes
                .iter()
                .find(|shape| shape.id == shape_id)
                .map(|shape| shape.geometry.clone())
                .ok_or(StretchSelectionError::StaleTarget)?;
            document
                .documentation_shapes
                .iter_mut()
                .find(|shape| shape.id == shape_id)
                .ok_or(StretchSelectionError::StaleTarget)?
                .geometry = geometry;
        }
    }
    Ok(())
}

pub fn target_is_live(state: &SchematicDocument, target: StretchTarget) -> bool {
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

fn conductor_points(state: &SchematicDocument, target: ConductorTarget) -> Option<&[Point]> {
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
    state: &mut SchematicDocument,
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
    state: &SchematicDocument,
    target: ConductorTarget,
    segment_index: usize,
    selected: WireSegment,
    terminal_points_by_component: &std::collections::HashMap<u64, Vec<Point>>,
    carried: &[CarriedEnd],
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
    // A dot on a corner the neighbours follow moves with them.
    if let Some(junction) = state.junctions.iter().find(|junction| {
        selected.contains_point(junction.pos) && !carries_corner(carried, junction.pos)
    }) {
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

    for (wire_index, wire) in state.wires.iter().enumerate() {
        if target == ConductorTarget::Wire(wire.id) || carries_wire(carried, wire_index) {
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
    original: &SchematicDocument,
    candidate: &mut SchematicDocument,
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

fn validate_all_tap_attachments(state: &SchematicDocument) -> Result<(), StretchSelectionError> {
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
    original: &SchematicDocument,
    candidate: &SchematicDocument,
    target: ConductorTarget,
    selected_index: usize,
    affected_indices: &[usize],
    carried: &[CarriedEnd],
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

    for (wire_index, wire) in candidate.wires.iter().enumerate() {
        let same = target == ConductorTarget::Wire(wire.id);
        let old_wire = original
            .wires
            .iter()
            .find(|old| old.id == wire.id)
            .expect("candidate preserves wire identities");
        // A carried wire is judged by the part the stretch leaves as it
        // was; the part it lays anew is checked on its own.
        let carried_end = carried.iter().find(|end| end.wire_index == wire_index);
        let (candidate_points, obstacle_indices) = match carried_end {
            Some(end) => {
                let last = old_wire.points.len().saturating_sub(2);
                let kept = (0..old_wire.points.len().saturating_sub(1))
                    .filter(|index| {
                        !((end.first.is_some() && *index == 0)
                            || (end.last.is_some() && *index == last))
                    })
                    .collect::<Vec<_>>();
                (old_wire.points.as_slice(), kept)
            }
            None => (
                wire.points.as_slice(),
                obstacle_indices(wire.points.len(), same, affected_indices),
            ),
        };
        if obstacle_indices.is_empty() {
            continue;
        }
        validate_against_obstacle(
            object_id,
            wire.id,
            ConductorMotion {
                original_affected: &original_affected,
                candidate_affected: &candidate_affected,
                old_moved,
                new_moved,
            },
            &old_wire.points,
            candidate_points,
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
            ConductorMotion {
                original_affected: &original_affected,
                candidate_affected: &candidate_affected,
                old_moved,
                new_moved,
            },
            &old_bus.points,
            &bus.points,
            &obstacle_indices,
        )?;
    }

    // A dot carried with a corner is compared where it was and where it is.
    for (old, junction) in original.junctions.iter().zip(&candidate.junctions) {
        let new_contact = candidate_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(junction.pos));
        let old_contact = original_affected
            .iter()
            .any(|(_, segment)| segment.contains_point(old.pos));
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

#[derive(Clone, Copy)]
struct ConductorMotion<'a> {
    original_affected: &'a [(usize, WireSegment)],
    candidate_affected: &'a [(usize, WireSegment)],
    old_moved: [Point; 2],
    new_moved: [Point; 2],
}

fn validate_against_obstacle(
    object_id: u64,
    other_id: u64,
    motion: ConductorMotion<'_>,
    original_obstacle_points: &[Point],
    candidate_obstacle_points: &[Point],
    obstacle_indices: &[usize],
) -> Result<(), StretchSelectionError> {
    let ConductorMotion {
        original_affected,
        candidate_affected,
        old_moved,
        new_moved,
    } = motion;
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
    candidate: &SchematicDocument,
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

pub fn documentation_shape_point_count(geometry: &DocumentationShapeGeometry) -> usize {
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
    use crate::schematic::net_label::Junction;
    use crate::schematic::wire::{Wire, WireConnection};

    fn document(wires: Vec<Wire>) -> SchematicDocument {
        SchematicDocument {
            wires,
            ..SchematicDocument::default()
        }
    }

    fn stretch(
        document: &mut SchematicDocument,
        wire_id: u64,
        delta: Point,
    ) -> Result<bool, StretchSelectionError> {
        stretch_target_resolved(
            document,
            delta,
            StretchTarget::WireSegment {
                wire_id,
                segment_index: 0,
            },
            StretchOrthogonalPolicy::PreserveOrthogonal,
            |_| Vec::new(),
            Component::bounding_box,
        )
    }

    fn points(document: &SchematicDocument, wire_id: u64) -> Vec<Point> {
        document
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .map(|wire| wire.points.clone())
            .expect("the wire is still there")
    }

    fn p(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    fn geometry(document: &SchematicDocument) -> (Vec<Wire>, Vec<Junction>, Vec<WireConnection>) {
        (
            document.wires.clone(),
            document.junctions.clone(),
            document.connections.clone(),
        )
    }

    /// The wire tool stores each segment of a route as its own wire, so the
    /// segments either side of the one being stretched are other wires.
    /// They share its corners and follow them instead of pinning them.
    #[test]
    fn a_drawn_route_stretches_with_the_segments_either_side() {
        let mut document = document(vec![
            Wire::segment(1, p(0, 0), p(0, 20)),
            Wire::segment(2, p(0, 20), p(40, 20)),
            Wire::segment(3, p(40, 20), p(40, 0)),
        ]);

        assert_eq!(stretch(&mut document, 2, p(0, 10)), Ok(true));

        assert_eq!(points(&document, 1), [p(0, 0), p(0, 30)]);
        assert_eq!(points(&document, 2), [p(0, 30), p(40, 30)]);
        assert_eq!(points(&document, 3), [p(40, 30), p(40, 0)]);
    }

    /// A neighbour running on in the stretched segment's own direction is
    /// squared again: it keeps its old corner and turns there to follow.
    #[test]
    fn a_straight_on_neighbour_turns_at_its_old_corner_to_follow() {
        for forward in [true, false] {
            let next = if forward {
                Wire::segment(3, p(40, 20), p(80, 20))
            } else {
                Wire::segment(3, p(80, 20), p(40, 20))
            };
            let mut document = document(vec![Wire::segment(2, p(0, 20), p(40, 20)), next]);

            assert_eq!(stretch(&mut document, 2, p(0, 10)), Ok(true));

            let mut expected = vec![p(40, 30), p(40, 20), p(80, 20)];
            if !forward {
                expected.reverse();
            }
            assert_eq!(points(&document, 3), expected, "forward: {forward}");
            assert!(document.wires.iter().all(Wire::is_orthogonal));
        }
    }

    /// A pin connection on the far end of a neighbour that gained a vertex
    /// at its start still names that far end.
    #[test]
    fn a_neighbours_pin_connection_follows_its_renumbered_far_end() {
        let mut document = document(vec![
            Wire::segment(2, p(0, 20), p(40, 20)),
            Wire::segment(3, p(40, 20), p(80, 20)),
        ]);
        document
            .connections
            .push(WireConnection::new(3, 1, 9, "1".to_owned()));

        assert_eq!(stretch(&mut document, 2, p(0, 10)), Ok(true));

        assert_eq!(points(&document, 3)[2], p(80, 20));
        assert_eq!(document.connections[0].point_index, 2);
    }

    /// A T at the corner moves as one: the stem on either side follows and
    /// its dot goes with the corner.
    #[test]
    fn a_dot_at_a_carried_corner_moves_with_it() {
        let mut document = document(vec![
            Wire::segment(2, p(0, 20), p(40, 20)),
            Wire::segment(3, p(40, 20), p(40, 0)),
            Wire::segment(4, p(40, 20), p(40, 40)),
        ]);
        document.junctions.push(Junction::new(5, p(40, 20)));

        assert_eq!(stretch(&mut document, 2, p(0, 10)), Ok(true));

        assert_eq!(points(&document, 3), [p(40, 30), p(40, 0)]);
        assert_eq!(points(&document, 4), [p(40, 30), p(40, 40)]);
        assert_eq!(document.junctions[0].pos, p(40, 30));
    }

    /// Following would lay one neighbour along another, a doubled
    /// conductor; the stretch refuses and changes nothing.
    #[test]
    fn a_neighbour_that_would_double_another_refuses_atomically() {
        let mut document = document(vec![
            Wire::segment(2, p(0, 20), p(40, 20)),
            Wire::segment(3, p(40, 20), p(40, 0)),
            Wire::segment(4, p(40, 20), p(80, 20)),
        ]);
        let before = geometry(&document);

        assert!(matches!(
            stretch(&mut document, 2, p(0, 10)),
            Err(StretchSelectionError::ConductorOverlap { .. })
        ));
        assert_eq!(geometry(&document), before);
    }

    /// A neighbour the stretch would shrink to nothing refuses.
    #[test]
    fn a_neighbour_collapsed_to_a_point_refuses_atomically() {
        let mut document = document(vec![
            Wire::segment(1, p(0, 0), p(0, 20)),
            Wire::segment(2, p(0, 20), p(40, 20)),
        ]);
        let before = geometry(&document);

        assert_eq!(
            stretch(&mut document, 2, p(0, -20)),
            Err(StretchSelectionError::DegenerateGeometry { object_id: 1 })
        );
        assert_eq!(geometry(&document), before);
    }

    /// A wire that ends on the stretched segment's interior, or that the
    /// segment's corner lands inside, is not a neighbour: it still pins it.
    #[test]
    fn a_stem_on_the_interior_and_a_through_wire_still_pin_the_segment() {
        for other in [
            Wire::segment(3, p(20, 20), p(20, 40)),
            Wire::segment(3, p(40, 0), p(40, 40)),
        ] {
            let mut document = document(vec![Wire::segment(2, p(0, 20), p(40, 20)), other]);
            assert!(matches!(
                stretch(&mut document, 2, p(0, 10)),
                Err(StretchSelectionError::FixedAnchor { .. })
            ));
        }
    }
}
