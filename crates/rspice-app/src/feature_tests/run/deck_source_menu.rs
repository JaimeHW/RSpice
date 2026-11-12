//! The deck editor's right-click menu, opened and chosen from in whole
//! frames of the application.
//!
//! The menu's rows are read from the code feature's context while it is
//! open, and the row chosen runs once the page has drawn: what it opens or
//! moves to is drawn on the next frame, which the frame of the choice asks
//! for at once.

use egui::accesskit::Role;
use egui::{Modifiers, PointerButton};

use super::engine_recovery_path::author_deck;
use crate::feature_tests::replay::{Replay, controls, only};
use crate::workbench::documents::netlist_document::{
    OwnedNetlistReplacement, replace_owned_sources_atomically,
};

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

/// The menu opened by a right-click on the middle of the call's line, which
/// puts the caret in the subcircuit's name there. A click on the line comes
/// first: it closes the completion list the typing of the deck left open
/// over the editor.
fn open_menu_on_the_call(replay: &mut Replay) {
    let line = replay
        .frames(Vec::new())
        .into_iter()
        .find(|control| control.role == Role::TextRun && control.value.trim_end() == CALL)
        .expect("the editor publishes the call's line");
    let press = |button, pressed| egui::Event::PointerButton {
        pos: line.center,
        button,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(line.center)]);
    for button in [PointerButton::Primary, PointerButton::Secondary] {
        replay.frame(vec![press(button, true)]);
        replay.frames(vec![press(button, false)]);
    }
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
