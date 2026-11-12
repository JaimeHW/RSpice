//! The editor's context menu: the keys that raise it, and whether it may
//! show in a frame, which a page reads before it draws the editor.

use egui::{Context, Event, Id, Key, Modifiers, PointerButton, Popup};

/// The context menu over the editor `editor_id`. The editor interacts under
/// `editor_id`, and egui keys a context menu by the response it opens over,
/// so this is the id egui gives it there.
pub(super) fn context_menu_id(editor_id: Id) -> Id {
    editor_id.with("popup")
}

/// Whether the context menu over the editor `editor_id` is open from an
/// earlier frame. While it is, the keys are the menu's: Escape closes it and
/// the arrows step its rows, so nothing drawn before it may take them.
pub(crate) fn context_menu_open(ctx: &Context, editor_id: Id) -> bool {
    Popup::is_id_open(ctx, context_menu_id(editor_id))
}

/// Whether the context menu over the editor `editor_id` may show in this
/// frame, read before the editor draws: it is open from an earlier frame, or
/// this frame carries what opens it (a secondary click, a touch held past a
/// click's longest press, Shift+F10 or the Menu key). It may answer yes in a
/// frame the menu does not open; it never answers no in one it does.
pub(crate) fn context_menu_may_show(ctx: &Context, editor_id: Id) -> bool {
    let longest_click = ctx.options(|options| options.input_options.max_click_duration);
    context_menu_open(ctx, editor_id)
        || menu_key_down(ctx)
        || ctx.input(|input| {
            let pointer = &input.pointer;
            let held = input.any_touches()
                && pointer.primary_down()
                && pointer
                    .press_start_time()
                    .is_some_and(|start| input.time - start > longest_click);
            let shift_f10 = input.events.iter().any(|event| {
                matches!(
                    event,
                    Event::Key { key: Key::F10, pressed: true, modifiers, .. }
                        if modifiers.matches_logically(Modifiers::SHIFT)
                )
            });
            pointer.button_clicked(PointerButton::Secondary) || held || shift_f10
        })
}

/// Shift+F10 and the Menu key open the context menu at the caret.
pub(super) fn take_menu_keys(ctx: &Context, editor_id: Id) -> bool {
    let shift_f10 = ctx.input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10));
    shift_f10 | menu_key_went_down(ctx, editor_id)
}

/// Whether the Menu key is down while the window has focus. egui has no
/// name for the Menu key, so the keyboard's own state is read. A press wakes
/// the frame this runs in like any other key does.
#[cfg(windows)]
fn menu_key_down(ctx: &Context) -> bool {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_APPS};
    // SAFETY: takes a virtual-key code and returns a value; nothing is borrowed.
    let down = unsafe { GetAsyncKeyState(i32::from(VK_APPS)) } < 0;
    down && ctx.input(|input| input.focused)
}

#[cfg(not(windows))]
const fn menu_key_down(_ctx: &Context) -> bool {
    false
}

/// Whether the Menu key has gone down since the editor `editor_id` last
/// read it.
#[cfg(windows)]
fn menu_key_went_down(ctx: &Context, editor_id: Id) -> bool {
    super::press_begins(ctx, editor_id.with("menu key"), menu_key_down(ctx))
}

#[cfg(not(windows))]
const fn menu_key_went_down(_ctx: &Context, _editor_id: Id) -> bool {
    false
}
