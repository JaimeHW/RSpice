//! Stretch gesture input and selected-handle queries over a borrowed design.

use super::{
    coordinates::screen_to_schematic,
    design_view::DesignView,
    snap_resolution::resolve_grid_pointer,
    transform_input::{
        TransformInputTransition, consume_keyboard, retain_canvas_focus_from_pointer,
    },
    viewport::Viewport,
};
use crate::{
    requests::EditorRequestSource,
    session::{
        snap::SnapEngine,
        stretch::{StretchCanvasSession, target_is_eligible},
    },
};
use egui::{Response, Ui};
use rspice_design::schematic::{
    document::SchematicDocument,
    selection::Selection,
    stretch::{StretchOrthogonalPolicy, StretchTarget},
    wire::WireSegment,
};
use rspice_design_model::Point;

const DELTA_OVERFLOW: &str = "The requested stretch exceeds the schematic coordinate range.";

#[derive(Debug, Clone)]
pub struct StretchInputRequest {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub generation: u64,
    pub policy: StretchOrthogonalPolicy,
    pub transition: TransformInputTransition<StretchCanvasSession>,
}

pub struct StretchInputView<'a> {
    pub design: DesignView<'a>,
    pub selection: &'a Selection,
    pub canvas: &'a StretchCanvasSession,
    pub policy: StretchOrthogonalPolicy,
    pub snap_engine: &'a SnapEngine,
}

pub fn input(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    view: StretchInputView<'_>,
) -> Option<TransformInputTransition<StretchCanvasSession>> {
    view.canvas.target?;
    let mut next = view.canvas.clone();
    let commit = apply_input(ui, response, viewport, &view, &mut next);
    (commit || next != *view.canvas).then(|| TransformInputTransition {
        expected: view.canvas.clone(),
        next,
        commit,
    })
}

fn apply_input(
    ui: &Ui,
    response: &Response,
    viewport: &Viewport,
    view: &StretchInputView<'_>,
    draft: &mut StretchCanvasSession,
) -> bool {
    retain_canvas_focus_from_pointer(response);
    let (keyboard_step, keyboard_commit) =
        consume_keyboard(ui, response.has_focus(), view.design.document.grid_size);
    if keyboard_step != Point::origin() {
        let target = draft.target.expect("validated stretch target");
        let requested = checked_accumulate_delta(draft.gesture.preview_delta, keyboard_step);
        draft.gesture.anchor = None;
        draft.gesture.pointer_drag = false;
        update_preview_delta(view, draft, target, requested);
    }
    if keyboard_commit {
        return true;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(position) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
    {
        let anchor = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        if let Some(target) = stretch_target_at(view, viewport, position) {
            draft.target = Some(target);
            draft.gesture.anchor = Some(anchor);
            draft.gesture.preview_delta = Point::origin();
            draft.gesture.pointer_drag = true;
            draft.gesture.preview_error = None;
        }
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && draft.gesture.pointer_drag
        && let (Some(anchor), Some(position), Some(target)) = (
            draft.gesture.anchor,
            response
                .hover_pos()
                .or_else(|| response.interact_pointer_pos()),
            draft.target,
        )
    {
        let destination = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        update_preview_delta(
            view,
            draft,
            target,
            checked_pointer_delta(anchor, destination),
        );
    }

    if response.drag_stopped_by(egui::PointerButton::Primary) && draft.gesture.pointer_drag {
        draft.gesture.pointer_drag = false;
        return true;
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let point = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        if let (Some(anchor), Some(target)) = (draft.gesture.anchor, draft.target) {
            update_preview_delta(view, draft, target, checked_pointer_delta(anchor, point));
            return true;
        } else if let Some(target) = stretch_target_at(view, viewport, position) {
            draft.target = Some(target);
            draft.gesture.anchor = Some(point);
            draft.gesture.preview_delta = Point::origin();
            draft.gesture.preview_error = None;
        }
    } else if !draft.gesture.pointer_drag
        && let (Some(anchor), Some(position), Some(target)) =
            (draft.gesture.anchor, response.hover_pos(), draft.target)
    {
        let destination = resolve_grid_pointer(
            view.snap_engine,
            view.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        update_preview_delta(
            view,
            draft,
            target,
            checked_pointer_delta(anchor, destination),
        );
    }

    false
}

fn checked_accumulate_delta(current: Point, step: Point) -> Result<Point, &'static str> {
    let Some(x) = current.x.checked_add(step.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = current.y.checked_add(step.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

fn checked_pointer_delta(anchor: Point, destination: Point) -> Result<Point, &'static str> {
    let Some(x) = destination.x.checked_sub(anchor.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = destination.y.checked_sub(anchor.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

fn update_preview_delta(
    view: &StretchInputView<'_>,
    draft: &mut StretchCanvasSession,
    target: StretchTarget,
    requested: Result<Point, &'static str>,
) {
    match requested {
        Ok(requested) => {
            let delta =
                stretch_delta_for_policy(requested, target, view.policy, view.design.document);
            draft.gesture.preview_delta = delta;
            draft.gesture.preview_error = None;
        }
        Err(message) => {
            draft.gesture.preview_delta = Point::origin();
            draft.gesture.preview_error = Some(message.to_owned());
        }
    }
}

fn stretch_target_at(
    view: &StretchInputView<'_>,
    viewport: &Viewport,
    position: egui::Pos2,
) -> Option<StretchTarget> {
    let point = screen_to_schematic(viewport, position);
    let tolerance = f64::from((6.0 / viewport.zoom.max(0.1)).ceil() as i32);
    let selection = view.selection;
    let mut candidates: Vec<(f64, u8, u64, usize, StretchTarget)> = Vec::new();

    for wire in view.design.document.wires.iter().filter(|wire| {
        view.design.object_is_visible(wire.id)
            && (selection.has_wire(wire.id)
                || selection
                    .wire_segments
                    .iter()
                    .any(|selected| selected.wire_id == wire.id)
                || selection
                    .wire_vertices
                    .iter()
                    .any(|selected| selected.wire_id == wire.id))
    }) {
        for (segment_index, endpoints) in wire.points.windows(2).enumerate() {
            let distance = WireSegment::new(endpoints[0], endpoints[1]).distance_to_point(point);
            let target = StretchTarget::WireSegment {
                wire_id: wire.id,
                segment_index,
            };
            if distance <= tolerance
                && target_is_eligible(view.design.document, view.selection, target)
            {
                candidates.push((distance, 1, wire.id, segment_index, target));
            }
        }
    }
    for bus in view
        .design
        .document
        .buses
        .iter()
        .filter(|bus| selection.has_bus(bus.id) && view.design.object_is_visible(bus.id))
    {
        for (segment_index, endpoints) in bus.points.windows(2).enumerate() {
            let distance = WireSegment::new(endpoints[0], endpoints[1]).distance_to_point(point);
            let target = StretchTarget::BusSegment {
                bus_id: bus.id,
                segment_index,
            };
            if distance <= tolerance
                && target_is_eligible(view.design.document, view.selection, target)
            {
                candidates.push((distance, 2, bus.id, segment_index, target));
            }
        }
    }
    for shape in view
        .design
        .document
        .documentation_shapes
        .iter()
        .filter(|shape| {
            selection.has_documentation_shape(shape.id) && view.design.object_is_visible(shape.id)
        })
    {
        for (point_index, control) in shape.geometry.points().into_iter().enumerate() {
            let dx = f64::from(control.x) - f64::from(point.x);
            let dy = f64::from(control.y) - f64::from(point.y);
            let distance = dx.hypot(dy);
            let target = StretchTarget::DocumentationShapePoint {
                shape_id: shape.id,
                point_index,
            };
            if distance <= tolerance
                && target_is_eligible(view.design.document, view.selection, target)
            {
                candidates.push((distance, 0, shape.id, point_index, target));
            }
        }
    }

    candidates.sort_by(|left, right| {
        left.0
            .total_cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
            .then(left.3.cmp(&right.3))
    });
    candidates.first().map(|candidate| candidate.4)
}

pub fn stretch_delta_for_policy(
    delta: Point,
    target: StretchTarget,
    policy: StretchOrthogonalPolicy,
    document: &SchematicDocument,
) -> Point {
    if policy == StretchOrthogonalPolicy::AllowDiagonal {
        return delta;
    }
    let segment_end = match target {
        StretchTarget::WireSegment { segment_index, .. }
        | StretchTarget::BusSegment { segment_index, .. } => segment_index.checked_add(1),
        StretchTarget::DocumentationShapePoint { .. } => return delta,
    };
    let Some(segment_end) = segment_end else {
        return delta;
    };
    let segment = match target {
        StretchTarget::WireSegment {
            wire_id,
            segment_index,
        } => document
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .and_then(|wire| wire.points.get(segment_index..=segment_end)),
        StretchTarget::BusSegment {
            bus_id,
            segment_index,
        } => document
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .and_then(|bus| bus.points.get(segment_index..=segment_end)),
        StretchTarget::DocumentationShapePoint { .. } => return delta,
    };
    let Some(segment) = segment else {
        return delta;
    };
    if segment[0].x == segment[1].x {
        Point::new(delta.x, 0)
    } else if segment[0].y == segment[1].y {
        Point::new(0, delta.y)
    } else {
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::wire::Wire;
    #[test]
    fn target_resolution_is_limited_to_the_frozen_selection() {
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        document
            .wires
            .push(Wire::segment(1, Point::new(0, 0), Point::new(100, 0)));
        document
            .wires
            .push(Wire::segment(2, Point::new(0, 10), Point::new(100, 10)));
        selection.select_only_wire_segment(2, 0);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(200.0)),
        };
        let canvas = StretchCanvasSession::default();
        let snap = SnapEngine::default();
        let view = StretchInputView {
            design: DesignView {
                document: &document,
                canvas_cache: None,
                sheet_catalog: None,
                review_markers: Default::default(),
            },
            selection: &selection,
            canvas: &canvas,
            policy: Default::default(),
            snap_engine: &snap,
        };
        let position = viewport.schematic_to_screen(Point::new(50, 10));
        assert_eq!(
            stretch_target_at(&view, &viewport, position),
            Some(StretchTarget::WireSegment {
                wire_id: 2,
                segment_index: 0,
            })
        );
        assert_eq!(
            StretchOrthogonalPolicy::default(),
            StretchOrthogonalPolicy::PreserveOrthogonal
        );
    }
    #[test]
    fn vertex_selection_can_resolve_either_authorized_incident_segment() {
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        document.wires.push(Wire::new(
            4,
            vec![Point::new(0, 0), Point::new(0, 20), Point::new(30, 20)],
        ));
        selection.select_only_wire_vertex(4, 1);
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 1.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(200.0)),
        };
        let canvas = StretchCanvasSession::default();
        let snap = SnapEngine::default();
        let view = StretchInputView {
            design: DesignView {
                document: &document,
                canvas_cache: None,
                sheet_catalog: None,
                review_markers: Default::default(),
            },
            selection: &selection,
            canvas: &canvas,
            policy: Default::default(),
            snap_engine: &snap,
        };
        let position = viewport.schematic_to_screen(Point::new(20, 20));
        assert_eq!(
            stretch_target_at(&view, &viewport, position),
            Some(StretchTarget::WireSegment {
                wire_id: 4,
                segment_index: 1,
            })
        );
    }
    #[test]
    fn pointer_and_keyboard_delta_overflow_is_rejected_not_saturated() {
        assert_eq!(
            checked_pointer_delta(Point::new(i32::MIN, 0), Point::new(i32::MAX, 0)),
            Err(DELTA_OVERFLOW)
        );
        assert_eq!(
            checked_accumulate_delta(Point::new(i32::MAX, 0), Point::new(1, 0)),
            Err(DELTA_OVERFLOW)
        );
    }
    #[test]
    fn preserve_orthogonal_projects_motion_perpendicular_to_segment() {
        let mut document = SchematicDocument::default();
        document
            .wires
            .push(Wire::new(7, vec![Point::new(0, 0), Point::new(20, 0)]));
        let target = StretchTarget::WireSegment {
            wire_id: 7,
            segment_index: 0,
        };
        assert_eq!(
            stretch_delta_for_policy(
                Point::new(30, 40),
                target,
                StretchOrthogonalPolicy::PreserveOrthogonal,
                &document,
            ),
            Point::new(0, 40)
        );
    }
    #[test]
    fn real_pointer_stretch_preserves_target_and_projects_only_orthogonal_policy() {
        for (drag, policy) in [
            (false, StretchOrthogonalPolicy::PreserveOrthogonal),
            (true, StretchOrthogonalPolicy::AllowDiagonal),
        ] {
            let ctx = egui::Context::default();
            let document = SchematicDocument {
                wires: vec![Wire::segment(1, Point::new(40, 50), Point::new(100, 50))],
                ..Default::default()
            };
            let mut selection = Selection::default();
            selection.select_only_wire_segment(1, 0);
            let target = StretchTarget::WireSegment {
                wire_id: 1,
                segment_index: 0,
            };
            let mut canvas = StretchCanvasSession {
                target: Some(target),
                ..Default::default()
            };
            let snap = SnapEngine::default();
            let mut frame = |events: Vec<egui::Event>| {
                let mut commit = false;
                let _ = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(400.0, 300.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |root| {
                        egui::CentralPanel::default().show(root, |ui| {
                            let bounds = ui.max_rect();
                            let viewport = Viewport {
                                offset: egui::pos2(-bounds.min.x, -bounds.min.y),
                                zoom: 1.0,
                                bounds,
                            };
                            let response = ui.interact(
                                bounds,
                                egui::Id::new("stretch"),
                                egui::Sense::click_and_drag(),
                            );
                            if let Some(transition) = input(
                                ui,
                                &response,
                                &viewport,
                                StretchInputView {
                                    design: DesignView {
                                        document: &document,
                                        canvas_cache: None,
                                        sheet_catalog: None,
                                        review_markers: Default::default(),
                                    },
                                    selection: &selection,
                                    canvas: &canvas,
                                    policy,
                                    snap_engine: &snap,
                                },
                            ) {
                                assert_eq!(transition.expected, canvas);
                                commit = transition.commit;
                                canvas = transition.next;
                            }
                        });
                    },
                );
                (commit, canvas.clone())
            };
            let button = |pos, pressed| {
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ]
            };
            frame(Vec::new());
            frame(button(egui::pos2(70.0, 50.0), true));
            if drag {
                let (commit, draft) =
                    frame(vec![egui::Event::PointerMoved(egui::pos2(120.0, 80.0))]);
                assert!(!commit);
                assert!(draft.gesture.pointer_drag);
            } else {
                let (commit, draft) = frame(button(egui::pos2(70.0, 50.0), false));
                assert!(!commit);
                assert_eq!(draft.gesture.anchor, Some(Point::new(70, 50)));
                frame(button(egui::pos2(120.0, 80.0), true));
            }
            let (commit, draft) = frame(button(egui::pos2(120.0, 80.0), false));
            assert!(commit);
            assert_eq!(draft.target, Some(target));
            assert_eq!(
                draft.gesture.preview_delta,
                if drag {
                    Point::new(50, 30)
                } else {
                    Point::new(0, 30)
                }
            );
            assert!(!draft.gesture.pointer_drag);
        }
        let mut selection = Selection::default();
        selection.select_only_wire_vertex(1, 0);
        assert!(!crate::session::stretch::selection_authorizes_target(
            &selection,
            StretchTarget::WireSegment {
                wire_id: 1,
                segment_index: usize::MAX
            }
        ));
    }
}
