//! What a reader without a mouse can reach in the Verilog-A editor, and what
//! a reader with a finger can hit, through whole frames.

use egui::{Key, Modifiers};

use super::language_tools::{NONE, caret_after, op_amp, source};
use super::*;
use crate::workbench::MessageId;
use crate::workbench::documents::code_workspace;

const COPY: &str = "Copy";
const SHIFT: Modifiers = Modifiers {
    shift: true,
    ..Modifiers::NONE
};

fn file_row(user: &User) -> String {
    let selected = code_workspace::selected_veriloga_source(&user.app).expect("the open view");
    selected.document().logical_path().to_owned()
}

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

fn file_actions(user: &User) -> Vec<egui::Rect> {
    let messages = user.app.state.workbench.messages();
    [
        MessageId::CodeSourceNewFile,
        MessageId::CodeSourceRename,
        MessageId::CodeSourceDelete,
        MessageId::CodeSourceAddFiles,
    ]
    .map(|id| user.expect(&messages.text(id)))
    .to_vec()
}

#[test]
fn the_rails_icons_are_at_least_24_points_square() {
    let user = op_amp();
    for action in file_actions(&user) {
        assert!(
            action.width() >= 24.0 && action.height() >= 24.0,
            "{action:?}"
        );
    }
}

#[test]
fn the_rails_icons_grow_for_a_finger_and_leave_the_title_alone() {
    let mut user = op_amp();
    user.app.state.workbench.coarse_pointer = true;
    user.settle();
    let target = user.app.state.model.preferences.minimum_touch_target();
    let actions = file_actions(&user);
    for action in &actions {
        assert!(
            action.width() >= target && action.height() >= target,
            "{action:?} against {target}"
        );
    }
    let file = user.expect(&file_row(&user));
    assert!(
        actions
            .iter()
            .all(|action| action.bottom() <= file.top() + 0.5),
        "the rows start below the icons: {file:?} {actions:?}"
    );
}

/// The rail is one list to the keyboard, as the Design rail is: the arrows
/// walk its rows.
#[test]
fn the_rails_rows_answer_the_arrow_keys() {
    let mut user = op_amp();
    let file = file_row(&user);
    user.focus_on(&file);
    assert_eq!(user.focused.as_deref(), Some(file.as_str()));

    user.key(Key::ArrowDown, NONE);
    user.settle();
    let next = user.focused.clone().expect("a row holds the keyboard");
    assert_ne!(next, file);

    user.key(Key::ArrowUp, NONE);
    user.settle();
    assert_eq!(user.focused.as_deref(), Some(file.as_str()));

    user.key(Key::End, NONE);
    user.settle();
    let last = user.focused.clone().expect("a row holds the keyboard");
    assert_ne!(last, file, "End reaches the end of the rail");
    user.key(Key::Home, NONE);
    user.settle();
    assert_eq!(
        user.focused.as_deref(),
        Some(file.as_str()),
        "and Home the start"
    );
}
