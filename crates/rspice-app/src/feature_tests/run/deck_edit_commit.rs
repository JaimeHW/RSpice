//! The deck page's editor, typed into, undone and redone in whole frames of
//! the application.
//!
//! A keystroke is committed to the project's deck by the end of its frame,
//! and the next frame draws the committed deck with the caret and the
//! selection where the reader left them. An edit the project refuses is
//! drawn only until the next frame, which draws the deck as the project
//! holds it, with the caret and the history as they were before it. The
//! editor raises the commit and the shell carries it out once the page has
//! drawn, before anything else the frame raised.

use egui::{Key, Modifiers};

use super::engine_recovery_path::author_deck;
use crate::feature_tests::replay::{PRIMARY, Replay};
use rspice_app_code::workbench::documents::netlist_document::{
    OwnedNetlistReplacement, replace_deck_files,
};

/// A key pressed and released.
fn key(key: Key, modifiers: Modifiers) -> Vec<egui::Event> {
    [true, false]
        .into_iter()
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        })
        .collect()
}

/// Typed text, in one frame.
fn typed(replay: &mut Replay, text: &str) {
    replay.frame(vec![egui::Event::Text(text.to_owned())]);
}

/// More than a second with no keystroke: the editor's next keystroke starts
/// an undo step of its own.
fn pause(replay: &mut Replay) {
    for _ in 0..11 {
        replay.frame(Vec::new());
    }
}

/// One frame of undo, or of redo.
fn undo(replay: &mut Replay) {
    replay.frame(key(Key::Z, PRIMARY));
}

fn redo(replay: &mut Replay) {
    replay.frame(key(Key::Z, PRIMARY | Modifiers::SHIFT));
}

/// The open deck's text, as the session holds it.
fn committed(replay: &Replay) -> String {
    replay
        .app
        .state
        .code
        .netlist
        .open_deck
        .as_ref()
        .expect("the deck is open")
        .root
        .text
        .clone()
}

/// The deck the editor shows.
fn shown(replay: &Replay) -> &str {
    &replay.app.state.model.simulation.source.netlist_content
}

fn caret(replay: &Replay) -> usize {
    replay.app.state.code.netlist.cursor_char_index
}

/// Whether `output` asks for the next frame at once.
fn repaints_at_once(output: &egui::FullOutput) -> bool {
    output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .is_some_and(|viewport| viewport.repaint_delay.is_zero())
}

/// A typed character is committed by the end of its frame, and the next
/// frame draws the committed deck with the caret and the selection kept.
#[test]
fn a_keystroke_is_committed_in_its_frame_and_the_next_frame_keeps_caret_and_selection() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let before = committed(&replay);
    let end = before.chars().count();
    assert_eq!(
        caret(&replay),
        end,
        "the caret is at the end of the typed deck"
    );

    // One frame types a character and selects it.
    let mut events = vec![egui::Event::Text("x".to_owned())];
    events.extend(key(Key::ArrowLeft, Modifiers::SHIFT));
    let output = replay.frame(events);
    assert_eq!(
        committed(&replay),
        format!("{before}x"),
        "the character is committed by the end of its frame"
    );
    assert!(
        repaints_at_once(&output),
        "the frame of the keystroke asks for the next one at once"
    );

    replay.frame(Vec::new());
    assert_eq!(
        shown(&replay),
        format!("{before}x"),
        "the next frame draws the committed deck"
    );
    assert_eq!(
        replay.app.state.code.netlist.selected_byte_range,
        Some(before.len()..before.len() + 1),
        "the selection made in the frame of the commit is kept"
    );

    // The kept selection is what the next keystroke replaces.
    typed(&mut replay, "y");
    assert_eq!(committed(&replay), format!("{before}y"));
    replay.frame(Vec::new());
    assert_eq!(
        caret(&replay),
        end + 1,
        "the caret stands after the replacement"
    );
}

/// Undo and redo in the editor step what was typed a word at a time, as the
/// editor groups typing, and each step is committed by the end of its frame.
#[test]
fn undo_and_redo_step_what_was_typed_as_the_editor_groups_it() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    pause(&mut replay);
    let before = committed(&replay);
    let end = before.chars().count();
    for text in ["a", "b", " ", "c", "d"] {
        typed(&mut replay, text);
    }
    assert_eq!(committed(&replay), format!("{before}ab cd"));

    undo(&mut replay);
    assert_eq!(
        committed(&replay),
        format!("{before}ab"),
        "the first undo takes the last word back with the space before it"
    );
    undo(&mut replay);
    assert_eq!(
        committed(&replay),
        before,
        "the second undo takes the first word back"
    );
    redo(&mut replay);
    assert_eq!(committed(&replay), format!("{before}ab"), "the redo");

    replay.frame(Vec::new());
    assert_eq!(
        shown(&replay),
        format!("{before}ab"),
        "the next frame draws the stepped deck"
    );
    assert_eq!(caret(&replay), end + 2, "the caret follows the redone text");
}

/// A word typed across frames is one undo step, two of its keys in one frame
/// among them, and a pause in typing starts the next step.
#[test]
fn a_word_typed_across_frames_is_one_undo_step_and_a_pause_starts_the_next() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    pause(&mut replay);
    let before = committed(&replay);
    typed(&mut replay, "de");
    replay.frame(vec![
        egui::Event::Text("f".to_owned()),
        egui::Event::Text("g".to_owned()),
    ]);
    pause(&mut replay);
    typed(&mut replay, "h");
    assert_eq!(committed(&replay), format!("{before}defgh"));

    undo(&mut replay);
    assert_eq!(
        committed(&replay),
        format!("{before}defg"),
        "the pause started a step"
    );
    undo(&mut replay);
    assert_eq!(
        committed(&replay),
        before,
        "the word typed across frames is one step"
    );
}

/// Past the editor's own history, undo steps the deck's own last edit back
/// and redo applies it again, each by the end of its frame.
#[test]
fn undo_past_the_editor_history_steps_the_decks_own_edit() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let before = committed(&replay);
    let replaced = before.replace("R1 in 0 1k", "R1 in 0 2k");
    assert_ne!(replaced, before);
    replace_deck_files(
        &mut replay.app.state.model,
        &mut replay.app.state.code.netlist,
        vec![OwnedNetlistReplacement::root(&before, replaced.clone(), 1)],
    )
    .expect("the deck is open");
    replay.frames(Vec::new());
    assert_eq!(
        shown(&replay),
        replaced,
        "the editor shows the replaced deck"
    );

    undo(&mut replay);
    assert_eq!(
        committed(&replay),
        before,
        "undo steps the replacement back"
    );
    replay.frame(Vec::new());
    assert_eq!(shown(&replay), before, "the next frame draws it");

    redo(&mut replay);
    assert_eq!(committed(&replay), replaced, "redo applies it again");
    replay.frame(Vec::new());
    assert_eq!(shown(&replay), replaced, "the next frame draws it");
}
