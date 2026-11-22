//! Run and Stop while a deck is what Run runs, the deck page's run strip's
//! Open in Results and its Open deck, pressed and released in whole frames
//! of the application.
//!
//! Each press is raised for the dispatcher, which carries it out once the
//! frame has drawn. A press must still take effect in the frame of its
//! release, and that frame must ask for the next at once, so what the press
//! changed is drawn without waiting for the reader to move the pointer.

use std::time::{Duration, Instant};

use egui::accesskit::Role;
use egui::{Modifiers, Pos2};

use super::engine_recovery_path::{author_deck, run_deck};
use crate::feature_tests::replay::{Replay, only};

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

/// How long a page may go on asking for its next frame at once before it is
/// taken never to settle.
const SETTLE_WITHIN: Duration = Duration::from_secs(20);

/// The first frame that waits for input before its next, or the frame that
/// still did not once `SETTLE_WITHIN` has passed.
///
/// The editor builds the typed deck's index on a thread of its own, and the
/// finished build wakes the next frame at once, which egui answers with two.
/// On a busy machine that build finishes frames after the typing, so a frame
/// a fixed count after it can be one the build woke. A frame that waits says
/// the page has settled.
fn settled(replay: &mut Replay) -> egui::FullOutput {
    let started = Instant::now();
    loop {
        let output = replay.frame(Vec::new());
        if !repaints_at_once(&output) || started.elapsed() >= SETTLE_WITHIN {
            return output;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// Whether a run of the deck was asked for, has started or has been kept.
fn run_requested(replay: &Replay) -> bool {
    let simulation = &replay.app.state.model.simulation;
    simulation.execution.trigger_simulation
        || simulation.execution.has_active_execution()
        || !simulation.retained.runs.is_empty()
}

/// Run, naming the deck, checks and runs the deck in the frame of its
/// release, with no check pressed first, and that frame asks for the next
/// at once.
#[test]
fn run_deck_runs_in_the_frame_it_is_released_and_repaints() {
    let mut replay = Replay::new();
    author_deck(&mut replay);
    let at = run_deck(&replay.frames(Vec::new())).center;
    assert!(!run_requested(&replay), "nothing has run before the press");
    assert!(
        !repaints_at_once(&settled(&mut replay)),
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
    let run_at = run_deck(&replay.frames(Vec::new())).center;
    // A run that stays going: the native engine finishes this deck's
    // operating point before a frame could press Stop.
    let simulation = &mut replay.app.state.model.simulation;
    let identity = simulation
        .start_run()
        .execution_identity()
        .expect("a started run has an execution identity");
    simulation.execution.active_execution = Some(identity);
    simulation.execution.is_running = false;
    // Stop is where Run was.
    let at = replay
        .frames(Vec::new())
        .into_iter()
        .filter(|control| {
            control.label.starts_with("Stop active simulation") && control.role == Role::Button
        })
        .map(|control| control.center)
        .min_by(|a, b| a.distance(run_at).total_cmp(&b.distance(run_at)))
        .expect("Run turns to Stop while a run is going");
    assert!(
        !replay.app.state.model.simulation.cancellation_is_pending(),
        "nothing has asked the run to stop before the press"
    );
    assert!(
        !repaints_at_once(&settled(&mut replay)),
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

/// Open in Results on the run strip selects the run and opens Results in the
/// frame of its release: the page selects the run, and the dispatcher opens
/// Results once the page has drawn.
#[test]
fn open_in_results_opens_the_run_in_the_frame_it_is_released() {
    use crate::workbench::commands::tests::{RunOrigin, app_with_prepared_run};

    const DECK: &str = "* strip check\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n";
    let mut app = app_with_prepared_run(RunOrigin::ManualDeck);
    let run = app.state.model.simulation.retained.runs[0].id;
    app.state.seed_test_deck(DECK);
    app.state.runs.manual_run.last_run_buffer = Some(DECK.to_owned());
    app.state.runs.manual_run.last_run_id = Some(run);
    app.state.model.simulation.view.active_run_idx = None;
    app.state.workbench.workspace = rspice_app_state::workbench::workspace::Workspace::Netlist;
    let mut replay = Replay::of(app);
    let open = only(&replay.frames(Vec::new()), "Open in Results", Role::Button).center;
    press_and_release(&mut replay, open);
    assert_eq!(
        replay.app.state.model.simulation.view.active_run_idx,
        Some(0),
        "the release selects the run"
    );
    assert_eq!(
        replay.app.state.workbench.workspace,
        rspice_app_state::workbench::workspace::Workspace::Results,
        "the frame of the release ends on Results"
    );
}

/// Open deck on the empty deck page is answered in the frame of its
/// release: on a project open read-only, that frame says why no deck can be
/// added, before any file is asked for, though the dispatcher carries it out
/// once the page has drawn.
#[test]
fn open_deck_is_answered_in_the_frame_it_is_released() {
    let mut replay = Replay::new();
    replay.app.state.model.safe_mode.activate(
        rspice_app_state::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );
    assert!(replay.app.state.model.safe_mode.project_read_only());
    replay.app.state.workbench.workspace =
        rspice_app_state::workbench::workspace::Workspace::Netlist;
    // The empty page measures its actions in an invisible pass before it
    // draws them, and the measured copies are in the tree too: the drawn
    // button is the last.
    let open = replay
        .frames(Vec::new())
        .into_iter()
        .rfind(|control| control.role == Role::Button && control.label == "Open deck\u{2026}")
        .expect("the empty page offers Open deck")
        .center;
    assert_eq!(
        replay.app.state.prompt_text(),
        None,
        "nothing is said before the press"
    );
    press_and_release(&mut replay, open);
    let said = replay.app.state.prompt_text().unwrap_or_default();
    assert!(
        said.contains("read-only"),
        "the frame of the release says why no deck can be added: {said:?}"
    );
}
