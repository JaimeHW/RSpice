//! Keyboard-owned instance traversal for the focused schematic canvas.
//!
//! Arrow traversal changes only presentation selection. It never opens an
//! undo transaction, moves the camera, or changes document topology.

use egui::{Event, InputState, Key, Popup, Response};

use crate::common::app::AppState;
use crate::state::Component;
use crate::workbench::TogglePreference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TraversalDirection {
    Previous,
    Next,
}

pub(super) fn handle_keyboard_instance_navigation(
    response: &Response,
    state: &mut AppState,
) -> bool {
    if !response.has_focus()
        || state.schematic.tool != crate::state::Tool::Select
        || !state
            .ui
            .preferences
            .toggle(TogglePreference::CanvasKeyboardNavigation)
        || state.application_modal_open()
        || Popup::is_any_open(&response.ctx)
        || state.schematic.components.is_empty()
    {
        return false;
    }

    let direction = response.ctx.input_mut(|input| {
        if consume_unmodified_key(input, Key::ArrowLeft)
            || consume_unmodified_key(input, Key::ArrowUp)
        {
            Some(TraversalDirection::Previous)
        } else if consume_unmodified_key(input, Key::ArrowRight)
            || consume_unmodified_key(input, Key::ArrowDown)
        {
            Some(TraversalDirection::Next)
        } else {
            None
        }
    });
    let Some(direction) = direction else {
        return false;
    };

    let current = state.schematic.selection.single_component();
    let Some(component_id) =
        traversed_component_id(&state.schematic.components, current, direction)
    else {
        return false;
    };
    state
        .schematic
        .selection
        .select_only_component(component_id);
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

fn traversed_component_id(
    components: &[Component],
    selected_component: Option<u64>,
    direction: TraversalDirection,
) -> Option<u64> {
    let len = components.len();
    if len == 0 {
        return None;
    }

    let current = selected_component.and_then(|selected| {
        components
            .iter()
            .position(|component| component.id == selected)
    });
    let index = match (current, direction) {
        (Some(index), TraversalDirection::Previous) => (index + len - 1) % len,
        (Some(index), TraversalDirection::Next) => (index + 1) % len,
        (None, TraversalDirection::Previous) => len - 1,
        (None, TraversalDirection::Next) => 0,
    };
    Some(components[index].id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Context, Id, Modifiers, RawInput, Rect, Sense, pos2, vec2};

    use crate::state::{ComponentType, Point};

    fn components() -> Vec<Component> {
        vec![
            Component::new(11, ComponentType::Resistor, Point::new(10, 20)),
            Component::new(22, ComponentType::Capacitor, Point::new(30, 40)),
            Component::new(33, ComponentType::Inductor, Point::new(50, 60)),
        ]
    }

    #[test]
    fn traversal_is_deterministic_and_wraps_in_both_directions() {
        let components = components();
        assert_eq!(
            traversed_component_id(&components, Some(11), TraversalDirection::Next),
            Some(22)
        );
        assert_eq!(
            traversed_component_id(&components, Some(33), TraversalDirection::Next),
            Some(11)
        );
        assert_eq!(
            traversed_component_id(&components, Some(11), TraversalDirection::Previous),
            Some(33)
        );
        assert_eq!(
            traversed_component_id(&components, Some(22), TraversalDirection::Previous),
            Some(11)
        );
    }

    #[test]
    fn traversal_handles_empty_absent_and_stale_selection() {
        let components = components();
        assert_eq!(
            traversed_component_id(&[], None, TraversalDirection::Next),
            None
        );
        assert_eq!(
            traversed_component_id(&components, None, TraversalDirection::Next),
            Some(11)
        );
        assert_eq!(
            traversed_component_id(&components, None, TraversalDirection::Previous),
            Some(33)
        );
        assert_eq!(
            traversed_component_id(&components, Some(999), TraversalDirection::Next),
            Some(11)
        );
        assert_eq!(
            traversed_component_id(&components, Some(999), TraversalDirection::Previous),
            Some(33)
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

            let (handled, _) =
                run_navigation_frame(&ctx, key_input(key, Modifiers::NONE), &mut state, true);

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
        input: RawInput,
        state: &mut AppState,
        focus_canvas: bool,
    ) -> (bool, bool) {
        let mut outcome = (false, false);
        let _ = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
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
                let handled = handle_keyboard_instance_navigation(&response, state);
                let key_still_available =
                    ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowRight));
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

        let (handled, key_still_available) = run_navigation_frame(
            &ctx,
            key_input(Key::ArrowRight, Modifiers::NONE),
            &mut state,
            true,
        );

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

        let (handled, key_still_available) = run_navigation_frame(
            &ctx,
            key_input(Key::ArrowRight, Modifiers::NONE),
            &mut state,
            false,
        );
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));

        let (handled, _) = run_navigation_frame(
            &ctx,
            key_input(
                Key::ArrowRight,
                Modifiers {
                    shift: true,
                    ..Modifiers::NONE
                },
            ),
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

        let (handled, key_still_available) = run_navigation_frame(
            &ctx,
            key_input(Key::ArrowRight, Modifiers::NONE),
            &mut state,
            true,
        );
        assert!(!handled);
        assert!(key_still_available);

        state.schematic.components = components();
        state.schematic.selection.select_only_component(11);
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::CanvasKeyboardNavigation, false);
        let (handled, key_still_available) = run_navigation_frame(
            &ctx,
            key_input(Key::ArrowRight, Modifiers::NONE),
            &mut state,
            true,
        );
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

        let (handled, key_still_available) = run_navigation_frame(
            &ctx,
            key_input(Key::ArrowRight, Modifiers::NONE),
            &mut state,
            true,
        );
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));

        state.dialogs.about = false;
        Popup::open_id(&ctx, Id::new("test-context-owner"));
        let (handled, key_still_available) = run_navigation_frame(
            &ctx,
            key_input(Key::ArrowRight, Modifiers::NONE),
            &mut state,
            true,
        );
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(state.schematic.selection.single_component(), Some(11));
        Popup::close_all(&ctx);
    }
}
