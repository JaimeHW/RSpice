//! Stretch draft and selection eligibility shared by canvas and app coordination.

use super::transform::TransformCanvasSession;
use rspice_design::schematic::{
    document::SchematicDocument,
    selection::Selection,
    stretch::{self, StretchTarget},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StretchCanvasSession {
    pub target: Option<StretchTarget>,
    pub gesture: TransformCanvasSession,
}

pub fn has_live_selection(document: &SchematicDocument, selection: &Selection) -> bool {
    document.wires.iter().any(|wire| {
        selection.has_wire(wire.id) && wire.segment_count() != 0
            || selection.wire_segments.iter().any(|selected| {
                selected.wire_id == wire.id && selected.segment_index < wire.segment_count()
            })
            || selection.wire_vertices.iter().any(|selected| {
                selected.wire_id == wire.id
                    && selected.vertex_index < wire.vertex_count()
                    && wire.segment_count() != 0
            })
    }) || document
        .buses
        .iter()
        .any(|bus| selection.has_bus(bus.id) && bus.points.len() >= 2)
        || document.documentation_shapes.iter().any(|shape| {
            selection.has_documentation_shape(shape.id)
                && stretch::documentation_shape_point_count(&shape.geometry) != 0
        })
}

pub fn default_target(
    document: &SchematicDocument,
    selection: &Selection,
) -> Option<StretchTarget> {
    let mut targets = Vec::new();
    for selected in &selection.wire_segments {
        push_unique_target(
            &mut targets,
            StretchTarget::WireSegment {
                wire_id: selected.wire_id,
                segment_index: selected.segment_index,
            },
            document,
            selection,
        );
    }
    for selected in &selection.wire_vertices {
        let Some(wire) = document
            .wires
            .iter()
            .find(|wire| wire.id == selected.wire_id)
        else {
            continue;
        };
        let segment_index = if selected.vertex_index < wire.segment_count() {
            selected.vertex_index
        } else if selected.vertex_index != 0 && selected.vertex_index - 1 < wire.segment_count() {
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
            document,
            selection,
        );
    }
    for &wire_id in &selection.wires {
        push_unique_target(
            &mut targets,
            StretchTarget::WireSegment {
                wire_id,
                segment_index: 0,
            },
            document,
            selection,
        );
    }
    for &bus_id in &selection.buses {
        push_unique_target(
            &mut targets,
            StretchTarget::BusSegment {
                bus_id,
                segment_index: 0,
            },
            document,
            selection,
        );
    }
    for &shape_id in &selection.documentation_shapes {
        push_unique_target(
            &mut targets,
            StretchTarget::DocumentationShapePoint {
                shape_id,
                point_index: 0,
            },
            document,
            selection,
        );
    }
    (targets.len() == 1).then(|| targets[0])
}

pub fn target_is_eligible(
    document: &SchematicDocument,
    selection: &Selection,
    target: StretchTarget,
) -> bool {
    stretch::target_is_live(document, target) && selection_authorizes_target(selection, target)
}

fn push_unique_target(
    targets: &mut Vec<StretchTarget>,
    target: StretchTarget,
    document: &SchematicDocument,
    selection: &Selection,
) {
    if target_is_eligible(document, selection, target) && !targets.contains(&target) {
        targets.push(target);
    }
}

pub fn selection_authorizes_target(selection: &Selection, target: StretchTarget) -> bool {
    match target {
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        } => {
            selection.has_wire(wire_id)
                || selection.has_wire_segment(wire_id, segment_index)
                || selection.has_wire_vertex(wire_id, segment_index)
                || segment_index
                    .checked_add(1)
                    .is_some_and(|vertex| selection.has_wire_vertex(wire_id, vertex))
        }
        StretchTarget::BusSegment { bus_id, .. } => selection.has_bus(bus_id),
        StretchTarget::DocumentationShapePoint { shape_id, .. } => {
            selection.has_documentation_shape(shape_id)
        }
    }
}
