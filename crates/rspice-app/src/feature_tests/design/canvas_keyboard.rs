//! The canvas's keyboard navigation run through the whole application: the shell's modal
//! and popup owners hold the keys, and Backspace on a focused select canvas deletes at
//! once in one undo entry.

use crate::schematic::view::keyboard_navigation::tests::{components, key_input};
use crate::schematic::view::keyboard_navigation::*;
use crate::schematic::view::schematic_symbol_context;
use crate::workbench::app_state::AppState;
use egui::{Context, Id, Key, Modifiers, Popup, Sense};

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
            let handled = handle_keyboard_object_navigation(
                &response,
                &mut crate::schematic::view::canvas_parts(state),
                &symbol_context,
            );
            let key_still_available = ui
                .ctx()
                .input_mut(|input| input.consume_key(modifiers, key));
            outcome = (handled, key_still_available);
        });
    });
    outcome
}

#[test]
fn modal_and_context_popup_owners_block_navigation() {
    let ctx = Context::default();
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().components = components();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(11);
    state.shell.about = true;

    let (handled, key_still_available) =
        run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
    assert!(!handled);
    assert!(key_still_available);
    assert_eq!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(11)
    );

    state.shell.about = false;
    Popup::open_id(&ctx, Id::new("test-context-owner"));
    let (handled, key_still_available) =
        run_navigation_frame(&ctx, Key::ArrowRight, Modifiers::NONE, &mut state, true);
    assert!(!handled);
    assert!(key_still_available);
    assert_eq!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(11)
    );
    Popup::close_all(&ctx);
}

#[test]
fn focused_select_canvas_backspace_deletes_immediately_in_one_undo_entry() {
    let ctx = Context::default();
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().components = components();
    state.sync_active_schematic_to_workspace();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(22);
    state.model.schematic.init_undo_history();

    let (handled, key_still_available) =
        run_navigation_frame(&ctx, Key::Backspace, Modifiers::NONE, &mut state, true);

    assert!(handled);
    assert!(!key_still_available);
    assert_eq!(state.model.schematic.document().components.len(), 2);
    assert!(!state.dialog_modal_open());
    assert_eq!(
        state.model.schematic.undo_description(),
        Some("delete selection")
    );
    assert!(state.model.schematic.undo());
    assert_eq!(state.model.schematic.document().components.len(), 3);
    assert!(!state.model.schematic.can_undo());
}
