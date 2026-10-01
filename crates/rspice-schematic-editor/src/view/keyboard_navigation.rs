//! Focused schematic keyboard input over read-only design and editor views.

use super::design_view::DesignView;
use super::symbol_context::SchematicSymbolContext;
use crate::requests::{EditorAction, EditorRequest, EditorRequestSource};
use crate::session::{
    EditorSession,
    selection::{SchematicKeyboardFocus, SchematicSelectionFilter},
    tool::Tool,
};
use egui::{Event, InputState, Key, Popup, Response};
use rspice_design::schematic::{document::SchematicDocument, selection::Selection};
use rspice_design_model::Point;

/// Read-only inputs needed to resolve spatial keyboard traversal.
pub struct KeyboardNavigationView<'a> {
    pub design: DesignView<'a>,
    pub editor: &'a EditorSession,
    pub keyboard_focus: Option<SchematicKeyboardFocus>,
    pub filter: SchematicSelectionFilter,
}

#[derive(Clone, Copy)]
pub struct KeyboardCapabilities {
    pub modal_open: bool,
    pub can_edit: bool,
    pub traversal_enabled: bool,
}

pub enum KeyboardNavigationOutcome {
    Ignored,
    Consumed,
    Request(Box<EditorRequest>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TraversalDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TraversalCandidate {
    object: SchematicKeyboardFocus,
    center: Point,
    scene_order: usize,
}

pub fn handle_keyboard_object_navigation(
    response: &Response,
    view: &KeyboardNavigationView<'_>,
    capabilities: KeyboardCapabilities,
    symbol_context: &SchematicSymbolContext,
    source: impl FnOnce() -> EditorRequestSource,
) -> KeyboardNavigationOutcome {
    if !response.has_focus()
        || view.editor.tool != Tool::Select
        || capabilities.modal_open
        || Popup::is_any_open(&response.ctx)
    {
        return KeyboardNavigationOutcome::Ignored;
    }

    if response
        .ctx
        .input_mut(|input| consume_unmodified_key(input, Key::Backspace))
    {
        return if capabilities.can_edit {
            KeyboardNavigationOutcome::Request(Box::new(EditorRequest {
                source: source(),
                selection: view.editor.selection.clone(),
                action: EditorAction::DeleteSelection,
            }))
        } else {
            KeyboardNavigationOutcome::Consumed
        };
    }

    if !capabilities.traversal_enabled {
        return KeyboardNavigationOutcome::Ignored;
    }

    let candidates = traversal_candidates(view, symbol_context);
    if candidates.is_empty() {
        return KeyboardNavigationOutcome::Ignored;
    }

    let direction = response.ctx.input_mut(|input| {
        if consume_unmodified_key(input, Key::ArrowLeft) {
            Some(TraversalDirection::Left)
        } else if consume_unmodified_key(input, Key::ArrowRight) {
            Some(TraversalDirection::Right)
        } else if consume_unmodified_key(input, Key::ArrowUp) {
            Some(TraversalDirection::Up)
        } else if consume_unmodified_key(input, Key::ArrowDown) {
            Some(TraversalDirection::Down)
        } else {
            None
        }
    });
    let Some(direction) = direction else {
        return KeyboardNavigationOutcome::Ignored;
    };

    let current = selected_keyboard_object(view);
    let Some(object) = traversed_object(&candidates, current, direction) else {
        // The key belongs to the focused canvas even when the current object
        // is already at the edge in that direction. Do not leak it to another
        // focus owner or wrap to the opposite side.
        return KeyboardNavigationOutcome::Consumed;
    };
    KeyboardNavigationOutcome::Request(Box::new(EditorRequest {
        source: source(),
        selection: view.editor.selection.clone(),
        action: EditorAction::Focus(object),
    }))
}

fn consume_unmodified_key(input: &mut InputState, requested: Key) -> bool {
    let Some(index) = input.events.iter().position(|event| {
        matches!(
            event,
            Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if *key == requested && modifiers.is_none()
        )
    }) else {
        return false;
    };
    input.events.remove(index);
    true
}

fn traversed_object(
    candidates: &[TraversalCandidate],
    selected: Option<SchematicKeyboardFocus>,
    direction: TraversalDirection,
) -> Option<SchematicKeyboardFocus> {
    let Some(current_index) =
        selected.and_then(|selected| candidates.iter().position(|item| item.object == selected))
    else {
        return candidates.first().map(|candidate| candidate.object);
    };
    let origin = candidates[current_index].center;

    candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| candidate.object != candidates[current_index].object)
        .filter_map(|(_, candidate)| {
            directional_score(origin, candidate.center, direction).map(|score| {
                // Preserve authored scene order for exact score ties, matching
                // the stable Array.sort used by the upgraded mockup.
                (score, candidate.scene_order, candidate.object)
            })
        })
        .min_by_key(|(score, scene_order, _)| (*score, *scene_order))
        .map(|(_, _, object)| object)
}

fn traversal_candidates(
    view: &KeyboardNavigationView<'_>,
    symbol_context: &SchematicSymbolContext,
) -> Vec<TraversalCandidate> {
    let filter = view.filter;
    let mut candidates = Vec::new();
    let mut push = |object, center| {
        let scene_order = candidates.len();
        candidates.push(TraversalCandidate {
            object,
            center,
            scene_order,
        });
    };

    if filter.instances {
        for component in &view.design.document.components {
            if view.design.object_is_visible(component.id) {
                let (min, max) = symbol_context.component_bounds(component);
                push(
                    SchematicKeyboardFocus::Component(component.id),
                    bounds_center(min, max),
                );
            }
        }
    }

    if filter.wires {
        for wire in &view.design.document.wires {
            if view.design.object_is_visible(wire.id)
                && let Some(center) = points_center(&wire.points)
            {
                push(SchematicKeyboardFocus::Wire(wire.id), center);
            }
        }
        for bus in &view.design.document.buses {
            if view.design.object_is_visible(bus.id)
                && let Some(center) = points_center(&bus.points)
            {
                push(SchematicKeyboardFocus::Bus(bus.id), center);
            }
        }
        for tap in &view.design.document.bus_taps {
            if view.design.object_is_visible(tap.id)
                && let Some(center) = points_center(&crate::bus_geometry::bus_tap_route_points(tap))
            {
                push(SchematicKeyboardFocus::BusTap(tap.id), center);
            }
        }
        for junction in &view.design.document.junctions {
            if view.design.object_is_visible(junction.id) {
                push(SchematicKeyboardFocus::Junction(junction.id), junction.pos);
            }
        }
    }

    if filter.labels {
        for label in &view.design.document.net_labels {
            if view.design.object_is_visible(label.id) {
                let (min, max) = super::net_labels::world_bounds(label);
                push(
                    SchematicKeyboardFocus::NetLabel(label.id),
                    bounds_center(min, max),
                );
            }
        }
        for probe in &view.design.document.probes {
            if view.design.object_is_visible(probe.id) {
                push(SchematicKeyboardFocus::Probe(probe.id), probe.position);
            }
        }
    }

    if filter.annotations {
        for note in view.design.visible_design_notes().iter() {
            let (min, max) = super::design_notes::conservative_world_bounds(note);
            push(
                SchematicKeyboardFocus::DesignNote(note.id),
                bounds_center(min, max),
            );
        }
        for shape in &view.design.document.documentation_shapes {
            if view.design.object_is_visible(shape.id) {
                let (min, max) = super::documentation_shapes::world_bounds(shape);
                push(
                    SchematicKeyboardFocus::DocumentationShape(shape.id),
                    bounds_center(min, max),
                );
            }
        }
    }

    candidates
}

fn selected_keyboard_object(view: &KeyboardNavigationView<'_>) -> Option<SchematicKeyboardFocus> {
    let selection = &view.editor.selection;
    if let Some(id) = selection.single_component() {
        return Some(SchematicKeyboardFocus::Component(id));
    }
    if let Some(id) = selection.single_wire() {
        return Some(SchematicKeyboardFocus::Wire(id));
    }
    if let Some(selected) = selection.single_wire_segment() {
        return Some(SchematicKeyboardFocus::Wire(selected.wire_id));
    }
    if let Some(selected) = selection.single_wire_vertex() {
        return Some(SchematicKeyboardFocus::Wire(selected.wire_id));
    }
    if let Some(id) = selection.single_bus() {
        return Some(SchematicKeyboardFocus::Bus(id));
    }
    if let Some(id) = selection.single_bus_tap() {
        return Some(SchematicKeyboardFocus::BusTap(id));
    }
    if let Some(position) = selection.single_junction()
        && let Some(junction) = view
            .design
            .document
            .junctions
            .iter()
            .find(|junction| junction.pos == position)
    {
        return Some(SchematicKeyboardFocus::Junction(junction.id));
    }
    if let Some(id) = selection.single_net_label() {
        return Some(SchematicKeyboardFocus::NetLabel(id));
    }
    if let Some(id) = selection.single_design_note() {
        return Some(SchematicKeyboardFocus::DesignNote(id));
    }
    if let Some(id) = selection.single_documentation_shape() {
        return Some(SchematicKeyboardFocus::DocumentationShape(id));
    }
    if selection.is_empty() {
        return view.keyboard_focus;
    }
    None
}

pub fn focus_keyboard_object(
    document: &SchematicDocument,
    selection: &mut Selection,
    object: SchematicKeyboardFocus,
) {
    selection.clear();
    match object {
        SchematicKeyboardFocus::Component(id) => {
            selection.select_only_component(id);
        }
        SchematicKeyboardFocus::Wire(id) => selection.select_only_wire(id),
        SchematicKeyboardFocus::Bus(id) => selection.select_only_bus(id),
        SchematicKeyboardFocus::BusTap(id) => selection.select_only_bus_tap(id),
        SchematicKeyboardFocus::Junction(id) => {
            if let Some(junction) = document.junctions.iter().find(|junction| junction.id == id) {
                selection.select_only_junction(junction.pos);
            }
        }
        SchematicKeyboardFocus::NetLabel(id) => {
            selection.select_only_net_label(id);
        }
        SchematicKeyboardFocus::Probe(_) => {}
        SchematicKeyboardFocus::DesignNote(id) => {
            selection.select_only_design_note(id);
        }
        SchematicKeyboardFocus::DocumentationShape(id) => {
            selection.select_only_documentation_shape(id);
        }
    }
}

fn points_center(points: &[Point]) -> Option<Point> {
    let first = *points.first()?;
    let (mut min, mut max) = (first, first);
    for point in &points[1..] {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }
    Some(bounds_center(min, max))
}

fn bounds_center(min: Point, max: Point) -> Point {
    fn center(a: i32, b: i32) -> i32 {
        ((i64::from(a) + i64::from(b)) / 2).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
    }
    Point::new(center(min.x, max.x), center(min.y, max.y))
}

fn directional_score(
    origin: Point,
    candidate: Point,
    direction: TraversalDirection,
) -> Option<i64> {
    let dx = i64::from(candidate.x) - i64::from(origin.x);
    let dy = i64::from(candidate.y) - i64::from(origin.y);
    let (along, across) = match direction {
        TraversalDirection::Left => (-dx, dy.abs()),
        TraversalDirection::Right => (dx, dy.abs()),
        TraversalDirection::Up => (-dy, dx.abs()),
        TraversalDirection::Down => (dy, dx.abs()),
    };
    // The mockup rejects centres within one canvas unit of the current
    // object's directional axis, then ranks by along + across * 0.6.
    (along > 1).then_some(along * 5 + across * 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::{component::Component, component_type::ComponentType};
    fn view<'a>(
        document: &'a SchematicDocument,
        editor: &'a EditorSession,
    ) -> KeyboardNavigationView<'a> {
        KeyboardNavigationView {
            design: DesignView {
                document,
                canvas_cache: None,
                sheet_catalog: None,
                review_markers: crate::session::visibility::SchematicReviewMarkerVisibility::All,
            },
            editor,
            keyboard_focus: None,
            filter: SchematicSelectionFilter::default(),
        }
    }
    fn document_with_every_keyboard_object_class() -> SchematicDocument {
        use rspice_design::schematic::{
            bus::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation},
            design_note::{DesignNote, DesignNoteKind},
            documentation_shape::{DocumentationShape, DocumentationShapeGeometry},
            net_label::{Junction, NetLabel},
            probe::SchematicProbe,
            wire::Wire,
        };

        let mut document = SchematicDocument::default();
        document.components.push(Component::new(
            11,
            ComponentType::Resistor,
            Point::new(0, 0),
        ));
        document
            .wires
            .push(Wire::segment(12, Point::new(18, 0), Point::new(22, 0)));
        let bus = Bus::segment(
            13,
            Point::new(38, 0),
            Point::new(42, 0),
            Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            14,
            &bus,
            Point::new(40, 0),
            Point::new(40, 10),
            BusSlice::parse("DATA[1]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        document.buses.push(bus);
        document.bus_taps.push(tap);
        document
            .junctions
            .push(Junction::new(15, Point::new(60, 0)));
        document
            .net_labels
            .push(NetLabel::new(16, Point::new(80, 0), "OUT"));
        document.probes.push(
            SchematicProbe::new(17, Point::new(100, 0), "V(OUT)", Some("V(OUT)".to_owned()))
                .unwrap(),
        );
        document
            .design_notes
            .push(DesignNote::new(18, Point::new(120, 0), DesignNoteKind::PlainText, "N").unwrap());
        document.documentation_shapes.push(
            DocumentationShape::new(
                19,
                DocumentationShapeGeometry::Rectangle {
                    first: Point::new(138, -2),
                    opposite: Point::new(142, 2),
                },
            )
            .unwrap(),
        );
        document
    }

    fn candidate(
        object: SchematicKeyboardFocus,
        x: i32,
        y: i32,
        scene_order: usize,
    ) -> TraversalCandidate {
        TraversalCandidate {
            object,
            center: Point::new(x, y),
            scene_order,
        }
    }

    #[test]
    fn traversal_is_spatial_and_does_not_wrap_at_directional_edges() {
        let candidates = vec![
            candidate(SchematicKeyboardFocus::Component(1), 0, 0, 0),
            candidate(SchematicKeyboardFocus::Wire(2), 10, 20, 1),
            candidate(SchematicKeyboardFocus::Probe(3), 15, 0, 2),
            candidate(SchematicKeyboardFocus::DesignNote(4), -8, 0, 3),
        ];
        assert_eq!(
            traversed_object(
                &candidates,
                Some(SchematicKeyboardFocus::Component(1)),
                TraversalDirection::Right
            ),
            Some(SchematicKeyboardFocus::Probe(3)),
            "the aligned candidate wins the mockup's along + 0.6 across score"
        );
        assert_eq!(
            traversed_object(
                &candidates,
                Some(SchematicKeyboardFocus::Probe(3)),
                TraversalDirection::Right
            ),
            None,
            "right-arrow stops at the right edge instead of wrapping"
        );
        assert_eq!(
            traversed_object(
                &candidates,
                Some(SchematicKeyboardFocus::DesignNote(4)),
                TraversalDirection::Left
            ),
            None,
            "left-arrow stops at the left edge instead of wrapping"
        );
        assert_eq!(
            traversed_object(
                &candidates,
                Some(SchematicKeyboardFocus::Component(1)),
                TraversalDirection::Down
            ),
            Some(SchematicKeyboardFocus::Wire(2))
        );
    }

    #[test]
    fn traversal_handles_empty_absent_and_stale_selection() {
        let candidates = vec![
            candidate(SchematicKeyboardFocus::Component(11), 10, 20, 0),
            candidate(SchematicKeyboardFocus::Wire(22), 30, 40, 1),
        ];
        assert_eq!(traversed_object(&[], None, TraversalDirection::Right), None);
        assert_eq!(
            traversed_object(&candidates, None, TraversalDirection::Right),
            Some(SchematicKeyboardFocus::Component(11))
        );
        assert_eq!(
            traversed_object(&candidates, None, TraversalDirection::Left),
            Some(SchematicKeyboardFocus::Component(11))
        );
        assert_eq!(
            traversed_object(
                &candidates,
                Some(SchematicKeyboardFocus::Probe(999)),
                TraversalDirection::Down
            ),
            Some(SchematicKeyboardFocus::Component(11))
        );
        assert_eq!(
            traversed_object(
                &candidates,
                Some(SchematicKeyboardFocus::Probe(999)),
                TraversalDirection::Up
            ),
            Some(SchematicKeyboardFocus::Component(11))
        );
    }

    #[test]
    fn candidate_catalog_covers_every_schematic_keyboard_taxonomy_in_scene_order() {
        let document = document_with_every_keyboard_object_class();
        let editor = EditorSession::default();
        let state = view(&document, &editor);
        let context = SchematicSymbolContext::default();
        let objects = traversal_candidates(&state, &context)
            .into_iter()
            .map(|candidate| candidate.object)
            .collect::<Vec<_>>();

        assert_eq!(
            objects,
            vec![
                SchematicKeyboardFocus::Component(11),
                SchematicKeyboardFocus::Wire(12),
                SchematicKeyboardFocus::Bus(13),
                SchematicKeyboardFocus::BusTap(14),
                SchematicKeyboardFocus::Junction(15),
                SchematicKeyboardFocus::NetLabel(16),
                SchematicKeyboardFocus::Probe(17),
                SchematicKeyboardFocus::DesignNote(18),
                SchematicKeyboardFocus::DocumentationShape(19),
            ]
        );
    }

    #[test]
    fn candidate_catalog_honors_the_existing_selection_class_filter() {
        let document = document_with_every_keyboard_object_class();
        let editor = EditorSession::default();
        let mut state = view(&document, &editor);
        state.filter.instances = false;
        state.filter.wires = false;
        state.filter.labels = false;
        let context = SchematicSymbolContext::default();
        let objects = traversal_candidates(&state, &context)
            .into_iter()
            .map(|candidate| candidate.object)
            .collect::<Vec<_>>();

        assert_eq!(
            objects,
            vec![
                SchematicKeyboardFocus::DesignNote(18),
                SchematicKeyboardFocus::DocumentationShape(19),
            ]
        );

        state.filter.annotations = false;
        assert!(traversal_candidates(&state, &context).is_empty());
    }
}
