//! Run reads the deck as the editor shows it, in whole frames of the
//! application.
//!
//! A browser hands the page the reader's last keystrokes and the Run key in
//! one frame, and so may a fast typist on the desktop. The key's command is
//! read before the page draws, and the editor applies the keystrokes as it
//! draws; Run waits for them, so it runs what the reader typed just before
//! pressing it, and refuses it, saying why, when that text cannot run.

use egui::{Key, Modifiers};

use super::engine_recovery_path::author_deck;
use crate::feature_tests::replay::{PRIMARY, Replay};

/// A key pressed and released.
fn key(key: Key, modifiers: Modifiers) -> [egui::Event; 2] {
    [true, false].map(|pressed| egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed,
        repeat: false,
        modifiers,
    })
}

/// `text` typed and Run's key (F5 on the desktop; a browser's is
/// Ctrl+Enter) pressed in the same frame, each line break the Enter key.
fn type_and_run(replay: &mut Replay, text: &str) {
    let mut events = Vec::new();
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            events.extend(key(Key::Enter, Modifiers::NONE));
        }
        if !line.is_empty() {
            events.push(egui::Event::Text(line.to_owned()));
        }
    }
    events.extend(key(Key::F5, Modifiers::NONE));
    replay.frame(events);
}

/// The deck text the run read: the one being run, or the one run last.
fn run_input(replay: &Replay) -> Option<String> {
    let manual = &replay.app.state.runs.manual_run;
    manual
        .pending_run_buffer
        .clone()
        .or_else(|| manual.last_run_buffer.clone())
}

/// Whether a run was asked for, has started or has been kept.
fn anything_ran(replay: &Replay) -> bool {
    let simulation = &replay.app.state.model.simulation;
    simulation.execution.trigger_simulation
        || simulation.execution.has_active_execution()
        || !simulation.retained.runs.is_empty()
        || run_input(replay).is_some()
}

/// A line typed in the frame Run's key is pressed in is in the deck the
/// run reads.
#[test]
fn run_pressed_in_the_frame_of_a_keystroke_runs_the_text_typed() {
    let mut replay = Replay::new();
    author_deck(&mut replay);

    type_and_run(&mut replay, "* typed with the Run key");

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let input = loop {
        if let Some(input) = run_input(&replay) {
            break input;
        }
        assert!(std::time::Instant::now() < deadline, "the deck never ran");
        replay.frame(Vec::new());
        std::thread::sleep(std::time::Duration::from_millis(10));
    };
    assert!(
        input.contains("* typed with the Run key"),
        "the run read the deck as typed:\n{input}"
    );
}

/// A deck typed with a blocking error in the frame Run's key is pressed in
/// does not run, neither as typed nor as it stood before: nothing starts,
/// no result is kept, and the notice names the deck and the line once, then
/// says what is wrong there as the line's finding does.
#[test]
fn a_deck_typed_with_an_error_in_the_frame_of_run_is_refused() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    replay.frame(key(Key::A, PRIMARY).to_vec());

    type_and_run(
        &mut replay,
        "* errored\nV1 in 0 1\nR1 in 0 {1k\n.op\n.end\n",
    );
    for _ in 0..5 {
        replay.frame(Vec::new());
    }

    assert!(
        replay
            .app
            .state
            .model
            .simulation
            .source
            .netlist_content
            .contains("{1k"),
        "the editor shows the typed deck"
    );
    assert!(!anything_ran(&replay), "nothing runs");
    let notice = replay
        .app
        .state
        .prompt_text()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        notice, "deck1.cir did not run: line 3: Unterminated expression",
        "the notice names the deck, the line and the line's finding"
    );
}
