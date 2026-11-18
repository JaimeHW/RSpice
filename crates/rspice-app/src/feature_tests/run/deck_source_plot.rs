//! Plotting from a deck: the source editor's menu on a node of a deck that
//! has run puts the node's voltage on the waveform viewer, through whole
//! frames of the application.

use egui::accesskit::Role;
use egui::{Key, Modifiers};

use super::engine_recovery_path::author_deck;
use crate::feature_tests::replay::{Replay, only};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::documents::netlist_document::{
    OwnedNetlistReplacement, editor_id_in, replace_owned_sources_atomically,
};
use rspice_app_code::workbench::documents::text_editor_commands::queue_caret_char_index;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;
use rspice_results::result_presentation::{AnalysisPresentationKey, SourceWaveformPresentationKey};

/// A low-pass filter driven by a pulse, its output node `out`.
const DECK: &str = "* rc low-pass\nV1 in 0 PULSE(0 1 0 1n 1n 5u 10u)\nR1 in out 1k\nC1 out 0 1n\n.tran 10n 20u\n.end";
/// The resistor's card, whose second node is `out`.
const CARD: &str = "R1 in out 1k";

/// The deck page showing `DECK`, put in place whole, with the caret on
/// `out` in the resistor's card.
fn deck_page_at_out() -> Replay {
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
        &mut replay.app.state.model,
        &mut replay.app.state.code.netlist,
        &mut replay.app.state.code.netlist_open_documents,
        vec![OwnedNetlistReplacement::root(&typed, DECK.to_owned(), 1)],
    )
    .expect("the project owns the deck");
    replay.frames(Vec::new());
    let line_start = DECK.find(CARD).expect("the deck writes the card");
    let caret = DECK[..line_start].chars().count() + CARD.find("out").expect("out") + 1;
    let editor = editor_id_in(&replay.app.state.model, &replay.app.state.code.netlist);
    queue_caret_char_index(replay.ctx(), editor, caret);
    replay
        .ctx()
        .memory_mut(|memory| memory.request_focus(editor));
    replay.frames(Vec::new());
    assert_eq!(replay.app.state.code.netlist.cursor_line, 2);
    replay
}

/// Before the deck has run, the menu on `out` offers Plot V(out) greyed,
/// saying why, and no row for a device's current.
#[test]
fn plotting_before_the_deck_has_run_says_why() {
    let mut replay = deck_page_at_out();

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
    let mut replay = deck_page_at_out();
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
            let wave = analysis.waveforms.iter_mut().find(|wave| {
                wave.name.eq_ignore_ascii_case("out") || wave.name.eq_ignore_ascii_case("V(out)")
            })?;
            // The run's own plot leaves the trace off, so only the row can
            // put it there.
            wave.visible = false;
            Some(SourceWaveformPresentationKey::new(key, wave.name.clone()))
        })
        .expect("the run keeps the voltage of out");
    let state = &mut replay.app.state;
    state.runs.manual_run.last_run_id = Some(run.id);
    state.model.simulation.retained.runs.push(run);
    let shown = |replay: &Replay| {
        replay
            .app
            .state
            .results
            .results
            .session
            .waveform_visibility(&key, false)
    };
    assert!(!shown(&replay), "V(out) is not on the plot yet");

    replay.key(Key::F10, Modifiers::SHIFT);
    replay.click("Plot V(out)", Role::MenuItem);
    replay.frames(Vec::new());

    assert_eq!(replay.app.state.workbench.workspace, Workspace::Results);
    assert!(shown(&replay), "V(out) is on the plot");
}
