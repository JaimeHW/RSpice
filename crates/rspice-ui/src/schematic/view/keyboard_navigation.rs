//! Keyboard-owned actions for the focused schematic canvas.
//!
//! Arrow traversal changes only presentation selection. Backspace delegates
//! to the same state-layer deletion transaction as the Edit command.

use egui::{Event, InputState, Key, Popup, Response};

use crate::workbench::app_state::{AppState, SchematicKeyboardFocus};
use crate::state::Point;
use crate::workbench::TogglePreference;

use super::SchematicSymbolContext;
use super::sheet_visibility::object_is_on_active_sheet;

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

pub(super) fn handle_keyboard_object_navigation(
    response: &Response,
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
) -> bool {
    if !response.has_focus()
        || state.schematic.tool != crate::state::Tool::Select
        || state.application_modal_open()
        || Popup::is_any_open(&response.ctx)
    {
        return false;
    }

    if response
        .ctx
        .input_mut(|input| consume_unmodified_key(input, Key::Backspace))
    {
        if !state.schematic.read_only && !state.active_view_read_only() {
            crate::workbench::app::open_delete_selection_dialog(state);
        }
        return true;
    }

    if !state
        .ui
        .preferences
        .toggle(TogglePreference::CanvasKeyboardNavigation)
    {
        return false;
    }

    let candidates = traversal_candidates(state, symbol_context);
    if candidates.is_empty() {
        return false;
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
        return false;
    };

    let current = selected_keyboard_object(state);
    let Some(object) = traversed_object(&candidates, current, direction) else {
        // The key belongs to the focused canvas even when the current object
        // is already at the edge in that direction. Do not leak it to another
        // focus owner or wrap to the opposite side.
        return true;
    };
    focus_keyboard_object(state, object);
    state.schematic.net_highlight.clear();
    true
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
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
) -> Vec<TraversalCandidate> {
    let filter = state.ui.schematic_selection_filter;
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
        for component in &state.schematic.components {
            if object_is_on_active_sheet(state, component.id) {
                let (min, max) = symbol_context.component_bounds(component);
                push(
                    SchematicKeyboardFocus::Component(component.id),
                    bounds_center(min, max),
                );
            }
        }
    }

    if filter.wires {
        for wire in &state.schematic.wires {
            if object_is_on_active_sheet(state, wire.id)
                && let Some(center) = points_center(&wire.points)
            {
                push(SchematicKeyboardFocus::Wire(wire.id), center);
            }
        }
        for bus in &state.schematic.buses {
            if object_is_on_active_sheet(state, bus.id)
                && let Some(center) = points_center(&bus.points)
            {
                push(SchematicKeyboardFocus::Bus(bus.id), center);
            }
        }
        for tap in &state.schematic.bus_taps {
            if object_is_on_active_sheet(state, tap.id)
                && let Some(center) =
                    points_center(&crate::schematic::bus_geometry::bus_tap_route_points(tap))
            {
                push(SchematicKeyboardFocus::BusTap(tap.id), center);
            }
        }
        for junction in &state.schematic.junctions {
            if object_is_on_active_sheet(state, junction.id) {
                push(SchematicKeyboardFocus::Junction(junction.id), junction.pos);
            }
        }
    }

    if filter.labels {
        for label in &state.schematic.net_labels {
            if object_is_on_active_sheet(state, label.id) {
                let (min, max) = super::net_labels::world_bounds(label);
                push(
                    SchematicKeyboardFocus::NetLabel(label.id),
                    bounds_center(min, max),
                );
            }
        }
        for probe in &state.schematic.probes {
            if object_is_on_active_sheet(state, probe.id) {
                push(SchematicKeyboardFocus::Probe(probe.id), probe.position);
            }
        }
    }

    if filter.annotations {
        for note in super::scene::visible_design_notes(state).iter() {
            let (min, max) = super::design_notes::conservative_world_bounds(note);
            push(
                SchematicKeyboardFocus::DesignNote(note.id),
                bounds_center(min, max),
            );
        }
        for shape in &state.schematic.documentation_shapes {
            if object_is_on_active_sheet(state, shape.id) {
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

fn selected_keyboard_object(state: &AppState) -> Option<SchematicKeyboardFocus> {
    let selection = &state.schematic.selection;
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
        && let Some(junction) = state
            .schematic
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
        return state.dialogs.interaction.schematic_keyboard_focus;
    }
    None
}

fn focus_keyboard_object(state: &mut AppState, object: SchematicKeyboardFocus) {
    state.schematic.selection.clear();
    match object {
        SchematicKeyboardFocus::Component(id) => {
            state.schematic.selection.select_only_component(id);
        }
        SchematicKeyboardFocus::Wire(id) => state.schematic.selection.select_only_wire(id),
        SchematicKeyboardFocus::Bus(id) => state.schematic.selection.select_only_bus(id),
        SchematicKeyboardFocus::BusTap(id) => state.schematic.selection.select_only_bus_tap(id),
        SchematicKeyboardFocus::Junction(id) => {
            if let Some(junction) = state
                .schematic
                .junctions
                .iter()
                .find(|junction| junction.id == id)
            {
                state.schematic.selection.select_only_junction(junction.pos);
            }
        }
        SchematicKeyboardFocus::NetLabel(id) => {
            state.schematic.selection.select_only_net_label(id);
        }
        SchematicKeyboardFocus::Probe(_) => {}
        SchematicKeyboardFocus::DesignNote(id) => {
            state.schematic.selection.select_only_design_note(id);
        }
        SchematicKeyboardFocus::DocumentationShape(id) => {
            state
                .schematic
                .selection
                .select_only_documentation_shape(id);
        }
    }
    state.dialogs.interaction.schematic_keyboard_focus = Some(object);
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
    use egui::{Context, Id, Modifiers, RawInput, Rect, Sense, pos2, vec2};

    use crate::state::{Component, ComponentType, Point};

    fn components() -> Vec<Component> {
        vec![
            Component::new(11, ComponentType::Resistor, Point::new(10, 20)),
            Component::new(22, ComponentType::Capacitor, Point::new(30, 40)),
            Component::new(33, ComponentType::Inductor, Point::new(50, 60)),
        ]
    }

    fn state_with_every_keyboard_object_class() -> AppState {
        use crate::state::{
            Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, DesignNote, DesignNoteKind,
            DocumentationShape, DocumentationShapeGeometry, Junction, NetLabel, SchematicProbe,
            Wire,
        };

        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            11,
            ComponentType::Resistor,
            Point::new(0, 0),
        ));
        state
            .schematic
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
        state.schematic.buses.push(bus);
        state.schematic.bus_taps.push(tap);
        state
            .schematic
            .junctions
            .push(Junction::new(15, Point::new(60, 0)));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(16, Point::new(80, 0), "OUT"));
        state.schematic.probes.push(
            SchematicProbe::new(17, Point::new(100, 0), "V(OUT)", Some("V(OUT)".to_owned()))
                .unwrap(),
        );
        state
            .schematic
            .design_notes
            .push(DesignNote::new(18, Point::new(120, 0), DesignNoteKind::PlainText, "N").unwrap());
        state.schematic.documentation_shapes.push(
            DocumentationShape::new(
                19,
                DocumentationShapeGeometry::Rectangle {
                    first: Point::new(138, -2),
                    opposite: Point::new(142, 2),
                },
            )
            .unwrap(),
        );
        state
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
        let state = state_with_every_keyboard_object_class();
        let context = SchematicSymbolContext::from_state(&state);
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
        let mut state = state_with_every_keyboard_object_class();
        state.ui.schematic_selection_filter.instances = false;
        state.ui.schematic_selection_filter.wires = false;
        state.ui.schematic_selection_filter.labels = false;
        let context = SchematicSymbolContext::from_state(&state);
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

        state.ui.schematic_selection_filter.annotations = false;
        assert!(traversal_candidates(&state, &context).is_empty());
    }

    #[test]
    fn probe_focus_is_visible_state_and_traversal_continues_into_annotations() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(0, 0),
        ));
        state.schematic.probes.push(
            crate::state::SchematicProbe::new(
                2,
                Point::new(30, 0),
                "V(OUT)",
                Some("V(OUT)".to_owned()),
            )
            .unwrap(),
        );
        state.schematic.design_notes.push(
            crate::state::DesignNote::new(
                3,
                Point::new(60, 0),
                crate::state::DesignNoteKind::PlainText,
                "N",
            )
            .unwrap(),
        );
        state.schematic.selection.select_only_component(1);

        let (handled, available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(handled);
        assert!(!available);
        assert!(state.schematic.selection.is_empty());
        assert_eq!(
            state.dialogs.interaction.schematic_keyboard_focus,
            Some(SchematicKeyboardFocus::Probe(2))
        );

        let (handled, available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(handled);
        assert!(!available);
        assert_eq!(state.schematic.selection.single_design_note(), Some(3));
        assert_eq!(
            state.dialogs.interaction.schematic_keyboard_focus,
            Some(SchematicKeyboardFocus::DesignNote(3))
        );
    }

    #[test]
    fn all_four_unmodified_arrows_follow_the_mockup_direction_contract() {
        for (key, expected) in [
            (Key::ArrowLeft, 11),
            (Key::ArrowUp, 11),
            (Key::ArrowRight, 33),
            (Key::ArrowDown, 33),
        ] {
            let ctx = Context::default();
            let mut state = AppState::default();
            state.schematic.components = components();
            state.schematic.selection.select_only_component(22);

            let (handled, _) = run_navigation_frame(&ctx, key, Modifiers::NONE, &mut state, true);

            assert!(handled, "{key:?} should traverse the focused canvas");
            assert_eq!(state.schematic.selection.single_component(), Some(expected));
        }
    }

    fn key_input(key: Key, modifiers: Modifiers) -> RawInput {
        RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
            events: vec![Event::Key {
                key,
                physical_key: Some(key),
                pressed: true,
                repeat: false,
                modifiers,
            }],
            ..Default::default()
        }
    }

    fn run_navigation_frame(
        ctx: &Context,
        key: Key,
        modifiers: Modifiers,
        state: &mut AppState,
        focus_canvas: bool,
    ) -> (bool, bool) {
        let mut outcome = (false, false);
        let _ = ctx.run_ui(key_input(key, modifiers), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // accessibility-pointer-shim: test-only canvas focus harness.
                let response = ui.interact(
                    ui.max_rect(),
                    Id::new("test-schematic-canvas"),
                    Sense::click(),
                );
                if focus_canvas {
                    response.request_focus();
                } else {
                    ui.memory_mut(|memory| memory.request_focus(Id::new("other-control")));
                }
                let symbol_context = SchematicSymbolContext::from_state(state);
                let handled = handle_keyboard_object_navigation(&response, state, &symbol_context);
                let key_still_available = ctx.input_mut(|input| input.consume_key(modifiers, key));
                outcome = (handled, key_still_available);
            });
        });
        outcome
    }

    #[test]
    fn focused_canvas_consumes_arrow_and_changes_only_selection() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.components = components();
        state.schematic.selection.select_only_component(11);
        state.schematic.net_highlight.active = true;
        state.schematic.net_highlight.highlighted_wires.insert(777);
        let topology = state.schematic.topology_version();
        let could_undo = state.schematic.can_undo();

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);

        assert!(handled);
        assert!(!key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(22));
        assert_eq!(state.schematic.center_request, None);
        assert_eq!(state.schematic.topology_version(), topology);
        assert_eq!(state.schematic.can_undo(), could_undo);
        assert!(!state.schematic.is_dirty);
        assert!(!state.schematic.net_highlight.active);
        assert!(state.schematic.net_highlight.highlighted_wires.is_empty());
    }

    #[test]
    fn unfocused_canvas_and_modified_arrow_do_not_navigate_or_consume() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.components = components();
        state.schematic.selection.select_only_component(11);

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, false);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));

        let (handled, _) = run_navigation_frame(
            &ctx,
            Key::ArrowRight,
            Modifiers {
                shift: true,
                ..Modifiers::NONE
            },
            &mut state,
            true,
        );
        assert!(!handled);
        assert_eq!(state.schematic.selection.single_component(), Some(11));
    }

    #[test]
    fn empty_canvas_and_disabled_preference_leave_arrows_unconsumed() {
        let ctx = Context::default();
        let mut state = AppState::default();

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);

        state.schematic.components = components();
        state.schematic.selection.select_only_component(11);
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::CanvasKeyboardNavigation, false);
        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));
    }

    #[test]
    fn modal_and_context_popup_owners_block_navigation() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.components = components();
        state.schematic.selection.select_only_component(11);
        state.dialogs.about = true;

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));

        state.dialogs.about = false;
        Popup::open_id(&ctx, Id::new("test-context-owner"));
        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));
        Popup::close_all(&ctx);
    }

    #[test]
    fn focused_select_canvas_backspace_opens_governed_delete_review() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.components = components();
        state.schematic.selection.select_only_component(22);
        state.schematic.init_undo_history();

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::Backspace, Modifiers::NONE, &mut state, true);

        assert!(handled);
        assert!(!key_still_available);
        assert_eq!(state.schematic.components.len(), 3);
        assert!(state.dialogs.selection_workflow.open);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn focused_select_canvas_consumes_backspace_but_never_edits_read_only_content() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.components = components();
        state.schematic.selection.select_only_component(22);
        state.schematic.read_only = true;

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::Backspace, Modifiers::NONE, &mut state, true);

        assert!(handled);
        assert!(!key_still_available);
        assert_eq!(state.schematic.components.len(), 3);
        assert!(!state.schematic.can_undo());
    }
}
