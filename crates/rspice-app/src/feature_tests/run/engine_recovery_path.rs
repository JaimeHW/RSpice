//! The browser engine-recovery qualification's path, replayed through whole
//! frames of the application.
//!
//! `tools/ci/check_browser_engine_recovery.py` drives the real browser
//! workbench by the labels and roles in the AccessKit tree it publishes, as
//! `tools/ci/browser_workbench.py`'s `controls` reads them. That job needs a
//! WebDriver and a WebGPU browser, so these cases replay the same clicks and
//! keys natively and read the same tree: a renamed control, a field that lost
//! its name or a button that no longer opens what the script expects fails
//! here first. The engine's start, failure and retry belong to the browser's
//! worker and are not replayed; the native engine is always ready.

use egui::Key;
use egui::accesskit::Role;

use crate::feature_tests::replay::{PRIMARY, Replay, only};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::ui::widgets::{NotificationAction, NotificationCategory, ToastKind};

/// The deck the script types into the open deck.
const DECK: &str = "* Engine recovery analytic check\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";

/// The script's `author_deck`: on the Netlist workspace the browser's New
/// deck makes a deck the project keeps as text and puts it in front, and the
/// editor takes the typed deck. The desktop's New deck asks where a file
/// goes, so the replay makes the deck as the browser's command does.
pub(super) fn author_deck(replay: &mut Replay) {
    replay.frames(Vec::new());

    replay.click("Netlist", Role::Button);
    assert!(replay.app.state.new_stored_deck(), "New deck makes a deck");
    let decks = &replay.app.state.model.workspace.content.decks;
    assert!(
        decks.len() == 1 && decks[0].stored_deck().is_some(),
        "the project keeps the new deck as text: {decks:?}"
    );
    replay.frames(Vec::new());
    replay.click("Project SPICE netlist editor", Role::MultilineTextInput);
    replay.key(Key::A, PRIMARY);
    replay.type_text(DECK);
    // The editor publishes its text line by line, not as one value.
    assert_eq!(
        replay.app.state.model.simulation.source.netlist_content, DECK,
        "select-all then typing replaces the new deck's text"
    );
    assert_eq!(
        replay.app.state.model.workspace.content.decks[0]
            .stored_deck()
            .map(|deck| deck.text.as_str()),
        Some(DECK),
        "what is typed is the project's at once"
    );
}

/// The browser's New deck is the command the script presses: there it asks
/// nothing, so its name has no ellipsis.
#[test]
fn the_browser_names_new_deck_as_the_script_presses_it() {
    assert_eq!(
        rspice_app_code::workbench::code::commands::new_deck_label(true),
        "New deck"
    );
    let script = include_str!("../../../../../tools/ci/check_browser_engine_recovery.py");
    assert!(
        script.contains(r#"choose_command(browser, "New deck")"#),
        "the script presses New deck"
    );
}

/// `author_deck` and the native half of `run`: the editor takes the typed
/// deck, Validate
/// source and Run deck run it, the finished run opens in Results, a press of
/// the Task log tab shows its DONE row there, and the palette opens the
/// Recovery page on its Create checkpoint.
#[test]
fn the_engine_recovery_script_authors_validates_and_runs_its_deck() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    replay.click("Validate source", Role::Button);

    replay.click("Run deck", Role::Button);
    // The run opens in Results the frame after it completes, so the DONE row
    // on the editor's page can last a single frame, which a browser polling
    // the tree misses. The script reads it on Results, whose own dock layout
    // starts closed, through a press of the Task log tab.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while replay.app.state.workbench.workspace != Workspace::Results {
        assert!(
            std::time::Instant::now() < deadline,
            "the finished run never opened in Results"
        );
        replay.frames(Vec::new());
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    let task_log = replay
        .frames(Vec::new())
        .into_iter()
        .find(|control| control.label.starts_with("Task log"))
        .expect("Results keeps the bottom dock's Task log tab");
    replay.click(&task_log.label, task_log.role);
    assert!(
        replay.frames(Vec::new()).iter().any(|control| {
            [&control.label, &control.value]
                .iter()
                .any(|text| text.starts_with("DONE · Run 1 "))
        }),
        "a press of the Task log tab on Results shows the run's DONE row"
    );

    let recovery = replay.choose_command("Recovery");
    only(&recovery, "Create checkpoint", Role::Button);
}

/// A failed run's notice leaves the editor's Run deck where the reader can
/// press it. The browser job retries a failed run from the Netlist workspace
/// while the run's failure notice is still up; the notice used to land
/// over the editor's toolbar row, took the press, and the retry never started.
#[test]
fn a_run_failure_notice_leaves_run_deck_pressable() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    replay.click("Validate source", Role::Button);
    let ctx = replay.ctx().clone();
    replay.app.state.workbench.toasts.notify_with_action(
        &ctx,
        NotificationCategory::Job,
        ToastKind::Error,
        "Run 1 failed",
        "DC Operating Point failed: The simulation engine stopped unexpectedly.",
        NotificationAction::ShowInConsole { log_id: 0 },
    );
    only(
        &replay.frames(Vec::new()),
        "Dismiss Run 1 failed",
        Role::Button,
    );

    replay.click("Run deck", Role::Button);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while replay.app.state.model.simulation.retained.runs.is_empty() {
        assert!(
            std::time::Instant::now() < deadline,
            "Run deck under a failure notice never started a run"
        );
        replay.frames(Vec::new());
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
