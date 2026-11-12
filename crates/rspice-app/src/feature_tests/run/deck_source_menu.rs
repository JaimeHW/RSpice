//! The deck editor's right-click menu, opened and chosen from in whole
//! frames of the application.
//!
//! The menu's rows are read from the page's context, which holds the
//! menu's answers from the frame that opens it until it closes, and the row
//! chosen runs once the page has drawn: what it opens or moves to is drawn
//! on the next frame, which the frame of the choice asks for at once.

use egui::accesskit::Role;
use egui::{Key, Modifiers, PointerButton, Pos2};

use super::engine_recovery_path::author_deck;
use crate::feature_tests::replay::{Replay, controls, only};
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::netlist_document::language::show_signature_help_at_cursor;
use crate::workbench::documents::netlist_document::{
    OwnedNetlistReplacement, editor_id_in, replace_owned_sources_atomically,
};
use crate::workbench::documents::virtual_text_editor::context_menu_may_show;
use crate::workbench::features;
use crate::workbench::framework::feature::{Cx, Feature};

/// A deck whose last instance calls the subcircuit it defines, under a name
/// long enough that the middle of the call's line falls inside it.
const DECK: &str = "* source menu check\n.subckt longsubcircuitname a b\nR1 a b 1k\n.ends\nX1 a b longsubcircuitname\n.end";
const CALL: &str = "X1 a b longsubcircuitname";
const NAME: &str = "longsubcircuitname";

/// The deck page showing `DECK`, put in place whole: typed, its lines
/// would be indented under the subcircuit.
fn deck_page() -> Replay {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let typed = replay
        .app
        .state
        .model
        .workspace
        .content
        .netlist_source
        .clone()
        .expect("the project owns the deck");
    replace_owned_sources_atomically(
        &mut replay.app.state,
        vec![OwnedNetlistReplacement::root(&typed, DECK.to_owned(), 1)],
    )
    .expect("the project owns the deck");
    replay.frames(Vec::new());
    assert_eq!(
        replay.app.state.model.simulation.source.netlist_content, DECK,
        "the editor shows the deck put in place"
    );
    replay
}

/// The line of the deck the caret is on, from zero.
fn caret_line(replay: &Replay) -> usize {
    let caret = replay.app.state.code.netlist.cursor_char_index;
    DECK.chars().take(caret).filter(|&c| c == '\n').count()
}

/// Whether `output` asks for the next frame at once.
fn repaints_at_once(output: &egui::FullOutput) -> bool {
    output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .is_some_and(|viewport| viewport.repaint_delay.is_zero())
}

/// Where the editor publishes the call's line.
fn the_call(replay: &mut Replay) -> Pos2 {
    replay
        .frames(Vec::new())
        .into_iter()
        .find(|control| control.role == Role::TextRun && control.value.trim_end() == CALL)
        .expect("the editor publishes the call's line")
        .center
}

/// `button` pressed or released at `at`.
fn button(at: Pos2, button: PointerButton, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

/// A click on the middle of the call's line, which focuses the editor and
/// closes the completion list the typing of the deck left open over it;
/// where it clicked.
fn click_the_call(replay: &mut Replay) -> Pos2 {
    let line = the_call(replay);
    replay.frame(vec![egui::Event::PointerMoved(line)]);
    replay.frame(vec![button(line, PointerButton::Primary, true)]);
    replay.frames(vec![button(line, PointerButton::Primary, false)]);
    line
}

/// A right-click on the middle of the call's line, after a click there;
/// what the frame of its release put out.
fn right_click_the_call(replay: &mut Replay) -> egui::FullOutput {
    let line = click_the_call(replay);
    replay.frame(vec![button(line, PointerButton::Secondary, true)]);
    replay.frame(vec![button(line, PointerButton::Secondary, false)])
}

/// The menu opened by a right-click on the middle of the call's line, which
/// puts the caret in the subcircuit's name there.
fn open_menu_on_the_call(replay: &mut Replay) {
    right_click_the_call(replay);
    replay.frame(Vec::new());
    replay.frame(Vec::new());
    // The deck is ASCII, so its byte offsets are its character offsets.
    let name = DECK.find(CALL).expect("the deck holds the call") + CALL.len() - NAME.len();
    let caret = replay.app.state.code.netlist.cursor_char_index;
    assert!(
        (name..=name + NAME.len()).contains(&caret),
        "the right-click puts the caret in the subcircuit's name, not at {caret}"
    );
}

/// The open menu's row `label` pressed and released; what the frame of the
/// release put out.
fn choose(replay: &mut Replay, label: &str) -> egui::FullOutput {
    let row = only(&replay.frames(Vec::new()), label, Role::MenuItem).center;
    let press = |pressed| egui::Event::PointerButton {
        pos: row,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(row)]);
    replay.frame(vec![press(true)]);
    replay.frame(vec![press(false)])
}

/// Whether the frame that put out `output` drew the source editor's menu.
fn draws_the_menu(output: &egui::FullOutput) -> bool {
    let tree = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("a frame with AccessKit on publishes its tree");
    tree.nodes
        .iter()
        .any(|(_, node)| node.role() == Role::Menu && node.label() == Some("Source editor menu"))
}

/// Whether the deck editor's menu may show, as the shell reads it before it
/// lends the deck page its context.
fn menu_may_show(replay: &Replay) -> bool {
    let state = &replay.app.state;
    context_menu_may_show(
        replay.ctx(),
        editor_id_in(&state.model, &state.code.netlist),
    )
}

/// The deck page's context as the shell lends it: with the menu's answers
/// while the menu may show.
fn page_context(replay: &mut Replay) -> Cx<'_, CodeFeature> {
    let ctx = replay.ctx().clone();
    features::cx_with_menu_answers(&mut replay.app, |app| {
        context_menu_may_show(
            &ctx,
            editor_id_in(&app.state.model, &app.state.code.netlist),
        )
    })
}

/// The page's context holds an answer for every row of the menu, each the
/// one the dispatcher gives.
fn holds_the_menus_answers(replay: &mut Replay) {
    let expected: Vec<_> = CodeFeature::MENU_AVAILABILITY
        .iter()
        .map(|key| (*key, replay.app.command_availability(*key)))
        .collect();
    let cx = page_context(replay);
    assert_eq!(
        cx.foreign_answers_taken(),
        CodeFeature::FOREIGN_AVAILABILITY.len() + expected.len(),
        "the page's context holds the menu's answers"
    );
    for (key, answer) in expected {
        assert_eq!(cx.foreign_availability(key), answer, "{key:?}");
    }
}

/// The page's context holds the menu's answers from the frame of the
/// right-click that opens it, and through a frame with no input while it
/// stays open; once a click on another line has closed it, the next frame's
/// holds only the three the page reads every frame. No frame reads an
/// answer its context was lent without.
#[test]
fn the_page_holds_the_menus_answers_while_the_menu_is_open_and_no_longer() {
    let mut replay = deck_page();
    let call = the_call(&mut replay);
    assert!(
        draws_the_menu(&right_click_the_call(&mut replay)),
        "the frame of the right-click draws the menu"
    );
    assert!(
        draws_the_menu(&replay.frame(Vec::new())),
        "a frame with no input still draws it"
    );
    assert!(menu_may_show(&replay), "the shell reads the menu as open");
    holds_the_menus_answers(&mut replay);
    // Three lines above the call the editor is clear of the menu, which
    // opens below and right of where the call was clicked.
    let above = call - egui::vec2(0.0, 48.0);
    replay.frame(vec![egui::Event::PointerMoved(above)]);
    replay.frame(vec![button(above, PointerButton::Primary, true)]);
    replay.frame(vec![button(above, PointerButton::Primary, false)]);
    assert!(
        !draws_the_menu(&replay.frame(Vec::new())),
        "a click on another line closes the menu"
    );
    assert!(
        !menu_may_show(&replay),
        "the shell reads the menu as closed"
    );
    assert_eq!(
        page_context(&mut replay).foreign_answers_taken(),
        CodeFeature::FOREIGN_AVAILABILITY.len(),
        "the page's context holds the three answers it reads every frame"
    );
}

/// A touch held still on the call's line in a window a phone's width opens
/// the menu once it is no longer a tap, and the frame it opens in holds its
/// answers; the menu stays open, still holding them, once the finger lifts.
#[test]
fn a_held_touch_opens_the_menu_with_its_answers_at_a_phones_width() {
    let mut replay = deck_page().sized(egui::vec2(390.0, 844.0));
    let line = the_call(&mut replay);
    let finger = |phase, pressed| {
        vec![
            egui::Event::Touch {
                device_id: egui::TouchDeviceId(1),
                id: egui::TouchId(1),
                phase,
                pos: line,
                force: None,
            },
            egui::Event::PointerMoved(line),
            button(line, PointerButton::Primary, pressed),
        ]
    };
    // A tap first closes the completion list the typing of the deck left
    // open over the editor.
    replay.frame(finger(egui::TouchPhase::Start, true));
    replay.frames(finger(egui::TouchPhase::End, false));
    replay.frame(finger(egui::TouchPhase::Start, true));
    let opened = (0..12).find(|_| draws_the_menu(&replay.frame(Vec::new())));
    assert!(
        opened.is_some_and(|frame| frame > 0),
        "the held touch opens the menu once it is no longer a tap: {opened:?}"
    );
    replay.frame(finger(egui::TouchPhase::End, false));
    assert!(
        draws_the_menu(&replay.frame(Vec::new())),
        "the menu stays open once the finger lifts"
    );
    assert!(menu_may_show(&replay), "the shell reads the menu as open");
    holds_the_menus_answers(&mut replay);
}

/// Shift+F10 in the deck editor draws the menu in the frame of the key, so
/// that frame's context holds the answers its rows read, as does the next.
#[test]
fn shift_f10_draws_the_menu_in_the_frame_of_the_key() {
    let mut replay = deck_page();
    click_the_call(&mut replay);
    // The keyboard is put on the editor as a screen reader puts it there.
    let editor = editor_id_in(&replay.app.state.model, &replay.app.state.code.netlist);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frame(Vec::new());
    assert!(
        replay.ctx().memory(|memory| memory.has_focus(editor)),
        "the editor has the keyboard"
    );
    let key = |pressed| egui::Event::Key {
        key: Key::F10,
        physical_key: Some(Key::F10),
        pressed,
        repeat: false,
        modifiers: Modifiers::SHIFT,
    };
    assert!(
        draws_the_menu(&replay.frame(vec![key(true)])),
        "the frame of Shift+F10 draws the menu"
    );
    replay.frame(vec![key(false)]);
    assert!(menu_may_show(&replay), "the shell reads the menu as open");
    holds_the_menus_answers(&mut replay);
}

/// `key` pressed and released in one frame.
fn press(replay: &mut Replay, key: Key) {
    let event = |pressed| egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed,
        repeat: false,
        modifiers: Modifiers::NONE,
    };
    replay.frame(vec![event(true), event(false)]);
}

/// The menu row the keyboard is on in the tree `output` published.
fn focused_row(output: &egui::FullOutput) -> Option<String> {
    let tree = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("a frame with AccessKit on publishes its tree");
    tree.nodes
        .iter()
        .find(|(id, node)| *id == tree.focus && node.role() == Role::MenuItem)
        .and_then(|(_, node)| node.label().map(str::to_owned))
}

/// Whether the deck editor has the keyboard.
fn editor_has_the_keyboard(replay: &Replay) -> bool {
    let state = &replay.app.state;
    let editor = editor_id_in(&state.model, &state.code.netlist);
    replay.ctx().memory(|memory| memory.has_focus(editor))
}

/// While the menu is open the arrows step its rows, and move neither the
/// completion list's choice nor the caret; Escape closes the menu and gives
/// the editor the keyboard back, and no key reaches the deck.
#[test]
fn the_arrows_step_the_open_menus_rows_and_escape_closes_it() {
    let mut replay = deck_page();
    open_menu_on_the_call(&mut replay);
    let netlist = &replay.app.state.code.netlist;
    assert!(
        !netlist.completion_open,
        "the right-click closes the completion list"
    );
    let (caret, listed) = (netlist.cursor_char_index, netlist.completion_index);
    let mut stepped = Vec::new();
    for key in [Key::ArrowDown, Key::ArrowDown, Key::ArrowUp] {
        press(&mut replay, key);
        let output = replay.frame(Vec::new());
        assert!(draws_the_menu(&output), "{key:?} keeps the menu open");
        stepped.push(focused_row(&output));
    }
    assert_eq!(
        stepped,
        ["Go to definition", "Find references", "Go to definition"].map(|row| Some(row.to_owned())),
        "the arrows step the menu's rows"
    );
    let netlist = &replay.app.state.code.netlist;
    assert_eq!(
        (netlist.cursor_char_index, netlist.completion_index),
        (caret, listed),
        "neither the caret nor the completion list moves"
    );
    press(&mut replay, Key::Escape);
    assert!(
        !draws_the_menu(&replay.frame(Vec::new())),
        "Escape closes the menu"
    );
    assert!(
        editor_has_the_keyboard(&replay),
        "the editor has the keyboard back"
    );
    assert_eq!(
        replay.app.state.model.simulation.source.netlist_content, DECK,
        "no key reached the deck"
    );
}

/// Enter on the row the arrows stepped to chooses it: Go to definition
/// puts the caret on the subcircuit's definition by the next frame.
#[test]
fn enter_chooses_the_row_the_arrows_stepped_to() {
    let mut replay = deck_page();
    open_menu_on_the_call(&mut replay);
    press(&mut replay, Key::ArrowDown);
    assert_eq!(
        focused_row(&replay.frame(Vec::new())).as_deref(),
        Some("Go to definition"),
        "the arrow steps to the first row"
    );
    press(&mut replay, Key::Enter);
    assert!(
        !draws_the_menu(&replay.frame(Vec::new())),
        "the choice closes the menu"
    );
    assert_eq!(
        caret_line(&replay),
        1,
        "the caret is on the subcircuit's definition"
    );
}

/// Once the menu has closed, a completion list typed up takes its keys
/// again: an arrow moves the list's choice and leaves the caret where it
/// is, and Escape closes the list while the editor keeps the keyboard.
#[test]
fn the_completion_list_takes_its_keys_once_the_menu_has_closed() {
    let mut replay = deck_page();
    open_menu_on_the_call(&mut replay);
    press(&mut replay, Key::Escape);
    assert!(
        !draws_the_menu(&replay.frame(Vec::new())),
        "Escape closes the menu"
    );
    // A new line under the call, begun with a dot: the directives' list.
    press(&mut replay, Key::End);
    replay.type_text("\n.");
    let netlist = &replay.app.state.code.netlist;
    assert!(netlist.completion_open, "the dot brings the list up");
    let (caret, listed) = (netlist.cursor_char_index, netlist.completion_index);
    press(&mut replay, Key::ArrowDown);
    replay.frame(Vec::new());
    let netlist = &replay.app.state.code.netlist;
    assert_eq!(
        netlist.cursor_char_index, caret,
        "the list takes the arrow, and the caret stays"
    );
    assert_ne!(
        netlist.completion_index, listed,
        "the arrow moves the list's choice"
    );
    press(&mut replay, Key::Escape);
    replay.frame(Vec::new());
    assert!(
        !replay.app.state.code.netlist.completion_open,
        "the list takes Escape and closes"
    );
    assert!(
        editor_has_the_keyboard(&replay),
        "the editor keeps the keyboard"
    );
}

/// Whether the frame that put out `output` drew the completion list.
fn draws_the_completion_list(output: &egui::FullOutput) -> bool {
    let tree = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("a frame with AccessKit on publishes its tree");
    tree.nodes.iter().any(|(_, node)| {
        node.role() == Role::ListBoxOption
            && node
                .label()
                .is_some_and(|label| label.contains(" completion: "))
    })
}

/// A right-click on a name draws only the menu: the completion list that
/// putting the caret in the name would bring up is closed from the frame
/// the menu opens in, and stays closed once the menu has gone, until the
/// next edit.
#[test]
fn a_right_click_on_a_name_draws_only_the_menu() {
    let mut replay = deck_page();
    let opened = right_click_the_call(&mut replay);
    assert!(
        draws_the_menu(&opened) && !draws_the_completion_list(&opened),
        "the frame of the right-click draws the menu alone"
    );
    let next = replay.frame(Vec::new());
    assert!(
        draws_the_menu(&next) && !draws_the_completion_list(&next),
        "so does the next"
    );
    press(&mut replay, Key::Escape);
    let closed = replay.frame(Vec::new());
    assert!(
        !draws_the_menu(&closed) && !draws_the_completion_list(&closed),
        "the list stays closed once the menu has gone"
    );
}

/// Opening the menu over the name the signature hint is open for closes the
/// hint. Shift+F10 opens it, so the caret stays where the hint was opened
/// and nothing else would close the hint.
#[test]
fn opening_the_menu_closes_the_signature_hint() {
    let mut replay = deck_page();
    click_the_call(&mut replay);
    // The keyboard is put on the editor as a screen reader puts it there.
    let editor = editor_id_in(&replay.app.state.model, &replay.app.state.code.netlist);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frame(Vec::new());
    let state = &mut replay.app.state;
    show_signature_help_at_cursor(&state.model, &mut state.code.netlist)
        .expect("the call has a signature");
    replay.frame(Vec::new());
    assert!(
        replay.app.state.code.netlist.signature_help.open,
        "the hint is open on the call"
    );
    let key = |pressed| egui::Event::Key {
        key: Key::F10,
        physical_key: Some(Key::F10),
        pressed,
        repeat: false,
        modifiers: Modifiers::SHIFT,
    };
    let opened = replay.frame(vec![key(true)]);
    replay.frame(vec![key(false)]);
    assert!(draws_the_menu(&opened), "Shift+F10 opens the menu");
    assert!(
        !replay.app.state.code.netlist.signature_help.open,
        "and closes the hint"
    );
}

/// Whether the frame that put out `output` drew the rename dialog.
fn draws_the_rename_dialog(output: &egui::FullOutput) -> bool {
    let tree = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("a frame with AccessKit on publishes its tree");
    controls(tree)
        .iter()
        .any(|control| control.role == Role::Label && control.value == "Current name")
}

/// Rename symbol, chosen from the menu, opens the rename of the name under
/// the caret once the page has drawn; the frame of the choice asks for the
/// next at once, and the next frame draws the dialog.
#[test]
fn choosing_rename_symbol_draws_its_dialog_on_the_next_frame() {
    let mut replay = deck_page();
    open_menu_on_the_call(&mut replay);
    let chosen = choose(&mut replay, "Rename symbol\u{2026}");
    assert!(
        repaints_at_once(&chosen),
        "the frame of the choice asks for the next one at once"
    );
    let dialog = &replay.app.state.code.netlist.rename_dialog;
    assert!(
        dialog.open && dialog.original == NAME,
        "the choice opens the rename of the subcircuit: {dialog:?}"
    );
    assert!(
        !draws_the_rename_dialog(&chosen),
        "the choice runs once the page has drawn, after its dialogs"
    );
    assert!(
        draws_the_rename_dialog(&replay.frame(Vec::new())),
        "the next frame draws the rename dialog"
    );
}

/// Go to definition, chosen from the menu, moves the caret from the call to
/// the subcircuit's definition by the next frame, which the frame of the
/// choice asks for at once.
#[test]
fn choosing_go_to_definition_puts_the_caret_on_the_definition_on_the_next_frame() {
    let mut replay = deck_page();
    open_menu_on_the_call(&mut replay);
    assert_eq!(caret_line(&replay), 4, "the caret is on the call");
    let chosen = choose(&mut replay, "Go to definition");
    assert!(
        repaints_at_once(&chosen),
        "the frame of the choice asks for the next one at once"
    );
    replay.frame(Vec::new());
    assert_eq!(
        caret_line(&replay),
        1,
        "the next frame has the caret on the subcircuit's definition"
    );
}
