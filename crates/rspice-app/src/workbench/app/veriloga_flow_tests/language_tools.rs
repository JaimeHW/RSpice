//! The Verilog-A editor's language tools at the keyboard, through whole
//! application frames: the completion list, signature help, the name card,
//! what a screen reader is told, and Find references.

use egui::{Event, Key, Modifiers};

use super::*;
use crate::workbench::documents::code_workspace;
use crate::workbench::documents::text_editor_commands::queue_caret_char_index;
use crate::workbench::state::ConsolePage;

pub(super) const EDITOR: &str = "Verilog-A/AMS source editor";
/// As the keyboard of a Windows or Linux desktop reports the key.
pub(super) const CTRL: Modifiers = Modifiers {
    ctrl: true,
    command: true,
    ..Modifiers::NONE
};
pub(super) const CTRL_SHIFT: Modifiers = Modifiers {
    shift: true,
    ..CTRL
};
pub(super) const NONE: Modifiers = Modifiers::NONE;

pub(super) fn op_amp() -> User {
    let (mut user, _, _) = new_device("opamp_bhv", "inp:in inn:in out:out vdd vss");
    super::super::veriloga_interface_tests::edit_source(&mut user.app, OPAMP);
    user.settle();
    user
}

pub(super) fn source(user: &User) -> String {
    let selected = code_workspace::selected_veriloga_source(&user.app).expect("the open view");
    let path = selected.document().logical_path().to_owned();
    selected
        .bundle()
        .file_content(&path)
        .expect("the open document")
        .to_owned()
}

/// The editor takes the keyboard, with the caret just after `text`.
pub(super) fn caret_after(user: &mut User, text: &str) {
    let source = source(user);
    let end = source.find(text).expect("the text") + text.len();
    caret_at(user, source[..end].chars().count());
}

fn caret_at(user: &mut User, index: usize) {
    let selected = code_workspace::selected_veriloga_source(&user.app).expect("the open view");
    let id = code_workspace::source_editor_id(
        crate::state::ProjectSourceLanguage::VerilogA,
        selected.bundle().id(),
        selected.document().logical_path(),
    );
    user.click(EDITOR);
    queue_caret_char_index(&user.ctx, id, index);
    user.settle();
}

/// A new line under `vtarget = vslew;`, with the keyboard on it.
fn on_a_new_line(user: &mut User) {
    caret_after(user, "vtarget = vslew;");
    user.key(Key::Enter, NONE);
}

pub(super) fn type_text(user: &mut User, text: &str) {
    user.frame(vec![Event::Text(text.to_owned())]);
    user.settle();
}

/// The list's rows, top to bottom.
fn rows(user: &User) -> Vec<String> {
    let mut rows = user
        .named
        .iter()
        .filter(|(name, _)| name.contains(" completion: "))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.1.top().total_cmp(&right.1.top()));
    rows.into_iter().map(|(name, _)| name.clone()).collect()
}

fn description(user: &User) -> String {
    user.described
        .iter()
        .find(|(label, _)| label == EDITOR)
        .map(|(_, description)| description.clone())
        .expect("the editor describes itself")
}

#[test]
fn a_dollar_lists_system_functions_with_signatures_and_tab_takes_the_one_chosen() {
    let mut user = op_amp();
    on_a_new_line(&mut user);
    type_text(&mut user, "$");
    let listed = rows(&user);
    assert_eq!(listed.len(), 10, "as many as the list shows: {listed:?}");
    assert!(
        listed.iter().all(|row| row.contains(": $")),
        "only system functions: {listed:?}"
    );
    assert!(
        listed.iter().any(|row| row.contains('(')),
        "with their signatures: {listed:?}"
    );

    type_text(&mut user, "temper");
    let listed = rows(&user);
    assert!(
        listed[0].contains("$temperature"),
        "narrowed to what was typed: {listed:?}"
    );
    assert!(
        description(&user).contains("$temperature"),
        "a screen reader hears the choice: {}",
        description(&user)
    );
    let typed = source(&user);
    user.key(Key::Tab, NONE);
    user.settle();
    assert!(rows(&user).is_empty(), "taking a row closes the list");
    let after = source(&user);
    assert!(after.contains("\n    $temperature\n"), "{after}");

    // One undo takes back the whole completion, not a character of it.
    user.key(Key::Z, CTRL);
    user.settle();
    assert_eq!(source(&user), typed);
}

#[test]
fn ctrl_space_offers_the_modules_names_and_enter_takes_only_a_list_asked_for() {
    let mut user = op_amp();
    on_a_new_line(&mut user);

    // Two characters open the list by themselves, and Enter still ends the
    // line.
    type_text(&mut user, "vt");
    assert!(rows(&user)[0].contains("vtarget"), "{:?}", rows(&user));
    let lines = source(&user).lines().count();
    user.key(Key::Enter, NONE);
    user.settle();
    assert_eq!(source(&user).lines().count(), lines + 1);
    assert!(rows(&user).is_empty());

    // Asked for with Ctrl+Space, Enter takes the row.
    type_text(&mut user, "ro");
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(rows(&user).is_empty(), "Esc closes the list");
    user.key(Key::Space, CTRL);
    user.settle();
    assert!(rows(&user)[0].contains("rout"), "{:?}", rows(&user));
    user.key(Key::Enter, NONE);
    user.settle();
    assert!(source(&user).contains("    rout\n"), "{}", source(&user));
}

#[test]
fn escape_keeps_the_list_closed_until_the_next_edit_opens_it() {
    let mut user = op_amp();
    on_a_new_line(&mut user);
    type_text(&mut user, "vt");
    assert_eq!(rows(&user).len(), 1, "{:?}", rows(&user));
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(rows(&user).is_empty());
    user.settle();
    assert!(rows(&user).is_empty(), "nothing typed, nothing opened");
    type_text(&mut user, "a");
    assert_eq!(rows(&user).len(), 1, "typing on opens it again");
}

#[test]
fn arrows_move_the_choice_in_a_list_that_is_open_and_the_caret_otherwise() {
    let mut user = op_amp();
    on_a_new_line(&mut user);
    let line = user.caret().0;
    type_text(&mut user, "$");
    let first = description(&user);
    user.key(Key::ArrowDown, NONE);
    user.settle();
    assert_ne!(description(&user), first, "the choice moved");
    assert_eq!(user.caret().0, line, "the caret stayed");
    user.key(Key::Escape, NONE);
    user.key(Key::ArrowDown, NONE);
    user.settle();
    assert_eq!(
        user.caret().0,
        line + 1,
        "the list closed, the arrow is the caret's"
    );
}

#[test]
fn an_open_bracket_shows_the_parameter_list_and_marks_the_argument_being_written() {
    let mut user = op_amp();
    on_a_new_line(&mut user);
    type_text(&mut user, "tau = ddt(");
    assert!(user.find("expr").is_some(), "the argument is drawn");
    let said = description(&user);
    assert!(
        said.contains("Argument 1 of ddt(expr [, abstol]): expr."),
        "{said}"
    );
    type_text(&mut user, "vin, ");
    assert!(user.find("abstol").is_some(), "the mark moved on");
    let said = description(&user);
    assert!(
        said.contains("Argument 2 of ddt(expr [, abstol]): abstol."),
        "{said}"
    );
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(user.find("abstol").is_none(), "Esc puts it away");
}

/// What the pointer's hover says of a name, a keyboard reader gets from
/// Ctrl+Shift+Space and a screen reader from the editor's description.
#[test]
fn hover_the_name_card_and_the_description_each_say_what_a_name_is() {
    let mut user = op_amp();
    caret_after(&mut user, "tau * ddt");
    let said = description(&user);
    assert!(said.contains("ddt(expr [, abstol])"), "{said}");
    assert!(said.contains("Time derivative"), "{said}");

    assert!(user.find("ddt(expr [, abstol])").is_none());
    user.key(Key::Space, CTRL_SHIFT);
    user.settle();
    assert!(user.find("ddt(expr [, abstol])").is_some(), "the card");
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(user.find("ddt(expr [, abstol])").is_none());

    // The parameter's own description, from its attribute.
    caret_after(&mut user, "ddt(V(mid));\n    vslew = slew(V(mid), s");
    let said = description(&user);
    assert!(said.contains("Slew rate"), "{said}");

    // The pointer, over the `ddt` of the line the window shows.
    caret_after(&mut user, "tau * d");
    let line = user
        .named
        .iter()
        .find(|(name, _)| name.contains("tau * ddt"))
        .cloned()
        .expect("the caret's line is in the accessible text");
    let (text, rect) = line;
    let text = text.trim_end();
    let over = rect.left()
        + rect.width() * (text.find("ddt").expect("ddt") + 1) as f32 / text.chars().count() as f32;
    // A tooltip waits for a pointer that has moved since the last click and
    // then rested, so: time for the click to pass, one move, and the wait.
    for _ in 0..12 {
        user.frame(Vec::new());
    }
    user.frame(vec![Event::PointerMoved(egui::pos2(over, rect.center().y))]);
    for _ in 0..70 {
        user.frame(Vec::new());
    }
    assert!(user.find("ddt(expr [, abstol])").is_some(), "the hover");

    // Nowhere to explain: the status bar says so.
    caret_after(&mut user, "single pole");
    user.key(Key::Space, CTRL_SHIFT);
    user.settle();
    assert_eq!(user.prompt(), "Nothing to explain at the caret.");
}

/// Find references puts every use in a table the keyboard reads, and Enter
/// opens the row at its column; none of it passes through the Console.
#[test]
fn find_references_is_a_list_the_keyboard_walks_and_enter_opens_at_the_column() {
    let mut user = op_amp();
    caret_after(&mut user, "parameter real gb");
    let logged = user.app.state.model.log_buffer.entries().count();

    crate::workbench::app::navigate_source_bundle(
        &mut user.app.code_workspace_parts(),
        crate::state::ProjectSourceLanguage::VerilogA,
        CodeCommand::FindReferences,
    )
    .expect("the caret is on a name");
    user.settle();

    assert_eq!(
        user.app.state.workbench.console_page,
        ConsolePage::References
    );
    assert_eq!(user.prompt(), "2 references to gbw in 1 file.");
    assert_eq!(user.app.state.model.log_buffer.entries().count(), logged);
    let list = user
        .app
        .state
        .workbench
        .source_references
        .as_ref()
        .expect("the list");
    let places = list
        .rows
        .iter()
        .map(|row| (row.line, row.column))
        .collect::<Vec<_>>();
    assert_eq!(places, [(13, 66), (21, 29)]);
    assert!(
        user.named
            .iter()
            .any(|(name, _)| name.contains("21") && name.contains("tau = a0")),
        "a row names its place and its text"
    );

    // The table took the keyboard: Down, then Enter.
    user.key(Key::ArrowDown, NONE);
    user.settle();
    assert_eq!(
        user.app
            .state
            .workbench
            .source_references
            .as_ref()
            .map(|list| list.selected),
        Some(1)
    );
    user.key(Key::Enter, NONE);
    user.settle();
    user.settle();
    assert_eq!(user.caret(), (21, 29), "opened at the row's column");

    // The tab goes when the list does.
    assert!(user.find("References").is_some());
    user.app.state.workbench.source_references = None;
    user.settle();
    assert!(user.find("References").is_none());
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Console);
}

/// Whether the last frame drew the editor's right-click menu, by its Copy
/// row.
fn menu_shown(user: &User) -> bool {
    user.find("Copy").is_some()
}

/// The right-click menu, raised at the caret by Shift+F10 while the list is
/// up, takes its own keys: the arrows step its rows, leaving the caret where
/// it is, and Escape closes it, giving the editor the keyboard back. With
/// the menu closed, the list typed up again takes the arrows and Escape.
#[test]
fn the_open_right_click_menu_takes_its_keys_and_the_list_takes_them_once_it_has_closed() {
    let mut user = op_amp();
    on_a_new_line(&mut user);
    let line = user.caret().0;
    type_text(&mut user, "$");
    user.key(Key::F10, Modifiers::SHIFT);
    user.settle();
    assert!(menu_shown(&user), "Shift+F10 raises the menu");
    assert!(rows(&user).is_empty(), "the menu closes the list");
    let mut stepped = Vec::new();
    for key in [Key::ArrowDown, Key::ArrowDown, Key::ArrowUp] {
        user.key(key, NONE);
        user.settle();
        stepped.push(user.focused.clone());
    }
    assert_eq!(
        stepped,
        ["Go to definition", "Find references", "Go to definition"].map(|row| Some(row.to_owned())),
        "the arrows step the menu's rows"
    );
    assert_eq!(user.caret().0, line, "the caret stays");
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(!menu_shown(&user), "Escape closes the menu");
    assert_eq!(
        user.focused.as_deref(),
        Some(EDITOR),
        "the editor has the keyboard back"
    );

    // The menu closed the list until the next edit; typing brings it up
    // again.
    user.key(Key::Backspace, NONE);
    type_text(&mut user, "$");
    let chosen = description(&user);
    assert!(!rows(&user).is_empty(), "the list is up");
    user.key(Key::ArrowDown, NONE);
    user.settle();
    assert_ne!(description(&user), chosen, "the list takes the arrow");
    assert_eq!(user.caret().0, line, "and the caret stays");
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(rows(&user).is_empty(), "the list takes Escape");
    assert_eq!(
        user.focused.as_deref(),
        Some(EDITOR),
        "the editor keeps the keyboard"
    );
}

/// Raising the right-click menu closes the list and the parameter card, so
/// no two popups overlap, and the list stays closed once the menu has gone,
/// until the next edit.
#[test]
fn raising_the_right_click_menu_closes_the_list_and_the_parameter_card() {
    let mut user = op_amp();
    on_a_new_line(&mut user);
    type_text(&mut user, "$");
    assert!(!rows(&user).is_empty(), "the list is up");
    user.key(Key::F10, Modifiers::SHIFT);
    user.settle();
    assert!(menu_shown(&user), "Shift+F10 raises the menu");
    assert!(rows(&user).is_empty(), "the menu closes the list");
    user.key(Key::Escape, NONE);
    user.settle();
    assert!(!menu_shown(&user), "Escape closes the menu");
    assert!(
        rows(&user).is_empty(),
        "the list stays closed once the menu has gone"
    );

    user.key(Key::Backspace, NONE);
    type_text(&mut user, "tau = ddt(");
    assert!(user.find("expr").is_some(), "the parameter card is up");
    user.key(Key::F10, Modifiers::SHIFT);
    user.settle();
    assert!(menu_shown(&user), "Shift+F10 raises the menu");
    assert!(user.find("expr").is_none(), "the menu closes the card");
}
