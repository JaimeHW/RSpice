//! What a reader without a mouse can reach in the Verilog-A editor, through
//! whole frames.

use egui::{Key, Modifiers};

use super::language_tools::{NONE, caret_after, op_amp, source};
use super::*;

const COPY: &str = "Copy";
const SHIFT: Modifiers = Modifiers {
    shift: true,
    ..Modifiers::NONE
};

fn open_menu_top(user: &mut User, after: &str) -> f32 {
    caret_after(user, after);
    assert!(user.find(COPY).is_none(), "no menu is open yet");
    user.key(Key::F10, SHIFT);
    user.settle();
    let top = user.expect(COPY).top();
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(user.find(COPY).is_none(), "Escape closes it");
    top
}

/// With no pointer to say where, the menu opens at the caret.
#[test]
fn shift_f10_opens_the_menu_at_the_caret() {
    let mut user = op_amp();
    let high = open_menu_top(&mut user, "module opamp_bhv");
    let low = open_menu_top(&mut user, "vtarget = vslew;");
    assert!(
        low > high + 17.0,
        "the menu follows the caret: {high} {low}"
    );
}

#[test]
fn the_keys_are_the_menus_while_it_is_open() {
    let mut user = op_amp();
    caret_after(&mut user, "module opamp_bhv");
    let caret = user.caret();
    let text = source(&user);
    user.key(Key::F10, SHIFT);
    user.settle();

    user.key(Key::ArrowDown, NONE);
    user.settle();
    assert_eq!(user.caret(), caret, "the arrow stepped the menu");
    assert!(user.focused.is_some());

    user.key(Key::Escape, NONE);
    user.settle();
    user.key(Key::ArrowDown, NONE);
    user.settle();
    assert_ne!(user.caret(), caret, "the keyboard came back to the text");
    assert_eq!(source(&user), text);
}

#[cfg(windows)]
#[test]
fn a_held_key_is_one_press() {
    use crate::workbench::documents::virtual_text_editor::press_begins;
    let ctx = egui::Context::default();
    let id = egui::Id::new("key");
    let presses = [false, true, true, false, true].map(|down| press_begins(&ctx, id, down));
    assert_eq!(presses, [false, true, false, false, true]);
}
