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
use crate::workbench::documents::netlist_document::{
    OwnedNetlistReplacement, replace_owned_sources_atomically,
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

/// One frame of undo, or of redo.
fn undo(replay: &mut Replay) {
    replay.frame(key(Key::Z, PRIMARY));
}

fn redo(replay: &mut Replay) {
    replay.frame(key(Key::Z, PRIMARY | Modifiers::SHIFT));
}

/// The project's copy of the deck.
fn committed(replay: &Replay) -> String {
    replay
        .app
        .state
        .model
        .workspace
        .content
        .netlist_source
        .clone()
        .expect("the project owns the deck")
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

/// Undo and redo in the editor step what was typed, one typed run at a time,
/// and each step is committed by the end of its frame.
#[test]
fn undo_and_redo_step_what_was_typed_as_the_editor_groups_it() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let before = committed(&replay);
    let end = before.chars().count();
    for text in ["a", "b", "c"] {
        typed(&mut replay, text);
    }
    assert_eq!(committed(&replay), format!("{before}abc"));

    undo(&mut replay);
    assert_eq!(committed(&replay), format!("{before}ab"), "the first undo");
    undo(&mut replay);
    assert_eq!(committed(&replay), format!("{before}a"), "the second undo");
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

/// One typed run is one undo step, and two runs in one frame are two.
#[test]
fn each_typed_run_is_its_own_undo_step_even_two_in_one_frame() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let before = committed(&replay);
    typed(&mut replay, "de");
    replay.frame(vec![
        egui::Event::Text("f".to_owned()),
        egui::Event::Text("g".to_owned()),
    ]);
    assert_eq!(committed(&replay), format!("{before}defg"));

    undo(&mut replay);
    assert_eq!(committed(&replay), format!("{before}def"));
    undo(&mut replay);
    assert_eq!(committed(&replay), format!("{before}de"));
    undo(&mut replay);
    assert_eq!(committed(&replay), before, "the run of two is one step");
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
    replace_owned_sources_atomically(
        &mut replay.app.state,
        vec![OwnedNetlistReplacement::root(&before, replaced.clone(), 1)],
    )
    .expect("the project owns the deck");
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

/// An edit the project refuses is not kept: its frame says why, and the
/// next frame draws the deck as the project holds it, with the caret where
/// it was before the refused edit and the history as it was.
#[test]
fn a_refused_edit_reverts_on_the_next_frame() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let before = committed(&replay);
    let end = before.chars().count();
    assert!(before.ends_with('\n'), "the deck ends its last line");
    typed(&mut replay, "x");
    let with_x = format!("{before}x");
    assert_eq!(committed(&replay), with_x);

    // The caret steps back before the deck's last line break, and the edit
    // made there is refused.
    replay.frame(key(Key::ArrowLeft, Modifiers::NONE));
    replay.frame(key(Key::ArrowLeft, Modifiers::NONE));
    assert_eq!(caret(&replay), end - 1);
    replay.app.state.model.live_write_locks.netlist = Some("Another session".to_owned());
    let output = replay.frame(vec![egui::Event::Text("a".to_owned())]);
    assert_eq!(committed(&replay), with_x, "the project refuses the edit");
    assert!(
        repaints_at_once(&output),
        "the frame of the refusal asks for the next one at once"
    );
    assert_eq!(
        replay.app.state.prompt_text(),
        Some("Another session holds the netlist write lease; no source was changed."),
        "the frame of the refusal says why"
    );
    assert_eq!(
        shown(&replay),
        format!("{}a\nx", &before[..before.len() - 1]),
        "the frame of the refusal still draws what was typed"
    );

    replay.frame(Vec::new());
    assert_eq!(
        shown(&replay),
        with_x,
        "the next frame draws the project's deck"
    );
    assert_eq!(
        caret(&replay),
        end - 1,
        "the caret is where it was before the refused edit"
    );
    replay.frame(Vec::new());
    assert_eq!(shown(&replay), with_x, "and the frame after it");

    // Once the lease is let go, undo steps back through what was typed
    // before the refusal, and redo applies it again.
    replay.app.state.model.live_write_locks.netlist = None;
    undo(&mut replay);
    assert_eq!(
        committed(&replay),
        before,
        "the typing before the refusal is undone"
    );
    redo(&mut replay);
    assert_eq!(committed(&replay), with_x, "and redone");
}
