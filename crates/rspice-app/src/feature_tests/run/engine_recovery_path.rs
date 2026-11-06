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

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use crate::feature_tests::replay::{PRIMARY, Replay, only};

/// The deck the script types into its new source document.
const DECK: &str = "* Engine recovery analytic check\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";

/// `author_deck` and the native half of `run`: the Netlist & Script Editor's
/// New source document opens its Logical path, Tab then Enter creates the
/// deck and closes the form, the editor takes the typed deck, Validate
/// source and Run deck run it to a DONE row in the Task log, and the palette
/// opens the recovery center on its Create checkpoint.
#[test]
fn the_engine_recovery_script_authors_validates_and_runs_its_deck() {
    let mut replay = Replay::new();
    replay.frames(Vec::new());

    replay.click("Netlist & Script Editor", Role::Button);
    replay.click("New source document", Role::Button);
    replay.click("Logical path", Role::TextInput);
    // The Create action follows the field in keyboard order.
    replay.key(Key::Tab, Modifiers::NONE);
    replay.key(Key::Enter, Modifiers::NONE);
    assert!(
        replay
            .app
            .state
            .model
            .workspace
            .content
            .netlist_descriptor
            .is_some(),
        "Tab then Enter from Logical path creates the deck"
    );
    replay.click("Project SPICE netlist editor", Role::MultilineTextInput);
    replay.key(Key::A, PRIMARY);
    replay.type_text(DECK);
    // The editor publishes its text line by line, not as one value.
    assert_eq!(
        replay.app.state.model.simulation.source.netlist_content, DECK,
        "select-all then typing replaces the new deck's text"
    );
    replay.click("Validate source", Role::Button);

    let launched = replay.click("Run deck", Role::Button);
    let task_log = launched
        .iter()
        .find(|control| control.label.starts_with("Task log"))
        .cloned()
        .expect("the bottom dock offers its Task log");
    replay.click(&task_log.label, task_log.role);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    loop {
        let shown = replay.frames(Vec::new());
        if shown.iter().any(|control| {
            [&control.label, &control.value]
                .iter()
                .any(|text| text.starts_with("DONE · Run 1 "))
        }) {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the run never reached a DONE row in the Task log"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    let recovery = replay.choose_command("Open recovery center");
    only(&recovery, "Create checkpoint", Role::Button);
}
