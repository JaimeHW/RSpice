//! Plotting from a deck: the source editor's menu on a node of a deck that
//! has run puts the node's voltage on the waveform viewer, and Plot device
//! current on a device puts its current there, through whole frames of the
//! application.

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use crate::feature_tests::replay::{Replay, only};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::netlist_document::editor_id_in;
use rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;
use rspice_results::result_presentation::{AnalysisPresentationKey, SourceWaveformPresentationKey};

/// A low-pass filter driven by a pulse, its output node `out`.
const DECK: &str = "* rc low-pass\nV1 in 0 PULSE(0 1 0 1n 1n 5u 10u)\nR1 in out 1k\nC1 out 0 1n\n.tran 10n 20u\n.end";
/// The resistor's card, whose second node is `out`.
const CARD: &str = "R1 in out 1k";
/// The source's card, which names `V1`.
const SOURCE: &str = "V1 in 0 PULSE(0 1 0 1n 1n 5u 10u)";

/// The deck page showing `DECK`, opened from its file, with the caret one
/// character into `word` on the line `card`.
fn deck_page_at(card: &str, word: &str) -> Replay {
    let mut replay = Replay::new();
    replay.frames(Vec::new());
    assert!(
        replay
            .app
            .state
            .open_test_deck(DECK.to_owned(), None, "rc.cir"),
        "the deck opens"
    );
    replay.frames(Vec::new());
    let line_start = DECK.find(card).expect("the deck writes the card");
    let caret = DECK[..line_start].chars().count() + card.find(word).expect("the word") + 1;
    let editor = editor_id_in(&replay.app.state.model, &replay.app.state.code.netlist);
    queue_caret_char_index(replay.ctx(), editor, caret);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frames(Vec::new());
    assert_eq!(
        replay.app.state.code.netlist.cursor_line,
        DECK[..line_start].matches('\n').count()
    );
    replay
}

/// Run `DECK` as the deck page's last run, with the waveform `named` picks
/// left off its plot; the key the viewer shows that waveform under.
fn run_the_deck(
    replay: &mut Replay,
    named: impl Fn(&str) -> bool,
) -> SourceWaveformPresentationKey {
    let scratch =
        std::env::temp_dir().join(format!("rspice-deck-source-plot-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).expect("a scratch directory");
    let path = scratch.join("rc.cir");
    std::fs::write(&path, DECK).expect("write the deck");
    let mut run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at(
        DECK,
        &path,
        rspice_results::run::SimulationRunLifecycle::Completed,
    );
    let _ = std::fs::remove_dir_all(&scratch);
    let dataset = run.dataset_id;
    let key = run
        .analyses
        .iter_mut()
        .find_map(|analysis| {
            let key = AnalysisPresentationKey::new(dataset, analysis);
            let wave = analysis
                .waveforms
                .iter_mut()
                .find(|wave| named(&wave.name))?;
            // The run's own plot leaves the trace off, so only the command
            // can put it there.
            wave.visible = false;
            Some(SourceWaveformPresentationKey::new(key, wave.name.clone()))
        })
        .expect("the run keeps the waveform");
    let state = &mut replay.app.state;
    state.runs.manual_run.last_run_id = Some(run.id);
    state.model.simulation.retained.runs.push(run);
    key
}

/// Whether the viewer shows the waveform of `key`.
fn shown(replay: &Replay, key: &SourceWaveformPresentationKey) -> bool {
    replay
        .app
        .state
        .results
        .results
        .session
        .waveform_visibility(key, false)
}

/// Before the deck has run, the menu on `out` offers Plot V(out) greyed,
/// saying why, and no row for a device's current.
#[test]
fn plotting_before_the_deck_has_run_says_why() {
    let mut replay = deck_page_at(CARD, "out");

    let menu = replay.key(Key::F10, Modifiers::SHIFT);
    only(&menu, "Plot V(out)", Role::MenuItem);
    assert!(
        !menu
            .iter()
            .any(|control| control.label.starts_with("Plot I(")),
        "the caret is on no device"
    );
    assert_eq!(
        replay
            .app
            .command_availability(CodeCommand::PlotNodeVoltage.key()),
        CommandAvailability::Disabled("the deck has not been run")
    );
}

/// After a transient run of the deck, Plot V(out) from the menu opens
/// Results with the run's trace of `out` on the plot.
#[test]
fn plotting_a_node_after_a_run_puts_its_voltage_on_the_plot() {
    let mut replay = deck_page_at(CARD, "out");
    let key = run_the_deck(&mut replay, |name| {
        name.eq_ignore_ascii_case("out") || name.eq_ignore_ascii_case("V(out)")
    });
    assert!(!shown(&replay, &key), "V(out) is not on the plot yet");

    replay.key(Key::F10, Modifiers::SHIFT);
    replay.click("Plot V(out)", Role::MenuItem);
    replay.frames(Vec::new());

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Results);
    assert!(shown(&replay, &key), "V(out) is on the plot");
}

/// After a transient run of the deck, Plot device current with the caret on
/// `V1` opens Results with the run's trace of the source's current.
#[test]
fn plotting_a_device_after_a_run_puts_its_current_on_the_plot() {
    let mut replay = deck_page_at(SOURCE, "V1");
    let key = run_the_deck(&mut replay, |name| name.eq_ignore_ascii_case("I(V1)"));
    assert!(!shown(&replay, &key), "I(V1) is not on the plot yet");
    assert_eq!(
        replay
            .app
            .command_availability(CodeCommand::PlotDeviceCurrent.key()),
        CommandAvailability::Available
    );

    replay
        .app
        .execute_command_key(CodeCommand::PlotDeviceCurrent.key());
    replay.frames(Vec::new());

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Results);
    assert!(shown(&replay, &key), "I(V1) is on the plot");
}
