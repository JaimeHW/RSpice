//! The deck page's Validate source, Run and Stop, pressed and released in
//! whole frames of the application.
//!
//! The toolbar raises all three for the dispatcher, which carries them out
//! once the page has drawn. A press must still take effect in the frame of its
//! release, and that frame must ask for the next at once, so what the press
//! changed is drawn without waiting for the reader to move the pointer.

use egui::accesskit::Role;
use egui::{Modifiers, Pos2};

use super::engine_recovery_path::author_deck;
use crate::feature_tests::replay::{Control, Replay, only};

/// Press the control at `at` in one frame and release it in the next; what
/// the frame of the release put out.
fn press_and_release(replay: &mut Replay, at: Pos2) -> egui::FullOutput {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frame(vec![egui::Event::PointerMoved(at), button(true)]);
    replay.frame(vec![button(false)])
}

/// Whether `output` asks for the next frame at once.
fn repaints_at_once(output: &egui::FullOutput) -> bool {
    output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .is_some_and(|viewport| viewport.repaint_delay.is_zero())
}

/// Whether a run of the deck was asked for, has started or has been kept.
fn run_requested(replay: &Replay) -> bool {
    let simulation = &replay.app.state.model.simulation;
    simulation.execution.trigger_simulation
        || simulation.execution.has_active_execution()
        || !simulation.retained.runs.is_empty()
}

/// Whether the deck has been validated.
fn validated(replay: &Replay) -> bool {
    replay
        .app
        .state
        .model
        .netlist_artifacts
        .validation
        .is_some()
}

/// Whether the tree `output` published draws Run deck disabled.
fn run_deck_disabled(output: &egui::FullOutput) -> bool {
    let tree = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("a frame with AccessKit on publishes its tree");
    let mut run_deck = tree
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == Role::Button && node.label() == Some("Run deck"));
    let (_, node) = run_deck.next().expect("the page draws Run deck");
    assert!(run_deck.next().is_none(), "the page draws one Run deck");
    node.is_disabled()
}

/// Where the page's own Validate source is. The application toolbar offers
/// one too, above the page, and carries it out before the page draws.
fn page_validate_source(controls: &[Control]) -> Pos2 {
    let mut offered = controls
        .iter()
        .filter(|control| control.role == Role::Button && control.label == "Validate source")
        .collect::<Vec<_>>();
    assert_eq!(
        offered.len(),
        2,
        "the application toolbar and the page each offer Validate source"
    );
    offered.sort_by(|a, b| a.center.y.total_cmp(&b.center.y));
    offered[1].center
}

/// The page's Validate source validates the deck once the page has drawn,
/// in the frame of its release. That frame still draws the deck as it was
/// before, and it asks for the next at once, which draws the deck validated.
#[test]
fn validate_validates_in_the_frame_it_is_released_and_the_next_frame_draws_it() {
    // Wide enough for the page's toolbar to show its full set of actions.
    let mut replay = Replay::new().sized(egui::vec2(2560.0, 1000.0));
    author_deck(&mut replay);
    let at = page_validate_source(&replay.frames(Vec::new()));
    assert!(!validated(&replay), "nothing is validated before the press");
    assert!(
        run_deck_disabled(&replay.frame(Vec::new())),
        "Run deck waits for the deck to be validated"
    );

    let output = press_and_release(&mut replay, at);
    assert!(
        validated(&replay),
        "Validate source validates the deck in the frame of its release"
    );
    assert!(
        run_deck_disabled(&output),
        "the frame of the release draws the deck as it was before validation"
    );
    assert!(
        repaints_at_once(&output),
        "the frame Validate source validated the deck in asks for the next at once"
    );
    assert!(
        !run_deck_disabled(&replay.frame(Vec::new())),
        "the next frame draws the deck validated"
    );
}

/// Run deck runs the deck in the frame of its release, and that frame asks
/// for the next at once.
#[test]
fn run_deck_runs_in_the_frame_it_is_released_and_repaints() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    replay.click("Validate source", Role::Button);
    let at = only(&replay.frames(Vec::new()), "Run deck", Role::Button).center;
    assert!(!run_requested(&replay), "nothing has run before the press");
    assert!(
        !repaints_at_once(&replay.frame(Vec::new())),
        "the settled page waits for input before its next frame"
    );

    let output = press_and_release(&mut replay, at);
    assert!(
        run_requested(&replay),
        "Run deck runs the deck in the frame of its release"
    );
    assert!(
        repaints_at_once(&output),
        "the frame Run deck ran the deck in asks for the next at once"
    );
}

/// Stop asks the run to stop in the frame of its release, and that frame
/// asks for the next at once.
#[test]
fn stop_stops_the_run_in_the_frame_it_is_released_and_repaints() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    replay.click("Validate source", Role::Button);
    let run_at = only(&replay.frames(Vec::new()), "Run deck", Role::Button).center;
    // A run that stays going: the native engine finishes this deck's
    // operating point before a frame could press Stop.
    let simulation = &mut replay.app.state.model.simulation;
    let identity = simulation
        .start_run()
        .execution_identity()
        .expect("a started run has an execution identity");
    simulation.execution.active_execution = Some(identity);
    simulation.execution.is_running = false;
    // The deck page's Stop is where its Run deck was.
    let at = replay
        .frames(Vec::new())
        .into_iter()
        .filter(|control| control.label == "Stop" && control.role == Role::Button)
        .map(|control| control.center)
        .min_by(|a, b| a.distance(run_at).total_cmp(&b.distance(run_at)))
        .expect("the deck page's Run deck turns to Stop while a run is going");
    assert!(
        !replay.app.state.model.simulation.cancellation_is_pending(),
        "nothing has asked the run to stop before the press"
    );
    assert!(
        !repaints_at_once(&replay.frame(Vec::new())),
        "the settled page waits for input before its next frame"
    );

    let output = press_and_release(&mut replay, at);
    assert!(
        replay.app.state.model.simulation.cancellation_is_pending(),
        "Stop asks the run to stop in the frame of its release"
    );
    assert!(
        repaints_at_once(&output),
        "the frame Stop asked the run to stop in asks for the next at once"
    );
}
