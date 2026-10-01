//! App authority and dispatch for focused schematic keyboard requests.

use super::requests::{apply_editor_request, editor_request_source};
use egui::Response;
use rspice_schematic_editor::view::keyboard_navigation::{
    self, KeyboardCapabilities, KeyboardNavigationOutcome, KeyboardNavigationView,
};

use super::SchematicSymbolContext;
use crate::workbench::TogglePreference;
use crate::workbench::app_state::AppState;

pub(super) fn handle_keyboard_object_navigation(
    response: &Response,
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
) -> bool {
    let outcome = keyboard_navigation::handle_keyboard_object_navigation(
        response,
        &KeyboardNavigationView {
            design: super::schematic_design_view(state),
            editor: &state.schematic.session.editor,
            keyboard_focus: state.dialogs.interaction.schematic_keyboard_focus,
            filter: state.ui.schematic_selection_filter,
        },
        KeyboardCapabilities {
            modal_open: state.application_modal_open(),
            can_edit: !state.schematic.session.read_only && !state.active_view_read_only(),
            traversal_enabled: state
                .ui
                .preferences
                .toggle(TogglePreference::CanvasKeyboardNavigation),
        },
        symbol_context,
        || editor_request_source(state),
    );
    match outcome {
        KeyboardNavigationOutcome::Ignored => false,
        KeyboardNavigationOutcome::Consumed => true,
        KeyboardNavigationOutcome::Request(request) => {
            apply_editor_request(state, *request);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::view::schematic_symbol_context;
    use crate::workbench::app_state::SchematicKeyboardFocus;
    use egui::{Context, Event, Id, Key, Modifiers, Popup, RawInput, Rect, Sense, pos2, vec2};
    use rspice_schematic_editor::requests::{EditorAction, EditorRequest};

    use crate::state::{Component, ComponentType, Point};

    fn components() -> Vec<Component> {
        vec![
            Component::new(11, ComponentType::Resistor, Point::new(10, 20)),
            Component::new(22, ComponentType::Capacitor, Point::new(30, 40)),
            Component::new(33, ComponentType::Inductor, Point::new(50, 60)),
        ]
    }

    #[test]
    fn probe_focus_is_visible_state_and_traversal_continues_into_annotations() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        state.schematic.document_mut_for_test().probes.push(
            crate::state::SchematicProbe::new(
                2,
                Point::new(30, 0),
                "V(OUT)",
                Some("V(OUT)".to_owned()),
            )
            .unwrap(),
        );
        state.schematic.document_mut_for_test().design_notes.push(
            crate::state::DesignNote::new(
                3,
                Point::new(60, 0),
                crate::state::DesignNoteKind::PlainText,
                "N",
            )
            .unwrap(),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(1);

        let (handled, available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(handled);
        assert!(!available);
        assert!(state.schematic.session.editor.selection.is_empty());
        assert_eq!(
            state.dialogs.interaction.schematic_keyboard_focus,
            Some(SchematicKeyboardFocus::Probe(2))
        );

        let (handled, available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(handled);
        assert!(!available);
        assert_eq!(
            state
                .schematic
                .session
                .editor
                .selection
                .single_design_note(),
            Some(3)
        );
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
            state.schematic.document_mut_for_test().components = components();
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_component(22);

            let (handled, _) = run_navigation_frame(&ctx, key, Modifiers::NONE, &mut state, true);

            assert!(handled, "{key:?} should traverse the focused canvas");
            assert_eq!(
                state.schematic.session.editor.selection.single_component(),
                Some(expected)
            );
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
        let _ = ctx.run_ui(key_input(key, modifiers), |root| {
            egui::CentralPanel::default().show(root, |ui| {
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
                let symbol_context = schematic_symbol_context(state);
                let handled = handle_keyboard_object_navigation(&response, state, &symbol_context);
                let key_still_available = ui
                    .ctx()
                    .input_mut(|input| input.consume_key(modifiers, key));
                outcome = (handled, key_still_available);
            });
        });
        outcome
    }

    #[test]
    fn focused_canvas_consumes_arrow_and_changes_only_selection() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components = components();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(11);
        state.schematic.session.editor.net_highlight.active = true;
        state
            .schematic
            .session
            .editor
            .net_highlight
            .highlighted_wires
            .insert(777);
        let topology = state.schematic.topology_version();
        let could_undo = state.schematic.can_undo();

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);

        assert!(handled);
        assert!(!key_still_available);
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(22)
        );
        assert_eq!(state.schematic.session.editor.center_request, None);
        assert_eq!(state.schematic.topology_version(), topology);
        assert_eq!(state.schematic.can_undo(), could_undo);
        assert!(!state.schematic.session.is_dirty);
        assert!(!state.schematic.session.editor.net_highlight.active);
        assert!(
            state
                .schematic
                .session
                .editor
                .net_highlight
                .highlighted_wires
                .is_empty()
        );
    }

    #[test]
    fn unfocused_canvas_and_modified_arrow_do_not_navigate_or_consume() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components = components();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(11);

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, false);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(11)
        );

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
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(11)
        );
    }

    #[test]
    fn empty_canvas_and_disabled_preference_leave_arrows_unconsumed() {
        let ctx = Context::default();
        let mut state = AppState::default();

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);

        state.schematic.document_mut_for_test().components = components();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(11);
        state
            .ui
            .preferences
            .set_toggle(TogglePreference::CanvasKeyboardNavigation, false);
        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(11)
        );
    }

    #[test]
    fn modal_and_context_popup_owners_block_navigation() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components = components();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(11);
        state.dialogs.about = true;

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(11)
        );

        state.dialogs.about = false;
        Popup::open_id(&ctx, Id::new("test-context-owner"));
        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
        assert!(!handled);
        assert!(key_still_available);
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(11)
        );
        Popup::close_all(&ctx);
    }

    #[test]
    fn focused_select_canvas_backspace_deletes_immediately_in_one_undo_entry() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components = components();
        state.sync_active_schematic_to_workspace();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(22);
        state.schematic.init_undo_history();

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::Backspace, Modifiers::NONE, &mut state, true);

        assert!(handled);
        assert!(!key_still_available);
        assert_eq!(state.schematic.document().components.len(), 2);
        assert!(!state.dialogs.application_modal_open());
        assert_eq!(state.schematic.undo_description(), Some("delete selection"));
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().components.len(), 3);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn focused_select_canvas_consumes_backspace_but_never_edits_read_only_content() {
        let ctx = Context::default();
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components = components();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(22);
        state.schematic.session.read_only = true;

        let (handled, key_still_available) =
            run_navigation_frame(&ctx, Key::Backspace, Modifiers::NONE, &mut state, true);

        assert!(handled);
        assert!(!key_still_available);
        assert_eq!(state.schematic.document().components.len(), 3);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn queued_keyboard_requests_cannot_follow_reused_occurrences_or_new_selections() {
        use crate::state::{CellViewRef, ViewType};
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components = components();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(22);
        state.schematic.init_undo_history();
        let master = CellViewRef::new("user", "amp", "schematic");
        state
            .workspace
            .descend_into("X1".into(), master.clone(), ViewType::Schematic);
        let capture = |state: &AppState, action| EditorRequest {
            source: editor_request_source(state),
            selection: state.schematic.session.editor.selection.clone(),
            action,
        };
        let delete = capture(&state, EditorAction::DeleteSelection);
        let focus = capture(
            &state,
            EditorAction::Focus(SchematicKeyboardFocus::Component(33)),
        );
        let pointer = capture(
            &state,
            EditorAction::SelectPointer {
                target: Some(
                    rspice_schematic_editor::view::pointer_target::PointerTarget::Component(33),
                ),
                additive: false,
                alt_held: false,
            },
        );
        state.workspace.ascend_one().expect("return to parent");
        state
            .workspace
            .descend_into("X2".into(), master, ViewType::Schematic);
        let current = editor_request_source(&state);
        assert_eq!(delete.source.document, current.document);
        assert_eq!(delete.source.content_version, current.content_version);
        assert_eq!(delete.source.topology_version, current.topology_version);
        assert_ne!(delete.source.occurrence, current.occurrence);
        apply_editor_request(&mut state, delete);
        apply_editor_request(&mut state, focus);
        apply_editor_request(&mut state, pointer);
        assert_eq!(state.schematic.document().components, components());
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(22)
        );
        assert!(!state.schematic.can_undo());

        let delete = capture(&state, EditorAction::DeleteSelection);
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(33);
        apply_editor_request(&mut state, delete);
        assert_eq!(state.schematic.document().components, components());
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(33)
        );
        assert!(!state.schematic.can_undo());

        let delete = capture(&state, EditorAction::DeleteSelection);
        state.schematic.session.read_only = true;
        apply_editor_request(&mut state, delete);
        assert_eq!(state.schematic.document().components, components());
        assert!(!state.schematic.can_undo());
        state.schematic.session.read_only = false;
        let delete = capture(&state, EditorAction::DeleteSelection);
        apply_editor_request(&mut state, delete);
        assert_eq!(state.schematic.document().components.len(), 2);
        assert_eq!(state.schematic.undo_description(), Some("delete selection"));
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().components, components());
    }
}
